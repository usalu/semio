/** 🌿️ Mutation leaves
 *
 * Mutation leaf descriptors, their payload schemas and the leaf scaffolding.
 */

//#region 🔌️Adapters
import { type MutationOutcomeClass, MUTATION_OUTCOME_CLASSES } from "@semio-tech/framework/test/adapter";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { type BreachRecord } from "../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { TAXONOMY_REL_PATH, testFilenameForKind, testTaxonomy } from "../🗂️taxonomy/🟦️.ts";
import { subsetCoordinatesOfOwner } from "../🪆️subset-policy/🟦️.ts";
import { isDeclaredDialectDefinition, isFixtureOwnedPath, rawTaxonomy, readJson, schemaJsonDialect, schemaModuleDirName } from "../📐️schema-contracts/🟦️.ts";
import { isMutationOutcomeClass, type MutationManifest } from "../📑️mutation-manifest/🟦️.ts";
import { type OracleRegistry } from "../🤝️contributions/🟦️.ts";
import { breach } from "../🚨️breaches/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🪪️LeafDescriptor
/**
 * 🪪️ One mutation leaf's own declaration, the fourteen-field record the `dsl::Mutations` derive reads
 * at expansion time from `🧬️mutations/<kind>/<taxonomy json filename>`.
 *
 * This is the single source Protocol v2 wanted and could not previously reach. It is DECLARATIVE and
 * LANGUAGE-NEUTRAL — a JSON file beside the Rust, read by the derive to generate production
 * registration — so a manifest built from it is generated from the same record production is, rather
 * than restated beside it. Crucially it carries `outcomeClasses`, which is the one field a manifest
 * cannot honestly invent: only the implementation knows which classes a mutation can reach.
 *
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs — `parse_mutation_leaf_descriptor`
 */
export type MutationLeafDescriptor = Readonly<{
  schemaVersion: number;
  owner: string;
  semanticKind: string;
  displayName: string;
  emoji: string;
  aggregateVariant: string;
  payloadSchema: string;
  textOpcode: string | null;
  binaryTag: number | null;
  invertibility: "self" | "explicit-mutation" | "plan" | "non-invertible";
  diffParticipation: "detect" | "apply-only" | "plan" | "none";
  outcomeClasses: readonly MutationOutcomeClass[];
  composition: "atomic" | "composite";
  requiredLanguageSurfaces: readonly string[];
}>;

/** 🎯️ The protocol outcome classes a leaf descriptor declares — carried verbatim, refusing any value outside the protocol vocabulary. */
export function outcomeClassesOf(descriptor: MutationLeafDescriptor): MutationOutcomeClass[] {
  const foreign = descriptor.outcomeClasses.filter((value) => !isMutationOutcomeClass(value));
  if (foreign.length > 0) throw new Error(`${descriptor.owner}: outcomeClasses ${JSON.stringify(foreign)} are outside the protocol vocabulary ${MUTATION_OUTCOME_CLASSES.join("|")}`);
  return [...descriptor.outcomeClasses];
}

/** 🪪️ Every mutation leaf descriptor beneath one owner, keyed by its semantic kind. */
/**
 * 🦠️ Whether a directory under `🧬️mutations` is a MUTATION LEAF rather than a shared facet.
 *
 * `💾️binary`, `📝️text` and `🧬️schema` live beside the leaves and are not leaves. Counting them as such
 * made every owner look permanently undescribed — 21 owners whose every real leaf carried a descriptor
 * were reported as 4, because three facet directories could never have one.
 */
export function isMutationLeafDirectory(repoRoot: string, name: string): boolean {
  const pattern = (rawTaxonomy(repoRoot) as { mutationDirectoryPattern?: unknown }).mutationDirectoryPattern;
  if (typeof pattern !== "string") throw new Error(`${TAXONOMY_REL_PATH} declares no \`mutationDirectoryPattern\``);
  return new RegExp(pattern, "u").test(name.normalize("NFC")) && !mutationFacetDirNames(repoRoot).has(name.normalize("NFC"));
}

/**
 * 🧩️ The directory names that live BESIDE and INSIDE mutation leaves without ever being one.
 *
 * `🧩️plan`, `📝️text`, `💾️binary`, `🧬️schema` (`mutationOrganizationalFacetDirs`) organise a leaf's
 * material; `🦠️mutation`, `🔺️diff`, `↩️inverse` (`mutationBehaviorFacetDirs`) carry its behaviour. Both
 * lists are the taxonomy's, because a walker that spelled them itself would keep counting a facet as
 * a leaf the day the taxonomy renames one — which is how 21 fully described owners once measured 4.
 */
