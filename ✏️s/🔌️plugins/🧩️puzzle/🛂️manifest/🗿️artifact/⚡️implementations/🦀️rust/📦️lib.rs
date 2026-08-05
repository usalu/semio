//! 🧩️ Puzzle plugin — 2D, 3D, and 5D play apps in one hot-swappable WASM plugin.

fn register_puzzle_exports() {
    puzzle_2d_ui::register_puzzle2d_exports();
    puzzle_3d_ui::register_puzzle3d_exports();
    puzzle_5d_ui::register_puzzle5d_exports();
}

semio_framework_plugin::semio_plugin! {
    id: "puzzle",
    label: "Puzzle",
    version: "0.1.0",
    setup: register_puzzle_exports,
    apps: [
        puzzle_2d_ui::create_puzzle2d_app => puzzle_2d_ui::Puzzle2dPlayApp,
        puzzle_3d_ui::create_puzzle3d_app => puzzle_3d_ui::Puzzle3dPlayApp,
        puzzle_5d_ui::create_puzzle5d_app => puzzle_5d_ui::Puzzle5dPlayApp,
    ]
}
