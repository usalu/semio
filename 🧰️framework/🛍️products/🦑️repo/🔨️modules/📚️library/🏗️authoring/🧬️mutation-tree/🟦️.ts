import { randomUUID } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmdirSync, rmSync, type Stats, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, posix, relative, resolve } from "node:path";
import { canonicalPrimaryFilenameForKind, loadCatalogTaxonomy as loadTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { inspectRustModuleGraphFacts, rustModuleScopeProof } from "../../🔍️discovery/🌐️rust-module-graph/🟦️.ts";
import { inspectRustMutationAggregateSpan } from "../../🔍️discovery/🔎️rust-source-identities/🟦️.ts";
import { inspectRustStructure } from "../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { mutationOwnerIdentity } from "../../🔍️discovery/🪪️mutation-owner-identity/🟦️.ts";
import { mutationPayloadSchemaRelativePath } from "../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import {
  POLICY_MUTATION_PLAN_DIR,
  POLICY_MUTATIONS_FACET,
  POLICY_RS_COMPONENT_LEAF_NAME,
  POLICY_TS_COMPONENT_LEAF,
  policyLeadingEmojiPrefix,
  policyStripEmoji,
  policyStructuralRelativeLocator,
} from "../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyListMutationDirs } from "../../🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
import { policyKebabToPascal } from "../../🧹️normalization/🧬️mutation/📐️structural-reachability/🟦️.ts";
import { NEW_SCAFFOLD_MARKER, newScaffoldRustLeaf, NEW_SCAFFOLD_TICKET_PATH, newScaffoldTsLeaf } from "../🧱️contract/🟦️.ts";
import { CRATE_HARD_BUDGET, CRATE_TARGET_BUDGET } from "../📦️crate-tree/🗺️plan/💰️budget/🟦️.ts";
import { measureRustCrate } from "../📦️crate-tree/🗺️plan/📏️measure/🟦️.ts";
import { resolveOwningCrate } from "../📦️crate-tree/🗺️plan/📦️cargo-lib/🟦️.ts";
import { addWorkspaceMembers, insertBuildOwnerRows, manifestRowKey, patchCargoLock, patchRouterPackages, readManifestDependencyRows, rebaseManifestRow, relativePosix, renderCrateManifest } from "../📦️crate-tree/📝️render/🟦️.ts";

export type NewMutationScaffoldOptions = {
  readonly composite?: boolean;
  readonly text?: boolean;
  readonly binary?: boolean;
  readonly typescript?: boolean;
  readonly graphql?: boolean;
  readonly protobuf?: boolean;
  readonly jsonSchema?: boolean;
  readonly cancelled?: () => boolean;
  readonly familyCrate?: boolean;
};

export type NewMutationSemanticParts = { emoji: string; semanticKind: string; moduleName: string; variantName: string; verb: string; entity: string };

export function newMutationSemanticParts(name: string, explicitIdentity?: string): NewMutationSemanticParts {
  const emoji = policyLeadingEmojiPrefix(posix.basename(name));
  const semanticKind = explicitIdentity ?? policyStripEmoji(name);
  const parts = semanticKind.split("-").filter(Boolean);
  if (!emoji || parts.length < 2 || parts.some((part) => !/^[a-z][a-z0-9]*$/u.test(part))) throw new Error(`new mutation: "${name}" must be an emoji-prefixed semantic verb-noun kebab name.`);
  const [verb, ...entityParts] = parts;
  return { emoji, semanticKind, moduleName: semanticKind.replaceAll("-", "_"), variantName: policyKebabToPascal(semanticKind), verb: verb!, entity: entityParts.join("-") };
}

