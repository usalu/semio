import {withNxGraphAuxiliaryCapacityV1} from "../../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts";
import { createHash } from "node:crypto";
import { lstat, mkdir, readFile, rename, rm, stat, writeFile } from "node:fs/promises";
import { join, sep } from "node:path";
import { nxGraphSourceMembershipV2 } from "./📥️membership/🟦️.ts";
export { nxGraphSourceMembershipV2 };
import * as observation from "../../../🏗️build/👁️observation/🟨️.mjs";
const {inferenceHeaderLine,observationsHold}=observation as {inferenceHeaderLine(path:string):string|null;observationsHold(observations:readonly unknown[],limit:number,stableDirectories:Set<string>,symlinks:Set<string>,signal?:AbortSignal):Promise<boolean>};

/** 🔏️ The persisted seal format; see `./🧬️schema/🔣️.json`. */
export const NX_GRAPH_SEAL_FORMAT = 2;

const SEAL_FILE = "semio-graph-seal.json";
const PREVIOUS_SEAL_FILE = "semio-graph-seal.previous.json";
const GRAPH_FILES = ["project-graph.json", "file-map.json"] as const;
const RECEIPTS = ["emoji-project-json.receipt.json", "test-cases.receipt.json"] as const;
const WALK_CONCURRENCY = 64;
const INPUT_FILE = /^(?:package\.json|project\.json|📋️project\.json|pyproject\.toml|Cargo\.toml|uv\.lock|bun\.lock|package-lock\.json|yarn\.lock|pnpm-lock\.yaml|nx\.json|\.nxignore|\.gitignore|tsconfig(?:\.[^/]*)?\.json|[^/]*\.patch|[^/]*\.feature)$/u;

/** 🏷️ The identity of one node: device, inode, size and both times; any content or entry change alters it. */
type Stamp = readonly [dev: number, ino: number, size: number, mtimeMs: number, ctimeMs: number];

/** 🔏️ Proof that a cached Nx project graph was computed from inputs that can be re-verified without recomputing it. */
export interface NxGraphSeal {
  readonly format: typeof NX_GRAPH_SEAL_FORMAT;
  readonly nx: string;
  readonly membership: string;
  readonly graph: Readonly<Record<string, Stamp>>;
  readonly receipts: Readonly<Record<string, string>>;
  readonly directories: readonly DirectoryRow[];
  readonly inputs: readonly (readonly [path: string, stamp: Stamp])[];
}

/** 📂️ One indexable directory: its stamp, the indexable child directories, graph-input files and symbolic links it holds (by name). */
type DirectoryRow = readonly [path: string, stamp: Stamp, children: readonly string[], inputs: readonly string[], symlinks: readonly string[]];

/** 🏷️ One node's stamp. */
function stampOf(value: { dev: number; ino: number; size: number; mtimeMs: number; ctimeMs: number }): Stamp {
  return [value.dev, value.ino, value.size, value.mtimeMs, value.ctimeMs];
}

/** ⚖️ Stamp equality. */
function sameStamp(left: Stamp, right: Stamp): boolean {
  return left.every((value, index) => value === right[index]);
}

/** 🧾️ SHA-256 of bytes or text. */
function sha256(value: string | Uint8Array): string {
  return createHash("sha256").update(value).digest("hex");
}

/** 🧵️ Runs `visit` over one finite source ancestry inventory, at most `limit` at a time. */
async function boundedRows<T>(rows:readonly T[],limit:number,visit:(row:T)=>Promise<void>,signal?:AbortSignal):Promise<void>{
 let next=0;await Promise.all(Array.from({length:Math.min(limit,rows.length)},async()=>{while(next<rows.length){signal?.throwIfAborted();await visit(rows[next++]!);}}));
}

