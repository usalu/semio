/** 🏗️ Canonical 🚪️source-admission source service. */
import { resolve, relative, isAbsolute, join, sep, basename, parse } from "node:path";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { execFileSync } from "node:child_process";
import { type Stats, readdirSync, lstatSync } from "node:fs";
import { semanticOwnedInputFileSnapshot } from "../../../🔍️discovery/🟦️.ts";
import { loadTaxonomy, TAXONOMY_RELATIVE_PATH, type LoadedTaxonomy } from "../../🔣️taxonomy/🟦️.ts";
import { type TaxonomySourceAdmission, taxonomyScopedGitPathspec, sourceAdmissionRepositoryFences, sourceAdmissionAssertRepositoryPath, sourceAdmissionOpaque, sourceAdmissionSafePath, type TaxonomySourceOrigin, type TaxonomySourceIndexEntry, type TaxonomySourceCandidateObservation, SOURCE_ADMISSION_ORIGINS, type TaxonomySourceAdmissionInput, projectTaxonomySourceAdmission, type TaxonomyScopedGitPathspec, sourceAdmissionByteCompare, sourceAdmissionContainingRepository } from "../🟦️.ts";
import { report, TaxonomyCancellationError, type TaxonomyProgress } from "../../🏃️operation/🟦️.ts";
import { inScope } from "../../🛣️path/🟦️.ts";
import { sha256, lstatOrNull, noFollowDirectoryChain, verifyNoFollowDirectoryChain, UnsafeDirectoryAncestorError } from "../../📁️input/🟦️.ts";

export interface TaxonomySourceInventoryOptions { readonly repoRoot: string; readonly scope?: string; readonly ticketDir?: string; readonly cancelFile?: string; readonly structuralDirectoryNames?: readonly string[]; readonly progress?: (progress: TaxonomyProgress) => void; readonly taxonomyPath?: string }

export interface TaxonomySourceInventory extends TaxonomySourceAdmission {
  readonly repoRoot: string;
  readonly taxonomyPath: string;
  readonly taxonomyContentHash: string;
  readonly membershipDigest: string;
}

interface SourceAdmissionPreparedOptions {
  readonly repoRoot: string;
  readonly scope?: string;
  readonly taxonomyPath: string;
  readonly ticketDir?: string;
  readonly cancelFile?: string;
  readonly indexRows: readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[];
  readonly repositoryFences: readonly string[];
  readonly structuralDirectoryNames?: readonly string[];
  readonly progress?: (progress: TaxonomyProgress) => void;
}

export function sourceAdmissionAssertLexical(value: string, label: string, allowAbsolute: boolean): void {
  if (typeof value !== "string" || !value || /[\u0000-\u001f\u007f]/u.test(value) || Buffer.from(value).toString("utf8") !== value) throw new Error(`${label} is not a lossless path`);
  if (value.replaceAll("\\", "/").split("/").some((segment) => segment.toLowerCase() === "compose")) throw new Error(`${label} is opaque`);
  if (!allowAbsolute && !sourceAdmissionSafePath(value)) throw new Error(`${label} is not a safe repository-relative path`);
  if (!allowAbsolute) return;
  const nativeRoot = isAbsolute(value) ? parse(value).root : "";
  const tail = value.slice(nativeRoot.length).split(sep).join("/");
  if ((tail && !sourceAdmissionSafePath(tail)) || (!nativeRoot && !tail)) throw new Error(`${label} has ambiguous or escaping path segments`);
}

