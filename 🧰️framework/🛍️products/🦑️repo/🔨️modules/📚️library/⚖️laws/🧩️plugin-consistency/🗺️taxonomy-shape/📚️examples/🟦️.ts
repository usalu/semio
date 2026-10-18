import { existsSync, statSync } from "node:fs";
import { join } from "node:path";
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { taxonomyMappedFilename } from "../../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";
import { policyReaddirSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { type PolicyCrateRef, policyNewSurfacePriority } from "../../🏛️foundation/🟦️.ts";
import { policyStripEmoji } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policySurfaceRoots } from "../../../../🔍️discovery/🗺️surface/🟦️.ts";

/**
 * 📏️ Per-example unit shape: `📚️examples/<emoji-slug>/{definition leaves, 🖼️assets/, 🧪️tests/}` under
 * every artifact and every app (apps own examples directly — never under `⚙️engine`). Plugin-root
 * `📚️examples` and plural facet dirs are forbidden.
 */
function policyExampleSlugOk(slug: string, taxonomy: ReturnType<typeof loadTaxonomy>): boolean {
  try {
    return new RegExp(taxonomy.exampleSlugPattern, "u").test(slug);
  } catch {
    return false;
  }
}

/** 📚️ True when `relDir` is an immediate child of `📚️examples` (dynamic emoji-slug leaf parent). */
function policyIsExampleSlugDir(relDir: string): boolean {
  const parts = relDir.replaceAll("\\", "/").split("/");
  return parts.length >= 2 && parts[parts.length - 2] === "📚️examples";
}

