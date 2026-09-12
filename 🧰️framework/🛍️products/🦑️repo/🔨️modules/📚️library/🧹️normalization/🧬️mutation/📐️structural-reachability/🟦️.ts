import { posix } from "node:path";
import { canonicalPrimaryFilenameForKind, loadTaxonomy, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { createRustMutationCodecOwnershipInspector, createRustMutationInputInspector, inspectRustModuleGraphFacts, inspectRustMutationMetadataFacts, inspectRustRunnableTests, inspectRustSourceIdentities, inspectRustStructure, jsonDocumentDuplicateKeys, mutationOwnerIdentity, mutationPayloadSchemaProblems, mutationPayloadSchemaRelativePath } from "../../../🔍️discovery/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validation/🟦️.ts";
import { MUTATION_DESCRIPTOR_SCHEMA_REL, mutationTaxonomyCompare, mutationTaxonomyStructuralView, type MutationTaxonomyStructuralSourceView } from "../📸️captured-source/🟦️.ts";
import { mutationTaxonomySourceIndex } from "../📇️index/🟦️.ts";
import { POLICY_MUTATIONS_FACET, POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF, policyArtifactRootOfMutationsDir, policyLeadingEmojiPrefix, policyStripEmoji, policyStructuralRelativeLocator } from "../🪪️identity/🟦️.ts";

export type PolicyStructuralMutationChildClassification = "direct-owner" | "domain-owner" | "root-infrastructure" | "malformed-child" | "unsafe-child" | "missing-directory-candidate" | "nonregular-or-unadmitted" | "root-file-evidence" | "absent" | "repository-boundary";

export type PolicyStructuralMutationChild = { readonly name: string; readonly path: string; readonly classification: PolicyStructuralMutationChildClassification };


export function policyStructuralMutationChildren(view: MutationTaxonomyStructuralSourceView, mutationsRel: string): readonly PolicyStructuralMutationChild[] {
  const taxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>, prefix = `${mutationsRel}/`, domains = taxonomy.mutationDomainOwners[mutationsRel];
  const candidates = new Map<string, { directory: boolean; file: boolean; nonregular: boolean; absent: boolean; boundary: boolean }>();
  const add = (name: string, patch: Partial<{ directory: boolean; file: boolean; nonregular: boolean; absent: boolean; boundary: boolean }>): void => {
    const parts = name.split("/");
    if (!name || parts.length > 2 || parts.length === 2 && (!domains || !Object.hasOwn(domains, parts[0]!))) return;
    const prior = candidates.get(name) ?? { directory: false, file: false, nonregular: false, absent: false, boundary: false };
    candidates.set(name, { directory: prior.directory || patch.directory === true, file: prior.file || patch.file === true, nonregular: prior.nonregular || patch.nonregular === true, absent: prior.absent || patch.absent === true, boundary: prior.boundary || patch.boundary === true });
  };
  for (const path of view.directories.keys()) {
    if (!path.startsWith(prefix)) continue;
    add(path.slice(prefix.length), { directory: true });
  }
  for (const observation of view.admission.observations) {
    if (!observation.sourcePath.startsWith(prefix)) continue;
    const name = observation.sourcePath.slice(prefix.length);
    if (observation.repositoryBoundary === "gitlink") add(name, { boundary: true });
    else if (observation.observedKind === "directory") add(name, { directory: true });
    else if (observation.observedKind === "file") add(name, { file: true });
    else if (observation.observedKind === "absent") add(name, { absent: true });
    else add(name, { nonregular: true });
  }
  return [...candidates].map(([name, state]) => {
    const path = `${mutationsRel}/${name}`;
    const classification: PolicyStructuralMutationChildClassification = state.boundary ? "repository-boundary"
      : state.nonregular || (state.directory && state.absent) ? "nonregular-or-unadmitted"
      : state.directory && state.file ? mutationOwnerIdentity(mutationsRel, name, taxonomy) !== null ? "missing-directory-candidate" : "nonregular-or-unadmitted"
      : state.directory && name.toLocaleLowerCase("en-US") === "compose" ? "unsafe-child"
      : state.directory && domains && Object.hasOwn(domains, name) ? "domain-owner"
      : state.directory && mutationOwnerIdentity(mutationsRel, name, taxonomy) !== null ? "direct-owner"
      : state.directory && (taxonomy.mutationBehaviorFacetDirs.includes(name) || taxonomy.mutationOrganizationalFacetDirs.includes(name)) ? "root-infrastructure"
      : state.directory ? "malformed-child"
      : state.file && mutationOwnerIdentity(mutationsRel, name, taxonomy) !== null ? "missing-directory-candidate"
      : state.file ? "root-file-evidence"
      : "absent";
    return { name, path, classification };
  }).sort((left, right) => mutationTaxonomyCompare(left.name, right.name));
}


export function policyStructuralMutationDirs(view: MutationTaxonomyStructuralSourceView, mutationsRel: string): string[] {
  return policyStructuralMutationChildren(view, mutationsRel).filter((child) => child.classification === "direct-owner").map((child) => child.name);
}


/** 🪪️ Reads the explicit domain identity, or the existing direct-owner basename contract. */
export function policyMutationSemanticIdentity(mutationsRel: string, ownerPath: string, taxonomy: ReturnType<typeof loadTaxonomy> = loadTaxonomy()): string {
  const identity = mutationOwnerIdentity(mutationsRel, ownerPath, taxonomy);
  if (identity !== null) return identity;
  if (Object.hasOwn(taxonomy.mutationDomainOwners, mutationsRel)) throw new Error(`Unregistered mutation owner ${mutationsRel}/${ownerPath}`);
  return policyStripEmoji(ownerPath);
}


export function policyStructuralSource(view: MutationTaxonomyStructuralSourceView, path: string): string | null {
  return view.contents.get(path) ?? null;
}


export function policyStructuralNodeState(view: MutationTaxonomyStructuralSourceView, path: string): "file" | "directory" | "symlink" | "absent" {
  if (view.contents.has(path)) return "file";
  if (view.directories.has(path)) return "directory";
  const observed = view.admission.observations.filter((observation) => observation.sourcePath === path);
  if (observed.some((observation) => observation.observedKind === "symlink")) return "symlink";
  return "absent";
}


/** 🪬️ Checks schema ownership using only the captured structural source and admission evidence. */
export function policyMutationPayloadSchemaProblems(view: MutationTaxonomyStructuralSourceView, owner: string, pointer: string, taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  return mutationPayloadSchemaProblems(owner, pointer, (path) => {
    const observations = view.admission.observations.filter((observation) => observation.sourcePath === path);
    const unsafe = observations.some((observation) => observation.observedKind !== "file" && observation.observedKind !== "directory");
    return { kind: unsafe ? "unadmitted" : policyStructuralNodeState(view, path), content: policyStructuralSource(view, path) ?? undefined, repositoryBoundary: observations.some((observation) => observation.repositoryBoundary === "gitlink") };
  }, taxonomy.mutationPayloadSchemaAuthority.jsonSchemaDialect);
}


export function policyStructuralMutationRoots(view: MutationTaxonomyStructuralSourceView, mutationRoots: readonly string[]): string[] {
  const validated: string[] = [];
  for (const root of mutationRoots) {
    const locator = policyStructuralRelativeLocator(root);
    if (locator === null || !locator.endsWith(`/${POLICY_MUTATIONS_FACET}`) || !view.directories.has(locator)) throw new Error(`mutation structural scope is absent, opaque, symlinked, or outside the captured source view: ${JSON.stringify(root)}.`);
    validated.push(locator);
  }
  return [...new Set(validated)].sort(mutationTaxonomyCompare);
}


export function policyMutationDirectOwnerBreachesView(view: MutationTaxonomyStructuralSourceView, mutationRoots: readonly string[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const mutationsRel of mutationRoots) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    for (const mutName of policyStructuralMutationDirs(view, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`, directRel = `${mutRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (policyStructuralSource(view, directRel) !== null) continue;
      breaches.push({ id: `mutation-direct-owner-${mutRel}`, summary: `"${mutRel}" has no direct ${POLICY_RS_COMPONENT_LEAF_NAME}`, kind: "mutation/direct-owner", scope: artRel, priority: "high", reason: "Every concrete mutation is represented by exactly one direct semantic folder whose component is authoritative.", solution: `Move the concrete implementation to ${directRel}; optional diff, inverse, plan, text, binary, and tests remain child facets.` });
    }
  }
  return breaches;
}


/**
 * 📏️SEMANTIC-MUTATIONS-OVERHAUL rule 2 (`policyMutationDispatchCoverageBreaches`, formerly a Wave-3
 * placeholder — this wave lands the real comparison): for every `🧬️mutations/🦀️.rs` dispatch
 * file, extracts its `pub enum \w*Mutation\w* { … }` variant names and compares them against the
 * concrete triad-dir stems (`policyListMutationDirs`, kebab-case minus emoji, PascalCased) sitting
 * beside it. Kept at `"medium"` (advisory) rather than `"high"` because zero facets have adopted the
 * `#[derive(dsl_derive::Mutations)]` 1:1 variant-per-triad-dir shape yet (today's dispatch enums are
 * still the generic `CollectionMutation<…>` shape `policySemanticVocabularyBreaches` flags separately) —
 * this rule graduates to `"high"` once the fan-out wave lands real per-mutation triad wiring, mirroring
 * `policyMutationImplPresenceBreaches`'s own "advisory while Wave 3 pilot lands" graduation comment.
 */
export function policyMutationEnumVariantNames(content: string): string[] {
  return inspectRustStructure(content).enums.filter((item) => item.name.includes("Mutation")).flatMap((item) => item.variants.map((variant) => variant.name));
}


/** 🐫Kebab (minus emoji) → PascalCase, for comparing a triad-dir stem against a dispatch-enum variant name. */
export function policyKebabToPascal(slug: string): string {
  return slug
    .split("-")
    .filter(Boolean)
    .map((seg) => seg.charAt(0).toUpperCase() + seg.slice(1))
    .join("");
}


//#region 🧬️DirectMutationPolicies
export const MUTATION_STRUCTURAL_POLICY_KINDS = [
  "mutation/direct-owner",
  "mutation/root-purity",
  "mutation/folder-variant-bijection",
  "mutation/descriptor-bijection",
  "mutation/reachability",
  "mutation/behavior-ownership",
  "mutation/codec-ownership",
  "mutation/wire-identity",
  "mutation/schema-parity",
  "mutation/language-parity",
  "mutation/catalog-parity",
  "mutation/no-hidden-generation",
  "mutation/no-sentinel",
  "mutation/no-generic-snapshot-fallback",
  "mutation/shared-helper-purity",
  "mutation/test-presence",
  "mutation/compose-exclusion",
] as const;

export type MutationStructuralKind = typeof MUTATION_STRUCTURAL_POLICY_KINDS[number];


export type MutationLeafDescriptor = {
  readonly schemaVersion: 1;
  readonly owner: string;
  readonly semanticKind: string;
  readonly displayName: string;
  readonly emoji: string;
  readonly aggregateVariant: string;
  readonly payloadSchema: string;
  readonly textOpcode: string | null;
  readonly binaryTag: number | null;
  readonly invertibility: "self" | "explicit-mutation" | "plan" | "non-invertible";
  readonly diffParticipation: "detect" | "apply-only" | "plan" | "none";
  readonly outcomeClasses: readonly string[];
  readonly composition: "atomic" | "composite";
  readonly requiredLanguageSurfaces: readonly string[];
};

//#endregion 🔣️JsonSchemaSubset

export function policyMutationDescriptorView(view: MutationTaxonomyStructuralSourceView, descriptorRel: string): { descriptor?: MutationLeafDescriptor; problem?: string } {
  const source = policyStructuralSource(view, descriptorRel);
  if (source === null) return { problem: "descriptor is missing" };
  try {
    const schema = JSON.parse(view.mutationDescriptorSchema.bytes.toString("utf8"));
    const descriptor = JSON.parse(source) as MutationLeafDescriptor;
    const errors = [...jsonDocumentDuplicateKeys(source), ...validateJsonSchemaSubset(schema, descriptor)];
    return errors.length === 0 ? { descriptor } : { problem: errors.join("; ") };
  } catch (error) { return { problem: error instanceof Error ? error.message : String(error) }; }
}


/**
 * 🧬️ Structural identity of one `🧬️mutations/🔣️.json` aggregate: every `oneOf` `$ref` must resolve to a
 * direct leaf's payload schema (by relative path or by that document's `$id`) and every leaf whose descriptor
 * requires the JSON Schema surface must be referenced. This replaces reading the leaf's semantic kind out of
 * the aggregate's `x-semio-mutationKinds` string list, which no consumer reads.
 */
export function policyMutationAggregateMembers(view: MutationTaxonomyStructuralSourceView, mutationsRel: string, aggregateRel: string, leafNames: readonly string[], taxonomy: ReturnType<typeof loadTaxonomy>): { readonly referenced: ReadonlySet<string>; readonly breaches: BreachRecord[] } {
  const solution = "Make the aggregate a pure $ref union over exactly the direct leaves' payload schemas; the union is the identity, no restated kind list.";
  const source = policyStructuralSource(view, aggregateRel);
  if (source === null) return { referenced: new Set(), breaches: [] };
  let parsed: { oneOf?: unknown };
  try {
    parsed = JSON.parse(source) as { oneOf?: unknown };
  } catch (error) {
    return { referenced: new Set(), breaches: [policyMutationStructuralBreach("mutation/schema-parity", aggregateRel, `"${aggregateRel}" is not readable JSON: ${error instanceof Error ? error.message : String(error)}`, solution)] };
  }
  if (!Array.isArray(parsed.oneOf)) return { referenced: new Set(), breaches: [policyMutationStructuralBreach("mutation/schema-parity", aggregateRel, `"${aggregateRel}" declares no oneOf union of its direct leaves`, solution)] };
  const descriptorFilename = canonicalPrimaryFilenameForKind(taxonomy.mutationDescriptorFileKindId, taxonomy);
  const byPath = new Map<string, string>();
  const byId = new Map<string, string>();
  const required: string[] = [];
  for (const leafName of leafNames) {
    const descriptor = policyMutationDescriptorView(view, `${mutationsRel}/${leafName}/${descriptorFilename}`).descriptor;
    if (!descriptor?.requiredLanguageSurfaces.includes("json-schema")) continue;
    required.push(leafName);
    const payloadRel = `${mutationsRel}/${leafName}/${descriptor.payloadSchema.split("#")[0]}`;
    byPath.set(payloadRel, leafName);
    const payloadSource = policyStructuralSource(view, payloadRel);
    if (payloadSource === null) continue;
    try {
      const id = (JSON.parse(payloadSource) as { $id?: unknown }).$id;
      if (typeof id === "string" && id) byId.set(id, leafName);
    } catch { /* an unreadable payload is already a schema-parity breach on the leaf */ }
  }
  const breaches: BreachRecord[] = [];
  const referenced = new Set<string>();
  for (const [index, member] of parsed.oneOf.entries()) {
    const ref = (member as { $ref?: unknown } | null)?.$ref;
    if (typeof ref !== "string" || !ref) {
      breaches.push(policyMutationStructuralBreach("mutation/schema-parity", aggregateRel, `"${aggregateRel}" oneOf[${index}] is not a $ref to a direct leaf payload schema`, solution));
      continue;
    }
    const target = ref.split("#")[0]!;
    const leafName = byId.get(target) ?? byPath.get(target.includes("://") ? target : posix.normalize(`${mutationsRel}/${target}`));
    if (leafName === undefined) breaches.push(policyMutationStructuralBreach("mutation/schema-parity", aggregateRel, `"${aggregateRel}" oneOf[${index}] references ${JSON.stringify(ref)}, which is not a direct leaf payload schema of "${mutationsRel}"`, solution));
    else referenced.add(leafName);
  }
  for (const leafName of required) if (!referenced.has(leafName)) breaches.push(policyMutationStructuralBreach("mutation/schema-parity", aggregateRel, `"${aggregateRel}" omits the payload schema of direct leaf "${leafName}"`, solution));
  return { referenced, breaches };
}


export function policyMutationStructuralBreach(kind: MutationStructuralKind, scope: string, summary: string, solution: string): BreachRecord {
  return { id: `${kind.replaceAll("/", "-")}-${scope}`, summary, kind, scope, priority: "high", reason: "The direct-leaf mutation architecture requires a visible one-to-one owner, identity, behavior, surface, and test correspondence.", solution };
}


/** 🔢️ Resolves a literal Rust tag or unambiguous local constant chain without evaluating expressions. */
export function policyMutationBinaryTag(value: string, constants: readonly { readonly name: string; readonly value: string }[]): number | null {
  const visited = new Set<string>();
  while (/^[A-Za-z_][A-Za-z0-9_]*$/u.test(value)) {
    if (visited.has(value)) return null;
    visited.add(value);
    const matches = constants.filter((constant) => constant.name === value);
    if (matches.length !== 1) return null;
    value = matches[0]!.value;
  }
  if (!/^(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|[0-9][0-9_]*)(?:u(?:8|16|32|64|128|size))?$/u.test(value)) return null;
  const tag = Number(value.replace(/u(?:8|16|32|64|128|size)$/u, "").replaceAll("_", ""));
  return Number.isSafeInteger(tag) ? tag : null;
}


export function policyMutationRootPurityBreaches(mutationsRel: string, raw: string): BreachRecord[] {
  const rootRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
  const facts = inspectRustStructure(raw);
  const forbidden: string[] = [];
  if (!raw.trim()) forbidden.push("empty aggregate source");
  if (facts.enums.filter((item) => item.name.includes("Mutation")).some((item) => item.variants.some((variant) => variant.fieldStyle !== "tuple" || variant.fieldTypes.length !== 1))) forbidden.push("aggregate variant without exactly one wrapped leaf");
  if (facts.inlinePayloads.length > 0) forbidden.push("payload struct");
  if (facts.matchArms.length > 0) forbidden.push("match behavior");
  if (facts.constants.some((constant) => constant.name === "KINDS")) forbidden.push("hand-maintained KINDS");
  if (facts.impls.some((implementation) => implementation.methods.some((method) => ["apply", "diff", "inverse", "between", "parse", "print", "encode", "decode", "transform"].includes(method)))) forbidden.push("mutation-specific method");
  if (facts.includes.length > 0) forbidden.push("hidden/generated implementation");
  return forbidden.length === 0 ? [] : [policyMutationStructuralBreach("mutation/root-purity", rootRel, `"${rootRel}" contains forbidden aggregate content: ${forbidden.join(", ")}`, "Move mutation-specific payloads, behavior, codecs, fixtures, and tests into their direct leaf owners; retain only visible aggregation and structural correspondence tests.")];
}


export interface WrappedMutationTypeOrigin {
  readonly sourcePath: string;
  readonly declarationName: string;
  readonly modulePath: readonly string[];
}


export interface MutationRootReachability {
  readonly leafName: string;
  readonly variantName: string;
  readonly moduleName: string;
  readonly mounted: boolean;
  readonly wrapped: boolean;
  readonly origin: WrappedMutationTypeOrigin | null;
  readonly reason: string | null;
}


/** 🔗️ Proves one wrapped aggregate payload's declared source without resolving a second source graph. */
export function inspectMutationRootReachability(repoRoot: string, mutationsRel: string, rootSource: string, leafNames: readonly string[], rustFilename: string): readonly MutationRootReachability[] {
  const unresolved = (reason: string): readonly MutationRootReachability[] => leafNames.map((leafName) => {
    let identity: string;
    try { identity = policyMutationSemanticIdentity(mutationsRel, leafName); } catch { identity = policyStripEmoji(leafName); }
    return { leafName, variantName: policyKebabToPascal(identity), moduleName: identity.replaceAll("-", "_"), mounted: false, wrapped: false, origin: null, reason };
  });
  const rootLocator = policyStructuralRelativeLocator(mutationsRel), filename = policyStructuralRelativeLocator(rustFilename);
  if (rootLocator === null || filename === null || filename.includes("/") || leafNames.some((leaf) => policyStructuralRelativeLocator(leaf) === null)) return unresolved("requires safe captured mutation source locators");
  let index: ReturnType<typeof mutationTaxonomySourceIndex>;
  try { index = mutationTaxonomySourceIndex(repoRoot, {}); } catch (error) { return unresolved(`captured source admission failed: ${error instanceof Error ? error.message : String(error)}`); }
  const view = mutationTaxonomyStructuralView(index);
  const taxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
  if (leafNames.some((leaf) => mutationOwnerIdentity(mutationsRel, leaf, taxonomy) === null)) return unresolved("requires safe captured mutation source locators");
  const captured = policyStructuralSource(view, `${rootLocator}/${filename}`);
  if (captured === null || captured !== rootSource) return unresolved("aggregate source is absent, changed, or outside the captured source view");
  return inspectMutationRootReachabilityView(view, rootLocator, captured, leafNames, filename);
}

export function inspectMutationRootReachabilityView(view: MutationTaxonomyStructuralSourceView, mutationsRel: string, rootSource: string, leafNames: readonly string[], rustFilename: string): readonly MutationRootReachability[] {
  const taxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
  const identity = (leafName: string): string => policyMutationSemanticIdentity(mutationsRel, leafName, taxonomy);
  const unresolved = (leafName: string, reason: string): MutationRootReachability => ({ leafName, variantName: policyKebabToPascal(identity(leafName)), moduleName: identity(leafName).replaceAll("-", "_"), mounted: false, wrapped: false, origin: null, reason });
  const graph = inspectRustModuleGraphFacts(rootSource), aggregates = inspectRustStructure(rootSource).enums.filter((item) => item.name.endsWith("Mutation") && item.visibility === "pub");
  if (aggregates.length !== 1 || aggregates[0]!.conditional) return leafNames.map((leafName) => unresolved(leafName, "requires exactly one unconditional public top-level aggregate Mutation enum"));
  const variants = aggregates[0]!.variants;
  return leafNames.map((leafName) => {
    const moduleName = identity(leafName).replaceAll("-", "_"), variantName = policyKebabToPascal(identity(leafName));
    const namedMounts = graph.modules.filter((module) => module.modulePath.length === 1 && module.name === moduleName);
    const mount = namedMounts.filter((module) => module.visibility === "pub" && !module.inline && !module.conditional && module.pathTarget === `${leafName}/${rustFilename}`);
    if (namedMounts.length !== 1 || mount.length !== 1) return unresolved(leafName, "requires exactly one unconditional public canonical direct-leaf mount");
    const leafRel = `${mutationsRel}/${leafName}`, leafPath = `${leafRel}/${rustFilename}`, leafSource = policyStructuralSource(view, leafPath);
    if (leafSource === null) return { leafName, variantName, moduleName, mounted: true, wrapped: false, origin: null, reason: `mounted direct leaf type source is ${policyStructuralNodeState(view, leafPath)} or outside the captured source view` };
    type OriginCandidate = { readonly origin: WrappedMutationTypeOrigin; readonly conditional: boolean };
    const origins = new Map<string, OriginCandidate[]>(), add = (name: string, origin: WrappedMutationTypeOrigin, conditional: boolean): void => { origins.set(name, [...(origins.get(name) ?? []), { origin, conditional }]); };
    const declarations = (source: string, sourcePath: string, requiredModulePath?: readonly string[]): readonly OriginCandidate[] => inspectRustMutationMetadataFacts(source).declarations.filter((item) => item.visibility === "pub" && (requiredModulePath === undefined || item.modulePath.join("\0") === requiredModulePath.join("\0"))).map((item) => ({ origin: { sourcePath, declarationName: item.name, modulePath: item.modulePath }, conditional: item.conditional === true }));
    const leafMetadata = inspectRustMutationMetadataFacts(leafSource);
    for (const candidate of declarations(leafSource, leafPath, [])) add(candidate.origin.declarationName, candidate.origin, candidate.conditional);
    const leafGraph = inspectRustModuleGraphFacts(leafSource);
    for (const use of leafMetadata.crateAliases.filter((entry) => entry.modulePath.length === 0 && entry.kind === "reexport" && !entry.restricted)) {
      const match = /^([A-Za-z_][A-Za-z0-9_]*)::([A-Za-z_][A-Za-z0-9_]*)$/u.exec(use.source);
      const children = match ? leafGraph.modules.filter((entry) => entry.modulePath.length === 1 && entry.name === match[1] && entry.visibility === "pub" && !entry.inline && entry.pathTarget !== null) : [];
      const inlineChildren = match ? leafGraph.modules.filter((entry) => entry.modulePath.length === 1 && entry.name === match[1] && entry.visibility === "pub" && entry.inline) : [];
      const child = children.length + inlineChildren.length === 1 ? [...children, ...inlineChildren][0] : undefined;
      if (!match || !child) continue;
      const childPath = child.inline ? leafPath : `${leafRel}/${child.pathTarget!}`, childSource = child.inline ? leafSource : policyStructuralSource(view, childPath);
      if (childSource === null) continue;
      const childOrigins = declarations(childSource, childPath, child.inline ? [child.name] : []).filter((candidate) => candidate.origin.declarationName === match[2]!);
      if (childOrigins.length === 1) add(use.alias, childOrigins[0]!.origin, childOrigins[0]!.conditional || child.conditional === true || use.conditional);
    }
    const aliases = new Map<string, OriginCandidate[]>();
    for (const [name, candidates] of origins) aliases.set(`${moduleName}::${name}`, candidates);
    const rootMetadata = inspectRustMutationMetadataFacts(rootSource), rootNames = new Set(rootMetadata.declarations.filter((item) => item.modulePath.length === 0).map((item) => item.name));
    for (const use of rootMetadata.crateAliases.filter((entry) => entry.modulePath.length === 0 && entry.kind === "reexport" && !entry.restricted)) {
      const match = new RegExp(`^${moduleName}::([A-Za-z_][A-Za-z0-9_]*)$`, "u").exec(use.source), candidates = match ? origins.get(match[1]!) : undefined;
      if (candidates && !rootNames.has(use.alias)) aliases.set(use.alias, [...(aliases.get(use.alias) ?? []), ...candidates.map((candidate) => ({ ...candidate, conditional: candidate.conditional || use.conditional }))]);
    }
    const named = variants.filter((variant) => variant.name === variantName), matching = named.filter((variant) => variant.fieldStyle === "tuple" && variant.fieldTypes.length === 1 && aliases.has(variant.fieldTypes[0]!)), candidates = matching.length === 1 ? aliases.get(matching[0]!.fieldTypes[0]!) ?? [] : [];
    return named.length === 1 && !named[0]!.conditional && matching.length === 1 && candidates.length === 1 && !candidates[0]!.conditional ? { leafName, variantName, moduleName, mounted: true, wrapped: true, origin: candidates[0]!.origin, reason: null } : { leafName, variantName, moduleName, mounted: true, wrapped: false, origin: null, reason: "requires exactly one aggregate variant with one unambiguous wrapped payload declaration origin" };
  }).sort((left, right) => mutationTaxonomyCompare(left.leafName, right.leafName));
}


export function policyMutationLeafHasRunnableTestView(view: MutationTaxonomyStructuralSourceView, leafRel: string, rustFilename: string): boolean {
  const pending = [{ sourcePath: `${leafRel}/${rustFilename}`, moduleBase: leafRel }], visited = new Set<string>();
  while (pending.length > 0) {
    const { sourcePath, moduleBase } = pending.pop()!;
    if (!sourcePath.startsWith(`${leafRel}/`) || visited.has(sourcePath)) continue;
    const source = policyStructuralSource(view, sourcePath);
    if (source === null) continue;
    visited.add(sourcePath);
    const facts = inspectRustRunnableTests(source);
    if (facts.runnableTests.length > 0) return true;
    for (const module of facts.mountedModules) {
      if (module.configuration !== "enabled" || module.pathTarget === null) continue;
      const targetLocator = policyStructuralRelativeLocator(module.pathTarget);
      if (targetLocator === null || module.mountBase.some((segment) => policyStructuralRelativeLocator(segment) === null)) continue;
      const target = `${[moduleBase, ...module.mountBase, targetLocator].join("/")}`;
      if (target.startsWith(`${leafRel}/`) && !visited.has(target)) pending.push({ sourcePath: target, moduleBase: posix.dirname(target) });
    }
  }
  return false;
}


/** 🧬️ Reports every mandatory direct-leaf structural invariant at high severity. */
export function policyMutationStructuralBreachesView(view: MutationTaxonomyStructuralSourceView, mutationRoots: readonly string[] = view.roots): BreachRecord[] {
  const validatedRoots = policyStructuralMutationRoots(view, mutationRoots);
  const breaches: BreachRecord[] = [...policyMutationDirectOwnerBreachesView(view, validatedRoots)];
  for (const mutationsRel of validatedRoots) {
    const rootRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    let rootSource = "";
    const capturedRoot = policyStructuralSource(view, rootRel);
    if (capturedRoot === null) breaches.push(policyMutationStructuralBreach("mutation/reachability", rootRel, `"${rootRel}" cannot be inspected from the captured source view`, "Provide a visible regular aggregate source file in the admitted source set; symlinked, absent, and out-of-view sources cannot prove reachability."));
    else { rootSource = capturedRoot; breaches.push(...policyMutationRootPurityBreaches(mutationsRel, rootSource)); }
    const rootFacts = inspectRustStructure(rootSource);
    const inspectMutationInputs = createRustMutationInputInspector(rootSource);
    const inspectMutationCodecOwnership = createRustMutationCodecOwnershipInspector(rootSource);
    const variants = new Set(policyMutationEnumVariantNames(rootSource));
    const mutationChildren = policyStructuralMutationChildren(view, mutationsRel);
    const leafNames = mutationChildren.filter((child) => child.classification === "direct-owner").map((child) => child.name);
    for (const child of mutationChildren) {
      if (child.classification === "direct-owner" || child.classification === "domain-owner" || child.classification === "root-file-evidence" || child.classification === "absent" || child.classification === "repository-boundary") continue;
      const detail = child.classification === "root-infrastructure" ? "is an optional mutation facet at the collection root"
        : child.classification === "missing-directory-candidate" ? "is a mutation-named regular file without its required direct owner directory"
        : child.classification === "nonregular-or-unadmitted" ? "is nonregular, conflicted, or outside the captured regular source view"
        : child.classification === "unsafe-child" ? "is an unsafe opaque direct child"
        : "is not a canonical direct mutation owner";
      breaches.push(policyMutationStructuralBreach("mutation/direct-owner", child.path, `"${child.path}" ${detail}`, "Account for the direct child explicitly: retain canonical root files as evidence, move optional facets below a valid direct owner, and make every concrete mutation a regular canonical owner directory."));
    }
    const taxonomy = JSON.parse(view.taxonomySchema.bytes.toString("utf8")) as ReturnType<typeof loadTaxonomy>;
    const rustFilename = canonicalPrimaryFilenameForKind(taxonomy.componentFileKinds["🦀️rust"]!, taxonomy);
    const reachability = inspectMutationRootReachabilityView(view, mutationsRel, rootSource, leafNames, rustFilename);
    const folderVariants = new Set(leafNames.map((name) => policyKebabToPascal(policyMutationSemanticIdentity(mutationsRel, name, taxonomy))));
    const missingFolders = [...variants].filter((name) => !folderVariants.has(name));
    const missingVariants = [...folderVariants].filter((name) => !variants.has(name));
    if (missingFolders.length > 0 || missingVariants.length > 0 || reachability.some((proof) => !proof.mounted || !proof.wrapped)) breaches.push(policyMutationStructuralBreach("mutation/folder-variant-bijection", mutationsRel, `"${mutationsRel}" lacks an exact public mount/wrapped-type/folder correspondence: ${reachability.filter((proof) => !proof.mounted || !proof.wrapped).map((proof) => `${proof.leafName}: ${proof.reason}`).join("; ") || `${missingFolders.length} orphan variant(s), ${missingVariants.length} orphan folder(s)`}`, "Make every aggregate variant a one-field wrapper of exactly one publicly mounted canonical direct leaf type, with matching semantic folder identity."));
    if (variants.has("NoMutation")) breaches.push(policyMutationStructuralBreach("mutation/no-sentinel", rootRel, `"${rootRel}" declares NoMutation`, "Represent absence with Option, an empty plan, or the protocol's explicit no-change outcome."));
    if (variants.has("SetSnapshot")) breaches.push(policyMutationStructuralBreach("mutation/no-generic-snapshot-fallback", rootRel, `"${rootRel}" declares SetSnapshot`, "Remove the fallback or replace a proven independent whole-snapshot operation with an explicitly reviewed replace-snapshot leaf."));
    if (rootFacts.includes.length > 0) breaches.push(policyMutationStructuralBreach("mutation/no-hidden-generation", rootRel, `"${rootRel}" hides mutation implementation behind generated/include machinery`, "Keep all concrete variants, payload types, descriptors, and behavior visibly source-controlled in their direct leaves."));
    const seenKinds = new Map<string, string>();
    const seenTextOpcodes = new Map<string, string>();
    const seenBinaryTags = new Map<number, string>();
    const descriptorFilename = canonicalPrimaryFilenameForKind(taxonomy.mutationDescriptorFileKindId, taxonomy);
    const surfaceSpecs = [
      { id: "typescript", root: `${mutationsRel}/${POLICY_TS_COMPONENT_LEAF}`, leaf: POLICY_TS_COMPONENT_LEAF, policy: "mutation/language-parity" as const },
      { id: "graphql", root: `${mutationsRel}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🔗️graphql"].fileKindId, taxonomy)}`, leaf: canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🔗️graphql"].fileKindId, taxonomy), policy: "mutation/schema-parity" as const },
      { id: "protobuf", root: `${mutationsRel}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🛰️protobuf"].fileKindId, taxonomy)}`, leaf: canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🛰️protobuf"].fileKindId, taxonomy), policy: "mutation/schema-parity" as const },
      { id: "json-schema", root: `${mutationsRel}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🔣️jsonschema"].fileKindId, taxonomy)}`, leaf: mutationPayloadSchemaRelativePath(taxonomy), policy: "mutation/schema-parity" as const },
      { id: "text", root: `${mutationsRel}/📝️text/${POLICY_RS_COMPONENT_LEAF_NAME}`, leaf: `📝️text/${POLICY_RS_COMPONENT_LEAF_NAME}`, policy: "mutation/language-parity" as const },
      { id: "binary", root: `${mutationsRel}/💾️binary/${POLICY_RS_COMPONENT_LEAF_NAME}`, leaf: `💾️binary/${POLICY_RS_COMPONENT_LEAF_NAME}`, policy: "mutation/language-parity" as const },
    ].map((surface) => {
      const source = policyStructuralSource(view, surface.root) ?? "";
      return { ...surface, source, identities: surface.id === "text" || surface.id === "binary" ? new Set(inspectRustSourceIdentities(source)) : null };
    });
    const aggregate = policyMutationAggregateMembers(view, mutationsRel, surfaceSpecs.find((surface) => surface.id === "json-schema")!.root, leafNames, taxonomy);
    breaches.push(...aggregate.breaches);
    const subsetRoot = mutationsRel.endsWith("/🧬️schema/🧬️mutations") ? mutationsRel.slice(0, -"/🧬️schema/🧬️mutations".length) : null;
    const catalogRel = subsetRoot ? `${subsetRoot}/${taxonomy.testOraclesDirName}/${canonicalPrimaryFilenameForKind(taxonomy.testContributionFileKindId, taxonomy)}` : null;
    const catalogSource = catalogRel ? policyStructuralSource(view, catalogRel) ?? "" : "";
    for (const leafName of leafNames) {
      const leafRel = `${mutationsRel}/${leafName}`;
      const directRel = `${leafRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`, raw = policyStructuralSource(view, directRel);
      if (raw === null) continue;
      const facts = inspectRustStructure(raw);
      const semanticKind = policyMutationSemanticIdentity(mutationsRel, leafName, taxonomy);
      const moduleName = semanticKind.replaceAll("-", "_");
      const variantName = policyKebabToPascal(semanticKind);
      const descriptorRel = `${leafRel}/${descriptorFilename}`;
      const descriptorResult = policyMutationDescriptorView(view, descriptorRel);
      const descriptor = descriptorResult.descriptor;
      if (!descriptor) breaches.push(policyMutationStructuralBreach("mutation/descriptor-bijection", descriptorRel, `"${descriptorRel}" ${descriptorResult.problem}`, `Add one ${descriptorFilename} conforming to ${MUTATION_DESCRIPTOR_SCHEMA_REL}.`));
      else {
        const identityProblems: string[] = [];
        if (descriptor.owner !== leafRel) identityProblems.push(`owner=${JSON.stringify(descriptor.owner)}`);
        if (descriptor.semanticKind !== semanticKind) identityProblems.push(`semanticKind=${JSON.stringify(descriptor.semanticKind)}`);
        if (descriptor.emoji !== policyLeadingEmojiPrefix(posix.basename(leafName))) identityProblems.push(`emoji=${JSON.stringify(descriptor.emoji)}`);
        if (descriptor.aggregateVariant !== variantName) identityProblems.push(`aggregateVariant=${JSON.stringify(descriptor.aggregateVariant)}`);
        const [payloadLeaf, payloadType, extraPayloadFragment] = descriptor.payloadSchema.split("#");
        const payloadProblems = descriptor.requiredLanguageSurfaces.includes("json-schema") ? policyMutationPayloadSchemaProblems(view, leafRel, descriptor.payloadSchema, taxonomy) : payloadLeaf === POLICY_RS_COMPONENT_LEAF_NAME && /^[A-Za-z_][A-Za-z0-9_]*$/u.test(payloadType ?? "") && extraPayloadFragment === undefined ? [] : ["non-JSON payload must name the direct Rust type"];
        if (payloadProblems.length > 0) identityProblems.push(`payloadSchema=${JSON.stringify(descriptor.payloadSchema)}: ${payloadProblems.join("; ")}`);
        if (identityProblems.length > 0) breaches.push(policyMutationStructuralBreach("mutation/descriptor-bijection", descriptorRel, `"${descriptorRel}" identity disagrees with its direct owner: ${identityProblems.join(", ")}`, "Make the language-neutral descriptor exactly identify its owner folder, aggregate variant, payload schema, and completed classifications."));
        if (descriptor.textOpcode !== null) {
          const prior = seenTextOpcodes.get(descriptor.textOpcode);
          if (prior) breaches.push(policyMutationStructuralBreach("mutation/wire-identity", descriptorRel, `"${descriptorRel}" duplicates text opcode "${descriptor.textOpcode}" from "${prior}"`, "Give every text-capable leaf a unique stable opcode."));
          else seenTextOpcodes.set(descriptor.textOpcode, descriptorRel);
        }
        if (descriptor.binaryTag !== null) {
          const prior = seenBinaryTags.get(descriptor.binaryTag);
          if (prior) breaches.push(policyMutationStructuralBreach("mutation/wire-identity", descriptorRel, `"${descriptorRel}" duplicates binary tag ${descriptor.binaryTag} from "${prior}"`, "Give every binary-capable leaf a unique stable tag."));
          else seenBinaryTags.set(descriptor.binaryTag, descriptorRel);
        }
        for (const surface of surfaceSpecs) {
          const rootExists = policyStructuralSource(view, surface.root) !== null;
          const descriptorRequires = descriptor.requiredLanguageSurfaces.includes(surface.id);
          if (!rootExists && !descriptorRequires) continue;
          const leafSurfaceRel = `${leafRel}/${surface.id === "json-schema" ? descriptor.payloadSchema : surface.leaf}`;
          const leafSurfaceSource = policyStructuralSource(view, leafSurfaceRel) ?? "";
          const names = [semanticKind, variantName, `${variantName}Mutation`, moduleName, ...facts.inlinePayloads.map((payload) => payload.name), ...facts.enums.map((item) => item.name)];
          const leafIdentities = surface.identities === null ? null : new Set(inspectRustSourceIdentities(leafSurfaceSource));
          const binaryConstants = surface.id === "binary" ? inspectRustStructure(leafSurfaceSource).constants : [];
          const binaryTag = binaryConstants.find((constant) => constant.name === "BINARY_TAG");
          const binaryTagMatches = binaryTag !== undefined && descriptor.binaryTag !== null && policyMutationBinaryTag(binaryTag.value, binaryConstants) === descriptor.binaryTag;
          const leafHasIdentity = (leafIdentities ? names.some((name) => leafIdentities.has(name)) || binaryTagMatches : leafSurfaceSource.includes(semanticKind) || leafSurfaceSource.includes(variantName)) && (binaryTag === undefined || binaryTagMatches);
          const rootHasIdentity = surface.id === "json-schema" ? aggregate.referenced.has(leafName) : surface.identities ? names.some((name) => surface.identities!.has(name)) : surface.source.includes(semanticKind) || surface.source.includes(variantName);
          if (binaryTag && !binaryTagMatches) breaches.push(policyMutationStructuralBreach("mutation/wire-identity", leafSurfaceRel, `"${leafSurfaceRel}" binary tag ${binaryTag.value} does not equal descriptor tag ${descriptor.binaryTag}`, "Use the same exact numeric tag in the direct binary contribution and its language-neutral descriptor."));
          if (!rootExists || !descriptorRequires || !leafSurfaceSource || !leafHasIdentity) breaches.push(policyMutationStructuralBreach(surface.policy, leafSurfaceRel, `"${descriptorRel}" does not have a complete direct ${surface.id} counterpart`, `Declare ${surface.id} in requiredLanguageSurfaces and add visible descriptor-backed identities to ${surface.root} and ${leafSurfaceRel}.`));
          if (!rootHasIdentity) breaches.push(policyMutationStructuralBreach(surface.policy, surface.root, `"${surface.root}" omits ${semanticKind}/${variantName}`, `Add the descriptor-backed ${surface.id} union, discriminator, opcode, or tag entry for ${semanticKind}.`));
        }
        if (catalogRel && catalogSource && !catalogSource.includes(semanticKind) && !catalogSource.includes(variantName)) breaches.push(policyMutationStructuralBreach("mutation/catalog-parity", catalogRel, `"${catalogRel}" omits direct mutation ${semanticKind}`, "Derive or verify the catalog/roster from the same direct leaf descriptor set."));
      }
      const proof = reachability.find((entry) => entry.leafName === leafName);
      if (!proof?.mounted || !proof.wrapped) breaches.push(policyMutationStructuralBreach("mutation/reachability", directRel, `"${directRel}" is not proven reachable from its aggregate: ${proof?.reason ?? "missing reachability proof"}`, `Publicly mount ${moduleName} from ${leafName}/${rustFilename} and wrap its Mutation type in ${variantName}.`));
      if (!facts.impls.some((implementation) => implementation.traitPath?.includes("Mutation"))) breaches.push(policyMutationStructuralBreach("mutation/behavior-ownership", directRel, `"${directRel}" does not visibly own mutation behavior`, "Move or mount apply, diff, inverse, validation, and typed-outcome behavior through this direct component."));
      const inputCarriers = inspectMutationInputs(raw);
      if (inputCarriers.length > 0) breaches.push(policyMutationStructuralBreach("mutation/no-generic-snapshot-fallback", directRel, `"${directRel}" accepts unrestricted aggregate state through ${inputCarriers.join("; ")}`, "Remove aggregate Diff/Snapshot payload carriers, including aliases and nested restore phases; return explicit semantic inverse mutations with operation-local prior values."));
      if (!policyMutationLeafHasRunnableTestView(view, leafRel, rustFilename)) breaches.push(policyMutationStructuralBreach("mutation/test-presence", directRel, `"${directRel}" has no enabled, reachable, non-ignored leaf-owned Rust test`, "Add an enabled #[test] function in the direct leaf or an explicitly mounted leaf-owned test module; do not rely on empty directories, comments, ignored tests, or unproven cfg conditions."));
      const rustSemanticKind = facts.constants.find((constant) => constant.name === "SEMANTICS")?.identityFields.kind ?? facts.constants.find((constant) => constant.name === "SEMANTIC_KIND")?.stringValue ?? undefined;
      if (!rustSemanticKind || rustSemanticKind !== descriptor?.semanticKind) breaches.push(policyMutationStructuralBreach("mutation/wire-identity", directRel, `"${directRel}" Rust semantic kind does not equal its language-neutral descriptor`, "Expose one stable Rust semantic kind that exactly mirrors the direct leaf descriptor."));
      else if (seenKinds.has(rustSemanticKind)) breaches.push(policyMutationStructuralBreach("mutation/wire-identity", directRel, `"${directRel}" duplicates semantic kind "${rustSemanticKind}" from "${seenKinds.get(rustSemanticKind)}"`, "Give every direct leaf a unique stable semantic kind, opcode, and binary tag."));
      else seenKinds.set(rustSemanticKind, directRel);
    }
    for (const codec of ["📝️text", "💾️binary"] as const) {
      const codecRel = `${mutationsRel}/${codec}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      const findings = inspectMutationCodecOwnership(policyStructuralSource(view, codecRel) ?? "");
      for (const finding of findings) breaches.push(policyMutationStructuralBreach("mutation/codec-ownership", codecRel, `"${codecRel}" contains executable ${finding.kind.replaceAll("-", " ")}`, "Keep only framing/tokenization/registry lookup here and move every operation's parser/printer/encoder/decoder into its direct leaf."));
    }
    if (variants.has("NoMutation") || variants.has("SetSnapshot")) breaches.push(policyMutationStructuralBreach("mutation/shared-helper-purity", rootRel, `"${rootRel}" contains concrete fallback dispatch`, "Move shared generic helpers to the nearest approved module and keep them free of concrete names, tags, branches, and defaults."));
  }
  return breaches.sort((left, right) => left.scope.localeCompare(right.scope) || left.kind.localeCompare(right.kind));
}


export function policyMutationStructuralBreaches(repoRoot: string, mutationRoots?: readonly string[]): BreachRecord[] {
  const index = mutationTaxonomySourceIndex(repoRoot, {}), view = mutationTaxonomyStructuralView(index);
  return policyMutationStructuralBreachesView(view, mutationRoots ?? view.roots);
}
