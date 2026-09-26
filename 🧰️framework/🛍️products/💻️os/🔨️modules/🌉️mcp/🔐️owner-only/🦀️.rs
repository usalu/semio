//! 🔐️ Owner-only files for the semio MCP (bridge offers, delegated agent credentials) on every platform. POSIX gets an
//! exact `0600`; Windows ignores modes, so the file's DACL is replaced with one entry for the current account (`icacls`)
//! and read back as SDDL (`Get-Acl`) to prove no other principal kept access. The TypeScript twin is
//! `📚️library/🏃️process/🔐️owner-only/🟦️.ts`; both are pinned by `📚️library/🧫️fixtures/🔐️owner-only/🔣️.json`.
//! <https://learn.microsoft.com/windows-server/administration/windows-commands/icacls>
//! <https://learn.microsoft.com/windows/win32/secauthz/security-descriptor-string-format>

use std::path::Path;

/// 🪪️ The account SID `whoami /user /fo csv /nh` reports (`"host\user","S-1-5-21-…"`), whatever the line endings.
pub fn parse_whoami_user_sid(stdout: &str) -> Option<String> {
    stdout.lines().find_map(|line| {
        let line = line.trim();
        let sid = line.strip_suffix('"')?.rsplit_once("\",\"")?.1;
        let digits = sid.strip_prefix("S-1-")?;
        (line.starts_with('"') && digits.split('-').count() >= 2 && digits.split('-').all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()))).then(|| sid.to_owned())
    })
}

/// 🧾️ The `icacls` arguments after the path: drop every inherited entry, grant full control to `sid` alone (a directory's
/// grant is inherited by everything created inside it).
pub fn owner_only_acl_flags(sid: &str, directory: bool) -> [String; 3] {
    ["/inheritance:r".to_owned(), "/grant:r".to_owned(), format!("*{sid}:{}F", if directory { "(OI)(CI)" } else { "" })]
}

/// 🧮️ Why an SDDL security descriptor is not owner-only for `sid`: a missing, null or unprotected DACL, or an allow entry
/// (inherit-only included: it reaches everything created inside) for anyone but `sid`, the owner (`OW`, `CO`), LocalSystem
/// or Administrators.
pub fn sddl_owner_only_violations(sddl: &str, sid: &str) -> Vec<String> {
    let Some(start) = sddl.find("D:") else { return vec!["no discretionary ACL (everyone has access)".to_owned()] };
    let rest = &sddl[start + 2..];
    let flags_end = rest.find(|character: char| !(character.is_ascii_uppercase() || character == '_')).unwrap_or(rest.len());
    let flags = &rest[..flags_end];
    if flags.contains("NO_ACCESS_CONTROL") {
        return vec!["no discretionary ACL (everyone has access)".to_owned()];
    }
    let trusted = [sid, "OW", "CO", "SY", "S-1-5-18", "BA", "S-1-5-32-544"];
    let mut violations = if flags.contains('P') { Vec::new() } else { vec!["inherits entries from its parent".to_owned()] };
    let mut aces = &rest[flags_end..];
    while let Some(body) = aces.strip_prefix('(') {
        let Some(end) = body.find(')') else { break };
        let fields: Vec<&str> = body[..end].split(';').collect();
        aces = &body[end + 1..];
        let (kind, account) = (fields.first().copied().unwrap_or(""), fields.get(5).copied().unwrap_or(""));
        if ["A", "OA", "XA", "ZA"].contains(&kind) && !trusted.contains(&account) {
            violations.push(format!("grants {account} access"));
        }
    }
    violations
}

