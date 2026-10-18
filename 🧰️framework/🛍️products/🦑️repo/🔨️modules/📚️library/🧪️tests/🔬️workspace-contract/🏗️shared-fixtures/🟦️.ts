import toml from "@iarna/toml";
import { expect } from "bun:test";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { chmodSync, closeSync, constants, existsSync, fstatSync, fsyncSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, readlinkSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, posix, relative, resolve, sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { registryCompilerInputDependencies, type RegistryCompilerInputRole } from "../../../🔍️discovery/📇️registry-catalog-inputs/🟦️.ts";
import { type FixedDirectoryContract, type FixedFilenameContract, type PackageSourceDisposition } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import { type SemanticProjectionAuthorityNode } from "../../../🔍️discovery/🧱️path-projection-contracts/🟦️.ts";
import { type Taxonomy } from "../../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { applyTaxonomyPlan } from "../../../🧹️normalization/🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../../🧹️normalization/🔍️inventory-planning/🟦️.ts";
import { noFollowTreeDigest, opaqueTreeDigest } from "../../../🧹️normalization/🌳️tree-digests/🟦️.ts";
import { parseTaxonomyPlan } from "../../../🧹️normalization/📃️plan-document/🟦️.ts";
import { planTaxonomy } from "../../../🧹️normalization/🧠️planning/🟦️.ts";
import { taxonomyPlanDigest } from "../../../🧹️normalization/🗃️affected-state/🟦️.ts";
import { type OpaqueTreeDigest, type TaxonomyApplyOptions, type TaxonomyApplyResult, type TaxonomyInventory, type TaxonomyInventoryOptions, type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";
import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { type TaxonomyProgress } from "../../../🧹️normalization/🏃️operation/🟦️.ts";
import { ownedFilePaths } from "../../🔍️filesystem/🟦️.ts";
import * as contractPath from "node:path";
import * as contractUrlModule from "node:url";

export const contractRoot = contractPath.resolve(import.meta.dir, "..");
export const contractUrl = contractUrlModule.pathToFileURL(contractPath.join(contractRoot, "🟦️.ts")).href;

/** 🧾️ `@iarna/toml` declares every document as `AnyJson`; a TOML document is always a keyed table, and each
 * reader below states the exact table shape it asserts against. */
export function readToml<T>(text: string): T {
  return toml.parse(text) as unknown as T;
}

/** 🧬️ A deeply mutable projection of one frozen contract record, for the `structuredClone`-then-mutate probes below. */
type Mutable<T> = { -readonly [Key in keyof T]: T[Key] };

/** 🧬️ The frozen taxonomy with exactly the registries these probes rewrite opened for writing; every other field
 * keeps its declared shape, so the value stays assignable wherever a `Taxonomy` is expected. */
type ProbeTaxonomy = Omit<Taxonomy, "fixedDirectoryContracts" | "fixedFilenameContracts" | "packageSourceDispositions" | "fixedDirectoryContractSets"> & {
  fixedDirectoryContracts: Record<string, Mutable<FixedDirectoryContract>>;
  fixedFilenameContracts: Record<string, Mutable<FixedFilenameContract>>;
  packageSourceDispositions: Record<string, Mutable<PackageSourceDisposition>>;
  fixedDirectoryContractSets: Record<string, readonly string[]>;
};

/** 🧬️ A structured deep copy is a fresh object graph, so the copy is safe to mutate even where the source type is frozen. */
export function probeClone(value: Taxonomy): ProbeTaxonomy {
  return structuredClone(value) as ProbeTaxonomy;
}

type ArtifactProjectionGolden = Readonly<{
  schemaVersion: 1;
  mappingDigestAlgorithm: "sha256-source-nul-destination-lines-v1";
  projections: readonly ArtifactProjectionGoldenEntry[];
}>;

export type ArtifactProjectionGoldenEntry = Readonly<{
  contractId: "artifact-example-model-catalog-v1" | "artifact-editor-command-bundle-v1";
  rationaleRule: string;
  artifactId: string;
  standardVersion: string;
  subsetId: string;
  profileDirectoryName: string;
  sourceRoot: string;
  destinationRoot: string;
  sourceFileCount: number;
  destinationDirectoryCount: number;
  destinationNodeCount: number;
  maxPathBytes: number;
  mappingDigest: string;
  mappings: readonly Readonly<{ sourcePath: string; destinationPath: string }>[];
  modelCatalog?: Readonly<{
    models: readonly Readonly<{ directoryName: string; id: string; schema: string; version: string }>[];
    categoryRules: readonly Readonly<{ sourceDirectoryName: string; sourceShape: string; manifestSchema: string; count: number }>[];
  }>;
}>;

export const ARTIFACT_PROJECTION_GOLDEN = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/📐️cad-draw-path-projection/🔣️.json"), "utf8")) as ArtifactProjectionGolden;

/** 🧵️ The live CAD carriers of the sealed golden's authored sources — unsealed, so moves rewrite them like any reference. */
type CadLiveBindings = Readonly<{ schemaVersion: 1; contract: "cad-draw-live-bindings-v1"; sealedContractId: "cad-draw-projection-vectors-v1"; projection: "artifact-example-model-catalog-v1"; bindings: readonly Readonly<{ mapping: number; live: string }>[] }>;

export const CAD_LIVE_BINDINGS = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🧫️cad-draw-live-bindings/🔣️.json"), "utf8")) as CadLiveBindings;

export type DrawSourceScenario = Readonly<{
  schemaVersion: 1;
  contractId: "draw-source-scenario-input-v1";
  producerContext: Readonly<{ generatorId: string; compilerRoots: readonly string[]; runtimeModules: readonly string[]; runtimeData: readonly string[]; runtimeReceiptCatalogs: readonly Readonly<{ manifestPath: string; sourceRoot: string; receiptPathField: string }>[]; workspaceInputs: readonly string[]; runtimePackages: readonly string[]; authority: string; workspaceBindings: string; initialOutputs: string; registryNodeCount: number }>;
  catalogContext: readonly DrawSourceScenarioInput[];
  cargoModuleRoot: DrawSourceScenarioInput;
  cadConsumerMount: Readonly<{ path: string; meaning: string }>;
  owner: Readonly<{ artifactId: string; standardVersion: string; subsetId: string; commandDirectoryName: string }>;
  commandBundle: Readonly<{ directoryKinds: Readonly<Record<string, Taxonomy["semanticDirectoryKinds"][string]>>; descendantContract: Extract<Taxonomy["semanticDescendantContracts"][string], Readonly<{ exclusiveAlternatives: unknown }>> }>;
  members: readonly DrawSourceScenarioInput[];
  consumers: readonly DrawSourceScenarioInput[];
  retention: Readonly<{ fixtureOwnerSegments: readonly string[]; transactionTicketSegments: readonly string[]; parentSegments: readonly string[]; recordFilename: string; disposition: string; cases: readonly Readonly<{ kind: "directory" | "symlink" | "file" | "missing" | "unreadable"; allowed: boolean; creates: number }>[] }>;
  oracle: Readonly<{
    sourceDirectoryCount: number;
    destinationDirectoryCount: number;
    destinationNodeCount: number;
    workspaceMemberCount: number;
    dependencyUserCount: number;
    projectCwdCount: number;
    workspaceGlobCount: number;
    rootCollectionCount: number;
    externalModuleCount: number;
    activatedGeneratorIds: readonly string[];
    cargoModuleMount: Readonly<{ manifestPath: string; packageName: string; libraryEntry: string; moduleName: string; moduleTarget: string }>;
  }>;
}>;

type DrawSourceScenarioInput = Readonly<{ path: string; format: "json" | "jsonc" | "toml" | "typescript" | "rust"; content: string }>;

export const DRAW_SOURCE_SCENARIO_ROOT = join(contractRoot, "../🧫️fixtures/🖍️draw-source-scenario");

export const DRAW_SOURCE_SCENARIO = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "🔣️.json"), "utf8")) as DrawSourceScenario;

