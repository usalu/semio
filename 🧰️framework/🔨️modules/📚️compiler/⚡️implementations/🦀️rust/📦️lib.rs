//! 📚️ `compiler` — facade for semio's incremental document compiler, replacing Typst. Technologies
//! depend on this one crate to reach the compiler's sub-slots (`📖️syntax`, and later `🌍️world`,
//! `⚙️eval`, `📐️layout`, `🧊️wgpu`, `📤️svg`, …), added one slot at a time as each wave lands.
//! Currently re-exports only the syntax kernel (the semio math notation lexer/parser/printer) —
//! see `compiler_syntax` for the crate that actually does the work.

pub use compiler_syntax as syntax;
