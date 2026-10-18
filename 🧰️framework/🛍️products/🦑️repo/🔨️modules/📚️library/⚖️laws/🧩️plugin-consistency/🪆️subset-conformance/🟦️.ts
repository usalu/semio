/** 🔖️ Wave 4 plugin consistency policy: subset conformance and artifact viewer and editor rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalFilenameForKind, canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { schemaFacetFormatEntries } from "../../../🔍️discovery/⚖️fixed-contract-resolution/🟦️.ts";
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { subsetIdForDirectoryName } from "../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { policyListPluginArtifactDirs } from "../../../🔍️discovery/🗿️artifact/🏠️roots/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, relative } from "node:path";
import { POLICY_ENGINE_FACET } from "../📋️allowlists/🟦️.ts";
import { policyOwnerOwnComponentFiles } from "../✍️handcrafted-mutation/🟦️.ts";
import { taxonomyMappedFilename } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

//#region 🔧️PolicyRuleSubsetConformance
/** 🪆️ Subset ownership + roundtrip readiness policies (ticket 26/08/12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS). Medium until seal. */

export function policyListTopLevelSubsetDirs(repoRoot: string): string[] {
  const taxonomy = loadTaxonomy();
  const out: string[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return out;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const arts = join(pluginsRoot, plugin.name, taxonomy.artifactsDirName ?? "🗿️artifacts");
    if (!existsSync(arts)) continue;
    for (const art of readdirSync(arts, { withFileTypes: true })) {
      if (!art.isDirectory()) continue;
      const standards = join(arts, art.name, taxonomy.standardsDirName ?? "🏅️standards");
      if (!existsSync(standards)) continue;
      for (const std of readdirSync(standards, { withFileTypes: true })) {
        if (!std.isDirectory() || !std.name.startsWith(taxonomy.standardDirPrefix ?? "🔖️")) continue;
        const subsets = join(standards, std.name, taxonomy.subsetsDirName ?? "🪆️subsets");
        if (!existsSync(subsets)) continue;
        const subsetsRel = relative(repoRoot, subsets).replaceAll("\\", "/");
        for (const sub of readdirSync(subsets, { withFileTypes: true })) {
          if (!sub.isDirectory() || subsetIdForDirectoryName(subsetsRel, sub.name, taxonomy) === null) continue;
          out.push(join("✏️s/🔌️plugins", plugin.name, taxonomy.artifactsDirName ?? "🗿️artifacts", art.name, taxonomy.standardsDirName ?? "🏅️standards", std.name, taxonomy.subsetsDirName ?? "🪆️subsets", sub.name).replaceAll("\\", "/"));
        }
      }
    }
  }
  return out;
}

