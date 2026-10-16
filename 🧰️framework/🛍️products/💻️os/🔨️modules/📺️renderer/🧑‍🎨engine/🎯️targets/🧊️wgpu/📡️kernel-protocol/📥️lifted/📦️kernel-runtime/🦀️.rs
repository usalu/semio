//! 📦️ Reads one completed component into the compiler's contiguous input page by page.
//!
//! 🧭️ Lifted out of the kernel runtime root into kernel-protocol.

use super::*;

//#region 🎠️KernelRuntime
/// 🧩️ Reads one completed component into the compiler's contiguous input page by page. The
/// mounted native I/O authority answers at most one `JOB_PAYLOAD_PAGE_BYTES` page per request, so a
/// whole-file read refused every real guest. Local unoptimized components use the same
/// 256 MiB input ceiling as descriptor emission; network execution-target admission is separate.
pub async fn read_native_component(path: &std::path::Path) -> Result<Vec<u8>, String> {
    let bound = 256 * 1024 * 1024;
    let mut component = Vec::new();
    let mut offset = 0u64;
    loop {
        let value = crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage { path: path.to_path_buf(), offset, max_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES }).await?;
        let semio_framework_os_services::NativeIoValue::Page { mut bytes, eof } = value else {
            return Err("kernel: native I/O returned the wrong value for a component page".into());
        };
        let count = bytes.len() as u64;
        offset = offset.saturating_add(count);
        if offset <= bound {
            (0..bytes.page_count()).filter_map(|index| bytes.page(index)).for_each(|page| component.extend_from_slice(page));
        }
        while !bytes.terminal_is_empty() {
            let demand = bytes.retirement_demands().map_err(|error| format!("kernel: component page demand refused: {error}"))?;
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            bytes.close_step(grant).map_err(|error| format!("kernel: component page close refused: {error}"))?;
        }
        if offset > bound || (count == 0 && !eof) {
            return Err(format!("kernel: component {} exceeds {bound} bytes or produced an empty nonterminal page", path.display()));
        }
        if eof {
            return Ok(component);
        }
    }
}
//#endregion 🎠️KernelRuntime
