import { spawnSync } from "node:child_process";
import { chmodSync, statSync } from "node:fs";

/**
 * 🔐️ Owner-only files and directories on every platform: credentials, session-broker records and the local hub's data
 * root must stay private to the account that created them. POSIX gets exact `0600`/`0700` modes (umask-independent);
 * Windows ignores modes, so the DACL is replaced with one entry for the current account (`icacls`) and read back as SDDL
 * (`Get-Acl`) to prove no other principal kept access. The same rules are implemented in Rust by the semio MCP
 * (`🌉️mcp/🔐️owner-only`) and pinned by one language-neutral fixture (`📚️library/🧫️fixtures/🔐️owner-only`).
 * https://learn.microsoft.com/windows-server/administration/windows-commands/icacls
 * https://learn.microsoft.com/windows/win32/secauthz/security-descriptor-string-format
 */
export type OwnerOnlyKind = "file" | "directory";

/** 🧰️ The operating-system seam the owner-only rules run through; laws inject a recording host to drive the Windows branch anywhere. */
export interface OwnerOnlyHost {
  readonly platform: NodeJS.Platform;
  run(command: string, args: readonly string[], env?: NodeJS.ProcessEnv): { readonly status: number | null; readonly stdout: string; readonly stderr: string };
}

/** 🖥️ The real host: `spawnSync` without a shell, hidden console windows on Windows. */
export const nativeOwnerOnlyHost: OwnerOnlyHost = {
  platform: process.platform,
  run(command, args, env) {
    const result = spawnSync(command, [...args], { encoding: "utf8", windowsHide: true, env: env ?? process.env });
    return { status: result.error ? null : result.status, stdout: result.stdout ?? "", stderr: result.error ? String(result.error) : result.stderr ?? "" };
  },
};

/** 🪪️ The current account's SID from `whoami /user /fo csv /nh` (`"host\user","S-1-5-21-…"`), whatever the line endings or account name. */
export function parseWhoamiUserSid(stdout: string): string {
  const match = stdout.match(/^\s*"(?:[^"]|"")*","(S-1-\d+(?:-\d+)+)"\s*$/m);
  if (!match) throw new Error("whoami /user did not report a SID");
  return match[1]!;
}

/** 🧾️ `icacls` arguments that drop every inherited entry and grant full control to `sid` alone; a directory's grant is inherited by everything created inside it. */
export function ownerOnlyAclArguments(path: string, sid: string, kind: OwnerOnlyKind): string[] {
  return [path, "/inheritance:r", "/grant:r", `*${sid}:${kind === "directory" ? "(OI)(CI)" : ""}F`];
}

/** 🧮️ Why an SDDL security descriptor is not owner-only for `sid`: a missing, null or unprotected DACL, or an allow entry (inherit-only included: it reaches everything created inside) for anyone but `sid`, the owner (`OW`, `CO`), LocalSystem or Administrators. */
export function sddlOwnerOnlyViolations(sddl: string, sid: string): string[] {
  const dacl = sddl.match(/D:([A-Z_]*)((?:\([^)]*\))*)/);
  if (!dacl || dacl[1]!.includes("NO_ACCESS_CONTROL")) return ["no discretionary ACL (everyone has access)"];
  const trusted = new Set([sid, "OW", "CO", "SY", "S-1-5-18", "BA", "S-1-5-32-544"]);
  const violations = dacl[1]!.includes("P") ? [] : ["inherits entries from its parent"];
  for (const ace of dacl[2]!.matchAll(/\(([^)]*)\)/g)) {
    const [type, , , , , account] = ace[1]!.split(";");
    if (["A", "OA", "XA", "ZA"].includes(type ?? "") && !trusted.has(account ?? "")) violations.push(`grants ${account} access`);
  }
  return violations;
}

/** 🪟️ The DACL of `path` as SDDL, read through `Get-Acl` with the path in the environment so no quoting can alter it. */
function windowsSddl(path: string, host: OwnerOnlyHost): string {
  const read = host.run("powershell.exe", ["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", "(Get-Acl -LiteralPath $env:SEMIO_OWNER_ONLY_PATH).Sddl"], { ...process.env, SEMIO_OWNER_ONLY_PATH: path });
  if (read.status !== 0 || !read.stdout.trim()) throw new Error(`Get-Acl could not read ${path}: ${read.stderr.trim()}`);
  return read.stdout.trim();
}

/** 🙋️ The current Windows account's SID. */
function windowsUserSid(host: OwnerOnlyHost): string {
  const whoami = host.run("whoami", ["/user", "/fo", "csv", "/nh"]);
  if (whoami.status !== 0) throw new Error(`whoami /user failed: ${whoami.stderr.trim()}`);
  return parseWhoamiUserSid(whoami.stdout);
}

/** 🔏️ Makes `path` private to the current account and proves it: POSIX `0600`/`0700`; Windows owner-only DACL, read back and checked. */
export function protectOwnerOnly(path: string, kind: OwnerOnlyKind, host: OwnerOnlyHost = nativeOwnerOnlyHost): void {
  if (host.platform !== "win32") {
    chmodSync(path, kind === "directory" ? 0o700 : 0o600);
    return;
  }
  const sid = windowsUserSid(host);
  const applied = host.run("icacls", ownerOnlyAclArguments(path, sid, kind));
  if (applied.status !== 0) throw new Error(`icacls could not restrict ${path}: ${(applied.stderr || applied.stdout).trim()}`);
  const violations = sddlOwnerOnlyViolations(windowsSddl(path, host), sid);
  if (violations.length) throw new Error(`${path} is not owner-only after icacls: ${violations.join("; ")}`);
}

/** 🔎️ Throws unless `path` is readable by the current account alone (POSIX: no group/other bits; Windows: owner-only DACL). */
export function assertOwnerOnly(path: string, host: OwnerOnlyHost = nativeOwnerOnlyHost): void {
  if (host.platform !== "win32") {
    const mode = statSync(path).mode & 0o777;
    if (mode & 0o077) throw new Error(`${path} is mode ${mode.toString(8).padStart(4, "0")}; it must not be accessible to group or others`);
    return;
  }
  const violations = sddlOwnerOnlyViolations(windowsSddl(path, host), windowsUserSid(host));
  if (violations.length) throw new Error(`${path} is not owner-only: ${violations.join("; ")}`);
}
