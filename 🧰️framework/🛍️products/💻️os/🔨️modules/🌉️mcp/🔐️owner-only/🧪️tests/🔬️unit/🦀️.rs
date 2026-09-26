//! 🧪️ `🔐️owner-only` — the Rust twin of `📚️library/🏃️process/🔐️owner-only/🟦️.ts` against the same language-neutral
//! fixture (whoami SID parsing, icacls arguments, SDDL verdicts), plus the real POSIX modes on this host.

use super::*;

const FIXTURE: &str = include_str!("../../../../../../🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔐️owner-only/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(FIXTURE).expect("owner-only fixture parses")
}

fn text(row: &serde_json::Value, key: &str) -> String {
    row[key].as_str().unwrap_or_else(|| panic!("fixture field `{key}`")).to_owned()
}

#[test]
fn whoami_rows_yield_exactly_the_fixture_sid() {
    for row in fixture()["whoami"].as_array().expect("whoami rows") {
        assert_eq!(parse_whoami_user_sid(&text(row, "stdout")), row["sid"].as_str().map(str::to_owned), "{}", text(row, "name"));
    }
}

#[test]
fn icacls_arguments_are_the_fixture_arguments() {
    for row in fixture()["icacls"].as_array().expect("icacls rows") {
        let mut arguments = vec![text(row, "path")];
        arguments.extend(owner_only_acl_flags(&text(row, "sid"), text(row, "kind") == "directory"));
        let expected: Vec<String> = row["arguments"].as_array().expect("arguments").iter().map(|value| value.as_str().expect("argument").to_owned()).collect();
        assert_eq!(arguments, expected, "{}", text(row, "name"));
    }
}

#[test]
fn sddl_verdicts_are_the_fixture_verdicts() {
    for row in fixture()["sddl"].as_array().expect("sddl rows") {
        let expected: Vec<String> = row["violations"].as_array().expect("violations").iter().map(|value| value.as_str().expect("violation").to_owned()).collect();
        assert_eq!(sddl_owner_only_violations(&text(row, "sddl"), &text(row, "sid")), expected, "{}", text(row, "name"));
    }
}

#[cfg(unix)]
#[test]
fn a_written_secret_is_mode_0600_even_over_a_world_readable_file() {
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!("semio-owner-only-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join("offer.json");
    std::fs::write(&path, b"old").expect("previous file");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).expect("mode 644");
    let refused = verify_owner_only(&path).expect_err("a world-readable file is refused");
    assert!(refused.contains("chmod 600"), "the refusal names the remedy: {refused}");
    write_owner_only(&path, b"secret").expect("secret written");
    assert_eq!(std::fs::metadata(&path).expect("metadata").permissions().mode() & 0o777, 0o600);
    assert_eq!(std::fs::read(&path).expect("read back"), b"secret");
    verify_owner_only(&path).expect("0600 is owner-only");
    let _ = std::fs::remove_dir_all(&directory);
}