/// 🔏️ Writes `bytes` to `path` so only the current account can read it. On Windows the empty file is restricted before
/// the bytes arrive, so the secret never sits behind an inherited ACL.
pub fn write_owner_only(path: &Path, bytes: &[u8]) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        let mut file = std::fs::OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(path).map_err(|error| format!("cannot write `{}`: {error}", path.display()))?;
        file.set_permissions(std::fs::Permissions::from_mode(0o600)).map_err(|error| format!("cannot restrict `{}`: {error}", path.display()))?;
        file.write_all(bytes).and_then(|()| file.flush()).map_err(|error| format!("cannot write `{}`: {error}", path.display()))
    }
    #[cfg(windows)]
    {
        std::fs::write(path, []).map_err(|error| format!("cannot create `{}`: {error}", path.display()))?;
        windows::restrict(path, false)?;
        std::fs::write(path, bytes).map_err(|error| format!("cannot write `{}`: {error}", path.display()))
    }
    #[cfg(not(any(unix, windows)))]
    {
        std::fs::write(path, bytes).map_err(|error| format!("cannot write `{}`: {error}", path.display()))
    }
}

/// 🔎️ `Err(reason)` unless `path` is readable by the current account alone (POSIX: no group/other bits; Windows:
/// owner-only DACL).
pub fn verify_owner_only(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path).map_err(|error| format!("is unreadable: {error}"))?.permissions().mode() & 0o777;
        if mode & 0o077 != 0 {
            return Err(format!("is mode {mode:04o}; it must not be readable by group or others (chmod 600)"));
        }
        Ok(())
    }
    #[cfg(windows)]
    {
        let violations = sddl_owner_only_violations(&windows::sddl(path)?, &windows::current_user_sid()?);
        if violations.is_empty() { Ok(()) } else { Err(format!("is not owner-only: {}", violations.join("; "))) }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = path;
        Ok(())
    }
}

#[cfg(windows)]
mod windows {
    use std::os::windows::process::CommandExt;
    use std::path::Path;
    use std::process::Command;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    /// 🙋️ The current account's SID, from `whoami`.
    pub(super) fn current_user_sid() -> Result<String, String> {
        let output = Command::new("whoami").args(["/user", "/fo", "csv", "/nh"]).creation_flags(CREATE_NO_WINDOW).output().map_err(|error| format!("whoami failed: {error}"))?;
        if !output.status.success() {
            return Err(format!("whoami /user failed: {}", String::from_utf8_lossy(&output.stderr).trim()));
        }
        super::parse_whoami_user_sid(&String::from_utf8_lossy(&output.stdout)).ok_or_else(|| "whoami /user did not report a SID".to_owned())
    }

    /// 🪟️ The DACL of `path` as SDDL; the path travels in the environment so no quoting can alter it.
    pub(super) fn sddl(path: &Path) -> Result<String, String> {
        let output = Command::new("powershell.exe")
            .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", "(Get-Acl -LiteralPath $env:SEMIO_OWNER_ONLY_PATH).Sddl"])
            .env("SEMIO_OWNER_ONLY_PATH", path)
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|error| format!("Get-Acl failed: {error}"))?;
        let sddl = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        if !output.status.success() || sddl.is_empty() {
            return Err(format!("Get-Acl could not read `{}`: {}", path.display(), String::from_utf8_lossy(&output.stderr).trim()));
        }
        Ok(sddl)
    }

    /// 🔒️ Replaces the DACL of `path` with one full-control entry for the current account and proves it took.
    pub(super) fn restrict(path: &Path, directory: bool) -> Result<(), String> {
        let sid = current_user_sid()?;
        let output = Command::new("icacls").arg(path).args(super::owner_only_acl_flags(&sid, directory)).creation_flags(CREATE_NO_WINDOW).output().map_err(|error| format!("icacls failed: {error}"))?;
        if !output.status.success() {
            return Err(format!("icacls could not restrict `{}`: {}", path.display(), String::from_utf8_lossy(&output.stdout).trim()));
        }
        let violations = super::sddl_owner_only_violations(&sddl(path)?, &sid);
        if violations.is_empty() { Ok(()) } else { Err(format!("`{}` is not owner-only after icacls: {}", path.display(), violations.join("; "))) }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit;
