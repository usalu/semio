import * as fs from "node:fs";
import { readFile as readFileAsync, stat as statAsync } from "node:fs/promises";
import { AsyncLocalStorage } from "node:async_hooks";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { openRecordStore, recordKey } from "../../⚡️caching/🗃️records/🟦️.ts";

/** 🗃️ This module's own bytes: persisted observations validated by another revision of these rules are a miss. */
const OBSERVATION_REVISION = createHash("sha256").update(fs.readFileSync(fileURLToPath(import.meta.url))).digest("hex");

/** ⏱️ Files stamped this close to the start of a computation may still change within one timestamp tick, so their result is never persisted (racy-git rule). */
const RACY_MS = 2000;

const scopes = new AsyncLocalStorage();
const recordScopes = new AsyncLocalStorage();
const recordStores = new Map();
const usedInference = new Map();
const originalInferencePublications = new Map();
const loadObservations = new Map();
const projectComputations = new Map();
const projectImages = new Map();
const projectChecks = new WeakMap();
let loadSealed = false;

/** 📊️ Exposes owned project computation counts without influencing source admission. */
export function projectInferenceCountsV1(){return Object.fromEntries(projectComputations);}

/** 🧷️ Shares a physical validation only within the same original parent inference whose final receipt still rechecks every fact. */
function projectObservationHoldsV1(row,parent){
 let checks=parent&&projectChecks.get(parent);if(parent&&!checks){checks=new Map();projectChecks.set(parent,checks);}
 const key=row.kind+"\0"+row.path,fact=JSON.stringify(row.fact),previous=checks?.get(key);if(previous?.fact===fact)return previous.holds;
 const holds=row.kind==="file"?fileFactHolds(row.path,row.fact)&&JSON.stringify(facts.file(row.path)?.sha256??null)===JSON.stringify(row.fact.sha256??null):current([row]);
 checks?.set(key,{fact,holds});return holds;
}

/** 📦️ Admits one physical project's exact input observations through its own computation. */
export async function observeProjectInferenceV1({cachePath,key,owner},compute){
 const parent=scopes.getStore(),identity=createHash("sha256").update(JSON.stringify([OBSERVATION_REVISION,key])).digest("hex"),previous=projectImages.get(cachePath);
 const inherit=rows=>{if(parent)for(const row of rows){const key=row.kind+"\0"+row.path;if(!parent.has(key))parent.set(key,row);}};
 if(previous?.identity===identity&&previous.observations.every(row=>projectObservationHoldsV1(row,parent))){
  inherit(previous.observations);
  originalInferencePublications.set(cachePath,previous.publication);
  return JSON.parse(previous.image);
 }
 const value=await observeInference({cachePath,key},()=>{projectComputations.set(owner,(projectComputations.get(owner)??0)+1);const image=jsonImage(compute());if(image===undefined)throw Error("Project inference requires a language-neutral JSON image");return JSON.parse(image);});
 const publication=originalInferencePublications.get(cachePath);
 if(!publication)throw Error("Original project inference observations required");
 inherit(publication.observations);
 projectImages.set(cachePath,{identity,image:JSON.stringify(value),observations:publication.observations,publication});
 return value;
}

/** 🧭️ Normalizes every accepted fs path argument to one absolute key. */
function absolute(path) {
  return resolve(path instanceof URL ? fileURLToPath(path) : typeof path === "string" ? path : String(path));
}

/** 🏷️ The node type a stat result denotes, the only stat fact graph inference reads. */
function nodeType(stat) {
  return stat.isSymbolicLink() ? "symlink" : stat.isFile() ? "file" : stat.isDirectory() ? "directory" : "other";
}

/** 🧾️ The identity of one file's bytes: stamp plus content digest, or the error code that replaced them. */
function fileFact(path) {
  try {
    const stat = fs.statSync(path);
    return { size: stat.size, mtimeMs: stat.mtimeMs, ctimeMs: stat.ctimeMs, ino: stat.ino, sha256: createHash("sha256").update(fs.readFileSync(path)).digest("hex") };
  } catch (error) { return { error: error.code ?? "EUNKNOWN" }; }
}

