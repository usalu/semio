use super::*;

struct Directory;
impl MemberDirectory for Directory {
    async fn head_pack(&self, id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        (id != "absent").then(|| Err(VcsError::Deserialize("head read refused".into())))
    }
    async fn checkpoint_pack(&self, id: &str, _: &str) -> Option<Result<Vec<u8>, VcsError>> {
        (id != "absent").then(|| Err(VcsError::Deserialize("checkpoint read refused".into())))
    }
}
struct Blobs;
impl BlobStore for Blobs {
    async fn put(&self, _: &[u8], _: &str) -> Result<BlobRef, VcsError> { Err(VcsError::Deserialize("unused write".into())) }
    async fn get(&self, _: &str) -> Result<Option<Vec<u8>>, VcsError> { Err(VcsError::Deserialize("blob read refused".into())) }
    async fn has(&self, _: &str) -> Result<bool, VcsError> { Ok(false) }
    async fn delete(&self, _: &str) -> Result<(), VcsError> { Ok(()) }
}
fn ready<T>(future: impl std::future::Future<Output=T>) -> T {
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
        std::task::Poll::Ready(value) => value,
        std::task::Poll::Pending => panic!("fixture ports resolve immediately"),
    }
}

#[test]
fn link_read_failures_preserve_the_original_backend_error() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let resolver = ready(MemberLinkResolver::with_blobs(Directory, Arc::new(Blobs)));
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: "weather".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.epw".into(), standard: "energyplus".into(), subset: "*".into() } };
    for case in law["cases"].as_array().unwrap() {
        let pin = match case["kind"].as_str().unwrap() {
            "head" => LinkPin::Head,
            "checkpoint" => LinkPin::Checkpoint { id: "pinned".into() },
            "snapshot" => LinkPin::Snapshot { blob: BlobRef { hash: "hash".into(), size: 3, media_type: "application/octet-stream".into() } },
            _ => unreachable!(),
        };
        let error = ready(resolver.resolve(&ArtifactLink { target: target.clone(), pin, role: "weather".into() })).unwrap_err();
        assert!(matches!(error, VcsError::Deserialize(message) if message == case["error"].as_str().unwrap()));
    }
    let missing = ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef { artifact_id: law["missing"].as_str().unwrap().into(), ..target }, pin: LinkPin::Head, role: "weather".into() };
    assert_eq!(ready(resolver.resolve(&missing)).unwrap(), LinkState::Missing);
}
