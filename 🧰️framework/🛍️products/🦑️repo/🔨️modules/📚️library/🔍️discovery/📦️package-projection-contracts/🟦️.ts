import { type GeneratorPreviewProgressPolicyV1 } from "../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import { leadingEmojiIdentity } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🧩️grapheme/🟦️.ts";
import { createHash } from "node:crypto";
import { basename, posix } from "node:path";
import type { CurrentJcoPackageDestination, GeneratorOutputRoot, GeneratorOwnership, GeneratorProjectionActivation, RegistryCatalogInputDiscovery } from "../📜️taxonomy-contracts/🟦️.ts";

/** 🏗️ Exact package artifacts and bounded projected inputs share one producer authority. */
export interface SemanticPackageGeneration {
  readonly kind: "wgpu-package-artifacts";
  readonly previewInput: { readonly protocol: "package-projected-inputs-v1"; readonly maxBytes: 67108864; readonly maxOperations: 200000 };
  readonly catalogPath: string;
  readonly catalogSha256: string;
  readonly browserProfile: SemanticPackageBrowserProfile;
}

/** ⚙️ Schema-owned generator identity; runnable commands derive only from exact build tasks. */
export interface GeneratorContract {
  readonly ownership: GeneratorOwnership;
  readonly ownerPath: string | null;
  readonly target: string | null;
  readonly previewTarget?: string;
  readonly previewArguments?: readonly string[];
  readonly previewScript?: string;
  readonly previewProgress?: GeneratorPreviewProgressPolicyV1;
  readonly previewLimits?: { readonly maxOutputBytes: number; readonly timeoutMs: number };
  readonly compilerInputManifest?: { readonly kind: "compiler-input-manifest-v1"; readonly manifestOutputPath: string; readonly manifestSchemaPath: string; readonly staticAuthorityPath: string; readonly maxFiles: number };
  readonly checkTarget?: string;
  readonly nativeConsumers?: readonly string[];
  readonly inputPatterns: readonly string[];
  readonly inputDiscovery?: RegistryCatalogInputDiscovery;
  readonly packageGeneration?: SemanticPackageGeneration;
  readonly currentPackageDestination?: CurrentJcoPackageDestination;
  readonly projectionActivation?: GeneratorProjectionActivation;
  readonly outputRoots: readonly GeneratorOutputRoot[];
  readonly reason: string;
}

/** 🏗️ Verifies known Nx identities and required tracked outputs against live workspace metadata. */
/** 🧾️ Frozen leaf ownership and byte preimage, independent of implementation language. */
export interface OwnerProjectionEntry {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly ownerEvidenceId: string;
  readonly disposition: "attribution-relocate" | "configurable-owner-license-relocate" | "fixed" | "generated-evidence-relocate" | "owner-documentation-relocate";
  readonly fixedContractId: "bun-package-license" | "bun-package-readme" | null;
  readonly projectionContractId: "exact-owner-license-projection" | "exact-owner-readme-projection" | null;
  readonly referenceOwnerIds: readonly string[];
  readonly generatorOwnerId: string | null;
  readonly preimage: Readonly<{ sha256: string; mode: string; size: number }>;
}

/** 📚️ One digest-locked owner catalog, including its concrete consumers and owner evidence. */
export interface OwnerProjectionPolicy {
  readonly schemaVersion: 1;
  readonly kind: "owner-projection-policy";
  readonly entries: readonly OwnerProjectionEntry[];
  readonly ownerEvidence: Readonly<Record<string, Readonly<{ kind: string; evidencePaths: readonly string[]; expectedPackageName?: string; private?: boolean }>>>;
  readonly referenceOwners: Readonly<Record<string, Readonly<{ kind: string; ownerPath: string }>>>;
  readonly generatorOwners: Readonly<Record<string, Readonly<{ ownerPath: string; target: string; currentOutputPath: string; requiredOutputPath: string }>>>;
}

/** 🧷️ One exact admitted Cargo-package source leaf and its semantic destination. */
export interface SemanticPackageProjectionMapping {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly sourceHash: string;
  readonly sourceSize: number;
  readonly disposition: "metadata" | "implementation" | "adapter" | "generated" | "tool-metadata";
  readonly sourceRole: "implementation" | "unresolved" | null;
}