export function projectionByteSort(left: string, right: string): number {
  return Buffer.from(left).compare(Buffer.from(right));
}

export function projectionGolden(contractId: ArtifactProjectionGoldenEntry["contractId"]): ArtifactProjectionGoldenEntry {
  const projection = ARTIFACT_PROJECTION_GOLDEN.projections.find((candidate) => candidate.contractId === contractId);
  if (!projection) throw new Error(`Missing projection golden ${contractId}.`);
  return projection;
}

/** 📦️ Overlays the authored Draw scenario's own sealed command-bundle authority (two declaration-only nested Rust packages) on a taxonomy, so the scenario's package-move coverage never binds to the live Draw command bundle. */
export function drawScenarioTaxonomy<T extends Readonly<{ semanticDirectoryKinds: object; semanticDescendantContracts: object }>>(taxonomy: T): T {
  return { ...taxonomy, semanticDirectoryKinds: { ...taxonomy.semanticDirectoryKinds, ...DRAW_SOURCE_SCENARIO.commandBundle.directoryKinds }, semanticDescendantContracts: { ...taxonomy.semanticDescendantContracts, "draw-editor-command-bundle-v1": DRAW_SOURCE_SCENARIO.commandBundle.descendantContract } };
}

export function drawSourceScenarioContent(content: string, projection: ArtifactProjectionGoldenEntry): string {
  if (projection.contractId !== "artifact-editor-command-bundle-v1") throw new Error("Authored Draw inputs require the exact Draw projection");
  const rendered = content.replaceAll("{{sourceRoot}}", projection.sourceRoot).replaceAll("{{artifactSourceRoot}}", artifactProjectionFixturePath(projection.sourceRoot));
  if (rendered.includes("{{")) throw new Error("Authored Draw input has an unknown binding");
  return rendered;
}

function drawSourceScenarioPreimage(destinationPath: string): string {
  const projection = projectionGolden("artifact-editor-command-bundle-v1");
  const mapping = projection.mappings.find((row) => row.destinationPath === destinationPath);
  const member = mapping && DRAW_SOURCE_SCENARIO.members.find(({ path }) => `${projection.sourceRoot}/${path}` === mapping.sourcePath);
  if (!member) throw new Error(`Authored Draw preimage does not own ${destinationPath}`);
  return createHash("sha256").update(drawSourceScenarioContent(member.content, projection)).digest("hex");
}

/** 🧵️ Binds every frozen CAD scenario source to its exact current carrier: each declared live path is one owned file,
 * and every owned file carries exactly one scenario source. */
export function cadProjectionLiveSources(projection: ArtifactProjectionGoldenEntry, paths = ownedFilePaths(join(getWorkspaceRoot(), projection.sourceRoot)).filter((path) => path.endsWith(".json")), live = CAD_LIVE_BINDINGS): ReadonlyMap<string, string> {
  if (projection.contractId !== live.projection) throw new Error("CAD source bindings require the exact catalog projection");
  const owned = new Set<string>();
  for (const path of paths) {
    const livePath = `${projection.sourceRoot}/${path}`;
    if (owned.has(livePath)) throw new Error(`Duplicate current CAD source: ${path}`);
    owned.add(livePath);
  }
  const bound = new Map<string, string>(), carriers = new Set<string>();
  for (const { mapping, live: livePath } of live.bindings) {
    const sourcePath = projection.mappings[mapping]?.sourcePath;
    if (!sourcePath || !livePath.startsWith(`${projection.sourceRoot}/`) || livePath.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("CAD scenario binding is outside its exact root");
    if (!owned.has(livePath) || bound.has(sourcePath) || carriers.has(livePath)) throw new Error(`Missing or duplicate CAD scenario binding: ${livePath}`);
    bound.set(sourcePath, livePath);
    carriers.add(livePath);
  }
  if (bound.size !== projection.mappings.length || projection.mappings.some(({ sourcePath }) => !bound.has(sourcePath))) throw new Error("CAD scenario bindings do not cover the projection exactly");
  if (carriers.size !== owned.size) throw new Error("Current CAD source census contains unbound members");
  return bound;
}

/** 🔗️ Rebinds a live CAD consumer's catalog-anchored carrier names onto the authored source names the golden projection plans from. */
function cadProjectionConsumerContent(content: string, projection = projectionGolden("artifact-example-model-catalog-v1")): string {
  const anchor = `${basename(projection.sourceRoot)}/`;
  let rendered = content;
  for (const [sourcePath, livePath] of cadProjectionLiveSources(projection)) if (sourcePath !== livePath) rendered = rendered.replaceAll(`${anchor}${livePath.slice(projection.sourceRoot.length + 1)}`, `${anchor}${sourcePath.slice(projection.sourceRoot.length + 1)}`);
  return rendered;
}

export function projectionAuthorityNodes(projection: ArtifactProjectionGoldenEntry, source: "live" | "authored-draw" = "live", readLive: (path: string) => string = (path) => readFileSync(join(getWorkspaceRoot(), path), "utf8")): SemanticProjectionAuthorityNode[] {
  if (source === "authored-draw" && projection.contractId !== "artifact-editor-command-bundle-v1") throw new Error("Authored Draw inputs cannot supply another projection");
  const liveSources = source === "live" && projection.contractId === "artifact-example-model-catalog-v1" ? cadProjectionLiveSources(projection) : undefined;
  const directories = new Set<string>([projection.sourceRoot]);
  const files = projection.mappings.map(({ sourcePath }) => {
    let owner = sourcePath.slice(0, sourcePath.lastIndexOf("/"));
    while (owner.length >= projection.sourceRoot.length) {
      directories.add(owner);
      if (owner === projection.sourceRoot) break;
      owner = owner.slice(0, owner.lastIndexOf("/"));
    }
    const member = source === "authored-draw" ? DRAW_SOURCE_SCENARIO.members.find(({ path }) => `${projection.sourceRoot}/${path}` === sourcePath) : undefined;
    if (source === "authored-draw" && !member) throw new Error(`Authored Draw input does not own ${sourcePath}`);
    return { path: sourcePath, nodeKind: "file" as const, content: member ? drawSourceScenarioContent(member.content, projection) : readLive(liveSources ? liveSources.get(sourcePath)! : sourcePath) };
  });
  return [...[...directories].sort(projectionByteSort).map((path) => ({ path, nodeKind: "directory" as const })), ...files];
}

type NormalizationFixture = {
  baselineCommit: string;
  opaqueDigest: OpaqueTreeDigest;
  options: TaxonomyInventoryOptions;
  repoRoot: string;
  root: string;
  scope: string;
  ticketDir: string;
  workspace: string;
};

export const NORMALIZATION_RUN_ROOT = realpathSync(tmpdir());

export const NORMALIZATION_RUN_DIRECTORY = "semio-normalization";

export const ARTIFACT_PROJECTION_RUN_PARENT = `${NORMALIZATION_RUN_DIRECTORY}/📓️draw-source-scenarios/🧪️runs`;

const ARTIFACT_PROJECTION_RETAINED_RUNS = new Map<string, Readonly<{ device: number; inode: number; reportHash: string; name: string; startedAt: string }>>();

export const DRAW_SUBMITTED_PROOF = JSON.parse(readFileSync(join(DRAW_SOURCE_SCENARIO_ROOT, "📨️submitted-proof/🔣️.json"), "utf8")) as Readonly<{ directoryName: string; filename: string; fileMode: number; progress: Readonly<{ externalParentSegments: readonly string[]; intervalMs: number }> }>;

const ARTIFACT_PROJECTION_PROGRESS_RECORDS = new Map<string, { path: string; bytes: Buffer; ordinal: number }>();

export const NORMALIZATION_SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

const TICKET_IMPORTANT_HISTORY_ROOT_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SCOPED-HISTORY-APPLY";

export const TICKET_IMPORTANT_HISTORY_SOURCE_REL = `${TICKET_IMPORTANT_HISTORY_ROOT_REL}/📌️important.md`;

export const TICKET_IMPORTANT_HISTORY_DESTINATION_REL = `${TICKET_IMPORTANT_HISTORY_ROOT_REL}/📓️important/📝️.md`;

export const SCOPED_INVENTORY_PATHSPEC_GOLDEN = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json"), "utf8")) as Readonly<{
  schemaVersion: 1;
  opaqueExclusions: readonly string[];
  cases: readonly Readonly<{ id: string; inputScope: string | null; normalizedScope: string | null; conservativePrefix: string; positivePathspec: string; exclusionPathspecs: readonly string[] }>[];
}>;

