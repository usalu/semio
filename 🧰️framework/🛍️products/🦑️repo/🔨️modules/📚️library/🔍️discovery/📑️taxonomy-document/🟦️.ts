import type { AreaState, ConfigurableEntryContract, Ecosystem, FileKindResolutionRuleSpec, FileKindSpec, FixedDirectoryContract, FixedFilenameContract, FixedFilenameRejectionContract, MutationCatalogProjectionContractIds, PackageBoundaryProfile, PackageBoundaryRule, PackageGlueGrammarSpec, PackageMaturity, PackageSourceDisposition, PathEmojiPolicy, PathExclusion, RustEntryPathRules, ScopedFileKindSpec, SemanticCollectionSpec, SemanticDescendantContract, SemanticDirectoryKindSpec, SemanticDirectoryMemberKindSpec, SemanticOwnedFileProjectionContract, SemanticOwnerLevel, SemanticPackageProjectionContract, SemanticPathProjectionCatalogContract, SemanticPathProjectionContract, SemanticPathProjectionProfileRenderer, SemanticPathProjectionReferenceConsumerContract, SemanticPolicyStateCoordinateContract, SemanticProjectedMemberKindSpec, TargetSpec } from "../📜️taxonomy-contracts/🟦️.ts";
import type { GeneratorContract } from "../📦️package-projection-contracts/🟦️.ts";

/** 📜️ Exact immutable Markdown source coordinates with bounded block and inline syntax. */
export interface FrozenMarkdownCoordinateEvidenceContract {
  readonly path: string;
  readonly grammar: "frozen-markdown-source-coordinates-v1";
  readonly sha256: string;
  readonly coordinates: readonly Readonly<{ start: number; end: number; kind: "source"; form: "inline-code" | "path-list-item"; valueSha256: string }>[];
}

/** 🧷️ Validates exact Markdown declarations without reading or discovering evidence. */
export function validateFrozenMarkdownCoordinateEvidenceContracts(value: unknown): string[] {
  const problems: string[] = [], paths = new Set<string>();
  const object = (entry: unknown): entry is Record<string, unknown> => entry !== null && typeof entry === "object" && !Array.isArray(entry);
  if (!object(value)) return ["frozenMarkdownCoordinateEvidenceContracts must be an object."];
  for (const [id, row] of Object.entries(value)) {
    const label = `frozenMarkdownCoordinateEvidenceContracts.${id}`;
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`${label} has an invalid contract id.`);
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    if (Object.keys(row).sort().join("\0") !== "coordinates\0grammar\0path\0sha256" || row.grammar !== "frozen-markdown-source-coordinates-v1") problems.push(`${label} requires the exact Markdown source-coordinate grammar and fields.`);
    if (typeof row.path !== "string" || !row.path.endsWith(".md") || /[\\:*?"<>|\u0000-\u001f]/u.test(row.path) || row.path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(row.path)) problems.push(`${label}.path must be one exact non-opaque repository-relative Markdown document.`);
    else { if (paths.has(row.path)) problems.push(`${label}.path duplicates another evidence owner.`); paths.add(row.path); }
    if (typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256)) problems.push(`${label}.sha256 must bind exact document bytes.`);
    if (!Array.isArray(row.coordinates) || row.coordinates.length === 0) { problems.push(`${label}.coordinates must be a nonempty array.`); continue; }
    const spans: { start: number; end: number }[] = [];
    for (const [index, coordinate] of row.coordinates.entries()) {
      if (!object(coordinate)) { problems.push(`${label}.coordinates[${index}] must be an object.`); continue; }
      if (Object.keys(coordinate).sort().join("\0") !== "end\0form\0kind\0start\0valueSha256" || coordinate.kind !== "source" || !["inline-code", "path-list-item"].includes(String(coordinate.form))) problems.push(`${label}.coordinates[${index}] requires one exact source-span form.`);
      if (!Number.isSafeInteger(coordinate.start) || Number(coordinate.start) < 0 || !Number.isSafeInteger(coordinate.end) || Number(coordinate.end) <= Number(coordinate.start)) problems.push(`${label}.coordinates[${index}] requires a nonempty exact UTF-16 span.`);
      else spans.push({ start: Number(coordinate.start), end: Number(coordinate.end) });
      if (typeof coordinate.valueSha256 !== "string" || !/^[a-f0-9]{64}$/u.test(coordinate.valueSha256)) problems.push(`${label}.coordinates[${index}].valueSha256 must bind exact UTF-8 path bytes.`);
    }
    spans.sort((left, right) => left.start - right.start || left.end - right.end);
    if (spans.some((span, index) => { const previous = spans[index - 1]; return previous !== undefined && span.start < previous.end; })) problems.push(`${label}.coordinates must not overlap or duplicate a span.`);
  }
  return problems;
}

/** 🗄️ Why a frozen document is gone, from a closed vocabulary. `ticket-close-generated-output-removed`
 * is AGENTS.md's own rule ("You MUST delete all tool generated output files … inside the ticket folder
 * after you are done") catching up with evidence that was frozen while the ticket was open. */
export type FrozenCoordinateEvidenceRetirementReason = "ticket-close-generated-output-removed";

