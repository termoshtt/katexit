[![crate](https://img.shields.io/badge/crates.io-katexit-blue)](https://crates.io/crates/katexit)
[![docs.rs](https://docs.rs/katexit/badge.svg)](https://docs.rs/katexit)

Insert KaTeX autorender script into rustdoc

Usage
-----

```rust
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
```

See the rendered result on [docs.rs](https://docs.rs/katexit-example/0.1.0/katexit_example/fn.my_func.html)