export function newMutationRustLeaf(parts: ReturnType<typeof newMutationSemanticParts>): string {
  return [
    `//! 🧬️ ${NEW_SCAFFOLD_MARKER}: authoritative direct mutation owner for \`${parts.semanticKind}\`.`,
    `//! @see ${NEW_SCAFFOLD_TICKET_PATH}`,
    "",
    "//#region 🪪️Descriptor",
    `pub const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "${parts.verb}", entity: "${parts.entity}", kind: "${parts.semanticKind}", record: "${parts.variantName}" };`,
    "//#endregion 🪪️Descriptor",
    "",
    "//#region 🧬️Mutation",
    "#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]",
    "pub struct Mutation;",
    "//#endregion 🧬️Mutation",
    "",
  ].join("\n");
}

export function newMutationDescriptor(owner: string, parts: ReturnType<typeof newMutationSemanticParts>, options: NewMutationScaffoldOptions): string {
  return `${JSON.stringify(
    {
      schemaVersion: 1,
      owner,
      semanticKind: parts.semanticKind,
      displayName: parts.semanticKind
        .split("-")
        .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
        .join(" "),
      emoji: parts.emoji,
      aggregateVariant: parts.variantName,
      payloadSchema: options.jsonSchema ? mutationPayloadSchemaRelativePath() : `${POLICY_RS_COMPONENT_LEAF_NAME}#Mutation`,
      textOpcode: options.text ? parts.semanticKind : null,
      binaryTag: null,
      invertibility: options.composite ? "plan" : "explicit-mutation",
      diffParticipation: options.composite ? "plan" : "detect",
      outcomeClasses: ["applied"],
      composition: options.composite ? "composite" : "atomic",
      requiredLanguageSurfaces: [
        "rust",
        ...(options.typescript ? ["typescript"] : []),
        ...(options.graphql ? ["graphql"] : []),
        ...(options.protobuf ? ["protobuf"] : []),
        ...(options.jsonSchema ? ["json-schema"] : []),
        ...(options.text ? ["text"] : []),
        ...(options.binary ? ["binary"] : []),
      ],
    },
    null,
    2,
  )}\n`;
}

export type NewMutationScaffoldOwnedPath = { readonly absolute: string; readonly content?: string; readonly device: number; readonly inode: number };

export function newMutationScaffoldLstat(path: string): Stats | null {
  try {
    return lstatSync(path);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return null;
    throw error;
  }
}

export function newMutationScaffoldPath(repoRoot: string, relPath: string): string {
  const rootStat = newMutationScaffoldLstat(repoRoot);
  if (!rootStat || rootStat.isSymbolicLink() || !rootStat.isDirectory()) throw new Error(`new mutation: repository root is not a regular directory: ${JSON.stringify(repoRoot)}`);
  const absolute = resolve(repoRoot, relPath);
  const escaped = relative(repoRoot, absolute).replaceAll("\\", "/");
  if (!escaped || escaped === ".." || escaped.startsWith("..") || isAbsolute(escaped)) throw new Error(`new mutation: path escapes repository root: ${JSON.stringify(relPath)}`);
  let cursor = repoRoot;
  const segments = escaped.split("/");
  for (const [index, segment] of segments.entries()) {
    cursor = join(cursor, segment);
    const stat = newMutationScaffoldLstat(cursor);
    if (!stat) continue;
    if (stat.isSymbolicLink()) throw new Error(`new mutation: scope contains a symlinked path and is not writable: ${JSON.stringify(relPath)}`);
    if (index < segments.length - 1 && !stat.isDirectory()) throw new Error(`new mutation: non-directory path segment is not writable: ${JSON.stringify(relPath)}`);
  }
  return absolute;
}

export function newMutationScaffoldOwnPath(absolute: string, content?: string): NewMutationScaffoldOwnedPath {
  const stat = newMutationScaffoldLstat(absolute);
  if (!stat || stat.isSymbolicLink() || !stat.isFile()) throw new Error(`new mutation: publication did not create a regular file: ${absolute}`);
  return { absolute, content, device: stat.dev, inode: stat.ino };
}

export function newMutationScaffoldRemoveOwnedFile(entry: NewMutationScaffoldOwnedPath): void {
  try {
    const stat = newMutationScaffoldLstat(entry.absolute);
    if (!stat || stat.isSymbolicLink() || !stat.isFile() || stat.dev !== entry.device || stat.ino !== entry.inode) return;
    if (entry.content !== undefined && readFileSync(entry.absolute, "utf8") !== entry.content) return;
    rmSync(entry.absolute, { force: false });
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
  }
}

export function newMutationScaffoldEnsureParents(repoRoot: string, absolute: string): NewMutationScaffoldOwnedPath[] {
  const relParent = relative(repoRoot, dirname(absolute)).replaceAll("\\", "/");
  if (!relParent || relParent === ".") return [];
  const owned: NewMutationScaffoldOwnedPath[] = [];
  let cursor = repoRoot;
  for (const segment of relParent.split("/")) {
    cursor = join(cursor, segment);
    const existing = newMutationScaffoldLstat(cursor);
    if (existing) {
      if (existing.isSymbolicLink() || !existing.isDirectory()) throw new Error(`new mutation: parent is not a regular directory: ${cursor}`);
      continue;
    }
    mkdirSync(cursor);
    const created = newMutationScaffoldLstat(cursor);
    if (!created || created.isSymbolicLink() || !created.isDirectory()) throw new Error(`new mutation: parent creation did not produce a regular directory: ${cursor}`);
    owned.push({ absolute: cursor, device: created.dev, inode: created.ino });
  }
  return owned;
}

export function newMutationScaffoldRemoveOwnedDirectory(entry: NewMutationScaffoldOwnedPath): void {
  try {
    const stat = newMutationScaffoldLstat(entry.absolute);
    if (!stat || stat.isSymbolicLink() || !stat.isDirectory() || stat.dev !== entry.device || stat.ino !== entry.inode) return;
    if (readdirSync(entry.absolute).length > 0) return;
    rmdirSync(entry.absolute);
  } catch (error) {
    if (!(["ENOENT", "ENOTEMPTY"] as const).includes((error as NodeJS.ErrnoException).code as "ENOENT" | "ENOTEMPTY")) throw error;
  }
}

export function newMutationCheckCancellation(options: NewMutationScaffoldOptions): void {
  if (options.cancelled?.()) throw new Error("new mutation: cancelled before publication.");
}

export function newMutationUpdateAggregate(repoRoot: string, mutationsRel: string, name: string, parts: ReturnType<typeof newMutationSemanticParts>, family: { readonly alias: string; readonly crateIdent: string } | null = null): { rootRel: string; before: string; source: string; updated: string[] } {
  const rootRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
  const rootAbs = newMutationScaffoldPath(repoRoot, rootRel);
  if (!existsSync(rootAbs)) throw new Error(`new mutation: missing aggregate ${rootRel}`);
  if (!lstatSync(rootAbs).isFile()) throw new Error(`new mutation: aggregate ${rootRel} is not a regular file.`);
  const source = readFileSync(rootAbs, "utf8");
  const moduleFacts = inspectRustModuleGraphFacts(source);
  if (rustModuleScopeProof(moduleFacts, []).state === "unresolved") throw new Error(`new mutation: ${rootRel} module scope is unproved.`);
  const span = inspectRustMutationAggregateSpan(source);
  if (!span) throw new Error(`new mutation: ${rootRel} must contain exactly one public aggregate Mutation enum.`);
  const facts = inspectRustStructure(source);
  const aggregate = facts.enums.find((item) => item.name === span.enumName);
  if (!aggregate) throw new Error(`new mutation: ${rootRel} aggregate source map did not resolve an enum.`);
  const mount = `#[path = "${name}/${POLICY_RS_COMPONENT_LEAF_NAME}"]\npub mod ${parts.moduleName};`;
  const edits: string[] = [];
  let next = source;
  const existingMounts = moduleFacts.modules.filter((item) => item.modulePath.length === 1 && item.name === parts.moduleName);
  if (existingMounts.length > 1) throw new Error(`new mutation: existing mount ${parts.moduleName} is ambiguous.`);
  const existingMount = existingMounts[0];
  if (family) {
    if (existingMount) throw new Error(`new mutation: ${parts.moduleName} is already mounted in the aggregate; a family crate owns it.`);
    const extern = `extern crate ${family.crateIdent} as ${family.alias};`;
    if (!next.includes(extern)) {
      next = `${next.slice(0, span.declarationStart)}${extern}\n\n${next.slice(span.declarationStart)}`;
      edits.push(`extern ${family.alias}`);
    }
  } else {
    if (existingMount && (existingMount.unresolved || existingMount.visibility !== "pub" || existingMount.inline || existingMount.pathTarget !== `${name}/${POLICY_RS_COMPONENT_LEAF_NAME}`)) {
      throw new Error(`new mutation: existing mount ${parts.moduleName} does not target ${name}/${POLICY_RS_COMPONENT_LEAF_NAME}.`);
    }
    if (!existingMount) {
      next = `${next.slice(0, span.declarationStart)}${mount}\n\n${next.slice(span.declarationStart)}`;
      edits.push(`mount ${parts.moduleName}`);
    }
  }
  const existingVariants = aggregate.variants.filter((item) => item.name === parts.variantName);
  if (existingVariants.length > 1) throw new Error(`new mutation: existing variant ${parts.variantName} is ambiguous.`);
  const existingVariant = existingVariants[0];
  const wrappedPath = family ? `${family.alias}::${parts.moduleName}::Mutation` : `${parts.moduleName}::Mutation`;
  const expectedWrappedType = wrappedPath;
  if (existingVariant && (existingVariant.fieldStyle !== "tuple" || existingVariant.fieldTypes.length !== 1 || existingVariant.wrappedTupleLeafType?.replaceAll(/\s+/gu, "") !== expectedWrappedType)) {
    throw new Error(`new mutation: existing variant ${parts.variantName} does not wrap ${expectedWrappedType}.`);
  }
  if (!existingVariant) {
    const refreshed = inspectRustMutationAggregateSpan(next);
    if (!refreshed || refreshed.enumName !== span.enumName) throw new Error(`new mutation: ${rootRel} aggregate source map changed during preparation.`);
    next = `${next.slice(0, refreshed.bodyOpen + 1)}\n    ${parts.variantName}(${wrappedPath}),${next.slice(refreshed.bodyOpen + 1)}`;
    edits.push(`variant ${parts.variantName}`);
  }
  return { rootRel, before: source, source: next, updated: edits };
}