/** 🪆️ Every subset must own schema, engine, io, and (eventually) examples. */
export function policySubsetFacetTotalityBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const required = taxonomy.subsetChildDirs ?? ["🧬️schema", "⚙️engine", "🚪️io", "📚️examples"];
  const breaches: BreachRecord[] = [];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    for (const child of required) {
      // examples are required structurally but many are still relocating — keep medium
      if (!existsSync(join(repoRoot, subRel, child))) {
        breaches.push({
          id: `subset-facet-missing-${subRel}/${child}`,
          summary: `"${subRel}" is missing required child ${child}/`,
          kind: "subset-conformance/facet-totality",
          scope: subRel,
          priority: "medium",
          reason: "Subsets own schema, engine, IO, and examples.",
          solution: `Create ${subRel}/${child}/ as part of the subset ownership migration.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * ⚙️ An artifact is a `🧬️schema` (snapshot, diff, mutations, inferences) plus a `🚪️io` system — never
 * an engine. Behaviour belongs to the app that edits the artifact (`🎛️apps/<app>/⚙️engine`, already a
 * required app component); pure algorithms belong one level up, in a module's `⚙️engine`, which
 * `taxonomyLeafParentDirs` keeps globally legal.
 *
 * Replaces the two rules that previously *mandated* the facet — `policySubsetEnginePresenceBreaches`
 * and `policyArtifactEnginePresenceBreaches` — the latter of which gated at `high` on a missing
 * folder. Both served `ArtifactEngine`, a trait that never shipped (`grep -rn "trait ArtifactEngine"`
 * → 0 hits; see `🏪️store/🦀️.rs`'s own note that it "never existed as a live trait").
 *
 * Lands at `low` while the 95 existing dirs burn down, then rises to `high` once the count is zero —
 * verified by counting directories on disk, never via the breach cache. Ticket
 * 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES (#2553).
 */
export function policyArtifactEngineFacetForbiddenBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    for (const owner of policyArtifactEngineOwnerDirs(repoRoot, artRel)) {
      if (!existsSync(join(repoRoot, join(owner, POLICY_ENGINE_FACET)))) continue;
      breaches.push({
        id: `artifact-engine-forbidden-${owner}`,
        summary: `"${owner}" declares a forbidden ${POLICY_ENGINE_FACET}/ facet`,
        kind: "subset-conformance/engine-forbidden",
        scope: owner,
        priority: "low",
        reason: "An artifact has a schema (snapshot, diff, mutations, inferences), an io system and examples — never an engine.",
        solution: `Dissolve ${owner}/${POLICY_ENGINE_FACET} per the D0–D6 procedure: derived compute → 🧬️schema/💡️inferences, (de)serialization → 🚪️io, edits → a 🧬️mutations triad, behaviour → the app's ⚙️engine, pure algorithms → a module's ⚙️engine one level up. Then delete the directory.`,
      });
    }
  }
  return breaches;
}

/** ⚙️ Every level of an artifact tree that could carry an `⚙️engine`: the artifact, its standards, and their subsets. */
function policyArtifactEngineOwnerDirs(repoRoot: string, artRel: string): string[] {
  const owners = [artRel];
  const standardsRel = join(artRel, "🏅️standards");
  for (const std of policyReaddirSafe(repoRoot, standardsRel).filter((e) => e.isDirectory)) {
    const stdRel = join(standardsRel, std.name);
    owners.push(stdRel);
    const subsetsRel = join(stdRel, "🪆️subsets");
    for (const subset of policyReaddirSafe(repoRoot, subsetsRel).filter((e) => e.isDirectory)) {
      owners.push(join(subsetsRel, subset.name));
    }
  }
  return owners;
}

/** 📚️ Examples must not remain at artifact/standard level. */
export function policyExampleNotAtArtifactLevelBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const arts = join(pluginsRoot, plugin.name, "🗿️artifacts");
    if (!existsSync(arts)) continue;
    for (const art of readdirSync(arts, { withFileTypes: true })) {
      if (!art.isDirectory()) continue;
      const artEx = join(arts, art.name, "📚️examples");
      if (existsSync(artEx)) {
        const rel = join("✏️s/🔌️plugins", plugin.name, "🗿️artifacts", art.name, "📚️examples").replaceAll("\\", "/");
        breaches.push({
          id: `artifact-examples-remaining-${rel}`,
          summary: `artifact-level examples still present at "${rel}"`,
          kind: "subset-conformance/example-placement",
          scope: rel,
          priority: "medium",
          reason: "Examples belong under 🪆️subsets/<id>/📚️examples/.",
          solution: "Move example units into the owning subset then delete the artifact examples dir.",
        });
      }
      const standards = join(arts, art.name, "🏅️standards");
      if (!existsSync(standards)) continue;
      for (const std of readdirSync(standards, { withFileTypes: true })) {
        if (!std.isDirectory()) continue;
        const stdEx = join(standards, std.name, "📚️examples");
        if (existsSync(stdEx)) {
          const rel = join("✏️s/🔌️plugins", plugin.name, "🗿️artifacts", art.name, "🏅️standards", std.name, "📚️examples").replaceAll("\\", "/");
          breaches.push({
            id: `standard-examples-remaining-${rel}`,
            summary: `standard-level examples still present at "${rel}"`,
            kind: "subset-conformance/example-placement",
            scope: rel,
            priority: "medium",
            reason: "Examples belong under 🪆️subsets/<id>/📚️examples/.",
            solution: "Move into the owning subset then delete the standard examples dir.",
          });
        }
      }
    }
  }
  return breaches;
}

/** 🧹 Phantom Chinese 🏅️标准 trees must be deleted. */
export function policyPhantomStandardsBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  const walk = (dir: string, rel: string) => {
    if (!existsSync(dir)) return;
    for (const ent of readdirSync(dir, { withFileTypes: true })) {
      if (!ent.isDirectory()) continue;
      const childRel = `${rel}/${ent.name}`.replaceAll("\\", "/");
      if (ent.name.includes("标准")) {
        breaches.push({
          id: `phantom-standards-${childRel}`,
          summary: `corrupt standards tree "${childRel}"`,
          kind: "subset-conformance/phantom-standards",
          scope: childRel,
          priority: "medium",
          reason: "Chinese-named 🏅️标准 directories are corrupt phantoms.",
          solution: "Delete the entire phantom tree.",
        });
      }
      walk(join(dir, ent.name), childRel);
    }
  };
  walk(pluginsRoot, "✏️s/🔌️plugins");
  return breaches;
}

/** 🪞 Meta-only TypeScript subset schema leaves (≤7 lines) must become real mirrors. */
export function policySubsetTsParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    const ts = join(repoRoot, subRel, "🧬️schema", (POLICY_TS_COMPONENT_LEAF));
    if (!existsSync(ts)) continue;
    const lines = readFileSync(ts, "utf8").split(/\r?\n/).length;
    if (lines > 0 && lines <= 7) {
      breaches.push({
        id: `subset-ts-meta-stub-${subRel}`,
        summary: `"${subRel}/🧬️schema/${POLICY_TS_COMPONENT_LEAF}" is a ${lines}-line meta stub`,
        kind: "subset-conformance/ts-parity",
        scope: `${subRel}/🧬️schema/${POLICY_TS_COMPONENT_LEAF}`,
        priority: "medium",
        reason: "Derived/owning subsets require a real TypeScript implementation mirroring Rust.",
        solution: "Replace the meta stub with a full TypeScript mirror of the Rust conformance/schema surface.",
      });
    }
  }
  return breaches;
}

/** 🚫 Glue compatibility shims keeping pre-migration module paths must be deleted. */
export function policyNoGlueShimBlocksBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins");
  if (!existsSync(pluginsRoot)) return breaches;
  for (const plugin of readdirSync(pluginsRoot, { withFileTypes: true })) {
    if (!plugin.isDirectory()) continue;
    const glue = join(pluginsRoot, plugin.name, "📦️packages/🦀️rust/🦀️.rs");
    if (!existsSync(glue)) continue;
    const content = readFileSync(glue, "utf8");
    if (/Shims:\s*keep pre-migration|pre-migration module paths/i.test(content)) {
      const rel = join("✏️s/🔌️plugins", plugin.name, "📦️packages/🦀️rust/🦀️.rs").replaceAll("\\", "/");
      breaches.push({
        id: `glue-shim-${rel}`,
        summary: `"${rel}" still contains pre-migration shim blocks`,
        kind: "subset-conformance/no-glue-shim-blocks",
        scope: rel,
        priority: "medium",
        reason: "Greenfield forbids compatibility shims.",
        solution: "Delete shim blocks after generated subset registration lands.",
      });
    }
  }
  return breaches;
}

/** 📦 Aggregate subset-conformance scanners. */
export function policySubsetConformanceBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policySubsetFacetTotalityBreaches(repoRoot),
    ...policyArtifactEngineFacetForbiddenBreaches(repoRoot),
    ...policyExampleNotAtArtifactLevelBreaches(repoRoot),
    ...policyPhantomStandardsBreaches(repoRoot),
    ...policySubsetTsParityBreaches(repoRoot),
    ...policyNoGlueShimBlocksBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleSubsetConformance


//#region 🔧️PolicyRuleArtifactViewersEditors
/**
 * 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, contract §6 "Policies" (W0/W1,
 * add-only). Four rules over the new `👁️viewer`/`✏️editor` subset-surface axis, driven entirely by
 * `🔣️taxonomy.json` (`viewerDirName`/`editorDirName`/`surfaceRoles`/`surfaceDirNames`/
 * `windowLeafLangs` and ecosystem component file-kind IDs) — never a hardcoded emoji literal for the axis itself.
 * `policySubsetSurfaceCompletenessBreaches` and `policyViewerPurityBreaches` land at `"medium"`/`"high"`
 * respectively per the contract; `policyOsConfigShapeBreaches` locks a facet that already shipped
 * (C4), so it fails the gate like any other completed-shape lock.
 */

/** 👁️✏️ Every OWNED subset (has `🧬️schema`, independent of `🚪️io` — see the scaffolder's identical
 * predicate in `📇️registry/📜️script.ts`) carries both `👁️viewer`/`✏️editor`, each with ≥1 mode
 * carrying ≥1 window with both `windowLeafLangs` leaves. A surface whose scaffolded leaves still carry
 * the `SCAFFOLD` marker is a separate, non-passing breach kind — see the scaffolder region docstring
 * in `📇️registry/📜️script.ts` for why the marker exists. */
export function policySubsetSurfaceCompletenessBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const scaffoldLeafNames = new Set([
    taxonomyMappedFilename(taxonomy, taxonomy.componentFileKinds, "🦀️rust"),
    taxonomyMappedFilename(taxonomy, taxonomy.componentFileKinds, "🟦️typescript"),
  ]);
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    if (!existsSync(join(repoRoot, subRel, "🧬️schema"))) continue;
    for (const role of taxonomy.surfaceRoles) {
      const surfaceDirName = taxonomy.surfaceDirNames[role];
      const surfaceRel = `${subRel}/${surfaceDirName}`;
      if (!existsSync(join(repoRoot, surfaceRel))) {
        breaches.push({
          id: `subset-surface-missing-${surfaceRel}`,
          summary: `"${subRel}" is missing required surface "${surfaceDirName}"`,
          kind: "taxonomy/surface-completeness",
          scope: subRel,
          priority: "medium",
          reason: `Every owned subset carries both ${taxonomy.subsetRequiredSurfaceDirs.join(", ")}.`,
          solution: `cd 📇️registry && bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> ${role} (or "new surface --all" to batch every missing surface).`,
        });
        continue;
      }
      const modesRel = `${surfaceRel}/${taxonomy.modesDirName}`;
      let hasCompleteWindow = false;
      for (const mode of policyReaddirSafe(repoRoot, modesRel).filter((e) => e.isDirectory)) {
        const windowsRel = `${modesRel}/${mode.name}/${taxonomy.windowsDirName}`;
        for (const w of policyReaddirSafe(repoRoot, windowsRel).filter((e) => e.isDirectory)) {
          const windowDir = `${windowsRel}/${w.name}`;
          if (taxonomy.windowLeafLangs.every((lang) => existsSync(join(repoRoot, windowDir, taxonomyMappedFilename(taxonomy, taxonomy.componentFileKinds, lang))))) hasCompleteWindow = true;
        }
      }
      if (!hasCompleteWindow) {
        breaches.push({
          id: `subset-surface-window-incomplete-${surfaceRel}`,
          summary: `"${surfaceRel}" has no mode with a window carrying both ${taxonomy.windowLeafLangs.join(", ")} leaves`,
          kind: "taxonomy/surface-completeness",
          scope: subRel,
          priority: "medium",
          reason: "Every surface must carry at least one mode with at least one window declaring every windowLeafLangs leaf.",
          solution: `Scaffold with "new surface … ${role}" (📇️registry/), then author a real mode/window.`,
        });
      }
      const scaffoldLeaves = policyWalkRelFiles(repoRoot, [surfaceRel], (_p, name) => scaffoldLeafNames.has(name)).filter((relPath) => policyReadFileSafe(repoRoot, relPath).includes("SCAFFOLD"));
      if (scaffoldLeaves.length > 0) {
        breaches.push({
          id: `subset-surface-scaffold-residue-${surfaceRel}`,
          summary: `"${surfaceRel}" still carries ${scaffoldLeaves.length} SCAFFOLD-marker leaf(ves)`,
          kind: "taxonomy/surface-scaffold-residue",
          scope: subRel,
          priority: "medium",
          reason: "A surface at scaffold content is not a finished W2 packet — the SCAFFOLD marker exists so this can never masquerade as done.",
          solution: `Author real render/view-model/commands for ${scaffoldLeaves.slice(0, 3).join(", ")}${scaffoldLeaves.length > 3 ? ", …" : ""}, removing the SCAFFOLD marker.`,
        });
      }
    }
  }
  return breaches;
}

/** 👁️ No file under a `👁️viewer` dir may reference mutation dispatch, `artifact_mutations`, or an
 * `::editor::` module path — `ViewEmit` is structurally read-only (contract §2.2); this is the
 * filesystem-content half of that guarantee. */
export function policyViewerPurityBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const patterns: readonly { re: RegExp; label: string }[] = [
    { re: /\.mutation\(/, label: ".mutation(" },
    { re: /Emit::mutations/, label: "Emit::mutations" },
    { re: /artifact_mutations/, label: "artifact_mutations" },
    { re: /::editor::/, label: "::editor::" },
  ];
  for (const subRel of policyListTopLevelSubsetDirs(repoRoot)) {
    const viewerRel = `${subRel}/${taxonomy.viewerDirName}`;
    if (!existsSync(join(repoRoot, viewerRel))) continue;
    for (const relPath of policyWalkRelFiles(repoRoot, [viewerRel], () => true)) {
      const content = policyReadFileSafe(repoRoot, relPath);
      for (const { re, label } of patterns) {
        if (!re.test(content)) continue;
        breaches.push({
          id: `viewer-purity-${relPath}-${label}`,
          summary: `"${relPath}" under a ${taxonomy.viewerDirName} dir matches forbidden pattern "${label}"`,
          kind: "taxonomy/viewer-purity",
          scope: subRel,
          priority: "high",
          reason: "ViewEmit is structurally read-only; no file under a viewer surface may reference mutation dispatch, artifact_mutations, or an editor-only module path.",
          solution: `Remove the "${label}" reference from ${relPath}, or move the mutating logic to the sibling ${taxonomy.editorDirName} surface.`,
        });
      }
    }
  }
  return breaches;
}

/** 🔗️A surface dir under a subset this plugin does NOT own (no `🧬️schema` of its own — a contributed,
 * mirrored surface per `contributedSubsetChildDirs`) must be backed by a `.depends_on(<owner-plugin>)`
 * on the contributing plugin's builder, exactly like `policyContributionTargetBreaches` requires for
 * mutation/inference contributions. Dormant on today's tree (no contributed surface exists yet — every
 * scaffolded surface sits under a subset its own plugin owns), verified structurally sound so it fires
 * correctly once W2 packets start contributing surfaces onto foreign kinds. */
export function policyContributedSurfaceTargetBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const subsetRels = policyListTopLevelSubsetDirs(repoRoot);
  const pluginRootOf = (subRel: string): string => subRel.split("/").slice(0, 3).join("/");
  const ownerRootBySuffix = new Map<string, string>();
  for (const subRel of subsetRels) {
    if (!existsSync(join(repoRoot, subRel, "🧬️schema"))) continue;
    const pluginRoot = pluginRootOf(subRel);
    const suffix = subRel.slice(pluginRoot.length + 1);
    if (!ownerRootBySuffix.has(suffix)) ownerRootBySuffix.set(suffix, pluginRoot);
  }
  for (const subRel of subsetRels) {
    if (existsSync(join(repoRoot, subRel, "🧬️schema"))) continue; // this plugin owns this triple — not a contribution
    const pluginRoot = pluginRootOf(subRel);
    const suffix = subRel.slice(pluginRoot.length + 1);
    const ownerRoot = ownerRootBySuffix.get(suffix);
    for (const role of taxonomy.surfaceRoles) {
      const surfaceRel = `${subRel}/${taxonomy.surfaceDirNames[role]}`;
      if (!existsSync(join(repoRoot, surfaceRel))) continue;
      if (!ownerRoot) {
        breaches.push({
          id: `contributed-surface-unknown-owner-${surfaceRel}`,
          summary: `"${surfaceRel}" contributes a surface but no other plugin owns "${suffix}"`,
          kind: "plugin-dependency/contributed-surface-target",
          scope: pluginRoot,
          priority: "medium",
          reason: "A contributed surface must mirror the path of a subset some OTHER plugin actually owns (has 🧬️schema for).",
          solution: `Verify "${suffix}" is a real owned subset elsewhere, or remove ${surfaceRel} if it is stray.`,
        });
        continue;
      }
      if (ownerRoot === pluginRoot) continue;
      const ownerPluginId = policyStripEmoji(ownerRoot.split("/").pop() ?? "");
      const declared = policyOwnerOwnComponentFiles(repoRoot, pluginRoot).some((relPath) =>
        new RegExp(`\\.depends_on\\s*\\(\\s*"${ownerPluginId}"`).test(policyReadFileSafe(repoRoot, relPath)),
      );
      if (declared) continue;
      breaches.push({
        id: `contributed-surface-target-undeclared-${surfaceRel}`,
        summary: `"${surfaceRel}" contributes a surface onto "${ownerPluginId}"'s subset without declaring .depends_on("${ownerPluginId}")`,
        kind: "plugin-dependency/contributed-surface-target",
        scope: pluginRoot,
        priority: "high",
        reason: "A surface contributed onto a subset this plugin does not own must be backed by a declared dependency on the owning plugin (surface.contribution-not-permitted).",
        solution: `Add .depends_on("${ownerPluginId}", …) to the plugin/extension builder in ${pluginRoot}.`,
      });
    }
  }
  return breaches;
}

