import { existsSync, lstatSync, readFileSync, readdirSync, mkdirSync, writeFileSync, rmSync, mkdtempSync, chmodSync, renameSync } from "node:fs";
import { createHash } from "node:crypto";
import { copyFile, open } from "node:fs/promises";
import { setTimeout as delay } from "node:timers/promises";
import { basename, dirname, resolve, join, isAbsolute, relative } from "node:path";
import { acquireResourceLease, checkResourceLeaseControl, type ResourceLeaseControl, type LeaseWait } from "../../🔒️leases/🟦️.ts";

export type ArtifactPublicationOptions = { readonly control?: ResourceLeaseControl; readonly signal?: AbortSignal; readonly leaseDirectory: string; readonly onWait?: (progress: LeaseWait) => void };

/** 🧮️ Compares file bytes in bounded async chunks while honoring publication cancellation. */
async function haveIdenticalBytes(source: string, destination: string, signal: AbortSignal, control?: ResourceLeaseControl): Promise<boolean> {
  const left = await open(source, "r");
  try {
    const right = await open(destination, "r");
    try {
      const leftBytes = Buffer.allocUnsafe(65536), rightBytes = Buffer.allocUnsafe(65536);
      for (;;) {
        signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
        const [a, b] = await Promise.all([left.read(leftBytes, 0, leftBytes.length, null), right.read(rightBytes, 0, rightBytes.length, null)]);
        signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
        if (a.bytesRead !== b.bytesRead || !leftBytes.subarray(0, a.bytesRead).equals(rightBytes.subarray(0, b.bytesRead))) return false;
        if (!a.bytesRead) return true;
      }
    } finally { await right.close(); }
  } finally { await left.close(); }
}

/** 🔍️ Recognizes identical owned files without replacing resources already open in consumers. */
async function hasIdenticalArtifacts(staging: string, marker: string, owner: string, files: ReadonlyMap<string, string>, signal: AbortSignal, control?: ResourceLeaseControl): Promise<boolean> {
  if (!existsSync(join(staging, marker))) return false;
  const names = [...files.keys()].sort(), metadata = JSON.parse(readFileSync(join(staging, marker), "utf8"));
  if (metadata.version !== 1 || metadata.owner !== owner || JSON.stringify(metadata.files) !== JSON.stringify(names)) return false;
  const paths = new Set([marker]);
  for (const name of names) {
    const destination = resolve(staging, name);
    for (let path = destination; path !== resolve(staging); path = dirname(path)) paths.add(relative(staging, path).replaceAll("\\", "/"));
  }
  const inspect = (directory: string): boolean => readdirSync(directory, { withFileTypes: true }).every(entry => {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    const path = join(directory, entry.name), name = relative(staging, path).replaceAll("\\", "/");
    return paths.has(name) && !entry.isSymbolicLink() && (entry.isDirectory() ? inspect(path) : entry.isFile());
  });
  if (!inspect(staging)) return false;
  for (const [name, source] of files) {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    const destination = join(staging, name);
    if (!existsSync(destination)) return false;
    const published = lstatSync(destination), authored = lstatSync(source);
    if (!published.isFile() || published.size !== authored.size || (published.mode & 0o777) !== (authored.mode & 0o777) || !await haveIdenticalBytes(source, destination, signal, control)) return false;
  }
  return true;
}


/** 🚚️ Waits for transient directory contention while preserving cancellation and bounded recovery. */
async function renameArtifactDirectory(source: string, destination: string, signal: AbortSignal, onWait?: (elapsedMs: number) => void, control?: ResourceLeaseControl): Promise<void> {
  const started = Date.now();
  for (let attempt = 0; ; attempt++) {
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    try { renameSync(source, destination); return; }
    catch (error) {
      const elapsedMs = Date.now() - started;
      if (!["EPERM", "EACCES", "EBUSY"].includes((error as NodeJS.ErrnoException).code ?? "") || elapsedMs >= 5000) throw error;
      onWait?.(elapsedMs);
      signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
      try { await delay(Math.min(25 * 2 ** Math.min(attempt, 3), 5000 - elapsedMs, control ? checkResourceLeaseControl(control) ?? 5000 : 5000), undefined, { signal }); }
      catch (error) { signal.throwIfAborted(); if (control) checkResourceLeaseControl(control); throw error; }
    }
  }
}

