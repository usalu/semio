//! 🧪️ `🛰️rendezvous` — the local bridge rendezvous: offer round-trip, owner-only permissions,
//! self-cleanup on drop, stale-session sweeping, and admission-proof freshness. Every test drives the
//! REAL directory layout under its OWN temporary root (never a process-global `HOME`, which sibling
//! tests read concurrently) and never a mocked filesystem.

use super::*;

/// 📂️ A rendezvous root of this test's own, removed when the test ends.
struct TemporaryRoot(PathBuf);

impl TemporaryRoot {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!("semio-mcp-rendezvous-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("temporary rendezvous root");
        Self(directory)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TemporaryRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn offer_for(pid: u32) -> BridgeOffer {
    BridgeOffer {
        schema_version: RENDEZVOUS_SCHEMA_VERSION,
        url: format!("ws://127.0.0.1:6301/bridge?pid={pid}"),
        admission_proof: mint_admission_proof(),
        principal: "agent:local".to_string(),
        pid,
        published_at_ms: 1_700_000_000_000,
    }
}

fn write_session(root: &Path, record: &OsSessionRecord) {
    let directory = sessions_dir_in(root);
    std::fs::create_dir_all(&directory).expect("sessions directory");
    std::fs::write(directory.join(format!("{}.json", record.pid)), serde_json::to_vec(record).expect("session record")).expect("write session record");
}

#[test]
fn a_published_offer_round_trips_and_is_owner_only() {
    let root = TemporaryRoot::new("offer");
    let published = publish_offer_in(root.path(), offer_for(4242)).expect("offer publishes");
    let bytes = std::fs::read(published.path()).expect("offer file readable");
    let decoded: BridgeOffer = serde_json::from_slice(&bytes).expect("offer decodes");
    assert_eq!(decoded, *published.offer());
    assert_eq!(decoded.schema_version, RENDEZVOUS_SCHEMA_VERSION);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(published.path()).expect("offer metadata").permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "a bridge offer carries an admission proof and must never be group/world readable");
    }
    assert!(!offers_dir_in(root.path()).join("4242.json.partial").exists(), "the atomic temp file is renamed away, never left behind");
}

#[test]
fn dropping_a_published_offer_removes_the_file() {
    let root = TemporaryRoot::new("drop");
    let path = {
        let published = publish_offer_in(root.path(), offer_for(4243)).expect("offer publishes");
        published.path().to_path_buf()
    };
    assert!(!path.exists(), "a gateway that exits must not leave a dead ws:// address behind");
}

#[test]
fn live_sessions_keep_this_process_and_sweep_a_dead_pid() {
    let root = TemporaryRoot::new("sessions");
    let alive = OsSessionRecord { schema_version: RENDEZVOUS_SCHEMA_VERSION, session_id: "shell-alive".into(), pid: std::process::id(), shell_kind: "react".into(), started_at_ms: 2 };
    let dead = OsSessionRecord { schema_version: RENDEZVOUS_SCHEMA_VERSION, session_id: "shell-dead".into(), pid: 0x7FFF_FFF0, shell_kind: "react".into(), started_at_ms: 1 };
    write_session(root.path(), &alive);
    write_session(root.path(), &dead);
    let live = live_os_sessions_in(root.path());
    #[cfg(unix)]
    {
        assert_eq!(live.len(), 1, "exactly the record whose process is alive survives");
        assert_eq!(live[0].session_id, "shell-alive");
        assert!(!sessions_dir_in(root.path()).join(format!("{}.json", dead.pid)).exists(), "a stale record is swept, never believed");
    }
    #[cfg(not(unix))]
    assert_eq!(live.len(), 2, "without a process probe both young records are believed");
}

#[test]
fn no_session_directory_is_an_empty_discovery_not_an_error() {
    let root = TemporaryRoot::new("empty");
    assert!(live_os_sessions_in(root.path()).is_empty());
}

#[test]
fn a_malformed_session_record_is_swept() {
    let root = TemporaryRoot::new("malformed");
    std::fs::create_dir_all(sessions_dir_in(root.path())).expect("sessions directory");
    let path = sessions_dir_in(root.path()).join("9999.json");
    std::fs::write(&path, b"{ not json").expect("write malformed record");
    assert!(live_os_sessions_in(root.path()).is_empty());
    assert!(!path.exists());
}

#[test]
fn admission_proofs_are_fresh_per_mint() {
    let first = mint_admission_proof();
    let second = mint_admission_proof();
    assert_eq!(first.len(), 32);
    assert!(first.chars().all(|character| character.is_ascii_hexdigit()));
    assert_ne!(first, second, "an admission proof is minted per process, never a derived constant");
}
