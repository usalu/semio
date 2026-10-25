import { createHash, randomBytes } from "node:crypto";

/** 🧭️ One relative source selection, independent of repository and task taxonomies. */
export type FilesystemJournalScopeV1 = Readonly<{ roots: readonly string[]; files: readonly string[] }>;
/** 🔔️ One owned native change without a backend event type in its public API. */
export type FilesystemJournalChangeV1 = Readonly<{ path: string; type: "create" | "update" | "delete" }>;
/** 📚️ A closed membership vector and its UTF-8 path digest. */
export type FilesystemJournalMembershipV1 = Readonly<{ paths: readonly string[]; sha256: string }>;
/** 🔏️ A live epoch and latest mutation affecting this selection or global identity. */
export type FilesystemJournalWitnessV1 = Readonly<{ version: 1; epoch: string; sequence: number; globalIdentity: FilesystemJournalMembershipV1; selectedMembership: FilesystemJournalMembershipV1; changes: readonly FilesystemJournalChangeV1[] }>;
/** 🎛️ Complete caller authority forwarded unchanged through observation. */
export interface FilesystemJournalControlV1 {
 readonly signal: AbortSignal;
 remainingMilliseconds(): number | null;
 publish(progress: Readonly<{ resource: string; mode: "shared"; elapsedMs: number }>): void | Promise<void>;
 yieldContinuation(): Promise<void>;
}
/** 🔌️ Native lifecycle and no-follow reconciliation behind owned filesystem ports. */
export interface FilesystemJournalSourceV1 {
 watch(receive: (error: unknown, events: readonly FilesystemJournalChangeV1[]) => void): (() => void | Promise<void>) | Promise<() => void | Promise<void>>;
 census(control: FilesystemJournalControlV1): Promise<readonly string[]>;
 flush(): readonly FilesystemJournalChangeV1[] | Promise<readonly FilesystemJournalChangeV1[]>;
 reconcile(event: FilesystemJournalChangeV1, control: FilesystemJournalControlV1, priorAffected: readonly string[]): Promise<readonly string[]>;
 admitScope(scope: FilesystemJournalScopeV1, control: FilesystemJournalControlV1): Promise<void>;
}
/** 🪟️ A journal authorizes reuse only while its native service remains live. */
export interface LiveFilesystemJournalV1 {
 capture(scope: FilesystemJournalScopeV1, control: FilesystemJournalControlV1): Promise<FilesystemJournalWitnessV1>;
 verify(witness: FilesystemJournalWitnessV1, scope: FilesystemJournalScopeV1, control: FilesystemJournalControlV1): Promise<boolean>;
 close(): Promise<void>;
}
/** 🧰️ Capacity and identity policy belong to the invoking repository extension. */
export type FilesystemJournalOptionsV1 = Readonly<{ source: FilesystemJournalSourceV1; isGlobalIdentity(path: string): boolean; excluded(path: string): boolean; maximumPaths: number; maximumEvents: number }>;