function mutationFacetDirNames(repoRoot: string): ReadonlySet<string> {
  const raw = rawTaxonomy(repoRoot);
  const names = [...(raw.mutationOrganizationalFacetDirs as unknown[] | undefined ?? []), ...(raw.mutationBehaviorFacetDirs as unknown[] | undefined ?? [])];
  return new Set(names.filter((name): name is string => typeof name === "string").map((name) => name.normalize("NFC")));
}

/**
 * 🍃️ Every mutation leaf of one owner, as a path relative to its `🧬️mutations` directory.
 *
 * A leaf sits at depth 1 (`🧬️mutations/<verb-noun>`) or depth 2 (`🧬️mutations/<domain>/<verb>`, the
 * shape execution contract §B admits and the gltf and architect vocabularies use). Reading one level
 * was the whole of ledger row 7: 47 architect `<domain>` grouping directories were reported as
 * undeclared leaves — they can never carry a descriptor — while the 266 real leaves beneath them were
 * never reached at all. `schema check`'s `mutationRootModule` already accepts depth 1 or 2; this is
 * the same rule on this side.
 *
 * The classification is DECLARED, never guessed: a directory carrying a descriptor is a leaf, whatever
 * its name; a directory carrying none whose children qualify is a grouping directory and never a leaf
 * itself; a directory with neither is a leaf only when its name matches the taxonomy's
 * `mutationDirectoryPattern`, so an undescribed leaf is still REPORTED as undescribed rather than
 * silently dropped. Test and fixture collections are excluded at both levels — a case's own
 * contribution file would otherwise read as a leaf descriptor.
 *
 * @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/📓️wp4b-mutations.md
 */
