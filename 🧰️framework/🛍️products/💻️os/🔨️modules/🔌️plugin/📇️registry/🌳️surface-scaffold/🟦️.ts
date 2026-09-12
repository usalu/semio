import { existsSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import type { ArtifactScaffoldLeaf, ArtifactScaffoldOptions, ArtifactScaffoldResult } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { authorArtifactScaffold, BundleScript, getWorkspaceRoot } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { PLUGIN_AREAS, TAXONOMY, primaryFilenameForKind } from "../🔎️discovery/🟦️.ts";
import { SCHEMA_FACET_DIR, WINDOW_EMPTY_FACET_FILENAME, listDirs } from "../🗿️taxonomy-validation/🟦️.ts";


//#endregion 🗿️TaxonomyValidator

//#region 🔖️SurfaceScaffolder
/**
 * 🏗️ `new surface` — the permanent, taxonomy-derived scaffolder for the 286-surface tree (143 subsets
 * × `{viewer, editor}`), ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Every path segment
 * below comes from `🔣️taxonomy.json` except the default mode/window ids (`SURFACE_DEFAULT_MODE_DIRNAME`
 * / `SURFACE_DEFAULT_WINDOW_DIRNAME`), which are not taxonomy vocabulary — a *specific* mode/window is
 * per-subset authoring content the owning W2 packet picks; this only supplies the placeholder shape a
 * fresh surface must start from. Every generated component leaf carries the `SCAFFOLD` marker so
 * `policySubsetSurfaceCompletenessBreaches` (root `📜️script.ts`) can flag scaffold residue distinctly
 * from a genuinely missing surface.
 */
export const SURFACE_SCAFFOLD_TICKET_PATH = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET";

/** @emoji 🚧️ Marker every scaffolded component leaf carries; scanned for by the completeness policy. */
export const SCAFFOLD_MARKER = "SCAFFOLD";

/** @emoji 🎭️ Default mode dir a freshly scaffolded surface gets — mirrors the pre-existing `🎛️apps`
 * convention (`✏️edit` for editors) onto the read-only side (`👁️view` for viewers). Not taxonomy
 * vocabulary (see the region docstring). */
export const SURFACE_DEFAULT_MODE_DIRNAME: Readonly<Record<string, string>> = { viewer: "👁️view", editor: "✏️edit" };

/** @emoji 🪟️ Default window dir a freshly scaffolded mode gets — one obviously-generic placeholder
 * window, replaced by real, per-subset window ids as the owning W2 packet fills the scaffold in. */
export const SURFACE_DEFAULT_WINDOW_DIRNAME = "🪟️main";


/** @emoji 🧹️ Drops every non-ASCII codepoint (emoji + variation selectors) — `"📐️cad"` -> `"cad"` — so a
 * bare CLI id (typed without emoji) can match the real on-disk directory name. Mirrors root
 * `📜️script.ts`'s `policyStripEmoji`; duplicated here because the two scripts are separate bundles with
 * no shared import path for this one-line helper. */
export function surfaceStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}


/** @emoji 🔎️ Resolves a bare CLI id to the real emoji-prefixed child directory name of `parentAbs`. */
export function surfaceResolveChildDir(parentAbs: string, wantStripped: string): string | undefined {
  if (!existsSync(parentAbs)) return undefined;
  for (const name of readdirSync(parentAbs)) {
    if (!statSync(join(parentAbs, name)).isDirectory()) continue;
    if (surfaceStripEmoji(name) === wantStripped) return name;
  }
  return undefined;
}


/** @emoji 🧭️ Resolves `<plugin> <kind> <standard> <subset>` CLI args to the subset's repo-relative path,
 * throwing a precise error naming the failing segment rather than silently creating the wrong tree.
 * `subsetArg === taxonomy.subsetAnyId` (`"*"`) is accepted as an alias for `subsetAnyDirName`
 * (`"✳️any"`), mirroring the taxonomy's own alias. */