/** 🏘️ Family crate bookkeeping of one `<domain>/<operation>` leaf: the family root that mounts it and the crate that owns the family. */
export type NewMutationFamilyPlan = { readonly domainRel: string; readonly crateName: string; readonly crateIdent: string; readonly alias: string; readonly packageDir: string; readonly owner: { manifest: string; name: string; entry: string } };

/** 🏘️ Resolves the family crate of `<domain>/<operation>` below a mutations root, or refuses when no Cargo crate owns the root. */
export function newMutationFamilyPlan(repoRoot: string, mutationsRel: string, name: string, parts: ReturnType<typeof newMutationSemanticParts>): NewMutationFamilyPlan {
  const [domain, ...rest] = name.split("/");
  if (!domain || rest.length !== 1) throw new Error(`new mutation: a family leaf is named <domain>/<operation>, got ${JSON.stringify(name)}.`);
  const owner = resolveOwningCrate(repoRoot, mutationsRel);
  if (!owner) throw new Error(`new mutation: no Cargo crate owns ${mutationsRel}; a family crate needs an owning crate to compose it.`);
  const slug = policyStripEmoji(domain).replaceAll(/[^A-Za-z0-9]+/gu, "-").replace(/^-|-$/gu, "").toLowerCase();
  const crateName = `${owner.name}-mut-${slug}`;
  const crateIdent = crateName.replaceAll("-", "_");
  return { domainRel: `${mutationsRel}/${domain}`, crateName, crateIdent, alias: `${slug.replaceAll("-", "_")}_mutations`, packageDir: `${mutationsRel}/${domain}/📦️packages/🦀️rust`, owner };
}

