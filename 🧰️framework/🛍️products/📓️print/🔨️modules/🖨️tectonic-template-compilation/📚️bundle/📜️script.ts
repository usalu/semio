import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, rmSync } from "node:fs";
import { join, dirname } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { stageArtifacts } from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

type BundleFile = { name: string; offset: number; bytes: number; sha256: string };
const source = readFileSync(new URL("./🔒️dependencies.json", import.meta.url));
const manifest: { version: number; url: string; archiveBytes: number; files: BundleFile[] } = JSON.parse(source.toString());
const hash = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const identity = hash(source), owner = "@semio-tech/print:deps-tex", names = new Set<string>();
if (manifest.version !== 1 || !manifest.url.startsWith("https://") || !Number.isSafeInteger(manifest.archiveBytes) || manifest.archiveBytes < 1 || !manifest.files.length) throw new Error("Invalid Print bundle manifest");
for (const file of manifest.files) {
  if (!file.name || /[/\\]/.test(file.name) || [".", "..", "SHA256SUM", ".nx-artifact.json"].includes(file.name) || names.has(file.name) || !/^[a-f0-9]{64}$/.test(file.sha256) || !Number.isSafeInteger(file.offset) || file.offset < 0 || !Number.isSafeInteger(file.bytes) || file.bytes < 0 || file.bytes > 16777216 || file.offset + file.bytes > manifest.archiveBytes) throw new Error(`Invalid Print bundle file: ${file.name}`);
  names.add(file.name);
}

/** 📚️ Uses one content-addressed TeX support directory across all Print documents. */
export function printBundleDirectory(workspace = getWorkspaceRoot()): string {
  return join(workspace, ".🧬semio/🦑️repo/⚡️cache/tools/tectonic/bundles", identity);
}

/** 🔏️ Admits only the exact pinned support files before a compiler can consume them. */
export function preparedPrintBundle(workspace = getWorkspaceRoot()): string {
  const directory = printBundleDirectory(workspace);
  if (JSON.parse(readFileSync(join(directory, ".nx-artifact.json"), "utf8")).owner !== owner || readFileSync(join(directory, "SHA256SUM"), "utf8").trim() !== identity || readdirSync(directory).length !== names.size + 2) throw new Error("Print bundle identity is invalid");
  for (const file of manifest.files) if (hash(readFileSync(join(directory, file.name))) !== file.sha256) throw new Error(`Print bundle checksum mismatch: ${file.name}`);
  return directory;
}

async function acquireFile(file: BundleFile, signal: AbortSignal): Promise<Buffer> {
  signal.throwIfAborted();
  if (file.bytes === 0) {
    const content = Buffer.alloc(0);
    if (hash(content) !== file.sha256) throw new Error(`Print bundle checksum mismatch: ${file.name}`);
    return content;
  }
  const range = `${file.offset}-${file.offset + file.bytes - 1}`;
  const response = await fetch(manifest.url, { headers: { Range: `bytes=${range}` }, signal });
  if (response.status !== 206 || response.headers.get("content-range") !== `bytes ${range}/${manifest.archiveBytes}` || !response.body) {
    await response.body?.cancel();
    throw new Error(`Print bundle range rejected: ${file.name} (${response.status})`);
  }
  const chunks: Uint8Array[] = []; let bytes = 0;
  for await (const chunk of response.body) {
    signal.throwIfAborted(); bytes += chunk.byteLength;
    if (bytes > file.bytes) throw new Error(`Print bundle range length exceeded: ${file.name}`);
    chunks.push(chunk);
  }
  const content = Buffer.concat(chunks);
  if (bytes !== file.bytes || hash(content) !== file.sha256) throw new Error(`Print bundle checksum mismatch: ${file.name}`);
  return content;
}

/** 📥️ Verifies bounded HTTP ranges and publishes a complete local compiler bundle. */
export async function preparePrintBundle(workspace = getWorkspaceRoot(), signal?: AbortSignal): Promise<string> {
  signal?.throwIfAborted();
  const directory = printBundleDirectory(workspace);
  if (existsSync(directory)) return preparedPrintBundle(workspace);
  mkdirSync(dirname(directory), { recursive: true });
  const temporary = mkdtempSync(join(dirname(directory), ".prepare-")), controller = new AbortController();
  const abort = (): void => controller.abort(signal?.reason);
  signal?.addEventListener("abort", abort, { once: true });
  let index = 0, completed = 0, failure: unknown;
  try {
    console.log(`[print-bundle] Acquiring ${manifest.files.length} verified TeX support files`);
    const workers = Array.from({ length: 4 }, async () => {
      try {
        while (index < manifest.files.length) {
          const file = manifest.files[index++]!, content = await acquireFile(file, controller.signal);
          writeFileSync(join(temporary, file.name), content);
          completed++;
          if (completed % 25 === 0) console.log(`[print-bundle] Verified ${completed}/${manifest.files.length}`);
        }
      } catch (error) { failure ??= error; controller.abort(error); }
    });
    await Promise.all(workers);
    if (failure) throw failure;
    signal?.throwIfAborted();
    writeFileSync(join(temporary, "SHA256SUM"), `${identity}\n`);
    if (!existsSync(directory)) stageArtifacts(directory, owner, new Map([...names, "SHA256SUM"].map(name => [name, join(temporary, name)])));
    console.log(`[print-bundle] Ready: ${manifest.files.length} files, ${manifest.files.reduce((sum, file) => sum + file.bytes, 0)} bytes`);
    return preparedPrintBundle(workspace);
  } finally { signal?.removeEventListener("abort", abort); rmSync(temporary, { recursive: true, force: true }); }
}

class PrepareScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Print bundle preparation accepts no arguments");
    const controller = new AbortController(), abort = (): void => controller.abort(new Error("Print bundle preparation cancelled"));
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    try { await preparePrintBundle(this.repoRoot, controller.signal); }
    finally { process.removeListener("SIGINT", abort); process.removeListener("SIGTERM", abort); }
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PrepareScript);
if (import.meta.main) await router.run(process.argv.slice(2));
