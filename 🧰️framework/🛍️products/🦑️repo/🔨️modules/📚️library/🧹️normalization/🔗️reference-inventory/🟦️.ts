import { type TaxonomyProgress, report } from "../🏃️operation/🟦️.ts";
import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull } from "../📁️input/🟦️.ts";
import { type SourceAdmissionCapture } from "../🚪️source-admission/📁️io/🟦️.ts";
import { readFileSync, readdirSync, readlinkSync } from "node:fs";
import type { Stats } from "node:fs";
import { type RegistryCatalogInputView } from "../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import { posix } from "node:path";
import type { TaxonomyInventory, TaxonomyInventoryEntry, TaxonomyPlanOptions } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, areaId, checkCancellation, isExcluded, ownerId } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { explicitTicketRows, gitRows, untrackedGitPaths } from "../📸️repository-capture/🟦️.ts";
import { fixedContractCandidates } from "../📏️fixed-contracts/🟦️.ts";

interface RetainedSourceAdmission {
  readonly originInventory: TaxonomyInventory;
  readonly originalInputText: string;
  readonly sourceInventoryText: string;
  readonly originSourceTreeDigest: string;
  readonly originInventoryDigest: string;
}

type ReferenceInventorySourceAdmission =
  | Readonly<{ state: "captured"; retained: RetainedSourceAdmission }>
  | Readonly<{ state: "derived-unproven"; retained: RetainedSourceAdmission }>
  | Readonly<{ state: "uncaptured" }>;

type ReferenceCandidatePathContext = Pick<ReferenceInventoryContext, "ticketDir" | "transactionRoots" | "exactEvidencePaths">;

interface ReferenceInventoryContext {
  readonly ticketDir?: string;
  readonly transactionRoots: readonly string[];
  readonly exactEvidencePaths: readonly string[];
  readonly sourceAdmission: ReferenceInventorySourceAdmission;
}

export interface IncomingReferenceSnapshot {
  readonly paths: readonly string[];
  readonly coordinateRoots: readonly string[];
  readonly entries: readonly TaxonomyInventoryEntry[];
  readonly contents: ReadonlyMap<string, string>;
}

export const referenceInventoryContexts = new WeakMap<TaxonomyInventory, ReferenceInventoryContext>();

/** 🪧️ The incoming-reference candidates of a multi-scope run, read once for every scope: the candidate paths, their coordinate
 * roots, and each symlink plus each textual file whose content the union of all scopes' changing paths lexically admits. */
export interface IncomingReferenceScan {
  readonly paths: readonly string[];
  readonly coordinateRoots: readonly string[];
  readonly observed: ReadonlyMap<string, Readonly<{ stat: Stats; target?: string; bytes?: Buffer }>>;
}

/** 📸️ The repository reads every scope of {@link verifyTaxonomyScopes} shares: the source admission, the taxonomy, the registry catalog
 * input view and, once any scope plans a move, the incoming-reference scan. */
export interface TaxonomyRepositoryCapture {
  readonly source: SourceAdmissionCapture;
  readonly taxonomy: LoadedTaxonomy;
  incoming?: IncomingReferenceScan;
  catalogView?: RegistryCatalogInputView;
}

export const inventoryCaptures = new WeakMap<TaxonomyInventory, TaxonomyRepositoryCapture>();

export const incomingReferenceSnapshots = new WeakMap<TaxonomyInventory, IncomingReferenceSnapshot>();

export function inheritReferenceInventoryContext(source: TaxonomyInventory, target: TaxonomyInventory, transactionRoot?: string, exactEvidencePath?: string): TaxonomyInventory {
  const prior = referenceInventoryContexts.get(source);
  const priorAdmission = prior?.sourceAdmission;
  const sourceAdmission: ReferenceInventorySourceAdmission = priorAdmission?.state === "captured" || priorAdmission?.state === "derived-unproven"
    ? Object.freeze({ state: "derived-unproven" as const, retained: priorAdmission.retained })
    : Object.freeze({ state: "uncaptured" as const });
  referenceInventoryContexts.set(target, { ticketDir: prior?.ticketDir, transactionRoots: [...new Set([...(prior?.transactionRoots ?? []), ...(transactionRoot ? [transactionRoot] : [])])], exactEvidencePaths: [...new Set([...(prior?.exactEvidencePaths ?? []), ...(exactEvidencePath ? [exactEvidencePath] : [])])], sourceAdmission });
  return target;
}

/** 🏠️ Anchored owning-ticket-root prefix of a governed ticket path — see {@link historicalDocumentEvidence}. */
const HISTORICAL_TICKET_ROOT_PATTERN = /^\.🧬semio\/🦑️repo\/🎫️tickets\/🎆️\d{2}\/🌙️\d{2}\/☀️\d{2}\/[^/]+/u;