/** 🏘️ Appends the `#[path]` mount of a family leaf to its family root text (creating the root text when the family has none). */
export function newMutationFamilyRoot(existing: string | null, domainLabel: string, name: string, parts: ReturnType<typeof newMutationSemanticParts>): { source: string; added: boolean } {
  const operation = name.split("/")[1]!;
  const mount = `#[path = "${operation}/${POLICY_RS_COMPONENT_LEAF_NAME}"]\npub mod ${parts.moduleName};`;
  if (existing !== null && existing.includes(mount)) return { source: existing, added: false };
  const head = existing ?? `//! 🧬️ ${NEW_SCAFFOLD_MARKER}: mutation family \`${domainLabel}\` — one crate of leaf payloads composed by the transparent aggregate; every leaf stays generic over its aggregate.\n\n#![allow(async_fn_in_trait)]\n\nextern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\n`;
  return { source: `${head.trimEnd()}\n\n${mount}\n`, added: true };
}

/** 📏️ What adding one mutation leaf does to the crate that owns the aggregate. */
export type MutationCrateBudgetAssessment = Readonly<{ crate: string; lib: { files: number; bytes: number; lines: number }; test: { files: number; bytes: number; lines: number }; verdict: "fits" | "near-limit" | "refused" | "already-over" }>;

