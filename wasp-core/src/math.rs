use wasp_abi::math;

#[inline]
pub fn acos(n: f64) -> f64 {
    unsafe { math::acos(n) }
}

#[inline]
pub fn acosf(n: f32) -> f32 {
    unsafe { math::acosf(n) }
}

#[inline]
pub fn asin(n: f64) -> f64 {
    unsafe { math::asin(n) }
}

#[inline]
pub fn asinf(n: f32) -> f32 {
    unsafe { math::asinf(n) }
}

#[inline]
pub fn atan(n: f64) -> f64 {
    unsafe { math::atan(n) }
}

#[inline]
pub fn atan2(a: f64, b: f64) -> f64 {
    unsafe { math::atan2(a, b) }
}

#[inline]
pub fn atan2f(a: f32, b: f32) -> f32 {
    unsafe { math::atan2f(a, b) }
}

#[inline]
pub fn atanf(n: f32) -> f32 {
    unsafe { math::atanf(n) }
}

#[inline]
pub fn cbrt(n: f64) -> f64 {
    unsafe { math::cbrt(n) }
}

#[inline]
pub fn cbrtf(n: f32) -> f32 {
    unsafe { math::cbrtf(n) }
}

#[inline]
pub fn cosh(n: f64) -> f64 {
    unsafe { math::cosh(n) }
}

#[inline]
pub fn coshf(n: f32) -> f32 {
    unsafe { math::coshf(n) }
}

#[inline]
pub fn expm1(n: f64) -> f64 {
    unsafe { math::expm1(n) }
}

#[inline]
pub fn expm1f(n: f32) -> f32 {
    unsafe { math::expm1f(n) }
}

#[inline]
pub fn fdim(a: f64, b: f64) -> f64 {
    unsafe { math::fdim(a, b) }
}

#[inline]
pub fn fdimf(a: f32, b: f32) -> f32 {
    unsafe { math::fdimf(a, b) }
}

#[inline]
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { math::hypot(x, y) }
}

#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    unsafe { math::hypotf(x, y) }
}

#[inline]
pub fn log1p(n: f64) -> f64 {
    unsafe { math::log1p(n) }
}

#[inline]
pub fn log1pf(n: f32) -> f32 {
    unsafe { math::log1pf(n) }
}

#[inline]
pub fn sin(n: f64) -> f64 {
    unsafe { math::sin(n) }
}

#[inline]
pub fn sinf(n: f32) -> f32 {
    unsafe { math::sinf(n) }
}

#[inline]
pub fn sinh(n: f64) -> f64 {
    unsafe { math::sinh(n) }
}

#[inline]
pub fn sinhf(n: f32) -> f32 {
    unsafe { math::sinhf(n) }
}

#[inline]
pub fn tan(n: f64) -> f64 {
    unsafe { math::tan(n) }
}

#[inline]
pub fn tanf(n: f32) -> f32 {
    unsafe { math::tanf(n) }
}

#[inline]
pub fn tanh(n: f64) -> f64 {
    unsafe { math::tanh(n) }
}

#[inline]
pub fn tanhf(n: f32) -> f32 {
    unsafe { math::tanhf(n) }
}