/** 🔌️ Deterministic package entry derived from its externally configured semantic target. */
export interface SemanticPackageAdapter {
  readonly id: string;
  readonly path: string;
  readonly language: "rust" | "typescript";
  readonly expectedRole: "declaration";
  readonly targetPaths: readonly string[];
  readonly content: string;
}

/** 🧷️ Exact source-owned replacement or registration merge, never a generated-output overwrite. */
export interface SemanticPackageSourceSplice {
  readonly id: "library-adapter-replacement" | "renderer-registration-merge";
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly oldValue: string;
  readonly newValue: string;
}

/** 📇️ Source-hash-bound semantic registration output outside every package boundary. */
export interface SemanticPackageDerivedLeaf {
  readonly id: "wgpu-renderer-registration";
  readonly path: string;
  readonly originSourcePath: string;
  readonly originSourceHash: string;
  readonly language: "rust";
  readonly expectedRole: "implementation";
  readonly content: string;
}

/** ♻️ Exact obsolete generated source; canonical bytes are owned independently by its preview. */
export interface SemanticPackageGeneratedSourceRetirement {
  readonly sourcePath: string;
  readonly destinationPath: string;
  readonly generatorContractId: "wgpu-frame-worker";
  readonly sourceMode: 420;
}

/** 🧷️ Reviewed authored fragment, bound to its complete package source preimage. */
export interface SemanticPackageAuthoredSourceFragment {
  readonly id: string;
  readonly kind: "reference-path" | "cargo-runtime-coordinate" | "tool-producer";
  readonly sourcePath: string;
  readonly oldValue: string;
  readonly newValue: string;
}

/** 🌐️ Closed browser compiler profile with exact module and workspace import authority. */
export interface SemanticPackageBrowserProfile {
  readonly schemaVersion: 1;
  readonly kind: "wgpu-browser-esm-v1";
  readonly inlineTestDefine: "undefined";
  readonly ownerPath: string;
  readonly entries: readonly Readonly<{ id: "frame-worker" | "browser-boot" | "renderer-boot"; sourceRelativePath: string; outputRelativePath: string; inclusion: "tracked" | "ignored" }>[];
  readonly workspaceImports: Readonly<Record<string, Readonly<{ manifestPath: string; entryPath: string }>>>;
  readonly sourceModulePaths: readonly string[];
}

/** 🪪️ Current WGPU package declarations, independent of frozen projection preimages. */
export interface CanonicalWgpuPackageCatalog {
  readonly $schema: "https://json.schemas.assets.semio-tech.com/os/renderer/component.json#/$defs/RendererPackageCatalogV1";
  readonly schemaVersion: 1;
  readonly kind: "canonical-wgpu-package";
  readonly ownerPath: string;
  readonly packageRelativePath: "📦️packages/🦀️rust";
  readonly nodePackageRelativePath: "📦️packages/🟦️typescript";
  readonly identity: Readonly<{ cargoPackageName: string; nodePackageName: string; projectName: string }>;
  readonly entryPaths: Readonly<{ cargoLibrary: string; cargoBinary: string; cargoBuild: "build.rs"; nodeLibrary: string }>;
  readonly artifacts: readonly Readonly<{ id: string; relativePath: string; targetRelativePath: string | null; language: "rust" | "typescript"; role: "declaration" | "implementation"; content: string }>[];
}

/** 🏠️ One exact Cargo identity; standalone fixtures are not inferred from directory basenames. */
export interface SemanticPackageProjectionCase {
  readonly id: "wgpu-renderer" | "jcoprobe-guest";
  readonly sourceRoot: string;
  readonly destinationRoot: string;
  readonly semanticOwnerRoot: string;
  readonly workspaceKind: "repository" | "standalone";
  readonly identity: Readonly<{ cargoPackageName: string; nodePackageName?: string; projectName?: string }>;
  readonly requiredManifestEvidence: Readonly<Record<string, unknown>>;
  readonly ignoredSourcePatterns: readonly string[];
  readonly mappings: readonly SemanticPackageProjectionMapping[];
  readonly adapters: readonly SemanticPackageAdapter[];
  readonly sourceSplices: readonly SemanticPackageSourceSplice[];
  readonly derivedLeaves: readonly SemanticPackageDerivedLeaf[];
  readonly joinedPathBindings: readonly SemanticPackageJoinedPathBinding[];
  readonly generatedSourceRetirements: readonly SemanticPackageGeneratedSourceRetirement[];
  readonly authoredSourceFragments: readonly SemanticPackageAuthoredSourceFragment[];
}

