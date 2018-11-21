pub use http::{
    header::{HeaderName, HeaderValue},
    Request as HttpRequest, Response as HttpResponse, StatusCode, Uri,
};
use http::{HttpTryFrom, Request, Response};
use std::io::{copy, Error as IOError, ErrorKind as IOErrorKind, Read, Write};

pub trait HttpMessage {
    type Response;
    fn send(self) -> Self::Response;
}

#[derive(Clone, Debug)]
pub struct HttpRequestBody {
    _v: (),
}

impl Read for HttpRequestBody {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, IOError> {
        wasp_core::request::read_body(bytes)
            .map_err(|_| IOError::new(IOErrorKind::BrokenPipe, "Could not read body"))
    }
}

pub fn read_request() -> Result<Request<HttpRequestBody>, IOError> {
    let mut request = Request::new(HttpRequestBody { _v: () });

    let method = wasp_core::request::read_method().unwrap();
    *request.method_mut() = HttpTryFrom::try_from(method.as_str()).unwrap();

    let headers = request.headers_mut();

    for (name, value) in wasp_core::request::read_headers() {
        headers.append::<HeaderName>(
            HttpTryFrom::try_from(name.as_str()).unwrap(),
            HttpTryFrom::try_from(value).unwrap(),
        );
    }

    let uri = wasp_core::request::read_uri().unwrap();
    *request.uri_mut() = uri.parse().unwrap();

    Ok(request)
}

struct HttpClientRequestBody<'a> {
    request: &'a mut wasp_core::http::Request,
}

impl<'a> Write for HttpClientRequestBody<'a> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, IOError> {
        self.request
            .write_body(bytes)
            .map_err(|_| IOError::new(IOErrorKind::BrokenPipe, "Could not read body"))
    }

    fn flush(&mut self) -> Result<(), IOError> {
        Ok(())
    }
}

pub struct HttpClientResponseBody {
    response: wasp_core::http::Request,
}

impl Read for HttpClientResponseBody {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize, IOError> {
        self.response
            .read_body(bytes)
            .map_err(|_| IOError::new(IOErrorKind::BrokenPipe, "Could not read body"))
    }
}

impl Into<Vec<u8>> for HttpClientResponseBody {
    fn into(mut self) -> Vec<u8> {
        let mut bytes = vec![];
        self.read_to_end(&mut bytes).unwrap();
        bytes
    }
}

impl HttpMessage for Request<()> {
    type Response = Result<Response<HttpClientResponseBody>, IOError>;

    fn send(self) -> Self::Response {
        let (head, _) = self.into_parts();
        Request::from_parts(head, std::io::Cursor::new(vec![])).send()
    }
}

#[macro_export]
macro_rules! impl_cursor_request {
    ($ty:ty) => {
        impl_cursor_request!($ty where);
    };
    ($ty:ty where $($params:tt)*) => {
        impl<$($params)*> HttpMessage for Request<$ty> {
            type Response = Result<Response<HttpClientResponseBody>, IOError>;

            fn send(self) -> Self::Response {
                let (head, body) = self.into_parts();
                Request::from_parts(head, std::io::Cursor::new(body)).send()
            }
        }
    }
}

impl_cursor_request!(String);
impl_cursor_request!(&str);
impl_cursor_request!(Vec<u8>);
impl_cursor_request!(&[u8]);

#[macro_export]
macro_rules! impl_reader_request {
    ($ty:ty) => {
        impl_reader_request!($ty where );
    };
    ($ty:ty where $($params:tt)*) => {
        impl<$($params)*> HttpMessage for Request<$ty> {
            type Response = Result<Response<HttpClientResponseBody>, IOError>;

            fn send(self) -> Self::Response {
                use wasp_core::http::Request;
                let mut req =
                    Request::open(self.method().as_str(), &format!("{}", self.uri())).unwrap();
                for (name, value) in self.headers() {
                    req.write_header(name.as_str(), value.to_str().unwrap())
                        .unwrap();
                }
                req.send_head().unwrap();

                copy(
                    &mut self.into_body(),
                    &mut HttpClientRequestBody { request: &mut req },
                )?;
                req.send_body().unwrap();

                let status_code = req.read_status_code().unwrap();
                let res_headers: Vec<_> = req.read_headers().unwrap().collect();
                let mut response = Response::new(HttpClientResponseBody { response: req });

                *response.status_mut() = HttpTryFrom::try_from(status_code as u16).unwrap();

                let headers = response.headers_mut();
                for (name, value) in res_headers {
                    headers.append::<HeaderName>(
                        HttpTryFrom::try_from(name.as_str()).unwrap(),
                        HttpTryFrom::try_from(value).unwrap(),
                    );
                }

                Ok(response)
            }
        }
    };
}

impl_reader_request!(std::io::Cursor<T> where T: AsRef<[u8]>);
impl_reader_request!(HttpRequestBody);
impl_reader_request!(HttpClientResponseBody);

struct ResponseBody {}

impl Write for ResponseBody {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, IOError> {
        wasp_core::response::write_body(bytes)
            .map_err(|_| IOError::new(IOErrorKind::BrokenPipe, "Could not write body"))
    }

    fn flush(&mut self) -> Result<(), IOError> {
        wasp_core::response::end_body()
            .map_err(|_| IOError::new(IOErrorKind::BrokenPipe, "Could not write body"))
    }
}

impl<B: Read> HttpMessage for Response<B> {
    type Response = Result<(), IOError>;

    fn send(self) -> Self::Response {
        wasp_core::response::write_status_code(self.status().as_u16()).unwrap();

        for (name, value) in self.headers() {
            wasp_core::response::write_header(name.as_str(), value.to_str().unwrap()).unwrap();
        }

        wasp_core::response::end_head().unwrap();

        copy(&mut self.into_body(), &mut ResponseBody {})?;

        wasp_core::response::end_body().unwrap();

        Ok(())
    }
}
