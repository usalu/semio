import { type LoadedTaxonomy } from "../🔣️taxonomy/📚️loaded/🟦️.ts";
import { normalizeRelative, sourceRelative } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull, LEXICAL_OPAQUE_ROOTS } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { parseCurrentSourceApprovals } from "../../🔍️discovery/✅️owned-file-approvals/🟦️.ts";
import { validateFrozenCoordinateEvidenceContracts, type FrozenCoordinateEvidenceContract, validateFrozenMarkdownCoordinateEvidenceContracts, type FrozenMarkdownCoordinateEvidenceContract } from "../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { relative } from "node:path";
import { requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import type { ReferenceToken } from "../🔤️reference-tokenizers/🟦️.ts";
import { type JsonStringCoordinate, jsonStringCoordinates } from "../➰️symlink-targets/🟦️.ts";
import { taxonomyFixturePayloadPath } from "../🧱️inventory-leaf-stamps/🟦️.ts";
import { exactOwnedFileCatalog, nestedCargoCatalog } from "../🗺️artifact-projection-catalogs/🟦️.ts";
import { parseTaxonomyPlan } from "../📃️plan-document/🟦️.ts";

/** 🧷️ An exact JSON coordinate span, optionally derived from one declared fixed identity prefix. */
export interface FrozenEvidenceCoordinate extends JsonStringCoordinate { readonly kind: "source" | "destination" }

/** 🔒️ Resolves only explicitly supplied evidence contracts; it performs no filesystem access. */
export function frozenCoordinateEvidenceCoordinates(path: string, bytes: Uint8Array, contracts: Readonly<Record<string, FrozenCoordinateEvidenceContract>>): readonly FrozenEvidenceCoordinate[] | null {
  const fail = (message: string): never => { throw new Error(`frozen-coordinate-evidence-invalid: ${path}: ${message}`); };
  const problems = validateFrozenCoordinateEvidenceContracts(contracts);
  if (problems.length) fail(problems.join("; "));
  const contract = Object.values(contracts).find((row) => row.path === path);
  if (!contract) return null;
  if (sha256(bytes) !== contract.sha256) fail("document digest does not match registered bytes");
  const content = Buffer.from(bytes).toString("utf8");
  if (!Buffer.from(content, "utf8").equals(bytes)) fail("document is not exact UTF-8");
  let document: unknown;
  try { document = JSON.parse(content); } catch { fail("document is not valid JSON"); }
  if (document === null || typeof document !== "object" || Array.isArray(document) !== (contract.rootKind === "array")) fail("document differs from its exact JSON root authority");
  if (contract.schemaVersion === null ? Object.hasOwn(document!, "schemaVersion") : !Object.hasOwn(document!, "schemaVersion") || (document as Record<string, unknown>).schemaVersion !== contract.schemaVersion) fail("document schemaVersion differs from registered presence/value authority");
  const spans = new Map(jsonStringCoordinates(content, true).map((row) => [row.pointer, row])), rows: FrozenEvidenceCoordinate[] = [], seen = new Set<string>();
  for (const declaration of contract.coordinates) {
    const segments = declaration.pointer.slice(1).split("/").map((segment) => segment.replaceAll("~1", "/").replaceAll("~0", "~"));
    const before = rows.length;
    const visit = (value: unknown, index: number, pointer: string): void => {
      if (index === segments.length) {
        if (typeof value !== "string" || !value || /[\u0000-\u001f]/u.test(value)) fail(`coordinate ${pointer} must be a physical repository-relative path string`);
        const ownerIdentity = "representation" in declaration && declaration.representation === "recorded-package-owner-identity";
        const escapedSource = "representation" in declaration && declaration.representation === "json-escaped-source-path";
        const prefix = "representation" in declaration ? declaration.representation === "recorded-package-owner-identity" ? declaration.identityPrefix : declaration.representation === "recorded-repository-absolute" ? declaration.recordedRepositoryRoot + "/" : "" : "";
        if (prefix && !(value as string).startsWith(prefix)) fail(`coordinate ${pointer} does not match its exact recorded prefix`);
        const relativeValue = prefix ? (value as string).slice(prefix.length) : value as string;
        if (!relativeValue || /^[A-Za-z]:/u.test(relativeValue) || ownerIdentity && relativeValue.includes(":")) fail(`coordinate ${pointer} must have a nonempty repository-relative suffix`);
        if (escapedSource && (Buffer.from(relativeValue, "utf8").toString("utf8") !== relativeValue || /[\\:*?"<>|\u007f]/u.test(relativeValue) || LEXICAL_OPAQUE_ROOTS.some((root) => relativeValue === root || relativeValue.startsWith(root + "/")))) fail(`coordinate ${pointer} must decode once to an exact non-opaque physical path`);
        try { if (normalizeRelative(relativeValue) !== relativeValue) fail(`coordinate ${pointer} is not repository-relative`); } catch { fail(`coordinate ${pointer} is not repository-relative`); }
        const span = spans.get(pointer);
        if (!span || span.value !== value || escapedSource !== (span.rawValue !== undefined)) fail(`coordinate ${pointer} has no exact declared JSON value encoding`);
        if (seen.has(pointer)) fail(`coordinate selectors overlap at ${pointer}`);
        seen.add(pointer);
        rows.push({ pointer, start: span!.start + (ownerIdentity ? prefix.length : 0), end: span!.end, value: escapedSource ? span!.rawValue! : ownerIdentity ? relativeValue : span!.value, kind: declaration.kind });
        return;
      }
      const segment = requireTaxonomyValue(segments[index],"segments[index]");
      if (segment === "*") {
        if (!Array.isArray(value)) fail(`wildcard ${declaration.pointer} must select an array`);
        (value as unknown[]).forEach((child, childIndex) => visit(child, index + 1, `${pointer}/${childIndex}`));
      } else {
        if (value === null || typeof value !== "object" || !Object.hasOwn(value, segment) || Array.isArray(value) && !/^(?:0|[1-9][0-9]*)$/u.test(segment)) fail(`coordinate pointer ${declaration.pointer} is missing`);
        visit(requireTaxonomyValue((value as Record<string, unknown>)[segment],"(value as Record<string, unknown>)[segment]"), index + 1, `${pointer}/${segment.replaceAll("~", "~0").replaceAll("/", "~1")}`);
      }
    };
    visit(document, 0, "");
    if (rows.length === before) fail(`coordinate pointer ${declaration.pointer} matched no value`);
  }
  return rows.sort((left, right) => left.start - right.start);
}

/** 📝️ Finds only plain single-backtick or path-only-list coordinates outside opaque Markdown blocks (fenced/indented code, blockquotes, HTML). ATX headings get ordinary inline backtick-span recognition too (CommonMark parses their inline content the same as any other leaf block; only block-level constructs are opaque here). */
export function markdownSourceCoordinateSpans(content: string): readonly Readonly<{ start: number; end: number; form: "inline-code" | "path-list-item" }>[] {
  const rows: { start: number; end: number; form: "inline-code" | "path-list-item" }[] = [];
  let fence = "", html = "", inline = 0;
  for (const match of content.matchAll(/[^\r\n]*(?:\r\n|\r|\n|$)/gu)) {
    const line = match[0].replace(/(?:\r\n|\r|\n)$/u, ""), offset = match.index!;
    if (!line.trim()) { inline = 0; if (html === "block") html = ""; continue; }
    const prefix = line.replace(/^ {0,3}(?:[-+*][ \t]+|\d+[.)][ \t]+)/u, ""), marker = prefix.match(/^ {0,3}(`{3,}|~{3,})(.*)$/u);
    if (fence) { if (marker && requireTaxonomyValue(marker[1],"marker[1]")[0] === fence[0] && requireTaxonomyValue(marker[1],"marker[1]").length >= fence.length && !requireTaxonomyValue(marker[2],"marker[2]").trim()) fence = ""; continue; }
    if (marker && !(requireTaxonomyValue(marker[1],"marker[1]")[0] === "`" && requireTaxonomyValue(marker[2],"marker[2]").includes("`"))) { fence = requireTaxonomyValue(marker[1],"fence marker"); inline = 0; continue; }
    if (html) { if (html === "comment" && line.includes("-->")) html = ""; continue; }
    if (line.includes("<!--")) { if (!line.slice(line.indexOf("<!--") + 4).includes("-->")) html = "comment"; inline = 0; continue; }
    if (/^ {0,3}</u.test(line)) { html = "block"; inline = 0; continue; }
    if (/^(?: {4}| *\t| {0,3}>)/u.test(line)) { inline = 0; continue; }
    const first = rows.length, blanks: (readonly [number, number])[] = [];
    const list = line.match(/^ {0,3}[-+*][ \t]+([^\s]+)[ \t]*$/u);
    if (list && !inline) { const start = offset + line.indexOf(requireTaxonomyValue(list[1],"list[1]")); rows.push({ start, end: start + requireTaxonomyValue(list[1],"list[1]").length, form: "path-list-item" }); }
    const runs = [...line.matchAll(/`+/gu)], nextSameLength = new Array<number>(runs.length), lastByLength = new Map<number, number>();
    for (let index = runs.length - 1; index >= 0; index--) {
      nextSameLength[index] = lastByLength.get(runs[index]![0].length) ?? -1;
      lastByLength.set(runs[index]![0].length, index);
    }
    for (let index = 0; index < runs.length; index++) {
      const run = runs[index]!, start = run.index!;
      if (inline) { if (run[0].length === inline) { inline = 0; blanks.push([0, start + run[0].length]); } continue; }
      let escapes = 0;
      while (start - escapes > 0 && line[start - escapes - 1] === "\\") escapes++;
      if (escapes % 2) continue;
      const close = nextSameLength[index]!;
      if (close < 0) { inline = run[0].length; continue; }
      const end = runs[close]!.index! + runs[close]![0].length;
      blanks.push([start, end]);
      if (run[0].length === 1) rows.push({ start: offset + start + 1, end: offset + runs[close]!.index!, form: "inline-code" });
      index = close;
    }
    let cursor = 0, delimited = false;
    for (const [from, to] of blanks.sort((left, right) => left[0] - right[0])) {
      if (from > cursor && /[\[\]<>]/u.test(line.slice(cursor, from))) { delimited = true; break; }
      cursor = Math.max(cursor, to);
    }
    if (delimited || /[\[\]<>]/u.test(line.slice(cursor))) rows.splice(first);
  }
  return rows;
}

/** 🔏️ Resolves declared historical Markdown source spans without filesystem access or prose rewriting. */
export function frozenMarkdownCoordinateEvidenceCoordinates(path: string, bytes: Uint8Array, contracts: Readonly<Record<string, FrozenMarkdownCoordinateEvidenceContract>>): readonly FrozenEvidenceCoordinate[] | null {
  const fail = (message: string): never => { throw new Error(`frozen-coordinate-evidence-invalid: ${path}: ${message}`); };
  const problems = validateFrozenMarkdownCoordinateEvidenceContracts(contracts);
  if (problems.length) fail(problems.join("; "));
  const contract = Object.values(contracts).find((row) => row.path === path);
  if (!contract) return null;
  if (sha256(bytes) !== contract.sha256) fail("document digest does not match registered bytes");
  const content = Buffer.from(bytes).toString("utf8");
  if (!Buffer.from(content, "utf8").equals(bytes)) fail("document is not exact UTF-8");
  const spans = new Set(markdownSourceCoordinateSpans(content).map((row) => `${row.form}\0${row.start}\0${row.end}`));
  return contract.coordinates.map((coordinate): FrozenEvidenceCoordinate => {
    const value = content.slice(coordinate.start, coordinate.end);
    if (!spans.has(`${coordinate.form}\0${coordinate.start}\0${coordinate.end}`)) fail("coordinate has no exact admitted Markdown source span");
    if (!value || /[\\:*?"<>|`\u0000-\u0020]/u.test(value) || /^(?:compose|temp\/compose)(?:\/|$)/u.test(value) || value.split("/").some((part) => !part || part === "." || part === "..")) fail("coordinate is not one non-opaque repository-relative source path");
    if (sha256(value) !== coordinate.valueSha256) fail("coordinate value digest differs from its exact source authority");
    return { pointer: `markdown:${coordinate.form}@${coordinate.start}`, start: coordinate.start, end: coordinate.end, value, kind: "source" };
  }).sort((left, right) => left.start - right.start);
}

const frozenEvidenceContractIndexes = new WeakMap<LoadedTaxonomy, ReadonlyMap<string, Readonly<{ path: string }>>>();

interface FrozenEvidenceRecord { readonly path: string; readonly taxonomy: LoadedTaxonomy; readonly coordinates: ReadonlySet<string>; readonly problem: string | null }

const frozenEvidenceCoordinateCache = new WeakMap<Uint8Array, FrozenEvidenceRecord>();

export function frozenEvidenceContractIndex(taxonomy: LoadedTaxonomy): ReadonlyMap<string, Readonly<{ path: string }>> {
  const cached = frozenEvidenceContractIndexes.get(taxonomy);
  if (cached) return cached;
  const index = new Map([...Object.values(taxonomy.discoverySchema.frozenCoordinateEvidenceContracts), ...Object.values(taxonomy.discoverySchema.frozenMarkdownCoordinateEvidenceContracts)].filter(contract => !taxonomyFixturePayloadPath(contract.path, taxonomy.discoverySchema)).map((contract) => [contract.path, contract]));
  frozenEvidenceContractIndexes.set(taxonomy, index);
  return index;
}

export function validateObservedFrozenEvidenceNodes(repoRoot: string, knownPaths: ReadonlySet<string>, taxonomy: LoadedTaxonomy): void {
  for (const path of frozenEvidenceContractIndex(taxonomy).keys()) {
    if (!knownPaths.has(path)) continue;
    const absolute = assertLexicalInputOutsideOpaque(repoRoot, path, "Frozen coordinate evidence"), stat = lstatOrNull(absolute);
    if (!stat?.isFile()) throw new Error(`frozen-coordinate-evidence-invalid: ${path}: observed document must remain a no-follow regular file`);
  }
}

/** 🧊️ The declared coordinates of a registered frozen evidence document, or its digest/shape problem with no coordinates at all, so a
 * planner reports an invalid document instead of aborting while every rewrite of it still fails closed; `null` when none is registered. */
export function frozenEvidenceRecord(path: string, bytes: Uint8Array, taxonomy: LoadedTaxonomy): FrozenEvidenceRecord | null {
  if (!frozenEvidenceContractIndex(taxonomy).has(path)) return null;
  const cached = frozenEvidenceCoordinateCache.get(bytes);
  if (cached?.path === path && cached.taxonomy === taxonomy) return cached;
  let record: FrozenEvidenceRecord;
  try {
    const rows = (frozenCoordinateEvidenceCoordinates(path, bytes, taxonomy.discoverySchema.frozenCoordinateEvidenceContracts) ?? frozenMarkdownCoordinateEvidenceCoordinates(path, bytes, taxonomy.discoverySchema.frozenMarkdownCoordinateEvidenceContracts))!;
    record = { path, taxonomy, coordinates: new Set(rows.map((row) => `${row.start}\0${row.end}\0${row.value}`)), problem: null };
  } catch (error) {
    if (!(error instanceof Error) || !error.message.startsWith(`frozen-coordinate-evidence-invalid: ${path}: `)) throw error;
    record = { path, taxonomy, coordinates: new Set(), problem: error.message };
  }
  frozenEvidenceCoordinateCache.set(bytes, record);
  return record;
}

/** 🔐️ Fail-closed form of {@link frozenEvidenceRecord} for transaction preflight: an invalid registered document aborts. */
export function frozenEvidenceCoordinateAuthority(path: string, bytes: Uint8Array, taxonomy: LoadedTaxonomy): ReadonlySet<string> | null {
  const record = frozenEvidenceRecord(path, bytes, taxonomy);
  if (record?.problem) throw new Error(record.problem);
  return record?.coordinates ?? null;
}

interface FrozenPlanCoordinateAuthority { readonly planLike: boolean; readonly coordinates: ReadonlySet<string> }

const frozenPlanCoordinateCache = new WeakMap<Uint8Array, FrozenPlanCoordinateAuthority>();

export function frozenPlanCoordinateAuthority(path: string, bytes: Uint8Array): FrozenPlanCoordinateAuthority {
  if (!path.endsWith(".json")) return { planLike: false, coordinates: new Set() };
  const cached = frozenPlanCoordinateCache.get(bytes);
  if (cached) return cached;
  const content = Buffer.from(bytes).toString("utf8"), coordinates = new Set<string>();
  let planLike = false;
  try {
    const value = JSON.parse(content);
    planLike = value !== null && typeof value === "object" && value.schemaVersion === 2 && value.taxonomySchemaVersion === 7 && "planDigest" in value;
    if (planLike) {
      const parsed = parseTaxonomyPlan(value);
      if (content !== canonicalJson(parsed) + "\n") throw new Error("Retained plan bytes are not canonical");
      const typedPath = /^(?:\/scope|\/excludedTreeDigests\/\d+\/relativeRoot|\/destinationAncestorPreimages\/\d+\/path|\/moves\/\d+\/(?:sourcePath|destinationPath|ownerId|sourcePreimage\/target|referenceEdits\/\d+\/(?:path|oldValue|newValue))|\/edits\/\d+\/(?:path|oldValue|newValue)|\/embeddedTicketRoots\/\d+\/(?:sourceMetadataRoot|sourceTicketRoot|canonicalTicketRoot)|\/embeddedTicketRootRelocations\/\d+\/(?:sourcePath|destinationPath|relativeEvidencePath|ownerId|preimage\/target)|\/symlinkTargetEdits\/\d+\/(?:sourcePath|finalPath|oldTarget|newTarget|logicalTargetSourcePath|logicalTargetFinalPath|ownerId|logicalTargetPreimage\/target)|\/evidenceRemovals\/\d+\/(?:sourcePath|ownerId|preimage\/target|authority\/(?:catalogPath|destinationPath|retainedFinalPath|sourcePath|ownerPath|manifestPath|serializedInputPath|members\/\d+\/(?:sourcePath|finalPath|preimage\/target)))|\/regenerations\/\d+\/(?:cwd|outputRoots\/\d+|(?:inputs|preOutputs|outputs)\/\d+\/(?:path|target)|preview\/(?:nodes\/\d+\/path|staleRemovals\/\d+)|staleRemovals\/\d+)|\/unresolved\/\d+\/path)$/u;
      for (const row of jsonStringCoordinates(content)) if (typedPath.test(row.pointer)) coordinates.add(`${row.start}\0${row.end}\0${row.value}`);
    }
  } catch {}
  const result = { planLike, coordinates };
  frozenPlanCoordinateCache.set(bytes, result);
  return result;
}

const frozenCoordinateCache = new WeakMap<Uint8Array, { readonly path: string; readonly taxonomy: LoadedTaxonomy; readonly coordinates: ReadonlySet<string>; readonly targets: ReadonlySet<string> }>();

export function isFrozenSourceCoordinateToken(path: string, bytes: Uint8Array, token: ReferenceToken, target: string, taxonomy: LoadedTaxonomy, repoRoot: string): boolean {
  if (frozenEvidenceRecord(path, bytes, taxonomy)?.coordinates.has(`${token.start}\0${token.end}\0${token.value}`)) return true;
  if (frozenPlanCoordinateAuthority(path, bytes).coordinates.has(`${token.start}\0${token.end}\0${token.value}`)) return true;
  const owner = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  const taxonomyPath = sourceRelative(relative(repoRoot, taxonomy.path));
  const approvalDeclaration = owner?.contractKind === "exact-owner-path-catalog" ? Object.values(owner.currentSourceRevisions ?? {}).find(row => row.approvalPath === path) : undefined;
  if (path !== taxonomyPath && path !== requireTaxonomyValue(taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"],"nested Cargo contract").authorityCatalogPath && !(owner?.contractKind === "exact-owner-path-catalog" && path === owner.authorityCatalogPath) && !approvalDeclaration) return false;
  let cached = frozenCoordinateCache.get(bytes);
  if (!cached || cached.path !== path || cached.taxonomy !== taxonomy) {
    const content = Buffer.from(bytes).toString("utf8"), allowed = new Map<string, string>(), targets = new Set<string>();
    const ownerContract = taxonomy.schema.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
    const packageContract = requireTaxonomyValue(taxonomy.schema.semanticPackageProjectionContracts["nested-cargo-packages-v1"],"nested Cargo contract");
    const pointer = (value: string): string => value.replaceAll("~", "~0").replaceAll("/", "~1");
    const add = (location: string, value: string): void => { allowed.set(location, value); targets.add(value); };
    if (ownerContract?.contractKind === "exact-owner-path-catalog" && path === ownerContract.authorityCatalogPath && sha256(bytes) === ownerContract.authorityCatalogSha256) {
      const catalog = exactOwnedFileCatalog(repoRoot, taxonomy)!;
      catalog.entries.forEach((row, index) => add(`/entries/${index}/sourcePath`, row.sourcePath));
      for (const [id, row] of Object.entries(catalog.ownerEvidence)) row.evidencePaths.forEach((value, index) => add(`/ownerEvidence/${pointer(id)}/evidencePaths/${index}`, value));
      for (const [id, row] of Object.entries(catalog.referenceOwners)) add(`/referenceOwners/${pointer(id)}/ownerPath`, row.ownerPath);
      for (const [id, row] of Object.entries(catalog.generatorOwners)) { add(`/generatorOwners/${pointer(id)}/ownerPath`, row.ownerPath); add(`/generatorOwners/${pointer(id)}/currentOutputPath`, row.currentOutputPath); }
    }
    if (approvalDeclaration && ownerContract?.contractKind === "exact-owner-path-catalog" && sha256(bytes) === approvalDeclaration.approvalSha256) {
      const catalog = exactOwnedFileCatalog(repoRoot, taxonomy), row = catalog?.entries.find(entry => entry.sourcePath === approvalDeclaration.sourcePath), approval = JSON.parse(content);
      if (row && catalog?.ownerEvidence[row.ownerEvidenceId]?.kind === "ordinary-owner-doc" && row.disposition === "owner-documentation-relocate" && canonicalJson(row.preimage) === canonicalJson(approvalDeclaration.baselinePreimage) && approval.schemaVersion === 1 && approval.kind === "current-source-approval" && approval.sourcePath === row.sourcePath && approval.policy?.path === ownerContract.authorityCatalogPath && approval.policy?.sha256 === ownerContract.authorityCatalogSha256 && approval.baseline?.commit === approvalDeclaration.baselineCommit && approval.baseline?.blob === approvalDeclaration.baselineBlob && canonicalJson(approval.baseline?.preimage) === canonicalJson(approvalDeclaration.baselinePreimage) && canonicalJson(approval.currentPreimage) === canonicalJson(approvalDeclaration.currentPreimage)) add("/sourcePath", row.sourcePath);
    }
    if (path === packageContract.authorityCatalogPath && sha256(bytes) === packageContract.authorityCatalogSha256) {
      const catalog = nestedCargoCatalog(repoRoot, taxonomy.discoverySchema);
      catalog?.packages.forEach((row, index) => { add(`/packages/${index}/sourceRoot`, row.sourceRoot); row.mappings.forEach((mapping, mappingIndex) => add(`/packages/${index}/mappings/${mappingIndex}/sourcePath`, mapping.sourcePath)); row.sourceSplices.forEach((splice, spliceIndex) => add(`/packages/${index}/sourceSplices/${spliceIndex}/sourcePath`, splice.sourcePath)); row.derivedLeaves.forEach((leaf, leafIndex) => add(`/packages/${index}/derivedLeaves/${leafIndex}/originSourcePath`, leaf.originSourcePath)); row.generatedSourceRetirements.forEach((retirement, retirementIndex) => add(`/packages/${index}/generatedSourceRetirements/${retirementIndex}/sourcePath`, retirement.sourcePath)); });
      catalog?.packages.forEach((row, index) => row.authoredSourceFragments.forEach((fragment, fragmentIndex) => add(`/packages/${index}/authoredSourceFragments/${fragmentIndex}/sourcePath`, fragment.sourcePath)));
      catalog?.referenceConsumers.forEach((row, index) => add(`/referenceConsumers/${index}/path`, row.path));
      for (const [id, row] of Object.entries(catalog?.referenceTokenTransforms ?? {})) add(`/referenceTokenTransforms/${pointer(id)}/sourceToken`, row.sourceToken);
    }
    if (path === taxonomyPath) {
      if (ownerContract?.contractKind === "exact-owner-path-catalog") {
        const data = JSON.parse(content).semanticOwnedFileProjectionContracts?.["readme-license-owner-leaves-v1"]?.authoredDocumentCorrections;
        if (canonicalJson(data) === canonicalJson(ownerContract.authoredDocumentCorrections)) {
          const owners = exactOwnedFileCatalog(repoRoot, taxonomy);
          for (const [id, row] of Object.entries(ownerContract.authoredDocumentCorrections)) {
            const owner = owners?.entries.find((entry) => entry.sourcePath === row.sourcePath && entry.destinationPath === row.destinationPath && entry.disposition === "owner-documentation-relocate" && canonicalJson(entry.preimage) === canonicalJson(row.preimage));
            if (owner) add(`/semanticOwnedFileProjectionContracts/readme-license-owner-leaves-v1/authoredDocumentCorrections/${pointer(id)}/sourcePath`, row.sourcePath);
          }
        }
        if (ownerContract.currentSourceRevisions && taxonomy.input?.path === taxonomyPath && taxonomy.input.contentHash === sha256(bytes) && taxonomy.input.size === bytes.byteLength && Buffer.from(taxonomy.input.bytes).equals(bytes)) {
          let revisions: ReturnType<typeof parseCurrentSourceApprovals> | undefined;
          try { revisions = parseCurrentSourceApprovals(JSON.parse(content).semanticOwnedFileProjectionContracts?.["readme-license-owner-leaves-v1"]?.currentSourceRevisions); } catch {}
          if (revisions && canonicalJson(revisions) === canonicalJson(ownerContract.currentSourceRevisions)) {
            const owners = exactOwnedFileCatalog(repoRoot, taxonomy);
            for (const [id, row] of Object.entries(revisions)) {
              const owner = owners?.entries.find(entry => entry.sourcePath === row.sourcePath);
              if (owner?.sourcePath === row.sourcePath && owner.disposition === "owner-documentation-relocate" && owner.fixedContractId === null && owner.generatorOwnerId === null && owners?.ownerEvidence[owner.ownerEvidenceId]?.kind === "ordinary-owner-doc" && canonicalJson(owner.preimage) === canonicalJson(row.baselinePreimage)) add(`/semanticOwnedFileProjectionContracts/readme-license-owner-leaves-v1/currentSourceRevisions/${pointer(id)}/sourcePath`, row.sourcePath);
            }
          }
        }
      }
      const catalog = nestedCargoCatalog(repoRoot, taxonomy.discoverySchema);
      for (const [id, contract] of Object.entries(taxonomy.schema.generatorContracts)) {
        const activation = contract.projectionActivation, row = catalog?.packages.find((row) => row.id === activation?.packageId);
        if (!activation || !row || activation.sourceManifestPath !== `${row.sourceRoot}/Cargo.toml` || activation.destinationManifestPath !== `${row.destinationRoot}/Cargo.toml`) continue;
        const data = JSON.parse(content).generatorContracts?.[id];
        if (canonicalJson(data?.projectionActivation) !== canonicalJson(activation)) continue;
        add(`/generatorContracts/${pointer(id)}/projectionActivation/sourceManifestPath`, activation.sourceManifestPath);
        if (Array.isArray(data.inputPatterns)) data.inputPatterns.forEach((value: unknown, index: number) => { if (typeof value === "string" && contract.inputPatterns.includes(value) && row.mappings.some((mapping) => mapping.sourcePath === value)) add(`/generatorContracts/${pointer(id)}/inputPatterns/${index}`, value); });
        const generation = contract.packageGeneration;
        if (generation && canonicalJson(data.packageGeneration) === canonicalJson(generation)) generation.browserProfile.sourceModulePaths.forEach((value, index) => { if (row.mappings.some((mapping) => mapping.sourcePath === value)) add(`/generatorContracts/${pointer(id)}/packageGeneration/browserProfile/sourceModulePaths/${index}`, value); });
      }
    }
    const coordinates = new Set(jsonStringCoordinates(content).filter((row) => allowed.get(row.pointer) === row.value).map((row) => `${row.start}\0${row.end}\0${row.value}`));
    cached = { path, taxonomy, coordinates, targets };
    frozenCoordinateCache.set(bytes, cached);
  }
  return cached.targets.has(target) && cached.coordinates.has(`${token.start}\0${token.end}\0${token.value}`);
}
