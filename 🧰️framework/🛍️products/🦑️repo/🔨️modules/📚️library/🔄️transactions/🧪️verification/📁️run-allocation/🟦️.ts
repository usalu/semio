import { lstatSync, mkdirSync } from "node:fs";
import { isAbsolute, join, parse, relative, resolve, sep } from "node:path";

export const TRANSACTION_V2_RUN_OWNER_DIRECTORY = "🧾️runs";

/** 🧫️ Allocates one exclusive no-follow semantic run owner and its bundle directory. */
export function transactionV2BundleRoot(artifactRoot: string, runId: string): string {
  const identity = /^[1-9][0-9]*-([0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12})$/u.exec(runId);
  if (!identity || !isAbsolute(artifactRoot) || resolve(artifactRoot) !== artifactRoot) throw new Error("Invalid transaction run allocation identity");
  const owner = join(artifactRoot, TRANSACTION_V2_RUN_OWNER_DIRECTORY);
  let path = parse(owner).root;
  for (const part of relative(path, owner).split(sep)) {
    path = join(path, part);
    let stat;
    try {
      stat = lstatSync(path);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || path !== owner) throw error;
      mkdirSync(path);
      stat = lstatSync(path);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction run ancestor must be a no-follow directory: " + path);
  }
  const root = join(owner, "🔖️" + identity[1]);
  mkdirSync(root);
  const bundle = join(root, "📦️bundle");
  mkdirSync(bundle);
  return bundle;
}
