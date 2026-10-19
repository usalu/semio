//! 🌐️ Native asset stream: the file and http page streams that feed a world asset fetch; mailbox transport of the runtime-mailbox crate of the renderer partition (plan section 5.3).
use super::*;

//#region 📮️RuntimeMailbox
#[cfg(not(target_arch = "wasm32"))]
fn native_renderer_asset_path(url: &str) -> Result<RendererIoPath<'_>, String> {
    if url.starts_with("/mesh/") {
        let source = &mesh_assets::resolve_mesh_asset(url)?.source;
        return Ok(RendererIoPath::new([env!("CARGO_MANIFEST_DIR"),"../../../../../../../../../../..",source]));
    }
    if let Some(path) = url.strip_prefix("file://") {
        return Ok(RendererIoPath::new([path,"",""]));
    }
    if url.starts_with('/') {
        return Ok(RendererIoPath::new([env!("CARGO_MANIFEST_DIR"),"../../../../..",url.trim_start_matches('/')]));
    }
    Ok(RendererIoPath::new([url,"",""]))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn stream_native_renderer_asset<R: MailboxRuntime>(mailbox: &RuntimeMailbox<R>, fetch: &mut RendererAssetFetchOwner) -> Result<(), NativeAssetStreamFailure> {
    if fetch.url().starts_with("http://") || fetch.url().starts_with("https://") {
        return stream_native_renderer_http_asset(mailbox, fetch).await;
    }
    let opened={let path=native_renderer_asset_path(fetch.url()).map_err(NativeAssetStreamFailure::Unavailable)?;let mut open=open_renderer_source_io(path);std::future::poll_fn(|context|{
        if mailbox.renderer_asset_cancelled(fetch){return std::task::Poll::Ready(Err(None))}
        match std::pin::Pin::new(&mut open).poll(context){std::task::Poll::Ready(answer)=>std::task::Poll::Ready(answer.map_err(Some)),std::task::Poll::Pending=>std::task::Poll::Pending}
    }).await};
    let mut source=match opened{Ok(source)=>source,Err(Some(error))=>return Err(NativeAssetStreamFailure::Original(error)),Err(None)=>return Err(NativeAssetStreamFailure::Cancelled)};
    loop {
        if mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let next={let mut next=source.next_page();std::future::poll_fn(|context|{
            if mailbox.renderer_asset_cancelled(fetch){return std::task::Poll::Ready(Err(None))}
            match std::pin::Pin::new(&mut next).poll(context){std::task::Poll::Ready(answer)=>std::task::Poll::Ready(answer.map_err(Some)),std::task::Poll::Pending=>std::task::Poll::Pending}
        }).await};
        let page=match next{Ok(Some(page))=>page,Ok(None)=>break,Err(Some(kind))=>return Err(NativeAssetStreamFailure::Original(source.failure(kind))),Err(None)=>return Err(NativeAssetStreamFailure::Cancelled)};
        #[cfg(any(test, feature = "testkit"))]
        mailbox.wait_native_asset_page_test_barrier(fetch);
        if mailbox.renderer_asset_cancelled(fetch) {
            drop(page);return Err(NativeAssetStreamFailure::Cancelled);
        }
        let ordinal=page.original_page_index() as u16;
        let page=WorldAssetResponsePage::try_from_original(page).expect("the original native and World page extents agree");
        if let Err(page)=fetch.owner_mut().push_page_at(ordinal,page){
            drop(page);return Err(NativeAssetStreamFailure::Original(source.failure(RendererSourceIoFailureKind::RecipientRefused)));
        }
    }
    Ok(())
}



#[cfg(not(target_arch = "wasm32"))]
async fn stream_native_renderer_http_asset<R: MailboxRuntime>(mailbox: &RuntimeMailbox<R>, fetch: &mut RendererAssetFetchOwner) -> Result<(), NativeAssetStreamFailure> {
    let cancel = mailbox.0.native_asset_http_cancel.child_now();
    let context = semio_framework_async::OperationContext {
        actor: 0,
        generation: 0,
        trace: semio_framework_async::TraceId(0),
        lane: semio_framework_async::Lane::Io as u8,
        deadline_ms: Some(semio_framework_job::default_now_ms().and_then(|now| now.checked_add(15_000)).ok_or_else(|| "native asset deadline requires a real clock and checked range".to_string())?),
        cancel: cancel.clone(),
        capability: None,
    };
    let request = semio_framework_os_services::HttpRequest { method: "GET".into(), url: fetch.url().to_owned(), headers: Vec::new(), body: Vec::new() };
    let pool = if fetch.url().starts_with("https://") { &mailbox.0.native_asset_https } else { &mailbox.0.native_asset_http };
    let mut transport = None;
    let response = pool
        .fetch_started(mailbox.0.native_asset_http_runtime.as_ref(), &mailbox.0.native_asset_http_scope, context, semio_framework_actor::PackageId("os.renderer.asset".into()), semio_framework_actor::ActorId(0), request, |cancellation| {
            transport = Some(mailbox.retain_native_asset_transport(fetch, cancel.clone(), cancellation))
        })
        .await
        .map_err(|error| if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) { NativeAssetStreamFailure::Cancelled } else { NativeAssetStreamFailure::Unavailable(error.to_string()) });
    let _transport = transport;
    let (head, mut body) = response?;
    if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
        return Err(NativeAssetStreamFailure::Cancelled);
    }
    if !(200..300).contains(&head.status) {
        return Err(NativeAssetStreamFailure::Unavailable(format!("native renderer asset HTTP status {}", head.status)));
    }
    if let Some(length) = head.headers.iter().find(|(name, _)| name.eq_ignore_ascii_case("content-length")).and_then(|(_, value)| value.parse::<usize>().ok()) {
        if length > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err("native renderer asset Content-Length exceeded fixed credits".into());
        }
    }
    loop {
        if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let page = body.next_chunk().await;
        if cancel.is_cancelled_now() || mailbox.renderer_asset_cancelled(fetch) {
            return Err(NativeAssetStreamFailure::Cancelled);
        }
        let Some(bytes) = page.map_err(|error| NativeAssetStreamFailure::Unavailable(error.to_string()))? else { break };
        if bytes.is_empty() || bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err("native renderer HTTP asset returned an invalid response page".into());
        }
        let page = WorldAssetResponsePage::try_from_owned(bytes).map_err(|_| "native renderer HTTP asset page exceeded fixed credits")?;
        fetch.owner_mut().push_page(page).map_err(|_| "native renderer HTTP asset response exceeded its reserved credits")?;
        let mut yielded = false;
        std::future::poll_fn(|context| {
            if yielded {
                std::task::Poll::Ready(())
            } else {
                yielded = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        })
        .await;
    }
    Ok(())
}
//#endregion 📮️RuntimeMailbox