function sourceAdmissionLstat(repoRoot: string, path: string): Stats | null {
  sourceAdmissionAssertLexical(path, "Source admission candidate", false);
  const ancestors = [...noFollowDirectoryChain(repoRoot)];
  const segments = path.split("/");
  let absolute = repoRoot;
  for (let index = 0; index + 1 < segments.length; index++) {
    absolute = join(absolute, segments[index]);
    const stat = lstatOrNull(absolute);
    if (!stat) return null;
    if (stat.isSymbolicLink() || !stat.isDirectory()) throw new UnsafeDirectoryAncestorError(`Source admission candidate has unsafe ancestry: ${path}`);
    ancestors.push({ path: absolute, stat });
  }
  const observed = lstatOrNull(join(absolute, segments[segments.length - 1]));
  verifyNoFollowDirectoryChain(ancestors);
  return observed;
}

export function sourceAdmissionPrepareOptions(options: TaxonomySourceInventoryOptions): SourceAdmissionPreparedOptions {
  const request = { ...options };
  options = Object.freeze({ ...request, structuralDirectoryNames: request.structuralDirectoryNames && Object.freeze([...request.structuralDirectoryNames]) });
  if (options.repoRoot !== ".") sourceAdmissionAssertLexical(options.repoRoot, "repoRoot", true);
  if (options.scope !== undefined) sourceAdmissionAssertLexical(options.scope, "scope", false);
  for (const [label, value] of [["ticketDir", options.ticketDir], ["taxonomyPath", options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH], ["cancelFile", options.cancelFile]] as const) if (value !== undefined) sourceAdmissionAssertLexical(value, label, true);
  if (options.structuralDirectoryNames) {
    if (new Set(options.structuralDirectoryNames).size !== options.structuralDirectoryNames.length) throw Error("Source admission structural directory names must be unique safe segments");
    for (const name of options.structuralDirectoryNames) {
      sourceAdmissionAssertLexical(name, "structuralDirectoryNames", false);
      if (name.includes("/")) throw Error("Source admission structural directory names must be unique safe segments");
    }
  }
  const repoRoot = resolve(options.repoRoot);
  sourceAdmissionAssertLexical(repoRoot, "repoRoot", true);
  const local = (value: string, label: string): string => {
    const path = relative(repoRoot, isAbsolute(value) ? value : join(repoRoot, value)).split(sep).join("/");
    sourceAdmissionAssertLexical(path, label, false);
    return path;
  };
  const taxonomyPath = local(options.taxonomyPath ?? TAXONOMY_RELATIVE_PATH, "taxonomyPath");
  const ticketDir = options.ticketDir === undefined ? undefined : local(options.ticketDir, "ticketDir");
  const cancelFile = options.cancelFile === undefined ? undefined : local(options.cancelFile, "cancelFile");
  noFollowDirectoryChain(repoRoot);
  report(options.progress, "inventory", "tracked-enumeration", 0, 1, options.scope);
  const indexRows = Object.freeze(sourceAdmissionGitRows(repoRoot, taxonomyScopedGitPathspec(undefined, ["compose"])).map(row => Object.freeze({ ...row, entry: Object.freeze({ ...row.entry }) })));
  const repositoryFences = Object.freeze([...sourceAdmissionRepositoryFences(indexRows)]);
  if (options.scope !== undefined) sourceAdmissionAssertRepositoryPath(options.scope, repositoryFences, "Source admission scope", true);
  if (ticketDir !== undefined) sourceAdmissionAssertRepositoryPath(ticketDir, repositoryFences, "Source admission ticket", true);
  sourceAdmissionAssertRepositoryPath(taxonomyPath, repositoryFences, "Source admission taxonomy", false);
  if (cancelFile !== undefined) sourceAdmissionAssertRepositoryPath(cancelFile, repositoryFences, "Source admission cancellation", false);
  report(options.progress, "inventory", "tracked-enumeration", 1, 1, options.scope);
  const schema = sourceAdmissionLstat(repoRoot, taxonomyPath);
  if (!schema?.isFile() || schema.isSymbolicLink()) throw new Error("Taxonomy schema is not a no-follow regular file");
  return Object.freeze({ repoRoot, scope: options.scope?.normalize("NFC"), taxonomyPath: join(repoRoot, ...taxonomyPath.split("/")), ticketDir, cancelFile, indexRows, repositoryFences, structuralDirectoryNames: options.structuralDirectoryNames, progress: options.progress });
}

