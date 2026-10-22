import assert from "node:assert/strict";
import { constants } from "node:fs";
import { lstat, open } from "node:fs/promises";
import { join, resolve, dirname } from "node:path";
import { homedir } from "node:os";
import { createHash } from "node:crypto";
import { performance } from "node:perf_hooks";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { resolveMcpBin } from "../../🚨️policy/🖥️cli/🟦️.ts";
import installationSchema from "../../../💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json" with { type: "json" };

const installation = Object.freeze(installationSchema.$defs.Protocol.const);
type Receipt = Readonly<{ version: 1; platform: "darwin" | "linux" | "win32"; digest: string; executable: string; directory: string; source: string; mode: "prototype" | "release" }>;
type FileProof = Readonly<{ path: string; ancestry: readonly string[]; stamp: string; digest: string; bytes: number; header: string; text?: string }>;
type Provenance = Readonly<{ receipt: FileProof; pin: FileProof; executable: FileProof; profile: FileProof; record: Receipt }>;

function physicalPath(value: string): string {
  assert.equal(resolve(value), value, "Absolute canonical path required");
  assert.ok(value.length <= installation.maximumPathLength && [...value].length <= installation.maximumPathLength && !value.includes("\0"), "Bounded canonical path required");
  return value;
}
function stamp(facts: Readonly<{ dev: number; ino: number; mode: number; nlink: number; size: number; mtimeMs: number; ctimeMs: number }>): string {
  return [facts.dev, facts.ino, facts.mode, facts.nlink, facts.size, facts.mtimeMs, facts.ctimeMs].join(":");
}
async function parents(file: string, control: ResourceLeaseControl): Promise<readonly string[]> {
  const identities: string[] = [];
  for (let directory = dirname(file);; directory = dirname(directory)) {
    checkResourceLeaseControl(control); const facts = await lstat(physicalPath(directory));
    assert.ok(facts.isDirectory() && !facts.isSymbolicLink(), "Physical installation ancestry required");
    identities.push([directory, facts.dev, facts.ino, facts.mode].join(":"));
    if (dirname(directory) === directory) return Object.freeze(identities);
  }
}
async function fileProof(file: string, maximum: number, control: ResourceLeaseControl, text = false): Promise<FileProof> {
  physicalPath(file); checkResourceLeaseControl(control); const ancestry = await parents(file, control), proofStarted = performance.now();
  const before = await lstat(file); assert.ok(before.isFile() && !before.isSymbolicLink() && before.nlink === 1 && before.size > 0 && before.size <= maximum, "Bounded physical publication required");
  const handle = await open(file, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    assert.equal(stamp(await handle.stat()), stamp(before), "Captured publication descriptor changed");
    const hash = createHash("sha256"), buffer = Buffer.alloc(65536), chunks: Buffer[] = [];
    let offset = 0, header = "";
    while (offset < before.size) {
      checkResourceLeaseControl(control);
      const result = await handle.read(buffer, 0, Math.min(buffer.length, before.size - offset), offset);
      assert.ok(result.bytesRead > 0, "Complete publication bytes required");
      const chunk = buffer.subarray(0, result.bytesRead); if (!offset) header = chunk.subarray(0, 4).toString("hex");
      hash.update(chunk); if (text) chunks.push(Buffer.from(chunk)); offset += result.bytesRead;
      if (offset % 1048576 === 0) { await control.publish({ resource: "test:installed-frontend:provenance", mode: "shared", elapsedMs: Math.floor(performance.now() - proofStarted) }); await control.yieldContinuation(); }
    }
    assert.equal(stamp(await handle.stat()), stamp(before), "Publication descriptor changed during read");
    assert.equal(stamp(await lstat(file)), stamp(before), "Publication path changed during read");
    assert.deepEqual(await parents(file, control), ancestry, "Installation ancestry changed during read");
    checkResourceLeaseControl(control);
    return Object.freeze({ path: file, ancestry, stamp: stamp(before), digest: hash.digest("hex"), bytes: offset, header, ...(text ? { text: Buffer.concat(chunks).toString("utf8") } : {}) });
  } finally { await handle.close(); }
}
function nativeHeader(header: string, platform: Receipt["platform"]): boolean {
  return platform === "win32" ? header.startsWith("4d5a") : platform === "linux" ? header === "7f454c46" : ["cffaedfe", "cefaedfe", "feedfacf", "feedface", "cafebabe", "bebafeca", "cafebabf", "bfbafeca"].includes(header);
}
export async function captureInstalledFrontendProofV1(root: string, control: ResourceLeaseControl, environment: Readonly<Record<string, string | undefined>>): Promise<Provenance> {
  assert.ok(["darwin", "linux", "win32"].includes(process.platform), "Qualified native frontend platform required");
  assert.ok(!environment.REPO_MCP_BIN && !environment.REPO_CLI_BIN && !process.env.REPO_MCP_BIN && !process.env.REPO_CLI_BIN, "Canonical frontend profile required without binary overrides");
  const platform = process.platform as Receipt["platform"], home = homedir(), local = environment.LOCALAPPDATA;
  if (platform === "win32") assert.ok(local, "Official LOCALAPPDATA installation root required");
  const installationRoot = platform === "win32" ? join(local!, ...installation.windowsDirectory) : join(home, ".local", "share", "semio", "repo");
  const receipt = await fileProof(join(installationRoot, "installed.json"), 4096, control, true), record = JSON.parse(receipt.text!) as Receipt;
  assert.deepEqual(Object.keys(record).sort(), ["version", "platform", "digest", "executable", "directory", "source", "mode"].sort());
  assert.equal(record.version, 1); assert.equal(record.platform, platform); assert.match(record.digest, /^[a-f0-9]{64}$/u); assert.ok(record.mode === "prototype" || record.mode === "release");
  assert.equal(record.mode, environment.SEMIO_BUILD_MODE, "Requested canonical frontend profile required");
  const directory = platform === "win32" ? join(installationRoot, record.digest) : join(home, ...installation.unixDirectory);
  assert.equal(record.directory, directory); assert.equal(record.executable, join(directory, platform === "win32" ? "repo.exe" : "repo"));
  assert.equal(record.source, resolveMcpBin(root, record.mode));
  const pin = await fileProof(join(root, ".🧬semio", "🦑️repo", "native-repo-path"), 4096, control, true);
  assert.equal(pin.text, record.executable.replaceAll("\\", "/") + "\n");
  const profile = await fileProof(record.source, installation.maximumBinaryBytes, control), executable = await fileProof(record.executable, installation.maximumBinaryBytes, control);
  assert.equal(profile.digest, record.digest); assert.equal(executable.digest, record.digest);
  assert.ok(nativeHeader(profile.header, platform) && nativeHeader(executable.header, platform), "Native profile and installed executable required");
  return Object.freeze({ receipt, pin, profile, executable, record: Object.freeze(record) });
}
export function assertSameInstalledFrontendProofV1(before: Provenance, after: Provenance): void {
  assert.deepEqual(after.record, before.record);
  for (const key of ["receipt", "pin", "profile", "executable"] as const) assert.deepEqual(after[key], before[key], "Canonical publication changed during protocol probe");
}

