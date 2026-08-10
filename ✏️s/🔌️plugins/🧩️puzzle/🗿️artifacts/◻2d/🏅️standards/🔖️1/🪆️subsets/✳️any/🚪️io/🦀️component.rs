//! 🚪️ IO s.puzzle2d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.glb", "stdio.json", "stdio.obj", "stdio.pdf", "stdio.png", "stdio.stl", "stdio.svg", "stdio.txt", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.dwg", "stdio.dxf", "stdio.glb", "stdio.json", "stdio.obj", "stdio.pdf", "stdio.png", "stdio.stl", "stdio.svg", "stdio.txt", "stdio.zip"] }