/** 🗺️ Captures custody only for actual source ancestry; membership owns additions, removals and renames. */
async function graphInputTree(root:string,signal?:AbortSignal):Promise<{directories:DirectoryRow[];inputs:[string,Stamp][];membership:string}>{
 const source=await nxGraphSourceMembershipV2(root,signal),paths=new Set([""]),directories:DirectoryRow[]=[],inputs:[string,Stamp][]=[];
 for(const file of source.files.filter(file=>INPUT_FILE.test(file.split("/").at(-1)!))){const parts=file.split("/");for(let i=1;i<parts.length;i++)paths.add(parts.slice(0,i).join("/"));}
 await boundedRows([...paths].sort(),WALK_CONCURRENCY,async path=>{const value=await lstat(nativePath(root,path));if(!value.isDirectory()||value.isSymbolicLink())throw Error("Graph source ancestry requires original real directory custody");directories.push([path,stampOf(value),[],[],[]]);},signal);
 await boundedRows(source.files.filter(file=>INPUT_FILE.test(file.split("/").at(-1)!)),WALK_CONCURRENCY,async path=>{inputs.push([path,stampOf(await lstat(nativePath(root,path)))]);},signal);
 return {directories:directories.sort(([a],[b])=>a<b?-1:a>b?1:0),inputs:inputs.sort(([a],[b])=>a<b?-1:a>b?1:0),membership:source.digest};
}

/** 🧭️ The Nx tooling identity a graph depends on: the installed Nx and plugin package manifests and `nx.json`. */
async function nxIdentity(root: string): Promise<string> {
  const parts = await Promise.all(["node_modules/nx/package.json", "node_modules/@nx/js/package.json", "node_modules/@nxlv/python/package.json", "nx.json"].map(async (path) => { try { return await readFile(join(root, path)); } catch { return Buffer.from(`!${path}`); } }));
  return sha256(Buffer.concat(parts.flatMap((part) => [Buffer.from(`${part.byteLength}:`), part])));
}

/** 🗂️ Stamps of the graph cache files, or `null` when one is missing. */
async function graphStamps(workspaceData: string): Promise<Record<string, Stamp> | null> {
  const stamps: Record<string, Stamp> = {};
  for (const name of GRAPH_FILES) { try { stamps[name] = stampOf(await stat(join(workspaceData, name))); } catch { return null; } }
  return stamps;
}

/**
 * ✍️ Seals the graph a finished Nx process (`pid`, started at `startedAt`) left in `workspaceData`, when both plugin receipts are that
 * process's and the graph was written after them; otherwise removes any seal. Returns whether a seal was written.
 */
export async function writeNxGraphSeal(root: string, workspaceData: string, pid: number, startedAt: number, expectedMembership:string, environment:Readonly<Record<string,string|undefined>>, signal?:AbortSignal): Promise<boolean> {
  const sealPath = join(workspaceData, SEAL_FILE);
  await rm(sealPath, { force: true });
  const graph = await graphStamps(workspaceData), receipts: Record<string, string> = {};
  if (!graph || graph["project-graph.json"]![3] < startedAt) return false;
  for (const name of RECEIPTS) {
    let text: string;
    try { text = await readFile(join(workspaceData, "inference", name), "utf8"); } catch { return false; }
    const receipt = JSON.parse(text) as { pid: number; completedAt: number };
    if (receipt.pid !== pid || receipt.completedAt < startedAt || receipt.completedAt > graph["project-graph.json"]![3]) return false;
    receipts[name] = sha256(text);
  }
  if(!/^[a-f0-9]{64}$/u.test(expectedMembership))throw Error("Original graph source membership required");
  const tree=await graphInputTree(root,signal);
  if(tree.membership!==expectedMembership)return false;
  const seal:NxGraphSeal={format:NX_GRAPH_SEAL_FORMAT,nx:await nxIdentity(root),membership:tree.membership,graph,receipts,directories:tree.directories,inputs:tree.inputs};
  if (JSON.stringify(await graphStamps(workspaceData)) !== JSON.stringify(graph)||!await verifyNxGraphSealV2(root,workspaceData,seal,signal,expectedMembership))return false;
  await mkdir(workspaceData, { recursive: true });
  const temporary = `${sealPath}.${process.pid}.tmp`, text = JSON.stringify(seal);
  await withNxGraphAuxiliaryCapacityV1(root,environment,{version:1,extraBytes:2*Math.ceil(Buffer.byteLength(text)/4096)*4096},async()=>{
    signal?.throwIfAborted();try{await writeFile(temporary,text,{flag:"wx"});signal?.throwIfAborted();await rename(temporary,sealPath);await writeFile(`${temporary}.previous`,text,{flag:"wx"});signal?.throwIfAborted();await rename(`${temporary}.previous`,join(workspaceData,PREVIOUS_SEAL_FILE));signal?.throwIfAborted();}finally{await rm(temporary,{force:true});await rm(`${temporary}.previous`,{force:true});}
  });
  return true;
}