export function sourceAdmissionCheckCancellation(repoRoot: string, cancelFile: string | undefined, repositoryFences: readonly string[]): void {
  if (!cancelFile) return;
  sourceAdmissionAssertRepositoryPath(cancelFile, repositoryFences, "Source admission cancellation", false);
  const stat = sourceAdmissionLstat(repoRoot, cancelFile);
  if (stat?.isSymbolicLink()) throw new Error("Source admission cancellation path is a symlink");
  if (stat) throw new TaxonomyCancellationError();
}

function sourceAdmissionGitRecords(bytes: Uint8Array, label: string): readonly string[] {
  if (bytes.length === 0) return [];
  if (bytes[bytes.length - 1] !== 0) throw new Error(`${label} is missing its terminal NUL`);
  const rows = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes).slice(0, -1).split("\0");
  if (rows.some((row) => !row)) throw new Error(`${label} contains an empty record`);
  return rows;
}

function sourceAdmissionGitExclusions(pathspec: TaxonomyScopedGitPathspec): readonly string[] {
  return [...pathspec.exclusionPathspecs, ":(exclude,icase,glob)**/compose", ":(exclude,icase,glob)**/compose/**"];
}

type SourceAdmissionIndexRows = readonly { readonly path: string; readonly entry: TaxonomySourceIndexEntry }[];

const sourceAdmissionIndexObservations = new Map<string, { readonly receipt: string; readonly rows: SourceAdmissionIndexRows }>();

