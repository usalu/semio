//! 🪟️ The wgpu Shell's tree window observer and scheduler — the host half of ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING
//! packet P5 on this renderer. After every retained body finishes a paint the Shell measures its windowed containers
//! (`Ui::tree_window_measures`), asks the ONE request rule (`ui_wgpu::wgpu::tree_window::served_requests`) which rows the viewport
//! needs, and hands the answer to [`TreeWindowScheduler`] — the Rust twin of React's `createTreeWindowSchedulerV1`
//! (`🛠️ShellHelpers/🟦️.tsx`): windows coalesce on a trailing debounce, at most one refresh per body is in flight, and a report
//! that changes nothing schedules nothing. The scheduled state crosses to guests as `ViewModel.tree_windows`/`tree_viewport_rows`
//! (the fields React stamps), and a shell-owned windowed panel reads it back through [`TreeWindowScheduler::window_of`].
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use ui_wgpu::wgpu::tree_window::{served_requests, TreeWindowRequest, TreeWindowServedMemory};