/** 🧾️ Captures precisely the typed, digest-bound bytes that apply will authenticate. */
export function artifactProjectionSubmittedPlanBytes(value: unknown): Buffer {
  const plan = parseTaxonomyPlan(value);
  if (taxonomyPlanDigest(plan) !== plan.planDigest) throw new Error("Submitted artifact plan has an invalid self digest");
  return Buffer.from(`${canonicalJson(plan)}\n`);
}

/** ⏱️ Retains bounded phase/count evidence without introducing source-coordinate references. */
export function artifactProjectionProgressSampler(retain: (row: Readonly<{ elapsedMs: number; operation: TaxonomyProgress["operation"]; phase: string; current: number; total: number }>) => void, now: () => number = performance.now.bind(performance)): (event: TaxonomyProgress) => void {
  const started = now(), prior = new Map<string, Readonly<{ at: number; complete: boolean }>>();
  return (event) => {
    const at = now(), key = `${event.operation}:${event.phase}`, previous = prior.get(key), complete = event.current === event.total;
    if (previous && at - previous.at < DRAW_SUBMITTED_PROOF.progress.intervalMs && (!complete || previous.complete)) return;
    prior.set(key, { at, complete });
    retain({ elapsedMs: Math.round(at - started), operation: event.operation, phase: event.phase, current: event.current, total: event.total });
  };
}

/** 🔐️ Revalidates the unique retained run and its exact source/ticket coordinates. */
function artifactProjectionAssertRun(fixture: NormalizationFixture): void {
  if (DRAW_SUBMITTED_PROOF.directoryName !== "📓️submitted-plans" || DRAW_SUBMITTED_PROOF.filename !== "🔣️.json" || DRAW_SUBMITTED_PROOF.fileMode !== 0o600 || DRAW_SUBMITTED_PROOF.progress.intervalMs !== 1000 || JSON.stringify(DRAW_SUBMITTED_PROOF.progress.externalParentSegments) !== JSON.stringify(["📓️draw-source-scenarios", "📓️progress"])) throw new Error("Artifact proof contract changed outside its exact evidence authority");
  const owner = normalizationRetainedRunParent(ARTIFACT_PROJECTION_RUN_PARENT), record = ARTIFACT_PROJECTION_RETAINED_RUNS.get(fixture.root);
  if (!record || dirname(fixture.root) !== owner || fixture.repoRoot !== fixture.root) throw new Error("Artifact proof has no retained run authority");
  const node = lstatSync(fixture.root), locations = artifactProjectionFixtureLocations(fixture.root);
  if (!node.isDirectory() || node.isSymbolicLink() || node.dev !== record.device || node.ino !== record.inode || fixture.ticketDir !== locations.ticketDir || fixture.workspace !== locations.workspace) throw new Error("Artifact proof run identity changed");
}

/** 💾️ Synchronizes directory entries where the host supports directory descriptors. */
function artifactProjectionSyncDirectory(path: string): void {
  if (process.platform === "win32") return;
  const descriptor = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try { fsyncSync(descriptor); } finally { closeSync(descriptor); }
}

/** 🗂️ Creates only requested evidence descendants after no-follow ancestor checks. */
function artifactProjectionEvidenceDirectory(root: string, segments: readonly string[], create: boolean): string {
  let path = root;
  for (const segment of ["", ...segments]) {
    if (segment && (segment !== basename(segment) || segment === "." || segment === ".." || segment.includes("\\") || segment.includes("\0"))) throw new Error("Artifact evidence segment is not a leaf");
    if (segment) path = join(path, segment);
    let node;
    try { node = lstatSync(path); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || !segment || !create) throw error;
      mkdirSync(path);
      artifactProjectionSyncDirectory(dirname(path));
      node = lstatSync(path);
    }
    if (!node.isDirectory() || node.isSymbolicLink()) throw new Error("Artifact evidence ancestor is not a no-follow directory: " + path);
  }
  return path;
}

/** 📖️ Reads evidence through a no-follow descriptor bound to its inspected identity. */
function artifactProjectionReadEvidence(path: string): Buffer {
  const before = lstatSync(path);
  if (!before.isFile() || before.isSymbolicLink()) throw new Error("Artifact evidence is not a regular no-follow file");
  const descriptor = openSync(path, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const opened = fstatSync(descriptor), bytes = readFileSync(descriptor), after = lstatSync(path);
    if (!opened.isFile() || after.isSymbolicLink() || opened.dev !== before.dev || opened.ino !== before.ino || after.dev !== before.dev || after.ino !== before.ino || after.mode !== before.mode || after.size !== before.size || after.mtimeMs !== before.mtimeMs || bytes.length !== before.size) throw new Error("Artifact evidence changed while reading");
    return bytes;
  } finally { closeSync(descriptor); }
}

/** 🧾️ Publishes exclusive file-synced evidence or verifies an identical prior publication. */
function artifactProjectionWriteEvidence(path: string, bytes: Buffer): void {
  let exists = true;
  try { lstatSync(path); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; exists = false; }
  if (exists) {
    if (process.platform !== "win32" && (lstatSync(path).mode & 0o7777) !== DRAW_SUBMITTED_PROOF.fileMode) throw new Error("Existing artifact evidence mode changed; retained without overwrite");
    if (!artifactProjectionReadEvidence(path).equals(bytes)) throw new Error("Existing artifact evidence differs; retained without overwrite");
    return;
  }
  const descriptor = openSync(path, "wx", DRAW_SUBMITTED_PROOF.fileMode);
  try { writeFileSync(descriptor, bytes); fsyncSync(descriptor); } finally { closeSync(descriptor); }
  artifactProjectionSyncDirectory(dirname(path));
  if (!artifactProjectionReadEvidence(path).equals(bytes)) throw new Error("Published artifact evidence differs from submitted bytes");
}