function sourceAdmissionIndexObservation(repoRoot: string): string | undefined {
  if (Object.keys(process.env).some((key) => key.startsWith("GIT_") && key !== "GIT_PAGER")) return undefined;
  const directory = lstatOrNull(join(repoRoot, ".git"));
  if (!directory?.isDirectory() || directory.isSymbolicLink()) return undefined;
  if (readdirSync(join(repoRoot, ".git")).some((name) => name === "commondir" || name.startsWith("sharedindex."))) return undefined;
  try {
    const paths = [".git/index", ".git/config", ".git/HEAD", ".git/packed-refs", ".git/config.worktree", ".git/objects/info/alternates"];
    const snapshots = paths.map((path) => semanticOwnedInputFileSnapshot(repoRoot, path, { maximumBytes: 64 * 1024 * 1024 }));
    const index = snapshots[0], config = snapshots[1], head = snapshots[2];
    if (!index || !config || !head || snapshots[5]) return undefined;
    const indexBytes = Buffer.from(index.bytes.buffer, index.bytes.byteOffset, index.bytes.byteLength);
    if (indexBytes.subarray(0, 4).toString() !== "DIRC" || indexBytes.includes(Buffer.from("sdir")) || indexBytes.includes(Buffer.from("link"))) return undefined;
    if (/^\s*\[\s*include(?:if)?\b/imu.test(Buffer.from(config.bytes.buffer, config.bytes.byteOffset, config.bytes.byteLength).toString("utf8"))) return undefined;
    const reference = /^ref: (refs\/[A-Za-z0-9_./-]+)\n$/u.exec(Buffer.from(head.bytes.buffer, head.bytes.byteOffset, head.bytes.byteLength).toString("utf8"));
    if (reference) snapshots.push(semanticOwnedInputFileSnapshot(repoRoot, ".git/" + reference[1], { maximumBytes: 64 * 1024 }));
    const after = lstatSync(join(repoRoot, ".git"));
    if (!after.isDirectory() || after.isSymbolicLink() || after.dev !== directory.dev || after.ino !== directory.ino) return undefined;
    return sha256(canonicalJson({ directory: { dev: directory.dev, ino: directory.ino }, files: snapshots.map((row) => row ? { path: row.path, contentHash: row.contentHash, size: row.size } : null) }));
  } catch { return undefined; }
}

export function sourceAdmissionGitRows(repoRoot: string, pathspec: TaxonomyScopedGitPathspec): SourceAdmissionIndexRows {
  const key = canonicalJson({ repoRoot, pathspec }), receipt = sourceAdmissionIndexObservation(repoRoot), cached = sourceAdmissionIndexObservations.get(key);
  if (receipt && cached?.receipt === receipt) return cached.rows;
  const bytes = execFileSync("git", ["ls-files", "--stage", "-z", "--", pathspec.positivePathspec, ...sourceAdmissionGitExclusions(pathspec)], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  const rows = sourceAdmissionGitRecords(bytes, "Git stage output").map((row) => {
    const tab = row.indexOf("\t"), match = /^(100644|100755|120000|160000) ([0-9a-f]{40}|[0-9a-f]{64}) ([0-3])$/u.exec(row.slice(0, tab));
    const path = row.slice(tab + 1);
    if (tab < 1 || !match || !sourceAdmissionSafePath(path)) throw new Error("Git stage output has an invalid header or source path");
    return { path, entry: { mode: match[1], objectId: match[2], stage: Number(match[3]) } };
  });
  if (receipt && sourceAdmissionIndexObservation(repoRoot) === receipt) {
    if (sourceAdmissionIndexObservations.size >= 64) sourceAdmissionIndexObservations.delete(sourceAdmissionIndexObservations.keys().next().value!);
    sourceAdmissionIndexObservations.set(key, { receipt, rows: Object.freeze(rows.map((row) => Object.freeze({ ...row, entry: Object.freeze({ ...row.entry }) }))) });
  } else sourceAdmissionIndexObservations.delete(key);
  return rows;
}

function sourceAdmissionUntrackedRows(repoRoot: string, pathspec: TaxonomyScopedGitPathspec, taxonomy: LoadedTaxonomy, repositoryFences: readonly string[]): readonly { readonly path: string; readonly directoryMarker: boolean }[] {
  const literal = (path: string): string => path.replace(/[\\*?\[\]#! ]/gu, "\\$&");
  const exclusions = [...taxonomy.exclusions.map((entry) => entry.path), ...repositoryFences].map((path) => `--exclude=/${literal(path)}`);
  const boundaries = repositoryFences.map((path) => `:(exclude,top,literal)${path}`);
  const bytes = execFileSync("git", ["ls-files", "--others", "--exclude-standard", "--exclude=[cC][oO][mM][pP][oO][sS][eE]", ...exclusions, "-z", "--", pathspec.positivePathspec, ...sourceAdmissionGitExclusions(pathspec), ...boundaries], { cwd: repoRoot, encoding: "buffer", maxBuffer: 256 * 1024 * 1024 });
  return sourceAdmissionGitRecords(bytes, "Git untracked output").map((record) => {
    const directoryMarker = record.endsWith("/"), path = directoryMarker ? record.slice(0, -1) : record;
    if (!sourceAdmissionSafePath(path)) throw new Error("Git untracked output has an invalid source path");
    return { path, directoryMarker };
  }).sort((left, right) => sourceAdmissionByteCompare(left.path, right.path));
}

function sourceAdmissionWalk(repoRoot: string, root: string, taxonomy: LoadedTaxonomy, scope: string | undefined, cancelFile: string | undefined, repositoryFences: readonly string[]): readonly string[] {
  const rows: string[] = [];
  const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
  const visit = (path: string): void => {
    if (!sourceAdmissionSafePath(path)) throw new Error(`Source admission walk has an invalid path: ${path}`);
    if (sourceAdmissionOpaque(path, opaquePrefixes) || !inScope(path, scope)) return;
    sourceAdmissionAssertRepositoryPath(path, repositoryFences, "Source admission walk", true);
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
    let stat: Stats | null;
    try { stat = sourceAdmissionLstat(repoRoot, path); }
    catch (error) {
      if (!(error instanceof UnsafeDirectoryAncestorError)) throw error;
      rows.push(path);
      return;
    }
    if (!stat) return;
    rows.push(path);
    if (!stat.isDirectory() || stat.isSymbolicLink()) return;
    if (sourceAdmissionContainingRepository(path, repositoryFences, true) !== null) return;
    const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
    if (nestedGit && basename(path) === ".git" && taxonomy.pathMatcher.matches(path, nestedGit.pathPattern)) return;
    const children = readdirSync(join(repoRoot, ...path.split("/")), { encoding: "buffer" }).map((name) => new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(name));
    const current = sourceAdmissionLstat(repoRoot, path);
    if (!current?.isDirectory() || current.isSymbolicLink() || current.dev !== stat.dev || current.ino !== stat.ino || current.mode !== stat.mode || current.mtimeMs !== stat.mtimeMs || current.ctimeMs !== stat.ctimeMs) throw new Error(`Source admission directory changed during enumeration: ${path}`);
    for (const child of children.sort(sourceAdmissionByteCompare)) visit(`${path}/${child}`);
  };
  visit(root);
  return rows.sort(sourceAdmissionByteCompare);
}

function sourceAdmissionStructuralDirectories(repoRoot: string, names: readonly string[], taxonomy: LoadedTaxonomy, scope: string | undefined, cancelFile: string | undefined, repositoryFences: readonly string[]): readonly string[] {
  const expected = new Set(names);
  if (expected.size !== names.length || names.some((name) => !sourceAdmissionSafePath(name) || name.includes("/"))) throw new Error("Source admission structural directory names must be unique safe segments");
  const found: string[] = [];
  const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
  const visit = (path: string | null, stat: Stats): void => {
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
    const absolute = path === null ? repoRoot : join(repoRoot, ...path.split("/"));
    const names = readdirSync(absolute, { encoding: "buffer" }).map((name) => new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(name));
    const dirents = new Map(readdirSync(absolute, { withFileTypes: true }).map((entry) => [entry.name, entry]));
    if (names.length !== dirents.size || names.some((name) => !dirents.has(name))) throw new Error(`Source admission structural directory names changed during enumeration: ${path ?? "."}`);
    const entries = names.map((name) => ({ name, directory: dirents.get(name)!.isDirectory(), symlink: dirents.get(name)!.isSymbolicLink() })).sort((left, right) => sourceAdmissionByteCompare(left.name, right.name));
    const current = path === null ? lstatSync(repoRoot) : sourceAdmissionLstat(repoRoot, path);
    if (!current?.isDirectory() || current.isSymbolicLink() || current.dev !== stat.dev || current.ino !== stat.ino || current.mode !== stat.mode || current.mtimeMs !== stat.mtimeMs || current.ctimeMs !== stat.ctimeMs) throw new Error(`Source admission structural directory changed during enumeration: ${path ?? "."}`);
    for (const entry of entries) {
      if (!entry.directory && !entry.symlink) continue;
      const child = path === null ? entry.name : `${path}/${entry.name}`;
      if (!sourceAdmissionSafePath(child)) throw new Error(`Source admission structural directory has an invalid path: ${child}`);
      if (sourceAdmissionOpaque(child, opaquePrefixes) || !inScope(child, scope)) continue;
      sourceAdmissionAssertRepositoryPath(child, repositoryFences, "Source admission structural directory", true);
      if (entry.symlink) { if (expected.has(entry.name)) found.push(child); continue; }
      const childStat = sourceAdmissionLstat(repoRoot, child);
      if (!childStat) continue;
      if (expected.has(entry.name)) found.push(child);
      if (!childStat.isDirectory() || childStat.isSymbolicLink() || sourceAdmissionContainingRepository(child, repositoryFences, true) !== null) continue;
      const nestedGit = taxonomy.schema.fixedDirectoryContracts["nested-git-metadata"];
      if (nestedGit && entry.name === ".git" && taxonomy.pathMatcher.matches(child, nestedGit.pathPattern)) continue;
      visit(child, childStat);
    }
  };
  visit(null, noFollowDirectoryChain(repoRoot).at(-1)!.stat);
  return found.sort(sourceAdmissionByteCompare);
}

function sourceAdmissionObservation(repoRoot: string, path: string, origins: readonly TaxonomySourceOrigin[], indexEntries: readonly TaxonomySourceIndexEntry[]): TaxonomySourceCandidateObservation {
  try {
    const stat = sourceAdmissionLstat(repoRoot, path);
    if (!stat) return { sourcePath: path, observedKind: "absent", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    if (stat.isSymbolicLink()) return { sourcePath: path, observedKind: "symlink", worktreeMode: "120000", explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    if (stat.isDirectory()) return { sourcePath: path, observedKind: "directory", worktreeMode: "040000", explicitDirectory: true, origins, indexEntries, unsafeAncestor: false };
    if (stat.isFile()) return { sourcePath: path, observedKind: "file", worktreeMode: (stat.mode & 0o111) !== 0 ? "100755" : "100644", explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
    return { sourcePath: path, observedKind: "other", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: false };
  } catch (error) {
    if (!(error instanceof UnsafeDirectoryAncestorError)) throw error;
    return { sourcePath: path, observedKind: "unobserved", worktreeMode: null, explicitDirectory: false, origins, indexEntries, unsafeAncestor: true };
  }
}

/** 🧭️ Collects source admission without reading admitted leaf content. */
export function collectTaxonomySourceAdmission(taxonomy: LoadedTaxonomy, prepared: SourceAdmissionPreparedOptions): CollectedTaxonomySourceAdmission {
  const { repoRoot, scope, cancelFile, indexRows, repositoryFences } = prepared;
  if (taxonomy.path !== prepared.taxonomyPath || !taxonomy.input) throw new Error("Source admission requires the exact loaded taxonomy input");
  const opaquePrefixes = ["compose", ...taxonomy.exclusions.map((entry) => entry.path)];
  if (scope && sourceAdmissionOpaque(scope, opaquePrefixes)) throw new Error(`Source admission scope is opaque: ${scope}`);
  for (const prefix of opaquePrefixes) if (!sourceAdmissionSafePath(prefix)) throw new Error("Source admission has an invalid opaque prefix");
  const generatorOutputRoots = Object.entries(taxonomy.schema.generatorContracts).flatMap(([contractId, contract]) => contract.outputRoots.map((root) => ({ contractId, rootPath: root.path, inclusion: root.inclusion === "ignored" ? "ignored" as const : "tracked" as const })));
  for (const output of generatorOutputRoots) {
    if (!sourceAdmissionSafePath(output.rootPath)) throw new Error("Source admission has an invalid generator output root");
    sourceAdmissionAssertRepositoryPath(output.rootPath, repositoryFences, "Source admission generator output", true);
  }
  const pathspec = taxonomyScopedGitPathspec(scope, opaquePrefixes);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  const rows = new Map<string, { origins: Set<TaxonomySourceOrigin>; indexEntries: TaxonomySourceIndexEntry[]; directoryMarker: boolean }>();
  const add = (path: string, origin: TaxonomySourceOrigin, entry?: TaxonomySourceIndexEntry, directoryMarker = false): void => {
    if (!sourceAdmissionSafePath(path)) throw new Error(`Source admission has an invalid candidate: ${path}`);
    if (sourceAdmissionOpaque(path, opaquePrefixes) || !inScope(path, scope)) return;
    sourceAdmissionAssertRepositoryPath(path, repositoryFences, "Source admission candidate", true);
    const row = rows.get(path) ?? { origins: new Set<TaxonomySourceOrigin>(), indexEntries: [], directoryMarker: false };
    row.directoryMarker ||= directoryMarker;
    row.origins.add(origin); if (entry) row.indexEntries.push(entry); rows.set(path, row);
  };
  for (const row of indexRows) add(row.path, "tracked", row.entry);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  report(prepared.progress, "inventory", "untracked-enumeration", 0, 1, scope);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  for (const row of sourceAdmissionUntrackedRows(repoRoot, pathspec, taxonomy, repositoryFences)) add(row.path, "nonignored-untracked", undefined, row.directoryMarker);
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(prepared.progress, "inventory", "untracked-enumeration", 1, 1, scope);
  if (prepared.structuralDirectoryNames?.length) {
    report(prepared.progress, "inventory", "structural-directory-admission", 0, 1, scope);
    for (const path of sourceAdmissionStructuralDirectories(repoRoot, prepared.structuralDirectoryNames, taxonomy, scope, cancelFile, repositoryFences)) add(path, "nonignored-untracked", undefined, true);
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(prepared.progress, "inventory", "structural-directory-admission", 1, 1, scope);
  }
  report(prepared.progress, "inventory", "ignored-generator-admission", 0, 1, scope);
  for (const output of generatorOutputRoots) if (output.inclusion === "ignored") for (const path of sourceAdmissionWalk(repoRoot, output.rootPath, taxonomy, scope, cancelFile, repositoryFences)) add(path, "ignored-generator");
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(prepared.progress, "inventory", "ignored-generator-admission", 1, 1, scope);
  report(prepared.progress, "inventory", "explicit-ticket-admission", 0, 1, scope);
  if (prepared.ticketDir) for (const path of sourceAdmissionWalk(repoRoot, prepared.ticketDir, taxonomy, scope, cancelFile, repositoryFences)) add(path, "explicit-ticket");
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences); report(prepared.progress, "inventory", "explicit-ticket-admission", 1, 1, scope);
  const candidates: TaxonomySourceCandidateObservation[] = [];
  report(prepared.progress, "inventory", "source-observation", 0, rows.size, scope);
  for (const [path, row] of rows) {
    sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
    const observation = sourceAdmissionObservation(repoRoot, path, SOURCE_ADMISSION_ORIGINS.filter((origin) => row.origins.has(origin)), row.indexEntries);
    if (row.directoryMarker && (observation.observedKind !== "directory" || observation.unsafeAncestor)) throw new Error(`Git untracked directory marker no longer matches a directory: ${path}`);
    candidates.push(observation);
    report(prepared.progress, "inventory", "source-observation", candidates.length, rows.size, path);
  }
  sourceAdmissionCheckCancellation(repoRoot, cancelFile, repositoryFences);
  const input: TaxonomySourceAdmissionInput = { scope: scope ?? null, opaquePrefixes: [...new Set(opaquePrefixes)], generatorOutputRoots, candidates };
  const inputText = JSON.stringify(input);
  const admission = projectTaxonomySourceAdmission(input);
  const inventory: TaxonomySourceInventory = { ...admission, repoRoot, taxonomyPath: relative(repoRoot, prepared.taxonomyPath).split(sep).join("/"), taxonomyContentHash: taxonomy.input.contentHash, membershipDigest: sha256(canonicalJson(admission)) };
  return { inventory, inputText };
}

/** 🧭️ Enumerates source admission without reading admitted leaf content. */
export function inventoryTaxonomySources(options: TaxonomySourceInventoryOptions): TaxonomySourceInventory {
  const prepared = sourceAdmissionPrepareOptions(options);
  const taxonomy = loadTaxonomy({ repoRoot: prepared.repoRoot, taxonomyPath: prepared.taxonomyPath });
  return collectTaxonomySourceAdmission(taxonomy, prepared).inventory;
}

interface CollectedTaxonomySourceAdmission {
  readonly inventory: TaxonomySourceInventory;
  readonly inputText: string;
}
