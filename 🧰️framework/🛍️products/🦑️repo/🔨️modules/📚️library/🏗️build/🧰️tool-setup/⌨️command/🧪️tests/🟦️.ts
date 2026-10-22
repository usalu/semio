/** 🧰️ The .NET SDK installation publishes a complete directory only from an installer script that matches its pinned SHA-256, selects the PowerShell script on Windows and stops when cancelled; the installer is a real shell script, not a mock. */
import {afterAll, describe, expect, setDefaultTimeout, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync, mkdtempSync, readFileSync, readdirSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {dotnetInstallerRowV1, dotnetSdkDirectoryV1, installDotnetSdkV1, type DotnetInstallerRowV1} from "../🟦️.ts";

setDefaultTimeout(60000);
const sandbox = mkdtempSync(join(tmpdir(), "dotnet-install-")), windows = process.platform === "win32";
const script = "#!/bin/bash\nwhile [ $# -gt 0 ]; do case \"$1\" in --version) version=\"$2\"; shift 2;; --install-dir) directory=\"$2\"; shift 2;; *) shift;; esac; done\nprintf '#!/bin/sh\\necho %s\\n' \"$version\" > \"$directory/dotnet\"\nchmod 755 \"$directory/dotnet\"\n";
const row = (text: string): DotnetInstallerRowV1 => ({tool: "dotnet-install", version: "x", platform: "any", arch: "any", url: "https://example.invalid/dotnet-install.sh", sha256: createHash("sha256").update(text).digest("hex")});
const port = (text: string, delay = 0) => ({platform: process.platform, offline: false, environment: {SEMIO_LANGUAGE: "en"}, fetch: async (_url: string, signal: AbortSignal) => { await new Promise(resolve => setTimeout(resolve, delay)); signal.throwIfAborted(); return new Response(text); }, run: async () => undefined, capture: async () => "", log: () => undefined});
const run = (command: string, args: string[]) => { const result = spawnSync(command, args, {encoding: "utf8"}); if (result.status !== 0) throw new Error(result.stderr); return Promise.resolve(); };

afterAll(() => rmSync(sandbox, {recursive: true, force: true}));

describe("the pinned .NET installer rows", () => {
  test("select the shell script everywhere except Windows, where the PowerShell script of the architecture is used", () => {
    expect(dotnetInstallerRowV1("linux", "x64")?.url.endsWith("dotnet-install.sh")).toBe(true);
    expect(dotnetInstallerRowV1("darwin", "arm64")?.url.endsWith("dotnet-install.sh")).toBe(true);
    expect(dotnetInstallerRowV1("win32", "x64")?.url.endsWith("dotnet-install.ps1")).toBe(true);
    expect(dotnetInstallerRowV1("win32", "arm64")?.url.endsWith("dotnet-install.ps1")).toBe(true);
    expect(dotnetInstallerRowV1("win32", "ia32")).toBeUndefined();
    for (const platform of ["linux", "darwin", "win32"] as const) expect(dotnetInstallerRowV1(platform, "x64")?.url).toMatch(/^https:\/\/raw\.githubusercontent\.com\/dotnet\/install-scripts\/[a-f0-9]{40}\/src\//u);
  });

  test("name the directory by SDK, host and the digest of the script", () => {
    expect(dotnetSdkDirectoryV1("/t", "8.0.421", "linux-x64", "ab".repeat(32)).replaceAll("\\", "/")).toBe("/t/native/dotnet/8.0.421/linux-x64-abababababababab");
  });
});

describe.skipIf(windows)("the installation of the SDK", () => {
  test("publishes the installed directory with a receipt of its executable, once", async () => {
    const directory = join(sandbox, "tools", "sdk"), installer = row(script);
    await installDotnetSdkV1("8.0.421", directory, installer.sha256, installer, "linux-x64", {SEMIO_LANGUAGE: "en"}, port(script), AbortSignal.timeout(30000), (command, args) => run(command, args));
    expect(spawnSync(join(directory, "dotnet"), [], {encoding: "utf8"}).stdout.trim()).toBe("8.0.421");
    const receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
    expect(receipt).toMatchObject({owner: "workspace:native-tools", identity: `dotnet:8.0.421:linux-x64:${installer.sha256}`});
    expect(Object.keys(receipt.files)).toEqual(["dotnet"]);
    expect(readdirSync(join(sandbox, "tools")).filter(name => name.startsWith(".prepare-"))).toEqual([]);
  });

  test("refuses a script that differs from its pinned SHA-256 and publishes nothing", async () => {
    const directory = join(sandbox, "tools2", "sdk"), installer = row(script);
    await expect(installDotnetSdkV1("8.0.421", directory, installer.sha256, installer, "linux-x64", {SEMIO_LANGUAGE: "de"}, port(script + "# tampered"), AbortSignal.timeout(30000), (command, args) => run(command, args))).rejects.toThrow(/Installationsprogramm für dotnet fehlt/u);
    expect(existsSync(directory)).toBe(false);
    expect(readdirSync(join(sandbox, "tools2")).filter(name => name.startsWith(".prepare-"))).toEqual([]);
  });

  test("refuses an installer that installs no executable and stops when cancelled", async () => {
    const empty = "#!/bin/bash\nexit 0\n", installer = row(empty), directory = join(sandbox, "tools3", "sdk");
    await expect(installDotnetSdkV1("8.0.421", directory, installer.sha256, installer, "linux-x64", {SEMIO_LANGUAGE: "en"}, port(empty), AbortSignal.timeout(30000), (command, args) => run(command, args))).rejects.toThrow(/pinned installer/u);
    const controller = new AbortController(), slow = row(script);
    setTimeout(() => controller.abort(new Error("cancelled")), 10);
    await expect(installDotnetSdkV1("8.0.421", join(sandbox, "tools4", "sdk"), slow.sha256, slow, "linux-x64", {SEMIO_LANGUAGE: "en"}, port(script, 200), controller.signal, (command, args) => run(command, args))).rejects.toThrow(/cancelled/u);
    expect(existsSync(join(sandbox, "tools4", "sdk"))).toBe(false);
  });
});