function policyValidateExampleUnit(
  repoRoot: string,
  exampleRel: string,
  scope: string,
  priority: BreachRecord["priority"],
): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const slug = exampleRel.split("/").pop() ?? "";
  if ((taxonomy.forbiddenExampleSlugs ?? []).includes(slug) || !policyExampleSlugOk(slug, taxonomy)) {
    breaches.push({
      id: `semio-examples-slug-${exampleRel}`,
      summary: `"${exampleRel}" is not a valid emoji+VS16+kebab example slug`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: `Example dirs must match exampleSlugPattern and must not be placeholders (${(taxonomy.forbiddenExampleSlugs ?? []).join(", ")}).`,
      solution: `Rename ${exampleRel} to an emoji+VS16+kebab slug that describes the scenario.`,
    });
  }
  for (const plural of taxonomy.forbiddenExamplePluralDirs ?? []) {
    if (existsSync(join(repoRoot, exampleRel, plural))) {
      breaches.push({
        id: `semio-examples-plural-${exampleRel}-${plural}`,
        summary: `"${exampleRel}/${plural}" plural example facet dir is forbidden`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example assets live flat under 🖼️assets/ with kind-emoji prefixes — plural dsls/packs/ops/sprs dirs are gone.",
        solution: `Move files from ${exampleRel}/${plural}/ into ${exampleRel}/${taxonomy.exampleAssetsDirName}/ and delete the plural dir.`,
      });
    }
  }
  const rustLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
  const tsLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🟦️typescript");
  const rustTest = taxonomyMappedFilename(taxonomy, taxonomy.exampleTestFileKinds, "🦀️rust");
  const tsTest = taxonomyMappedFilename(taxonomy, taxonomy.exampleTestFileKinds, "🟦️typescript");
  if (!existsSync(join(repoRoot, exampleRel, rustLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-rs-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${rustLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a Rust definition leaf as the single source of truth for id/labels/assets.",
      solution: `Add ${exampleRel}/${rustLeaf}.`,
    });
  }
  if (!existsSync(join(repoRoot, exampleRel, tsLeaf))) {
    breaches.push({
      id: `semio-examples-leaf-ts-${exampleRel}`,
      summary: `"${exampleRel}" is missing definition leaf ${tsLeaf}`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit needs a TypeScript definition leaf for TS consumers.",
      solution: `Add ${exampleRel}/${tsLeaf}.`,
    });
  }
  const assetsRel = `${exampleRel}/${taxonomy.exampleAssetsDirName}`;
  if (!existsSync(join(repoRoot, assetsRel))) {
    breaches.push({
      id: `semio-examples-assets-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleAssetsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries its assets under 🖼️assets/.",
      solution: `Add ${assetsRel}/ with kind-prefixed asset files.`,
    });
  } else {
    for (const entry of policyReaddirSafe(repoRoot, assetsRel)) {
      if (entry.isDirectory) continue;
      const assetAbs = join(repoRoot, assetsRel, entry.name);
      if (statSync(assetAbs).size === 0) {
        breaches.push({
          id: `semio-examples-zero-byte-asset-${exampleRel}-${entry.name}`,
          summary: `"${assetsRel}/${entry.name}" is a 0-byte placeholder asset`,
          kind: "taxonomy/semio-examples",
          scope,
          priority,
          reason: "Example assets must be real, builder-emitted (or genuinely sourced) content — a 0-byte file proves nothing and can't be decoded by any analyzer test.",
          solution: `Replace ${assetsRel}/${entry.name} with a small file the artifact's own builder/analyzer can actually round-trip, or a real fixture.`,
        });
      }
    }
  }
  const testsRel = `${exampleRel}/${taxonomy.exampleTestsDirName}`;
  if (!existsSync(join(repoRoot, testsRel))) {
    breaches.push({
      id: `semio-examples-tests-${exampleRel}`,
      summary: `"${exampleRel}" is missing ${taxonomy.exampleTestsDirName}/`,
      kind: "taxonomy/semio-examples",
      scope,
      priority,
      reason: "Every example unit carries co-located tests under 🧪️tests/.",
      solution: `Add ${testsRel}/ with ${rustTest} and ${tsTest}.`,
    });
  } else {
    if (!existsSync(join(repoRoot, testsRel, rustTest))) {
      breaches.push({
        id: `semio-examples-test-rs-${exampleRel}`,
        summary: `"${testsRel}" is missing ${rustTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the Rust test leaf.",
        solution: `Add ${testsRel}/${rustTest}.`,
      });
    }
    if (!existsSync(join(repoRoot, testsRel, tsTest))) {
      breaches.push({
        id: `semio-examples-test-ts-${exampleRel}`,
        summary: `"${testsRel}" is missing ${tsTest}`,
        kind: "taxonomy/semio-examples",
        scope,
        priority,
        reason: "Example tests must include the TypeScript test leaf.",
        solution: `Add ${testsRel}/${tsTest}.`,
      });
    }
  }
  return breaches;
}

/**
 * 📏️ Semio example layout: ≥1 emoji-slug under each artifact and each app `📚️examples/`
 * (not under `⚙️engine`); slug matches pattern; definition leaf + 🖼️assets/ + 🧪️tests/; no plural
 * dirs; no plugin-root `📚️examples`.
 */
export function policySemioArtifactExamplesBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const examplesDir = "📚️examples";
  const rustExampleLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;
    const scopeId = crate.pluginId || policyStripEmoji(ownerRoot.split("/").pop() ?? "");
    const priority = policyNewSurfacePriority(crate, "medium");
    if (existsSync(join(repoRoot, ownerRoot, examplesDir))) {
      breaches.push({
        id: `semio-examples-plugin-root-${ownerRoot}`,
        summary: `"${ownerRoot}/${examplesDir}" must not exist at plugin root`,
        kind: "taxonomy/semio-examples",
        scope: scopeId,
        priority: policyNewSurfacePriority(crate, "high"),
        reason: "Examples belong under 🗿️artifacts/<artifact>/📚️examples or a subset's 👁️viewer|✏️editor/📚️examples — never at the plugin root.",
        solution: `Move fixtures into artifact or app ${examplesDir}/<emoji-slug>/ units.`,
      });
    }
    const artifactsRoot = `${ownerRoot}/${taxonomy.artifactsDirName}`;
    for (const artifact of policyReaddirSafe(repoRoot, artifactsRoot).filter((e) => e.isDirectory)) {
      const artifactRel = `${artifactsRoot}/${artifact.name}`;
      const examplesRel = `${artifactRel}/${examplesDir}`;
      const artScope = `${scopeId}/${policyStripEmoji(artifact.name)}`;
      if (!existsSync(join(repoRoot, examplesRel))) {
        breaches.push({
          id: `semio-examples-missing-${examplesRel}`,
          summary: `"${examplesRel}" is missing`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Every artifact must ship at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/{${rustExampleLeaf}, ${taxonomy.exampleAssetsDirName}/, ${taxonomy.exampleTestsDirName}/}.`,
        });
        continue;
      }
      const sets = policyReaddirSafe(repoRoot, examplesRel).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-empty-${examplesRel}`,
          summary: `"${examplesRel}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: artScope,
          priority,
          reason: "Artifact examples must contain at least one emoji-slug example unit.",
          solution: `Add ${examplesRel}/<emoji-slug>/ with definition leaves, assets, and tests.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${examplesRel}/${set.name}`, artScope, priority));
      }
    }
    for (const surfaceRoot of policySurfaceRoots(repoRoot, ownerRoot, taxonomy)) {
      // 👁️✏️ Unlike the old 🎛️apps facet, 📚️examples is NOT in surfaceRequiredChildDirs (contract
      // §7.5) — a surface with no examples is legal, so this only validates units that DO exist.
      const [subsetName, roleDirName] = surfaceRoot.split("/").slice(-2);
      const surfaceScope = `${scopeId}/${policyStripEmoji(subsetName ?? "")}/${policyStripEmoji(roleDirName ?? "")}`;
      const surfaceExamples = `${surfaceRoot}/${examplesDir}`;
      if (!existsSync(join(repoRoot, surfaceExamples))) continue;
      const sets = policyReaddirSafe(repoRoot, surfaceExamples).filter((e) => e.isDirectory);
      if (sets.length === 0) {
        breaches.push({
          id: `semio-examples-surface-empty-${surfaceExamples}`,
          summary: `"${surfaceExamples}" has no example slug directory`,
          kind: "taxonomy/semio-examples",
          scope: surfaceScope,
          priority,
          reason: "A declared surface 📚️examples/ must contain at least one emoji-slug example unit.",
          solution: `Add ${surfaceExamples}/<emoji-slug>/ with definition leaves, assets, and tests, or remove the empty ${examplesDir}/ dir.`,
        });
      }
      for (const set of sets) {
        breaches.push(...policyValidateExampleUnit(repoRoot, `${surfaceExamples}/${set.name}`, surfaceScope, priority));
      }
    }
  }
  return breaches;
}

