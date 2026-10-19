//! 📦️ Reads one completed component into the compiler's contiguous input page by page.
//!
//! 🧭️ Lifted out of the kernel runtime root into kernel-protocol.

use super::*;

//#region 🎠️KernelRuntime
/// 🧩️ The authentic startup source retains one exact contiguous component through compilation and paid return.
pub async fn read_native_component(path:&std::path::Path)->Result<semio_framework_job::OriginalSuppliedJobBytes,crate::RendererSourceIoError>{crate::read_renderer_source_bytes(path,256*1024*1024).await}
//#endregion 🎠️KernelRuntime
