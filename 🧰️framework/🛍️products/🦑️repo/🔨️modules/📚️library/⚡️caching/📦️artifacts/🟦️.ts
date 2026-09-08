import { existsSync, lstatSync, readFileSync, mkdirSync, writeFileSync, rmSync, mkdtempSync, copyFileSync, chmodSync, renameSync } from "node:fs";
import { dirname, resolve, join } from "node:path";

/** 🧱️ Replaces only a previously owned deliverable tree and restores it if publication fails. */
export function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>): void {
  const marker = ".nx-artifact.json";
  for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
  if (existsSync(staging) && (!existsSync(join(staging, marker)) || JSON.parse(readFileSync(join(staging, marker), "utf8")).owner !== owner)) throw new Error(`Unowned artifact directory: ${staging}`);
  mkdirSync(dirname(staging), { recursive: true });
  const lease = `${staging}.lease`;
  writeFileSync(lease, JSON.stringify({ owner, pid: process.pid }), { flag: "wx" });
  let temporary: string | undefined;
  try { temporary = mkdtempSync(`${staging}.stage-`); }
  catch (error) { rmSync(lease); throw error; }
  const previous = `${temporary}.previous`;
  try {
    for (const [name, source] of files) {
      if (name.startsWith("/") || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      copyFileSync(source, destination);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    if (existsSync(staging)) renameSync(staging, previous);
    try { renameSync(temporary, staging); }
    catch (error) { if (existsSync(previous)) renameSync(previous, staging); throw error; }
    rmSync(previous, { recursive: true, force: true });
  } finally { rmSync(temporary, { recursive: true, force: true }); rmSync(lease); }
}

