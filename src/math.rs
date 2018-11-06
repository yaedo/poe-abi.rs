mod raw {
    #[link(wasm_import_module = "/poe/math")]
    extern "C" {
        #[link_name = "/poe/math/acos__1"]
        pub fn acos(n: f64) -> f64;
        #[link_name = "/poe/math/acosf__1"]
        pub fn acosf(n: f32) -> f32;
        #[link_name = "/poe/math/asin__1"]
        pub fn asin(n: f64) -> f64;
        #[link_name = "/poe/math/asinf__1"]
        pub fn asinf(n: f32) -> f32;
        #[link_name = "/poe/math/atan__1"]
        pub fn atan(n: f64) -> f64;
        #[link_name = "/poe/math/atan2__1"]
        pub fn atan2(a: f64, b: f64) -> f64;
        #[link_name = "/poe/math/atan2f__1"]
        pub fn atan2f(a: f32, b: f32) -> f32;
        #[link_name = "/poe/math/atanf__1"]
        pub fn atanf(n: f32) -> f32;
        #[link_name = "/poe/math/cbrt__1"]
        pub fn cbrt(n: f64) -> f64;
        #[link_name = "/poe/math/cbrtf__1"]
        pub fn cbrtf(n: f32) -> f32;
        #[link_name = "/poe/math/cosh__1"]
        pub fn cosh(n: f64) -> f64;
        #[link_name = "/poe/math/coshf__1"]
        pub fn coshf(n: f32) -> f32;
        #[link_name = "/poe/math/expm1__1"]
        pub fn expm1(n: f64) -> f64;
        #[link_name = "/poe/math/expm1f__1"]
        pub fn expm1f(n: f32) -> f32;
        #[link_name = "/poe/math/fdim__1"]
        pub fn fdim(a: f64, b: f64) -> f64;
        #[link_name = "/poe/math/fdimf__1"]
        pub fn fdimf(a: f32, b: f32) -> f32;
        #[link_name = "/poe/math/hypot__1"]
        pub fn hypot(x: f64, y: f64) -> f64;
        #[link_name = "/poe/math/hypotf__1"]
        pub fn hypotf(x: f32, y: f32) -> f32;
        #[link_name = "/poe/math/log1p__1"]
        pub fn log1p(n: f64) -> f64;
        #[link_name = "/poe/math/log1pf__1"]
        pub fn log1pf(n: f32) -> f32;
        #[link_name = "/poe/math/sin__1"]
        pub fn sin(n: f64) -> f64;
        #[link_name = "/poe/math/sinf__1"]
        pub fn sinf(n: f32) -> f32;
        #[link_name = "/poe/math/sinh__1"]
        pub fn sinh(n: f64) -> f64;
        #[link_name = "/poe/math/sinhf__1"]
        pub fn sinhf(n: f32) -> f32;
        #[link_name = "/poe/math/tan__1"]
        pub fn tan(n: f64) -> f64;
        #[link_name = "/poe/math/tanf__1"]
        pub fn tanf(n: f32) -> f32;
        #[link_name = "/poe/math/tanh__1"]
        pub fn tanh(n: f64) -> f64;
        #[link_name = "/poe/math/tanhf__1"]
        pub fn tanhf(n: f32) -> f32;
    }
}

#[inline]
pub fn acos(n: f64) -> f64 {
    unsafe { raw::acos(n) }
}

#[inline]
pub fn acosf(n: f32) -> f32 {
    unsafe { raw::acosf(n) }
}

#[inline]
pub fn asin(n: f64) -> f64 {
    unsafe { raw::asin(n) }
}

#[inline]
pub fn asinf(n: f32) -> f32 {
    unsafe { raw::asinf(n) }
}

#[inline]
pub fn atan(n: f64) -> f64 {
    unsafe { raw::atan(n) }
}

#[inline]
pub fn atan2(a: f64, b: f64) -> f64 {
    unsafe { raw::atan2(a, b) }
}

#[inline]
pub fn atan2f(a: f32, b: f32) -> f32 {
    unsafe { raw::atan2f(a, b) }
}

#[inline]
pub fn atanf(n: f32) -> f32 {
    unsafe { raw::atanf(n) }
}

#[inline]
pub fn cbrt(n: f64) -> f64 {
    unsafe { raw::cbrt(n) }
}

#[inline]
pub fn cbrtf(n: f32) -> f32 {
    unsafe { raw::cbrtf(n) }
}

#[inline]
pub fn cosh(n: f64) -> f64 {
    unsafe { raw::cosh(n) }
}

#[inline]
pub fn coshf(n: f32) -> f32 {
    unsafe { raw::coshf(n) }
}

#[inline]
pub fn expm1(n: f64) -> f64 {
    unsafe { raw::expm1(n) }
}

#[inline]
pub fn expm1f(n: f32) -> f32 {
    unsafe { raw::expm1f(n) }
}

#[inline]
pub fn fdim(a: f64, b: f64) -> f64 {
    unsafe { raw::fdim(a, b) }
}

#[inline]
pub fn fdimf(a: f32, b: f32) -> f32 {
    unsafe { raw::fdimf(a, b) }
}

#[inline]
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { raw::hypot(x, y) }
}

#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    unsafe { raw::hypotf(x, y) }
}

#[inline]
pub fn log1p(n: f64) -> f64 {
    unsafe { raw::log1p(n) }
}

#[inline]
pub fn log1pf(n: f32) -> f32 {
    unsafe { raw::log1pf(n) }
}

#[inline]
pub fn sin(n: f64) -> f64 {
    unsafe { raw::sin(n) }
}

#[inline]
pub fn sinf(n: f32) -> f32 {
    unsafe { raw::sinf(n) }
}

#[inline]
pub fn sinh(n: f64) -> f64 {
    unsafe { raw::sinh(n) }
}

#[inline]
pub fn sinhf(n: f32) -> f32 {
    unsafe { raw::sinhf(n) }
}

#[inline]
pub fn tan(n: f64) -> f64 {
    unsafe { raw::tan(n) }
}

#[inline]
pub fn tanf(n: f32) -> f32 {
    unsafe { raw::tanf(n) }
}

#[inline]
pub fn tanh(n: f64) -> f64 {
    unsafe { raw::tanh(n) }
}

#[inline]
pub fn tanhf(n: f32) -> f32 {
    unsafe { raw::tanhf(n) }
}