/** 🧱️ Reuses identical owned artifacts or replaces their tree with rollback on publication failure. */
export async function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>, options: ArtifactPublicationOptions): Promise<void> {
  if(!options||!isAbsolute(options.leaseDirectory))throw Error("Explicit absolute publication lease directory required");
  const control = options.control, signal = control?.signal ?? options.signal ?? new AbortController().signal;
  if (control) { checkResourceLeaseControl(control); if (options.signal && options.signal !== signal) throw Error("Artifact publication requires original signal"); }
  const started = Date.now();
  const lease = await acquireResourceLease({ directory: options.leaseDirectory, resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal, control, onWait: options.onWait });
  let temporary: string | undefined;
  try {
    const marker = ".semio-artifact.json";
    for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
    for (const name of files.keys()) if (isAbsolute(name) || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
    if (existsSync(staging)) {
      const markerPath = join(staging, marker);
      if (!existsSync(markerPath)||JSON.parse(readFileSync(markerPath, "utf8")).owner !== owner) throw new Error(`Unowned artifact directory: ${staging}`);
    }
    const identical = await hasIdenticalArtifacts(staging, marker, owner, files, signal, control);
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    if (identical) return;
    mkdirSync(dirname(staging), { recursive: true });
    temporary = mkdtempSync(`${staging}.stage-`);
    const previous = `${temporary}.previous`;
    for (const [name, source] of files) {
      signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      await copyFile(source, destination);
      if (control) checkResourceLeaseControl(control);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    signal.throwIfAborted(); if (control) checkResourceLeaseControl(control);
    const onWait = (): void => options.onWait?.({ resource: `artifact:${resolve(staging)}`, mode: "exclusive", elapsedMs: Date.now() - started });
    if (existsSync(staging)) await renameArtifactDirectory(staging, previous, signal, onWait, control);
    try { await renameArtifactDirectory(temporary, staging, signal, onWait, control); }
    catch (error) {
      try { if (existsSync(previous)) await renameArtifactDirectory(previous, staging, new AbortController().signal, onWait); }
      catch (rollback) { throw new AggregateError([error, rollback], `Artifact publication rollback failed: ${staging}`); }
      throw error;
    }
    rmSync(previous, { recursive: true, force: true });
  } finally { try { if (temporary) rmSync(temporary, { recursive: true, force: true }); } finally { lease.release(); } }
}

/** 🧊️ Acquires immutable executable bytes before releasing the publisher's shared lease. */
export async function pinExecutableArtifact(source: string, cacheDirectory: string, owner: string, options: ArtifactPublicationOptions): Promise<string> {
  if (!isAbsolute(source) || !isAbsolute(cacheDirectory) || !isAbsolute(options.leaseDirectory)) throw Error("Explicit absolute executable publication paths required");
  const control=options.control,signal=control?.signal??options.signal??new AbortController().signal;
  if(control){checkResourceLeaseControl(control);if(options.signal&&options.signal!==signal)throw Error("Executable acquisition requires original signal");}
  const lease=await acquireResourceLease({directory:options.leaseDirectory,resource:`artifact:${dirname(source)}`,mode:"shared",signal,control,onWait:options.onWait});
  try {
    const digest=await executableDigest(source,signal,control),name=basename(source),directory=join(cacheDirectory,digest);
    await stageArtifacts(directory,owner,new Map([[name,source]]),options);
    const executable=join(directory,name);
    if(await executableDigest(executable,signal,control)!==digest)throw Error("Executable artifact changed during acquisition");
    return executable;
  } finally {lease.release();}
}

/** 🔏️ Hashes executable bytes in bounded cancellable reads. */
async function executableDigest(path: string, signal: AbortSignal, control?: ResourceLeaseControl): Promise<string> {
  signal.throwIfAborted();if(control)checkResourceLeaseControl(control);
  const file=await open(path,"r"),hash=createHash("sha256"),bytes=Buffer.allocUnsafe(65536),started=Date.now();let chunks=0;
  try {
    for(;;){signal.throwIfAborted();if(control)checkResourceLeaseControl(control);const {bytesRead}=await file.read(bytes,0,bytes.length,null);signal.throwIfAborted();if(control)checkResourceLeaseControl(control);if(!bytesRead)return hash.digest("hex");hash.update(bytes.subarray(0,bytesRead));if(control&&++chunks%64===0){await control.publish({resource:`artifact:${path}`,mode:"shared",elapsedMs:Date.now()-started});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);}}
  } finally {await file.close();}
}
