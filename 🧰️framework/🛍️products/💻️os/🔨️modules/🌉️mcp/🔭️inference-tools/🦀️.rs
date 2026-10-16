//! 🔭️ Inference discovery, jobs and resources.

extern crate semio_framework_os_kernel as store;

#[path = "../💡️inference/🦀️.rs"]
pub mod inference;

#[cfg(test)]
#[path = "../💡️inference/🧪️tests/🔬️quick/🦀️.rs"]
pub mod inference_quick;

#[path = "../💡️inference/💼️jobs/🦀️.rs"]
pub mod jobs;

#[cfg(test)]
#[path = "../💡️inference/🧪️tests/💼️inference-service-law/🦀️.rs"]
pub mod jobs_inference_service_law;