export function resolveSubsetRel(repoRoot: string, pluginArg: string, kindArg: string, standardArg: string, subsetArg: string): string {
  let area: string | undefined;
  let pluginDir: string | undefined;
  for (const candidate of PLUGIN_AREAS) {
    const found = surfaceResolveChildDir(join(repoRoot, candidate), pluginArg);
    if (found) {
      area = candidate;
      pluginDir = found;
      break;
    }
  }
  if (!area || !pluginDir) throw new Error(`new surface: no plugin "${pluginArg}" under ${PLUGIN_AREAS.join(", ")}`);
  const artifactsAbs = join(repoRoot, area, pluginDir, TAXONOMY.artifactsDirName);
  const kindDir = surfaceResolveChildDir(artifactsAbs, kindArg);
  if (!kindDir) throw new Error(`new surface: no artifact kind "${kindArg}" under ${area}/${pluginDir}/${TAXONOMY.artifactsDirName}`);
  const standardsAbs = join(artifactsAbs, kindDir, TAXONOMY.standardsDirName);
  const standardDir = surfaceResolveChildDir(standardsAbs, standardArg);
  if (!standardDir) throw new Error(`new surface: no standard "${standardArg}" under .../${kindDir}/${TAXONOMY.standardsDirName}`);
  const subsetsAbs = join(standardsAbs, standardDir, TAXONOMY.subsetsDirName);
  const wantSubset = subsetArg === TAXONOMY.subsetAnyId ? surfaceStripEmoji(TAXONOMY.subsetAnyDirName ?? "") : subsetArg;
  const subsetDir = surfaceResolveChildDir(subsetsAbs, wantSubset);
  if (!subsetDir) throw new Error(`new surface: no subset "${subsetArg}" under .../${standardDir}/${TAXONOMY.subsetsDirName}`);
  return relative(repoRoot, join(subsetsAbs, subsetDir)).replaceAll("\\", "/");
}


/** @emoji 🪆️ Every subset dir across every plugin area whose `🧬️schema` facet is present — the "owned"
 * predicate this ticket freezes (contract §6): schema presence alone, independent of `🚪️io`, because
 * the 286-surface target (143 subsets × 2 roles) only holds when every schema-bearing subset counts,
 * including the one subset (🧩️assembly) that has no `🚪️io` yet. */
export function discoverOwnedSubsetRels(repoRoot: string): string[] {
  const out: string[] = [];
  for (const area of PLUGIN_AREAS) {
    const areaAbs = join(repoRoot, area);
    for (const plugin of listDirs(areaAbs)) {
      const artifactsAbs = join(areaAbs, plugin, TAXONOMY.artifactsDirName);
      for (const kind of listDirs(artifactsAbs)) {
        const standardsAbs = join(artifactsAbs, kind, TAXONOMY.standardsDirName);
        for (const std of listDirs(standardsAbs)) {
          const subsetsAbs = join(standardsAbs, std, TAXONOMY.subsetsDirName);
          for (const sub of listDirs(subsetsAbs)) {
            const subsetAbs = join(subsetsAbs, sub);
            if (!existsSync(join(subsetAbs, SCHEMA_FACET_DIR))) continue;
            out.push(relative(repoRoot, subsetAbs).replaceAll("\\", "/"));
          }
        }
      }
    }
  }
  return out.sort();
}


export function scaffoldRustLeaf(label: string): string {
  return `//! 🚧️ ${SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new surface\`, not implemented.\n//! @see ${SURFACE_SCAFFOLD_TICKET_PATH}\npub const SCAFFOLD: bool = true;\n`;
}


export function scaffoldTsLeaf(label: string): string {
  return `// 🚧️ ${SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new surface\`, not implemented.\n// @see ${SURFACE_SCAFFOLD_TICKET_PATH}\nexport const SCAFFOLD = true;\n`;
}


export function scaffoldEmptyFacetMarkdown(facetLabel: string): string {
  return `# Empty ${surfaceStripEmoji(facetLabel)} Facet\n\nThis facet currently declares no specific items. Authored by \`bun ./📜️script.ts new surface\`.\n`;
}


export function scaffoldLeafFilename(lang: string): string {
  const kindId = TAXONOMY.ecosystems[lang]?.componentFileKindId;
  if (!kindId) throw new Error(`new surface: 🔣️taxonomy.json ecosystems has no component file kind for "${lang}"`);
  return primaryFilenameForKind(kindId);
}


export function scaffoldLeafContentForLang(lang: string, label: string): string {
  return lang === "🦀️rust" ? scaffoldRustLeaf(label) : scaffoldTsLeaf(label);
}


export type SurfaceScaffoldResult = ArtifactScaffoldResult;


/**
 * 🏗️ Creates one surface's full scaffold shape under `<subsetRel>/<viewerDirName|editorDirName>` per
 * the frozen tree (ticket Deliverable A): 2 surface leaves + 4 surface facets + 1 mode leaf + 4 mode
 * facets + 2 window leaves + 6 window facets = 19 files. Idempotent — never overwrites.
 */
