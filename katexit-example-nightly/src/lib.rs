#![feature(prelude_import, custom_inner_attributes)]
#![cfg_attr(doc, katexit::katexit)]
//! Module level document example with $\KaTeX$!

#[cfg_attr(doc, katexit::katexit)]
/// We can write $\LaTeX$ expressions
///
/// Display style
/// -------------
///
/// $$
/// c = \\pm\\sqrt{a^2 + b^2}
/// $$
pub fn my_func() {}
