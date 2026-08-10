//! 🚪️ IO s.block3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] { &["stdio.glb", "stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.txt", "stdio.zip"] }
pub fn export_stdio_kinds() -> &'static [&'static str] { &["stdio.glb", "stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.txt", "stdio.zip"] }
