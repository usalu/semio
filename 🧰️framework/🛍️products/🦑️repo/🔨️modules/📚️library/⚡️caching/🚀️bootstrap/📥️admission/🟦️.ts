import admissionSchema from "./🧬️schema/🔣️.json" with { type: "json" };
import { readdir } from "node:fs/promises";

import { isAbsolute, join, resolve } from "node:path";
import { startToolV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** 🧵️ Bounds concurrent original reads and drains every admitted reader before release. */
export async function boundedAdmissionWalkV1<T>(initial:readonly T[],limit:number,visit:(item:T,enqueue:(next:T)=>void)=>Promise<void>):Promise<void>{
 const bounds=admissionSchema.properties.maximumConcurrentReads;if(!Number.isSafeInteger(limit)||limit<bounds.minimum||limit>bounds.maximum)throw Error("Bounded admission concurrency required");
 const pending=[...initial],active=new Set<Promise<void>>();let failure:{error:unknown}|undefined;
 while(pending.length||active.size){
  while(!failure&&active.size<limit&&pending.length){const item=pending.pop()!,operation=Promise.resolve().then(()=>visit(item,next=>{if(!failure)pending.push(next);})).catch(error=>{failure??={error};}).finally(()=>{active.delete(operation);});active.add(operation);}
  if(!active.size)break;await Promise.race(active);
 }
 if(failure)throw failure.error;
}

type AdmissionPolicy = { generatedDirectories: readonly string[]; manifestBasenames: readonly string[] };
type CollectionPolicy = { names: readonly string[]; moduleMember: string };

/** 🧭️ Resolves complete structural collection sequences, including nested taxonomy roots. */
export function moduleCollectionAncestry(parts: readonly string[], collections: CollectionPolicy): { inside: boolean; invalid: boolean } {
  const positions = collections.names.flatMap(name => {
    const sequence = name.split("/");
    return parts.flatMap((_, index) => sequence.every((part, offset) => parts[index + offset] === part) ? [index] : []);
  });
  return { inside: positions.some(index => parts[index - 1] === collections.moduleMember), invalid: positions.some(index => parts[index - 1] !== collections.moduleMember) };
}

/** 🧫️ Admits exact module collection ancestry independently of version-control tracking. */
export function collectionManifestCandidate(path: string, policy: AdmissionPolicy, collections: CollectionPolicy): boolean {
  const parts = path.split("/");
  if (isAbsolute(path) || parts.some(part => !part || part === "." || part === ".." || part === ".🧬semio" || ["🤖️generated", "pkg", ...policy.generatedDirectories].includes(part))) return false;
  const ancestry = moduleCollectionAncestry(parts, collections);
  if (ancestry.invalid || !ancestry.inside) return false;
  return policy.manifestBasenames.some(name => name === "*.patch" ? parts.at(-1)!.endsWith(".patch") : parts.at(-1) === name);
}

/** 🦑️ Reads native index and untracked inventories without constructing a checkout-wide JavaScript directory census. */
export async function gitCollectionManifests(root: string, policy: AdmissionPolicy, collections: CollectionPolicy, signal?: AbortSignal): Promise<string[] | undefined> {
  const read = (args: string[]): Promise<string | undefined> => new Promise((accept, reject) => {
    signal?.throwIfAborted();
    const child = startToolV1("git", ["-c", "core.quotepath=false", "-c", "core.fsmonitor=false", ...args], { cwd: root, stdio: ["ignore", "pipe", "ignore"], windowsHide: true });
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
  if (indexed === undefined) return undefined;
  const ordinary = await untrackedStructuralModules(root, indexed, policy, collections, signal);

  if (ordinary === undefined) return undefined;
  return collectionManifestsFromInventory(root, [indexed, ordinary], policy, collections, signal);
}

/** 📂️ Observes membership only inside structural collection roots supplied by the native source inventory. */
export async function collectionManifestsFromInventory(root: string, inventory: readonly string[], policy: AdmissionPolicy, collections: CollectionPolicy, signal?: AbortSignal): Promise<string[]> {
  const modules = new Set<string>();
  for (const path of inventory.flatMap(value => value.split("\0"))) {
    const parts = path.split("/");
    for (let index = 0; index < parts.length; index++) if (parts[index] === collections.moduleMember) modules.add(parts.slice(0, index + 1).join("/"));
  }
  const found = new Set<string>(), excluded = new Set([".🧬semio", "🤖️generated", "pkg", ...policy.generatedDirectories]);
  const roots=[...modules].flatMap(module=>collections.names.map(collection=>`${module}/${collection}`));
  await boundedAdmissionWalkV1(roots,64,async(path,enqueue)=>{
    signal?.throwIfAborted();
    let entries;
    try { entries = await readdir(join(root, path), { withFileTypes: true }); }
    catch (error) { if (["ENOENT", "ENOTDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return; throw error; }
    for (const entry of entries) {
      if (entry.isSymbolicLink() || excluded.has(entry.name)) continue;
      const child = `${path}/${entry.name}`;
      if (entry.isDirectory()) enqueue(child);
      else if (entry.isFile() && collectionManifestCandidate(child, policy, collections)) found.add(child);
    }
  });
  return [...found].sort();
}

/** 🌳️ Finds newly introduced structural roots through indexed module ancestry and freshly read untracked trees. */
async function untrackedStructuralModules(root: string, indexed: string, policy: AdmissionPolicy, collections: CollectionPolicy, signal?: AbortSignal): Promise<string> {
  const indexedDirectories = new Set<string>(), observed = new Set<string>([""]), modules = new Set<string>();
  for (const path of indexed.split("\0")) {
    const parts = path.split("/");
    if (parts.some(part => [".🧬semio", ".codex", ".claude", "compose", "temp", "🤖️generated", "pkg", ...policy.generatedDirectories].includes(part))) continue;
    if (moduleCollectionAncestry(parts, collections).invalid) continue;
    for (let index = 1; index < parts.length; index++) indexedDirectories.add(parts.slice(0, index).join("/"));
    for (let index = 0; index < parts.length; index++) if (parts[index] === collections.moduleMember) {
      for (let ancestor = 1; ancestor <= index + 2 && ancestor < parts.length; ancestor++) observed.add(parts.slice(0, ancestor).join("/"));
    }
  }
  const excluded = new Set([".🧬semio", ".codex", ".claude", "compose", "temp", "🤖️generated", "pkg", ...policy.generatedDirectories]);
  let inspected = 0;
  await boundedAdmissionWalkV1([...observed].map(directory=>({directory,fresh:false})),64,async({directory,fresh},enqueue)=>{
    if (++inspected % 1024 === 0) console.log(`[nx] Admitting structural source membership (${inspected} directories)`);
    signal?.throwIfAborted();
    let entries;
    try { entries = await readdir(join(root, directory), { withFileTypes: true }); }
    catch (error) { if (["ENOENT", "ENOTDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return; throw error; }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.isSymbolicLink() || excluded.has(entry.name)) continue;
      const child = directory ? `${directory}/${entry.name}` : entry.name;
      if (entry.name === collections.moduleMember) modules.add(`${child}/`);
      if (fresh || !indexedDirectories.has(child)) enqueue({directory:child,fresh:true});
    }
  });
  return [...modules].join("\0");
}
