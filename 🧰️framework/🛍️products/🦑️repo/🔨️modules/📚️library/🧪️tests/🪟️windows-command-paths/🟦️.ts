/** 🪟️ Windows command-path laws: the owner-only rules (`🏃️process/🔐️owner-only`) and the process table
 * (`🏃️process/📋️process-table`) run their Windows branch through injected hosts on any machine, their POSIX branch on the
 * real host against Python and Node oracles, and the import closure of every fleet entrypoint (`dev s`, local hub, hub,
 * semio MCP, repository cache) is held free of POSIX-only primitives outside platform-aware owners.
 * @see ../../🧫️fixtures/🔐️owner-only/🔣️.json
 * @see ../../🧫️fixtures/📋️process-table/🔣️.json */
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { assertOwnerOnly, ownerOnlyAclArguments, parseWhoamiUserSid, protectOwnerOnly, sddlOwnerOnlyViolations, type OwnerOnlyHost } from "../../🏃️process/🔐️owner-only/🟦️.ts";
import { parsePosixProcessTable, parseWindowsProcessTable, processTableSnapshot, WINDOWS_PROCESS_TABLE_ARGUMENTS } from "../../🏃️process/📋️process-table/🟦️.ts";
import { ACTIVE_SEMIO_TECH_BUILD } from "../../🧼️workspace-cleanup/🧟️stray-processes/🟦️.ts";

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const load = (path: string): any => JSON.parse(readFileSync(join(libraryRoot, path), "utf8"));
const ownerOnly = load("🧫️fixtures/🔐️owner-only/🔣️.json");
const processTable = load("🧫️fixtures/📋️process-table/🔣️.json");
const python = Bun.which("python3") ?? Bun.which("python");
const powershell = Bun.which("pwsh") ?? Bun.which("powershell.exe");

/** 📼️ A Windows host that answers from a script and records every command it was asked to run. */
function recordingWindowsHost(answers: Record<string, { status: number; stdout: string }>): OwnerOnlyHost & { readonly calls: { command: string; args: readonly string[]; path?: string }[] } {
  const calls: { command: string; args: readonly string[]; path?: string }[] = [];
  return {
    platform: "win32",
    calls,
    run(command, args, env) {
      calls.push({ command, args, path: env?.SEMIO_OWNER_ONLY_PATH });
      const answer = answers[command];
      return answer ? { ...answer, stderr: "" } : { status: null, stdout: "", stderr: `${command} missing` };
    },
  };
}