export function semanticPackageAuthoredFragmentProblem(owner: SemanticPackageProjectionCase): string | null {
  const fragments = owner.authoredSourceFragments;
  if (!Array.isArray(fragments) || fragments.length > 64 || new Set(fragments.map((fragment) => fragment.id)).size !== fragments.length) return "invalid or duplicated declaration";
  for (const fragment of fragments) {
    const mapping = owner.mappings.find((mapping) => mapping.sourcePath === fragment.sourcePath);
    if (Object.keys(fragment).sort().join("\0") !== "id\0kind\0newValue\0oldValue\0sourcePath" || !/^[a-z][a-z0-9-]*$/u.test(fragment.id) || !["reference-path", "cargo-runtime-coordinate", "tool-producer"].includes(fragment.kind) || !exactOwnerPath(fragment.sourcePath) || !mapping || mapping.disposition === "generated" || typeof fragment.oldValue !== "string" || !fragment.oldValue || typeof fragment.newValue !== "string" || !fragment.newValue || Buffer.byteLength(fragment.oldValue) > 262144 || Buffer.byteLength(fragment.newValue) > 262144 || fragment.kind === "cargo-runtime-coordinate" && !fragment.sourcePath.endsWith(".rs")) return "unowned or malformed declaration";
  }
  return null;
}

/** 🔐️ Projects only exact reviewed source spans; all other program bytes remain authored. */
export function semanticPackageAuthoredFragmentReferences(
  facts: Readonly<{ path: string; content: string; layout: "source" | "destination" }>,
  owner: SemanticPackageProjectionCase,
): Readonly<{ references: readonly SemanticPackageJoinedPathReference[]; problems: readonly string[] }> {
  const reject = (message: string) => ({ references: [], problems: ["Nested Cargo authored fragment: " + message] });
  const malformed = semanticPackageAuthoredFragmentProblem(owner);
  if (malformed) return reject(malformed);
  const fragments = owner.authoredSourceFragments;
  const mapping = owner.mappings.find((mapping) => facts.path === (facts.layout === "source" ? mapping.sourcePath : mapping.destinationPath));
  if (!mapping || !["source", "destination"].includes(facts.layout)) return reject("unknown source or canonical owner");
  if (facts.layout === "source" && (createHash("sha256").update(facts.content).digest("hex") !== mapping.sourceHash || Buffer.byteLength(facts.content) !== mapping.sourceSize)) return reject("complete source preimage changed");
  const references: SemanticPackageJoinedPathReference[] = [];
  for (const fragment of fragments.filter((fragment) => fragment.sourcePath === mapping.sourcePath)) {
    const value = facts.layout === "source" ? fragment.oldValue : fragment.newValue, start = facts.content.indexOf(value);
    if (start < 0 || facts.content.indexOf(value, start + value.length) >= 0) return reject("missing or repeated span " + fragment.id);
    references.push({ start, end: start + value.length, oldValue: value, newValue: fragment.newValue, targetSourcePath: mapping.sourcePath });
  }
  references.sort((left, right) => left.start - right.start || left.end - right.end);
  if (references.some((reference, index) => index > 0 && reference.start < references[index - 1]!.end)) return reject("overlapping spans");
  return { references: facts.layout === "source" ? references : [], problems: [] };
}

/** 🧵️ Exact local directory binding and its source-owned file reads. */
export interface SemanticPackageJoinedPathBinding {
  readonly kind: "source-owned-joined-path-bindings";
  readonly id: string;
  readonly consumerRelativePath: string;
  readonly sourceDirectoryRelativePath: string;
  readonly destinationDirectory: "semantic-owner";
  readonly rootBinding: string;
  readonly directoryBinding: string;
  readonly reads: readonly Readonly<{ binding: string; relativePath: string }>[];
}

/** 🧷️ One exact literal operand; executable source bytes are never synthesized. */
export interface SemanticPackageJoinedPathReference {
  readonly start: number;
  readonly end: number;
  readonly oldValue: string;
  readonly newValue: string;
  readonly targetSourcePath: string;
}