/** 📌️ Persists the exact submitted plan before apply without reconstructing prior lost plans. */
export function artifactProjectionCaptureSubmittedPlan(fixture: NormalizationFixture, value: unknown): string {
  artifactProjectionAssertRun(fixture);
  const bytes = artifactProjectionSubmittedPlanBytes(value), plan = JSON.parse(bytes.toString()) as TaxonomyPlan;
  if (plan.baselineCommit !== fixture.baselineCommit || plan.scope !== fixture.scope) throw new Error("Submitted plan is outside its fixture identity");
  const ticket = artifactProjectionEvidenceDirectory(fixture.root, relative(fixture.root, fixture.ticketDir).split(sep), false);
  const owner = artifactProjectionEvidenceDirectory(ticket, [DRAW_SUBMITTED_PROOF.directoryName, `🔖️${plan.planDigest}`], true), path = join(owner, DRAW_SUBMITTED_PROOF.filename);
  artifactProjectionWriteEvidence(path, bytes);
  return path;
}

/** 📡️ Keeps mutable diagnostics outside the fixture repository's reference/read universe. */
export function artifactProjectionProgressRecord(fixture: NormalizationFixture): { path: string; bytes: Buffer; ordinal: number } {
  artifactProjectionAssertRun(fixture);
  const ticket = artifactProjectionEvidenceDirectory(NORMALIZATION_RUN_ROOT, NORMALIZATION_RUN_DIRECTORY.split("/"), false);
  const owner = artifactProjectionEvidenceDirectory(ticket, [...DRAW_SUBMITTED_PROOF.progress.externalParentSegments, basename(fixture.root)], true);
  let record = ARTIFACT_PROJECTION_PROGRESS_RECORDS.get(fixture.root);
  if (!record) {
    const path = join(owner, "📝️.md"), bytes = Buffer.from(`# Retained Draw Phase Evidence\n\nWorker PID: ${process.pid}\n\nRun: ${basename(fixture.root)}\n\n`);
    if (existsSync(path)) throw new Error("Artifact progress owner already has an unbound record");
    artifactProjectionWriteEvidence(path, bytes);
    record = { path, bytes, ordinal: 0 };
    ARTIFACT_PROJECTION_PROGRESS_RECORDS.set(fixture.root, record);
  }
  return record;
}

/** 📝️ Appends only to the exact previously observed progress file and synchronizes its bytes. */
function artifactProjectionAppendProgress(fixture: NormalizationFixture, value: unknown): void {
  const record = artifactProjectionProgressRecord(fixture), before = lstatSync(record.path);
  if (!before.isFile() || before.isSymbolicLink()) throw new Error("Artifact progress record is not a regular file");
  const descriptor = openSync(record.path, constants.O_RDWR | (constants.O_NOFOLLOW ?? 0));
  try {
    const opened = fstatSync(descriptor), bytes = readFileSync(descriptor), current = lstatSync(record.path);
    if (opened.dev !== before.dev || opened.ino !== before.ino || current.isSymbolicLink() || current.dev !== before.dev || current.ino !== before.ino || current.mode !== before.mode || !bytes.equals(record.bytes)) throw new Error("Artifact progress evidence changed before append");
    const suffix = Buffer.from(`- ${JSON.stringify(value)}\n`);
    writeFileSync(descriptor, suffix);
    fsyncSync(descriptor);
    record.bytes = Buffer.concat([bytes, suffix]);
  } finally { closeSync(descriptor); }
  if (!artifactProjectionReadEvidence(record.path).equals(record.bytes)) throw new Error("Artifact progress evidence changed after append");
}

/** 🧭️ Records a bounded phase stream and terminal state around one synchronous fixture operation. */
export function artifactProjectionRecordedStage<T>(fixture: NormalizationFixture, stage: "plan" | "apply", run: (progress: (event: TaxonomyProgress) => void) => T): T {
  const ordinal = ++artifactProjectionProgressRecord(fixture).ordinal;
  artifactProjectionAppendProgress(fixture, { stage, ordinal, state: "started" });
  const progress = artifactProjectionProgressSampler((row) => artifactProjectionAppendProgress(fixture, { stage, ordinal, ...row }));
  try {
    const result = run(progress);
    artifactProjectionAppendProgress(fixture, { stage, ordinal, state: "completed" });
    return result;
  } catch (error) {
    artifactProjectionAppendProgress(fixture, { stage, ordinal, state: "failed" });
    throw error;
  }
}

/** 🛡️ Binds every artifact apply to its retained submitted-plan bytes and external phase evidence. */
export function artifactProjectionApply(fixture: NormalizationFixture, plan: TaxonomyPlan, options: Omit<TaxonomyApplyOptions, "repoRoot" | "ticketDir" | "expectedBaselineCommit" | "expectedPlanDigest" | "planArtifactPath"> = {}): TaxonomyApplyResult {
  const planArtifactPath = artifactProjectionCaptureSubmittedPlan(fixture, plan);
  return artifactProjectionRecordedStage(fixture, "apply", (progress) => applyTaxonomyPlan(plan, { ...options, repoRoot: fixture.repoRoot, ticketDir: fixture.ticketDir, expectedBaselineCommit: plan.baselineCommit, expectedPlanDigest: plan.planDigest, planArtifactPath, progress: (event) => { progress(event); options.progress?.(event); } }));
}

/** 🧹️ Gives one absent fixture residue an exact repository-local ignore rule. */
export function artifactProjectionIgnoreResidue(fixture: NormalizationFixture, residue: string): void {
  artifactProjectionAssertRun(fixture);
  const path = relative(fixture.root, residue).replaceAll("\\", "/");
  if (!path.startsWith(`${fixture.scope}/`) || basename(residue) !== "🧪️unexpected" || /[\r\n\0*?[\]\\]/u.test(path) || path.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("Residue ignore rule is outside its exact fixture scope");
  try { lstatSync(residue); throw new Error("Residue must be absent before ignore authority is installed"); } catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
  artifactProjectionEvidenceDirectory(fixture.root, path.split("/").slice(0, -1), false);
  const ignorePath = join(fixture.root, ".gitignore"), before = lstatSync(ignorePath), bytes = artifactProjectionReadEvidence(ignorePath);
  const suffix = Buffer.from(`${bytes.length && bytes.at(-1) !== 10 ? "\n" : ""}/${path}/\n`), descriptor = openSync(ignorePath, constants.O_RDWR | (constants.O_NOFOLLOW ?? 0));
  try {
    const opened = fstatSync(descriptor), observed = readFileSync(descriptor), current = lstatSync(ignorePath);
    if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino || current.isSymbolicLink() || current.dev !== before.dev || current.ino !== before.ino || current.mode !== before.mode || !observed.equals(bytes)) throw new Error("Fixture ignore authority changed before append");
    writeFileSync(descriptor, suffix);
    fsyncSync(descriptor);
  } finally { closeSync(descriptor); }
  if (lstatSync(ignorePath).mode !== before.mode || !artifactProjectionReadEvidence(ignorePath).equals(Buffer.concat([bytes, suffix]))) throw new Error("Fixture ignore authority changed after append");
}