const POLICY_OS_CONFIG_ROOT = "🧰️framework/🛍️products/💻️os/🎚️config";
const POLICY_OS_CONFIG_SCHEMA_ID = "os.config.opening";
const POLICY_OS_CONFIG_MUTATION_KINDS = ["set-default-app", "clear-default-app"] as const;

/** 🎚️Locks the shape of the already-shipped C4 OS config facet: five `schemaFormats` leaves, the
 * frozen `os.config.opening` schema id, and both direct `set-default-app`/`clear-default-app`
 * mutations. Content-keyed (schema id / semantic identity), not folder-name-keyed, so it survives a slug
 * rename without drifting from the real frozen contract. */
export function policyOsConfigShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const schemaRel = `${POLICY_OS_CONFIG_ROOT}/🧬️schema`;
  if (!existsSync(join(repoRoot, schemaRel))) {
    breaches.push({
      id: `os-config-shape-missing-schema`,
      summary: `"${schemaRel}" is missing`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: `The OS-level opening-preferences config facet (${POLICY_OS_CONFIG_SCHEMA_ID}) is a frozen C4 facet.`,
      solution: `Restore ${schemaRel}/ per contract §4.`,
    });
    return breaches;
  }
  for (const format of schemaFacetFormatEntries(schemaRel, taxonomy).map(([, f]) => f)) {
    const leafFilename = canonicalPrimaryFilenameForKind(format.fileKindId, taxonomy);
    if (existsSync(join(repoRoot, schemaRel, leafFilename))) continue;
    breaches.push({
      id: `os-config-shape-schema-leaf-${leafFilename}`,
      summary: `"${schemaRel}" is missing required schema leaf "${leafFilename}"`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: `Every schemaFormats leaf for facet kind 🧬️data is frozen for ${POLICY_OS_CONFIG_SCHEMA_ID}.`,
      solution: `Add ${schemaRel}/${leafFilename}.`,
    });
  }
  const rootRust = policyReadFileSafe(repoRoot, schemaRel, (POLICY_RS_COMPONENT_LEAF_NAME));
  if (!rootRust.includes(POLICY_OS_CONFIG_SCHEMA_ID)) {
    breaches.push({
      id: `os-config-shape-schema-id`,
      summary: `"${schemaRel}/${POLICY_RS_COMPONENT_LEAF_NAME}" no longer declares the frozen schema id "${POLICY_OS_CONFIG_SCHEMA_ID}"`,
      kind: "taxonomy/os-config-shape",
      scope: "os/config",
      priority: "high",
      reason: "C4 froze the schema id os.config.opening; OpeningResolver and both hosts key off this exact string.",
      solution: `Restore the "${POLICY_OS_CONFIG_SCHEMA_ID}" schema id constant in ${schemaRel}/${POLICY_RS_COMPONENT_LEAF_NAME}.`,
    });
  }
  const mutationsRel = `${schemaRel}/🧬️mutations`;
  const componentFilename = canonicalFilenameForKind(taxonomy.mutationComponentFileKindId, taxonomy);
  const mutationDirs = policyReaddirSafe(repoRoot, mutationsRel).filter((e) => e.isDirectory);
  for (const mutationKind of POLICY_OS_CONFIG_MUTATION_KINDS) {
    const owner = mutationDirs.find((d) => {
      const component = policyReadFileSafe(repoRoot, mutationsRel, d.name, componentFilename);
      return component.includes(`kind: "${mutationKind}"`) || component.includes(`\"${mutationKind}\"`);
    });
    if (!owner) {
      breaches.push({
        id: `os-config-shape-mutation-missing-${mutationKind}`,
        summary: `"${mutationsRel}" is missing the frozen direct "${mutationKind}" mutation`,
        kind: "taxonomy/os-config-shape",
        scope: "os/config",
        priority: "high",
        reason: `C4 froze both mutation kinds (set-default-app, clear-default-app) on ${POLICY_OS_CONFIG_SCHEMA_ID}.`,
        solution: `Restore ${mutationsRel}/<slug>/${componentFilename} declaring the semantic identity "${mutationKind}".`,
      });
      continue;
    }
    const componentRel = `${mutationsRel}/${owner.name}/${componentFilename}`;
    if (!existsSync(join(repoRoot, componentRel))) {
      breaches.push({
        id: `os-config-shape-mutation-leaf-${componentRel}`,
        summary: `"${componentRel}" is missing`,
        kind: "taxonomy/os-config-shape",
        scope: "os/config",
        priority: "high",
        reason: "Every mutation folder directly owns its mandatory component file.",
        solution: `Add ${componentRel}.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleArtifactViewersEditors
