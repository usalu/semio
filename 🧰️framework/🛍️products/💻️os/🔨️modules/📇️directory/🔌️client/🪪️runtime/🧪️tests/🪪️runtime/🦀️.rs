use super::*;
use semio_framework_async::{ProcessKind, ScopeOwner, WorkerPool, WorkerPoolConfig};

//#region 🪪️RuntimeIdentity
#[test]
fn directory_native_runtime_identity_uses_the_services_owned_constructor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let _: fn(Arc<ComputePool>, Arc<TokioHostRuntime>, ScopeHandle) -> UreqStreamingHttpTransport = UreqStreamingHttpTransport::new;
    assert_eq!(std::any::TypeId::of::<TokioHostRuntime>(), std::any::TypeId::of::<TokioHostRuntime>());
    assert_eq!(fixture["provider"], "semio_framework_os_services::TokioHostRuntime");
}

#[semio_framework_async_macros::async_test]
async fn directory_native_runtime_identity_preserves_original_injected_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let workers = usize::try_from(case["workers"].as_u64().unwrap()).unwrap();
        let capacity = u32::try_from(case["computeCapacity"].as_u64().unwrap()).unwrap();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, workers));
        let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
        let foreign = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
        let compute = Arc::new(ComputePool::with_pool(capacity, pool.clone()));
        let scope = runtime.open_scope_now(ScopeOwner::Service("directory-runtime-identity"), None);
        let transport = UreqStreamingHttpTransport::new(compute.clone(), runtime.clone(), scope.clone());
        let observed = (Arc::ptr_eq(&transport.runtime, &runtime), Arc::ptr_eq(&transport.runtime, &foreign), Arc::ptr_eq(&transport.compute, &compute), transport.scope.id == scope.id, pool.worker_count());
        drop(transport);
        drop(compute);
        let drain = runtime.cancel_scope(&scope.owner, 0).await;
        drop(scope);
        drop(foreign);
        drop(runtime);
        pool.shutdown().expect("released directory fixture pool shuts down");
        assert_eq!(observed, (true, false, true, true, workers), "{case}");
        assert_eq!((drain.finished, drain.cancelled, drain.leaked), (0, 0, 0));
    }
}
//#endregion 🪪️RuntimeIdentity

//#region ⏱️RequestBudget
/// ⏱️ The per-request budget follows the caller's deadline (shared fixture `⏱️request-budget.json`).
#[test]
fn every_request_budget_is_what_the_callers_deadline_leaves() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⏱️request-budget.json")).unwrap();
    assert_eq!(fixture["unboundedRequestMs"], UREQ_HTTP_UNBOUNDED_REQUEST_MS);
    for case in fixture["cases"].as_array().unwrap() {
        assert_eq!(ureq_request_budget_ms(case["deadlineMs"].as_u64(), case["nowMs"].as_u64().unwrap()), case["budgetMs"].as_u64().unwrap(), "{case}");
    }
}

/// 🐢️ A hub whose answer head takes `delay` arrives inside a longer caller deadline and is refused past a
/// shorter one — the transport's bound is the caller's, never a fixed cap of its own.
#[semio_framework_async_macros::async_test]
async fn a_slow_answer_arrives_inside_the_callers_deadline_and_not_after_it() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/slow", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        for _ in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            std::thread::sleep(Duration::from_millis(600));
            let _ = std::io::Write::write_all(&mut stream, b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok");
        }
    });
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("directory-request-budget"), None);
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime.clone(), scope.clone(), compute.clone(), 1 << 20, 2, PackageId("os.directory-client".into()), ActorId(0));
    let context = |budget_ms: u64, now_ms: u64| OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: Some(now_ms + budget_ms), cancel: semio_framework_async::CancelToken::root_now(), capability: None };
    let now = runtime.now_ms().await;
    let inside = transport.http(&context(5_000, now), HttpMethod::Get, &url, None, None).await;
    let now = runtime.now_ms().await;
    let past = transport.http(&context(200, now), HttpMethod::Get, &url, None, None).await;
    server.join().unwrap();
    assert_eq!(inside.map(|response| (response.status, response.body)), Ok((200, b"ok".to_vec())));
    assert!(past.is_err(), "a 600 ms head past a 200 ms deadline is refused");
    drop(transport);
    drop(compute);
    let _ = runtime.cancel_scope(&scope.owner, 0).await;
    drop(scope);
    drop(runtime);
    pool.shutdown().expect("request budget pool shuts down");
}
//#endregion ⏱️RequestBudget

