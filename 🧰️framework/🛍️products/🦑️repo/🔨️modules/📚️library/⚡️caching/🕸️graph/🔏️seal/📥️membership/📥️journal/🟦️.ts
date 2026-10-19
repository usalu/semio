import { lstat, readFile, readdir } from "node:fs/promises";
import { isAbsolute, join, resolve } from "node:path";
import { createLiveFilesystemJournalV1, type FilesystemJournalChangeV1, type FilesystemJournalControlV1, type FilesystemJournalScopeV1, type LiveFilesystemJournalV1 } from "@semio-tech/framework/filesystem/observation/journal";
import { collectionManifestCandidate,collectionManifestsFromInventory,gitCollectionManifests } from "../../../../🚀️bootstrap/📥️admission/🟦️.ts";
import cachePolicy from "../../../../🔣️policy.json" with {type: "json"};
import ownership from "../../../../../🗂️workspaces/🔣️policy.json" with {type: "json"};

/** 🔌️ The existing Nx watcher supplies events and drains through owned ports. */
export interface RepositoryFilesystemJournalObservationV1 {
 watch(receive: (error: unknown, events: readonly FilesystemJournalChangeV1[]) => void): (() => void | Promise<void>) | Promise<() => void | Promise<void>>;
 flush(): readonly FilesystemJournalChangeV1[] | Promise<readonly FilesystemJournalChangeV1[]>;
 census(control: FilesystemJournalControlV1): Promise<readonly string[]>;
 admittedPaths(paths: readonly string[], control: FilesystemJournalControlV1): Promise<readonly string[]>;
 isGlobalIdentity(path: string): boolean;
}
function check(control: FilesystemJournalControlV1): void {
 control.signal.throwIfAborted();
 const budget = control.remainingMilliseconds();
 if (budget !== null && (!Number.isFinite(budget) || budget <= 0)) throw Error("Repository journal original deadline exhausted");
}
async function checkpoint(control: FilesystemJournalControlV1, phase: string): Promise<void> {
 check(control); await control.publish({ resource: "filesystem-journal:" + phase, mode: "shared", elapsedMs: 0 }); check(control);
 await control.yieldContinuation(); check(control);
}

