//! 🖼️ Render side of the mounted artifact instance: composition graph, interaction state, render, media jobs and media export.
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[path = "📥️staging/📜️history-view/🦀️.rs"]
pub mod render_history_view;
#[path = "📥️staging/🛠️tool-machine/🦀️.rs"]
pub mod render_tool_machine;
#[path = "📥️staging/🧭️interaction/🦀️.rs"]
pub mod render_interaction;
#[path = "📥️staging/🏷️reserved-jobs/🦀️.rs"]
pub mod render_reserved_jobs;
#[path = "📥️staging/🧺️bundle/🦀️.rs"]
pub mod render_bundle;