//#region 🔁️ByteBudgetRefill
/// 🔁️ A directory transport's byte budget is an allowance per refill turn, never a lifetime one: once exhausted, the
/// transport reads it as spent and refuses with the typed budget refusal a caller waits on, and the pool's own driver
/// admits requests again on its next turn (shared
/// fixture `🔁️byte-budget-refill.json`; the semio MCP gateway wedged on the lifetime budget, ticket 26/09/23 14b).
#[semio_framework_async_macros::async_test]
async fn an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️byte-budget-refill.json")).unwrap();
    assert_eq!(fixture["productionRefillIntervalMs"], HTTP_BUCKET_REFILL_INTERVAL_MS);
    let body = fixture["body"].as_str().unwrap().to_string();
    let admitted = usize::try_from(fixture["admittedRequests"].as_u64().unwrap()).unwrap();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let url = format!("http://{address}/budget");
    let answer = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
    let server = std::thread::spawn(move || {
        for _ in 0..admitted {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0u8; 1024];
            let _ = stream.read(&mut request);
            let _ = std::io::Write::write_all(&mut stream, answer.as_bytes());
        }
    });
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 2));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("directory-byte-budget-refill"), None);
    let budget = u64::try_from(url.len() + body.len()).unwrap();
    let http_pool = refilled_http_pool(&runtime, &scope, compute.clone(), budget, 2, fixture["refillIntervalMs"].as_u64().unwrap());
    let transport = NativeDirectoryTransport::new_now(runtime.clone(), scope.clone(), http_pool, PackageId("os.directory-client".into()), ActorId(0));
    let context = || OperationContext { actor: 0, generation: 0, trace: semio_framework_async::TraceId(0), lane: 0, deadline_ms: None, cancel: semio_framework_async::CancelToken::root_now(), capability: None };
    assert_eq!(transport.network_budget(), semio_framework_os_services::HttpPackageBudget { remaining_bytes: budget, capacity_bytes: budget });
    let first = transport.http(&context(), HttpMethod::Get, &url, None, None).await;
    let spent = transport.network_budget();
    let refused = transport.http(&context(), HttpMethod::Get, &url, None, None).await;
    let started = std::time::Instant::now();
    let refilled = loop {
        match transport.http(&context(), HttpMethod::Get, &url, None, None).await {
            Ok(response) => break Some(response),
            Err(_) if started.elapsed() < Duration::from_secs(fixture["refillBoundSeconds"].as_u64().unwrap()) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => break None,
        }
    };
    if refilled.is_none() {
        let _ = std::net::TcpStream::connect(address);
    }
    server.join().unwrap();
    assert_eq!(first.map(|response| (response.status, response.body)), Ok((200, body.as_bytes().to_vec())));
    assert_eq!(spent, semio_framework_os_services::HttpPackageBudget { remaining_bytes: 0, capacity_bytes: budget }, "one request spent the whole budget");
    assert_eq!(fixture["refusal"], "budget-exhausted");
    assert!(matches!(&refused, Err(TransportError::BudgetExhausted)), "an exhausted budget is its own typed refusal: {refused:?}");
    assert_eq!(refilled.map(|response| (response.status, response.body)), Some((200, body.as_bytes().to_vec())), "the pool's own refill turn admits requests again");
    drop(transport);
    drop(compute);
    let _ = runtime.cancel_scope(&scope.owner, 0).await;
    drop(scope);
    drop(runtime);
    pool.shutdown().expect("byte budget pool shuts down");
}
//#endregion 🔁️ByteBudgetRefill