/** 📏️ Measures the crate owning a mutations root with the canonical source counter and decides whether one more leaf (one mounted file, three active lines) keeps it inside the source gate. Returns null where no Cargo crate owns the root. */
export async function assessMutationCrateBudget(repoRoot: string, mutationsRel: string): Promise<MutationCrateBudgetAssessment | null> {
  const owner = resolveOwningCrate(repoRoot, mutationsRel);
  if (!owner) return null;
  const lib = await measureRustCrate(repoRoot, owner.entry, false);
  const test = await measureRustCrate(repoRoot, owner.entry, true);
  const over = (measure: { files: number; bytes: number; lines: number }, budget: { files: number; bytes: number; lines: number }): boolean => measure.files > budget.files || measure.bytes > budget.bytes || measure.lines > budget.lines;
  const grown = (measure: { files: number; bytes: number; lines: number }) => ({ files: measure.files + 1, bytes: measure.bytes + 2048, lines: measure.lines + 12 });
  const alreadyOver = over(lib, CRATE_HARD_BUDGET) || over(test, CRATE_HARD_BUDGET);
  const verdict = alreadyOver ? "already-over" : over(grown(lib), CRATE_HARD_BUDGET) || over(grown(test), CRATE_HARD_BUDGET) ? "refused" : over(grown(lib), CRATE_TARGET_BUDGET) || over(grown(test), CRATE_TARGET_BUDGET) ? "near-limit" : "fits";
  return { crate: owner.name, lib: { files: lib.files, bytes: lib.bytes, lines: lib.lines }, test: { files: test.files, bytes: test.bytes, lines: test.lines }, verdict };
}

