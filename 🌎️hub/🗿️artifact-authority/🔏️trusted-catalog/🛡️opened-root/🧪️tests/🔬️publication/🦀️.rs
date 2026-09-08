
use super::*;
use crate::artifact_authority::{AuthorityLimits, AuthorityOperationControl, AuthorityProgress};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

struct Control(AtomicBool);

impl AuthorityOperationControl for Control {
    fn now_ms(&self) -> u64 {
        0
    }
    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    fn report(&self, _: AuthorityProgress) {}
}

fn fixture_root() -> std::path::PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let artifact = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned artifact root");
    let root = std::path::PathBuf::from(artifact).join(format!("publication-owner-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::SeqCst)));
    std::fs::create_dir_all(root.join("trusted-catalog")).unwrap();
    root
}

#[tokio::test]
async fn trusted_publication_owner_process_crash_releases_exact_lock() {
    const ROOT_ENV: &str = "SEMIO_TRUSTED_PUBLICATION_PROCESS_ROOT";
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📤️publication/🔒️owner.json")).unwrap();
    let control = Control(AtomicBool::new(false));
    let context = OperationContext::new(60_000, AuthorityLimits::maximum(), &control);
    if let Some(root) = std::env::var_os(ROOT_ENV) {
        let root = std::path::PathBuf::from(root);
        let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
        let _owner = data.acquire_publication(&context).await.unwrap();
        std::fs::write(root.join("holder-ready.json"), fixture["process"]["holder"].as_str().unwrap()).unwrap();
        std::future::pending::<()>().await;
        return;
    }
    let root = fixture_root();
    let pointer = root.join("trusted-catalog/current.json");
    let before = fixture["before"].as_str().unwrap().as_bytes();
    std::fs::write(&pointer, before).unwrap();
    let mut child = tokio::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "artifact_authority::trusted_catalog::opened_root::publication_tests::trusted_publication_owner_process_crash_releases_exact_lock", "--nocapture", "--test-threads=1"])
        .env(ROOT_ENV, &root)
        .stdin(std::process::Stdio::null())
        .stdout(File::create(root.join("holder.stdout.txt")).unwrap())
        .stderr(File::create(root.join("holder.stderr.txt")).unwrap())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let held = tokio::time::timeout(std::time::Duration::from_secs(20), async {
        while !root.join("holder-ready.json").exists() {
            if let Some(status) = child.try_wait().unwrap() {
                return Err(format!("publication child exited before lock admission: {status}"));
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
        if std::fs::read_to_string(root.join("holder-ready.json")).unwrap() != fixture["process"]["holder"].as_str().unwrap() {
            return Err("publication child did not acknowledge its exact lock".to_owned());
        }
        let directory = platform::open_server_owned(&root.join("trusted-catalog")).unwrap();
        let competing = super::super::super::file_fence::try_acquire(platform::open_lock(&directory).unwrap()).unwrap();
        if competing.is_some() {
            return Err("competing process bypassed the owned publication fence".to_owned());
        }
        Ok(())
    })
    .await;
    child.kill().await.unwrap();
    assert!(!child.wait().await.unwrap().success());
    held.expect("bounded child lock admission").unwrap();
    let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
    let owner = tokio::time::timeout(std::time::Duration::from_secs(5), data.acquire_publication(&context)).await.expect("crash releases the OS fence").unwrap();
    assert_eq!(owner.open_current().unwrap().unwrap().read_bounded(65_536, &context).await.unwrap(), before);
    assert_eq!(std::fs::read(pointer).unwrap(), before);
    assert_eq!(fixture["process"]["competing"], "contended");
    assert_eq!(fixture["process"]["afterCrash"], "acquired");
    assert_eq!(fixture["process"]["current"], "unchanged");
    println!("[DEBUG] publication cross-process fence: holder=acquired competing=contended crash=reaped next=acquired current=unchanged");
}

#[tokio::test]
async fn trusted_publication_owner_lock_drop_and_exact_replacement_match_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📤️publication/🔒️owner.json")).unwrap();
    let root = fixture_root();
    let control = Control(AtomicBool::new(false));
    let context = OperationContext::new(60_000, AuthorityLimits::maximum(), &control);
    let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
    let owner = data.acquire_publication(&context).await.unwrap();
    let competing = super::super::super::file_fence::try_acquire(platform::open_lock(&owner.directory).unwrap()).unwrap();
    assert!(competing.is_none());
    std::fs::write(root.join("trusted-catalog/current.json"), fixture["before"].as_str().unwrap()).unwrap();
    let after = fixture["after"].as_str().unwrap().as_bytes();
    let sync = owner.replace_current(fixture["nonce"].as_str().unwrap(), after, &context).unwrap();
    #[cfg(unix)]
    assert!(matches!(sync, TrustedPublicationSync::Durable));
    #[cfg(windows)]
    assert!(matches!(sync, TrustedPublicationSync::Unconfirmed));
    assert_eq!(owner.open_current().unwrap().unwrap().read_bounded(65_536, &context).await.unwrap(), after);
    control.0.store(true, Ordering::SeqCst);
    assert!(owner.replace_current("1123456789abcdef0123456789abcdef", b"cancelled", &context).is_err());
    assert_eq!(std::fs::read(root.join("trusted-catalog/current.json")).unwrap(), after);
    drop(owner);
    control.0.store(false, Ordering::SeqCst);
    let next = data.acquire_publication(&context).await.unwrap();
    let collision = root.join("trusted-catalog/.current-2123456789abcdef0123456789abcdef.json");
    std::fs::write(&collision, b"other writer").unwrap();
    assert!(next.replace_current("2123456789abcdef0123456789abcdef", b"collision", &context).is_err());
    assert_eq!(std::fs::read(collision).unwrap(), b"other writer");
    println!("[DEBUG] publication owner: competing=contended drop=reacquired replacement=exact cancellation=retained collision=retained");
}