/** 🗂️ Anchored owning-directory prefix of a governed dev prompt-log path — see {@link historicalDocumentEvidence}. Unlike a ticket, a prompt log has no per-item lifecycle root below it: the population's own directory (`💬️prompts/`) is the whole boundary. */
const HISTORICAL_PROMPT_LOG_ROOT_PATTERN = /^\.🧬semio\/🦑️repo\/💬️prompts/u;

/** 🧱️ Literal basenames of every `fixedFilenameContracts` entry that marks a package-root manifest (`scope.kind === "package-root"`) — Cargo.toml, package.json, go.mod, … — derived from the schema so a future contract addition is protected automatically, never hardcoded. */
function packageRootManifestBasenames(taxonomy: LoadedTaxonomy): ReadonlySet<string> {
  return new Set(Object.values(taxonomy.discoverySchema.fixedFilenameContracts).filter((contract) => contract.scope.kind === "package-root").map((contract) => posix.basename(contract.pathPattern)));
}

/** 📂️ True when `ancestor` or any directory between it and `path`'s owning historical-evidence boundary root (inclusive) directly contains a package-root manifest — i.e. `path` sits inside an embedded package boundary (a ticket-embedded crate, or — equally — a hypothetical package rooted directly under `💬️prompts/`) and must stay a live reference regardless of any historical-document population match. Only reached by {@link historicalDocumentEvidence} for a non-`ticket-report` population match: a `📓️` narrative report can never `open()` a file, so proximity to a manifest cannot make it live and this function is never consulted for one, however deeply the manifest shadows the rest of the directory. Deliberately not narrowed to the manifest's own ecosystem's source extensions: a co-located non-source file in a ticket-embedded package directory can be a genuinely live, load-bearing generator/build script with its own real repo-path references (verified concretely — `…/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS/generate_w1_a_gltf_create_scene.mjs`, a co-located `.mjs` sibling of that ticket's `Cargo.toml`, joins `process.cwd()` with a real repo-relative path and writes generated source under it at run time), so "shares a directory with the manifest" is the safe, already-tested boundary — narrowing it by extension would silently stop protecting exactly this kind of file. */
function historicalEvidenceBoundaryOwns(path: string, boundaryRoot: string, taxonomy: LoadedTaxonomy, repoRoot: string, manifestDirectories: Map<string, boolean> = new Map()): boolean {
  const manifests = packageRootManifestBasenames(taxonomy);
  for (let dir = posix.dirname(path); dir === boundaryRoot || dir.startsWith(`${boundaryRoot}/`); dir = posix.dirname(dir)) {
    let owns = manifestDirectories.get(dir);
    if (owns === undefined) {
      let entries: readonly string[];
      try { entries = readdirSync(absolutePath(repoRoot, dir)); } catch { entries = []; }
      owns = entries.some((name) => manifests.has(name));
      manifestDirectories.set(dir, owns);
    }
    if (owns) return true;
    if (dir === boundaryRoot) break;
  }
  return false;
}

const historicalLeafExpressions = new Map<string, RegExp>();

/** 📄️ True when `path` is a whole-document historical-evidence population (`🔣️taxonomy.json#historicalDocumentEvidencePopulations`) — a ticket's own 📓️-slugged narrative report, ticket workspace (evidence snapshots, scratch scripts, working notes), a Cursor plan snapshot, or a developer's own `💬️prompts/` transcript — and therefore excluded from every reference-candidate scan (never a reference source, never rewritten, never blocks a move). Document kind alone is the discriminator, not ticket lifecycle status — and kind is exactly what separates the two negatives below: the `ticket-report` population is prose by construction (`^📓️.+\.md$`), so it is exempt outright once matched; every other population (`ticket-workspace` most of all, whose `^.+$` leaf admits any file, evidence snapshot, scratch script or generator alike) can be code and stays subject to the package-boundary negative, because a `Cargo.toml`/`package.json`/… beside it means the file may genuinely be live, load-bearing source or a build script, not narrative. Never overrides a real machine-read contract: refuses whenever `path` itself matches any `fixedFilenameContracts` pattern; for every non-`ticket-report` population, refuses whenever `path` also sits inside a directory (up to the ticket root, or up to `💬️prompts/` itself for a prompt log) that owns a package-root manifest — so a ticket-embedded Cargo/Node/Go package (or, equally, a hypothetical one dropped directly under `💬️prompts/`) keeps its ordinary live-reference treatment. */
export function historicalDocumentEvidence(path: string, taxonomy: LoadedTaxonomy, repoRoot: string, manifestDirectories?: Map<string, boolean>): boolean {
  const populations = taxonomy.discoverySchema.historicalDocumentEvidencePopulations;
  const leafExpression = (pattern: string): RegExp => {
    let expression = historicalLeafExpressions.get(pattern);
    if (!expression) historicalLeafExpressions.set(pattern, expression = new RegExp(pattern, "u"));
    return expression;
  };
  const matches = (population: { directoryPattern: string; leafPattern: string }) => taxonomy.pathMatcher.matches(path, population.directoryPattern) && leafExpression(population.leafPattern).test(posix.basename(path));
  const ticketReport = populations["ticket-report"];
  const isNarrativeReport = Boolean(ticketReport && matches(ticketReport));
  const populated = isNarrativeReport || Object.values(populations).some(matches);
  if (!populated) return false;
  if (fixedContractCandidates(taxonomy.discoverySchema.fixedFilenameContracts, path).some(([, contract]) => taxonomy.pathMatcher.matches(path, contract.pathPattern))) return false;
  if (isNarrativeReport) return true;
  const boundaryRoot = HISTORICAL_TICKET_ROOT_PATTERN.exec(path)?.[0] ?? HISTORICAL_PROMPT_LOG_ROOT_PATTERN.exec(path)?.[0];
  if (boundaryRoot && historicalEvidenceBoundaryOwns(path, boundaryRoot, taxonomy, repoRoot, manifestDirectories)) return false;
  return true;
}

