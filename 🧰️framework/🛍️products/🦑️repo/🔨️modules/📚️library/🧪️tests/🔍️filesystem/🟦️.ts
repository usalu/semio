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
  const visitDirectory = (absolute: string, path: string): void => {
    const children = readdirSync(absolute, { withFileTypes: true }).sort((left, right) => ownedPathByteSort(left.name, right.name));
    for (const child of children) {
      const childPath = path ? `${path}/${child.name}` : child.name;
      const nodeKind = child.isSymbolicLink() ? "symlink" : child.isFile() ? "file" : child.isDirectory() ? "directory" : undefined;
      if (!nodeKind) throw new Error(`Unsupported filesystem node: ${childPath}`);
      entries.push({ nodeKind, path: childPath });
      if (nodeKind === "directory") visitDirectory(join(absolute, child.name), childPath);
    }
  };
  const rootStat = lstatSync(root);
  const rootKind = rootStat.isSymbolicLink() ? "symlink" : rootStat.isFile() ? "file" : rootStat.isDirectory() ? "directory" : undefined;
  if (!rootKind) throw new Error("Unsupported filesystem node: .");
  if (includeRoot) entries.push({ nodeKind: rootKind, path: "." });
  if (rootKind === "directory") visitDirectory(root, "");
  return entries;
}

export function ownedFilePaths(root: string): string[] {
  return ownedFilesystemEntries(root).filter(({ nodeKind }) => nodeKind === "file").map(({ path }) => path);
}
//#endregion 🗂️Discovery