describe("owner-only files", () => {
  test("validates the language-neutral owner-only fixture", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(load("🧬️schema/🔐️owner-only/🔣️.json"));
    expect(validate(ownerOnly), JSON.stringify(validate.errors)).toBe(true);
  });

  test("reads the account SID from whoami, refuses everything else", () => {
    for (const row of ownerOnly.whoami) {
      if (row.sid === null) expect(() => parseWhoamiUserSid(row.stdout), row.name).toThrow("SID");
      else expect(parseWhoamiUserSid(row.stdout), row.name).toBe(row.sid);
    }
  });

  test("builds the exact icacls arguments and judges every SDDL case", () => {
    for (const row of ownerOnly.icacls) expect(ownerOnlyAclArguments(row.path, row.sid, row.kind), row.name).toEqual(row.arguments);
    for (const row of ownerOnly.sddl) expect(sddlOwnerOnlyViolations(row.sddl, row.sid), row.name).toEqual(row.violations);
  });

  test("agrees with .NET's RawSecurityDescriptor on every SDDL case (PowerShell oracle, where one is installed)", () => {
    if (!powershell) return console.log("[owner-only] no pwsh/powershell.exe on PATH: .NET oracle skipped");
    const script = `$cases = $env:SEMIO_SDDL_CASES | ConvertFrom-Json; $trusted = @('S-1-5-18','S-1-5-32-544','S-1-3-0','S-1-3-4'); $cases | ForEach-Object { $d = [System.Security.AccessControl.RawSecurityDescriptor]::new($_.sddl); if ($null -eq $d.DiscretionaryAcl) { 1 } else { $n = 0; if (-not ($d.ControlFlags -band [System.Security.AccessControl.ControlFlags]::DiscretionaryAclProtected)) { $n++ }; foreach ($ace in $d.DiscretionaryAcl) { if ($ace.AceQualifier -eq 'AccessAllowed' -and $ace.SecurityIdentifier.Value -ne $_.sid -and $trusted -notcontains $ace.SecurityIdentifier.Value) { $n++ } }; $n } }`;
    const oracle = spawnSync(powershell, ["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", script], { encoding: "utf8", env: { ...process.env, SEMIO_SDDL_CASES: JSON.stringify(ownerOnly.sddl.filter((row: { sddl: string }) => !row.sddl.includes("NO_ACCESS_CONTROL"))) } });
    expect(oracle.status, oracle.stderr).toBe(0);
    const counts = oracle.stdout.trim().split(/\r?\n/).map(Number);
    expect(counts).toEqual(ownerOnly.sddl.filter((row: { sddl: string }) => !row.sddl.includes("NO_ACCESS_CONTROL")).map((row: { violations: string[] }) => row.violations.length));
  });

  test("POSIX: files become 0600 and directories 0700 whatever the umask left, and group/other access is refused (Python os.stat oracle)", () => {
    if (process.platform === "win32") return;
    const root = mkdtempSync(join(tmpdir(), "semio-owner-only-"));
    try {
      const directory = join(root, "hub-dev"), file = join(directory, "local-session-broker.json");
      mkdirSync(directory, { mode: 0o777 });
      chmodSync(directory, 0o777);
      writeFileSync(file, "{}", { mode: 0o644 });
      chmodSync(file, 0o644);
      expect(() => assertOwnerOnly(file)).toThrow("group or others");
      protectOwnerOnly(directory, "directory");
      protectOwnerOnly(file, "file");
      expect(() => assertOwnerOnly(file)).not.toThrow();
      expect([statSync(directory).mode & 0o777, statSync(file).mode & 0o777]).toEqual([0o700, 0o600]);
      expect(python).toBeTruthy();
      const oracle = spawnSync(python!, ["-c", "import os,stat,sys; print(' '.join(oct(stat.S_IMODE(os.stat(p).st_mode)) for p in sys.argv[1:]))", directory, file], { encoding: "utf8" });
      expect(oracle.stdout.trim()).toBe("0o700 0o600");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("Windows: protection runs whoami, then icacls with the fixture arguments, then proves the DACL through Get-Acl", () => {
    const [directoryCase] = ownerOnly.icacls.filter((row: { kind: string }) => row.kind === "directory");
    const whoami = ownerOnly.whoami[0];
    const clean = ownerOnly.sddl.find((row: { violations: string[]; sid: string }) => row.violations.length === 0 && row.sid === directoryCase.sid);
    const host = recordingWindowsHost({ whoami: { status: 0, stdout: whoami.stdout }, icacls: { status: 0, stdout: "processed file" }, "powershell.exe": { status: 0, stdout: `${clean.sddl}\r\n` } });
    protectOwnerOnly(directoryCase.path, "directory", host);
    expect(host.calls.map((call) => call.command)).toEqual(["whoami", "icacls", "powershell.exe"]);
    expect(host.calls[1]!.args).toEqual(directoryCase.arguments);
    expect(host.calls[2]!.path).toBe(directoryCase.path);
    expect(host.calls[2]!.args.join(" ")).not.toContain(directoryCase.path);
  });

  test("Windows: a DACL that kept another principal, a failing icacls or a missing whoami is an error, never a silent pass", () => {
    const leaking = ownerOnly.sddl.find((row: { violations: string[] }) => row.violations.includes("grants AU access"));
    const path = ownerOnly.icacls[0].path;
    const base = { whoami: { status: 0, stdout: `"host\\user","${leaking.sid}"\r\n` }, icacls: { status: 0, stdout: "" } };
    expect(() => protectOwnerOnly(path, "file", recordingWindowsHost({ ...base, "powershell.exe": { status: 0, stdout: leaking.sddl } }))).toThrow("grants AU access");
    expect(() => assertOwnerOnly(path, recordingWindowsHost({ ...base, "powershell.exe": { status: 0, stdout: leaking.sddl } }))).toThrow("inherits entries from its parent");
    expect(() => protectOwnerOnly(path, "file", recordingWindowsHost({ ...base, icacls: { status: 5, stdout: "Access is denied." } }))).toThrow("icacls could not restrict");
    expect(() => protectOwnerOnly(path, "file", recordingWindowsHost({}))).toThrow("whoami");
  });
});

describe("process table", () => {
  test("validates the language-neutral process-table fixture", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(load("🧬️schema/📋️process-table/🔣️.json"));
    expect(validate(processTable), JSON.stringify(validate.errors)).toBe(true);
  });

  test("parses POSIX ps and Windows Get-CimInstance output to the same row shape", () => {
    expect(parsePosixProcessTable(processTable.posix.stdout)).toEqual(processTable.posix.rows);
    for (const row of processTable.windows) expect(parseWindowsProcessTable(row.stdout), row.name).toEqual(row.rows);
  });

  test("recognizes an active Semio Tech build on POSIX and Windows command lines", () => {
    expect(ACTIVE_SEMIO_TECH_BUILD.source).toBe(processTable.activeBuild.pattern);
    const rows = [...processTable.posix.rows, ...processTable.windows.flatMap((row: { rows: unknown[] }) => row.rows)] as { pid: number; command: string }[];
    expect(rows.filter((row) => ACTIVE_SEMIO_TECH_BUILD.test(row.command)).map((row) => row.pid)).toEqual(processTable.activeBuild.matches);
  });

  test("the Windows snapshot asks PowerShell for UTF-8 JSON; an unavailable tool yields undefined, never an empty table", () => {
    const calls: string[][] = [];
    const rows = processTableSnapshot("win32", (command, args) => {
      calls.push([command, ...args]);
      return { status: 0, stdout: processTable.windows[0].stdout };
    });
    expect(calls).toEqual([["powershell.exe", ...WINDOWS_PROCESS_TABLE_ARGUMENTS]]);
    expect(WINDOWS_PROCESS_TABLE_ARGUMENTS.at(-1)).toContain("[System.Text.Encoding]::UTF8");
    expect(rows).toEqual(processTable.windows[0].rows);
    expect(processTableSnapshot("win32", () => ({ status: null, stdout: "" }))).toBeUndefined();
    expect(processTableSnapshot("linux", () => ({ status: 1, stdout: "" }))).toBeUndefined();
    expect(processTableSnapshot("win32", () => ({ status: 0, stdout: "{not json" }))).toBeUndefined();
  });

  test("the host snapshot lists this process under its real parent (Node process.ppid oracle)", () => {
    const rows = processTableSnapshot();
    expect(rows).toBeDefined();
    expect(rows!.find((row) => row.pid === process.pid)?.parent).toBe(process.ppid);
  });
});

/** 🧭️ Entrypoints whose import closure the fleet runs: root router (`dev s`), os dev (local hub), hub (Rust + TS), semio MCP, repository cache. */
const ENTRYPOINTS = [
  "📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts",
  "🌎️hub/📦️packages/🦀️rust/📜️script.ts",
  "🌎️hub/📦️packages/🟦️typescript/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts",
];

/** 🕸️ Relative-import closure of `entries` (static and literal dynamic imports), product files only. */
function productClosure(entries: readonly string[]): string[] {
  const seen = new Set<string>(), queue = entries.map((entry) => resolve(repoRoot, entry));
  const resolveImport = (from: string, specifier: string): string | undefined => {
    const base = resolve(dirname(from), specifier);
    return [base, `${base}.ts`, join(base, "🟦️.ts"), join(base, "📜️script.ts")].find((path) => existsSync(path) && statSync(path).isFile());
  };
  while (queue.length) {
    const file = queue.pop()!;
    if (seen.has(file)) continue;
    seen.add(file);
    for (const reference of ts.preProcessFile(readFileSync(file, "utf8"), true, true).importedFiles) {
      const target = reference.fileName.startsWith(".") ? resolveImport(file, reference.fileName) : undefined;
      if (target && /\.(?:ts|tsx|mts|mjs)$/.test(target)) queue.push(target);
    }
  }
  return [...seen].filter((file) => !/[\\/]🧪️tests?[\\/]/u.test(relative(repoRoot, file))).sort();
}

describe("fleet command paths", () => {
  const closure = productClosure(ENTRYPOINTS);
  const offenders = (rule: (source: string) => boolean): string[] => closure.filter((file) => rule(readFileSync(file, "utf8"))).map((file) => relative(repoRoot, file));

  test("the closure is the fleet's real command surface", () => {
    expect(closure.length).toBeGreaterThan(300);
    for (const owner of ["🏃️process/🔐️owner-only/🟦️.ts", "🏃️process/📋️process-table/🟦️.ts", "🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts"]) expect(closure.some((file) => file.endsWith(owner)), owner).toBe(true);
  });

  test("module locations come from fileURLToPath or import.meta.dir, never a file URL's pathname (/C:/… on Windows)", () => {
    expect(offenders((source) => /new URL\([^)]*import\.meta\.url[^)]*\)\.pathname/.test(source))).toEqual([]);
  });

  test("no spawn of the running executable through a shell (unquoted paths with spaces, cmd.exe re-parsing)", () => {
    expect(offenders((source) => /spawn(?:Sync)?\(\s*process\.execPath[^;]*shell:\s*true/.test(source))).toEqual([]);
  });

  test("package runners start through `bun x`, never the `bunx`/`npx`/`npm` launchers (`.cmd` files Windows cannot spawn without a shell)", () => {
    expect(offenders((source) => /\b(?:runCmd|runCmdStatus|tryRun|runTool|runOwnedCommand|spawn|spawnSync)\(\s*["'](?:bunx|npx|npm)["']/.test(source))).toEqual([]);
  });

  test("process-group signals and POSIX process tools appear only in files with a Windows branch", () => {
    expect(offenders((source) => /process\.kill\(\s*-/.test(source) && !source.includes("win32"))).toEqual([]);
    expect(offenders((source) => /spawn(?:Sync)?\(\s*["'](?:ps|lsof|pgrep|pkill)["']/.test(source) && !source.includes("win32"))).toEqual([]);
  });

  test("every launch row runs unchanged in PowerShell, cmd.exe and POSIX shells (only VS Code `${…}` variables)", () => {
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, path), "utf8")) as { configurations: { name: string; command?: string; runtimeExecutable?: string }[] };
      const posixOnly = launch.configurations.filter(({ command = "", runtimeExecutable = "" }) =>
        /(?:^|\s)[A-Z_][A-Z0-9_]*=\S*\s|&&|\|\||;\s|\s\|\s|2>&1|>\s*\/dev\/null|\$[A-Za-z_(]|\$\{(?!workspaceFolder\}|input:|env:|config:|userHome\}|pathSeparator\})|(?:^|\s)(?:nohup|sh|bash|zsh|env|export|source|sleep|kill|lsof)\s/.test(command) || /^(?:sh|bash|zsh)$/.test(runtimeExecutable),
      );
      expect(posixOnly.map((row) => row.name), path).toEqual([]);
    }
  });

  test("credential and local-hub data roots are protected through the owner-only owner, never a bare chmod", () => {
    const owners = [
      "🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts",
      "🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts",
    ];
    for (const owner of owners) {
      const source = readFileSync(join(repoRoot, owner), "utf8");
      expect(source, owner).toContain("protectOwnerOnly(");
      expect(/chmodSync\([^)]*0o[67]00\)/.test(source), owner).toBe(false);
    }
  });
});