export function semanticPackageJoinedPathBindingRecords(contract: SemanticPackageJoinedPathBinding, owner: SemanticPackageProjectionCase) {
  const reject = (message: string): never => { throw new Error("Nested Cargo joined-path authority: " + message); };
  const keys = (value: object, expected: readonly string[]) => JSON.stringify(Object.keys(value).sort()) === JSON.stringify([...expected].sort());
  const identifier = (value: unknown): value is string => typeof value === "string" && /^[A-Za-z_$][\w$]*$/u.test(value);
  if (!contract || !keys(contract, ["kind", "id", "consumerRelativePath", "sourceDirectoryRelativePath", "destinationDirectory", "rootBinding", "directoryBinding", "reads"]) || contract.kind !== "source-owned-joined-path-bindings" || typeof contract.id !== "string" || !/^[a-z][a-z0-9-]*$/u.test(contract.id) || contract.destinationDirectory !== "semantic-owner" || !identifier(contract.rootBinding) || !identifier(contract.directoryBinding) || !exactOwnerPath(contract.consumerRelativePath) || !exactOwnerPath(contract.sourceDirectoryRelativePath) || !Array.isArray(contract.reads) || contract.reads.length !== 2) return reject("invalid binding contract");
  if (contract.reads.some((entry) => !entry || !keys(entry, ["binding", "relativePath"]) || !identifier(entry.binding) || !exactOwnerPath(entry.relativePath)) || new Set([contract.rootBinding, contract.directoryBinding, ...contract.reads.map((entry) => entry.binding)]).size !== 4) return reject("ambiguous read binding");
  const consumers = owner.mappings.filter((mapping) => mapping.sourcePath === owner.sourceRoot + "/" + contract.consumerRelativePath);
  if (consumers.length !== 1) return reject("consumer must have one exact mapped owner");
  const consumer = consumers[0]!, sourceDirectory = owner.sourceRoot + "/" + contract.sourceDirectoryRelativePath;
  const reads = contract.reads.map((entry) => owner.mappings.filter((mapping) => mapping.sourcePath === sourceDirectory + "/" + entry.relativePath));
  if (reads.some((mappings) => mappings.length !== 1 || mappings[0]!.sourceRole !== "implementation" || !mappings[0]!.destinationPath.startsWith(owner.semanticOwnerRoot + "/")) || new Set(reads.map((mappings) => mappings[0]!.sourcePath)).size !== 2) return reject("read target must have one exact implementation owner");
  return { consumer, sourceDirectory, reads };
}

/** 🔐️ Proves three reviewed literal operands using a complete catalog source preimage. */
export function semanticPackageJoinedPathReferenceAuthority(
  facts: Readonly<{ path: string; content: string; layout: "source" | "destination" }>,
  contract: SemanticPackageJoinedPathBinding,
  owner: SemanticPackageProjectionCase,
): Readonly<{ references: readonly SemanticPackageJoinedPathReference[]; problems: readonly string[] }> {
  const reject = (message: string) => ({ references: [], problems: ["Nested Cargo joined-path authority: " + message] });
  let records: ReturnType<typeof semanticPackageJoinedPathBindingRecords>;
  try { records = semanticPackageJoinedPathBindingRecords(contract, owner); } catch (error) { return { references: [], problems: [String(error)] }; }
  const { consumer, sourceDirectory, reads } = records;
  if (facts.layout !== "source" && facts.layout !== "destination" || facts.path !== (facts.layout === "source" ? consumer.sourcePath : consumer.destinationPath)) return reject("consumer path or phase mismatch");
  if (facts.layout === "source" && (createHash("sha256").update(facts.content).digest("hex") !== consumer.sourceHash || Buffer.byteLength(facts.content) !== consumer.sourceSize)) return reject("complete source preimage changed");
  const imports = ['import { readFileSync } from "node:fs";', 'import { dirname, join } from "node:path";', 'import { fileURLToPath } from "node:url";'];
  if (!facts.content.startsWith(imports.join("\n") + "\n") || imports.some((statement) => facts.content.split(statement).length !== 2)) return reject("binding imports changed");
  const oldValues = [posix.relative(posix.dirname(consumer.sourcePath), sourceDirectory), ...contract.reads.map((entry) => entry.relativePath)];
  const newValues = [posix.relative(posix.dirname(consumer.destinationPath), owner.semanticOwnerRoot) || ".", ...reads.map((mappings) => posix.relative(owner.semanticOwnerRoot, mappings[0]!.destinationPath))];
  const values = facts.layout === "source" ? oldValues : newValues;
  const statements = [
    `const ${contract.rootBinding} = dirname(fileURLToPath(import.meta.url));`,
    `const ${contract.directoryBinding} = join(${contract.rootBinding}, ${JSON.stringify(values[0])});`,
    ...contract.reads.map((entry, index) => `const ${entry.binding} = readFileSync(join(${contract.directoryBinding}, ${JSON.stringify(values[index + 1])}), "utf8");`),
  ];
  const fragment = statements.join("\n    "), fragmentStart = facts.content.indexOf(fragment);
  if (fragmentStart < 0 || facts.content.indexOf(fragment, fragmentStart + fragment.length) >= 0 || statements.some((statement) => facts.content.split(statement).length !== 2)) return reject("reviewed consecutive binding statements changed");
  if (facts.layout === "destination") return { references: [], problems: [] };
  const references = statements.slice(1).map((statement, index) => {
    const start = fragmentStart + fragment.indexOf(statement) + statement.indexOf(JSON.stringify(oldValues[index])) + 1;
    return { start, end: start + oldValues[index]!.length, oldValue: oldValues[index]!, newValue: newValues[index]!, targetSourcePath: reads[Math.max(0, index - 1)]![0]!.sourcePath };
  });
  return { references, problems: [] };
}