/** 🔎️ Recomputes one observation's fact exactly as it was first taken. */
const facts = {
  file: (path) => fileFact(path),
  exists: (path) => fs.existsSync(path),
  stat: (path) => { try { return nodeType(fs.statSync(path)); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  lstat: (path) => { try { return nodeType(fs.lstatSync(path)); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  readdir: (path) => { try { return createHash("sha256").update(JSON.stringify(fs.readdirSync(path, { withFileTypes: true }).map((entry) => [entry.name, entry.isSymbolicLink() ? "symlink" : entry.isFile() ? "file" : entry.isDirectory() ? "directory" : "other"]).sort())).digest("hex"); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  realpath: (path) => { try { return fs.realpathSync(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  realpathNative: (path) => { try { return fs.realpathSync.native(path); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
  resolve: (key) => { const [from, request] = JSON.parse(key); try { return createRequire(from).resolve(request); } catch (error) { return `!${error.code ?? "EUNKNOWN"}`; } },
};

/** ✍️ Records one fact in the active inference scope, or — until the first inference begins — in the module-load set; reads outside both are not inference inputs. */
function record(kind, path, fact) {
  const target = scopes.getStore() ?? (loadSealed ? undefined : loadObservations), key = `${kind}\0${path}`;
  if (target && !target.has(key)) target.set(key, { kind, path, fact: fact === undefined ? facts[kind](path) : fact });
}

/** 🔬️ Runs one probe and records the fact its own outcome proves, so observing costs no second syscall. */
function probe(kind, path, run, fact) {
  let value;
  try { value = run(); } catch (error) { record(kind, absolute(path), `!${error.code ?? "EUNKNOWN"}`); throw error; }
  record(kind, absolute(path), fact(value));
  return value;
}

/** 🔤️ The encoding a `readFileSync`/`readFile` options argument selects, if any. */
function encodingOf(options) {
  return typeof options === "string" ? options : options?.encoding ?? undefined;
}

/** 📖️ Reads bytes once: the stamp is taken first and the digest from the very bytes returned, so a change racing the read can only make the fact stale, never wrong. */
function readObserved(path, stamp, bytes, options) {
  const fact = stamp instanceof Error ? { error: stamp.code ?? "EUNKNOWN" } : { size: stamp.size, mtimeMs: stamp.mtimeMs, ctimeMs: stamp.ctimeMs, ino: stamp.ino, sha256: createHash("sha256").update(bytes).digest("hex") };
  record("file", absolute(path), fact);
  const encoding = encodingOf(options);
  return encoding ? bytes.toString(encoding) : bytes;
}

/** 👁️ The `node:fs` reads graph inference may perform, each recording what it observed so a persisted result can later be proven current. Writes stay on `node:fs`. */
export const observedFs = Object.freeze({
  readFileSync(path, options) {
    let stamp;
    try { stamp = fs.statSync(path); } catch (error) { stamp = error; }
    let bytes;
    try { bytes = fs.readFileSync(path); } catch (error) { record("file", absolute(path), { error: error.code ?? "EUNKNOWN" }); throw error; }
    return readObserved(path, stamp, bytes, options);
  },
  existsSync(path) {
    const scope = scopes.getStore(), key = absolute(path);
    if (scope?.has(`exists\0${key}`)) return scope.get(`exists\0${key}`).fact;
    const value = fs.existsSync(path);
    record("exists", key, value);
    return value;
  },
  statSync(path, options) { return probe("stat", path, () => fs.statSync(path, options), (value) => value ? nodeType(value) : "!ENOENT"); },
  lstatSync(path, options) { return probe("lstat", path, () => fs.lstatSync(path, options), (value) => value ? nodeType(value) : "!ENOENT"); },
  readdirSync(path, options) { const key = absolute(path); record("readdir", key); return fs.readdirSync(path, options); },
  realpathSync: Object.assign((path, options) => probe("realpath", path, () => fs.realpathSync(path, options), (value) => String(value)), { native: (path, options) => probe("realpathNative", path, () => fs.realpathSync.native(path, options), (value) => String(value)) }),
  async readFile(path, options) {
    let stamp;
    try { stamp = await statAsync(path); } catch (error) { stamp = error; }
    let bytes;
    try { bytes = await readFileAsync(path); } catch (error) { record("file", absolute(path), { error: error.code ?? "EUNKNOWN" }); throw error; }
    return readObserved(path, stamp, bytes, options);
  },
});

/** 🧷️ `createRequire(from).resolve(request)` with its outcome recorded, so a persisted result is invalidated when module resolution would now answer differently. */
export function observedRequireResolve(from, request) {
  const key = JSON.stringify([absolute(from), request]);
  try {
    const resolved = createRequire(from).resolve(request);
    record("resolve", key, resolved);
    return resolved;
  } catch (error) { record("resolve", key, `!${error.code ?? "EUNKNOWN"}`); throw error; }
}

/** 🧩️ A TypeScript module resolution host that delegates to TypeScript's own `sys` and records every probe it answers. */
export function observedTypeScriptHost(sys) {
  return {
    ...sys,
    fileExists: (path) => { const value = sys.fileExists(path); record("stat", absolute(path)); return value; },
    directoryExists: (path) => { const value = sys.directoryExists(path); record("stat", absolute(path)); return value; },
    readFile: (path, encoding) => { record("file", absolute(path)); return sys.readFile(path, encoding); },
    realpath: sys.realpath && ((path) => { record("realpathNative", absolute(path)); return sys.realpath(path); }),
    getDirectories: (path) => { record("readdir", absolute(path)); return sys.getDirectories(path); },
  };
}

/** ⚖️ Structural equality that distinguishes absent from `undefined`, so only results that survive JSON exactly are persisted. */
function jsonExact(value) {
  if (value === null || typeof value === "string" || typeof value === "boolean") return true;
  if (typeof value === "number") return Number.isFinite(value);
  if (Array.isArray(value)) return value.every((item, index) => index in value && jsonExact(item));
  if (typeof value !== "object" || Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null) return false;
  return Object.values(value).every(jsonExact);
}

/** 🪞️ The JSON image of a value when it is built only from plain objects, arrays, strings, booleans, finite numbers and `null` (object
 * properties holding `undefined` are dropped, exactly as a consumer reading them sees `undefined`), or `undefined` when it is not. */
function jsonImage(value) {
  const representable = (item) => item === null || typeof item === "string" || typeof item === "boolean" || typeof item === "number" && Number.isFinite(item) || Array.isArray(item) && item.every((element, index) => index in item && element !== undefined && representable(element)) || typeof item === "object" && !Array.isArray(item) && (Object.getPrototypeOf(item) === Object.prototype || Object.getPrototypeOf(item) === null) && Object.values(item).every((entry) => entry === undefined || representable(entry));
  return representable(value) ? JSON.stringify(value) : undefined;
}

/**
 * 🧠️ Memoizes a pure per-content analysis across inference runs: inside {@link observeInference} the result for `namespace` (which must
 * name the analysis code revision) and `inputs` (every byte it reads) is kept in a content-addressed record store, and both a hit and a miss
 * return the value's JSON image so they are indistinguishable. Outside an inference, or for a value without a JSON image, it computes.
 * @template T
 * @param {string} namespace
 * @param {readonly (string | Uint8Array)[]} inputs
 * @param {() => T} compute
 * @returns {T}
 */
export function memoizedRecord(namespace, inputs, compute) {
  const store = recordScopes.getStore()?.records;
  if (!store) return compute();
  const key = recordKey(namespace, ...inputs), hit = store.get(key);
  if (hit !== undefined) { try { return JSON.parse(Buffer.from(hit).toString("utf8")); } catch {} }
  const value = compute(), image = jsonImage(value);
  if (image === undefined) return value;
  store.put(key, Buffer.from(image, "utf8"));
  return JSON.parse(image);
}

/**
 * 🗂️ {@link memoizedRecord} for an analysis of one file's bytes, keyed by their digest, that reads the file only when it must: a file whose
 * stamp equals the one the previous inference recorded (results are persisted only when no input was racy) is known by that digest, its
 * fact is recorded without a read, and a stored analysis is returned. Otherwise the bytes are read (and observed) and analysed.
 * @template T
 * @param {string} namespace
 * @param {string} path
 * @param {(bytes: Buffer) => T} compute
 * @returns {T}
 */
export function memoizedFileRecord(namespace, path, compute) {
  const scope = recordScopes.getStore();
  if (!scope) return compute(observedFs.readFileSync(path));
  const key = absolute(path), hint = scope.hints.get(key);
  if (hint) {
    let stamp;
    try { stamp = fs.statSync(path); } catch {}
    if (stamp && stamp.size === hint.size && stamp.mtimeMs === hint.mtimeMs && stamp.ctimeMs === hint.ctimeMs && stamp.ino === hint.ino) {
      const hit = scope.records.get(recordKey(namespace, hint.sha256));
      if (hit !== undefined) {
        try { const value = JSON.parse(Buffer.from(hit).toString("utf8")); record("file", key, hint); return value; } catch {}
      }
    }
  }
  const bytes = observedFs.readFileSync(path), value = compute(bytes), image = jsonImage(value);
  if (image === undefined) return value;
  scope.records.put(recordKey(namespace, createHash("sha256").update(bytes).digest("hex")), Buffer.from(image, "utf8"));
  return JSON.parse(image);
}

/** 🌊️ Reads a module asynchronously only when its original settled byte fingerprint has no verified semantic record. */
export async function memoizedFileRecordAsyncV1(namespace,path,compute){
 const scope=recordScopes.getStore(),key=absolute(path),hint=scope?.hints.get(key);
 if(hint){
  let stamp;try{stamp=await statAsync(path);}catch{}
  if(stamp&&stamp.size===hint.size&&stamp.mtimeMs===hint.mtimeMs&&stamp.ctimeMs===hint.ctimeMs&&stamp.ino===hint.ino){
   const hit=scope.records.get(recordKey(namespace,hint.sha256));if(hit!==undefined)try{const value=JSON.parse(Buffer.from(hit).toString("utf8"));record("file",key,hint);return value;}catch{}
  }
 }
 const bytes=await observedFs.readFile(path),value=compute(bytes),image=jsonImage(value);
 if(scope&&image!==undefined&&Buffer.byteLength(image)<=1024*1024){scope.records.put(recordKey(namespace,createHash("sha256").update(bytes).digest("hex")),Buffer.from(image,"utf8"));return JSON.parse(image);}
 return value;
}

/** 🪪️ True when a recorded file fact still names the same bytes: an unchanged stamp proves it cheaply, and a restamped file of the same size is proven by its digest, so rewrites of identical content stay hits. */
function fileFactHolds(path, fact) {
  if (fact.error !== undefined) { try { fs.readFileSync(path); return false; } catch (error) { return fact.error === (error.code ?? "EUNKNOWN"); } }
  let current;
  try { current = fs.statSync(path); } catch { return false; }
  if (current.size !== fact.size) return false;
  if (current.mtimeMs === fact.mtimeMs && current.ctimeMs === fact.ctimeMs && current.ino === fact.ino) return true;
  try { return createHash("sha256").update(fs.readFileSync(path)).digest("hex") === fact.sha256; } catch { return false; }
}

/** ✅️ True when every recorded fact still holds; stops at the first stale fact. Synchronous probes beat the four-thread pool here: each is one cached-inode syscall. */
function current(observations) {
  return observations.every(({ kind, path, fact }) => kind === "file" ? fileFactHolds(path, fact) : typeof facts[kind] === "function" && JSON.stringify(facts[kind](path)) === JSON.stringify(fact));
}

/**
 * 📸️ Returns `compute()`'s result, reusing the persisted result for `key` while every filesystem fact the computation observed — reads,
 * existence and type probes, directory listings, real paths and module resolutions, including those made while its modules loaded —
 * still holds. A result is persisted only when it is exactly JSON-representable and no file it read was stamped within {@link RACY_MS}
 * of the start, so a hit is indistinguishable from a recomputation.
 * @template T
 * @param {{ readonly cachePath: string; readonly key: unknown }} options
 * @param {() => Promise<T> | T} compute
 * @returns {Promise<T>}
 */
export async function observeInference({ cachePath, key }, compute) {
  loadSealed = true;
  const digest = createHash("sha256").update(JSON.stringify([OBSERVATION_REVISION, key])).digest("hex"), hints = new Map();
  try {
    const text = fs.readFileSync(cachePath, "utf8"), split = text.indexOf("\n"), header = JSON.parse(text.slice(0, split));
    if (header.digest === digest && header.resultSha256 === createHash("sha256").update(text.slice(split + 1)).digest("hex") && Array.isArray(header.observations) && current(header.observations)) {
      usedInference.set(cachePath, createHash("sha256").update(text.slice(0, split)).digest("hex"));
      originalInferencePublications.set(cachePath, {path:cachePath,digest,authorityObservations:header.authorityObservations,observations:header.observations});
      return JSON.parse(text.slice(split + 1));
    }
    for (const { kind, path, fact } of Array.isArray(header.observations) ? header.observations : []) if (kind === "file" && typeof fact?.sha256 === "string") hints.set(path, fact);
  } catch {}
  const observations = new Map(), started = Date.now();
  let records = recordStores.get(cachePath);
  if (!records) recordStores.set(cachePath, records = openRecordStore(`${cachePath}.records`));
  const result = await recordScopes.run({ records, hints }, () => scopes.run(observations, compute));
  records.flush();
  if (records.stats.records > 2 * records.stats.accessed) records.retainAccessed();
  const rows = [...loadObservations.values(), ...observations.values()];
  originalInferencePublications.delete(cachePath);
  if (jsonImage(result) !== undefined) originalInferencePublications.set(cachePath, {path:cachePath,digest,authorityObservations:[...loadObservations.values()],observations:rows});
  const racy = rows.some(({ kind, fact }) => kind === "file" && typeof fact === "object" && fact !== null && (fact.mtimeMs >= started - RACY_MS || fact.ctimeMs >= started - RACY_MS));
  usedInference.set(cachePath, null);
  if (!racy && jsonExact(result) && current(rows)) {
    fs.mkdirSync(dirname(cachePath), { recursive: true });
    const image=JSON.stringify(result),temporary = `${cachePath}.${process.pid}.${started}.tmp`, header = JSON.stringify({ digest, resultSha256:createHash("sha256").update(image).digest("hex"), authorityObservations: [...loadObservations.values()], observations: rows });
    fs.writeFileSync(temporary, `${header}\n${image}`);
    fs.renameSync(temporary, cachePath);
    usedInference.set(cachePath, createHash("sha256").update(header).digest("hex"));
  }
  return result;
}

/**
 * 🧾️ Runs a graph dependency computation while recording every filesystem fact it observes, then publishes a receipt for the graph seal:
 * this process, when it completed, the exact inference records (by header digest) this process's node inference used in the receipt's
 * directory, and the dependency
 * facts. A receipt is published only when every input is non-racy and still holds and every inference record used was persisted; otherwise
 * any previous receipt is removed, so a seal can never pair a graph with inputs it was not computed from.
 * @template T
 * @param {{ readonly receiptPath: string; readonly recordDirectory?: string }} options
 * @param {() => Promise<T> | T} compute
 * @returns {Promise<T>}
 */
export async function observeDependencies({ receiptPath, recordDirectory }, compute) {
  const observations = new Map(), started = Date.now(),publicationPath=receiptPath.replace(/\.receipt\.json$/u,".publication.json"),hints=new Map();
  if(recordDirectory)try{const previous=JSON.parse(fs.readFileSync(publicationPath,"utf8"));if(previous.version===1&&previous.revision===OBSERVATION_REVISION&&Array.isArray(previous.observations)&&Number.isSafeInteger(previous.startedAt))for(const row of previous.observations)if(row.kind==="file"&&/^[a-f0-9]{64}$/u.test(row.fact?.sha256??"")&&row.fact.mtimeMs<previous.startedAt-RACY_MS&&row.fact.ctimeMs<previous.startedAt-RACY_MS)hints.set(row.path,row.fact);}catch{}
  let records=recordDirectory&&recordStores.get(recordDirectory);if(recordDirectory&&!records)recordStores.set(recordDirectory,records=openRecordStore(recordDirectory));
  const run=()=>scopes.run(observations,compute),result=await(records?recordScopes.run({records,hints},run):run());
  records?.flush();if(records&&records.stats.records>2*records.stats.accessed)records.retainAccessed();
  const rows = [...observations.values()], inference = [...usedInference].filter(([path]) => dirname(path) === dirname(receiptPath)).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0);
  const racy = rows.some(({ kind, fact }) => kind === "file" && typeof fact === "object" && fact !== null && (fact.mtimeMs >= started - RACY_MS || fact.ctimeMs >= started - RACY_MS));
  fs.mkdirSync(dirname(receiptPath), { recursive: true });
  const originalInference=[...originalInferencePublications.values()].filter(value=>dirname(value.path)===dirname(receiptPath)).sort((left,right)=>Buffer.compare(Buffer.from(left.path),Buffer.from(right.path)));
  if(publicationPath===receiptPath)throw Error("Original inference receipt must have an owned publication path");
  const producerPath=resolve(dirname(publicationPath),"p-"+process.pid,basename(publicationPath));
  if(originalInference.length&&jsonImage(result)!==undefined){const image=JSON.stringify({version:1,revision:OBSERVATION_REVISION,pid:process.pid,startedAt:started,completedAt:Date.now(),inference:originalInference,observations:rows});fs.mkdirSync(dirname(producerPath),{recursive:true});for(const path of [producerPath,publicationPath]){const temporary=path+"."+process.pid+"."+started+".tmp",parent=resolve(dirname(path)),status=fs.lstatSync(parent);if(temporary.length>256||[...temporary].length>256||!status.isDirectory()||status.isSymbolicLink()||fs.realpathSync(parent)!==parent)throw Error("Original inference producer namespace refused");fs.writeFileSync(temporary,image,{flag:"wx"});fs.renameSync(temporary,path);}}else for(const path of [producerPath,publicationPath])fs.rmSync(path,{force:true});
  if (racy || inference.length === 0 || inference.some(([, digest]) => digest === null) || !current(rows)) { fs.rmSync(receiptPath, { force: true }); return result; }
  const temporary = `${receiptPath}.${process.pid}.${started}.tmp`;
  fs.writeFileSync(temporary, JSON.stringify({ format: 1, revision: OBSERVATION_REVISION, pid: process.pid, completedAt: Date.now(), inference, observations: rows }));
  fs.renameSync(temporary, receiptPath);
  return result;
}

/** 📥️ Captures freshly computed source facts and refuses a preparation whose inputs changed before receipt. */
export async function captureSourcePreparationV1(compute, control) {
  const observations = new Map(), value = await scopes.run(observations, compute), original = [...observations.values()];
  return {value,original,received:await receiveSourcePreparationV1(original,control)};
}

/** 🔐️ Rechecks and records every original source fingerprint under the same preparation control. */
export async function receiveSourcePreparationV1(original,control){
  const received=[];
  for(const row of original){
    control.check();if(received.length%128===0)await control.advance("source-fingerprints");
    const next={kind:row.kind,path:row.path,fact:facts[row.kind](row.path)};
    if(JSON.stringify(row.fact)!==JSON.stringify(next.fact))throw Error(`Original source preparation inputs changed: ${row.kind}:${row.path}`);
    received.push(next);
  }
  return received;
}

/** 📜️ The header line of a persisted inference record (its digest and observations), read without the result that follows it. */
export function inferenceHeaderLine(cachePath) {
  const fd = fs.openSync(cachePath, "r"), chunks = [], chunk = Buffer.allocUnsafe(1 << 20);
  try {
    for (let position = 0; ;) {
      const count = fs.readSync(fd, chunk, 0, chunk.byteLength, position);
      if (count === 0) return null;
      const newline = chunk.subarray(0, count).indexOf(10);
      chunks.push(Buffer.from(chunk.subarray(0, newline < 0 ? count : newline)));
      if (newline >= 0) return Buffer.concat(chunks).toString("utf8");
      position += count;
    }
  } finally { fs.closeSync(fd); }
}

/**
 * ✅️ {@link current} without blocking: every recorded fact re-checked through the libuv pool, `limit` at a time; false at the first stale
 * fact. Existence, type and listing facts about entries of a directory in `stableDirectories` (absolute paths whose entry sets are proven
 * unchanged, e.g. by unchanged directory stamps) hold without a probe: an entry can only appear, vanish or change type by changing its parent.
 * Existence follows symbolic links, so it is probed for every entry in `symlinks`.
 */
export async function observationsHold(observations, limit = 64, stableDirectories = new Set(), symlinks = new Set()) {
  let next = 0, stale = false;
  const check = async ({ kind, path, fact }) => {
    const parent = path.slice(0, Math.max(path.lastIndexOf("/"), path.lastIndexOf("\\")));
    if (kind === "lstat" && stableDirectories.has(parent) || kind === "exists" && stableDirectories.has(parent) && !symlinks.has(path) || kind === "readdir" && stableDirectories.has(path)) return true;
    if (kind === "file") {
      if (fact.error !== undefined) return fileFactHolds(path, fact);
      let stamp;
      try { stamp = await statAsync(path); } catch { return false; }
      if (stamp.size !== fact.size) return false;
      if (stamp.mtimeMs === fact.mtimeMs && stamp.ctimeMs === fact.ctimeMs && stamp.ino === fact.ino) return true;
      try { return createHash("sha256").update(await readFileAsync(path)).digest("hex") === fact.sha256; } catch { return false; }
    }
    if (kind === "exists") { try { await statAsync(path); return fact === true; } catch { return fact === false; } }
    return typeof facts[kind] === "function" && JSON.stringify(facts[kind](path)) === JSON.stringify(fact);
  };
  const worker = async () => { while (!stale && next < observations.length) if (!await check(observations[next++])) stale = true; };
  await Promise.all(Array.from({ length: Math.min(limit, observations.length) }, worker));
  return !stale;
}