export function policyComponentFileBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const rustFileKindId = taxonomy.componentFileKinds["🦀️rust"]!;
  const typescriptFileKindId = taxonomy.componentFileKinds["🟦️typescript"]!;
  const leafFilename = canonicalPrimaryFilenameForKind(rustFileKindId, taxonomy);
  const sourceExtensions = taxonomy.fileKinds[rustFileKindId]!.extensionChains;
  const sourceExtension = sourceExtensions[0]!;
  const tsLeafFilename = canonicalPrimaryFilenameForKind(typescriptFileKindId, taxonomy);
  const tsSourceExtensions = taxonomy.fileKinds[typescriptFileKindId]!.extensionChains;
  const tsSourceExtension = tsSourceExtensions[0]!;
  const artifactFacetDirs = new Set(taxonomy.artifactComponentDirs ?? []);
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (crate.shape !== "taxonomy") continue;
    const ownerRoot = crate.ownerRel;

    const walk = (relDir: string): void => {
      const entries = policyReaddirSafe(repoRoot, relDir);
      const parentName = relDir.split("/").pop() ?? "";
      const hasRustFile = entries.some((e) => !e.isDirectory && sourceExtensions.some((extension) => e.name.endsWith(extension)));
      const hasTsFile = entries.some((e) => !e.isDirectory && tsSourceExtensions.some((extension) => e.name.endsWith(extension)));
      const exampleRustLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🦀️rust");
      const exampleTsLeaf = taxonomyMappedFilename(taxonomy, taxonomy.exampleFileKinds, "🟦️typescript");
      if (policyIsExampleSlugDir(relDir)) {
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === exampleRustLeaf)) {
          breaches.push({
            id: `component-file-example-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${exampleRustLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs (grandparent 📚️examples) use ${exampleRustLeaf} as the definition leaf.`,
            solution: `Rename ${relDir}'s primary source file to ${exampleRustLeaf}.`,
          });
        }
        if (hasTsFile && !entries.some((e) => !e.isDirectory && e.name === exampleTsLeaf)) {
          breaches.push({
            id: `component-file-example-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${exampleTsLeaf}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Example slug dirs use ${exampleTsLeaf} as the TypeScript definition leaf.`,
            solution: `Rename ${relDir}'s primary TypeScript file to ${exampleTsLeaf}.`,
          });
        }
      } else if (taxonomy.taxonomyLeafParentDirs.includes(parentName)) {
        // relDir itself is a vocabulary dir (e.g. 🔧️op/) whose immediate .rs files are the leaf — or,
        // for 🎮️commands/🛠️tools/📌️panels, an extra <name>/ nesting level holds the leaf instead.
        if (hasRustFile && !entries.some((e) => !e.isDirectory && e.name === leafFilename)) {
          breaches.push({
            id: `component-file-${relDir}`,
            summary: `"${relDir}" has ${sourceExtension} source but no ${leafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: every taxonomy leaf's primary file must be literally named ${leafFilename}.`,
            solution: `Rename ${relDir}'s primary source file to ${leafFilename} (sibling 🦀️<topic>${sourceExtension} files may stay).`,
          });
        }
        if (artifactFacetDirs.has(parentName) && hasTsFile && !entries.some((e) => !e.isDirectory && e.name === tsLeafFilename)) {
          breaches.push({
            id: `component-file-ts-${relDir}`,
            summary: `"${relDir}" has ${tsSourceExtension} source but no ${tsLeafFilename}`,
            kind: "taxonomy/component-file",
            scope: crate.pluginId || crate.ownerRel,
            priority: policyNewSurfacePriority(crate, "medium"),
            reason: `Discovery contract: artifact facet dirs carry the TypeScript taxonomy leaf (${tsLeafFilename}) alongside ${leafFilename} and any mapped artifactSpecFileKinds entry.`,
            solution: `Rename ${relDir}'s primary TypeScript facade to ${tsLeafFilename} (normative 📖️/📡️ *.semio specs mapped in artifactSpecFileKinds may stay as siblings).`,
          });
        }
      }
      for (const entry of entries.filter((e) => e.isDirectory)) walk(`${relDir}/${entry.name}`);
    };
    // 👁️✏️ Surfaces now nest under 🗿️artifacts/*/🏅️standards/*/🪆️subsets/*/{👁️viewer,✏️editor}/, which
    // this recursive walk already reaches — no separate root needed once 🎛️apps is gone.
    walk(`${ownerRoot}/${taxonomy.artifactsDirName}`);
  }
  return breaches;
}
