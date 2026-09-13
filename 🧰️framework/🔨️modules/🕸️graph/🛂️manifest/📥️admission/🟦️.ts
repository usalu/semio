/** 📥️ No-follow discovery and parsing for graph manifest inputs. */
import { lstatSync, readdirSync, readFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import { loadTaxonomy, pathIsExcluded } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

export type ManifestAxes = { portModel?: string; directedness?: string };
export type ManifestPropertyDef = { name: string; kind: "data" | "derived"; valueType?: unknown; expr?: string };
export type ManifestKindRow = { id: string; name?: string; properties?: ManifestPropertyDef[]; ports?: string[]; presentation?: Record<string, unknown> };
export type ManifestDocument = {
  schema: string; id: string; name?: string; axes?: ManifestAxes;
  nodeKinds?: ManifestKindRow[]; edgeKinds?: ManifestKindRow[]; portKinds?: ManifestKindRow[];
  wireKinds?: ManifestKindRow[]; layerKinds?: ManifestKindRow[]; languageKinds?: ManifestKindRow[];
  surfaceKinds?: ManifestKindRow[]; windowKinds?: ManifestKindRow[]; fileNodeKinds?: ManifestKindRow[];
  descriptorKinds?: ManifestKindRow[]; edgeTips?: Record<string, unknown>[]; kindCompatibility?: Record<string, unknown>[];
};

type FileStatus = ReturnType<typeof lstatSync>;

function inspect(path: string, root: string): FileStatus | undefined {
  try {
    return lstatSync(path);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw new Error(`cannot inspect admitted graph manifest input ${relative(root, path)}`, { cause: error });
  }
}

function admittedDirectory(root: string, area: string): string | undefined {
  const absoluteRoot = resolve(root);
  const scanRoot = resolve(root, area);
  const relation = relative(absoluteRoot, scanRoot);
  if (!relation || relation === ".." || relation.startsWith(`..${sep}`) || isAbsolute(relation)) throw new Error(`graph manifest area is outside its root: ${area}`);
  let current = absoluteRoot;
  for (const segment of ["", ...relation.split(sep)]) {
    if (segment) current = join(current, segment);
    const status = inspect(current, absoluteRoot);
    if (!status) return undefined;
    if (status.isSymbolicLink()) throw new Error(`admitted graph manifest ancestor is a symbolic link: ${relative(absoluteRoot, current) || "."}`);
    if (!status.isDirectory()) throw new Error(`admitted graph manifest ancestor is not a directory: ${relative(absoluteRoot, current) || "."}`);
  }
  return scanRoot;
}

export function findManifestFiles(root: string, pluginAreas: readonly string[] = loadTaxonomy().pluginAreas): string[] {
  const out: string[] = [];
  const taxonomy = loadTaxonomy();
  function walk(dir: string): void {
    let names: string[];
    try {
      names = readdirSync(dir);
    } catch (error) {
      throw new Error(`cannot read admitted graph manifest directory ${relative(root, dir)}`, { cause: error });
    }
    for (const name of names) {
      if (name === "node_modules" || name === "generated" || name === "🤖️generated" || name === "target" || name.startsWith(".")) continue;
      const path = join(dir, name);
      if (pathIsExcluded(root, path, taxonomy)) continue;
      const status = inspect(path, root);
      if (!status) throw new Error(`admitted graph manifest input disappeared: ${relative(root, path)}`);
      if (status.isSymbolicLink()) throw new Error(`admitted graph manifest input is a symbolic link: ${relative(root, path)}`);
      if (status.isDirectory()) walk(path);
      else if (name.endsWith("manifest.json")) out.push(path);
    }
  }
  for (const area of pluginAreas) {
    const scanRoot = admittedDirectory(root, area);
    if (scanRoot && !pathIsExcluded(root, scanRoot, taxonomy)) walk(scanRoot);
  }
  return out.sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
}

export function readGraphManifestDocuments(
  root: string,
  log = true,
  pluginAreas?: readonly string[],
  read: (path: string) => string = (path) => readFileSync(path, "utf8"),
): readonly ManifestDocument[] {
  const files = findManifestFiles(root, pluginAreas);
  if (files.length === 0) throw new Error("no *.manifest.json files found");
  const docs: ManifestDocument[] = [];
  for (const path of files) {
    let doc: ManifestDocument;
    try {
      doc = JSON.parse(read(path)) as ManifestDocument;
    } catch (error) {
      throw new Error(`cannot read admitted graph manifest ${relative(root, path)}`, { cause: error });
    }
    if (doc.schema !== "manifest") {
      if (log) console.log(`[framework-graph] skip ${relative(root, path)} (${doc.schema})`);
      continue;
    }
    if (log) console.log(`[framework-graph] ${relative(root, path)}`);
    docs.push(doc);
  }
  if (docs.length === 0) throw new Error("no graph manifest documents found");
  return docs;
}