/** 🎛️ Compares actual fixed-budget routes through independent compilers. */
export async function assertArtifactProjectionSingleCaseRoute(vector: Readonly<{ command: string; target: string; budgetMs: number; bunTimeoutMs: number; testName: string }>): Promise<void> {
  const packageRoot = resolve(contractRoot, "../../📦️packages/🟦️typescript");
  const project = JSON.parse(readFileSync(join(packageRoot, "📋️project.json"), "utf8"));
  expect(project.targets[vector.target]).toBeDefined();
  expect(project.targets[vector.target].executor).toBe("repo:script");
  expect(project.targets[vector.target].options).toEqual({ cwd: relative(getWorkspaceRoot(), packageRoot).replaceAll("\\", "/"), command: `bun ./📜️script.ts test ${vector.command}` });
  const ts = await import("typescript"), sourcePath = join(packageRoot, "📜️script.ts"), source = readFileSync(sourcePath, "utf8"), syntax = ts.createSourceFile(sourcePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declarations = syntax.statements.filter((node) => ts.isClassDeclaration(node) && node.name?.text === "TestScript");
  expect(declarations).toHaveLength(1);
  const code = `${declarations[0]!.getText(syntax)}\nreturn new TestScript();`;
  const expectedArgs = ["test", join(contractRoot, "🟦️.ts"), "--timeout", String(vector.bunTimeoutMs), "-t", vector.testName];
  for (const javascript of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), ts.transpileModule(code, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText]) {
    const invocations: { command: string; args: string[]; options: { cwd: string; budgetMs: number } }[] = [];
    class FixtureBundle { root = packageRoot; repoRoot = getWorkspaceRoot(); }
    const router = new Function("BundleScript", "join", "runRepositoryTestCommand", "resolveTestLevel", javascript)(FixtureBundle, join, async (command: string, args: string[], options: { cwd: string; budgetMs: number }) => { invocations.push({ command, args, options }); }, () => { throw new Error("Dedicated case fell through to generic test routing"); });
    await router.run([vector.command]);
    expect(invocations.map(({ command, args, options }) => ({ command, args, cwd: options.cwd, budgetMs: options.budgetMs }))).toEqual([{ command: process.execPath, args: expectedArgs, cwd: getWorkspaceRoot(), budgetMs: vector.budgetMs }]);
    await expect(router.run([vector.command, "--test-name-pattern=other"])).rejects.toThrow();
    expect(invocations).toHaveLength(1);
  }
}

export function normalizationGit(root: string, args: readonly string[]): string {
  const result = Bun.spawnSync(["git", ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
  const stdout = result.stdout.toString();
  const stderr = result.stderr.toString();
  if (result.exitCode !== 0) throw new Error(`fixture git ${args.join(" ")} failed: ${stderr || stdout}`);
  return stdout.trim();
}

export function normalizationWriteFiles(root: string, files: Readonly<Record<string, string>>): void {
  for (const [path, content] of Object.entries(files)) {
    const absolute = join(root, path);
    mkdirSync(resolve(absolute, ".."), { recursive: true });
    writeFileSync(absolute, content);
  }
}

export function normalizationRetainedRunParent(parent: string, inspect: (path: string) => { isDirectory(): boolean; isSymbolicLink(): boolean } = lstatSync, create: (path: string) => void = (path) => { mkdirSync(path); }): string {
  if (parent !== ARTIFACT_PROJECTION_RUN_PARENT) throw new Error("Retained normalization requires the exact artifact run parent");
  let current = NORMALIZATION_RUN_ROOT;
  const workspace = inspect(current);
  if (!workspace.isDirectory() || workspace.isSymbolicLink()) throw new Error("Retained fixture workspace is not a no-follow directory");
  for (const segment of parent.split("/")) {
    current = join(current, segment);
    let node: ReturnType<typeof inspect>;
    try { node = inspect(current); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
      create(current);
      node = inspect(current);
    }
    if (!node.isDirectory() || node.isSymbolicLink()) throw new Error(`Retained fixture ancestor is not a no-follow directory: ${current}`);
  }
  return current;
}

export function retainArtifactProjectionFixture(fixture: NormalizationFixture, passed: boolean): void {
  const record = ARTIFACT_PROJECTION_RETAINED_RUNS.get(fixture.root);
  const parent = normalizationRetainedRunParent(ARTIFACT_PROJECTION_RUN_PARENT);
  if (!record || dirname(fixture.root) !== parent) throw new Error("Artifact fixture run has no exact retention ownership");
  const root = lstatSync(fixture.root), reportPath = join(fixture.root, "📝️.md"), report = lstatSync(reportPath);
  if (!root.isDirectory() || root.isSymbolicLink() || root.dev !== record.device || root.ino !== record.inode || !report.isFile() || report.isSymbolicLink()) throw new Error("Artifact fixture retention node identity changed");
  if (createHash("sha256").update(readFileSync(reportPath)).digest("hex") !== record.reportHash) throw new Error("Artifact fixture retention report changed");
  writeFileSync(reportPath, `# Retained Artifact Projection Run\n\nScenario: ${record.name}\n\nStarted: ${record.startedAt}\n\nFinished: ${new Date().toISOString()}\n\nOutcome: ${passed ? "passed" : "failed or interrupted before completion"}\n\nAll authored inputs and generated/recovery evidence remain retained pending exact review. No output or fixture input was deleted by the finalizer.\n`);
}

/** 🗂️ Keeps the authored source owner distinct from its metadata transaction ticket. */
export function artifactProjectionFixtureLocations(root: string): Readonly<{ ticketDir: string; workspace: string }> {
  return {
    ticketDir: join(root, ...DRAW_SOURCE_SCENARIO.retention.transactionTicketSegments),
    workspace: join(root, ...DRAW_SOURCE_SCENARIO.retention.fixtureOwnerSegments, "🧪️fixture"),
  };
}

export function normalizationFixture(name: string, files: Readonly<Record<string, string>>, configure?: (fixture: { repoRoot: string; ticketDir: string; workspace: string }) => void, retainedRunParent?: string): NormalizationFixture {
  const owner = retainedRunParent === undefined ? resolve(NORMALIZATION_RUN_ROOT, NORMALIZATION_RUN_DIRECTORY) : normalizationRetainedRunParent(retainedRunParent);
  mkdirSync(owner, { recursive: true });
  const root = mkdtempSync(join(owner, `🧪️s-test-${name}-`));
  if (retainedRunParent !== undefined) {
    const node = lstatSync(root), startedAt = new Date().toISOString();
    if (!node.isDirectory() || node.isSymbolicLink() || dirname(root) !== owner) throw new Error("Retained fixture allocation lost its unique directory ownership");
    const report = `# Retained Artifact Projection Run\n\nScenario: ${name}\n\nStarted: ${startedAt}\n\nOutcome: prepared; final completion not yet recorded\n\nAll authored inputs and generated/recovery evidence remain retained pending exact review.\n`;
    writeFileSync(join(root, "📝️.md"), report, { flag: "wx" });
    ARTIFACT_PROJECTION_RETAINED_RUNS.set(root, { device: node.dev, inode: node.ino, reportHash: createHash("sha256").update(report).digest("hex"), name, startedAt });
  }
  const { ticketDir, workspace } = retainedRunParent === undefined ? { ticketDir: join(root, "🧪️tests"), workspace: join(root, "🧪️tests", "🧪️fixture") } : artifactProjectionFixtureLocations(root);
  const schemaPath = join(root, NORMALIZATION_SCHEMA_REL);
  mkdirSync(resolve(schemaPath, ".."), { recursive: true });
  mkdirSync(join(root, "compose"), { recursive: true });
  mkdirSync(workspace, { recursive: true });
  if (retainedRunParent !== undefined) mkdirSync(ticketDir, { recursive: true });
  writeFileSync(schemaPath, readFileSync(resolve(getWorkspaceRoot(), NORMALIZATION_SCHEMA_REL), "utf8"));
  writeFileSync(join(root, "compose", "keep.txt"), "opaque\n");
  normalizationWriteFiles(workspace, files);
  configure?.({ repoRoot: root, ticketDir, workspace });
  normalizationGit(root, ["init", "--quiet"]);
  normalizationGit(root, ["config", "user.name", "Semio Taxonomy Fixture"]);
  normalizationGit(root, ["config", "user.email", "taxonomy-fixture@invalid.example"]);
  normalizationGit(root, ["config", "commit.gpgsign", "false"]);
  normalizationGit(root, ["add", "--all"]);
  normalizationGit(root, ["commit", "--quiet", "-m", "taxonomy fixture"]);
  const head = readFileSync(join(root, ".git", "HEAD"), "utf8").trim();
  const baselineCommit = head.startsWith("ref: ") ? readFileSync(join(root, ".git", head.slice(5)), "utf8").trim() : head;
  const scope = relative(root, workspace).replaceAll("\\", "/");
  return {
    baselineCommit,
    opaqueDigest: opaqueTreeDigest(root, "compose"),
    options: { repoRoot: root, scope, ticketDir, workers: 1 },
    repoRoot: root,
    root,
    scope,
    ticketDir,
    workspace,
  };
}

export function normalizationPlan(fixture: NormalizationFixture): { inventory: TaxonomyInventory; plan: TaxonomyPlan } {
  const retained = ARTIFACT_PROJECTION_RETAINED_RUNS.has(fixture.root);
  const build = (progress?: (event: TaxonomyProgress) => void) => {
    const inventory = inventoryTaxonomy(progress ? { ...fixture.options, progress } : fixture.options);
    const plan = planTaxonomy(inventory, { baselineCommit: fixture.baselineCommit, excludedTreeDigests: [], progress });
    if (retained) artifactProjectionCaptureSubmittedPlan(fixture, plan);
    return { inventory, plan };
  };
  return retained ? artifactProjectionRecordedStage(fixture, "plan", build) : build();
}

export function normalizationWorkspaceSnapshot(workspace: string): Readonly<Record<string, string>> {
  const rows: [string, string][] = [];
  const visit = (relativePath: string): void => {
    const path = relativePath ? join(workspace, relativePath) : workspace;
    const stat = lstatSync(path), mode = (stat.mode & 0o7777).toString(8);
    if (stat.isSymbolicLink()) rows.push([relativePath || ".", `symlink|${mode}|${Buffer.from(readlinkSync(path)).toString("base64")}`]);
    else if (stat.isFile()) rows.push([relativePath || ".", `file|${mode}|${readFileSync(path).toString("base64")}`]);
    else if (stat.isDirectory()) {
      rows.push([relativePath || ".", `directory|${mode}`]);
      for (const name of readdirSync(path).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)))) visit(relativePath ? `${relativePath}/${name}` : name);
    } else rows.push([relativePath || ".", `other|${mode}`]);
  };
  visit("");
  return Object.fromEntries(rows);
}

