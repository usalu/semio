//! 📺️ tui terminal I/O: ANSI codec, backend capabilities, native raw-mode backend and pty (feature `native`).

#[path = "../🔡️ansi/🦀️.rs"]
pub mod ansi;

#[path = "../🔌️backend/🦀️.rs"]
pub mod backend;

#[path = "../🚇️pty/🦀️.rs"]
#[cfg(feature = "native")]
pub mod pty;

#[path = "../🪟️windows/🦀️.rs"]
#[cfg(all(feature = "native", windows))]
mod windows_abi;
