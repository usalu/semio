import { randomUUID } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, rmdirSync, rmSync, type Stats, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, posix, relative, resolve } from "node:path";
import {
  canonicalPrimaryFilenameForKind,
  inspectRustModuleGraphFacts,
  inspectRustMutationAggregateSpan,
  inspectRustStructure,
  loadCatalogTaxonomy as loadTaxonomy,
  mutationOwnerIdentity,
  mutationPayloadSchemaRelativePath,
} from "../../🔍️discovery/🟦️.ts";
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

export type NewMutationScaffoldOptions = {
  readonly composite?: boolean;
  readonly text?: boolean;
  readonly binary?: boolean;
  readonly typescript?: boolean;
  readonly graphql?: boolean;
  readonly protobuf?: boolean;
  readonly jsonSchema?: boolean;
  readonly cancelled?: () => boolean;
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
  if (!escaped || escaped === ".." || escaped.startsWith("../") || isAbsolute(escaped)) throw new Error(`new mutation: path escapes repository root: ${JSON.stringify(relPath)}`);
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

export function newMutationUpdateAggregate(repoRoot: string, mutationsRel: string, name: string, parts: ReturnType<typeof newMutationSemanticParts>): { rootRel: string; before: string; source: string; updated: string[] } {
  const rootRel = `${mutationsRel}/${POLICY_RS_COMPONENT_LEAF_NAME}`;
  const rootAbs = newMutationScaffoldPath(repoRoot, rootRel);
  if (!existsSync(rootAbs)) throw new Error(`new mutation: missing aggregate ${rootRel}`);
  if (!lstatSync(rootAbs).isFile()) throw new Error(`new mutation: aggregate ${rootRel} is not a regular file.`);
  const source = readFileSync(rootAbs, "utf8");
  const span = inspectRustMutationAggregateSpan(source);
  if (!span) throw new Error(`new mutation: ${rootRel} must contain exactly one public aggregate Mutation enum.`);
  const facts = inspectRustStructure(source);
  const aggregate = facts.enums.find((item) => item.name === span.enumName);
  if (!aggregate) throw new Error(`new mutation: ${rootRel} aggregate source map did not resolve an enum.`);
  const mount = `#[path = "${name}/${POLICY_RS_COMPONENT_LEAF_NAME}"]\npub mod ${parts.moduleName};`;
  const edits: string[] = [];
  let next = source;
  const existingMounts = inspectRustModuleGraphFacts(source).modules.filter((item) => item.modulePath.length === 1 && item.name === parts.moduleName);
  if (existingMounts.length > 1) throw new Error(`new mutation: existing mount ${parts.moduleName} is ambiguous.`);
  const existingMount = existingMounts[0];
  if (existingMount && (existingMount.visibility !== "pub" || existingMount.inline || existingMount.pathTarget !== `${name}/${POLICY_RS_COMPONENT_LEAF_NAME}`)) {
    throw new Error(`new mutation: existing mount ${parts.moduleName} does not target ${name}/${POLICY_RS_COMPONENT_LEAF_NAME}.`);
  }
  if (!existingMount) {
    next = `${next.slice(0, span.declarationStart)}${mount}\n\n${next.slice(span.declarationStart)}`;
    edits.push(`mount ${parts.moduleName}`);
  }
  const existingVariants = aggregate.variants.filter((item) => item.name === parts.variantName);
  if (existingVariants.length > 1) throw new Error(`new mutation: existing variant ${parts.variantName} is ambiguous.`);
  const existingVariant = existingVariants[0];
  const expectedWrappedType = `${parts.moduleName}::Mutation`;
  if (existingVariant && (existingVariant.fieldStyle !== "tuple" || existingVariant.fieldTypes.length !== 1 || existingVariant.wrappedTupleLeafType?.replaceAll(/\s+/gu, "") !== expectedWrappedType)) {
    throw new Error(`new mutation: existing variant ${parts.variantName} does not wrap ${expectedWrappedType}.`);
  }
  if (!existingVariant) {
    const refreshed = inspectRustMutationAggregateSpan(next);
    if (!refreshed || refreshed.enumName !== span.enumName) throw new Error(`new mutation: ${rootRel} aggregate source map changed during preparation.`);
    next = `${next.slice(0, refreshed.bodyOpen + 1)}\n    ${parts.variantName}(${parts.moduleName}::Mutation),${next.slice(refreshed.bodyOpen + 1)}`;
    edits.push(`variant ${parts.variantName}`);
  }
  return { rootRel, before: source, source: next, updated: edits };
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
  const aggregate = newMutationUpdateAggregate(normalizedRoot, rel, name, parts);
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
  if (dryRun) return { created: created.map((entry) => entry.relPath), skipped, updated: aggregate.updated };
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
    if (aggregate.updated.length > 0) {
      newMutationCheckCancellation(options);
      const aggregateAbs = newMutationScaffoldPath(normalizedRoot, aggregate.rootRel);
      const aggregateStat = newMutationScaffoldLstat(aggregateAbs);
      if (!aggregateStat || aggregateStat.isSymbolicLink() || !aggregateStat.isFile()) throw new Error("new mutation: aggregate changed to a non-regular file during publication.");
      if (readFileSync(aggregateAbs, "utf8") !== aggregate.before) throw new Error("new mutation: aggregate changed during publication.");
      const temporaryAbs = newMutationScaffoldPath(normalizedRoot, `${aggregate.rootRel}.scaffold-${process.pid}-${randomUUID()}.tmp`);
      writeFileSync(temporaryAbs, aggregate.source, { flag: "wx" });
      temporary = newMutationScaffoldOwnPath(temporaryAbs, aggregate.source);
      renameSync(temporary.absolute, aggregateAbs);
      temporary = null;
    }
  } catch (error) {
    if (temporary) newMutationScaffoldRemoveOwnedFile(temporary);
    for (const entry of published.reverse()) newMutationScaffoldRemoveOwnedFile(entry);
    for (const directory of createdDirectories.reverse()) newMutationScaffoldRemoveOwnedDirectory(directory);
    throw error;
  }
  return { created: created.map((entry) => entry.relPath), skipped, updated: aggregate.updated };
}
//#endregion 🧬️MutationScaffolding
