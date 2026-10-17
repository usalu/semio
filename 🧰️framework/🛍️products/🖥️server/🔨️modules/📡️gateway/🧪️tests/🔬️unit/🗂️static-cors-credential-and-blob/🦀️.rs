//! 🗂️ Static app hosting, CORS reflection, transport credentials and blob writes of the gateway unit laws. See [`super`].

use super::*;

//#region 🔖️StaticApp
#[test]
fn a_static_host_refuses_traversal() {
    let host = StaticAppHost::new("/srv/app");
    assert!(host.resolve("../secret").is_none());
    assert!(host.resolve("nested/../../secret").is_none());
    assert!(host.resolve("nested\\secret").is_none());
    assert_eq!(host.resolve("/etc/passwd"), Some(PathBuf::from("/srv/app/etc/passwd")));
    assert_eq!(host.resolve("assets/app.js"), Some(PathBuf::from("/srv/app/assets/app.js")));
}

#[tokio::test]
async fn a_client_route_falls_back_to_index_html() {
    let root = scratch("spa");
    std::fs::write(root.join("index.html"), b"<!doctype html>shell").unwrap();
    std::fs::write(root.join("app.js"), b"export {}").unwrap();
    let host = StaticAppHost::new(&root);

    let asset = host.serve("app.js");
    assert_eq!(asset.status(), StatusCode::OK);
    assert_eq!(asset.headers().get(header::CONTENT_TYPE).unwrap(), "text/javascript");

    let route = host.serve("spaces/sp-1");
    assert_eq!(route.status(), StatusCode::OK);
    let body = axum::body::to_bytes(route.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body.as_ref(), b"<!doctype html>shell");

    assert_eq!(host.serve("../secret").status(), StatusCode::BAD_REQUEST);
    assert_eq!(StaticAppHost::new(root.join("missing")).serve("index.html").status(), StatusCode::SERVICE_UNAVAILABLE);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn installs_are_scanned_and_ordered() {
    let root = scratch("apps");
    for id in ["beta", "alpha"] {
        std::fs::create_dir_all(root.join(id)).unwrap();
        std::fs::write(root.join(id).join("install.json"), format!("{{\"extensionId\":\"{id}\"}}")).unwrap();
    }
    std::fs::create_dir_all(root.join("empty")).unwrap();
    let registry = AppRegistry::new();
    registry.register("extensions", &root);
    let installs = registry.installs("extensions");
    assert_eq!(installs.iter().map(|install| install.id.as_str()).collect::<Vec<_>>(), vec!["alpha", "beta"]);
    assert_eq!(installs[0].manifest["extensionId"], "alpha");
    assert_eq!(registry.names(), vec!["extensions".to_string()]);
    assert!(registry.installs("nothing").is_empty());
    std::fs::remove_dir_all(&root).unwrap();
}
//#endregion 🔖️StaticApp

//#region 🔖️Cors
#[test]
fn cors_reflects_the_callers_own_origin_and_never_allows_credentials() {
    let mut headers = HeaderMap::new();
    apply_cors_headers(&mut headers, Some(&HeaderValue::from_static("http://127.0.0.1:6072")));
    assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "http://127.0.0.1:6072");
    assert!(headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS).is_none());
    assert_eq!(headers.get(header::VARY).unwrap(), "origin");

    let mut anonymous = HeaderMap::new();
    apply_cors_headers(&mut anonymous, None);
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none());
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_METHODS).is_some());
}
//#endregion 🔖️Cors

//#region 🔖️Credential
#[test]
fn a_bearer_and_the_loopback_fact_are_read_from_the_transport() {
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer tok"));
    headers.insert(CAPABILITY_HEADER, HeaderValue::from_static("share-1"));
    let local = credential(&headers, Some(loopback()));
    assert_eq!(local.bearer.as_deref(), Some("tok"));
    assert_eq!(local.capability.map(|proof| proof.0), Some("share-1".to_string()));
    assert!(local.loopback);
    assert!(!credential(&headers, Some(SocketAddr::from(([10, 0, 0, 7], 80)))).loopback);
    assert!(!credential(&HeaderMap::new(), None).loopback);
    assert!(credential(&HeaderMap::new(), None).bearer.is_none());
}
//#endregion 🔖️Credential

//#region 🔖️Blob
#[tokio::test]
async fn a_blob_address_that_does_not_match_its_bytes_is_a_conflict() {
    let state = state().await;
    grant(&state, PolicyPoint::BlobWrite, "write");
    grant(&state, PolicyPoint::BlobRead, "read");

    let honest = content_hash(b"hello").to_string();
    let receipt = put_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect("an honest address is accepted");
    assert_eq!(receipt.0.size, 5);

    let lie = content_hash(b"world").to_string();
    let error = put_blob(Path(lie), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect_err("a mismatched address is refused");
    assert_eq!(error.status(), StatusCode::CONFLICT);

    assert_eq!(head_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::OK);
    let missing = content_hash(b"world").to_string();
    assert_eq!(head_blob(Path(missing), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::NOT_FOUND);
    let short = put_blob(Path("beef".to_string()), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await;
    assert_eq!(short.err().map(|error| error.status()), Some(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn a_blob_write_without_a_grant_is_forbidden() {
    let state = state().await;
    let hash = content_hash(b"hello").to_string();
    let error = put_blob(Path(hash), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await.expect_err("closed by default");
    assert_eq!(error.status(), StatusCode::FORBIDDEN);
}
//#endregion 🔖️Blob