/** 🧭️ The absolute native path of a repository-relative POSIX path, built without re-normalizing (seal paths are already normal). */
function nativePath(root: string, path: string): string {
  return path ? `${root}${sep}${sep === "/" ? path : path.replaceAll("/", sep)}` : root;
}

/** 🏷️ True when every recorded path still carries its stamp, checked `WALK_CONCURRENCY` at a time. */
async function stampsHold(root: string, rows: readonly (readonly [string, Stamp])[], read: (path: string) => Promise<{ dev: number; ino: number; size: number; mtimeMs: number; ctimeMs: number }>, ancestry=false,signal?:AbortSignal): Promise<boolean> {
  let next = 0, holds = true;
  const worker = async (): Promise<void> => {
    while (holds && next < rows.length) {
      signal?.throwIfAborted();
      const [path, stamp] = rows[next++]!;
      try { const value=await read(nativePath(root,path));if(ancestry?value.dev!==stamp[0]||value.ino!==stamp[1]:!sameStamp(stampOf(value),stamp))holds=false; } catch { holds = false; }
    }
  };
  await Promise.all(Array.from({ length: Math.min(WALK_CONCURRENCY, rows.length) }, worker));
  return holds;
}

/**
 * ✅️ True when the sealed graph in `workspaceData` still equals what Nx would compute: the Nx tooling and graph files are unchanged, no
 * indexable directory gained, lost or renamed an entry, no graph-input file changed, and every filesystem fact both plugins' node
 * inference and dependency computations observed still holds. Nx may then reuse its cached graph (`NX_FORCE_REUSE_CACHED_GRAPH`).
 */
export async function nxGraphSealHolds(root: string, workspaceData: string, signal?:AbortSignal,membership?:string): Promise<boolean> {
  let seal: NxGraphSeal;
  try { seal = JSON.parse(await readFile(join(workspaceData, SEAL_FILE), "utf8")) as NxGraphSeal; } catch { return false; }
  return verifyNxGraphSealV2(root,workspaceData,seal,signal,membership);
}

/** 🔐️ Proves complete publisher custody before any seal is visible to another caller. */
async function verifyNxGraphSealV2(root:string,workspaceData:string,seal:NxGraphSeal,signal?:AbortSignal,membership?:string):Promise<boolean>{
  if (seal.format !== NX_GRAPH_SEAL_FORMAT || seal.nx !== await nxIdentity(root)) return false;
  const graph = await graphStamps(workspaceData);
  if (!graph || GRAPH_FILES.some((name) => !seal.graph[name] || !sameStamp(graph[name]!, seal.graph[name]!))) return false;
  if((membership??(await nxGraphSourceMembershipV2(root,signal)).digest)!==seal.membership)return false;
  const facts: unknown[] = [];
  for (const name of RECEIPTS) {
    let text: string;
    try { text = await readFile(join(workspaceData, "inference", name), "utf8"); } catch { return false; }
    if (sha256(text) !== seal.receipts[name]) return false;
    const receipt = JSON.parse(text) as { inference: [string, string][]; observations: unknown[] };
    for (const [cachePath, digest] of receipt.inference) {
      let header: string | null;
      try { header = inferenceHeaderLine(cachePath); } catch { return false; }
      if (header === null || sha256(header) !== digest) return false;
      facts.push(...(JSON.parse(header) as { observations: unknown[] }).observations);
    }
    facts.push(...receipt.observations);
  }
  return await stampsHold(root, seal.directories.map(([path, stamp]) => [path, stamp] as const), lstat,true,signal) && await stampsHold(root, seal.inputs, lstat,false,signal) && await observationsHold(facts, 64, new Set(seal.directories.map(([path]) => nativePath(root, path))), new Set(seal.directories.flatMap(([path, , , , links]) => links.map((name) => nativePath(root, path ? `${path}/${name}` : name)))));
}