export function normalizationMove(plan: TaxonomyPlan, sourceSuffix: string): TaxonomyPlan["moves"][number] {
  const move = plan.moves.find((candidate) => candidate.sourcePath.endsWith(sourceSuffix));
  if (!move) throw new Error(`normalization plan has no move for ${sourceSuffix}: ${canonicalJson(plan)}`);
  return move;
}

export function inScopeForTest(path: string, scope: string): boolean {
  const normalizedPath = path.normalize("NFC"), normalizedScope = scope.normalize("NFC");
  return normalizedPath === normalizedScope || normalizedPath.startsWith(`${normalizedScope}/`) || normalizedScope.startsWith(`${normalizedPath}/`);
}

/** 🧾️ Reduces inventory entries to the physical census fields governed by scoped admission. */
export function normalizationInventoryCensus(inventory: TaxonomyInventory): readonly Readonly<Record<string, unknown>>[] {
  return inventory.entries.filter(({ nodeKind }) => nodeKind !== "directory").map(({ sourcePath, nodeKind, mode, size, contentHash, symlinkTarget }) => ({ sourcePath, nodeKind, mode, size, contentHash, ...(symlinkTarget === undefined ? {} : { symlinkTarget }) }));
}

/** 📡️ Preserves the complete, duplicate-free inventory phase transition sequence. */
export function normalizationPhaseSequence(events: readonly TaxonomyProgress[]): readonly string[] {
  return events.map(({ phase }) => phase).filter((phase, index, rows) => index === 0 || phase !== rows[index - 1]);
}

export function artifactProjectionFixturePath(path: string): string {
  const marker = "/🗿️artifacts/";
  const index = path.indexOf(marker);
  if (index < 0) throw new Error(`Artifact projection path has no artifact marker: ${path}`);
  return path.slice(index + 1);
}

export function artifactProjectionSourceFiles(references: boolean, readLive: (path: string) => string = (path) => readFileSync(join(getWorkspaceRoot(), path), "utf8")): Record<string, string> {
  const files: Record<string, string> = {};
  for (const projection of ARTIFACT_PROJECTION_GOLDEN.projections) for (const node of projectionAuthorityNodes(projection, projection.contractId === "artifact-editor-command-bundle-v1" ? "authored-draw" : "live", readLive)) if (node.nodeKind === "file") files[artifactProjectionFixturePath(node.path)] = node.content!;
  if (references) {
    const cadRuntime = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts";
    const cadInteraction = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🕹️interaction/🦀️.rs";
    const interactionSpec = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️.rs";
    const interactionSpecTests = "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🧪️tests/🔬️unit/🦀️.rs";
    for (const path of [cadRuntime, cadInteraction, interactionSpec, interactionSpecTests]) files[artifactProjectionFixturePath(path)] = cadProjectionConsumerContent(readLive(path));
    files[DRAW_SOURCE_SCENARIO.cadConsumerMount.path] = readLive(DRAW_SOURCE_SCENARIO.cadConsumerMount.path);
    const draw = projectionGolden("artifact-editor-command-bundle-v1");
    for (const row of DRAW_SOURCE_SCENARIO.consumers) files[row.path] = drawSourceScenarioContent(row.content, draw);
    files[DRAW_SOURCE_SCENARIO.cargoModuleRoot.path] = DRAW_SOURCE_SCENARIO.cargoModuleRoot.content;
  }
  return files;
}

export type ArtifactProducerInput = Readonly<{ content: string; mode: number; sha256: string; origin: "current-compiler-context" | "authored-scenario" }>;

