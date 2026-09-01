use super::*;
use semio_framework_async::{ProcessKind, ScopeOwner, WorkerPool, WorkerPoolConfig};

//#region 🪪️RuntimeIdentity
#[test]
fn directory_native_runtime_identity_uses_the_services_owned_constructor() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let _: fn(Arc<ComputePool>, Arc<semio_framework_os_services::TokioHostRuntime>, ScopeHandle) -> UreqStreamingHttpTransport = UreqStreamingHttpTransport::new;
    assert_eq!(std::any::TypeId::of::<TokioHostRuntime>(), std::any::TypeId::of::<semio_framework_os_services::TokioHostRuntime>());
    assert_eq!(fixture["provider"], "semio_framework_os_services::TokioHostRuntime");
}

#[semio_framework_async_macros::async_test]
async fn directory_native_runtime_identity_preserves_original_injected_owners() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let workers = usize::try_from(case["workers"].as_u64().unwrap()).unwrap();
        let capacity = u32::try_from(case["computeCapacity"].as_u64().unwrap()).unwrap();
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, workers));
        let runtime = Arc::new(semio_framework_os_services::TokioHostRuntime::with_pool(pool.clone()));
        let foreign = Arc::new(semio_framework_os_services::TokioHostRuntime::with_pool(pool.clone()));
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
        pool.shutdown();
        assert_eq!(observed, (true, false, true, true, workers), "{case}");
        assert_eq!((drain.finished, drain.cancelled, drain.leaked), (0, 0, 0));
    }
}
//#endregion 🪪️RuntimeIdentity
