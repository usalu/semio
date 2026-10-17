//! 🔭 Black-box laws of the arena-backed B-Rep engine through its public surface: the sync API, the operation jobs, the shape value and the
//! session analysis. The white-box laws that need the engine's private session state stay in `semio-framework-3d-brep-engine`.

#[cfg(test)]
#[path = "🔭️api/🦀️.rs"]
mod api;

#[cfg(test)]
#[path = "⏱️operation-jobs/🦀️.rs"]
mod operation_jobs;

#[cfg(test)]
#[path = "🪅️shape-values/🦀️.rs"]
mod shape_values;

#[cfg(test)]
#[path = "🔎️analysis/🦀️.rs"]
mod analysis;