/** 🔒️ Reads one exact current compiler input after checking every no-follow path component. */
export function artifactProjectionProducerInput(path: string): ArtifactProducerInput {
  if (!path || path.startsWith("/") || path.includes("\\") || path.split("/").some((part) => !part || part === "." || part === "..") || path === "compose" || path.startsWith("compose/") || path === "temp/compose" || path.startsWith("temp/compose/")) throw new Error("Producer input must be an exact nonopaque repository-relative file");
  const root = getWorkspaceRoot(), segments = path.split("/");
  for (let index = 1; index <= segments.length; index++) {
    const selected = join(root, ...segments.slice(0, index)), stat = lstatSync(selected);
    if (stat.isSymbolicLink() || (index === segments.length ? !stat.isFile() : !stat.isDirectory())) throw new Error("Producer input must have no-follow regular ancestry: " + path);
  }
  const absolute = join(root, path), before = lstatSync(absolute), bytes = readFileSync(absolute), after = lstatSync(absolute);
  if (!after.isFile() || after.isSymbolicLink() || before.dev !== after.dev || before.ino !== after.ino || before.mode !== after.mode || before.mtimeMs !== after.mtimeMs || before.size !== after.size || bytes.byteLength !== after.size) throw new Error("Producer input changed during capture: " + path);
  const content = bytes.toString("utf8");
  if (!Buffer.from(content).equals(bytes)) throw new Error("Producer input is not exact UTF-8: " + path);
  return { content, mode: after.mode & 0o7777, sha256: createHash("sha256").update(bytes).digest("hex"), origin: "current-compiler-context" };
}

/** 📦️ Captures current producer inputs from exact contracts and static imports without repository traversal. */
export function artifactProjectionProducerInputs(taxonomy: Taxonomy, readInput: (path: string) => ArtifactProducerInput = artifactProjectionProducerInput, context: DrawSourceScenario["producerContext"] = DRAW_SOURCE_SCENARIO.producerContext) {
  const contract = taxonomy.generatorContracts[context.generatorId], authority = contract?.inputDiscovery;
  if (!contract?.ownerPath || !authority || authority.kind !== "registry-catalog") throw new Error("Authored Draw producer requires exact registry catalog authority");
  const files: Record<string, ArtifactProducerInput> = {};
  const read = (path: string): ArtifactProducerInput => files[path] ??= readInput(path);
  for (const path of [...context.workspaceInputs, ...context.runtimeData, ...contract.inputPatterns]) {
    if (/[*?[\]]/u.test(path)) throw new Error("Draw producer context requires exact declared input files");
    read(path);
  }
  for (const catalog of context.runtimeReceiptCatalogs) {
    const receipts = (JSON.parse(read(catalog.manifestPath).content) as { receipts: Record<string, unknown>[] }).receipts;
    for (const receipt of receipts) {
      const path = String(receipt[catalog.receiptPathField]);
      if (posix.normalize(path) !== path || !path.startsWith(`${catalog.sourceRoot}/`)) throw new Error("Runtime receipt escapes its authored source owner: " + path);
      read(path);
    }
  }
  const nx = JSON.parse(read("nx.json").content) as { plugins: { plugin: string }[] };
  const pending: { path: string; role: RegistryCompilerInputRole }[] = [...context.compilerRoots, ...context.runtimeModules, ...authority.implementationEntryPaths, ...nx.plugins.filter(({ plugin }) => plugin.startsWith("./")).map(({ plugin }) => plugin.slice(2))].map((path) => ({ path, role: "implementation-entry" }));
  const modules = new Map<string, { path: string; kind: "module" | "json-data"; imports: readonly string[] }>();
  while (pending.length) {
    const { path, role } = pending.shift()!, previous = modules.get(path);
    if (previous) {
      if (role === "implementation-entry" && previous.kind === "json-data") registryCompilerInputDependencies(files[path]!.content, path, role);
      continue;
    }
    const input = read(path), dependencies = registryCompilerInputDependencies(input.content, path, role), { imports } = dependencies;
    modules.set(path, { path, ...dependencies });
    for (const specifier of imports) {
      if (specifier.startsWith("node:") || specifier.startsWith("bun:")) continue;
      if (specifier.startsWith(".")) pending.push({ path: posix.normalize(posix.join(posix.dirname(path), specifier)), role: "static-import" });
      else {
        const binding = authority.workspaceImports[specifier];
        if (!binding) throw new Error("Producer static import lacks exact workspace authority: " + path + " -> " + specifier);
        read(binding.manifestPath);
        pending.push({ path: binding.entryPath, role: "implementation-entry" });
      }
    }
  }
  return { files, modules: [...modules.values()].sort((left, right) => projectionByteSort(left.path, right.path)), bindings: authority.workspaceImports };
}

/** 🏭️ Produces the fixture baseline through the unchanged Nx generator and freshness commands. */
function artifactProjectionPrepareProducer(repoRoot: string, taxonomy: Taxonomy, captured: ReturnType<typeof artifactProjectionProducerInputs>): void {
  const context = DRAW_SOURCE_SCENARIO.producerContext, contract = taxonomy.generatorContracts[context.generatorId]!;
  const reportRoot = join(repoRoot, "📇️producer");
  mkdirSync(reportRoot);
  const reportPath = join(reportRoot, "📝️.md"), evidence: { target: string; status: number | null; signal: string | null; stdout: string; stderr: string }[] = [];
  const inputs = Object.entries(captured.files).sort(([left], [right]) => projectionByteSort(left, right)).map(([path, input]) => ({ path, origin: input.origin, mode: input.mode, bytes: Buffer.byteLength(input.content), sourceSha256: input.sha256, materializedSha256: createHash("sha256").update(readFileSync(join(repoRoot, path))).digest("hex") }));
  const writeReport = (): void => { const fence = String.fromCharCode(96).repeat(3); writeFileSync(reportPath, "# Draw Fixture Producer Context\n\nCurrent exact compiler inputs are copied into this isolated repository; source scenario members remain independently authored. No production generator runs in the real workspace.\n\n" + fence + "json\n" + JSON.stringify({ inputs, modules: captured.modules, evidence }, null, 2) + "\n" + fence + "\n"); };
  writeReport();
  for (const target of [contract.target!, contract.checkTarget!]) {
    const result = spawnSync(process.execPath, ["repo", "run", target, "--skip-nx-cache"], { cwd: repoRoot, encoding: "utf8", timeout: 60_000, maxBuffer: 8 * 1024 * 1024, env: { ...process.env, REPO_ROOT: repoRoot, NX_DAEMON: "false" } });
    evidence.push({ target, status: result.status, signal: result.signal, stdout: result.stdout, stderr: result.stderr });
    writeReport();
    if (result.error || result.status !== 0) throw new Error("Actual Draw fixture producer failed: " + target + "\n" + (result.error?.message ?? "") + result.stderr + result.stdout);
  }
  const outputNodes = contract.outputRoots.map(({ path }) => noFollowTreeDigest(repoRoot, path));
  expect(outputNodes.reduce((count, row) => count + row.files + row.directories, 0)).toBe(context.registryNodeCount);
  expect(outputNodes.every((row) => row.symlinks === 0 && row.others === 0)).toBe(true);
  for (const [path, input] of Object.entries(captured.files)) {
    if (input.origin === "authored-scenario") continue;
    const current = artifactProjectionProducerInput(path);
    if (current.sha256 !== input.sha256 || current.mode !== input.mode) throw new Error("Live producer input changed after capture: " + path);
  }
}

