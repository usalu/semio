import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { open } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { Script, ScriptRouter } from "../../../../🏃️process/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { withResourceLeases } from "../../../🔒️leases/🟦️.ts";
import { runTool } from "../../📦️dependencies/📜️script.ts";
import binaryenManifest from "./🔣️.json";

export type BinaryenDistribution = { readonly platform: string; readonly architecture: string; readonly archive: string; readonly bytes: number; readonly sha256: string };
const manifest: { version: string; release: string; platforms: BinaryenDistribution[] } = binaryenManifest;
const store = ".🧬semio/🦑️repo/⚡️cache/tools/binaryen", owner = "workspace:deps-wasm-opt";
const digest = (path: string): string => createHash("sha256").update(readFileSync(path)).digest("hex");

/** 🧭️ Selects a published, checksum-pinned native distribution. */
export function binaryenDistribution(platform = process.platform as string, architecture = process.arch as string): BinaryenDistribution {
  const row = manifest.platforms.find(row => row.platform === platform && row.architecture === architecture);
  if (!row) throw new Error(`Unsupported Binaryen host: ${platform}/${architecture}`);
  return row;
}

/** 🪪️ Hashes the intended toolchain independently of installation state. */
export function binaryenIdentity(platform = process.platform as string, architecture = process.arch as string): string {
  return `binaryen:${manifest.version}:${platform}/${architecture}:${binaryenDistribution(platform, architecture).sha256}`;
}

/** 📦️ Retains the optimizer and its dynamic libraries while rejecting nonportable archive paths. */
export function binaryenMembers(members: readonly string[], platform: string): string[] {
  const prefix = `binaryen-version_${manifest.version}/`, selected: string[] = [];
  for (const member of members) {
    if (!member.startsWith(prefix) || /[\\:\x00-\x1f]/.test(member) || member.replace(/\/$/, "").split("/").some(part => !part || part === "." || part === "..")) throw new Error(`Invalid Binaryen archive member: ${member}`);
    const path = member.slice(prefix.length);
    if (path === `bin/wasm-opt${platform === "win32" ? ".exe" : ""}` || /^(?:bin|lib)\/[a-zA-Z0-9_.+-]+(?:\.dylib|\.dll|\.so(?:\.[0-9]+)*)$/.test(path)) selected.push(path);
  }
  if (new Set(selected).size !== selected.length) throw new Error("Invalid duplicate Binaryen archive members");
  return selected;
}

/** 🛣️ Keeps immutable native tools outside compiler and task-result stores. */
export function binaryenDirectory(workspace: string): string {
  const row = binaryenDistribution();
  return join(workspace, store, manifest.version, `${row.platform}-${row.architecture}-${row.sha256}`);
}