export function scaffoldSurfaceTree(repoRoot: string, subsetRel: string, role: string, dryRun: boolean, options: ArtifactScaffoldOptions = {}): SurfaceScaffoldResult {
  const leaves: ArtifactScaffoldLeaf[] = [];
  const add = (path: string, content: string): void => { leaves.push({ path, content }); };
  const surfaceRel = `${subsetRel}/${TAXONOMY.surfaceDirNames[role]}`;
  const label = `${role} surface`;

  for (const lang of TAXONOMY.surfaceComponentLangs) {
    add(`${surfaceRel}/${scaffoldLeafFilename(lang)}`, scaffoldLeafContentForLang(lang, label));
  }
  for (const facet of TAXONOMY.surfaceRequiredChildDirs) {
    if (facet === TAXONOMY.modesDirName) continue;
    add(`${surfaceRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Surface ${facet}`));
  }

  const modeRel = `${surfaceRel}/${TAXONOMY.modesDirName}/${SURFACE_DEFAULT_MODE_DIRNAME[role]}`;
  add(`${modeRel}/${scaffoldLeafFilename("🦀️rust")}`, scaffoldRustLeaf(`${role} mode`));
  for (const facet of TAXONOMY.modeRequiredChildDirs ?? []) {
    if (facet === TAXONOMY.windowsDirName) continue;
    add(`${modeRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Mode ${facet}`));
  }

  const windowRel = `${modeRel}/${TAXONOMY.windowsDirName}/${SURFACE_DEFAULT_WINDOW_DIRNAME}`;
  for (const lang of TAXONOMY.windowLeafLangs) {
    add(`${windowRel}/${scaffoldLeafFilename(lang)}`, scaffoldLeafContentForLang(lang, `${role} window`));
  }
  for (const facet of TAXONOMY.windowRequiredChildDirs) {
    add(`${windowRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Window ${facet}`));
  }

  return authorArtifactScaffold(repoRoot, { kind: "surface", subsetPath: subsetRel, role }, leaves, TAXONOMY, { ...options, dryRun });
}


export function reportSurfaceScaffoldResult(label: string, result: SurfaceScaffoldResult, dryRun: boolean): void {
  const verb = dryRun ? "would create" : "created";
  console.log(`${label}: ${verb} ${result.created.length} file(s), ${result.skipped.length} already present`);
  for (const path of result.created) console.log(`  ${dryRun ? "+ (dry-run)" : "+"} ${path}`);
}


/** @emoji 🌊️ `new surface --all`: walks every owned subset on disk and scaffolds whatever surface is
 * missing, idempotently. Reports surface-granularity totals (subset × role pairs touched) alongside
 * the raw file count, so a dry-run answers "how many of the 286 surfaces still need scaffolding". */
export function runSurfaceScaffoldAll(repoRoot: string, dryRun: boolean): void {
  const subsetRels = discoverOwnedSubsetRels(repoRoot);
  let surfacesTouched = 0;
  let filesCreated = 0;
  let filesSkipped = 0;
  for (const subsetRel of subsetRels) {
    for (const role of TAXONOMY.surfaceRoles) {
      const result = scaffoldSurfaceTree(repoRoot, subsetRel, role, dryRun);
      filesCreated += result.created.length;
      filesSkipped += result.skipped.length;
      if (result.created.length > 0) surfacesTouched += 1;
    }
  }
  const totalSurfaces = subsetRels.length * TAXONOMY.surfaceRoles.length;
  const verb = dryRun ? "would scaffold" : "scaffolded";
  console.log(`new surface --all: ${verb} ${surfacesTouched}/${totalSurfaces} surface(s) across ${subsetRels.length} owned subset(s) (${filesCreated} file(s) ${dryRun ? "would be created" : "created"}, ${filesSkipped} already present).`);
}


/** @emoji 🚪️ `new surface` CLI: single surface (`<plugin> <kind> <standard> <subset> <role>`) or batch
 * (`--all [--dry-run]`). Registered as `bun ./📜️script.ts new surface …` via `ScriptRouter`. */
export class NewScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "surface") {
      console.error("usage: bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> <role>");
      console.error("   or: bun ./📜️script.ts new surface --all [--dry-run]");
      process.exit(1);
    }
    const repoRoot = getWorkspaceRoot();
    const rest = segments.slice(1);
    const dryRun = rest.includes("--dry-run");
    const positional = rest.filter((arg) => arg !== "--dry-run");
    if (positional[0] === "--all") {
      runSurfaceScaffoldAll(repoRoot, dryRun);
      return;
    }
    if (positional.length !== 5) {
      console.error("usage: bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> <role>");
      process.exit(1);
      return;
    }
    const [pluginArg, kindArg, standardArg, subsetArg, roleArg] = positional;
    if (!TAXONOMY.surfaceRoles.includes(roleArg!)) {
      console.error(`new surface: role must be one of ${TAXONOMY.surfaceRoles.join(", ")}, got "${roleArg}"`);
      process.exit(1);
      return;
    }
    let subsetRel: string;
    try {
      subsetRel = resolveSubsetRel(repoRoot, pluginArg!, kindArg!, standardArg!, subsetArg!);
    } catch (error) {
      console.error((error as Error).message);
      process.exit(1);
      return;
    }
    const result = scaffoldSurfaceTree(repoRoot, subsetRel, roleArg!, dryRun);
    reportSurfaceScaffoldResult(`${subsetRel}#${roleArg}`, result, dryRun);
  }
}
