//! 🚪️ IO s.raster (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.pdf", "stdio.png", "stdio.svg", "stdio.tiff"] }
