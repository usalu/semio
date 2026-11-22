//! 📥 Drives only bounded native journal windows through the actual Shell lease.
use protocol::channel::archive_ingress::DocumentArchiveIngressSeal;
use protocol::channel::archive_ingress::folder_archive_load::{FolderArchiveLoadAction as Action, FolderArchiveLoadKey, FolderArchiveLoadReply as Reply, FolderArchiveLoadRequest as Request};
use std::future::Future;
use store_sync::sync::{FolderEventLogStorage, NativeFolderArchiveSource, FOLDER_ARCHIVE_PAGE_BYTES};

/// 📐 Native startup Folder archive wire capacity, distinct from mounted Pack and history decoding.
pub const NATIVE_FOLDER_ARCHIVE_REOPEN_MAXIMUM_BYTES: usize = 8 * 1024 * 1024;

pub async fn restore(
    storage: &FolderEventLogStorage,
    mut source: NativeFolderArchiveSource,
    mut key: FolderArchiveLoadKey,
    document: &str,
    schema: &str,
    cancelled: impl Fn() -> bool,
    mut exchange: impl for<'r> FnMut(Request<'r>) -> std::pin::Pin<Box<dyn Future<Output = Result<Reply, String>> + Send + 'r>>,
) -> Result<(), String> {
    let selected = source.seal();
    let seal = DocumentArchiveIngressSeal { bytes: selected.extent, sha256: selected.sha256 };
    let result = async {
        loop {
            if cancelled() { return Err("folder archive reopen cancelled".into()); }
            let request = Request { key, action: Action::Begin { seal, document, schema } };
            match exchange(request).await? {
                Reply::Pending => continue,
                Reply::Admitted { activation, lifetime } => {
                    key.activation = activation;
                    key.lifetime = lifetime;
                    break;
                }
                _ => return Err("folder archive reopen admission refused".into()),
            }
        }

        let mut page = [0; FOLDER_ARCHIVE_PAGE_BYTES];
        let mut offset = 0;
        while offset < selected.extent {
            let count = source.read_next(&mut page, &cancelled).map_err(|error| error.to_string())?;
            if count == 0 { return Err("folder archive record ended before its committed extent".into()); }
            let mut consumed = 0;
            while consumed < count {
                if cancelled() { return Err("folder archive reopen cancelled".into()); }
                let request = Request { key, action: Action::Page {
                    offset: (offset + consumed) as u32, bytes: &page[consumed..count],
                }};
                match exchange(request).await? {
                    Reply::Received { through } if (offset + consumed + 1..=offset + count).contains(&(through as usize)) => {
                        consumed = through as usize - offset;
                    }
                    Reply::Pending => {}
                    _ => return Err("folder archive reopen refused its exact pending page".into()),
                }
            }
            offset += count;
        }

        loop {
            if cancelled() { return Err("folder archive reopen cancelled".into()); }
            match exchange(Request { key, action: Action::Finish }).await? {
                Reply::Pending => {}
                Reply::Verified => break,
                _ => return Err("folder archive reopen failed to verify its durable seal".into()),
            }
        }

        if !storage.archive_source_is_current(document, schema, selected, NATIVE_FOLDER_ARCHIVE_REOPEN_MAXIMUM_BYTES, &cancelled)
            .map_err(|error| error.to_string())?
        {
            return Err("folder archive was superseded during reopen".into());
        }
        loop {
            if cancelled() { return Err("folder archive reopen cancelled".into()); }
            match exchange(Request { key, action: Action::Publish }).await? {
                Reply::Pending => {}
                Reply::Submitted => return Ok(()),
                _ => return Err("folder archive could not publish its hydrated document".into()),
            }
        }
    }.await;
    if result.is_err() && key.lifetime != 0 {
        loop {
            match exchange(Request { key, action: Action::Cancel }).await {
                Ok(Reply::Pending) => {}
                Ok(Reply::Closed) => break,
                _ => break,
            }
        }
    }
    result
}
