/** 🧼️ Wave 4 plugin consistency policy: clean artifact standard subset mechanism rules and derived config.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { canonicalPrimaryFilenameForKind, loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { discoverPackageProblems } from "../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { checkDerivedConfig } from "../../../⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts";
import { policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, POLICY_TS_COMPONENT_LEAF, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { POLICY_RS_COMPONENT_LEAF } from "../✍️handcrafted-mutation/🟦️.ts";
import { policyListTopLevelSubsetDirs } from "../🪆️subset-conformance/🟦️.ts";

//#region 🔧️PolicyRuleCleanMechanism
/**
 * 🧹 Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, wave W1 (allow-half): seven new
 * report-mode policies sizing the migration backlog the design doc's four rules imply — owner-mounted
 * children, subset isolation, module consumer counts, io exclusivity, io declaration reachability,
 * subset standalone-ness, and the plugin->artifact->standard->subset declaration tree. ALL land at
 * `priority: "medium"` (never `"high"`) — report mode, per the ticket: counted into the cache JSON
 * (`formatBreachReport`'s "full breach set (including non-blocking priorities)"), never failing
 * `runPolicyExit`'s high-priority gate. W6 is the wave that promotes any of these to blocking and
 * deletes the shapes they still tolerate (design.md's debt row D4).
 */

//#region 🔖️ModulePathSlug
/** 🔤️ General module-path slug (design.md §1 "Module path slugs"): strip the emoji prefix, kebab ->
 * snake, a leading digit gets an underscore prefix. Applies to module/artifact/subset dir names. */
function policyModulePathSlug(dirName: string): string {
  const stripped = policyStripEmoji(dirName).replace(/-/g, "_");
  return /^[0-9]/.test(stripped) ? `_${stripped}` : stripped;
}

/** 🔖️ Standard-slug variant (design.md §1): same kebab->snake folding, but prefixed `v` (digit-leading
 * raw slug) or `v_` (letter-leading raw slug) instead of the general rule's bare underscore prefix —
 * `🔖️1`->`v1`, `🔖️1.4`->`v1_4`, `🔖️ap214`->`v_ap214`, `🔖️ecma-376`->`v_ecma_376`. Idempotent: a raw
 * dir already in slug form (confirmed live today: `🧿️semio`'s `🔖️v1`/`🔖️v3`) is returned unchanged
 * instead of double-prefixed (`v_v1`) — verified against the real `standards::v1::` module path. */
function policyStandardModulePathSlug(dirName: string): string {
  const stripped = policyStripEmoji(dirName).replace(/[.\-]/g, "_");
  if (/^v[0-9]/.test(stripped) || /^v_/.test(stripped)) return stripped;
  return /^[0-9]/.test(stripped) ? `v${stripped}` : `v_${stripped}`;
}

/** 🧯️Escapes a literal string for embedding in a `RegExp` constructor. */
function policyEscapeRegExp(literal: string): string {
  return literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
//#endregion 🔖️ModulePathSlug

//#region 🔖️OwnerDiscovery
type PolicySubsetRel = {
  subsetRel: string;
  standardRel: string;
  artRel: string;
  pluginRel: string;
  subsetSlug: string;
  standardSlug: string;
  artSlug: string;
};

/** 🗂️ Splits a `policyListTopLevelSubsetDirs` row into its four owner levels plus the Rust module-path
 * slug (per [[policyModulePathSlug]]/[[policyStandardModulePathSlug]]) each level resolves to. */
function policySplitSubsetRel(subRel: string): PolicySubsetRel {
  const parts = subRel.split("/");
  // ✏️s / 🔌️plugins / <plugin> / 🗿️artifacts / <art> / 🏅️standards / 🔖️<std> / 🪆️subsets / ✳️<sub>
  const artDir = parts[4] ?? "";
  const standardDir = parts[6] ?? "";
  const subsetDir = parts[8] ?? "";
  return {
    subsetRel: subRel,
    standardRel: parts.slice(0, 7).join("/"),
    artRel: parts.slice(0, 5).join("/"),
    pluginRel: parts.slice(0, 3).join("/"),
    subsetSlug: policyModulePathSlug(subsetDir),
    standardSlug: policyStandardModulePathSlug(standardDir),
    artSlug: policyModulePathSlug(artDir),
  };
}

/** 🌳️ Every distinct (plugin, artifact, standard, subset) row discovered under ✏️s/🔌️plugins today. */
function policyAllSubsetSplits(repoRoot: string): PolicySubsetRel[] {
  return policyListTopLevelSubsetDirs(repoRoot).map(policySplitSubsetRel);
}

function policyListArtifactRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.artRel))].sort();
}

function policyListStandardRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.standardRel))].sort();
}

function policyListPluginRels(splits: readonly PolicySubsetRel[]): string[] {
  return [...new Set(splits.map((s) => s.pluginRel))].sort();
}

/** 🎯️ Nearest ancestor "owner root" of a repo-relative path — subset root, else standard root, else
 * artifact root, else the first three path segments (plugin/product root). Used to count DISTINCT
 * consumer roots for [[policyModuleConsumerCountBreaches]]. */
function policyNearestOwnerRoot(relPath: string): string {
  const subsetMatch = relPath.match(/^(.*\/🪆️subsets\/[^/]+)\//);
  if (subsetMatch) return subsetMatch[1]!;
  const standardMatch = relPath.match(/^(.*\/🏅️standards\/[^/]+)\//);
  if (standardMatch) return standardMatch[1]!;
  const artifactMatch = relPath.match(/^(.*\/🗿️artifacts\/[^/]+)\//);
  if (artifactMatch) return artifactMatch[1]!;
  return relPath.split("/").slice(0, 3).join("/");
}

/** 🔎️ Every directory literally named `dirName` under `roots`, not descending past a match (its own
 * children are enumerated by the caller, e.g. [[policyModuleConsumerCountBreaches]]'s module units). */
function policyFindDirsNamed(repoRoot: string, roots: readonly string[], dirName: string): string[] {
  const found: string[] = [];
  const walk = (rel: string): void => {
    for (const e of policyReaddirSafe(repoRoot, rel)) {
      if (!e.isDirectory) continue;
      const childRel = `${rel}/${e.name}`;
      if (e.name === dirName) {
        found.push(childRel);
        continue;
      }
      walk(childRel);
    }
  };
  for (const root of roots) walk(root);
  return found.sort();
}

const POLICY_CLEAN_MECHANISM_SCAN_ROOTS = ["✏️s/🔌️plugins", "✏️s/🔨️modules", "🧰️framework"];
/** 🏛️ Architecture rule over owner-root mounts, subset isolation and module-consumer relationships.
 * Named for what it constrains — the shape of an owner's tree — NOT for removing generated files;
 * that unrelated mechanism is `clean test` in the testing domain. Renamed from the ambiguous
 * "clean-mechanism" in ticket 26/08/23/END-TO-END-TESTING-REFACTOR. */
const POLICY_CLEAN_MECHANISM_KIND = "taxonomy/owner-shape";
//#endregion 🔖️OwnerDiscovery

//#region 🔖️Policy1-OwnerMountsChildren
/** 🧭️ `#[path = "…"]` mount targets declared anywhere in `content` (order of first appearance). */
function policyExtractPathMountTargets(content: string): string[] {
  const targets: string[] = [];
  const re = /#\[path\s*=\s*"([^"]+)"\]/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(content))) targets.push(m[1]!);
  return targets;
}

/** 🧭️ Child dir names of `ownerRel` that are BOTH real on disk AND legal per `allowedChildDirs`. */
function policyExistingLegalChildren(repoRoot: string, ownerRel: string, allowedChildDirs: readonly string[]): string[] {
  return policyReaddirSafe(repoRoot, ownerRel)
    .filter((e) => e.isDirectory && allowedChildDirs.includes(e.name))
    .map((e) => e.name);
}

/**
 * 🦀️ For every artifact/standard/subset root that HAS its own `🦀️.rs`, its own `#[path]`
 * mounts must cover its existing, legal child dirs (design.md §1: "each level one root component.rs
 * that mounts its own children"). Most owners have no root component yet — `missing-owner-root` is
 * the migration backlog counter design.md's recipe (§5 step 6) still has to work through; the ones
 * that DO exist today (confirmed: ~30 artifact roots, one full standard+subset chain under
 * `🗄️stdio/🧊️gltf`) carry zero self-mounts because mounting is still centralized in `🦀️.rs` — the
 * exact shape this ticket dissolves. Plugin `🦀️.rs` may only `#[path]`-mount artifact roots,
 * plugin `🔨️modules`, `🎮️commands`, and its own plugin root; any deeper mount (a standard/subset/
 * schema/io leaf reached directly) is that old centralized-glue shape. `pub use …subsets::`/
 * `…standards::` lines in glue.rs are the sibling "shim" pattern — this scans for the pattern itself,
 * independent of the "Shims: keep pre-migration" text marker `policyNoGlueShimBlocksBreaches` already
 * polices.
 */
function policyOwnerMountsChildrenBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const breaches: BreachRecord[] = [];
  const splits = policyAllSubsetSplits(repoRoot);

  const checkOwnerRoot = (rel: string, level: "artifact" | "standard" | "subset", allowedChildDirs: readonly string[]): void => {
    const rootFile = `${rel}/${POLICY_RS_COMPONENT_LEAF}`;
    if (!existsSync(join(repoRoot, rootFile))) {
      breaches.push({
        id: `owner-mounts-children-missing-root-${rel}`,
        summary: `"${rel}" has no root ${POLICY_RS_COMPONENT_LEAF} yet`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: rel,
        priority: "medium",
        reason: `design.md §1: every ${level} has one root ${POLICY_RS_COMPONENT_LEAF_NAME} that mounts its own children and exports one declaration function — this is the ownership backlog counter.`,
        solution: `bun ./📜️script.ts new ${level} … to scaffold the root skeleton, then hand-author its mounts and declaration fn per the recipe (design.md §5).`,
      });
      return;
    }
    const content = readFileSync(join(repoRoot, rootFile), "utf8");
    const mountCount = policyExtractPathMountTargets(content).length;
    const existingChildren = policyExistingLegalChildren(repoRoot, rel, allowedChildDirs);
    if (existingChildren.length > 0 && mountCount === 0) {
      breaches.push({
        id: `owner-mounts-children-not-self-mounting-${rel}`,
        summary: `"${rootFile}" exists but does not #[path]-mount any of its ${existingChildren.length} child dir(s) (${existingChildren.join(", ")})`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: rel,
        priority: "medium",
        reason: "design.md §1: an owner root mounts its own children — mounting for this owner still lives centralized in the plugin's 🦀️.rs.",
        solution: `Add #[path = "…"] pub mod … mounts for ${existingChildren.join(", ")} directly in ${rootFile}, and remove the equivalent nested mounts from 🦀️.rs.`,
      });
    }
  };

  for (const rel of policyListArtifactRels(splits)) checkOwnerRoot(rel, "artifact", taxonomy.artifactChildDirs);
  for (const rel of policyListStandardRels(splits)) checkOwnerRoot(rel, "standard", taxonomy.standardChildDirs);
  for (const s of splits) checkOwnerRoot(s.subsetRel, "subset", taxonomy.subsetChildDirs);

  for (const pluginRel of policyListPluginRels(splits)) {
    const glueRel = `${pluginRel}/📦️packages/🦀️rust/🦀️.rs`;
    if (!existsSync(join(repoRoot, glueRel))) continue;
    const content = readFileSync(join(repoRoot, glueRel), "utf8");
    const targets = policyExtractPathMountTargets(content);
    const outOfScope = targets.filter((t) => {
      if (t.includes("/🔨️modules/") || t.includes("/🎮️commands/")) return false;
      if (t.endsWith(`/${POLICY_RS_COMPONENT_LEAF_NAME}`) && /\/🗿️artifacts\/[^/]+$/u.test(t.slice(0, -POLICY_RS_COMPONENT_LEAF_NAME.length - 1))) return false;
      const isDeepMount = ["🗿️artifacts/", "🏅️standards/", "🪆️subsets/", "🧬️schema/", "🚪️io/", "👁️viewer/", "✏️editor/"].some((seg) => t.includes(seg));
      if (!isDeepMount && t.endsWith(POLICY_RS_COMPONENT_LEAF)) return false; // the plugin's own root mount
      return true;
    });
    if (outOfScope.length > 0) {
      breaches.push({
        id: `owner-mounts-children-glue-scope-${pluginRel}`,
        summary: `"${glueRel}" #[path]-mounts ${outOfScope.length} item(s) beyond artifact roots/🔨️modules/🎮️commands/plugin root (still centralized)`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: pluginRel,
        priority: "medium",
        reason: "design.md §1: a plugin's 🦀️.rs mounts ONLY artifact roots, plugin modules, commands, and the plugin root — everything below an artifact root is that artifact's own job to mount.",
        solution: `Move each deep mount (first example: "${outOfScope[0]}") down into its owning artifact/standard/subset root's own #[path] list.`,
      });
    }
    const shimHits = [...content.matchAll(/pub use\s+[^;]*::(?:subsets|standards)::[^;]*;/g)];
    if (shimHits.length > 0) {
      breaches.push({
        id: `owner-mounts-children-glue-shim-${pluginRel}`,
        summary: `"${glueRel}" carries ${shimHits.length} "pub use …subsets::"/"…standards::" shim re-export(s)`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/owner-mounts-children`,
        scope: pluginRel,
        priority: "medium",
        reason: "design.md §5 step 9: delete every 🦀️.rs shim that re-exports a sibling subset/standard path — the owning root should export it directly.",
        solution: `Delete the shim re-export(s) (first example: "${shimHits[0]![0]}") once the owning root exports the symbol directly.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy1-OwnerMountsChildren

//#region 🔖️Policy2-SubsetIsolation
/**
 * 🪆️ A file under a subset's own tree must not reach into a SIBLING subset or a DIFFERENT standard of
 * its own artifact (design.md rule 2: "a subset never uses a sibling subset or another standard"),
 * and a TS file must never `import` its way above its own subset root. `modules::`, framework crates
 * (`semio_framework*`), and other plugin crates (`semio_s_plugin_*`) stay legal — only same-artifact
 * cross-standard/cross-subset reach-through and a subset-escaping relative import are flagged.
 */
function policySubsetIsolationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const files = policyWalkRelFiles(repoRoot, [split.subsetRel], (_p, name) => name.endsWith(".rs") || name.endsWith(".ts"));
    const crossStandardRe = new RegExp(`artifacts::${policyEscapeRegExp(split.artSlug)}::standards::(?!${policyEscapeRegExp(split.standardSlug)}\\b)(\\w+)`, "g");
    const crossSubsetRe = new RegExp(`artifacts::${policyEscapeRegExp(split.artSlug)}::standards::${policyEscapeRegExp(split.standardSlug)}::subsets::(?!${policyEscapeRegExp(split.subsetSlug)}\\b)(\\w+)`, "g");
    for (const relPath of files) {
      const content = policyReadFileSafe(repoRoot, relPath);
      if (relPath.endsWith(".rs")) {
        const crossSubsetTargets = new Set<string>();
        for (const m of content.matchAll(crossSubsetRe)) {
          crossSubsetTargets.add(m[1]!);
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-cross-subset-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" reaches into sibling subset "${m[1]}" (own subset is "${split.subsetSlug}")`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2: a subset never uses a sibling subset.",
            solution: `Extract the shared code into a 🔨️modules owner at the lowest common level (design.md rule 4) instead of reaching into subsets::${m[1]}.`,
          });
        }
        for (const m of content.matchAll(crossStandardRe)) {
          if (m[1] === split.standardSlug || crossSubsetTargets.has(m[1]!)) continue; // own standard (post-lookahead false match) or already counted as a cross-subset hit at the same span
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-cross-standard-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" reaches into standard "${m[1]}" of its own artifact "${split.artSlug}" (own standard is "${split.standardSlug}")`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2: a subset never reaches into another standard of its own artifact.",
            solution: `Extract the shared code into a 🔨️modules owner at the lowest common level (design.md rule 4) instead of reaching across standards::${m[1]}.`,
          });
        }
      } else {
        const depth = relPath.slice(split.subsetRel.length + 1).split("/").length - 1;
        for (const m of content.matchAll(/from\s+["'](\.\.[^"']*)["']/g)) {
          const spec = m[1]!;
          const upCount = (spec.match(/\.\.\//g) ?? []).length;
          if (upCount <= depth) continue;
          // 🔨️ design.md rule 2's "Allowed: modules::, framework crates, other plugin crates" applies to
          // TS the same way it does to Rust — resolve the spec against the file's real dir and allow it
          // through when it lands in a 🔨️modules owner, a different plugin's own tree, or 🧰️framework.
          const resolved = join(dirname(join(repoRoot, relPath)), spec).replaceAll("\\", "/");
          const resolvedRel = relative(repoRoot, resolved).replaceAll("\\", "/");
          if (/(^|\/)🔨️modules(\/|$)/.test(resolvedRel) || resolvedRel.startsWith("🧰️framework/") || (resolvedRel.startsWith("✏️s/🔌️plugins/") && !resolvedRel.startsWith(split.pluginRel + "/"))) continue;
          const line = content.slice(0, m.index).split("\n").length;
          breaches.push({
            id: `subset-isolation-ts-climb-${relPath}-${m.index}`,
            summary: `"${relPath}:${line}" imports "${spec}", which climbs above its own subset root "${split.subsetRel}"`,
            kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-isolation`,
            scope: split.subsetRel,
            line,
            priority: "medium",
            reason: "design.md rule 2 (TS half): no relative import may leave its own subset dir, except into a 🔨️modules owner, 🧰️framework, or another plugin.",
            solution: `Replace the relative import with an absolute import, or move the shared code into a 🔨️modules owner (design.md rule 4).`,
          });
        }
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy2-SubsetIsolation

//#region 🔖️Policy3-ModuleConsumerCount
/**
 * 🔨️ Every `🔨️modules/<m>` needs ≥2 distinct consumer roots (design.md rule 4: "≥2 distinct consumer
 * roots ... One consumer ⇒ inline it"); a subset-level `🔨️modules` is ALWAYS a breach — confirmed live
 * today at `🗄️stdio/🧊️gltf/…/✳️any/🔨️modules` (design.md §4's own "gltf subset 🔨️modules/*" debt row).
 */
function policyModuleConsumerCountBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const modulesDirs = policyFindDirsNamed(repoRoot, POLICY_CLEAN_MECHANISM_SCAN_ROOTS, "🔨️modules");
  for (const modulesDirRel of modulesDirs) {
    const isSubsetLevel = /\/🪆️subsets\/[^/]+\/🔨️modules$/.test(modulesDirRel);
    const units = policyReaddirSafe(repoRoot, modulesDirRel).filter((e) => e.isDirectory);
    for (const unit of units) {
      const unitRel = `${modulesDirRel}/${unit.name}`;
      if (isSubsetLevel) {
        breaches.push({
          id: `module-consumer-subset-level-${unitRel}`,
          summary: `"${unitRel}" is a subset-level 🔨️modules — always forbidden`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/module-consumer-count`,
          scope: unitRel,
          priority: "medium",
          reason: "design.md rule 4: no subset-level 🔨️modules; shared code lives at the lowest common owner at or above standard level.",
          solution: `Lift "${unit.name}" to the owning standard's/artifact's 🔨️modules (or inline it if it truly has one consumer).`,
        });
        continue;
      }
      const slug = policyModulePathSlug(unit.name);
      const token = `modules::${slug}`;
      const searchRoots = modulesDirRel.startsWith("✏️s/🔌️plugins/") ? [modulesDirRel.split("/").slice(0, 3).join("/")] : POLICY_CLEAN_MECHANISM_SCAN_ROOTS;
      const matchFiles = policyWalkRelFiles(repoRoot, searchRoots, (relPath, name) => name.endsWith(".rs") && !relPath.startsWith(`${unitRel}/`) && policyReadFileSafe(repoRoot, relPath).includes(token));
      const consumerRoots = new Set(matchFiles.map((f) => policyNearestOwnerRoot(f)));
      if (consumerRoots.size < 2) {
        breaches.push({
          id: `module-consumer-count-${unitRel}`,
          summary: `"${unitRel}" (module path "${token}") has ${consumerRoots.size} distinct consumer root(s), needs >=2`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/module-consumer-count`,
          scope: unitRel,
          priority: "medium",
          reason: "design.md rule 4: a 🔨️modules member needs ≥2 distinct production consumers or it is inlined.",
          solution:
            consumerRoots.size === 0
              ? `"${unit.name}" has no discovered consumer at all — inline it back into its sole intended caller, or delete it if dead.`
              : `"${unit.name}" has exactly one consumer (${[...consumerRoots][0]}) — inline it there, or find/add its second real consumer.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy3-ModuleConsumerCount

//#region 🔖️Policy4-IoExclusivity
const POLICY_IO_EXCLUSIVITY_PATTERNS: readonly { re: RegExp; label: string }[] = [
  { re: /\bparse_dsl\s*\(/, label: "parse_dsl(" },
  { re: /\bprint_dsl\s*\(/, label: "print_dsl(" },
  { re: /\bencode_pack\s*\(/, label: "encode_pack(" },
  { re: /\bdecode_pack\s*\(/, label: "decode_pack(" },
  { re: /\bArtifactDsl::/, label: "ArtifactDsl::" },
  { re: /\bArtifactPack::/, label: "ArtifactPack::" },
  { re: /include_bytes!/, label: "include_bytes!" },
  { re: /\bstd::fs::/, label: "std::fs::" },
  { re: /\bsemio_s_plugin_\w+::[\w:]*\bio\b/, label: "semio_s_plugin_<other>::…::io" },
];

/** ✂️ Strips top-level `#[cfg(test)] mod … { … }` bodies via brace matching, so test-only code never
 * trips the io-exclusivity scan (design.md §3 "Exclusivity … and #[cfg(test)]"). */
function policyStripCfgTestModules(content: string): string {
  const marker = "#[cfg(test)]";
  let out = "";
  let i = 0;
  while (i < content.length) {
    const idx = content.indexOf(marker, i);
    if (idx === -1) {
      out += content.slice(i);
      break;
    }
    out += content.slice(i, idx);
    const braceStart = content.indexOf("{", idx);
    if (braceStart === -1) {
      out += content.slice(idx);
      break;
    }
    let depth = 0;
    let j = braceStart;
    for (; j < content.length; j++) {
      if (content[j] === "{") depth++;
      else if (content[j] === "}") {
        depth--;
        if (depth === 0) {
          j++;
          break;
        }
      }
    }
    i = j;
  }
  return out;
}

/**
 * 🚪️ Outside `🚪️io/**` and `#[cfg(test)]`/`🧪️tests`, native codec/raw-fs primitives are policy
 * breaches — design.md §3 "Exclusivity". `serde_json` is explicitly NOT banned (it is the UI/command
 * protocol — 741 legitimate call sites per the ticket).
 */
function policyIoExclusivityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, POLICY_CLEAN_MECHANISM_SCAN_ROOTS, (relPath, name) => name.endsWith(".rs") && !relPath.includes("/🚪️io/") && !relPath.includes("/🧪️tests/"));
  for (const relPath of files) {
    const content = policyStripCfgTestModules(policyReadFileSafe(repoRoot, relPath));
    for (const { re, label } of POLICY_IO_EXCLUSIVITY_PATTERNS) {
      const m = re.exec(content);
      if (!m) continue;
      const line = content.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `io-exclusivity-${relPath}-${label}`,
        summary: `"${relPath}:${line}" uses "${label}" outside 🚪️io/**`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-exclusivity`,
        scope: relPath,
        line,
        priority: "medium",
        reason: "design.md §3: all IO — including the native DSL grammar and pack binary protocol — goes exclusively through the io system's Serializer/Deserializer entries.",
        solution: `Move this call behind an io_route()/io_run() call, or relocate the codec logic under 🚪️io/.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy4-IoExclusivity

//#region 🔖️Policy5-IoDeclaration
/**
 * 🚪️ Every `🚪️io` codec leaf dir (one carrying its own `🦀️.rs`) must be referenced BY NAME
 * from its subset's `🚪️io/🦀️.rs` root, and carry a `🟦️.ts` twin (design.md §1:
 * "io() -> IoDeclaration / IoEntryDescriptor[] mirror").
 */
function policyIoDeclarationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const ioRel = `${split.subsetRel}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRel))) continue;
    const ioRootFile = `${ioRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const ioRootContent = policyReadFileSafe(repoRoot, ioRootFile);
    const leafFiles = policyWalkRelFiles(repoRoot, [ioRel], (relPath, name) => name === POLICY_RS_COMPONENT_LEAF && relPath !== ioRootFile);
    for (const leafFile of leafFiles) {
      const leafDirRel = leafFile.slice(0, leafFile.length - `/${POLICY_RS_COMPONENT_LEAF}`.length);
      const leafName = leafDirRel.split("/").pop() ?? "";
      if (!ioRootContent.includes(leafName)) {
        breaches.push({
          id: `io-declaration-unreachable-${leafFile}`,
          summary: `"${leafDirRel}" is not referenced by name from "${ioRootFile}"`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-declaration`,
          scope: split.subsetRel,
          priority: "medium",
          reason: "design.md §1: io() -> IoDeclaration must reach every codec leaf; an unreferenced leaf is dead weight the io registry never sees.",
          solution: `Mount/reference "${leafName}" from ${ioRootFile}, or delete the leaf if it is genuinely unused.`,
        });
      }
      if (!existsSync(join(repoRoot, leafDirRel, (POLICY_TS_COMPONENT_LEAF)))) {
        breaches.push({
          id: `io-declaration-missing-ts-twin-${leafDirRel}`,
          summary: `"${leafDirRel}" has no ${POLICY_TS_COMPONENT_LEAF} twin`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/io-declaration`,
          scope: split.subsetRel,
          priority: "medium",
          reason: ("design.md §1: 🚪️io/" + POLICY_TS_COMPONENT_LEAF + " exports the IoEntryDescriptor[] mirror — every Rust codec leaf needs its TS twin."),
          solution: `Add ${leafDirRel}/${POLICY_TS_COMPONENT_LEAF} mirroring the Rust leaf's IoEntryDescriptor row(s).`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy5-IoDeclaration

//#region 🔖️Policy6-SubsetStandalone
/**
 * 🪆️ A subset's `🧬️schema/🦀️component.rs` must not be a bare `pub use …subsets::<other>::*`
 * re-export of a sibling subset, and the subset's `🧬️schema/**` subtree must declare its OWN
 * `…Snapshot` struct somewhere (design.md rule 2: "own snapshot/diff/mutations/inferences types").
 */
function policySubsetStandaloneBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const split of policyAllSubsetSplits(repoRoot)) {
    const schemaRootFile = `${split.subsetRel}/🧬️schema/${POLICY_RS_COMPONENT_LEAF}`;
    if (!existsSync(join(repoRoot, schemaRootFile))) continue;
    const rootContent = readFileSync(join(repoRoot, schemaRootFile), "utf8");
    for (const m of rootContent.matchAll(/pub use\s+[\w:]*subsets::(\w+)[\w:]*::\*/g)) {
      const other = m[1]!;
      if (other === split.subsetSlug) continue;
      const line = rootContent.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `subset-standalone-bare-reexport-${schemaRootFile}-${m.index}`,
        summary: `"${schemaRootFile}:${line}" is a bare re-export of sibling subset "${other}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-standalone`,
        scope: split.subsetRel,
        line,
        priority: "medium",
        reason: "design.md rule 2: every subset is a complete standalone implementation, never a bare re-export of another subset's schema.",
        solution: `Give "${split.subsetSlug}" its own snapshot/diff/mutation types (built from shared 🔨️modules types where genuinely shared), instead of re-exporting subsets::${other}::*.`,
      });
    }
    const schemaFiles = policyWalkRelFiles(repoRoot, [`${split.subsetRel}/🧬️schema`], (_p, name) => name.endsWith(".rs"));
    const hasOwnSnapshot = schemaFiles.some((f) => /(?:^|[^\w])struct\s+\w*Snapshot\b/.test(policyReadFileSafe(repoRoot, f)));
    if (!hasOwnSnapshot) {
      breaches.push({
        id: `subset-standalone-no-own-snapshot-${split.subsetRel}`,
        summary: `"${split.subsetRel}/🧬️schema" declares no own "…Snapshot" struct`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/subset-standalone`,
        scope: split.subsetRel,
        priority: "medium",
        reason: "design.md rule 2: every subset owns its own snapshot type.",
        solution: `Define a "…Snapshot" struct somewhere under ${split.subsetRel}/🧬️schema, or if genuinely derived, a Deserializer FROM the base dialect (design.md §5 step 4) rather than a bare re-export.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔖️Policy6-SubsetStandalone

//#region 🔖️Policy7-DeclarationTree
/**
 * 🌳️ Where the NEW-shape declaration functions exist (design.md §1: `pub fn artifact() ->
 * ArtifactDeclaration`, `pub fn standard() -> StandardDeclaration`, `pub fn subset() ->
 * SubsetDeclaration`), the parent level must register every child that owns one, matching
 * `🪆️subsets/🔣️.json`. Dormant on today's tree — a repo-wide grep confirmed these exact
 * zero-arg signatures do not exist anywhere yet (the SDK's `ArtifactDeclaration` today is still the
 * OLD `ArtifactDeclarationBuilder` shape, per `🔌️plugin/🦀️.rs`) — verified structurally sound
 * so it fires correctly once a W2+ packet lands the first real declaration tree, the same precedent
 * `policyContributedSurfaceTargetBreaches` already set for a dormant-but-ready rule.
 */
function policyDeclarationTreeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const splits = policyAllSubsetSplits(repoRoot);
  for (const split of splits) {
    const artFile = `${split.artRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const stdFile = `${split.standardRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const subFile = `${split.subsetRel}/${POLICY_RS_COMPONENT_LEAF}`;
    const artContent = policyReadFileSafe(repoRoot, artFile);
    const stdContent = policyReadFileSafe(repoRoot, stdFile);
    const subContent = policyReadFileSafe(repoRoot, subFile);
    if (/pub fn artifact\(\)\s*->\s*ArtifactDeclaration/.test(artContent) && /pub fn standard\(\)\s*->\s*StandardDeclaration/.test(stdContent) && !artContent.includes(split.standardSlug)) {
      breaches.push({
        id: `declaration-tree-artifact-missing-standard-${split.artRel}-${split.standardSlug}`,
        summary: `"${artFile}" declares artifact() but never references standard "${split.standardSlug}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
        scope: split.artRel,
        priority: "medium",
        reason: "design.md §1: artifact() registers every standard it owns.",
        solution: `Add "${split.standardSlug}"'s standard() to the artifact()'s .standards vec in ${artFile}.`,
      });
    }
    if (/pub fn standard\(\)\s*->\s*StandardDeclaration/.test(stdContent) && /pub fn subset\(\)\s*->\s*SubsetDeclaration/.test(subContent) && !stdContent.includes(split.subsetSlug)) {
      breaches.push({
        id: `declaration-tree-standard-missing-subset-${split.standardRel}-${split.subsetSlug}`,
        summary: `"${stdFile}" declares standard() but never references subset "${split.subsetSlug}"`,
        kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
        scope: split.standardRel,
        priority: "medium",
        reason: ("design.md §1: standard() registers every subset it owns, matching 🪆️subsets/" + canonicalPrimaryFilenameForKind(loadTaxonomy().semanticManifestFileKindId) + "."),
        solution: `Add "${split.subsetSlug}"'s subset() to the standard()'s .subsets vec in ${stdFile}.`,
      });
    }
  }
  for (const pluginRel of policyListPluginRels(splits)) {
    const glueRel = `${pluginRel}/📦️packages/🦀️rust/🦀️.rs`;
    const glueContent = policyReadFileSafe(repoRoot, glueRel);
    if (!glueContent) continue;
    for (const artRel of policyListArtifactRels(splits.filter((s) => s.pluginRel === pluginRel))) {
      const artContent = policyReadFileSafe(repoRoot, `${artRel}/${POLICY_RS_COMPONENT_LEAF}`);
      if (!/pub fn artifact\(\)\s*->\s*ArtifactDeclaration/.test(artContent)) continue;
      const artSlug = policyModulePathSlug(artRel.split("/").pop() ?? "");
      if (!glueContent.includes(".artifact(") || !glueContent.includes(artSlug)) {
        breaches.push({
          id: `declaration-tree-plugin-missing-artifact-${pluginRel}-${artSlug}`,
          summary: `"${glueRel}" does not register artifact "${artSlug}" via .artifact(…)`,
          kind: `${POLICY_CLEAN_MECHANISM_KIND}/declaration-tree`,
          scope: pluginRel,
          priority: "medium",
          reason: "design.md §2: PluginBuilder<Ready>.artifact(...) is the ONLY registration channel.",
          solution: `Call .artifact(${artSlug}::artifact()) from ${glueRel}'s Plugin::builder chain.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔖️Policy7-DeclarationTree

/** ⚖️ Aggregates the seven ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1 report-mode
 * policies (task 2) — ALL land at `"medium"` priority; see the region docstring above. */
export function policyCleanArtifactStandardSubsetMechanismBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyOwnerMountsChildrenBreaches(repoRoot),
    ...policySubsetIsolationBreaches(repoRoot),
    ...policyModuleConsumerCountBreaches(repoRoot),
    ...policyIoExclusivityBreaches(repoRoot),
    ...policyIoDeclarationBreaches(repoRoot),
    ...policySubsetStandaloneBreaches(repoRoot),
    ...policyDeclarationTreeBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleCleanMechanism

/** 📦️ Shape V2 package-folder purity — language-neutral assets must not live inside `📦️packages/<lang>/`. */
const POLICY_PACKAGE_PURITY_PRIORITY = "high" as const;

export function policyPackageLanguagePurityBreaches(repoRoot: string): BreachRecord[] {
  const problems = discoverPackageProblems(repoRoot, loadTaxonomy());
  return problems
    .filter((problem) => problem.kind === "packaging-violation" || problem.kind === "unknown-lang" || problem.kind === "package-implementation" || problem.kind === "package-role-unresolved")
    .map((problem) => {
      const implementation = problem.kind === "package-implementation", unresolved = problem.kind === "package-role-unresolved";
      return {
        id: `package-purity-${problem.path.replaceAll("/", "-")}`,
        summary: problem.message,
        kind: implementation ? "taxonomy/package-body-ownership" : unresolved ? "taxonomy/package-body-unresolved" : "taxonomy/package-purity",
        scope: problem.path,
        priority: POLICY_PACKAGE_PURITY_PRIORITY,
        reason: implementation
          ? "Language packages contain manifests and minimal compiler or tool glue; authored domain declarations and computation belong to semantic owners."
          : unresolved
            ? "Unsupported or ambiguous source syntax cannot establish that a package leaf is minimal compiler or tool glue."
            : "Shape V2 package boundaries contain declared language packaging assets only.",
        solution: implementation
          ? "Extract the owned body to an anonymous implementation leaf under its semantic owner and leave only the required package adapter."
          : unresolved
            ? "Classify the syntax with parser-backed evidence, then extract owned behavior or reduce the leaf to declared glue."
            : "Move language-neutral or undeclared assets to the registered semantic owner outside the package boundary.",
      };
    });
}

//#region 🔧️PolicyRuleDerivedConfig
/** 📝️ Derived-config rule: the devcontainer editor block, the nextest scopes and the dependabot directories equal their derivation.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts */
export function policyDerivedConfigBreaches(repoRoot: string): BreachRecord[] {
  return checkDerivedConfig(repoRoot).map((problem) => ({
    id: `derived-config-${problem}`,
    summary: problem,
    kind: "config/derived-drift",
    scope: /^[^:\s]+/.exec(problem)?.[0] ?? problem,
    priority: "high" as const,
    reason: "A checked-in file that derives from one canonical source must equal its derivation, otherwise two copies drift and the editor, container and test runners disagree.",
    solution: "Run `bun repo run workspace:generate-config` to rewrite the derived files; edit the canonical source (.vscode/settings.json, .vscode/extensions.json, .devcontainer/editor-overlay.json, .config/nextest.toml) instead of the copy.",
  }));
}
//#endregion 🔧️PolicyRuleDerivedConfig