export function repositoryReferenceCandidatePaths(repoRoot: string, taxonomy: LoadedTaxonomy, context?: ReferenceCandidatePathContext, cancelFile?: string): readonly string[] {
  checkCancellation(repoRoot, cancelFile);
  const manifestDirectories = new Map<string, boolean>();
  const ignored = (path: string): boolean => isExcluded(path, taxonomy) || Boolean(context?.exactEvidencePaths.includes(path) || context?.transactionRoots.some((root) => path === root || path.startsWith(`${root}/`))) || historicalDocumentEvidence(path, taxonomy, repoRoot, manifestDirectories);
  const paths = new Set<string>();
  for (const row of gitRows(repoRoot, taxonomy)) if (!ignored(row.path)) paths.add(row.path);
  checkCancellation(repoRoot, cancelFile);
  for (const path of untrackedGitPaths(repoRoot, taxonomy)) if (!ignored(path)) paths.add(path);
  if (context?.ticketDir) for (const row of explicitTicketRows(repoRoot, context.ticketDir, taxonomy, undefined, cancelFile)) if (!ignored(row.path)) paths.add(row.path);
  return [...paths].sort(generatorPathCompare);
}

export function ancestorReferenceCoordinateRoot(path: string, roots: ReadonlySet<string>): string | undefined {
  for (let parent = posix.dirname(path); parent && parent !== "."; parent = posix.dirname(parent)) if (roots.has(parent)) return parent;
  return undefined;
}

export function incomingReferenceLexicalAdmission(targets: Iterable<string>): (content: string) => boolean {
  const needles = new Set<string>();
  for (const target of targets) {
    const name = posix.basename(target).normalize("NFC");
    needles.add(name);
    const stem = name.replace(/\.[^/.]+(?:\.[^/.]+)*$/u, "");
    if (stem) needles.add(stem);
    if (name === "__init__.py") needles.add(posix.basename(posix.dirname(target)).normalize("NFC"));
  }
  const values = [...needles].filter(Boolean);
  return (content) => {
    if (/\\(?:u(?:\{|[a-f0-9])|x[a-f0-9])/iu.test(content) || /["'`]\.\.?(?:\/\.\.?)*\/?["'`]/u.test(content)) return true;
    const normalized = content.normalize("NFC");
    return values.some((needle) => normalized.includes(needle));
  };
}

export function referenceEntry(repoRoot: string, path: string, taxonomy: LoadedTaxonomy): TaxonomyInventoryEntry | null {
  if (isExcluded(path, taxonomy)) throw new Error(`Reference candidate is opaque: ${path}`);
  const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Reference candidate"), stat = lstatOrNull(absolute);
  if (!stat) return null;
  const nodeKind = stat.isSymbolicLink() ? "symlink" : stat.isDirectory() ? "directory" : "file";
  if (nodeKind === "file" && !stat.isFile()) throw new Error(`Reference candidate is not a regular file: ${path}`);
  const target = nodeKind === "symlink" ? readlinkSync(absolute) : undefined;
  const bytes = nodeKind === "directory" ? Buffer.from("directory") : nodeKind === "symlink" ? Buffer.from(target!) : readFileSync(absolute);
  return { sourcePath: path, normalizedPath: path, nodeKind, ownerId: ownerId(path), areaId: areaId(path), fileKind: null, semanticStem: null, contentHash: sha256(bytes), mode: stat.mode & 0o7777, size: nodeKind === "directory" ? 0 : bytes.byteLength, ...(target === undefined ? {} : { symlinkTarget: target }), referencesIn: [], referencesOut: [], violations: [] };
}

export function* referenceCandidatesWithProgress(paths: readonly string[], operation: TaxonomyProgress["operation"], progress?: TaxonomyPlanOptions["progress"]): Generator<readonly [number, string]> {
  report(progress, operation, "incoming-candidates", 0, paths.length);
  for (const [index, path] of paths.entries()) {
    try {
      yield [index, path];
    } finally {
      report(progress, operation, "incoming-candidates", index + 1, paths.length, path);
    }
  }
}