/** 🪦️ An explicit, evidenced retirement: the document these bytes were frozen from no longer exists,
 * and this row records that fact instead of being deleted.
 *
 * A silent delete would make the count law pass again while destroying the only record that those
 * bytes were ever frozen — precisely what the seal exists to prevent. Retirement is metadata ABOUT the
 * record, never part of the evidence, so {@link frozenCoordinateEvidenceSeal} excludes it and a
 * retirement can never move the digest that proves `path`/`sha256`/`coordinates` were not edited. */
export interface FrozenCoordinateEvidenceRetirement {
  readonly ticket: string;
  readonly reason: FrozenCoordinateEvidenceRetirementReason;
}

/** 🔐️ Exact immutable JSON evidence with explicitly typed physical-coordinate value locations. */
export interface FrozenCoordinateEvidenceContract {
  readonly path: string;
  readonly sha256: string;
  readonly schemaVersion: number | null;
  readonly retired?: FrozenCoordinateEvidenceRetirement;
  readonly rootKind?: "array";
  readonly coordinates: readonly (Readonly<{ pointer: string; kind: "source" | "destination" }> | Readonly<{ pointer: string; kind: "source" | "destination"; representation: "recorded-repository-absolute"; recordedRepositoryRoot: string }> | Readonly<{ pointer: string; kind: "source"; representation: "recorded-package-owner-identity"; identityPrefix: "unmarked:" }> | Readonly<{ pointer: string; kind: "source"; representation: "json-escaped-source-path" }>)[];
}