/** 🏗️ Scaffolds one direct mutation leaf and visibly wires its aggregate without overwriting leaves. */
export function newScaffoldMutationTree(repoRoot: string, mutationsRel: string, name: string, options: NewMutationScaffoldOptions = {}, dryRun = false): { created: string[]; skipped: string[]; updated: string[] } {
  const normalizedRoot = resolve(repoRoot);
  const mutationRoot = resolve(normalizedRoot, mutationsRel);
  const rel = relative(normalizedRoot, mutationRoot).replaceAll("\\", "/");
  if (rel === "compose" || rel.startsWith("compose/") || rel === "temp/compose" || rel.startsWith("temp/compose/")) throw new Error(`new mutation: scope is excluded from authoring: ${JSON.stringify(mutationsRel)}.`);
  if (policyStructuralRelativeLocator(rel) === null || !rel.endsWith(`/${POLICY_MUTATIONS_FACET}`)) throw new Error(`new mutation: scope must be a safe repository-relative ${POLICY_MUTATIONS_FACET} directory: ${JSON.stringify(mutationsRel)}.`);
  const taxonomy = loadTaxonomy();
  const identity = mutationOwnerIdentity(rel, name, taxonomy);
  if (Object.hasOwn(taxonomy.mutationDomainOwners, rel) && identity === null) throw new Error(`new mutation: owner ${rel}/${name} has no exact domain-operation registration.`);
  const parts = newMutationSemanticParts(name, identity ?? undefined);
  for (const sibling of policyListMutationDirs(normalizedRoot, rel)) {
    if (sibling === name) continue;
    const siblingParts = newMutationSemanticParts(sibling, mutationOwnerIdentity(rel, sibling, taxonomy) ?? undefined);
    if (posix.dirname(sibling) === posix.dirname(name) && siblingParts.emoji === parts.emoji) throw new Error(`new mutation: emoji ${JSON.stringify(parts.emoji)} is already owned by ${sibling}.`);
    if (siblingParts.semanticKind === parts.semanticKind) throw new Error(`new mutation: semantic kind ${JSON.stringify(parts.semanticKind)} is already owned by ${sibling}.`);
    if (siblingParts.variantName === parts.variantName) throw new Error(`new mutation: aggregate variant ${parts.variantName} is already owned by ${sibling}.`);
  }
  const leafRel = `${rel}/${name}`;
  const descriptorFilename = canonicalPrimaryFilenameForKind(taxonomy.mutationDescriptorFileKindId, taxonomy);
  const proposed: { relPath: string; content: string }[] = [
    { relPath: `${leafRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`, content: newMutationRustLeaf(parts) },
    { relPath: `${leafRel}/${descriptorFilename}`, content: newMutationDescriptor(leafRel, parts, options) },
    { relPath: `${leafRel}/🧪️tests/${POLICY_RS_COMPONENT_LEAF_NAME}`, content: `//! 🧪️ ${NEW_SCAFFOLD_MARKER}: behavioral and algebraic tests for ${parts.semanticKind}.\n` },
    ...(options.composite ? [{ relPath: `${leafRel}/${POLICY_MUTATION_PLAN_DIR}/${POLICY_RS_COMPONENT_LEAF_NAME}`, content: newScaffoldRustLeaf(`${parts.semanticKind} composite plan`) }] : []),
    ...(options.text ? [{ relPath: `${leafRel}/📝️text/${POLICY_RS_COMPONENT_LEAF_NAME}`, content: newScaffoldRustLeaf(`${parts.semanticKind} text codec contribution`) }] : []),
    ...(options.binary ? [{ relPath: `${leafRel}/💾️binary/${POLICY_RS_COMPONENT_LEAF_NAME}`, content: newScaffoldRustLeaf(`${parts.semanticKind} binary codec contribution`) }] : []),
    ...(options.typescript ? [{ relPath: `${leafRel}/${POLICY_TS_COMPONENT_LEAF}`, content: newScaffoldTsLeaf(`${parts.semanticKind} mutation`) }] : []),
    ...(options.graphql ? [{ relPath: `${leafRel}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🔗️graphql"]!.fileKindId, taxonomy)}`, content: `# 🧬️ ${NEW_SCAFFOLD_MARKER}: ${parts.variantName} GraphQL mutation input.\n` }] : []),
    ...(options.protobuf ? [{ relPath: `${leafRel}/${canonicalPrimaryFilenameForKind(taxonomy.schemaFormats["🛰️protobuf"]!.fileKindId, taxonomy)}`, content: `// 🧬️ ${NEW_SCAFFOLD_MARKER}: ${parts.variantName} protobuf mutation message.\n` }] : []),
    ...(options.jsonSchema ? [{ relPath: `${leafRel}/${mutationPayloadSchemaRelativePath(taxonomy)}`, content: `${JSON.stringify({ $schema: "http://json-schema.org/draft-07/schema#", title: parts.variantName, type: "object" }, null, 2)}\n` }] : []),
  ];
  const family = options.familyCrate ? newMutationFamilyPlan(normalizedRoot, rel, name, parts) : null;
  const aggregate = newMutationUpdateAggregate(normalizedRoot, rel, name, parts, family ? { alias: family.alias, crateIdent: family.crateIdent } : null);
  const updates: { rootRel: string; before: string; source: string; updated: string[] }[] = aggregate.updated.length > 0 ? [aggregate] : [];
  if (family) {
    const domain = name.split("/")[0]!;
    const familyRootRel = `${family.domainRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
    const familyRootAbs = newMutationScaffoldPath(normalizedRoot, familyRootRel);
    const existingRoot = newMutationScaffoldLstat(familyRootAbs) ? readFileSync(familyRootAbs, "utf8") : null;
    const familyRoot = newMutationFamilyRoot(existingRoot, domain, name, parts);
    if (existingRoot === null) proposed.push({ relPath: familyRootRel, content: familyRoot.source });
    else if (familyRoot.added) updates.push({ rootRel: familyRootRel, before: existingRoot, source: familyRoot.source, updated: [`mount ${parts.moduleName}`] });
    const ownerDir = dirname(family.owner.manifest);
    const packageDirAbs = resolve(normalizedRoot, family.packageDir);
    const rows = readManifestDependencyRows(readFileSync(family.owner.manifest, "utf8"));
    const workspaceRel = relativePosix(packageDirAbs, resolve(normalizedRoot, family.owner.manifest, "..", readWorkspacePointer(family.owner.manifest)));
    proposed.push({
      relPath: `${family.packageDir}/Cargo.toml`,
      content: renderCrateManifest({
        name: family.crateName,
        description: `Mutation family ${domain} of ${family.owner.name}`,
        workspaceRel,
        libPath: "../../🦀️.rs",
        dependencyRows: rows.dependencies.map((row) => rebaseManifestRow(row, ownerDir, packageDirAbs)),
        devDependencyRows: rows.devDependencies.map((row) => rebaseManifestRow(row, ownerDir, packageDirAbs)),
        targetSections: rows.targetSections.map((section) => rebaseManifestRow(section, ownerDir, packageDirAbs)),
      }),
    });
  }
  const created: { relPath: string; content: string }[] = [];
  const skipped: string[] = [];
  for (const entry of proposed) {
    const absolute = newMutationScaffoldPath(normalizedRoot, entry.relPath);
    const existing = newMutationScaffoldLstat(absolute);
    if (!existing) created.push(entry);
    else if (existing.isSymbolicLink() || !existing.isFile()) throw new Error(`new mutation: existing target is not a regular file: ${entry.relPath}`);
    else skipped.push(entry.relPath);
  }
  newMutationCheckCancellation(options);
  if (dryRun) return { created: created.map((entry) => entry.relPath), skipped, updated: updates.flatMap((update) => update.updated) };
  const published: NewMutationScaffoldOwnedPath[] = [];
  const createdDirectories: NewMutationScaffoldOwnedPath[] = [];
  let temporary: NewMutationScaffoldOwnedPath | null = null;
  try {
    for (const entry of created) {
      newMutationCheckCancellation(options);
      const absolute = newMutationScaffoldPath(normalizedRoot, entry.relPath);
      createdDirectories.push(...newMutationScaffoldEnsureParents(normalizedRoot, absolute));
      newMutationScaffoldPath(normalizedRoot, entry.relPath);
      writeFileSync(absolute, entry.content, { flag: "wx" });
      published.push(newMutationScaffoldOwnPath(absolute, entry.content));
    }
    for (const update of updates) {
      newMutationCheckCancellation(options);
      const targetAbs = newMutationScaffoldPath(normalizedRoot, update.rootRel);
      const targetStat = newMutationScaffoldLstat(targetAbs);
      if (!targetStat || targetStat.isSymbolicLink() || !targetStat.isFile()) throw new Error(`new mutation: ${update.rootRel} changed to a non-regular file during publication.`);
      if (readFileSync(targetAbs, "utf8") !== update.before) throw new Error(`new mutation: ${update.rootRel} changed during publication.`);
      const temporaryAbs = newMutationScaffoldPath(normalizedRoot, `${update.rootRel}.scaffold-${process.pid}-${randomUUID()}.tmp`);
      writeFileSync(temporaryAbs, update.source, { flag: "wx" });
      temporary = newMutationScaffoldOwnPath(temporaryAbs, update.source);
      renameSync(temporary.absolute, targetAbs);
      temporary = null;
    }
  } catch (error) {
    if (temporary) newMutationScaffoldRemoveOwnedFile(temporary);
    for (const entry of published.reverse()) newMutationScaffoldRemoveOwnedFile(entry);
    for (const directory of createdDirectories.reverse()) newMutationScaffoldRemoveOwnedDirectory(directory);
    throw error;
  }
  const registered = family ? registerMutationFamilyCrate(normalizedRoot, family, created.some((entry) => entry.relPath === `${family.packageDir}/Cargo.toml`)) : [];
  return { created: created.map((entry) => entry.relPath), skipped, updated: [...updates.flatMap((update) => update.updated), ...registered] };
}

function readWorkspacePointer(manifest: string): string {
  return (Bun.TOML.parse(readFileSync(manifest, "utf8")) as { package?: { workspace?: string } }).package?.workspace ?? ".";
}

/** 🏘️ Registers a new family crate with the owning workspace: member list, aggregate dependency, router packages and lockfile. */
function registerMutationFamilyCrate(repoRoot: string, family: NewMutationFamilyPlan, createdManifest: boolean): string[] {
  if (!createdManifest) return [];
  const updated: string[] = [];
  const ownerDir = dirname(family.owner.manifest);
  const workspaceDir = resolve(ownerDir, readWorkspacePointer(family.owner.manifest));
  const packageDirAbs = resolve(repoRoot, family.packageDir);
  const workspaceManifest = join(workspaceDir, "Cargo.toml");
  if (existsSync(workspaceManifest)) {
    const text = readFileSync(workspaceManifest, "utf8");
    const next = addWorkspaceMembers(text, [relativePosix(workspaceDir, packageDirAbs)]);
    if (next !== text) { writeFileSync(workspaceManifest, next); updated.push(`member ${family.crateName}`); }
  }
  const ownerText = readFileSync(family.owner.manifest, "utf8");
  const row = `${family.crateName} = { path = "${relativePosix(ownerDir, packageDirAbs)}" }`;
  const at = ownerText.search(/^\[dependencies\]\s*$/mu);
  if (at >= 0 && !ownerText.includes(`${family.crateName} =`)) {
    const lineEnd = ownerText.indexOf("\n", at) + 1;
    writeFileSync(family.owner.manifest, ownerText.slice(0, lineEnd) + row + "\n" + ownerText.slice(lineEnd));
    updated.push(`dependency ${family.crateName}`);
  }
  const router = join(ownerDir, "📜️script.ts");
  if (existsSync(router)) {
    const text = readFileSync(router, "utf8");
    try { const next = patchRouterPackages(text, family.owner.name, [family.crateName]); if (next !== text) { writeFileSync(router, next); updated.push("router packages"); } } catch { updated.push("router packages unchanged"); }
  }
  const ownersPath = resolve(repoRoot, "📋️build-owners.json");
  if (existsSync(ownersPath)) {
    const raw = readFileSync(ownersPath, "utf8");
    const { text, added } = insertBuildOwnerRows(raw, [{ name: family.crateName, root: family.packageDir }]);
    if (added.length) { writeFileSync(ownersPath, text); updated.push("build owner row"); }
  }
  const lock = join(workspaceDir, "Cargo.lock");
  if (existsSync(lock)) {
    const text = readFileSync(lock, "utf8");
    try { const next = patchCargoLock(text, family.owner.name, [{ name: family.crateName, dependsOn: [] }]); if (next !== text) { writeFileSync(lock, next); updated.push("lockfile"); } } catch { updated.push("lockfile unchanged"); }
  }
  return updated;
}
//#endregion 🧬️MutationScaffolding
