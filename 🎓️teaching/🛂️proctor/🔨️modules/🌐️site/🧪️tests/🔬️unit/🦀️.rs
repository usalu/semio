use super::*;
use crate::storage::tests::{scratch, Scratch};
use axum::http::header::{ALLOW, CACHE_CONTROL, CONTENT_TYPE};

const DASHED_HASH: &str = "🌐️-Djvsi-pa.js";
const DASHED_HASH_URL: &str = "/assets/%F0%9F%8C%90%EF%B8%8F-Djvsi-pa.js";

fn site() -> (Scratch, SiteHost) {
    let directory = scratch("site");
    std::fs::create_dir_all(directory.0.join("assets")).unwrap();
    std::fs::create_dir_all(directory.0.join("🖼️assets/🔤️fonts")).unwrap();
    std::fs::write(directory.0.join(DOCUMENT), "<!doctype html><title>quiz</title>").unwrap();
    std::fs::write(directory.0.join("404.html"), "<!doctype html><title>missing</title>").unwrap();
    std::fs::write(directory.0.join("assets/index-B3xK9aQz.js"), "export {};").unwrap();
    std::fs::write(directory.0.join("assets").join(DASHED_HASH), "export const dashed = 1;").unwrap();
    std::fs::write(directory.0.join("🖼️assets/🔤️fonts/compressed.woff2"), "wOF2").unwrap();
    std::fs::write(directory.0.join("favicon.svg"), "<svg/>").unwrap();
    std::fs::write(directory.0.join(".env"), "SECRET=1").unwrap();
    let host = SiteHost::open(&directory.0).expect("a built site");
    (directory, host)
}

async fn body(response: Response) -> String {
    String::from_utf8(axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap().to_vec()).unwrap()
}

#[test]
fn a_site_without_a_document_is_refused() {
    let directory = scratch("unbuilt");
    assert!(matches!(SiteHost::open(&directory.0), Err(SiteError::NoDocument { .. })));
    assert!(matches!(SiteHost::open(&directory.0.join("absent")), Err(SiteError::Missing { .. })));
}

#[test]
fn paths_resolve_to_files_the_document_a_miss_or_a_refusal() {
    let (_directory, host) = site();
    assert_eq!(host.resolve("/"), Resolution::Document);
    assert!(matches!(host.resolve("/assets/index-B3xK9aQz.js"), Resolution::File(path) if path.ends_with("index-B3xK9aQz.js")));
    assert!(matches!(host.resolve(DASHED_HASH_URL), Resolution::File(path) if path.ends_with(DASHED_HASH)));
    assert!(matches!(host.resolve("/favicon%2Esvg"), Resolution::File(_)));
    assert_eq!(host.resolve("/quiz/power/run"), Resolution::Document);
    assert_eq!(host.resolve("/assets"), Resolution::Document);
    assert_eq!(host.resolve("/assets/missing.js"), Resolution::Missing);
    for refused in ["/../Cargo.toml", "/%2e%2e/secret", "/assets/%2E%2E/%2E%2E/x", "/a%5Cb", "/.env", "/C:%5Cwindows", "/con.txt", "/nul", "/%zz", "/%c3%28", "/a%00b"] {
        assert_eq!(host.resolve(refused), Resolution::Refused, "{refused}");
    }
}

#[test]
fn cache_policy_follows_the_build_layout_not_the_hash_alphabet() {
    for hashed in ["assets/index-B3xK9aQz.js", "assets/🌐️-Djvsi-pa.js", "assets/🌐️-B0ABbSUq.css", "assets/pdf.worker.min-iDqQPrd3.mjs", "assets/nested/logo-_-_-_-_.svg"] {
        assert_eq!(cache_control(Path::new(hashed)), IMMUTABLE, "{hashed}");
    }
    for document in ["index.html", "404.html", "assets/stray.HTML", "🌐️.html"] {
        assert_eq!(cache_control(Path::new(document)), REVALIDATE, "{document}");
    }
    for plain in ["favicon.svg", "CNAME", "🖼️assets/🔤️fonts/compressed.woff2", "assets", "service-worker-B3xK9aQz.js"] {
        assert_eq!(cache_control(Path::new(plain)), SHORT, "{plain}");
    }
}

#[test]
fn content_types_are_recognized() {
    assert_eq!(content_type(Path::new("a/index.html")), "text/html; charset=utf-8");
    assert_eq!(content_type(Path::new("a/app.MJS")), "text/javascript; charset=utf-8");
    assert_eq!(content_type(Path::new("font.woff2")), "font/woff2");
    assert_eq!(content_type(Path::new("unknown.bin")), "application/octet-stream");
}

#[tokio::test]
async fn serving_sets_type_and_cache_per_kind_of_file() {
    let (_directory, host) = site();
    let asset = host.serve(&Method::GET, "/assets/index-B3xK9aQz.js").await;
    assert_eq!(asset.status(), StatusCode::OK);
    assert_eq!(asset.headers()[CONTENT_TYPE], "text/javascript; charset=utf-8");
    assert_eq!(asset.headers()[CACHE_CONTROL], IMMUTABLE);
    assert_eq!(body(asset).await, "export {};");
    let dashed = host.serve(&Method::GET, DASHED_HASH_URL).await;
    assert_eq!((dashed.status(), dashed.headers()[CACHE_CONTROL].to_str().unwrap()), (StatusCode::OK, IMMUTABLE));
    let route = host.serve(&Method::GET, "/quiz/power").await;
    assert_eq!(route.headers()[CACHE_CONTROL], REVALIDATE);
    assert!(body(route).await.contains("<title>quiz</title>"));
    assert_eq!(host.serve(&Method::GET, "/404.html").await.headers()[CACHE_CONTROL], REVALIDATE);
    assert_eq!(host.serve(&Method::GET, "/favicon.svg").await.headers()[CACHE_CONTROL], SHORT);
    assert_eq!(host.serve(&Method::GET, "/%F0%9F%96%BC%EF%B8%8Fassets/%F0%9F%94%A4%EF%B8%8Ffonts/compressed.woff2").await.headers()[CACHE_CONTROL], SHORT);
    let head = host.serve(&Method::HEAD, "/index.html").await;
    assert_eq!(head.headers()[CACHE_CONTROL], REVALIDATE);
    assert_eq!(body(head).await, "");
    assert_eq!(host.serve(&Method::GET, "/assets/gone-B3xK9aQz.js").await.status(), StatusCode::NOT_FOUND);
    assert_eq!(host.serve(&Method::GET, "/%2e%2e/Cargo.toml").await.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn other_methods_are_not_allowed() {
    let (_directory, host) = site();
    for method in [Method::POST, Method::PUT, Method::DELETE, Method::PATCH] {
        let refused = host.serve(&method, "/quiz").await;
        assert_eq!((refused.status(), refused.headers()[ALLOW].to_str().unwrap()), (StatusCode::METHOD_NOT_ALLOWED, ALLOWED_METHODS), "{method}");
        assert!(body(refused).await.contains("\"kind\":\"methodNotAllowed\""));
    }
    assert_eq!(method_not_allowed(&Method::POST, "/assets/x.js").headers()[ALLOW], ALLOWED_METHODS);
}