/** 🧾️ Validates explicit evidence authority without discovering or reading any document. */
export function validateFrozenCoordinateEvidenceContracts(value: unknown): string[] {
  const problems: string[] = [], paths = new Set<string>();
  const object = (candidate: unknown): candidate is Record<string, unknown> => candidate !== null && typeof candidate === "object" && !Array.isArray(candidate);
  if (!object(value)) return ["frozenCoordinateEvidenceContracts must be an object."];
  for (const [id, row] of Object.entries(value)) {
    const label = `frozenCoordinateEvidenceContracts.${id}`;
    if (!/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(id)) problems.push(`${label} has an invalid contract id.`);
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    const arrayRoot = Object.hasOwn(row, "rootKind"), retired = Object.hasOwn(row, "retired");
    const expectedFields = ["coordinates", "path", "schemaVersion", "sha256", ...(arrayRoot ? ["rootKind"] : []), ...(retired ? ["retired"] : [])].sort().join("\0");
    if (Object.keys(row).sort().join("\0") !== expectedFields) problems.push(`${label} must contain only its exact document-root fields.`);
    if (arrayRoot && (row.rootKind !== "array" || row.schemaVersion !== null)) problems.push(`${label}.rootKind requires explicit array authority with absent schemaVersion.`);
    if (retired) {
      const record = row.retired;
      if (!object(record) || Object.keys(record).sort().join("\0") !== "reason\0ticket") problems.push(`${label}.retired must contain only its exact ticket and reason fields.`);
      else {
        if (record.reason !== "ticket-close-generated-output-removed") problems.push(`${label}.retired.reason must name one declared retirement cause.`);
        if (typeof record.ticket !== "string" || !/^\d{4}\/\d{2}\/\d{2}\/[A-Z0-9]+(?:-[A-Z0-9]+)*$/u.test(record.ticket)) problems.push(`${label}.retired.ticket must be one exact YYYY/MM/DD/TICKET-SLUG identifier.`);
      }
    }
    const path = row.path;
    if (typeof path !== "string" || !path.endsWith(".json") || /[\\:*?"<>|\u0000-\u001f]/u.test(path) || path.split("/").some((part) => !part || part === "." || part === "..") || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) problems.push(`${label}.path must be one exact non-opaque repository-relative JSON document.`);
    else { if (paths.has(path)) problems.push(`${label}.path duplicates another evidence owner.`); paths.add(path); }
    if (typeof row.sha256 !== "string" || !/^[a-f0-9]{64}$/u.test(row.sha256)) problems.push(`${label}.sha256 must bind exact document bytes.`);
    if (row.schemaVersion !== null && (!Number.isSafeInteger(row.schemaVersion) || Number(row.schemaVersion) < 1)) problems.push(`${label}.schemaVersion must be a positive integer or explicit absent-property null authority.`);
    if (!Array.isArray(row.coordinates) || row.coordinates.length === 0) { problems.push(`${label}.coordinates must be a nonempty array.`); continue; }
    const pointers = new Set<string>();
    for (const [index, coordinate] of row.coordinates.entries()) {
      if (!object(coordinate)) { problems.push(`${label}.coordinates[${index}] must be an object.`); continue; }
      const recorded = Object.hasOwn(coordinate, "representation");
      const ownerIdentity = coordinate.representation === "recorded-package-owner-identity";
      const escapedSource = coordinate.representation === "json-escaped-source-path";
      if (Object.keys(coordinate).sort().join("\0") !== (ownerIdentity ? "identityPrefix\0kind\0pointer\0representation" : escapedSource ? "kind\0pointer\0representation" : recorded ? "kind\0pointer\0recordedRepositoryRoot\0representation" : "kind\0pointer")) { problems.push(`${label}.coordinates[${index}] must contain only the fields of its exact coordinate representation.`); continue; }
      if (escapedSource) {
        if (coordinate.kind !== "source") problems.push(`${label}.coordinates[${index}] requires source-only JSON encoding authority.`);
      } else if (ownerIdentity) {
        if (coordinate.identityPrefix !== "unmarked:" || coordinate.kind !== "source") problems.push(`${label}.coordinates[${index}] requires the exact unmarked: source-owner identity prefix.`);
      } else if (recorded) {
        const root = coordinate.recordedRepositoryRoot;
        if (coordinate.representation !== "recorded-repository-absolute" || typeof root !== "string" || !/^(?:\/(?!\/)|[A-Za-z]:\/).+/u.test(root) || /[\\*?"<>|\u0000-\u001f]/u.test(root) || root.replace(/^[A-Za-z]:/u, "").slice(1).split("/").some((part) => !part || part === "." || part === ".." || part.includes(":"))) problems.push(`${label}.coordinates[${index}] requires one exact lexical POSIX or drive-qualified recorded repository root.`);
      }
      if (typeof coordinate.pointer !== "string" || !/^(?:\/(?:\*|(?:[^/~*]|~[01])+))+$/u.test(coordinate.pointer) || /[\u0000-\u001f]/u.test(coordinate.pointer)) problems.push(`${label}.coordinates[${index}].pointer must use exact JSON-pointer segments or array-index wildcards.`);
      else { if (pointers.has(coordinate.pointer)) problems.push(`${label}.coordinates[${index}].pointer duplicates another declaration.`); pointers.add(coordinate.pointer); }
      if (coordinate.kind !== "source" && coordinate.kind !== "destination") problems.push(`${label}.coordinates[${index}].kind must be source or destination.`);
    }
  }
  return problems;
}

/** 🧮️ The projection a frozen-evidence seal digests: the EVIDENCE of every contract — `path`,
 * `sha256`, `schemaVersion`, `rootKind`, `coordinates` — and nothing else.
 *
 * {@link FrozenCoordinateEvidenceRetirement} is deliberately outside it. A seal exists to prove that
 * no frozen path, digest or coordinate was quietly edited; recording that a document was deleted at
 * ticket close changes none of those, so retiring a row must never force whoever retires it to rewrite
 * the very digest that would have caught them editing the evidence. Anything added to a contract that
 * is evidence belongs in here; anything added that is bookkeeping about the record does not. */
export function frozenCoordinateEvidenceSeal(contracts: Readonly<Record<string, FrozenCoordinateEvidenceContract>>): Record<string, Omit<FrozenCoordinateEvidenceContract, "retired">> {
  return Object.fromEntries(Object.entries(contracts).map(([id, contract]) => {
    const { retired: _retired, ...evidence } = contract;
    return [id, evidence];
  }));
}

/** 🗂️ A whole-document historical-evidence population — ticket narrative reports, ticket workspace (evidence snapshots, scratch scripts, working notes), Cursor plan snapshots and per-developer prompt-log transcripts are never live references, so the engine excludes them from reference-candidate scanning entirely instead of freezing them coordinate-by-coordinate. Document KIND is the discriminator, not ticket lifecycle status. Membership never overrides a real machine-read contract: the engine additionally refuses to exempt any path matching a `fixedFilenameContracts` pattern, or sitting inside a directory that owns a package-root manifest (Cargo.toml/package.json/go.mod/…, derived from `fixedFilenameContracts[*].scope.kind === "package-root"`), so a future contract addition is protected automatically. */
export interface HistoricalDocumentEvidencePopulation {
  readonly grammar: "historical-document-evidence-v1";
  readonly directoryPattern: string;
  readonly leafPattern: string;
  readonly reason: string;
}

const HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS = ["ticket-report", "ticket-workspace", "cursor-plan-snapshot", "dev-prompt-log"] as const;

/** 🧷️ Validates the exact, closed, four-population historical-document-evidence grammar without touching the filesystem. */
export function validateHistoricalDocumentEvidencePopulations(value: unknown): string[] {
  const problems: string[] = [];
  const object = (entry: unknown): entry is Record<string, unknown> => entry !== null && typeof entry === "object" && !Array.isArray(entry);
  if (!object(value)) return ["historicalDocumentEvidencePopulations must be an object."];
  if (Object.keys(value).join("\0") !== HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS.join("\0")) problems.push('historicalDocumentEvidencePopulations must contain exactly ordered "ticket-report", "ticket-workspace", "cursor-plan-snapshot" and "dev-prompt-log" populations.');
  for (const id of HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS) {
    const row = (value as Record<string, unknown>)[id];
    const label = `historicalDocumentEvidencePopulations.${id}`;
    if (!object(row)) { problems.push(`${label} must be an object.`); continue; }
    if (Object.keys(row).sort().join("\0") !== "directoryPattern\0grammar\0leafPattern\0reason" || row.grammar !== "historical-document-evidence-v1") problems.push(`${label} requires the exact historical-document-evidence grammar and fields.`);
    if (typeof row.directoryPattern !== "string" || !row.directoryPattern) problems.push(`${label}.directoryPattern must be a nonempty glob.`);
    if (typeof row.leafPattern !== "string") problems.push(`${label}.leafPattern must be a regular-expression source string.`);
    else try { void new RegExp(row.leafPattern, "u"); } catch { problems.push(`${label}.leafPattern must be a valid regular expression.`); }
    if (typeof row.reason !== "string" || !row.reason) problems.push(`${label}.reason must be a nonempty justification.`);
  }
  const ticketReport = (value as Record<string, Record<string, unknown> | undefined>)["ticket-report"];
  if (object(ticketReport) && (ticketReport.directoryPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**" || ticketReport.leafPattern !== "^📓️.+\\.md$")) problems.push("historicalDocumentEvidencePopulations.ticket-report must use the exact ticket-document scope and leaf grammar.");
  const ticketWorkspace = (value as Record<string, Record<string, unknown> | undefined>)["ticket-workspace"];
  if (object(ticketWorkspace) && (ticketWorkspace.directoryPattern !== "**/.🧬semio/🦑️repo/🎫️tickets/🎆️[0-9][0-9]/🌙️[0-9][0-9]/☀️[0-9][0-9]/*/**" || ticketWorkspace.leafPattern !== "^.+$")) problems.push("historicalDocumentEvidencePopulations.ticket-workspace must use the exact ticket-root nested-descendant scope and leaf grammar.");
  const cursorPlanSnapshot = (value as Record<string, Record<string, unknown> | undefined>)["cursor-plan-snapshot"];
  if (object(cursorPlanSnapshot) && (cursorPlanSnapshot.directoryPattern !== ".cursor/plans/*" || cursorPlanSnapshot.leafPattern !== "^.+\\.plan\\.md$")) problems.push("historicalDocumentEvidencePopulations.cursor-plan-snapshot must use the exact Cursor plan-snapshot scope.");
  const devPromptLog = (value as Record<string, Record<string, unknown> | undefined>)["dev-prompt-log"];
  if (object(devPromptLog) && (devPromptLog.directoryPattern !== "**/.🧬semio/🦑️repo/💬️prompts/**" || devPromptLog.leafPattern !== "^.+\\.md$")) problems.push("historicalDocumentEvidencePopulations.dev-prompt-log must use the exact prompt-log scope and leaf grammar.");
  return problems;
}

/**
 * 🔣️ Shape of `🔣️taxonomy.json` — the single source of truth for taxonomy directory-name/role/lang
 * vocabulary and the package-discovery contract, replacing the two independently hand-maintained copies in
 * framework/os/plugin/registry script.ts (`TAXONOMY_ARTIFACT_COMPONENTS`/`TAXONOMY_WINDOW_CHILDREN`) and
 * root script.ts (`POLICY_ARTIFACT_COMPONENT_DIRS`/`POLICY_WINDOW_CHILD_DIRS`) — see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`.
 */
export interface Taxonomy {
  readonly _comment?: string;
  readonly schemaVersion: number;
  readonly fileKinds: Readonly<Record<string, FileKindSpec>>;
  readonly implementationLeafPolicy: { readonly roles: readonly FileKindSpec["role"][]; readonly fileKindIds: readonly string[]; readonly ignoredPathPatterns: readonly string[] };
  readonly fileKindResolutionRules: Readonly<Record<string, FileKindResolutionRuleSpec>>;
  readonly scopedFileKinds: Readonly<Record<string, ScopedFileKindSpec>>;
  readonly semanticDirectoryKinds: Readonly<Record<string, SemanticDirectoryKindSpec>>;
  readonly semanticDirectoryMemberKinds: Readonly<Record<string, SemanticDirectoryMemberKindSpec>>;
  readonly semanticProjectedMemberKinds: Readonly<Record<string, SemanticProjectedMemberKindSpec>>;
  readonly semanticPathProjectionProfileRenderers: Readonly<Record<string, SemanticPathProjectionProfileRenderer>>;
  readonly semanticDescendantContracts: Readonly<Record<string, SemanticDescendantContract>>;
  readonly semanticPathProjectionCatalogContracts: Readonly<Record<string, SemanticPathProjectionCatalogContract>>;
  readonly semanticPathProjectionContracts: Readonly<Record<string, SemanticPathProjectionContract>>;
  readonly semanticOwnedFileProjectionContracts: Readonly<Record<string, SemanticOwnedFileProjectionContract>>;
  readonly semanticPackageProjectionContracts: Readonly<Record<string, SemanticPackageProjectionContract>>;
  readonly semanticPolicyStateCoordinateContracts: Readonly<Record<string, SemanticPolicyStateCoordinateContract>>;
  readonly semanticPathProjectionReferenceConsumerContracts: Readonly<Record<string, SemanticPathProjectionReferenceConsumerContract>>;
  readonly mutationCatalogProjection: MutationCatalogProjectionContractIds;
  readonly fixedFilenameContracts: Readonly<Record<string, FixedFilenameContract>>;
  readonly fixedFilenameRejectionContracts: Readonly<Record<string, FixedFilenameRejectionContract>>;
  readonly fixedDirectoryContracts: Readonly<Record<string, FixedDirectoryContract>>;
  readonly fixedDirectoryContractSets?: Readonly<Record<string, readonly string[]>>;
  readonly configurableEntryContracts: Readonly<Record<string, ConfigurableEntryContract>>;
  readonly packageBoundaryRules: Readonly<Record<string, PackageBoundaryRule>>;
  readonly packageBoundaryProfiles: Readonly<Record<string, PackageBoundaryProfile>>;
  readonly packageGlueGrammar: Readonly<Record<string, PackageGlueGrammarSpec>>;
  readonly packageSourceDispositions: Readonly<Record<string, PackageSourceDisposition>>;
  readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
  readonly pathExclusions: Readonly<Record<string, PathExclusion>>;
  readonly pathEmojiPolicy: PathEmojiPolicy;
  readonly unicodeNormalization: { readonly form: "NFC"; readonly caseFold: "lower"; readonly locale: "und" };
  readonly variationSelectorPolicy: { readonly selector: "\uFE0F"; readonly requiredAfterEmoji: true; readonly comparison: "ignore-selector" };
  readonly collisionPolicy: {
    readonly comparisons: readonly ("byte" | "nfc" | "case-fold" | "vs16-fold" | "same-kind")[];
    readonly maxPathBytes: number;
    readonly rejectWindowsReservedNames: true;
    readonly rejectTrailingDotsAndSpaces: true;
  };
  readonly areaEnforcement: { readonly requiredState: "clean"; readonly undeclaredAreas: "enforce"; readonly opaquePathExclusionIds: readonly string[] };
  readonly semanticManifestFileKindId: string;
  /** 📇️ Exact collection-owner manifest filenames whose semantic direction requires a stemmed identity. */
  readonly semanticManifestFilenameOverrides?: Readonly<Record<string, string>>;
  readonly semanticExtensionKey: string;
  readonly semanticConsumerMinimum: number;
  readonly semanticAllowedOwnerLevels: readonly SemanticOwnerLevel[];
  readonly semanticCollections: Readonly<Record<string, SemanticCollectionSpec>>;
  readonly roles: readonly string[];
  readonly langs: readonly string[];
  readonly ecosystems: Readonly<Record<string, Ecosystem>>;
  readonly targets: Readonly<Record<string, TargetSpec>>;
  readonly rustEntryPathRules: RustEntryPathRules;
  readonly packagesDirName: string;
  /** 🎯️ Target owner axis: `<owner>/🎯️targets/<target>/📦️packages/<lang>/<manifest>`. */
  readonly targetsDirName: string;
  /** 🧱️ Flat co-location dir holding one subdir per logical element. */
  readonly elementsDirName: string;
  /** 🚪️ Shape V2: the entry file lives inside `📦️packages/<lang>/`, no longer at the owner root. */
  readonly entryLocation: "packages" | "owner-root";
  /** 📁 Global semantic directory kinds legal directly below a language package. */
  readonly packagingDirectoryKindIds?: readonly string[];
  readonly artifactsDirName: string;
  readonly modesDirName: string;
  readonly windowsDirName: string;
  readonly standardsDirName: string;
  readonly subsetsDirName: string;
  readonly standardDirPrefix?: string;
  readonly standardSlugPattern?: string;
  /** 🪆️ Dir-name prefix for a subset slug under `🪆️subsets/` (mirrors `standardDirPrefix` one level down). */
  readonly subsetDirPrefix?: string;
  /** 🪆️ Legal shape for a subset id (the logical id, `*`/`subsetAnyId` excepted — that one never matches this pattern by design). */
  readonly subsetSlugPattern?: string;
  /** 🪆️ The logical id every standard's unconstrained base subset carries. */
  readonly subsetAnyId?: string;
  /** 🪆️ The on-disk dir name `subsetAnyId` maps to — canonical single source for the `"*"` ⇔ `✳️any` mapping. */
  readonly subsetAnyDirName?: string;
  /** 🪆️ Exact per-`🪆️subsets` physical identities for standards whose sibling semantics require distinct emojis. */
  readonly subsetDirectoryOverrides?: Readonly<Record<string, Readonly<Record<string, string>>>>;
  /** 🔣️ File kind of each per-standard subset vocabulary manifest. */
  readonly subsetsManifestFileKindId?: string;
  /** ✅️ COMPLETENESS set: every legacy artifact carries schema, engine, and IO. Lifecycle capabilities are schema-derived. */
  readonly artifactComponentDirs: readonly string[];
  /** 🌳️ STRUCTURAL set: every dir allowed as a child of an artifact (superset of `artifactComponentDirs`; the structural-only extra is `📚️examples`). */
  readonly artifactChildDirs: readonly string[];
  /** 🧬️ Required children of a standards-based artifact. */
  readonly newArtifactComponentDirs: readonly string[];
  readonly newArtifactChildDirs: readonly string[];
  /** 🏅️ Required and allowed children of a standard. */
  readonly standardComponentDirs: readonly string[];
  readonly standardChildDirs: readonly string[];
  /** 🪆️ Required and allowed children of a subset. */
  readonly subsetComponentDirs: readonly string[];
  readonly subsetChildDirs: readonly string[];
  /** 👁️ Surface dir name for the read-only role. */
  readonly viewerDirName: string;
  /** ✏️ Surface dir name for the mutating role. */
  readonly editorDirName: string;
  /** 👁️✏️ The closed role vocabulary a surface may carry — also the `AppRole` wire strings. */
  readonly surfaceRoles: readonly string[];
  /** 👁️✏️ Role → on-disk surface dir name; the canonical single source for the mapping. */
  readonly surfaceDirNames: Readonly<Record<string, string>>;
  /** 👁️✏️ STRUCTURAL set: surface dirs a subset may carry. */
  readonly subsetSurfaceDirs: readonly string[];
  /** 👁️✏️ COMPLETENESS set: surface dirs every OWNED subset must carry. */
  readonly subsetRequiredSurfaceDirs: readonly string[];
  /** 👁️✏️ The only dirs a CONTRIBUTED (foreign-kind) subset mirror may carry — never schema or io. */
  readonly contributedSubsetChildDirs: readonly string[];
  /** 👁️✏️ STRUCTURAL set: every directory allowed directly below a surface. */
  readonly surfaceChildDirs: readonly string[];
  /** 👁️✏️ COMPLETENESS set: surface children that must exist even when empty. */
  readonly surfaceRequiredChildDirs: readonly string[];
  /** 👁️✏️ IMPLEMENTATION set: language leaves every surface root must carry. */
  readonly surfaceComponentLangs: readonly string[];
  /** 🔣️ Normative schema file kind per surface schema facet path. */
  readonly surfaceSchemaSpecFileKinds: Readonly<Record<string, string>>;
  /** 🪆️ Allowed subset archetypes: owning owns types; derived reuses types + conformance gate. */
  readonly subsetArchetypes?: readonly string[];
  /** ⚖️ Allowed IO fidelity class names a subset may declare. */
  readonly ioFidelityClasses?: readonly string[];
  /** 🧬️ Full-match pattern for one direct `<emoji><verb>-<noun>` mutation owner directory. */
  readonly mutationDirectoryPattern: string;
  /** 🏘️ Exact domain and operation owners; semantic IDs remain independent of compact operation basenames. */
  readonly mutationDomainOwners: Readonly<Record<string, Readonly<Record<string, Readonly<Record<string, string>>>>>>;
  /** 🧭️ Exact catalog subset owners whose physical mutations belong to another registered subset of the same artifact and standard. */
  readonly mutationCatalogSourceOwners: Readonly<Record<string, string>>;
  /** 🦀️ File kind of the mandatory direct `<mutation>/🦀️.rs` owner leaf. */
  readonly mutationComponentFileKindId: string;
  /** 🪪️ Language-neutral descriptor file kind owned beside every direct mutation component. */
  readonly mutationDescriptorFileKindId: string;
  readonly mutationPayloadSchemaLocation: Readonly<{ directoryKindId: "schema"; directoryName: "🧬️schema"; fileKindId: "json" }>;
  readonly mutationPayloadSchemaAuthority: Readonly<{ contractKind: "descriptor-linked-mutation-payload-schema"; ownerAuthority: "mutationOwnerIdentity"; descriptorFileKindId: "json"; descriptorField: "payloadSchema"; descriptorSchemaVersion: 1; descriptorCardinality: "one-canonical-no-competing-descriptor"; descriptorOwnerField: "owner"; descriptorIdentityField: "semanticKind"; jsonSchemaDialect: string; targetAuthority: "owner-relative-regular-json-schema" }>;
  /** 🧬️ PLACEMENT-bound facets below a direct mutation owner: `🦠️mutation`/`🔺️diff`/`↩️inverse` behavior, required-when-present, never inlinable into the direct leaf. */
  readonly mutationBehaviorFacetDirs: readonly string[];
  /** 🔖️ Region marker whose presence in a direct mutation leaf proves that facet's behavior was inlined instead of split into its own directory. */
  readonly mutationDirectLeafForbiddenRegionMarkers: Readonly<Record<string, string>>;
  /** 🧩️ Optional organizational facets below a direct mutation owner; none is a completeness requirement. */
  readonly mutationOrganizationalFacetDirs: readonly string[];
  /** 🧬️ Required children of each `🧬️schema/` facet: snapshot, diff, mutations. */
  readonly schemaChildDirs: readonly string[];
  /** 📝️ Representation nodes under schema snapshot/diff/mutations. */
  readonly representationDirs: readonly string[];
  /** 🧬️ Required native wire representations; storage representations remain optional. */
  readonly nativeRepresentationDirs: readonly string[];
  /** 🚪️ Top-level dirs under `🚪️io/`: import and export. */
  readonly ioDirectionDirs: readonly string[];
  /** 🚪️ Semantic codec facets below each native I/O representation; member collections are declared at `🚪️io/<representation>/<facet>`. */
  readonly ioSemanticCollectionDirNames: readonly string[];
  /** 🚪️ Direction to codec folder (import→deserializers, export→serializers). */
  readonly ioDirectionChildDirs: Readonly<Record<string, string>>;
  /** 📖️ File kinds required under every text representation node. */
  readonly textSpecFileKinds: readonly string[];
  /** 📡️ File kinds required under every binary representation node. */
  readonly binarySpecFileKinds: readonly string[];
  /**
   * 🧬️ Schema serialisation formats a `🧬️schema` facet must carry, one handcrafted leaf each. `fieldCasing`
   * is the canonical casing a field name takes in that format, which the parity scanners normalise through.
   */
  readonly schemaFormats: Readonly<Record<string, { readonly fileKindId: string; readonly fieldCasing: string }>>;
  /** 🧬️ Per-facet-kind schema format subsets. `facetPathIdentities` DECLARES the exact facet modules of a non-default kind; kind is never probed from the files that happen to sit on disk. */
  readonly schemaFacetKinds?: Readonly<Record<string, { readonly normativeFormat: string; readonly formats: readonly string[]; readonly facetPathIdentities: readonly string[] }>>;
  /** 🧬️ The facet kind every schema module carries unless a kind declares it in `facetPathIdentities`. */
  readonly schemaDefaultFacetKind: string;
  /** 🔣️ The single JSON Schema dialect every `🧬️data` facet declares, shared with `mutationPayloadSchemaAuthority`. */
  readonly schemaJsonDialect: string;
  /** 🧬️ Eligible schema scope-owner levels: only a directory matching one level (and no exclusion) may carry a `🧬️schema` module. */
  readonly schemaScopeOwnerLevels: {
    readonly facetDirName: string;
    readonly directoryKindId: string;
    readonly levels: Readonly<Record<string, { readonly pathPatterns: readonly string[]; readonly reason: string }>>;
    readonly excludedOwnerPathPatterns: readonly string[];
    readonly fixtureOwnerPathPatterns: readonly string[];
    readonly fixtureOwnerReason: string;
    readonly moduleMemberDirName: string;
    readonly mutationsFacetDirName: string;
    readonly inertSchemaDataKeyword: string;
  };
  /** 🔗️ How a consumer names one schema export: `schema://<scope id>/<ExportId>`, resolved through the generated catalog. */
  readonly schemaExportResolution: {
    readonly uriScheme: string;
    readonly uriPattern: string;
    readonly idBase: string;
    readonly idFacetFilenamePattern: string;
    readonly scopeIdSeparator: string;
    readonly scopeIdPattern: string;
    readonly exportIdPattern: string;
    /** 🔤️ Taxonomy format key to the ascii format id the Rust `SchemaFormat::id()` spelling uses; the only declared bridge between the two spellings. */
    readonly formatIds: Readonly<Record<string, string>>;
    /** 🦀️ Contract id every `schema verify` dump of `schema_export_catalog_entries()` declares. */
    readonly rustEntriesContractId: string;
    /** 🧬️ The `$id` segment that turns a mutation leaf into its own scope `<root>.mutation.<semanticKind>`. */
    readonly mutationScopeSegment: string;
    /** 🧬️ The facet filename a mutation leaf's own module declares. */
    readonly mutationLeafFacetFilename: string;
    readonly mutationAggregateFacetFilename: string;
    readonly rootExportKeyword: string;
    readonly exportsKeyword: string;
    readonly catalogPath: string;
    readonly catalogDocumentPath: string;
    readonly catalogContractId: string;
    readonly generator: string;
    readonly forbiddenPlacementPatterns: readonly string[];
    readonly placementExceptions: Readonly<Record<string, string>>;
  };
  /** 🔣️ Normative schema file kind per `🧬️schema` facet path. */
  readonly artifactSchemaSpecFileKinds: Readonly<Record<string, string>>;
  /** 🎛 Required children of each `🎚️config/` facet: its schema. */
  readonly configChildDirs: readonly string[];
  /** 👥️ Required children of each `👥️presence/` facet: its schema. */
  readonly presenceChildDirs: readonly string[];
  /** 🫧️ Required children of each `🫧️transient/` facet: its schema. The ephemeral local-only UI lane, fourth and last of the state mechanisms. */
  readonly transientChildDirs: readonly string[];
  readonly exampleAssetsDirName: string;
  readonly exampleTestsDirName: string;
  readonly exampleSlugPattern: string;
  readonly exampleAssetKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleMediaKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleFileKinds: Readonly<Record<string, string>>;
  readonly exampleTestFileKinds: Readonly<Record<string, string>>;
  readonly forbiddenExampleSlugs: readonly string[];
  readonly forbiddenExamplePluralDirs: readonly string[];
  /** 🎭️ STRUCTURAL set: every directory allowed directly below a mode. */
  readonly modeChildDirs: readonly string[];
  /** 🎭️ COMPLETENESS set: mode children that must exist even when empty. */
  readonly modeRequiredChildDirs: readonly string[];
  /** 📖️ Normative specification file kind per constitutional artifact facet. */
  readonly artifactSpecFileKinds: Readonly<Record<string, string>>;
  /** 🪟️ STRUCTURAL set: every directory allowed directly below a window. */
  readonly windowChildDirs: readonly string[];
  /** 🪟️ COMPLETENESS set: capability directories every window must carry, empty modules allowed. */
  readonly windowRequiredChildDirs: readonly string[];
  /** 🌐️ IMPLEMENTATION set: language component leaves every concrete capability member must carry. */
  readonly windowComponentLangs: readonly string[];
  /** 🪟️ IMPLEMENTATION set: language leaves the window ROOT itself must carry (facet items use `windowComponentLangs`). */
  readonly windowLeafLangs: readonly string[];
  /** 📌️ Tracked marker kind used when a required window facet has no specific items. */
  readonly windowEmptyFacetFileKindId: string;
  readonly taxonomyLeafParentDirs: readonly string[];
  /** 🍃️ Component file-kind identity, keyed by target or language. */
  readonly componentFileKinds: Readonly<Record<string, string>>;
  readonly physicalLeafRendering: Readonly<{
    direction: "forward-only";
    filename: "file-kind-emoji-and-extension-chain";
    sourceExtension: "longest-registered-chain";
    authoringExtension: "schema-ordered-primary";
    runtimeLookup: "canonical-only";
  }>;
  readonly referenceClosure: Readonly<{
    scope: "repository-incoming-and-moved-outgoing";
    candidateSource: "git-tracked-and-untracked-plus-explicit-ticket";
    candidateAdmission: "opaque-first-no-follow";
    coordinateRoots: "verified-repository-ownership";
    unsupportedPathBearingForms: "error";
    frozenSourceCoordinates: "exact-digest-and-token-authority";
    frozenPlanCoordinates: "canonical-schema-v2-digest-and-typed-token-authority";
    preimageDrift: "reject";
    newIncomingReferences: "reject-or-rollback";
    ordering: "utf8-byte";
    historicalDocumentEvidence: "ticket-report-workspace-cursor-plan-snapshot-and-dev-prompt-log-whole-document-excluded";
  }>;
  readonly frozenCoordinateEvidenceContracts: Readonly<Record<string, FrozenCoordinateEvidenceContract>>;
  readonly frozenMarkdownCoordinateEvidenceContracts: Readonly<Record<string, FrozenMarkdownCoordinateEvidenceContract>>;
  readonly historicalDocumentEvidencePopulations: Readonly<Record<string, HistoricalDocumentEvidencePopulation>>;
  readonly storyFileKindId: string;
  readonly testFeatureFileKindId: string;
  readonly testAdapterFileKinds: Readonly<Record<string, string>>;
  readonly testImplementationIds: Readonly<Record<string, string>>;
  readonly testImplementationFileKindIds: readonly string[];
  readonly testLegacyDirectoryNames: readonly string[];
  readonly testFixtureLegacyDirectoryNames: readonly string[];
  /** 🧪️ Canonical tests directory name (`🧪️tests`) — the owner-local home of every test case. */
  readonly testsDirName: string;
  /** 🧫️ Canonical test-fixture directory name (`🧫️fixtures`) — opaque input data inside its owning tests collection. */
  readonly testFixturesDirName: string;
  /** 🏷️ The owner kinds that may carry `🧪️tests` with nested `🧫️fixtures` beside their facets. */
  readonly testOwnerKinds: readonly string[];
  readonly testExamplesDirName: string;
  readonly testOraclesDirName: string;
  readonly testObsoleteCategoryStems: readonly string[];
  readonly testObsoleteTestEmojiCategoryStems: readonly string[];
  readonly testLegacyFilenamePatterns: readonly Readonly<{ id: string; pattern: string }>[];
  /** 🎚️ The case name that holds a package's test-runner configuration; it configures one delivery package, so it may sit under a delivery scope. */
  readonly testRunnerConfigurationCaseName: string;
  /** 🔬️ Owner collections whose members bound the search for a case's subject package. */
  readonly testSubjectBoundaryOwnerKinds: readonly string[];
  readonly testDeliveryScopeDirectoryNames: readonly string[];
  readonly testJavaScriptFrameworkModules: readonly string[];
  readonly testAssertionModules: readonly string[];
  readonly testSelfTestDeclarationPattern: string;
  readonly testContributionFileKindId: string;
  readonly testOutputMarkerFileKindId: string;
  readonly testOracleRegistryLocation: { readonly directoryPath: string; readonly fileKindId: string };
  readonly testSchemaLocation: { readonly directoryPath: string; readonly fileKindId: string };
  readonly libWiringLineBudget: number;
  readonly forbiddenPathSegments: readonly string[];
  /** 🔌️ Structural facet folders allowed directly under each plugin root. */
  readonly pluginChildDirs: readonly string[];
  readonly pluginRequiredChildDirs: readonly string[];
  /** 💻️ Structural facet folders owned directly by the OS product. */
  readonly osChildDirs: readonly string[];
  readonly osRequiredChildDirs: readonly string[];
  /** 🚫️ Emoji-stripped directory/file stems banned repo-wide (e.g. `core`, `shared`). */
  readonly bannedNameStems: readonly string[];
  /** 🪆️ Banned stems that stay legal only as a direct child of a `subsetsDirName` owner (the standard's own subset vocabulary, e.g. `base`), and the exact vendored files whose fixed internal import identity forbids a rename. */
  readonly bannedNameStemExemptions: { readonly subsetDirectories: readonly string[]; readonly vendoredFiles: readonly string[] };
  readonly rootDataDirNames: readonly string[];
  readonly rootDataContractIds: readonly string[];
  readonly rootDocumentContractIds: readonly string[];
  readonly repoWideContractIds: readonly string[];
  readonly dependencyDirections: {
    /** 🏺️ Vendored third-party source trees: byte-identical upstream output that the dependency graph neither lints nor follows. */
    readonly vendoredPaths: readonly string[];
    readonly roles: Readonly<Record<string, { readonly ownerPaths: readonly string[]; readonly externalPackages: readonly string[] }>>;
    /** 🖱️ The ui boundary: which declared roles the ui subject may use at runtime (foundation) and from tests only (test support). */
    readonly uiBoundary: { readonly version: 1; readonly subjectRole: string; readonly foundationRoles: readonly string[]; readonly testSupportRoles: readonly string[] };
    readonly rules: Readonly<Record<string, { readonly fromRoles: readonly string[]; readonly toRoles: readonly string[] }>>;
  };
  readonly cargoDependencyDirections: {
    readonly roles: readonly string[];
    readonly ownerRoles: readonly { readonly path: string; readonly roles: readonly string[] }[];
    readonly rules: Readonly<Record<string, { readonly fromRoles: readonly string[]; readonly toRoles: readonly string[]; readonly fromOwnerPaths?: readonly string[]; readonly toOwnerSegments?: readonly string[] }>>;
  };
  readonly areaLayers: Readonly<Record<string, "framework" | "implementation" | "repo-wide">>;
  readonly packageMaturityStates: readonly PackageMaturity[];
  /** 🧭️ How migration is detected — structurally, never from a hand-maintained package list. */
  readonly migratedMarker: "packages-dir-exists";
  /** 🔌️ Area roots whose package owners contribute plugins. */
  readonly pluginAreas: readonly string[];
  readonly areas: Readonly<Record<string, AreaState>>;
}