/** 🏗️ Materializes the permanent language-neutral CAD/Draw authority under one isolated normalization scope. */
export function artifactProjectionNormalizationFixture(name: string, references = false): NormalizationFixture {
  const files = artifactProjectionSourceFiles(references);
  return normalizationFixture(`artifact-projection-${name}`, files, ({ repoRoot, ticketDir, workspace }) => {
    const schemaPath = join(repoRoot, NORMALIZATION_SCHEMA_REL);
    const schema = drawScenarioTaxonomy(JSON.parse(readFileSync(schemaPath, "utf8")));
    if (!references) {
      writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
      return;
    }
    expect(lstatSync(ticketDir).isDirectory()).toBe(true);
    expect(lstatSync(ticketDir).isSymbolicLink()).toBe(false);
    expect(existsSync(join(repoRoot, ".git"))).toBe(false);
    const scope = relative(repoRoot, workspace).replaceAll("\\", "/");
    const producer = artifactProjectionProducerInputs(schema);
    if (producer.files[NORMALIZATION_SCHEMA_REL]!.content !== readFileSync(schemaPath, "utf8")) throw new Error("Producer taxonomy changed during fixture preparation");
    normalizationWriteFiles(repoRoot, Object.fromEntries(Object.entries(producer.files).map(([path, input]) => [path, input.content])));
    normalizationWriteFiles(repoRoot, Object.fromEntries(DRAW_SOURCE_SCENARIO.catalogContext.map(({ path, content }) => [path, content])));
    for (const [path, input] of Object.entries(producer.files)) chmodSync(join(repoRoot, path), input.mode);
    const links = [...Object.entries(producer.bindings).map(([name, binding]) => [name, join(repoRoot, dirname(binding.manifestPath))] as const), ...DRAW_SOURCE_SCENARIO.producerContext.runtimePackages.map((name) => [name, realpathSync(join(getWorkspaceRoot(), "node_modules", name))] as const)];
    for (const [name, target] of links) {
      const destination = join(repoRoot, "node_modules", name);
      mkdirSync(dirname(destination), { recursive: true });
      symlinkSync(target, destination, process.platform === "win32" ? "junction" : "dir");
    }
    if (!schema.semanticPathProjectionReferenceConsumerContracts["cad-spatial-kernel-geometry"]!.sourcePathIdentities.includes(DRAW_SOURCE_SCENARIO.cadConsumerMount.path)) throw new Error("Authored CAD consumer mount has no exact source identity");
    const fixtureConsumerPaths: Readonly<Record<string, readonly string[]>> = {
      "draw-workspace-cargo": [`${scope}/Cargo.toml`],
      "draw-dependency-registry": [`${scope}/🔒️dependencies.json`],
      "draw-workspace-script": [`${scope}/📜️script.ts`],
      "cad-spatial-kernel-geometry": [`${scope}/${DRAW_SOURCE_SCENARIO.cadConsumerMount.path}`],
    };
    const escaped = (value: string): string => value.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
    for (const [id, paths] of Object.entries(fixtureConsumerPaths)) {
      const contract = schema.semanticPathProjectionReferenceConsumerContracts[id]!;
      contract.sourcePathIdentities = [...paths];
      contract.sourcePathPattern = `^(?:${paths.map(escaped).join("|")})$`;
    }
    schema.fixedFilenameContracts["fixture-workspace-cargo"] = { pathPattern: `${scope}/Cargo.toml`, authority: "normalization fixture", reason: "The isolated fixture relocates the schema-owned workspace Cargo consumer without changing its fixed identity.", configurability: "unconfigurable", scope: { kind: "exact-path", path: `${scope}/Cargo.toml` }, verification: "normalization fixture inventory", expires: null };
    writeFileSync(schemaPath, `${JSON.stringify(schema, null, 2)}\n`);
    const cad = projectionGolden("artifact-example-model-catalog-v1");
    const draw = projectionGolden("artifact-editor-command-bundle-v1");
    const target = join(workspace, artifactProjectionFixturePath(cad.mappings[0]!.sourcePath));
    const manifest = JSON.parse(readFileSync(target, "utf8")) as Record<string, unknown>;
    manifest.projectionReferences = {
      cadSourceRoot: `${scope}/${artifactProjectionFixturePath(cad.sourceRoot)}`,
      drawSourceFile: `${scope}/${artifactProjectionFixturePath(draw.mappings[0]!.sourcePath)}`,
    };
    writeFileSync(target, `${JSON.stringify(manifest, null, 2)}\n`);
    const drawSourceRoot = artifactProjectionFixturePath(draw.sourceRoot);
    for (const mapping of draw.mappings.filter(({ sourcePath }) => sourcePath.endsWith("📋️project.json"))) {
      const project = join(workspace, artifactProjectionFixturePath(mapping.sourcePath));
      writeFileSync(project, readFileSync(project, "utf8").replaceAll(`{workspaceRoot}/${draw.sourceRoot}`, `{workspaceRoot}/${scope}/${drawSourceRoot}`).replaceAll(draw.sourceRoot, `${scope}/${drawSourceRoot}`));
    }
    for (const relativePath of ["🔒️dependencies.json", "Cargo.toml"]) {
      const target = join(workspace, relativePath);
      writeFileSync(target, readFileSync(target, "utf8").replaceAll(draw.sourceRoot, `${scope}/${drawSourceRoot}`));
    }
    const rootScript = join(workspace, "📜️script.ts");
    writeFileSync(rootScript, readFileSync(rootScript, "utf8").replaceAll("__SCOPE__", scope));
    artifactProjectionPrepareProducer(repoRoot, schema, producer);
  }, ARTIFACT_PROJECTION_RUN_PARENT);
}

export function scopedUnrelatedProjectionStaleFixture(name: string): Readonly<{ fixture: NormalizationFixture; externalPath: string; staleToken: string }> {
  const externalPath = `🧪️external-${name}/🗿️artifacts/📐️cad/📝️.md`;
  let staleToken = "";
  const fixture = normalizationFixture(`scoped-stale-${name}`, { "🟦️.ts": "export const scoped = true;\n" }, ({ repoRoot }) => {
    const schema = JSON.parse(readFileSync(join(repoRoot, NORMALIZATION_SCHEMA_REL), "utf8")) as { semanticPathProjectionReferenceConsumerContracts: Record<string, { projectionContractId: string; staleMarkers: string[] }> };
    staleToken = Object.values(schema.semanticPathProjectionReferenceConsumerContracts).find((contract) => contract.projectionContractId === "artifact-example-model-catalog-v1")!.staleMarkers[0]!;
    normalizationWriteFiles(repoRoot, { [externalPath]: "# Unrelated clean owner\n" });
  });
  return { fixture, externalPath, staleToken };
}

export function expectNormalizationApplyFailure(run: () => TaxonomyApplyResult): void {
  let failed = false;
  try {
    const result = run();
    failed = !/^(?:applied|committed|complete)$/u.test(String(result.state));
  } catch {
    failed = true;
  }
  expect(failed).toBe(true);
}