/** 🔒️ Rejects substituted directories before reading or publishing tool state. */
function regularDirectory(path: string): void {
  for (let current = resolve(path); ; current = dirname(current)) {
    try { const stat = lstatSync(current); if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Invalid Binaryen directory: ${current}`); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (dirname(current) === current) return;
  }
}

/** 🔐️ Checks the optimizer and every required library against the verified archive receipt. */
export function preparedBinaryen(workspace: string): string {
  const directory = binaryenDirectory(workspace), executable = `bin/wasm-opt${process.platform === "win32" ? ".exe" : ""}`;
  regularDirectory(directory);
  const receipt = JSON.parse(readFileSync(join(directory, ".toolchain.json"), "utf8"));
  if (receipt.owner !== owner || receipt.identity !== binaryenIdentity() || !receipt.files?.[executable]) throw new Error("Binaryen installation identity mismatch");
  const names = Object.keys(receipt.files);
  if (binaryenMembers(names.map(path => `binaryen-version_${manifest.version}/${path}`), process.platform).length !== names.length) throw new Error("Invalid Binaryen receipt paths");
  for (const name of names) {
    const path = join(directory, name); regularDirectory(dirname(path));
    const stat = lstatSync(path);
    if (!stat.isFile() || stat.isSymbolicLink() || digest(path) !== receipt.files[name]) throw new Error(`Binaryen installation checksum mismatch: ${name}`);
  }
  return join(directory, executable);
}

/** 📥️ Acquires one pinned optimizer in an isolated directory and publishes it atomically. */
export async function prepareBinaryen(workspace: string, signal: AbortSignal): Promise<string> {
  signal.throwIfAborted();
  const row = binaryenDistribution(), directory = binaryenDirectory(workspace), parent = dirname(directory);
  regularDirectory(parent); mkdirSync(parent, { recursive: true });
  return withResourceLeases({ directory: join(workspace, ".🧬semio/🦑️repo/⚡️cache/agents/resource-leases"), signal, resources: [{ resource: binaryenIdentity(), mode: "exclusive" }] }, async () => {
    let present = true;
    try { lstatSync(directory); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; present = false; }
    if (present) return preparedBinaryen(workspace);
    const temporary = mkdtempSync(join(parent, ".prepare-"));
    const progress = setInterval(() => console.log(`Preparing Binaryen ${manifest.version}…`), 10000);
    try {
      console.log(`Downloading pinned Binaryen ${manifest.version} for ${row.platform}/${row.architecture}…`);
      const response = await fetch(`${manifest.release}/${row.archive}`, { signal });
      if (!response.ok || !response.body) throw new Error(`Binaryen download failed: ${response.status}`);
      const archive = join(temporary, "binaryen.tar.gz"), file = await open(archive, "wx"), hash = createHash("sha256");
      let bytes = 0;
      try {
        for await (const chunk of response.body) {
          signal.throwIfAborted(); bytes += chunk.byteLength;
          if (bytes > row.bytes) throw new Error("Binaryen archive exceeds its pinned size");
          hash.update(chunk); await file.writeFile(chunk);
        }
      } finally { await file.close(); }
      if (bytes !== row.bytes || hash.digest("hex") !== row.sha256) throw new Error("Binaryen archive checksum mismatch");
      const members = (await runTool("tar", ["-tzf", archive], temporary, signal, true)).trim().split(/\r?\n/);
      const selected = binaryenMembers(members, process.platform), prefix = `binaryen-version_${manifest.version}`;
      const executable = `bin/wasm-opt${process.platform === "win32" ? ".exe" : ""}`;
      if (!selected.includes(executable)) throw new Error("Binaryen archive has no optimizer");
      await runTool("tar", ["-xzf", archive, "-C", temporary, ...selected.map(path => `${prefix}/${path}`)], temporary, signal);
      const payload = join(temporary, prefix), files: Record<string, string> = {};
      for (const name of selected) {
        const path = join(payload, name); regularDirectory(dirname(path));
        if (!lstatSync(path).isFile() || lstatSync(path).isSymbolicLink()) throw new Error(`Invalid Binaryen payload: ${name}`);
        files[name] = digest(path);
      }
      const version = await runTool(join(payload, executable), ["--version"], temporary, signal, true);
      if (version.trim() !== `wasm-opt version ${manifest.version} (version_${manifest.version})`) throw new Error(`Binaryen version mismatch: ${version.trim()}`);
      writeFileSync(join(payload, ".toolchain.json"), JSON.stringify({ owner, identity: binaryenIdentity(), files }) + "\n");
      signal.throwIfAborted(); renameSync(payload, directory);
      return preparedBinaryen(workspace);
    } finally { clearInterval(progress); rmSync(temporary, { recursive: true, force: true }); }
  });
}

class PrepareScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Optimizer preparation accepts no arguments");
    const controller = new AbortController(), stop = (): void => controller.abort(new Error("Optimizer preparation cancelled"));
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    try { console.log(`Prepared ${await prepareBinaryen(this.root, AbortSignal.any([controller.signal, AbortSignal.timeout(300000)]))}`); }
    finally { process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop); }
  }
}

/** 🔏️ Hashes tool versions without application imports or acquisition side effects. */
class FingerprintScript extends Script {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("Tool fingerprint accepts no arguments");
    const controller = new AbortController(), stop = (): void => controller.abort(new Error("Tool fingerprint cancelled"));
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    try {
      const versions: Record<string, string> = {};
      for (const tool of ["wasm-pack", "wasm-bindgen", "wasm-opt", "trunk"]) {
        controller.signal.throwIfAborted();
        const override = tool === "wasm-bindgen" ? process.env.SEMIO_WASM_BINDGEN_BIN : tool === "wasm-opt" ? process.env.SEMIO_WASM_OPT_BIN : undefined;
        if (tool === "wasm-opt" && !override) { versions[tool] = binaryenIdentity(); continue; }
        const path = override ? resolve(this.root, override) : Bun.which(tool, { PATH: process.env.PATH });
        if (!path) { versions[tool] = "unavailable"; continue; }
        const version = (await runTool(path, ["--version"], this.root, AbortSignal.any([controller.signal, AbortSignal.timeout(10000)]), true)).trim();
        if (!version) throw new Error(`Cannot fingerprint ${tool}`);
        versions[tool] = version;
      }
      console.log(JSON.stringify(versions));
    } finally { process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop); }
  }
}

if (import.meta.main) await new ScriptRouter(getWorkspaceRoot()).register("prepare", PrepareScript).register("fingerprint", FingerprintScript).run(process.argv.slice(2));
