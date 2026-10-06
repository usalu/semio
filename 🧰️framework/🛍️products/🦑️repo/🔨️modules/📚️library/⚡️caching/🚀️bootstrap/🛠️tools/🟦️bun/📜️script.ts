import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { inflateRawSync } from "node:zlib";
import { runTool } from "../../📦️dependencies/📜️script.ts";
import { withResourceLeases } from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

type BunDistribution = { platform: string; arch: string; musl: boolean; archive: string; bytes: number; sha256: string };
const manifest = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as { version: string; release: string; distributions: BunDistribution[] };

/** 🧭️ Selects the authored portable Bun release without changing the installed runtime. */
export function bunDistribution(version: string, platform: string, arch: string, musl: boolean): BunDistribution {
  const row = version === manifest.version && manifest.distributions.find(row => row.platform === platform && row.arch === arch && row.musl === musl);
  if (!row) throw new Error(`Unsupported pinned Bun: ${version} ${platform}/${arch}${musl ? "/musl" : ""}`);
  return row;
}

/** 📦️ Extracts only the executable from a checksum-verified ordinary ZIP release. */
export function bunArchiveExecutable(archive: Buffer, member: string): Buffer {
  for (let at = Math.max(0, archive.length - 22); at >= Math.max(0, archive.length - 65557); at--) {
    if (archive.readUInt32LE(at) !== 0x06054b50) continue;
    let cursor = archive.readUInt32LE(at + 16);
    for (let index = 0; index < archive.readUInt16LE(at + 10); index++) {
      if (archive.readUInt32LE(cursor) !== 0x02014b50) throw new Error("Invalid Bun ZIP directory");
      const nameLength = archive.readUInt16LE(cursor + 28), name = archive.subarray(cursor + 46, cursor + 46 + nameLength).toString();
      if (name === member) {
        const flags = archive.readUInt16LE(cursor + 8), method = archive.readUInt16LE(cursor + 10), size = archive.readUInt32LE(cursor + 20), expanded = archive.readUInt32LE(cursor + 24), local = archive.readUInt32LE(cursor + 42);
        if (flags & 1 || ![0, 8].includes(method) || expanded > 256 * 1024 * 1024 || archive.readUInt32LE(local) !== 0x04034b50) throw new Error("Invalid Bun ZIP executable");
        const start = local + 30 + archive.readUInt16LE(local + 26) + archive.readUInt16LE(local + 28), compressed = archive.subarray(start, start + size);
        const payload = method === 0 ? Buffer.from(compressed) : inflateRawSync(compressed, { maxOutputLength: expanded });
        if (compressed.length !== size || payload.length !== expanded) throw new Error("Invalid Bun ZIP size");
        return payload;
      }
      cursor += 46 + nameLength + archive.readUInt16LE(cursor + 30) + archive.readUInt16LE(cursor + 32);
    }
    break;
  }
  throw new Error("Bun archive has no selected executable");
}

/** 🔐️ Acquires and atomically publishes a pinned workspace-owned installer with cancellation. */
export async function prepareBun(workspace: string, version: string, signal: AbortSignal): Promise<string> {
  signal.throwIfAborted();
  if (process.versions.bun === version) return process.execPath;
  const report = process.platform === "linux" ? process.report?.getReport?.() as { header?: { glibcVersionRuntime?: string } } : undefined;
  const row = bunDistribution(version, process.platform, process.arch, process.platform === "linux" && !report?.header?.glibcVersionRuntime);
  const base = join(workspace, ".🧬semio/🦑️repo/⚡️cache/tools/bun"), directory = join(base, row.sha256), executable = join(directory, process.platform === "win32" ? "bun.exe" : "bun");
  for (let current = base; ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid Bun directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) break;
  }
  mkdirSync(base, { recursive: true });
  return withResourceLeases({ directory: join(workspace, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), signal, resources: [{ resource: `bun-${row.sha256}`, mode: "exclusive" }] }, async () => {
    const validate = async (): Promise<string> => {
      const stat = lstatSync(executable), receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
      if (!stat.isFile() || stat.isSymbolicLink() || lstatSync(directory).isSymbolicLink() || receipt.archive !== row.sha256 || receipt.executable !== createHash("sha256").update(readFileSync(executable)).digest("hex")) throw new Error("Bun installation checksum mismatch");
      if ((await runTool(executable, ["--version"], directory, signal, true)).trim() !== version) throw new Error("Bun executable version mismatch");
      return executable;
    };
    try { lstatSync(directory); return await validate(); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    const staging = mkdtempSync(join(base, ".prepare-")), progress = setInterval(() => console.log(`Acquiring pinned Bun ${version}…`), 10000);
    try {
      console.log(`Acquiring pinned Bun ${version} for ${row.platform}/${row.arch}…`);
      const response = await fetch(`${manifest.release}/${row.archive}`, { signal });
      if (!response.ok || !response.body) throw new Error(`Bun download failed: ${response.status}`);
      const chunks: Buffer[] = [], hash = createHash("sha256"), reader = response.body.getReader(); let size = 0;
      try { for (let read = await reader.read(); !read.done; read = await reader.read()) { signal.throwIfAborted(); size += read.value.length; if (size > row.bytes) throw new Error("Bun archive exceeds its pinned size"); const chunk = Buffer.from(read.value); chunks.push(chunk); hash.update(chunk); } }
      catch (error) { await reader.cancel().catch(() => undefined); throw error; }
      if (size !== row.bytes || hash.digest("hex") !== row.sha256) throw new Error("Bun archive checksum mismatch");
      const payload = bunArchiveExecutable(Buffer.concat(chunks), `${row.archive.slice(0, -4)}/bun${process.platform === "win32" ? ".exe" : ""}`), staged = join(staging, process.platform === "win32" ? "bun.exe" : "bun");
      writeFileSync(staged, payload, { flag: "wx" }); chmodSync(staged, 0o755);
      if ((await runTool(staged, ["--version"], staging, signal, true)).trim() !== version) throw new Error("Bun release version mismatch");
      writeFileSync(join(staging, ".toolchain.json"), JSON.stringify({ archive: row.sha256, executable: createHash("sha256").update(payload).digest("hex") }));
      signal.throwIfAborted(); renameSync(staging, directory); return executable;
    } finally { clearInterval(progress); rmSync(staging, { recursive: true, force: true }); }
  });
}
