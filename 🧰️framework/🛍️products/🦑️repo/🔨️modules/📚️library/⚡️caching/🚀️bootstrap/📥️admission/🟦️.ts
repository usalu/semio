import { readdir } from "node:fs/promises";
import { spawn } from "node:child_process";
import { isAbsolute, join, resolve } from "node:path";

type AdmissionPolicy = { generatedDirectories: readonly string[]; manifestBasenames: readonly string[] };
type CollectionPolicy = { names: readonly string[]; moduleMember: string };

/** 🧫️ Admits exact module collection ancestry independently of version-control tracking. */
export function collectionManifestCandidate(path: string, policy: AdmissionPolicy, collections: CollectionPolicy): boolean {
  const parts = path.split("/");
  if (isAbsolute(path) || parts.some(part => !part || part === "." || part === ".." || part === ".🧬semio" || ["🤖️generated", "pkg", ...policy.generatedDirectories].includes(part))) return false;
  if (parts.some((part, index) => collections.names.includes(part) && parts[index - 1] !== collections.moduleMember)) return false;
  if (!parts.some((part, index) => collections.names.includes(part) && parts[index - 1] === collections.moduleMember)) return false;
  return policy.manifestBasenames.some(name => name === "*.patch" ? parts.at(-1)!.endsWith(".patch") : parts.at(-1) === name);
}

/** 🦑️ Reads native index and untracked inventories without constructing a checkout-wide JavaScript directory census. */
export async function gitCollectionManifests(root: string, policy: AdmissionPolicy, collections: CollectionPolicy, signal?: AbortSignal): Promise<string[] | undefined> {
  const read = (args: string[]): Promise<string | undefined> => new Promise((accept, reject) => {
    signal?.throwIfAborted();
    const child = spawn("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: root, stdio: ["ignore", "pipe", "ignore"], windowsHide: true });
    const chunks: Buffer[] = []; let bytes = 0;
    const abort = (): void => { child.kill(); reject(signal?.reason ?? Error("Nx admission cancelled")); };
    signal?.addEventListener("abort", abort, { once: true });
    child.stdout.on("data", chunk => { bytes += chunk.length; if (bytes > 64 * 1024 * 1024) { child.kill(); reject(Error("Nx collection inventory exceeds its bounded source budget")); } else chunks.push(chunk); });
    child.once("error", () => { signal?.removeEventListener("abort", abort); accept(undefined); });
    child.once("close", code => { signal?.removeEventListener("abort", abort); if (signal?.aborted) reject(signal.reason); else accept(code === 0 ? Buffer.concat(chunks).toString("utf8") : undefined); });
  });
  const top = await read(["rev-parse", "--show-toplevel"]);
  if (top === undefined || resolve(top.trim()) !== resolve(root)) return undefined;
  const indexed = await read(["ls-files", "-z", "--cached"]);
  const ordinary = await read(["ls-files", "-z", "--others", "--exclude-standard", "--", `:(glob)**/${collections.moduleMember}/**`]);
  if (indexed === undefined || ordinary === undefined) return undefined;
  const modules = new Set<string>();
  for (const path of [...indexed.split("\0"), ...ordinary.split("\0")]) {
    const parts = path.split("/");
    for (let index = 0; index < parts.length; index++) if (parts[index] === collections.moduleMember) modules.add(parts.slice(0, index + 1).join("/"));
  }
  const found = new Set<string>(), excluded = new Set([".🧬semio", "🤖️generated", "pkg", ...policy.generatedDirectories]);
  const walk = async (path: string): Promise<void> => {
    signal?.throwIfAborted();
    let entries;
    try { entries = await readdir(join(root, path), { withFileTypes: true }); }
    catch (error) { if (["ENOENT", "ENOTDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return; throw error; }
    for (const entry of entries) {
      if (entry.isSymbolicLink() || excluded.has(entry.name)) continue;
      const child = `${path}/${entry.name}`;
      if (entry.isDirectory()) await walk(child);
      else if (entry.isFile() && collectionManifestCandidate(child, policy, collections)) found.add(child);
    }
  };
  for (const module of modules) for (const collection of collections.names) await walk(`${module}/${collection}`);
  return [...found].sort();
}