/** 📚️ Digest-locked nested package catalog and exact live reference owners. */
export interface SemanticPackageProjectionCatalog {
  readonly schemaVersion: 1;
  readonly contractKind: "exact-nested-cargo-package-catalog";
  readonly packages: readonly SemanticPackageProjectionCase[];
  readonly referenceConsumers: readonly Readonly<{ packageId: string; path: string; destinationPath?: string; transformId: string; occurrenceCount: number; ownership: "authored" | "generated"; generatorOwnerPaths?: readonly string[] }>[];
  readonly referenceTokenTransforms: Readonly<Record<string, Readonly<{ sourceToken: string; destinationToken: string }>>>;
}

/** 🪪️ Rejects noncanonical, colliding or historical coordinates for the single current JCO package. */
export function parseCurrentJcoPackageDestination(input: unknown): CurrentJcoPackageDestination {
  const semanticOwnerRoot = "🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/👽️guest";
  const packageRoot = semanticOwnerRoot + "/📦️packages/🦀️rust";
  const expected: CurrentJcoPackageDestination = { kind: "jco-canonical-package-v1", packageId: "jcoprobe-guest", semanticOwnerRoot, packageRoot, cargoManifestPath: packageRoot + "/Cargo.toml", cargoLockPath: packageRoot + "/Cargo.lock", componentPath: semanticOwnerRoot + "/🧩️component/🦀️.rs", witPath: packageRoot + "/🧬️schema/📜️world.wit", adapterPath: packageRoot + "/📚️library/🦀️.rs" };
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Current JCO destination must be an object");
  const value = input as Record<string, unknown>;
  if (Object.keys(value).sort().join("\0") !== Object.keys(expected).sort().join("\0") || Object.entries(expected).some(([key, path]) => value[key] !== path)) throw new Error("Current JCO destination differs from its explicit handpicked authority");
  const paths = [expected.cargoManifestPath, expected.cargoLockPath, expected.componentPath, expected.witPath, expected.adapterPath];
  const folded = paths.map((path) => path.normalize("NFC").replaceAll("\ufe0f", "").toLocaleLowerCase("und"));
  if (paths.some((path) => !exactOwnerPath(path)) || new Set(folded).size !== paths.length || folded.some((path, index) => folded.some((other, otherIndex) => index !== otherIndex && other.startsWith(path + "/")))) throw new Error("Current JCO paths are invalid or collide");
  return expected;
}

export function exactOwnerPath(path: unknown): path is string {
  return typeof path === "string" && path.length > 0 && path === path.normalize("NFC") && !path.startsWith("/") && !path.includes("\\") && !/[\u0000-\u001f]/u.test(path) && path.split("/").every((part) => part !== "" && part !== "." && part !== "..") && !["compose", "temp/compose"].some((root) => path === root || path.startsWith(root + "/"));
}

/** 📖️ Resolves the externally meaningful README/LICENSE basename beneath repository emoji identity. */
export function semanticOwnedSourceBasename(path: string): string {
  return leadingEmojiIdentity(basename(path)).rest;
}