function check(control: FilesystemJournalControlV1): void {
 if (!control || !(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Complete filesystem journal control required");
 control.signal.throwIfAborted();
 const budget = control.remainingMilliseconds();
 if (budget !== null && (!Number.isFinite(budget) || budget <= 0)) throw Error("Filesystem journal original deadline exhausted");
}
function pathValue(path: unknown): string {
 if (typeof path !== "string" || !path || path.length > 256 || path.normalize("NFC") !== path || /[:\\\0]/u.test(path) || path.split("/").some(part => !part || part === "." || part === "..")) throw Error("Invalid filesystem journal relative path");
 return path;
}
function pathsValue(paths: readonly string[]): string[] {
 if (!Array.isArray(paths)) throw Error("Filesystem journal paths must be an array");
 return [...new Set(paths.map(pathValue))].sort(comparePath);
}
function comparePath(left: string, right: string): number { return Buffer.compare(Buffer.from(left), Buffer.from(right)); }
function descendants(paths: readonly string[], root: string): readonly string[] {
 const prefix = root + "/";
 let low = 0, high = paths.length;
 while (low < high) { const middle = (low + high) >>> 1; if (comparePath(paths[middle]!, prefix) < 0) low = middle + 1; else high = middle; }
 let end = low; while (end < paths.length && paths[end]!.startsWith(prefix)) end++;
 return paths.slice(low, end);
}
/** 📨️ Reads a closed immutable journal scope without granting observer or filesystem authority. */
export function readFilesystemJournalScopeV1(scope: FilesystemJournalScopeV1): FilesystemJournalScopeV1 {
 if (!scope || Object.keys(scope).sort().join(",") !== "files,roots") throw Error("Closed filesystem journal scope required");
 const roots = pathsValue(scope.roots), files = pathsValue(scope.files);
 if (roots.length !== scope.roots.length || files.length !== scope.files.length) throw Error("Filesystem journal scope repeats a path");
 return Object.freeze({ roots: Object.freeze(roots), files: Object.freeze(files) });
}
function membership(paths: readonly string[]): FilesystemJournalMembershipV1 {
 const ordered = pathsValue(paths), hash = createHash("sha256");
 for (const path of ordered) hash.update(path + "\0");
 return Object.freeze({ paths: Object.freeze(ordered), sha256: hash.digest("hex") });
}
function witnessValid(witness: FilesystemJournalWitnessV1): boolean {
 if (!witness || Object.keys(witness).sort().join(",") !== "changes,epoch,globalIdentity,selectedMembership,sequence,version" || witness.version !== 1 || !/^[a-f0-9]{32}$/u.test(witness.epoch) || !Number.isSafeInteger(witness.sequence) || witness.sequence < 0 || !Array.isArray(witness.changes)) return false;
 for (const row of [witness.globalIdentity, witness.selectedMembership]) {
  if (!row || Object.keys(row).sort().join(",") !== "paths,sha256" || !Array.isArray(row.paths) || row.paths.length !== new Set(row.paths).size || membership(row.paths).sha256 !== row.sha256) return false;
 }
 return witness.changes.every(row => row && Object.keys(row).sort().join(",") === "path,type" && ["create", "update", "delete"].includes(row.type) && pathValue(row.path) === row.path);
}
function awaitControl<T>(work: Promise<T>, control: FilesystemJournalControlV1, abandon: (reason: unknown) => void, ownerSignal: AbortSignal): Promise<T> {
 check(control);
 return new Promise((accept, reject) => {
  let settled = false, timer: ReturnType<typeof setTimeout> | undefined;
  const finish = (failed: boolean, value: unknown): void => { if (settled) return; settled = true; clearTimeout(timer); control.signal.removeEventListener("abort", abort); ownerSignal.removeEventListener("abort", ownerAbort); if (failed) { abandon(value); reject(value); } else accept(value as T); };
  const abort = (): void => finish(true, control.signal.reason ?? Error("Filesystem journal caller cancelled"));
  const ownerAbort = (): void => finish(true, ownerSignal.reason ?? Error("Filesystem journal owner cancelled"));
  const arm = (): void => {
   if (settled) return;
   let left: number | null;
   try { left = control.remainingMilliseconds(); } catch (error) { finish(true, error); return; }
   if (left === null) return;
   if (!Number.isFinite(left) || left <= 0) { finish(true, Error("Filesystem journal original deadline exhausted")); return; }
   timer = setTimeout(arm, Math.max(1, Math.min(2147483647, Math.ceil(left))));
  };
  arm();
  control.signal.addEventListener("abort", abort, { once: true });
  ownerSignal.addEventListener("abort", ownerAbort, { once: true });
  work.then(value => finish(false, value), error => finish(true, error));
  if (control.signal.aborted) abort();
  else if (ownerSignal.aborted) ownerAbort();
 });
}

/** 👁️ Registers before census, drains native events around capture, and refuses uncertain custody.
 * @see ./🧬️schema/🔣️.json
 */
export async function createLiveFilesystemJournalV1(options: FilesystemJournalOptionsV1, control: FilesystemJournalControlV1): Promise<LiveFilesystemJournalV1> {
 check(control);
 if (!Number.isSafeInteger(options.maximumPaths) || options.maximumPaths < 1 || !Number.isSafeInteger(options.maximumEvents) || options.maximumEvents < 1) throw Error("Finite filesystem journal capacities required");
 const epoch = randomBytes(16).toString("hex"), files = new Set<string>(), globalFiles = new Set<string>(), mutations = new Map<string, { sequence: number; event: FilesystemJournalChangeV1; global: boolean }>(), pending: FilesystemJournalChangeV1[] = [];
 let counter = 0, closed = false, refusal: Error | undefined, tail: Promise<unknown> = Promise.resolve();
 let orderedFiles: readonly string[] | undefined, globalMembership: FilesystemJournalMembershipV1 | undefined;
 const fileIndex = (): readonly string[] => orderedFiles ??= [...files].sort(comparePath);
 const put = (path: string): void => { if (files.has(path)) return; files.add(path); orderedFiles = undefined; if (options.isGlobalIdentity(path)) { globalFiles.add(path); globalMembership = undefined; } };
 const remove = (path: string): void => { if (!files.delete(path)) return; orderedFiles = undefined; if (globalFiles.delete(path)) globalMembership = undefined; };
 const select = (scope: FilesystemJournalScopeV1): FilesystemJournalMembershipV1 => membership([...scope.files.filter(path => files.has(path)), ...scope.roots.flatMap(root => [...(files.has(root) ? [root] : []), ...descendants(fileIndex(), root)])]);
 const receive = (error: unknown, events: readonly FilesystemJournalChangeV1[]): void => {
  if (closed || refusal) return;
  if (error) { refusal = Error("Filesystem journal watcher refused", { cause: error }); return; }
  try {
   if (!Array.isArray(events)) throw Error("Unknown filesystem journal event batch");
   for (const event of events) {
    if (!event || Object.keys(event).sort().join(",") !== "path,type" || !["create", "update", "delete"].includes(event.type)) throw Error("Unknown filesystem journal event");
    if (typeof event.path === "string" && options.excluded(event.path)) continue;
    const path = pathValue(event.path);
    pending.push(Object.freeze({ path, type: event.type }));
    if (pending.length > options.maximumEvents) throw Error("Filesystem journal event overflow");
   }
  } catch (error) { refusal = error instanceof Error ? error : Error("Filesystem journal event refused"); }
 };
 const stop = await options.source.watch(receive);
 let stopping: Promise<void> | undefined, closing: Promise<void> | undefined;
 const stopOnce = (): Promise<void> => stopping ??= Promise.resolve().then(stop);
 const close = (): Promise<void> => closing ??= (async () => { closed = true; control.signal.removeEventListener("abort", ownerAbort); await stopOnce(); await tail; files.clear(); globalFiles.clear(); mutations.clear(); pending.length = 0; orderedFiles = undefined; globalMembership = undefined; })();
 const ownerAbort = (): void => { void close().catch(error => { refusal = Error("Filesystem journal retirement refused", {cause: error}); }); };
 control.signal.addEventListener("abort", ownerAbort, {once: true});
 if (control.signal.aborted) ownerAbort();
 const live = (): void => { check(control); if (closed) throw Error("Filesystem journal closed"); if (refusal) throw refusal; };
 const drain = async (current: FilesystemJournalControlV1): Promise<void> => {
  receive(null, await options.source.flush()); live(); check(current);
  while (pending.length) {
   const event = pending[0]!, affected = files.has(event.path) ? [event.path] : descendants(fileIndex(), event.path);
   let reconciled: readonly string[];
   try { reconciled = await options.source.reconcile(event, current, affected); check(current); live(); }
   catch (error) { check(current); refusal = error instanceof Error ? error : Error("Filesystem journal source refused"); throw refusal; }
   pending.shift();
   const admitted = pathsValue(reconciled).filter(path => !options.excluded(path)), retained = new Set(admitted), added = admitted.filter(path => !files.has(path));
   const global = options.isGlobalIdentity(event.path) || affected.some(path => globalFiles.has(path)) || admitted.some(path => options.isGlobalIdentity(path)) || mutations.get(event.path)?.global === true;
   for (const path of affected) if (!retained.has(path)) remove(path);
   for (const path of admitted) put(path);
   mutations.set(event.path, { sequence: ++counter, event, global });
   for (const path of added) if (path !== event.path) mutations.set(path, { sequence: ++counter, event: Object.freeze({path,type:"create"}), global: options.isGlobalIdentity(path) });
   if (files.size > options.maximumPaths || mutations.size > options.maximumEvents) { refusal = Error("Filesystem journal capacity overflow"); throw refusal; }
   await current.publish({ resource: "filesystem-journal:reconcile", mode: "shared", elapsedMs: 0 }); check(current);
   await current.yieldContinuation(); check(current);
  }
 };
 try {
  for (const path of pathsValue(await options.source.census(control))) if (!options.excluded(path)) put(path);
  if (files.size > options.maximumPaths) throw Error("Filesystem journal path capacity overflow");
  await drain(control);
 } catch (error) { await close(); throw error; }
 const capture = async (requested: FilesystemJournalScopeV1, current: FilesystemJournalControlV1): Promise<FilesystemJournalWitnessV1> => {
  check(current);
  let abandoned = false, abandonmentReason: unknown;
  const work = tail.then(async () => {
   if (abandoned) throw abandonmentReason;
   check(current); live();
   const scope = readFilesystemJournalScopeV1(requested), selected = (path: string): boolean => scope.files.includes(path) || scope.roots.some(root => path === root || path.startsWith(root + "/"));
   await options.source.admitScope(scope, current); check(current); live(); if (abandoned) throw abandonmentReason;
   for (let attempt = 0; attempt < 4; attempt++) {
    await drain(current);
    const before = counter, globalIdentity = globalMembership ??= membership([...globalFiles]), selectedMembership = select(scope);
    await drain(current); check(current); live(); if (abandoned) throw abandonmentReason;
    if (before !== counter) continue;
    const relevant = [...mutations.values()].filter(row => row.global || selected(row.event.path) || scope.roots.some(root => root.startsWith(row.event.path + "/")) || scope.files.some(file => file.startsWith(row.event.path + "/")));
    return Object.freeze({ version: 1 as const, epoch, sequence: relevant.reduce((maximum, row) => Math.max(maximum, row.sequence), 0), globalIdentity, selectedMembership, changes: Object.freeze(relevant.map(row => row.event)) });
   }
   throw Error("Filesystem journal capture did not reach a stable boundary");
  });
  tail = work.then(() => undefined, () => undefined);
  return awaitControl(work, current, reason => { abandoned = true; abandonmentReason = reason; }, control.signal);
 };
 return Object.freeze({
  capture,
  async verify(witness: FilesystemJournalWitnessV1, scope: FilesystemJournalScopeV1, current: FilesystemJournalControlV1): Promise<boolean> {
   check(current);
   try { if (!witnessValid(witness)) return false; const observed = await capture(scope, current); return witness.epoch === observed.epoch && witness.sequence === observed.sequence && witness.globalIdentity.sha256 === observed.globalIdentity.sha256 && witness.selectedMembership.sha256 === observed.selectedMembership.sha256; }
   catch { check(current); return false; }
  },
  close
 });
}