#[tokio::test]
async fn trusted_publication_owner_remains_rooted_after_path_replacement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/📤️publication/🔒️owner.json")).unwrap();
    let root = fixture_root();
    let control = Control(AtomicBool::new(false));
    let context = OperationContext::new(60_000, AuthorityLimits::maximum(), &control);
    let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
    let owner = data.acquire_publication(&context).await.unwrap();
    std::fs::rename(root.join("trusted-catalog"), root.join("retained-catalog")).unwrap();
    std::fs::create_dir(root.join("trusted-catalog")).unwrap();
    let foreign = fixture["replacement"]["foreignRoot"].as_str().unwrap().as_bytes();
    std::fs::write(root.join("trusted-catalog/current.json"), foreign).unwrap();
    let selected = fixture["replacement"]["selected"].as_str().unwrap().as_bytes();
    owner.replace_current(fixture["nonce"].as_str().unwrap(), selected, &context).unwrap();
    assert_eq!(std::fs::read(root.join("retained-catalog/current.json")).unwrap(), selected);
    assert_eq!(std::fs::read(root.join("trusted-catalog/current.json")).unwrap(), foreign);
    println!("[DEBUG] publication rooted replace: retained=selected replacement-root=unchanged");
}

#[tokio::test]
async fn trusted_publication_owner_refuses_nonregular_lock_leaves() {
    let root = fixture_root();
    let control = Control(AtomicBool::new(false));
    let context = OperationContext::new(60_000, AuthorityLimits::maximum(), &control);
    std::fs::create_dir(root.join("trusted-catalog/.publication.lock")).unwrap();
    let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
    assert!(data.acquire_publication(&context).await.is_err());
    #[cfg(unix)]
    {
        let root = fixture_root();
        std::fs::write(root.join("foreign.lock"), b"foreign").unwrap();
        std::os::unix::fs::symlink(root.join("foreign.lock"), root.join("trusted-catalog/.publication.lock")).unwrap();
        let data = TrustedCatalogDataRoot::open_server_owned(&root).unwrap();
        assert!(data.acquire_publication(&context).await.is_err());
        assert_eq!(std::fs::read(root.join("foreign.lock")).unwrap(), b"foreign");
    }
    println!("[DEBUG] publication lock admission: directory=refused linked-leaf=refused-on-unix");
}
