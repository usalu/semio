import { existsSync, lstatSync, readdirSync } from "node:fs";
import { join } from "node:path";

//#region 🗂️Discovery
export type OwnedFilesystemEntry = Readonly<{ nodeKind: "directory" | "file" | "symlink"; path: string }>;

export function ownedPathByteSort(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

export function ownedFilesystemEntries(root: string, includeRoot = false): OwnedFilesystemEntry[] {
  if (!existsSync(root)) return [];
  const entries: OwnedFilesystemEntry[] = [];
  const visit = (absolute: string, path: string, include: boolean): void => {
    const stat = lstatSync(absolute);
    const nodeKind = stat.isSymbolicLink() ? "symlink" : stat.isFile() ? "file" : stat.isDirectory() ? "directory" : undefined;
    if (!nodeKind) throw new Error(`Unsupported filesystem node: ${path || "."}`);
    if (include) entries.push({ nodeKind, path: path || "." });
    if (nodeKind !== "directory") return;
    for (const name of readdirSync(absolute).sort(ownedPathByteSort)) visit(join(absolute, name), path ? `${path}/${name}` : name, true);
  };
  visit(root, "", includeRoot);
  return entries.sort((left, right) => ownedPathByteSort(left.path, right.path));
}

export function ownedFilePaths(root: string): string[] {
  return ownedFilesystemEntries(root).filter(({ nodeKind }) => nodeKind === "file").map(({ path }) => path);
}
//#endregion 🗂️Discovery
