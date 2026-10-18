import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { POLICY_SUBSETS_DIR, type PolicyArtifactDialect, policyListArtifactDialectDirs } from "../../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { subsetDirectoryNameForId, subsetIdForDirectoryName } from "../../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { POLICY_STDIO_ARTIFACTS_REL, policyRustFileHasRealTraitImpl } from "../📋️owner-table/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/** ⚖️Every migrated standard/subset dir carries its required engine/schema/IO children. */
export function policyStandardsCoverageBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const standardChildDirs = ((taxonomy as any).standardChildDirs as string[] | undefined) ?? [];
  const subsetChildDirs = ((taxonomy as any).subsetChildDirs as string[] | undefined) ?? [];
  const slugPattern = new RegExp((taxonomy as any).standardSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    if (!slugPattern.test(dialect.standardSlug)) {
      breaches.push({
        id: `standards-slug-${dialect.standardRel}`,
        summary: `"${dialect.standardRel}" standard slug "${dialect.standardSlug}" does not match ${slugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Standard directory slugs are normalized lowercase identifiers (🔖️2.0, 🔖️ap214, 🔖️1, …).",
        solution: `Rename ${dialect.standardRel} to match the slug pattern.`,
      });
    }
    if (subsetIdForDirectoryName(dialect.subsetsRel, dialect.subsetDirName, taxonomy) === null) {
      breaches.push({
        id: `standards-subset-prefix-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset dir "${dialect.subsetDirName}" is not declared by its exact semantic subset roster`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset dir under 🪆️subsets/ must resolve through either its owner-scoped semantic roster or the default subset convention.",
        solution: `Rename ${dialect.subsetRel} to one registered semantic subset directory.`,
      });
    } else if (dialect.subsetId !== ((taxonomy as any).subsetAnyId ?? "*") && !subsetSlugPattern.test(dialect.subsetId)) {
      breaches.push({
        id: `standards-subset-slug-${dialect.subsetRel}`,
        summary: `"${dialect.subsetRel}" subset id "${dialect.subsetId}" does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Real subset ids are normalized lowercase identifiers naming an industry conformance profile/class/view or semantic type (✳️a, ✳️cc6, ✳️rv, 🧊️brep, 🔺️mesh, …), never a version or a conformance level.",
        solution: `Rename ${dialect.subsetRel} to match the slug pattern.`,
      });
    }
    for (const child of standardChildDirs) {
      if (child === POLICY_SUBSETS_DIR) continue;
      if (existsSync(join(repoRoot, `${dialect.standardRel}/${child}`))) continue;
      breaches.push({
        id: `standards-standard-child-${dialect.standardRel}-${child}`,
        summary: `"${dialect.standardRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every standard carries engine/subsets per 🔣️taxonomy.json standardChildDirs.",
        solution: `Add ${dialect.standardRel}/${child}/.`,
      });
    }
    for (const child of subsetChildDirs) {
      if (existsSync(join(repoRoot, `${dialect.subsetRel}/${child}`))) continue;
      breaches.push({
        id: `standards-subset-child-${dialect.subsetRel}-${child}`,
        summary: `"${dialect.subsetRel}" is missing required child ${child}/`,
        kind: "stdio-artifacts/standards-coverage",
        scope: dialect.artRel,
        priority: "high",
        reason: "Every subset carries schema/io; lifecycle capabilities are derived from their hooks.",
        solution: `Add ${dialect.subsetRel}/${child}/.`,
      });
    }
  }
  return breaches;
}

/** 🔣 One row per (migrated artifact, standard) — the manifest a `policyStandardSubsetVocabularyBreaches` group checks. */
type PolicyStandardManifestGroup = {
  artRel: string;
  standardRel: string;
  standardSlug: string;
  manifestRel: string;
  dialects: PolicyArtifactDialect[];
};

function policyGroupDialectsByStandard(repoRoot: string): PolicyStandardManifestGroup[] {
  const taxonomy = loadTaxonomy();
  const subsetsDirName = taxonomy.subsetsDirName;
  if (!taxonomy.subsetsManifestFileKindId) throw new Error("[taxonomy] subsets manifest file kind is not declared.");
  const manifestFilename = canonicalFilenameForKind(taxonomy.subsetsManifestFileKindId, taxonomy);
  const groups = new Map<string, PolicyStandardManifestGroup>();
  for (const dialect of policyListArtifactDialectDirs(repoRoot)) {
    let group = groups.get(dialect.standardRel);
    if (!group) {
      group = {
        artRel: dialect.artRel,
        standardRel: dialect.standardRel,
        standardSlug: dialect.standardSlug,
        manifestRel: `${dialect.standardRel}/${subsetsDirName}/${manifestFilename}`,
        dialects: [],
      };
      groups.set(dialect.standardRel, group);
    }
    group.dialects.push(dialect);
  }
  return [...groups.values()];
}

/**
 * ⚖️ Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: every migrated standard declares
 * its real industry subset vocabulary (or, for non-stdio/"domain" artifacts, explicitly declares
 * none) in `🪆️subsets/🔣️.json`, and that declaration must always equal what's actually on
 * disk — in both directions, so a manifest can never silently drift ahead of or behind the real
 * dirs. Real (non-`*`) subsets on stdio artifacts additionally need a registered `SubsetValidator`
 * on their subset composer — the static half of "every real subset gets a real validator"; the
 * generic runtime half (`io.subset.validator-missing`) is `run_subset_validation` in
 * `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs`.
 */
export function policyStandardSubsetVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const anyId = (taxonomy as any).subsetAnyId ?? "*";
  const subsetSlugPattern = new RegExp((taxonomy as any).subsetSlugPattern ?? "^[a-z0-9][a-z0-9.\\-]*$");
  const stdioPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const group of policyGroupDialectsByStandard(repoRoot)) {
    const isStdio = group.artRel.startsWith(stdioPrefix);
    const manifestAbs = join(repoRoot, group.manifestRel);
    if (!existsSync(manifestAbs)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-${group.standardRel}`,
        summary: `"${group.standardRel}" has no ${group.manifestRel.split("/").pop()} subset vocabulary manifest`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: ("Every standard declares its subset vocabulary in 🪆️subsets/" + canonicalPrimaryFilenameForKind(loadTaxonomy().semanticManifestFileKindId) + " — the single source of truth policy checks on-disk dirs against."),
        solution: `Create ${group.manifestRel} declaring {"${anyId}": {...}${isStdio ? ", plus each real subset this standard actually has" : ""}}.`,
      });
      continue;
    }
    let parsed: { artifact?: unknown; standard?: unknown; subsets?: Record<string, unknown> } | undefined;
    try {
      parsed = JSON.parse(readFileSync(manifestAbs, "utf8"));
    } catch (e) {
      breaches.push({
        id: `standards-subset-vocabulary-invalid-${group.standardRel}`,
        summary: `"${group.manifestRel}" is not valid JSON (${(e as Error).message})`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The subset vocabulary manifest must parse as JSON.",
        solution: `Fix ${group.manifestRel}.`,
      });
      continue;
    }
    const declaredIds = Object.keys(parsed?.subsets ?? {});
    if (parsed?.standard !== group.standardSlug) {
      breaches.push({
        id: `standards-subset-vocabulary-standard-mismatch-${group.standardRel}`,
        summary: `"${group.manifestRel}" declares standard "${String(parsed?.standard)}" but lives under 🔖️${group.standardSlug}/`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "The manifest's own standard field must match the 🔖️<slug> directory it lives under.",
        solution: `Fix "standard" in ${group.manifestRel} to "${group.standardSlug}".`,
      });
    }
    if (!declaredIds.includes(anyId)) {
      breaches.push({
        id: `standards-subset-vocabulary-missing-any-${group.standardRel}`,
        summary: `"${group.manifestRel}" does not declare the base subset "${anyId}"`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Every standard carries the unconstrained base subset; it must always be declared.",
        solution: `Add "${anyId}" to the "subsets" object in ${group.manifestRel}.`,
      });
    }
    for (const id of declaredIds) {
      if (id === anyId || subsetSlugPattern.test(id)) continue;
      breaches.push({
        id: `standards-subset-vocabulary-bad-id-${group.standardRel}-${id}`,
        summary: `"${group.manifestRel}" declares subset id "${id}" which does not match ${subsetSlugPattern}`,
        kind: "stdio-artifacts/standards-subset-vocabulary",
        scope: group.artRel,
        priority: "high",
        reason: "Declared subset ids must be normalized lowercase identifiers.",
        solution: `Fix or remove "${id}" in ${group.manifestRel}.`,
      });
    }
    if (!isStdio) {
      const extra = declaredIds.filter((id) => id !== anyId);
      if (extra.length > 0) {
        breaches.push({
          id: `standards-subset-vocabulary-domain-real-subset-${group.standardRel}`,
          summary: `"${group.manifestRel}" declares real subset(s) [${extra.join(", ")}] on a non-stdio (domain) artifact`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "Domain (non-stdio) artifacts stay at v1 with the \"*\" subset only — real industry subset vocabularies are a stdio-only concept.",
          solution: `Remove [${extra.join(", ")}] from ${group.manifestRel}, or move this artifact under stdio if it genuinely has industry subsets.`,
        });
      }
    }
    const subsetsRel = `${group.standardRel}/${taxonomy.subsetsDirName}`;
    const declaredDirs = new Set(declaredIds.map((id) => subsetDirectoryNameForId(subsetsRel, id, taxonomy)).filter((name): name is string => name !== null));
    for (const id of declaredIds) if (subsetDirectoryNameForId(subsetsRel, id, taxonomy) === null) breaches.push({
      id: `standards-subset-vocabulary-unmapped-id-${group.standardRel}-${id}`,
      summary: `"${group.manifestRel}" declares subset id "${id}" without an owner-scoped physical directory identity`,
      kind: "stdio-artifacts/standards-subset-vocabulary",
      scope: group.artRel,
      priority: "high",
      reason: "Every declared logical subset id must map to one exact semantic directory in its standard's roster.",
      solution: `Register an exact physical directory for "${id}" under ${subsetsRel}.`,
    });
    const actualDirs = new Set(group.dialects.map((d) => d.subsetDirName));
    for (const dir of actualDirs) {
      if (!declaredDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-undeclared-dir-${group.standardRel}-${dir}`,
          summary: `"${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/${dir}" exists on disk but is not declared in ${group.manifestRel}`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Add an entry for this subset to ${group.manifestRel}.`,
        });
      }
    }
    for (const dir of declaredDirs) {
      if (!actualDirs.has(dir)) {
        breaches.push({
          id: `standards-subset-vocabulary-missing-dir-${group.standardRel}-${dir}`,
          summary: `${group.manifestRel} declares a subset whose dir "${dir}" does not exist under ${group.standardRel}/${(taxonomy as any).subsetsDirName ?? POLICY_SUBSETS_DIR}/`,
          kind: "stdio-artifacts/standards-subset-vocabulary",
          scope: group.artRel,
          priority: "high",
          reason: "On-disk subset dirs and the manifest's declared vocabulary must be exactly equal, in both directions.",
          solution: `Create the ${dir}/ subset dir, or remove its entry from ${group.manifestRel} if it was declared ahead of the real implementation.`,
        });
      }
    }
    if (isStdio) {
      for (const dialect of group.dialects) {
        if (dialect.subsetId === anyId) continue;
        const composerRel = `${dialect.subsetRel}/🚪️io/${POLICY_RS_COMPONENT_LEAF}`;
        const composerAbs = join(repoRoot, composerRel);
        const body = existsSync(composerAbs) ? readFileSync(composerAbs, "utf8") : "";
        const hasValidatorImpl = policyRustFileHasRealTraitImpl(body, "SubsetValidator");
        const hasRegisterCall = /register_subset_validator\s*\(/.test(policyStripRustCommentsAndStrings(body));
        if (!hasValidatorImpl || !hasRegisterCall) {
          breaches.push({
            id: `standards-subset-vocabulary-validator-missing-${dialect.subsetRel}`,
            summary: `"${dialect.subsetRel}" is a real subset but its IO hook does not ${!hasValidatorImpl ? "implement SubsetValidator" : "call register_subset_validator"}`,
            kind: "stdio-artifacts/standards-subset-vocabulary",
            scope: dialect.artRel,
            priority: "high",
            reason: "Every real (non-\"*\") stdio subset registers a real SubsetValidator (io.subset.validator-missing otherwise fires at runtime for every compose of this dialect) — see the PDF/A pilot.",
            solution: `Add \`impl SubsetValidator for ...\` and a \`register_subset_validator(...)\` call to ${composerRel}.`,
          });
        }
      }
    }
  }
  return breaches;
}
