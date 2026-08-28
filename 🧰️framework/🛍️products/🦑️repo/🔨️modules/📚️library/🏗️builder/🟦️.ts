import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, fsyncSync, lstatSync, mkdirSync, openSync, readSync, writeFileSync, type BigIntStats, type Stats } from "node:fs";
import { basename, dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { canonicalPrimaryFilenameForKind, generatorContractIdsForOutputPath, semanticDirectoryKindId, taxonomyRelativePathIsExcluded, type Taxonomy } from "../🔍️discovery/🟦️component.ts";

export type ArtifactScaffoldOwner = Readonly<{ kind: "subset"; subsetPath: string } | { kind: "surface"; subsetPath: string; role: string }>;
export type ArtifactScaffoldLeaf = Readonly<{ path: string; content: string }>;
export type ArtifactScaffoldResult = { created: string[]; skipped: string[] };
export type ArtifactScaffoldProgress = Readonly<{ phase: "preflight" | "reading" | "before-create" | "created" | "skipped" | "complete"; path?: string; bytesRead?: number; current: number; total: number }>;
export type ArtifactScaffoldOptions = Readonly<{ dryRun?: boolean; cancelled?: () => boolean; progress?: (event: ArtifactScaffoldProgress) => void }>;
export type ArtifactScaffoldDirectory = Readonly<{ path: string; device: number; inode: number; mode: number }>;
export type ArtifactScaffoldFile = ArtifactScaffoldDirectory & Readonly<{ bytes: number; sha256: string | null }>;
export type ArtifactScaffoldPartial = Readonly<{ created: readonly ArtifactScaffoldFile[]; skipped: readonly string[]; directories: readonly ArtifactScaffoldDirectory[]; failedPath: string | null }>;

/** 🧾️ Retains exact observed publication evidence without claiming all-or-nothing rollback. */
export class ArtifactScaffoldError extends Error {
  readonly partial: ArtifactScaffoldPartial;
  constructor(cause: unknown, partial: ArtifactScaffoldPartial) {
    super(`Artifact scaffold authoring failed: ${cause instanceof Error ? cause.message : String(cause)}`, { cause });
    this.name = "ArtifactScaffoldError";
    this.partial = Object.freeze({ ...partial, created: Object.freeze(partial.created.map((entry) => Object.freeze({ ...entry }))), skipped: Object.freeze([...partial.skipped]), directories: Object.freeze(partial.directories.map((entry) => Object.freeze({ ...entry }))) });
  }
}

function scaffoldStat(path: string): Stats | null {
  try { return lstatSync(path); }
  catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
}

function scaffoldCoordinate(path: string, taxonomy: Taxonomy): string[] {
  const parts = path.split("/");
  if (!path || path !== path.normalize(taxonomy.unicodeNormalization.form) || parts.some((part) => !part || part === "." || part === ".." || /[\\\0:]/u.test(part)) || taxonomyRelativePathIsExcluded(path, taxonomy)) throw new Error(`Invalid authoring coordinate: ${JSON.stringify(path)}`);
  return parts;
}

function scaffoldKind(name: string, parentKindId: string, taxonomy: Taxonomy): string {
  const kind = semanticDirectoryKindId(name, taxonomy, { parentKindId });
  if (!kind) throw new Error(`Unregistered authoring directory: ${JSON.stringify(name)}`);
  const spec = taxonomy.semanticDirectoryKinds[kind], members = taxonomy.semanticDirectoryMemberKinds[kind];
  if (spec && !name.startsWith(spec.emoji) || !spec && !members?.memberNames.includes(name)) throw new Error(`Unregistered authoring directory: ${JSON.stringify(name)}`);
  return kind;
}

function scaffoldOwner(owner: ArtifactScaffoldOwner, taxonomy: Taxonomy): { path: string; kind: string; required: string } {
  const contract = taxonomy.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  if (contract?.contractKind !== "semantic-facet-primary-file" || contract.sourceDisposition !== "authored" || contract.authoringCommand.writeDisposition !== "create-if-absent") throw new Error("The authored empty-facet authority is required");
  const root = scaffoldCoordinate(contract.sourceRoot, taxonomy), parts = scaffoldCoordinate(owner.subsetPath, taxonomy);
  if (root.some((part, index) => parts[index] !== part) || parts.length !== root.length + 7) throw new Error("Authoring owner must be an exact artifact standard/subset path");
  const captures = ["plugin", null, "artifact", null, "standard", null, "subset"], names = [null, taxonomy.artifactsDirName, null, taxonomy.standardsDirName, null, taxonomy.subsetsDirName, null];
  let kind = "plugins";
  for (const [index, name] of parts.slice(root.length).entries()) {
    kind = scaffoldKind(name, kind, taxonomy);
    const capture = captures[index], rule = capture ? contract.directoryCaptures[capture] : null;
    if (capture ? !rule?.kindIds.includes(kind) || rule.names && !rule.names.includes(name) : name !== names[index]) throw new Error(`Wrong structural authoring owner at ${JSON.stringify(name)}`);
  }
  if (owner.kind === "subset") return { path: owner.subsetPath, kind, required: parts.slice(0, -2).join("/") };
  if (owner.kind !== "surface" || !taxonomy.surfaceRoles.includes(owner.role)) throw new Error("Unknown artifact surface role");
  const surface = taxonomy.surfaceDirNames[owner.role];
  if (!surface || !contract.directoryCaptures.surface?.names?.includes(surface)) throw new Error("Surface role lacks exact schema ownership");
  const schemaDirs = taxonomy.subsetComponentDirs.filter((name) => semanticDirectoryKindId(name, taxonomy, { parentKindId: kind }) === "schema");
  if (schemaDirs.length !== 1) throw new Error("Surface owner requires one registered schema facet");
  return { path: `${owner.subsetPath}/${surface}`, kind: scaffoldKind(surface, kind, taxonomy), required: `${owner.subsetPath}/${schemaDirs[0]}` };
}

function scaffoldLeafOwner(path: string, authority: { path: string; kind: string }, owner: ArtifactScaffoldOwner, taxonomy: Taxonomy): void {
  const contract = taxonomy.semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"];
  if (contract?.contractKind !== "semantic-facet-primary-file") throw new Error("The authored empty-facet authority is required");
  let state: string = owner.kind, kind = authority.kind;
  for (const name of path.slice(authority.path.length + 1).split("/").slice(0, -1)) {
    const childKind = scaffoldKind(name, kind, taxonomy);
    const capture = state === "modes" ? contract.directoryCaptures.mode : state === "windows" ? contract.directoryCaptures.window : null;
    const allowed = state === "subset" ? taxonomy.subsetChildDirs : state === "surface" ? taxonomy.surfaceRequiredChildDirs : state === "mode" ? taxonomy.modeRequiredChildDirs : state === "window" ? taxonomy.windowRequiredChildDirs : state === "io" ? taxonomy.ioSemanticCollectionDirNames : state === "collection" ? taxonomy.representationDirs : [];
    if (capture ? !capture.kindIds.includes(childKind) || capture.names && !capture.names.includes(name) : !allowed.includes(name)) throw new Error(`Unpermitted authoring child: ${name}`);
    if (state === "subset") state = taxonomy.subsetSurfaceDirs.includes(name) ? "surface" : taxonomy.subsetComponentDirs.includes(name) ? childKind === "io" ? "io" : "component" : "facet";
    else if (state === "surface") state = name === taxonomy.modesDirName ? "modes" : "facet";
    else if (state === "modes") state = "mode";
    else if (state === "mode") state = name === taxonomy.windowsDirName ? "windows" : "facet";
    else if (state === "windows") state = "window";
    else if (state === "window") state = "facet";
    else if (state === "io") state = "collection";
    else if (state === "collection") state = "component";
    kind = childKind;
  }
  const markdown = basename(path) === canonicalPrimaryFilenameForKind(taxonomy.windowEmptyFacetFileKindId, taxonomy);
  if (markdown ? !["facet", "collection"].includes(state) : !["subset", "surface", "mode", "window", "io", "component"].includes(state)) throw new Error(`Unowned authored leaf: ${path}`);
}

function scaffoldIdentity(path: string, stat: Stats | BigIntStats): ArtifactScaffoldDirectory {
  return { path, device: Number(stat.dev), inode: Number(stat.ino), mode: Number(stat.mode) & 0o7777 };
}

function scaffoldSameVersion(left: BigIntStats, right: BigIntStats): boolean {
  return (["dev", "ino", "mode", "size", "mtimeNs", "ctimeNs"] as const).every((key) => left[key] === right[key]);
}

function scaffoldFile(fd: number, path: string, progress: (bytesRead: number) => void): ArtifactScaffoldFile {
  const stat = fstatSync(fd, { bigint: true }), size = Number(stat.size);
  if (!stat.isFile()) throw new Error(`Authoring node is not a regular file: ${path}`);
  if (!Number.isSafeInteger(size)) throw new Error(`Authoring file is too large to inspect exactly: ${path}`);
  const digest = createHash("sha256"), buffer = Buffer.alloc(65536);
  for (let position = 0; position < size;) {
    const count = readSync(fd, buffer, 0, Math.min(buffer.length, size - position), position);
    if (!count) throw new Error(`Authoring file changed while reading: ${path}`);
    digest.update(buffer.subarray(0, count)); position += count; progress(position);
  }
  const after = fstatSync(fd, { bigint: true });
  if (!scaffoldSameVersion(stat, after)) throw new Error(`Authoring file changed while reading: ${path}`);
  return { ...scaffoldIdentity(path, stat), bytes: size, sha256: digest.digest("hex") };
}

function scaffoldExistingFile(absolute: string, path: string, progress: (bytesRead: number) => void): ArtifactScaffoldFile {
  const before = lstatSync(absolute, { bigint: true });
  if (!before.isFile() || before.isSymbolicLink()) throw new Error(`Authoring target is not a regular file: ${path}`);
  const fd = openSync(absolute, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    if (!scaffoldSameVersion(before, fstatSync(fd, { bigint: true }))) throw new Error(`Authoring target identity changed: ${path}`);
    const evidence = scaffoldFile(fd, path, progress), current = lstatSync(absolute, { bigint: true });
    if (!current.isFile() || current.isSymbolicLink() || !scaffoldSameVersion(before, current)) throw new Error(`Authoring target identity changed: ${path}`);
    return evidence;
  } finally { closeSync(fd); }
}

/** 🏗️ Preflights all targets and publishes exclusively under cooperative identity rechecks, retaining partial output on failure. */
export function authorArtifactScaffold(repoRoot: string, owner: ArtifactScaffoldOwner, leaves: readonly ArtifactScaffoldLeaf[], taxonomy: Taxonomy, options: ArtifactScaffoldOptions = {}): ArtifactScaffoldResult {
  const result: ArtifactScaffoldResult = { created: [], skipped: [] }, created: ArtifactScaffoldFile[] = [], directories: ArtifactScaffoldDirectory[] = [];
  const known = new Map<string, ArtifactScaffoldDirectory>(), existing = new Map<string, ArtifactScaffoldFile>();
  let failedPath: string | null = null;
  const cancel = (): void => { if (options.cancelled?.()) throw new Error("Authoring cancelled"); };
  const emit = (phase: ArtifactScaffoldProgress["phase"], path?: string, bytesRead?: number): void => { options.progress?.({ phase, path, ...(bytesRead === undefined ? {} : { bytesRead }), current: result.created.length + result.skipped.length, total: leaves.length }); cancel(); };
  const directory = (absolute: string, required: boolean): void => {
    const stat = scaffoldStat(absolute);
    if (!stat) { if (required) throw new Error(`Missing governing authoring directory: ${absolute}`); return; }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error(`Authoring ancestor is not a no-follow directory: ${absolute}`);
    const identity = scaffoldIdentity(absolute, stat), previous = known.get(absolute);
    if (previous && (previous.device !== identity.device || previous.inode !== identity.inode || previous.mode !== identity.mode)) throw new Error(`Authoring ancestor identity changed: ${absolute}`);
    known.set(absolute, identity);
  };
  const ancestry = (absolute: string, required: boolean): void => {
    let cursor = parse(absolute).root;
    directory(cursor, true);
    for (const part of relative(cursor, absolute).split(sep)) { cursor = join(cursor, part); directory(cursor, required); }
  };
  const recheck = (): void => { for (const path of known.keys()) directory(path, true); };
  try {
    cancel();
    if (!isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Authoring repository root must be an exact absolute directory");
    const authority = scaffoldOwner(owner, taxonomy), proposed = leaves.map((leaf) => ({ path: leaf.path, content: leaf.content }));
    const allowed = new Set([taxonomy.windowEmptyFacetFileKindId, ...Object.values(taxonomy.componentFileKinds)].map((kind) => canonicalPrimaryFilenameForKind(kind, taxonomy)));
    const targets = new Set<string>();
    ancestry(repoRoot, true);
    ancestry(join(repoRoot, authority.required), true);
    if (!proposed.length) throw new Error("Authoring request must contain leaves");
    for (const leaf of proposed) {
      failedPath = leaf.path;
      scaffoldCoordinate(leaf.path, taxonomy);
      if (typeof leaf.content !== "string" || !leaf.path.startsWith(`${authority.path}/`) || !allowed.has(basename(leaf.path)) || targets.has(leaf.path) || generatorContractIdsForOutputPath(leaf.path, taxonomy).length) throw new Error(`Invalid authored leaf request: ${leaf.path}`);
      targets.add(leaf.path);
      scaffoldLeafOwner(leaf.path, authority, owner, taxonomy);
      const absolute = join(repoRoot, leaf.path);
      ancestry(dirname(absolute), false);
      if (scaffoldStat(absolute)) existing.set(leaf.path, scaffoldExistingFile(absolute, leaf.path, (bytes) => emit("reading", leaf.path, bytes)));
      cancel();
    }
    failedPath = null;
    recheck(); emit("preflight"); recheck();
    for (const leaf of proposed) {
      failedPath = leaf.path; cancel();
      const absolute = join(repoRoot, leaf.path), previous = existing.get(leaf.path);
      recheck();
      if (previous) {
        if (JSON.stringify(scaffoldExistingFile(absolute, leaf.path, (bytes) => emit("reading", leaf.path, bytes))) !== JSON.stringify(previous)) throw new Error(`Existing authored leaf changed: ${leaf.path}`);
        result.skipped.push(leaf.path); emit("skipped", leaf.path); continue;
      }
      if (options.dryRun) { result.created.push(leaf.path); continue; }
      let cursor = repoRoot;
      for (const name of relative(repoRoot, dirname(absolute)).split(sep)) {
        cursor = join(cursor, name);
        recheck();
        if (!scaffoldStat(cursor)) {
          let owned = false;
          try { mkdirSync(cursor); owned = true; }
          catch (error) { if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error; }
          directory(cursor, true);
          if (owned) directories.push({ ...known.get(cursor)!, path: relative(repoRoot, cursor).replaceAll("\\", "/") });
        } else directory(cursor, true);
      }
      emit("before-create", leaf.path); recheck();
      let fd: number;
      try { fd = openSync(absolute, constants.O_RDWR | constants.O_CREAT | constants.O_EXCL | (constants.O_NOFOLLOW ?? 0), 0o666); }
      catch (error) {
        if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
        scaffoldExistingFile(absolute, leaf.path, (bytes) => emit("reading", leaf.path, bytes));
        result.skipped.push(leaf.path); emit("skipped", leaf.path); continue;
      }
      const index = created.push({ ...scaffoldIdentity(leaf.path, fstatSync(fd)), bytes: 0, sha256: createHash("sha256").digest("hex") }) - 1;
      try { writeFileSync(fd, leaf.content, "utf8"); fsyncSync(fd); }
      finally {
        try { created[index] = scaffoldFile(fd, leaf.path, (bytes) => emit("reading", leaf.path, bytes)); }
        catch (error) { const stat = fstatSync(fd); created[index] = { ...scaffoldIdentity(leaf.path, stat), bytes: stat.size, sha256: null }; throw error; }
        finally { closeSync(fd); }
      }
      const current = scaffoldStat(absolute), evidence = created[index]!;
      if (!current?.isFile() || current.isSymbolicLink() || current.dev !== evidence.device || current.ino !== evidence.inode) throw new Error(`Published authored leaf identity changed: ${leaf.path}`);
      result.created.push(leaf.path); emit("created", leaf.path);
    }
    failedPath = null;
    recheck(); emit("complete"); recheck();
    return result;
  } catch (error) { throw new ArtifactScaffoldError(error, { created, skipped: result.skipped, directories, failedPath }); }
}
