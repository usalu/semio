import { existsSync, lstatSync, readFileSync, readdirSync, mkdirSync, writeFileSync, rmSync, mkdtempSync, chmodSync, renameSync } from "node:fs";
import { createHash } from "node:crypto";
import { copyFile, open } from "node:fs/promises";
import { setTimeout as delay } from "node:timers/promises";
import { basename, dirname, resolve, join, isAbsolute, relative } from "node:path";
import { acquireResourceLease, type LeaseWait } from "../../🔒️leases/🟦️.ts";

export type ArtifactPublicationOptions = { readonly signal?: AbortSignal; readonly leaseDirectory: string; readonly onWait?: (progress: LeaseWait) => void };

/** 🧮️ Compares file bytes in bounded async chunks while honoring publication cancellation. */
async function haveIdenticalBytes(source: string, destination: string, signal: AbortSignal): Promise<boolean> {
  const left = await open(source, "r");
  try {
    const right = await open(destination, "r");
    try {
      const leftBytes = Buffer.allocUnsafe(65536), rightBytes = Buffer.allocUnsafe(65536);
      for (;;) {
        signal.throwIfAborted();
        const [a, b] = await Promise.all([left.read(leftBytes, 0, leftBytes.length, null), right.read(rightBytes, 0, rightBytes.length, null)]);
        signal.throwIfAborted();
        if (a.bytesRead !== b.bytesRead || !leftBytes.subarray(0, a.bytesRead).equals(rightBytes.subarray(0, b.bytesRead))) return false;
        if (!a.bytesRead) return true;
      }
    } finally { await right.close(); }
  } finally { await left.close(); }
}

/** 🔍️ Recognizes identical owned files without replacing resources already open in consumers. */
async function hasIdenticalArtifacts(staging: string, marker: string, owner: string, files: ReadonlyMap<string, string>, signal: AbortSignal): Promise<boolean> {
  if (!existsSync(join(staging, marker))) return false;
  const names = [...files.keys()].sort(), metadata = JSON.parse(readFileSync(join(staging, marker), "utf8"));
  if (metadata.version !== 1 || metadata.owner !== owner || JSON.stringify(metadata.files) !== JSON.stringify(names)) return false;
  const paths = new Set([marker]);
  for (const name of names) {
    const destination = resolve(staging, name);
    for (let path = destination; path !== resolve(staging); path = dirname(path)) paths.add(relative(staging, path).replaceAll("\\", "/"));
  }
  const inspect = (directory: string): boolean => readdirSync(directory, { withFileTypes: true }).every(entry => {
    signal.throwIfAborted();
    const path = join(directory, entry.name), name = relative(staging, path).replaceAll("\\", "/");
    return paths.has(name) && !entry.isSymbolicLink() && (entry.isDirectory() ? inspect(path) : entry.isFile());
  });
  if (!inspect(staging)) return false;
  for (const [name, source] of files) {
    signal.throwIfAborted();
    const destination = join(staging, name);
    if (!existsSync(destination)) return false;
    const published = lstatSync(destination), authored = lstatSync(source);
    if (!published.isFile() || published.size !== authored.size || (published.mode & 0o777) !== (authored.mode & 0o777) || !await haveIdenticalBytes(source, destination, signal)) return false;
  }
  return true;
}


/** 🚚️ Waits for transient directory contention while preserving cancellation and bounded recovery. */
async function renameArtifactDirectory(source: string, destination: string, signal: AbortSignal, onWait?: (elapsedMs: number) => void): Promise<void> {
  const started = Date.now();
  for (let attempt = 0; ; attempt++) {
    signal.throwIfAborted();
    try { renameSync(source, destination); return; }
    catch (error) {
      const elapsedMs = Date.now() - started;
      if (!["EPERM", "EACCES", "EBUSY"].includes((error as NodeJS.ErrnoException).code ?? "") || elapsedMs >= 5000) throw error;
      onWait?.(elapsedMs);
      signal.throwIfAborted();
      try { await delay(Math.min(25 * 2 ** Math.min(attempt, 3), 5000 - elapsedMs), undefined, { signal }); }
      catch (error) { signal.throwIfAborted(); throw error; }
    }
  }
}

/** 🧱️ Reuses identical owned artifacts or replaces their tree with rollback on publication failure. */
export async function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>, options: ArtifactPublicationOptions): Promise<void> {
  if(!options||!isAbsolute(options.leaseDirectory))throw Error("Explicit absolute publication lease directory required");
  const signal = options.signal ?? new AbortController().signal;
  const started = Date.now();
  const lease = await acquireResourceLease({ directory: options.leaseDirectory, resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal, onWait: options.onWait });
  let temporary: string | undefined;
  try {
    const marker = ".nx-artifact.json";
    for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
    for (const name of files.keys()) if (isAbsolute(name) || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
    if (existsSync(staging)) {
      const markerPath = join(staging, marker);
      if (!existsSync(markerPath)) rmSync(staging, { recursive: true, force: true });
      else if (JSON.parse(readFileSync(markerPath, "utf8")).owner !== owner) throw new Error(`Unowned artifact directory: ${staging}`);
    }
    const identical = await hasIdenticalArtifacts(staging, marker, owner, files, signal);
    signal.throwIfAborted();
    if (identical) return;
    mkdirSync(dirname(staging), { recursive: true });
    temporary = mkdtempSync(`${staging}.stage-`);
    const previous = `${temporary}.previous`;
    for (const [name, source] of files) {
      signal.throwIfAborted();
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      await copyFile(source, destination);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    signal.throwIfAborted();
    const onWait = (): void => options.onWait?.({ resource: `artifact:${resolve(staging)}`, mode: "exclusive", elapsedMs: Date.now() - started });
    if (existsSync(staging)) await renameArtifactDirectory(staging, previous, signal, onWait);
    try { await renameArtifactDirectory(temporary, staging, signal, onWait); }
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
  const signal=options.signal??new AbortController().signal;
  const lease=await acquireResourceLease({directory:options.leaseDirectory,resource:`artifact:${dirname(source)}`,mode:"shared",signal,onWait:options.onWait});
  try {
    const digest=await executableDigest(source,signal),name=basename(source),directory=join(cacheDirectory,digest);
    await stageArtifacts(directory,owner,new Map([[name,source]]),options);
    const executable=join(directory,name);
    if(await executableDigest(executable,signal)!==digest)throw Error("Executable artifact changed during acquisition");
    return executable;
  } finally {lease.release();}
}

/** 🔏️ Hashes executable bytes in bounded cancellable reads. */
async function executableDigest(path: string, signal: AbortSignal): Promise<string> {
  const file=await open(path,"r"),hash=createHash("sha256"),bytes=Buffer.allocUnsafe(65536);
  try {
    for(;;){signal.throwIfAborted();const {bytesRead}=await file.read(bytes,0,bytes.length,null);signal.throwIfAborted();if(!bytesRead)return hash.digest("hex");hash.update(bytes.subarray(0,bytesRead));}
  } finally {await file.close();}
}