export function mutationLeafDirectories(repoRoot: string, ownerRel: string): string[] {
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const root = join(repoRoot, ownerRel, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  const facets = mutationFacetDirNames(repoRoot);
  const directories = (absDir: string): string[] => {
    try {
      return readdirSync(absDir, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink()).map((entry) => entry.name);
    } catch {
      return [];
    }
  };
  const structural = (name: string): boolean => !facets.has(name.normalize("NFC")) && !isFixtureOwnedPath(repoRoot, name);
  const described = (absDir: string): boolean => existsSync(join(absDir, filename));
  const qualifies = (absParent: string, name: string): boolean => structural(name) && (described(join(absParent, name)) || isMutationLeafDirectory(repoRoot, name));
  const leaves: string[] = [];
  for (const name of directories(root)) {
    if (!structural(name)) continue;
    const absDir = join(root, name);
    if (described(absDir)) {
      leaves.push(name);
      continue;
    }
    const children = directories(absDir).filter((child) => qualifies(absDir, child));
    if (children.length > 0) leaves.push(...children.map((child) => `${name}/${child}`));
    else if (isMutationLeafDirectory(repoRoot, name)) leaves.push(name);
  }
  return leaves.sort();
}

export function readLeafDescriptors(repoRoot: string, owner: string): Map<string, MutationLeafDescriptor> {
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const root = join(repoRoot, owner, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  const found = new Map<string, MutationLeafDescriptor>();
  if (!existsSync(root)) return found;
  for (const leaf of mutationLeafDirectories(repoRoot, owner)) {
    const descriptorPath = join(root, leaf, filename);
    if (!existsSync(descriptorPath)) continue;
    try {
      const parsed = JSON.parse(readFileSync(descriptorPath, "utf8")) as MutationLeafDescriptor;
      if (typeof parsed.semanticKind === "string" && Array.isArray(parsed.outcomeClasses)) found.set(parsed.semanticKind, parsed);
    } catch {
      /* 🧭️A malformed descriptor is the derive's own gate to report; it fails the build. */
    }
  }
  return found;
}

/** 🪪️ How many of an owner's mutation leaves carry a descriptor — the ratio that gates generation. */
export function leafDescriptorCoverage(repoRoot: string, owner: string): { leaves: number; described: number; missing: string[] } {
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const root = join(repoRoot, owner, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  if (!existsSync(root)) return { leaves: 0, described: 0, missing: [] };
  const directories = mutationLeafDirectories(repoRoot, owner);
  const missing = directories.filter((name) => !existsSync(join(root, name, filename)));
  return { leaves: directories.length, described: directories.length - missing.length, missing };
}

/**
 * 🧬️ Builds a v2 mutation manifest for one owner FROM ITS LEAF DESCRIPTORS.
 *
 * Returns `null` when any leaf lacks a descriptor. That refusal is the whole point: a manifest whose
 * outcome classes were guessed for even one mutation would report coverage of an outcome nobody can
 * reach, or hide one everybody can — and a partial manifest is worse than none, because the gate would
 * then measure against a denominator that silently omits the undescribed leaves.
 */
export function manifestFromLeafDescriptors(repoRoot: string, owner: string, capability: string): MutationManifest | null {
  const coordinates = subsetCoordinatesOfOwner(owner);
  if (coordinates === null) return null;
  const coverage = leafDescriptorCoverage(repoRoot, owner);
  if (coverage.leaves === 0 || coverage.missing.length > 0) return null;
  const descriptors = readLeafDescriptors(repoRoot, owner);
  if (descriptors.size !== coverage.leaves) return null;
  const artifact = artifactOfOwner(repoRoot, owner);
  if (artifact === null) return null;
  return {
    schema: "semio.repository-test.mutation-manifest/v2",
    artifact,
    standard: coordinates.standard,
    subset: coordinates.subset,
    standardDirectoryName: coordinates.standardDirectoryName,
    subsetDirectoryName: coordinates.subsetDirectoryName,
    mutations: [...descriptors.values()]
      .sort((a, b) => a.semanticKind.localeCompare(b.semanticKind))
      .map((descriptor) => ({
        id: descriptor.semanticKind,
        capability,
        payloadSchema: descriptor.payloadSchema,
        outcomes: outcomeClassesOf(descriptor),
        productionDispatch: { operation: descriptor.textOpcode ?? descriptor.semanticKind, bridgeVersion: 1, variant: descriptor.aggregateVariant },
        oracleRequirements: [{ capability, qualifyingKind: "third-party-library" as const }],
      })),
  };
}

/** 🗿️ The artifact id an owner path belongs to, read from its own 🪆️subsets component. */
export function artifactOfOwner(repoRoot: string, owner: string): string | null {
  const marker = owner.indexOf("/🏅️standards/");
  if (marker < 0) return null;
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const subsetsRoot = join(repoRoot, owner.slice(0, owner.lastIndexOf("/🪆️subsets/") + "/🪆️subsets".length));
  for (const candidate of [filename, "🔣️.json"]) {
    const manifestPath = join(subsetsRoot, candidate);
    if (!existsSync(manifestPath)) continue;
    try {
      const parsed = JSON.parse(readFileSync(manifestPath, "utf8")) as { artifact?: string };
      if (typeof parsed.artifact === "string") return parsed.artifact;
    } catch {
      return null;
    }
  }
  return null;
}
//#endregion 🪪️LeafDescriptor

//#region 🏗️LeafScaffold
/** 🏗️ One derived descriptor field, with the file and line that justifies it. */
export type DerivedField = Readonly<{ value: unknown; evidence: string }>;

/** 🏗️ A scaffolded descriptor, or the exact reason one field could not be derived. */
export type LeafScaffold = Readonly<{ leaf: string; kind: string; descriptor: MutationLeafDescriptor | null; evidence: Readonly<Record<string, string>>; refused: readonly string[] }>;

function readFirst(paths: readonly string[]): { path: string; text: string } | null {
  for (const path of paths) {
    if (!existsSync(path)) continue;
    try {
      return { path, text: readFileSync(path, "utf8") };
    } catch {
      continue;
    }
  }
  return null;
}

function titleCase(kind: string): string {
  return kind.split("-").map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join(" ");
}

function pascalCase(kind: string): string {
  return kind.split("-").map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join("");
}

/**
 * 🏗️ Derives one mutation leaf's descriptor FROM THE LEAF ITSELF, and refuses any field it cannot
 * justify with a file and a line.
 *
 * The descriptor is the single source a v2 manifest is generated from, and its `outcomeClasses` is the
 * one field nobody can honestly invent from outside the implementation — so this does not invent it.
 * It reads the leaf's own `🔺️diff`, where the semantics actually live: `MutationOutcome::error(…)` is a
 * REJECTED path, `MutationOutcome::empty()` is a NO-OP path, and `MutationOutcome::new(…)` is an
 * APPLIED path. Same for the rest: the aggregate variant comes from the `pub struct` the leaf declares,
 * the binary tag from the `record <kind> tag=<n>` line in the owner's binary protocol, the text opcode
 * from the `#[dsl(keyword = "…")]` attribute. A field with no such line is REFUSED and the leaf is
 * reported, never guessed — a scaffolded descriptor that guessed one field would put a wrong outcome
 * class into production registration AND into the coverage denominator at the same time.
 */
export function scaffoldLeafDescriptor(repoRoot: string, ownerRel: string, leafDirName: string): LeafScaffold {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  const leafAbs = join(vocabulary, leafDirName);
  const leafRel = `${ownerRel}/${schemaModuleDirName(repoRoot)}/${taxonomy.testMutationVocabularyDirName}/${leafDirName}`;
  // 🍃️A two-segment leaf (`<domain>/<verb>`) is named by its OWN directory, never by the grouping
  // directory above it: `🌳️node/⚖️change-weights` declares `change-node-morph-weights`, which no
  // concatenation of the two segments produces. The scaffolder therefore refuses such a leaf rather
  // than inventing a kind — the leaf's descriptor is the authority, and this tool exists to write one.
  const leafName = leafDirName.split("/").at(-1) ?? leafDirName;
  const kind = leafName.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)?.[0] ?? "";
  const emoji = kind.length > 0 ? leafName.slice(0, leafName.length - kind.length) : "";
  const evidence: Record<string, string> = {};
  const refused: string[] = [];

  if (kind.length === 0) refused.push("semanticKind: the directory name does not end in a two-or-more-segment kebab identifier");
  if (emoji.length === 0) refused.push("emoji: the directory name carries no leading emoji");

  // 🦀️The payload struct, and therefore the aggregate variant, is whatever the leaf actually declares.
  // 🧭️`🧬️operation` is the same payload directory under a rename a concurrent session is applying; both
  // spellings are READ here so a half-migrated tree is still measurable. Nothing is written under either.
  const mutationSource = readFirst([join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🦠️mutation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🧬️operation", "🦀️.rs"), join(leafAbs, "🦀️.rs"), join(leafAbs, "🦀️.rs")]);
  let aggregateVariant = "";
  if (mutationSource === null) refused.push("aggregateVariant: the leaf declares no Rust source");
  else {
    // 📄️`set-snapshot` is the whole-document replace, and its payload IS the artifact's snapshot type —
    // there is no dedicated struct to find, which is why all 50 of these refused. The type is stated in
    // the leaf's own apply signature (`apply(projection: &mut DwgSnapshot, …)`), so it is read from
    // there rather than assumed, and the variant follows the name the aggregate already uses for it.
    const snapshotApply = mutationSource.text.match(/fn apply\s*\(\s*[a-z_]+\s*:\s*&mut\s+([A-Za-z0-9_]+)/)?.[1] ?? "";
    const declared = mutationSource.text.match(/^pub struct ([A-Za-z0-9_]+)/m)?.[1] ?? mutationSource.text.match(/^pub enum ([A-Za-z0-9_]+)/m)?.[1] ?? (kind === "set-snapshot" && snapshotApply.length > 0 ? "SetSnapshot" : "");
    if (declared.length === 0) refused.push(`aggregateVariant: no \`pub struct\` in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`);
    else {
      aggregateVariant = declared;
      evidence.aggregateVariant = `pub struct ${declared} in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`;
      if (declared !== pascalCase(kind)) evidence.aggregateVariant += ` (note: differs from the PascalCase of ${kind}, so the declaration wins)`;
    }
  }

  // 🎯️OUTCOME CLASSES, from the diff implementation. This is the field the whole exercise exists for.
  // 🔺️The diff is not always a SUBDIRECTORY. In the taxonomy's canonical leaf layout — the one the
  // `MutationLeaf` derive requires, and the one this migration produces — payload, diff and inverse all
  // live in the leaf's single `🦀️.rs`. 315 leaves are already in that shape, and reading only
  // `🔺️diff/` refused every one of them for evidence that was sitting in the file next door.
  const diffSource = readFirst([join(leafAbs, "🔺️diff", "🦀️.rs"), join(leafAbs, "🔺️diff", "🦀️.rs"), join(leafAbs, "🦀️.rs"), join(leafAbs, "🦀️.rs")]);
  const outcomes = new Set<MutationOutcomeClass>();
  if (diffSource === null) refused.push("outcomeClasses: the leaf declares no 🔺️diff implementation to read them from");
  else {
    const lines = diffSource.text.split(/\r?\n/);
    const cite: string[] = [];
    for (const [index, line] of lines.entries()) {
      if (/MutationOutcome::(?:error|fatal)\s*\(/.test(line)) {
        outcomes.add("rejected");
        cite.push(`rejected@${index + 1}`);
      }
      if (/MutationOutcome::empty\s*\(|"mutation\.no-op"/.test(line)) {
        outcomes.add("no-op");
        cite.push(`no-op@${index + 1}`);
      }
      if (/MutationOutcome::new\s*\(/.test(line)) {
        outcomes.add("applied");
        cite.push(`new@${index + 1}`);
      }
    }
    if (outcomes.size === 0) refused.push(`outcomeClasses: no MutationOutcome:: call in ${relative(repoRoot, diffSource.path).split(sep).join("/")}`);
    else evidence.outcomeClasses = `${relative(repoRoot, diffSource.path).split(sep).join("/")} — ${cite.join(", ")}`;
  }

  // 🎯️SECOND EVIDENCE SOURCE, used when the leaf has no 🔺️diff to read: the committed mutation vectors.
  // Every scenario bundle records what the implementation ACTUALLY produced in its `🎯️outcome`, so the
  // union across a leaf's scenarios is a lower bound on the classes it reaches — observed behaviour
  // rather than inferred behaviour. It is a LOWER bound and the evidence string says so, because a
  // class no committed scenario exercises is invisible here.
  if (outcomes.size === 0) {
    const scenarios = join(leafAbs, taxonomy.testsDirName);
    if (existsSync(scenarios)) {
      const observed: string[] = [];
      for (const scenario of readdirSync(scenarios, { withFileTypes: true }).filter((entry) => entry.isDirectory() && !entry.isSymbolicLink())) {
        const outcome = readFirst([join(scenarios, scenario.name, "🎯️outcome", "🔣️.json"), join(scenarios, scenario.name, "🎯️outcome", "🔣️.json")]);
        if (outcome === null) continue;
        try {
          const status = String((JSON.parse(outcome.text) as { status?: unknown }).status ?? "");
          if (status.length === 0) continue;
          if (isMutationOutcomeClass(status)) {
            outcomes.add(status);
            observed.push(`${scenario.name}=${status}`);
          }
        } catch {
          /* 🧭️A malformed vector is the vector audit's finding, not this one's. */
        }
      }
      if (observed.length > 0) {
        const index = refused.findIndex((why) => why.startsWith("outcomeClasses"));
        if (index >= 0) refused.splice(index, 1);
        evidence.outcomeClasses = `LOWER BOUND from ${observed.length} committed vector(s): ${observed.slice(0, 4).join(", ")} — the leaf has no 🔺️diff, so this records observed outcomes only and a class no scenario exercises is not represented`;
      }
    }
  }

  // 🔢️The binary tag is declared once, in the owner's binary protocol, and must stay that number.
  const binaryProtocol = readFirst([join(vocabulary, "💾️binary", "📡️.protocol.semio"), join(vocabulary, "💾️binary", "📡️.protocol.semio")]);
  let binaryTag: number | null = null;
  if (binaryProtocol !== null) {
    const lines = binaryProtocol.text.split(/\r?\n/);
    const line = lines.findIndex((candidate) => new RegExp(`record\\s+${kind}\\s+tag=(\\d+)`).test(candidate));
    if (line >= 0) {
      binaryTag = Number(lines[line]!.match(/tag=(\d+)/)![1]);
      evidence.binaryTag = `${relative(repoRoot, binaryProtocol.path).split(sep).join("/")}:${line + 1}`;
    } else {
      // 🔢️A binary surface that does not carry this kind is STALE, and inventing a tag would put a
      // wire number into production registration that nothing on the wire agrees with. The CAD binary
      // protocol is exactly this case: it still lists the fourteen verbs retired in ticket
      // 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 3 and none of the twenty that replaced them.
      // 🔢️A STALE wire protocol is real drift and is reported as such — every one of the hundred owners
      // that has a binary protocol is missing records, 1 361 kinds in total. But it is NOT a reason to
      // block this mutation from being testable: the descriptor contract permits a null `binaryTag`,
      // the two concerns are orthogonal, and conflating them would hold subset-scoped external-oracle
      // coverage hostage to a wire format nothing in the test path reads. The drift is surfaced by
      // `binaryProtocolDriftBreaches` instead, where it can be acted on without blocking anything.
      const declared = lines.filter((candidate) => /^record\s/.test(candidate)).length;
      evidence.binaryTag = `null — ${relative(repoRoot, binaryProtocol.path).split(sep).join("/")} declares ${declared} record(s) and none is ${JSON.stringify(kind)}; the wire protocol is stale for this vocabulary`;
    }
  }

  // 🔤️The text opcode is the DSL keyword the payload declares, not an assumption about its name.
  let textOpcode: string | null = null;
  if (mutationSource !== null) {
    const keyword = mutationSource.text.match(/#\[dsl\(keyword\s*=\s*"([a-z0-9-]+)"\)\]/)?.[1];
    if (keyword !== undefined) {
      textOpcode = keyword;
      evidence.textOpcode = `#[dsl(keyword = "${keyword}")] in ${relative(repoRoot, mutationSource.path).split(sep).join("/")}`;
      if (keyword !== kind) refused.push(`textOpcode: the DSL keyword ${JSON.stringify(keyword)} disagrees with the directory kind ${JSON.stringify(kind)}`);
    }
  }

  const hasInverse = existsSync(join(leafAbs, "↩️inverse"));
  const hasPlan = existsSync(join(leafAbs, "🧩️plan"));
  const hasText = existsSync(join(vocabulary, "📝️text"));
  const hasBinary = binaryProtocol !== null;
  // 🧬️ONE payload-schema convention, and it is the taxonomy's: `mutationPayloadSchemaLocation` puts a
  // leaf's payload contract at `🧬️schema/<json>` beside the leaf, and the descriptor points at exactly
  // that path. A second accepted spelling made "which file is the contract" depend on which one
  // happened to exist, which is the same as having no owner at all.
  const canonicalPayloadSchema = payloadSchemaRelativePath(repoRoot);
  const hasSchema = existsSync(join(leafAbs, canonicalPayloadSchema));
  const payloadSchema = hasSchema ? canonicalPayloadSchema : "";
  if (hasSchema) evidence.payloadSchema = `${payloadSchema} exists beside the leaf`;
  evidence.invertibility = hasPlan ? "🧩️plan facet present" : hasInverse ? "↩️inverse facet present" : "neither ↩️inverse nor 🧩️plan is present";
  evidence.diffParticipation = diffSource === null ? "no 🔺️diff facet" : "🔺️diff facet present";
  evidence.requiredLanguageSurfaces = `facets present: rust${hasSchema ? `, ${payloadSchema}` : ""}${hasText ? ", 📝️text" : ""}${hasBinary ? ", 💾️binary" : ""}`;

  // 🧬️The payload schema is a taxonomy LOCATION, not a guess. A leaf without one has no payload
  // contract to point at, and that absence is reported rather than filled in from somewhere else.
  if (!hasSchema) refused.push(`payloadSchema: the leaf carries no ${canonicalPayloadSchema}, so there is no payload contract to reference`);
  if (refused.length > 0) return { leaf: leafRel, kind, descriptor: null, evidence, refused };

  const surfaces = ["rust", ...(hasSchema ? ["json-schema"] : []), ...(hasText ? ["text"] : []), ...(hasBinary ? ["binary"] : [])];
  return {
    leaf: leafRel,
    kind,
    evidence,
    refused,
    descriptor: {
      schemaVersion: 1,
      owner: leafRel,
      semanticKind: kind,
      displayName: titleCase(kind),
      emoji,
      aggregateVariant,
      payloadSchema,
      textOpcode,
      binaryTag,
      invertibility: hasPlan ? "plan" : hasInverse ? "explicit-mutation" : "non-invertible",
      diffParticipation: diffSource === null ? "none" : "detect",
      outcomeClasses: [...outcomes],
      composition: "atomic",
      requiredLanguageSurfaces: surfaces,
    },
  };
}

/**
 * 🔢️ Where the binary wire protocol has fallen behind the mutation vocabulary it is supposed to carry.
 *
 * Reported separately from descriptor derivation on purpose: a stale wire format is a real defect and a
 * different one from "this mutation has no test", and blocking the second on the first would hold every
 * mutation hostage to a file nothing in the test path reads.
 */
export function binaryProtocolDriftBreaches(repoRoot: string, registry: OracleRegistry): BreachRecord[] {
  const taxonomy = testTaxonomy(repoRoot);
  const breaches: BreachRecord[] = [];
  for (const owner of [...new Set(registry.contributions.map((entry) => entry.owner))]) {
    const vocabulary = join(repoRoot, owner, "🧬️schema", taxonomy.testMutationVocabularyDirName);
    if (!existsSync(vocabulary)) continue;
    const descriptors = readLeafDescriptors(repoRoot, owner);
    const kinds = mutationLeafDirectories(repoRoot, owner)
      .map((leaf) => descriptors.get(leaf)?.semanticKind ?? leaf.split("/").at(-1)!.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)?.[0])
      .filter((kind): kind is string => kind !== undefined);
    if (kinds.length === 0) continue;
    const protocol = readFirst([join(vocabulary, "💾️binary", "📡️.protocol.semio")]);
    if (protocol === null) continue;
    const records = [...protocol.text.matchAll(/^\s*record\s+([a-z][a-z0-9-]*)\s+tag=(\d+)/gm)].map((match) => ({ kind: match[1]!, tag: Number(match[2]) }));
    const declared = new Set(records.map((record) => record.kind));
    const missing = kinds.filter((kind) => !declared.has(kind));
    const orphaned = [...declared].filter((kind) => !kinds.includes(kind));
    const reused = records.filter((record, index) => records.findIndex((other) => other.tag === record.tag) !== index || records.findIndex((other) => other.kind === record.kind) !== index);
    if (reused.length > 0) {
      breaches.push(
        breach(
          "testing/contract",
          "binary-protocol-tag-reuse",
          relative(repoRoot, protocol.path).split(sep).join("/"),
          `${reused.length} record(s) reuse a tag or a kind: ${reused.slice(0, 6).map((record) => `${record.kind}=${record.tag}`).join(", ")}`,
          "Every codec derives its op tags from these records, so a reused tag decodes two kinds as one and a reused kind makes the tag ambiguous.",
          "Give every kind exactly one record with a tag no other record uses.",
          "high",
        ),
      );
    }
    if (missing.length === 0 && orphaned.length === 0) continue;
    breaches.push(
      breach(
        "testing/contract",
        "binary-protocol-drift",
        relative(repoRoot, protocol.path).split(sep).join("/"),
        `${missing.length} mutation kind(s) have no wire record and ${orphaned.length} record(s) name a kind that no longer exists${orphaned.length > 0 ? `: ${orphaned.slice(0, 6).join(", ")}` : ""}`,
        "A wire protocol that has fallen behind its vocabulary cannot carry the mutations the implementation actually dispatches, and its orphaned records describe verbs nothing can emit.",
        "Regenerate the protocol from the current mutation leaves, assigning a tag to each new kind and removing the records whose kind is gone.",
        "medium",
      ),
    );
  }
  return breaches;
}

/** 🏗️ Scaffolds every leaf of one owner, so roster-level uniqueness can be checked before anything is written. */
export function scaffoldOwnerDescriptors(repoRoot: string, ownerRel: string): LeafScaffold[] {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  if (!existsSync(vocabulary)) return [];
  return mutationLeafDirectories(repoRoot, ownerRel)
    .map((leaf) => scaffoldLeafDescriptor(repoRoot, ownerRel, leaf))
    .sort((a, b) => a.kind.localeCompare(b.kind));
}
//#endregion 🏗️LeafScaffold

//#region 🧬️PayloadSchema
/** 🧬️ One mutation leaf's payload contract as the leaf DECLARES it, or the reason there is none. */
export type PayloadSchemaResolution = Readonly<{ leaf: string; kind: string; declared: string; path: string | null; schema: Record<string, unknown> | null; refused: readonly string[] }>;

/** 🧬️ The one place a mutation leaf's payload schema may live, straight out of the taxonomy. */
export function payloadSchemaRelativePath(repoRoot: string): string {
  const taxonomy = testTaxonomy(repoRoot);
  const location = rawTaxonomy(repoRoot).mutationPayloadSchemaLocation as { directoryName?: unknown; fileKindId?: unknown } | undefined;
  if (typeof location?.directoryName !== "string" || typeof location.fileKindId !== "string") throw new Error(`${TAXONOMY_REL_PATH} declares no mutationPayloadSchemaLocation`);
  return `${location.directoryName}/${testFilenameForKind(taxonomy, location.fileKindId)}`;
}

/**
 * 🧬️ Resolves one mutation leaf's payload schema THROUGH ITS DESCRIPTOR.
 *
 * The leaf is the declared authority for its payload (`mutationPayloadSchemaAuthority`), so the
 * descriptor's `payloadSchema` field is the whole resolution: one declared owner-relative path, read
 * or refused. Nothing is searched for, nothing is inferred from a same-named Rust type somewhere else
 * in the tree, and there is no second accepted convention — a payload contract that two files could
 * satisfy is a payload contract nobody owns.
 */
export function resolvePayloadSchema(repoRoot: string, ownerRel: string, leafDirName: string): PayloadSchemaResolution {
  const taxonomy = testTaxonomy(repoRoot);
  const canonical = payloadSchemaRelativePath(repoRoot);
  const descriptorFile = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  const leafRel = `${ownerRel}/${schemaModuleDirName(repoRoot)}/${taxonomy.testMutationVocabularyDirName}/${leafDirName}`;
  const descriptor = readJson(join(repoRoot, leafRel, descriptorFile)) as MutationLeafDescriptor | undefined;
  // 🧬️The DESCRIPTOR names the kind; the directory only hints at it. A two-segment leaf
  // (`🌳️node/⚖️change-weights` → `change-node-morph-weights`) has no kind derivable from its path at all,
  // so reading the declaration first is what makes such a leaf reportable under its own name.
  const kind = typeof descriptor?.semanticKind === "string" && descriptor.semanticKind.length > 0 ? descriptor.semanticKind : leafDirName.match(/[a-z][a-z0-9]*(?:-[a-z0-9]+)+$/)?.[0] ?? "";
  if (descriptor === undefined) return { leaf: leafRel, kind, declared: "", path: null, schema: null, refused: [`the leaf carries no ${descriptorFile} descriptor, so nothing declares its payload contract`] };
  const declared = typeof descriptor.payloadSchema === "string" ? descriptor.payloadSchema : "";
  if (declared.length === 0) return { leaf: leafRel, kind, declared, path: null, schema: null, refused: ["the descriptor declares no `payloadSchema`"] };
  if (declared !== canonical) return { leaf: leafRel, kind, declared, path: null, schema: null, refused: [`the descriptor declares ${JSON.stringify(declared)}; a leaf payload schema lives at ${canonical}`] };
  const rel = `${leafRel}/${declared}`;
  if (!existsSync(join(repoRoot, rel))) return { leaf: leafRel, kind, declared, path: rel, schema: null, refused: [`the declared payload schema ${rel} does not exist`] };
  const document = readJson(join(repoRoot, rel));
  if (document === undefined) return { leaf: leafRel, kind, declared, path: rel, schema: null, refused: [`${rel} is not readable JSON`] };
  if (!isDeclaredDialectDefinition(repoRoot, document)) return { leaf: leafRel, kind, declared, path: rel, schema: null, refused: [`${rel} does not declare ${schemaJsonDialect(repoRoot)}`] };
  return { leaf: leafRel, kind, declared, path: rel, schema: document as Record<string, unknown>, refused: [] };
}

/** 🧬️ Resolves every leaf's declared payload schema for one owner. */
export function resolvePayloadSchemas(repoRoot: string, ownerRel: string): PayloadSchemaResolution[] {
  const taxonomy = testTaxonomy(repoRoot);
  const vocabulary = join(repoRoot, ownerRel, schemaModuleDirName(repoRoot), taxonomy.testMutationVocabularyDirName);
  if (!existsSync(vocabulary)) return [];
  return mutationLeafDirectories(repoRoot, ownerRel)
    .map((leaf) => resolvePayloadSchema(repoRoot, ownerRel, leaf))
    .sort((a, b) => a.kind.localeCompare(b.kind));
}
//#endregion 🧬️PayloadSchema
