#![no_std]

pub mod args {
    #[link(wasm_import_module = "/wasp/args")]
    extern "system" {
        #[link_name = "/wasp/args/len__1"]
        pub fn len() -> u32;

        #[link_name = "/wasp/args/get__1"]
        pub fn get(index: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;
    }
}

pub mod cdn {
    #[link(wasm_import_module = "/wasp/cdn")]
    extern "system" {
        #[link_name = "/wasp/cdn/read_static_url__1"]
        pub fn read_static_url(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/cdn/sign_protected_object__1"]
        pub fn sign_protected_object(
            path_index: u32,
            path_addr: *const u8,
            path_len: u32,
            expires_at: u64,
            valid_at: u64,
            ip_address_index: u32,
            ip_address_addr: *const u8,
            ip_address_len: u32,
            out_index: u32,
            out_addr: *mut u8,
            out_len: u32,
        ) -> u64;
    }
}

pub mod env {
    #[link(wasm_import_module = "/wasp/env")]
    extern "system" {
        #[link_name = "/wasp/env/len__1"]
        pub fn len() -> u32;

        #[link_name = "/wasp/env/key_at_index__1"]
        pub fn key_at_index(index: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/env/value_at_index__1"]
        pub fn value_at_index(index: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/env/get__1"]
        pub fn get(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;
    }
}

pub mod http {
    #[link(wasm_import_module = "/wasp/http")]
    extern "C" {
        #[link_name = "/wasp/http/open__1"]
        pub fn open(
            method_mem_index: u32,
            method_mem_addr: *const u8,
            method_mem_len: u32,
            url_mem_index: u32,
            url_mem_addr: *const u8,
            url_mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/http/write_header__1"]
        pub fn write_header(
            id: u32,
            name_index: u32,
            name_addr: *const u8,
            name_len: u32,
            value_index: u32,
            value_addr: *const u8,
            value_len: u32,
        ) -> u64;

        #[link_name = "/wasp/http/send_head__1"]
        pub fn send_head(id: u32) -> u32;

        #[link_name = "/wasp/http/write_body__1"]
        pub fn write_body(id: u32, mem_index: u32, mem_addr: *const u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/http/send_body__1"]
        pub fn send_body(id: u32) -> u32;

        #[link_name = "/wasp/http/read_status_code__1"]
        pub fn read_status_code(id: u32) -> u64;

        #[link_name = "/wasp/http/read_header_len__1"]
        pub fn read_header_len(id: u32) -> u64;

        #[link_name = "/wasp/http/read_header_name_at_index__1"]
        pub fn read_header_name_at_index(
            id: u32,
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/http/read_header_value_at_index__1"]
        pub fn read_header_value_at_index(
            id: u32,
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/http/read_header_value__1"]
        pub fn read_header_value(
            id: u32,
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/http/read_body__1"]
        pub fn read_body(id: u32, mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/http/close__1"]
        pub fn close(id: u32) -> u32;
    }
}

pub mod kvs {
    #[link(wasm_import_module = "/wasp/kvs")]
    extern "C" {
        #[link_name = "/wasp/kvs/get_open__1"]
        pub fn get_open(key_index: u32, key_addr: *const u8, key_len: u32) -> u64;

        #[link_name = "/wasp/kvs/get_read__1"]
        pub fn get_read(id: u32, memory_index: u32, memory_addr: *mut u8, memory_len: u32) -> u64;

        #[link_name = "/wasp/kvs/get_close__1"]
        pub fn get_close(id: u32) -> u32;

        #[link_name = "/wasp/kvs/list_open__1"]
        pub fn list_open(key_index: u32, key_addr: *const u8, key_len: u32) -> u64;

        #[link_name = "/wasp/kvs/list_read__1"]
        pub fn list_read(id: u32, memory_index: u32, memory_addr: *mut u8, memory_len: u32) -> u64;

        #[link_name = "/wasp/kvs/list_close__1"]
        pub fn list_close(id: u32) -> u32;

        #[link_name = "/wasp/kvs/put_open__1"]
        pub fn put_open(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            content_length: u32,
            cache_control: u32,
        ) -> u64;

        #[link_name = "/wasp/kvs/put_new_open__1"]
        pub fn put_new_open(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            content_length: u32,
            cache_control: u32,
        ) -> u64;

        #[link_name = "/wasp/kvs/put_write__1"]
        pub fn put_write(
            id: u32,
            memory_index: u32,
            memory_addr: *const u8,
            memory_len: u32,
        ) -> u64;

        #[link_name = "/wasp/kvs/put_close__1"]
        pub fn put_close(id: u32) -> u32;

        #[link_name = "/wasp/kvs/delete__1"]
        pub fn delete(key_index: u32, key_addr: *const u8, key_len: u32) -> u32;

        #[link_name = "/wasp/kvs/copy__1"]
        pub fn copy(
            from_index: u32,
            from_addr: *const u8,
            from_len: u32,
            to_index: u32,
            to_addr: *const u8,
            to_len: u32,
        ) -> u32;

        #[link_name = "/wasp/kvs/rename__1"]
        pub fn rename(
            from_index: u32,
            from_addr: *const u8,
            from_len: u32,
            to_index: u32,
            to_addr: *const u8,
            to_len: u32,
        ) -> u32;
    }
}

pub mod log {
    #[link(wasm_import_module = "/wasp/log")]
    extern "C" {
        #[link_name = "/wasp/log/write__1"]
        pub fn write(level: u32, memory_index: u32, memory_addr: *const u8, memory_len: u32)
            -> u64;
    }
}

pub mod math {
    #[link(wasm_import_module = "/wasp/math")]
    extern "C" {
        #[link_name = "/wasp/math/acos__1"]
        pub fn acos(n: f64) -> f64;
        #[link_name = "/wasp/math/acosf__1"]
        pub fn acosf(n: f32) -> f32;
        #[link_name = "/wasp/math/asin__1"]
        pub fn asin(n: f64) -> f64;
        #[link_name = "/wasp/math/asinf__1"]
        pub fn asinf(n: f32) -> f32;
        #[link_name = "/wasp/math/atan__1"]
        pub fn atan(n: f64) -> f64;
        #[link_name = "/wasp/math/atan2__1"]
        pub fn atan2(a: f64, b: f64) -> f64;
        #[link_name = "/wasp/math/atan2f__1"]
        pub fn atan2f(a: f32, b: f32) -> f32;
        #[link_name = "/wasp/math/atanf__1"]
        pub fn atanf(n: f32) -> f32;
        #[link_name = "/wasp/math/cbrt__1"]
        pub fn cbrt(n: f64) -> f64;
        #[link_name = "/wasp/math/cbrtf__1"]
        pub fn cbrtf(n: f32) -> f32;
        #[link_name = "/wasp/math/cosh__1"]
        pub fn cosh(n: f64) -> f64;
        #[link_name = "/wasp/math/coshf__1"]
        pub fn coshf(n: f32) -> f32;
        #[link_name = "/wasp/math/expm1__1"]
        pub fn expm1(n: f64) -> f64;
        #[link_name = "/wasp/math/expm1f__1"]
        pub fn expm1f(n: f32) -> f32;
        #[link_name = "/wasp/math/fdim__1"]
        pub fn fdim(a: f64, b: f64) -> f64;
        #[link_name = "/wasp/math/fdimf__1"]
        pub fn fdimf(a: f32, b: f32) -> f32;
        #[link_name = "/wasp/math/hypot__1"]
        pub fn hypot(x: f64, y: f64) -> f64;
        #[link_name = "/wasp/math/hypotf__1"]
        pub fn hypotf(x: f32, y: f32) -> f32;
        #[link_name = "/wasp/math/log1p__1"]
        pub fn log1p(n: f64) -> f64;
        #[link_name = "/wasp/math/log1pf__1"]
        pub fn log1pf(n: f32) -> f32;
        #[link_name = "/wasp/math/sin__1"]
        pub fn sin(n: f64) -> f64;
        #[link_name = "/wasp/math/sinf__1"]
        pub fn sinf(n: f32) -> f32;
        #[link_name = "/wasp/math/sinh__1"]
        pub fn sinh(n: f64) -> f64;
        #[link_name = "/wasp/math/sinhf__1"]
        pub fn sinhf(n: f32) -> f32;
        #[link_name = "/wasp/math/tan__1"]
        pub fn tan(n: f64) -> f64;
        #[link_name = "/wasp/math/tanf__1"]
        pub fn tanf(n: f32) -> f32;
        #[link_name = "/wasp/math/tanh__1"]
        pub fn tanh(n: f64) -> f64;
        #[link_name = "/wasp/math/tanhf__1"]
        pub fn tanhf(n: f32) -> f32;
    }
}

pub mod process {
    #[link(wasm_import_module = "/wasp/process")]
    extern "C" {
        #[link_name = "/wasp/process/yield__1"]
        pub fn yield_();
        #[link_name = "/wasp/process/nanosleep__1"]
        pub fn nanosleep(duration: u64);
        #[link_name = "/wasp/process/getpid__1"]
        pub fn getpid() -> u32;
        #[link_name = "/wasp/process/getppid__1"]
        pub fn getppid() -> u32;
        #[link_name = "/wasp/process/exit__1"]
        pub fn exit(_: u32) -> !;
    }
}

pub mod rand {
    #[link(wasm_import_module = "/wasp/rand")]
    extern "C" {
        #[link_name = "/wasp/rand/read__1"]
        pub fn read(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;
        #[link_name = "/wasp/rand/u32__1"]
        pub fn u32() -> u32;
        #[link_name = "/wasp/rand/u64__1"]
        pub fn u64() -> u64;
    }
}

pub mod request {
    #[link(wasm_import_module = "/wasp/request")]
    extern "C" {
        #[link_name = "/wasp/request/read_method__1"]
        pub fn read_method(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/request/read_uri__1"]
        pub fn read_uri(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/request/read_path__1"]
        pub fn read_path(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/request/read_query__1"]
        pub fn read_query(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;

        #[link_name = "/wasp/request/read_header_len__1"]
        pub fn read_header_len() -> u32;

        #[link_name = "/wasp/request/read_header_name_at_index__1"]
        pub fn read_header_name_at_index(
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/request/read_header_value_at_index__1"]
        pub fn read_header_value_at_index(
            index: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/request/read_header_value__1"]
        pub fn read_header_value(
            key_index: u32,
            key_addr: *const u8,
            key_len: u32,
            mem_index: u32,
            mem_addr: *mut u8,
            mem_len: u32,
        ) -> u64;

        #[link_name = "/wasp/request/read_body__1"]
        pub fn read_body(mem_index: u32, mem_addr: *mut u8, mem_len: u32) -> u64;
    }
}

pub mod response {
    #[link(wasm_import_module = "/wasp/response")]
    extern "C" {
        #[link_name = "/wasp/response/write_status_code__1"]
        pub fn write_status_code(code: u32) -> u32;

        #[link_name = "/wasp/response/write_header__1"]
        pub fn write_header(
            name_index: u32,
            name_addr: *const u8,
            name_len: u32,
            value_index: u32,
            value_addr: *const u8,
            value_len: u32,
        ) -> u64;

        #[link_name = "/wasp/response/end_head__1"]
        pub fn end_head() -> u32;

        #[link_name = "/wasp/response/write_body__1"]
        pub fn write_body(data_index: u32, data_addr: *const u8, data_len: u32) -> u64;

        #[link_name = "/wasp/response/end_body__1"]
        pub fn end_body() -> u32;
    }
}

pub mod time {
    #[link(wasm_import_module = "/wasp/time")]
    extern "C" {
        #[link_name = "/wasp/time/monotonic_resolution__1"]
        pub fn monotonic_resolution() -> u64;
        #[link_name = "/wasp/time/monotonic_now__1"]
        pub fn monotonic_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;

        #[link_name = "/wasp/time/cpu_resolution__1"]
        pub fn cpu_resolution() -> u64;
        #[link_name = "/wasp/time/cpu_now__1"]
        pub fn cpu_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;

        #[link_name = "/wasp/time/os_resolution__1"]
        pub fn os_resolution() -> u64;
        #[link_name = "/wasp/time/os_now__1"]
        pub fn os_now(
            precision_index: u32,
            precision: *mut u64,
            timestamp_index: u32,
            timestamp_secs: *mut u64,
            timestamp_sub_index: u32,
            timestamp_subsecs: *mut u32,
        ) -> u32;
    }
}