/** 🗺️ Static output exclusions match source census while leaving ignored structural inputs visible. */
export async function repositoryFilesystemJournalPolicyV1(root: string): Promise<Readonly<{ roots: readonly string[]; names: readonly string[] }>> {
 const roots = [".🧬semio"], names = new Set<string>(cachePolicy.generatedDirectories);
 let ignored = ""; try { ignored = await readFile(join(root, ".nxignore"), "utf8"); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
 for (const line of ignored.split(/\r?\n/u)) {
  const anchored = line.trim().match(/^\/([^*?[\]!#]+)\/\*\*$/u), name = line.trim().match(/^\*\*\/([^/*?[\]!#]+)\/\*\*$/u);
  if (anchored) roots.push(anchored[1]!); else if (name) names.add(name[1]!);
 }
 return Object.freeze({ roots: Object.freeze([...new Set(roots)]), names: Object.freeze([...names]) });
}

/** 👁️ Joins the daemon's live watcher to the same admitted source membership and no-follow filesystem rules. */
export async function createRepositoryFilesystemJournalV1(root: string, observation: RepositoryFilesystemJournalObservationV1, control: FilesystemJournalControlV1): Promise<LiveFilesystemJournalV1> {
 const workspace = resolve(root), policy = await repositoryFilesystemJournalPolicyV1(workspace);
 const excluded = (path: string): boolean => policy.roots.some(prefix => path === prefix || path.startsWith(prefix + "/")) || path.split("/").some(part => policy.names.includes(part));
 const admit = async (path: string, current: FilesystemJournalControlV1, expected?: "file" | "directory"): Promise<Awaited<ReturnType<typeof lstat>> | undefined> => {
  check(current);
  if (isAbsolute(path) || !path || path.split("/").some(part => !part || part === "." || part === "..") || /[:\\\0]/u.test(path)) throw Error("Repository journal relative ancestry required");
  let absolute = workspace, status = await lstat(workspace); check(current);
  if (!status.isDirectory() || status.isSymbolicLink()) throw Error("Repository journal root requires no-follow directory");
  const parts = path.split("/");
  for (let index = 0; index < parts.length; index++) {
   absolute = join(absolute, parts[index]!);
   if (absolute.length > 256 || [...absolute].length > 256) throw Error("Repository journal materialized path exceeds 256 characters");
   try { status = await lstat(absolute); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined; throw error; }
   check(current);
   if (status.isSymbolicLink() || (index + 1 < parts.length && !status.isDirectory())) throw Error("Repository journal source requires no-follow ancestry");
  }
  if (expected && (expected === "file" ? !status.isFile() : !status.isDirectory())) throw Error("Repository journal source has the wrong physical kind");
  return status;
 };
 const reconcile = async (event: FilesystemJournalChangeV1, current: FilesystemJournalControlV1, priorAffected: readonly string[]): Promise<readonly string[]> => {
  if (event.path === ".nxignore" || event.path.split("/").at(-1) === ".gitignore") throw Error("Filesystem journal census policy changed");
  let visited = 0;
  const paths: string[] = [], walk = async (path: string): Promise<void> => {
   if (excluded(path)) return;
   if (++visited > 200_000) throw Error("Repository journal reconciliation capacity exceeded");
   const status = await admit(path, current); if (!status) return;
   if (status.isFile()) paths.push(path);
   else if (status.isDirectory()) {
    for (const name of await readdir(join(workspace, path))) await walk(path + "/" + name);
   } else throw Error("Repository journal refuses nonregular source");
   if (paths.length > 2_000_000) throw Error("Repository journal subtree capacity exceeded");
  };
  if (event.type === "delete" && priorAffected.some(path => path.startsWith(event.path + "/"))) {
   const parent = event.path.split("/").slice(0,-1).join("/");
   if (parent) { const status = await admit(parent,current,"directory"); if (status) await walk(parent); }
   else for (const name of await readdir(workspace)) await walk(name);
  } else await walk(event.path);
  await checkpoint(current, "reconcile");
  if (!paths.length) return paths;
  const physical = new Set(paths), effective = await observation.admittedPaths(paths, current); check(current);
  if (effective.some(path => !physical.has(path))) throw Error("Native journal admission returned foreign paths");
  const admitted = await collectionManifestsFromInventory(workspace, [paths.join("\0") + "\0"], cachePolicy, ownership.collections, current.signal);
  check(current);
  return [...new Set([...effective, ...admitted, ...paths.filter(path=>collectionManifestCandidate(path,cachePolicy,ownership.collections))])].filter(path => !excluded(path));
 };
 return createLiveFilesystemJournalV1({
  source: {
   watch: receive => observation.watch(receive),
   flush: () => observation.flush(),
   async census(current) {
    await checkpoint(current, "census");
    const effective = await observation.census(current); check(current);
    const admitted = await gitCollectionManifests(workspace,cachePolicy,ownership.collections,current.signal)??await collectionManifestsFromInventory(workspace, [effective.join("\0") + "\0"], cachePolicy, ownership.collections, current.signal);
    const files = [...new Set([...effective, ...admitted])].filter(path => !excluded(path));
    const global=files.filter(observation.isGlobalIdentity);let next=0;
    await Promise.all(Array.from({length:Math.min(64,global.length)},async()=>{while(next<global.length)await admit(global[next++]!,current,"file");}));
    return files;
   },
   reconcile,
   async admitScope(scope: FilesystemJournalScopeV1, current) {
    for (const path of scope.roots) await admit(path, current, "directory");
    for (const path of scope.files) await admit(path, current, "file");
    await checkpoint(current, "scope");
   }
  },
  isGlobalIdentity: path => observation.isGlobalIdentity(path), excluded, maximumPaths: 2_000_000, maximumEvents: 200_000
 }, control);
}
