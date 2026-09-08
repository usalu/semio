import { createHash } from "node:crypto";
import { readFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, lstatSync, writeFileSync, rmSync } from "node:fs";
import { open } from "node:fs/promises";
import { join, dirname, relative } from "node:path";
import { spawn } from "node:child_process";
import { BundleScript, ScriptRouter } from "../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { stageArtifacts } from "../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";

export type TectonicDistribution = { readonly platform: string; readonly architecture: string; readonly target: string; readonly archive: string; readonly sha256: string; readonly bytes: number };
const manifest: { version: string; release: string; platforms: TectonicDistribution[] } = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
const owner = "@semio-tech/print:deps-tectonic";
const digest = (file: string): string => createHash("sha256").update(readFileSync(file)).digest("hex");

/** 🧭️ Selects an explicitly published distribution for the current host. */
export function tectonicDistribution(platform = process.platform as string, architecture = process.arch as string): TectonicDistribution {
  const distribution = manifest.platforms.find((row) => row.platform === platform && row.architecture === architecture);
  if (!distribution) throw new Error(`Unsupported Tectonic host: ${platform}/${architecture}`);
  return distribution;
}

/** 📦️ Keeps compiler installations separate by pinned version and native target. */
export function tectonicBinaryPath(workspace = getWorkspaceRoot()): string {
  const distribution = tectonicDistribution();
  return join(workspace, ".🧬semio/🦑️repo/⚡️cache/tools/tectonic", manifest.version, distribution.target, "tectonic" + (distribution.platform === "win32" ? ".exe" : ""));
}

/** 🔐️ Checks the installed tool against its verified publication receipt. */
export function preparedTectonic(workspace = getWorkspaceRoot()): string {
  const binary = tectonicBinaryPath(workspace), directory = dirname(binary), distribution = tectonicDistribution();
  const marker = JSON.parse(readFileSync(join(directory, ".nx-artifact.json"), "utf8"));
  const receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
  if (marker.owner !== owner || receipt.version !== manifest.version || receipt.target !== distribution.target || receipt.archiveSha256 !== distribution.sha256 || !Object.hasOwn(receipt.files, relative(directory, binary))) throw new Error("Tectonic installation identity is invalid");
  for (const [name, sha256] of Object.entries(receipt.files)) {
    if (name.startsWith("/") || name.split(/[\\/]/).includes("..") || digest(join(directory, name)) !== sha256) throw new Error(`Tectonic installation checksum is invalid: ${name}`);
  }
  return binary;
}

/** 📥️ Acquires and verifies the pinned archive before publishing its executable files. */
export async function prepareTectonic(workspace = getWorkspaceRoot(), signal?: AbortSignal): Promise<string> {
  const binary = tectonicBinaryPath(workspace), directory = dirname(binary), distribution = tectonicDistribution();
  if (existsSync(directory)) return preparedTectonic(workspace);
  mkdirSync(dirname(directory), { recursive: true });
  const temporary = mkdtempSync(join(dirname(directory), ".prepare-"));
  try {
    signal?.throwIfAborted();
    console.log(`[print-toolchain] Downloading Tectonic ${manifest.version} for ${distribution.target}`);
    const response = await fetch(`${manifest.release}/${distribution.archive}`, { signal });
    if (!response.ok || !response.body) throw new Error(`Tectonic archive download failed: ${response.status}`);
    const archive = join(temporary, distribution.archive), file = await open(archive, "wx"), hash = createHash("sha256");
    let bytes = 0;
    try {
      for await (const chunk of response.body) {
        signal?.throwIfAborted(); bytes += chunk.byteLength;
        if (bytes > distribution.bytes) throw new Error("Tectonic archive exceeds its published size");
        hash.update(chunk); await file.writeFile(chunk);
      }
    } finally { await file.close(); }
    if (bytes !== distribution.bytes || hash.digest("hex") !== distribution.sha256) throw new Error("Tectonic archive checksum does not match the pinned release");
    console.log(`[print-toolchain] Verified ${bytes} archive bytes; extracting`);
    const extracted = join(temporary, "extracted"); mkdirSync(extracted);
    await new Promise<void>((accept, reject) => {
      const child = spawn("tar", ["-xf", archive, "-C", extracted], { signal, stdio: "inherit" });
      let failure: Error | undefined;
      child.once("error", (error) => { failure = error; });
      child.once("close", (code) => failure ? reject(failure) : code === 0 ? accept() : reject(new Error(`Tectonic archive extraction failed: ${code}`)));
    });
    const files = new Map<string, string>();
    const visit = (root: string): void => {
      for (const name of readdirSync(root)) {
        const file = join(root, name), stat = lstatSync(file);
        if (stat.isDirectory()) visit(file);
        else if (stat.isFile()) files.set(relative(extracted, file).replaceAll("\\", "/"), file);
        else throw new Error(`Tectonic archive contains an unsupported node: ${name}`);
      }
    };
    visit(extracted);
    if (!files.has(relative(directory, binary))) throw new Error("Tectonic archive has no executable");
    const receipt = join(temporary, ".toolchain.json");
    writeFileSync(receipt, JSON.stringify({ version: manifest.version, target: distribution.target, archiveSha256: distribution.sha256, files: Object.fromEntries([...files].map(([name, file]) => [name, digest(file)])) }) + "\n");
    files.set(".toolchain.json", receipt);
    signal?.throwIfAborted();
    if (!existsSync(directory)) stageArtifacts(directory, owner, files);
    console.log(`[print-toolchain] Ready: ${distribution.target}`);
    return preparedTectonic(workspace);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}

class PrepareScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Tectonic preparation accepts no arguments");
    const controller = new AbortController(), abort = (): void => controller.abort(new Error("Tectonic preparation cancelled"));
    process.once("SIGINT", abort); process.once("SIGTERM", abort);
    try { await prepareTectonic(this.repoRoot, controller.signal); }
    finally { process.removeListener("SIGINT", abort); process.removeListener("SIGTERM", abort); }
  }
}

class VerifyScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw new Error("Tectonic verification accepts no arguments");
    const child = Bun.spawnSync([preparedTectonic(this.repoRoot), "--version"], { stdout: "pipe", stderr: "pipe", timeout: 10000 });
    if (child.exitCode !== 0 || child.stdout.toString().trim() !== `Tectonic ${manifest.version}`) throw new Error(`Prepared Tectonic version mismatch: ${child.stdout.toString()}${child.stderr.toString()}`);
    console.log(`[DEBUG] Verified published Tectonic ${manifest.version} executable PASS`);
  }
}

const router = new ScriptRouter(import.meta.dir).register("prepare", PrepareScript).register("verify", VerifyScript);
if (import.meta.main) await router.run(process.argv.slice(2));
