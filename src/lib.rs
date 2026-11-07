#![no_std]
#![forbid(unsafe_code)]

//! A crate for things you don't care about.
//!
//! Explicitly discard values. Suppress nothing else.

/// Consume and immediately drop a value.
///
/// Unlike `swallow!`, this moves the value out of its place.
#[inline]
pub fn swallow<T>(value: T) {
    core::mem::drop(value);
}

/// Evaluate an expression and explicitly ignore its value.
///
/// If the expression is a place expression, it does not move
/// the value out of that place.
///
/// Otherwise, the resulting temporary is discarded.
#[macro_export]
macro_rules! swallow {
    ($value:expr $(,)?) => {{
        let _ = $value;
    }};
}
