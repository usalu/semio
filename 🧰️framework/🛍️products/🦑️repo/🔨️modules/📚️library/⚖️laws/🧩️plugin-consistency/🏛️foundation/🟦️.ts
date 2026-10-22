/** 🏛️ Wave 4 plugin consistency policy: crate discovery, region and function parsing.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type PolicyModSpan, policyMaskLiterals } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { discoverPackages } from "../../../🔍️discovery/🔭️repository-scan/🟦️.ts";
import { loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { type PackageRole } from "../../../🔍️discovery/🪧️owner-markers/🟦️.ts";
import { POLICY_SKIP_DIRS } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME, policyStripEmoji } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { join, relative, resolve } from "node:path";
import { taxonomyEcosystemEntryFilenames } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

//#region 🔖️Policy
/**
 * ⚖️ Wave 4 app-plugin consistency policy — the machine-checkable subset of the Wave 4 V1 (duplication),
 * V2 (structure), V3 (coupling) audit findings under `.🧬semio/🦑️repo/🎫️tickets/26/07/18/WAVE-4-*-AUDIT`, wired via
 * the repo build inference into the synthetic `breach-script_ts` lint task (`repo ./📜️script.ts policy`).
 * Judgment-call findings (a real SDK/primitive gap, e.g. the terminology native/reuse Labels axis, or
 * puzzle's icon-based `tree_item_with_action`) are encoded as explicit low-priority allowlisted/tracked
 * breaches, never as a hard `policy` failure — see `POLICY_SDK_GAP_ALLOWLIST` below.
 */

//#region 🔧️PolicyFsScan

/** 🔀️Concept renames that must resolve to the SAME canonical component id on both sides of the taxonomy migration (see the master ticket's discovery contract: `📡️protocol` -> `📡️spr`). */
const POLICY_COMPONENT_ALIASES: Record<string, string> = { protocol: "spr" };

function policyCanonicalComponent(segment: string): string {
  const ascii = policyStripEmoji(segment);
  return POLICY_COMPONENT_ALIASES[ascii] ?? ascii;
}

/** 💾️Memoized `policyPluginOwnerDirs` result per repo root — one lint run asks for it once per crate. */
const policyPluginOwnerCache = new Map<string, readonly string[]>();

/**
 * 🗂️Owner dirs of every discovered `role = "plugin"` package, longest first — the discovery-driven
 * replacement for the old hardcoded `✏️s/🔌️plugins` prefix constant (mechanism step M4): "which plugin
 * owns this path" is answered by the shared package catalog (`🔣️taxonomy.json` + `🟦️discovery.ts`)
 * instead of a path literal this file has to keep in sync. Nested non-plugin owners (trinity's
 * `role = "tool"` jack shell/lsp packages) are deliberately excluded so they keep resolving to their
 * enclosing plugin's scope key, exactly as the prefix form did.
 */
function policyPluginOwnerDirs(repoRoot: string): readonly string[] {
  const cached = policyPluginOwnerCache.get(repoRoot);
  if (cached) return cached;
  const pluginOwners = discoverPackages(repoRoot).flatMap((pkg) => (pkg.role === "plugin" ? [pkg.ownerRel] : []));
  const owners = [...new Set(pluginOwners)].sort((a, b) => b.length - a.length);
  policyPluginOwnerCache.set(repoRoot, owners);
  return owners;
}

/**
 * 🔑Canonical `<pluginId>` scope key for a path inside a discovered plugin owner — the ASCII plugin slug,
 * stable across crate moves, so allowlists keyed off it survive them untouched (see the master ticket's discovery contract).
 * Returns `""` for a path outside every plugin owner (framework and hub trees keep their own scope
 * identity — this function is deliberately plugin-scoped, matching every allowlist that uses it).
 */
export function policyScopeKey(repoRoot: string, relPath: string): string {
  const norm = relPath.replaceAll("\\", "/");
  for (const owner of policyPluginOwnerDirs(repoRoot)) {
    if (norm === owner || norm.startsWith(`${owner}/`)) return policyStripEmoji(owner.split("/").pop() ?? "");
  }
  return "";
}

/** 🔑The `🎛️apps/<app>` id inside a crate path (ASCII, e.g. `"3d"`, `"5d"`), or `""` when the crate isn't nested under an app dir (plugin-root module/manifest crates). Used as the disambiguator for allowlist keys where the app id itself (not an enclosing `pub mod`, which no longer exists now each app is its own crate) is what distinguishes sibling crates of a multi-app plugin. */
export function policyAppIdFromCrateDir(cratePath: string): string {
  const segments = cratePath.split("/");
  const appsIdx = segments.indexOf("🎛️apps");
  return appsIdx >= 0 && segments.length > appsIdx + 1 ? policyStripEmoji(segments[appsIdx + 1]!) : "";
}

/** 🧵Non-default file inside a crate/component dir (e.g. `🐞️benches/🦀️🟪️protocol.rs`) stays distinguishable from its crate's default entry file (`🦀️.rs` package glue, `🦀️.rs` domain leaf) instead of collapsing onto the same key — dropped extension, emoji-stripped, dash-joined. */
function policyFileSuffix(tailSegments: readonly string[], defaultFile: string): string {
  if (tailSegments.length === 1 && tailSegments[0] === defaultFile) return "";
  return `#${tailSegments.map((s) => policyStripEmoji(s.replace(/\.rs$/, ""))).join("-")}`;
}

/**
 * 🔑Canonical `<pluginId>/<component>` key for a *source file path* under `✏️s/🔌️plugins/` — maps the taxonomy
 * layout (`…/🗿️artifacts/<artifact>/<module>/🦀️.rs`) onto a string that stays stable when a component moves. Widens to
 * `<pluginId>/<appOrArtifact>/<component>` only when that disambiguator differs from the plugin id
 * itself (most plugins are single-app/single-artifact, where it doesn't — the 2-part key is the common
 * case, matching the discovery contract's `<pluginId>/<component>` shape). Falls back to the path
 * unchanged when no recognized owner/component shape is found (for example, a path this wave's rename
 * does not touch).
 */
export function policyNormalizeRelPath(relPath: string): string {
  const norm = relPath.startsWith(".") ? relPath.slice(2) : relPath;
  const segments = norm.split("/");

  const artifactsIdx = segments.indexOf("🗿️artifacts");
  if (artifactsIdx > 0 && segments.length > artifactsIdx + 2) {
    const pluginId = policyStripEmoji(segments[artifactsIdx - 1] ?? "");
    const artifactId = policyStripEmoji(segments[artifactsIdx + 1] ?? "");
    const component = policyCanonicalComponent(segments[artifactsIdx + 2]!);
    const suffix = policyFileSuffix(segments.slice(artifactsIdx + 3), (POLICY_RS_COMPONENT_LEAF_NAME));
    return (artifactId && artifactId !== pluginId ? `${pluginId}/${artifactId}/${component}` : `${pluginId}/${component}`) + suffix;
  }

  return norm;
}

/**
 * 🏗️One discovered rust crate. `dir` is its `Cargo.toml`-containing directory (repo-relative),
 * `libRelPath` its crate-root source file as the manifest itself declares it (`[lib]`/`[[bin]]`
 * `path`) — never assumed. `role`/`ownerRel` come straight from the shared discovery contract;
 * `pluginId` is `""` for every crate outside a plugin owner (framework/hub/s-modules).
 */
export type PolicyCrateRef = {
  dir: string;
  libRelPath: string;
  role: PackageRole | "";
  ownerRel: string;
  pluginId: string;
};

/** 🚦️Priority floor for a rule firing on newly-visible surface: framework/hub/s-module crates entered these rules' field of view only with mechanism step M4, so their findings land as tracked (`"low"`) until a dedicated sweep triages them — plugin findings keep the priority the rule already used. */
export function policyNewSurfacePriority(crate: PolicyCrateRef, pluginPriority: BreachRecord["priority"]): BreachRecord["priority"] {
  return crate.pluginId ? pluginPriority : "low";
}

/** 🚪️Crate-root source file a `Cargo.toml` declares, resolved repo-relative: reads the `[lib]`/`[[bin]]` `path` key, falling back to the vocabulary's rust entry filenames beside the manifest and then at the owner root. */
function policyCrateEntryPath(repoRoot: string, manifestDirRel: string, ownerRel: string): string {
  const manifestAbs = join(repoRoot, manifestDirRel, "Cargo.toml");
  if (existsSync(manifestAbs)) {
    const lines = readFileSync(manifestAbs, "utf8").split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      if (!/^\s*\[\[?(?:lib|bin)\]?\]\s*$/.test(lines[i]!)) continue;
      for (let j = i + 1; j < lines.length && !/^\s*\[/.test(lines[j]!); j++) {
        const declared = lines[j]!.match(/^\s*path\s*=\s*"([^"]+)"\s*$/)?.[1];
        if (declared) return relative(repoRoot, resolve(join(repoRoot, manifestDirRel), declared)).replaceAll("\\", "/");
      }
    }
  }
  for (const entry of taxonomyEcosystemEntryFilenames(loadTaxonomy(), "🦀️rust")) {
    if (existsSync(join(repoRoot, manifestDirRel, entry))) return `${manifestDirRel}/${entry}`;
    if (existsSync(join(repoRoot, ownerRel, entry))) return `${ownerRel}/${entry}`;
  }
  throw new Error(`[taxonomy] no declared Rust package entry exists below ${manifestDirRel}.`);
}

/**
 * 🔎️Discovers every rust crate these rules can see, driven by the shared package catalog
 * (`discoverPackages` over `🔣️taxonomy.json`) rather than by a hardcoded area path: every
 * `📦️packages/🦀️rust[/🎯️targets/<target>]/Cargo.toml` carrying a semio role marker — plugins and
 * framework/hub/s-module/tool packages alike; an owner that has not adopted `📦️packages` stays out of scope.
 */
export function policyDiscoverCrateDirs(repoRoot: string): PolicyCrateRef[] {
  const taxonomy = loadTaxonomy();
  const found = new Map<string, PolicyCrateRef>();

  for (const pkg of discoverPackages(repoRoot, taxonomy)) {
    if (pkg.lang !== "🦀️rust") continue;
    found.set(pkg.packageRel, {
      dir: pkg.packageRel,
      libRelPath: policyCrateEntryPath(repoRoot, pkg.packageRel, pkg.ownerRel),
      role: pkg.role,
      ownerRel: pkg.ownerRel,
      pluginId: policyScopeKey(repoRoot, pkg.ownerRel),
    });
  }

  return [...found.values()].sort((a, b) => a.dir.localeCompare(b.dir));
}
/** 🗂️ Directories whose `🟦️.ts` modules hold the logic the root `📜️script.ts` routes to, in router order. */
export const POLICY_ROOT_SCRIPT_OWNER_DIRECTORIES: readonly string[] = [
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🔬️interactivity-audit",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧪️self-test-lanes",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗄️stdio-ledger",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔣️taxonomy/🏷️filenames",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🦀️rust-evidence",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/📍️checkout-root",
];

/** 🗂️ Directories whose `🟦️.ts` modules hold the logic a package `📜️script.ts` routes to (the script itself is found by the repo-wide `📜️script.ts` walk). */
export const POLICY_PACKAGE_SCRIPT_OWNER_DIRECTORIES: readonly string[] = [
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands",
];

function policyOwnedModulePaths(repoRoot: string, directories: readonly string[]): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    let entries: Dirent[];
    try {
      entries = readdirSync(join(repoRoot, relDir), { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = `${relDir}/${ent.name}`;
      if (ent.isDirectory()) {
        if (!POLICY_SKIP_DIRS.has(ent.name) && ent.name !== "🧪️tests") walk(childRel);
        continue;
      }
      if (ent.name === "🟦️.ts") found.push(childRel);
    }
  };
  for (const directory of directories) walk(directory);
  return found.sort();
}

/** 📋️ Repo-relative paths of the root `📜️script.ts` and of every module it owns (`🧪️tests` below an owner directory are not part of it). */
export function policyRootScriptModulePaths(repoRoot: string): string[] {
  return ["📜️script.ts", ...policyOwnedModulePaths(repoRoot, POLICY_ROOT_SCRIPT_OWNER_DIRECTORIES)];
}

/** 📋️ Repo-relative paths of the modules owned by package scripts (`POLICY_PACKAGE_SCRIPT_OWNER_DIRECTORIES`). */
export function policyPackageScriptModulePaths(repoRoot: string): string[] {
  return policyOwnedModulePaths(repoRoot, POLICY_PACKAGE_SCRIPT_OWNER_DIRECTORIES);
}
/** 🪧️ True for the root `📜️script.ts` and for every module it owns: their policy literals name the vocabulary the rules ban, so scans of the rule vocabulary skip them. */
export function policyIsRootScriptSource(relPath: string): boolean {
  return relPath === "📜️script.ts" || POLICY_ROOT_SCRIPT_OWNER_DIRECTORIES.some((directory) => relPath.startsWith(`${directory}/`));
}
//#endregion 🔧️PolicyFsScan

//#region 🔧️PolicyRegionParsing
type PolicyRegionEvent = { kind: "open" | "close"; line: number; spaceAfterSlashes: boolean; label: string };
type PolicyRegionSpan = { label: string; closeLabel: string; startLine: number; endLine: number };


const POLICY_REGION_OPEN_RE = /^(\s*)\/\/(\s*)#region(?:\s+(.*))?\s*$/;
const POLICY_REGION_CLOSE_RE = /^(\s*)\/\/(\s*)#endregion(?:\s+(.*))?\s*$/;

export function policyParseRegionEvents(lines: readonly string[]): PolicyRegionEvent[] {
  const events: PolicyRegionEvent[] = [];
  lines.forEach((line, i) => {
    const om = line.match(POLICY_REGION_OPEN_RE);
    if (om) {
      events.push({ kind: "open", line: i + 1, spaceAfterSlashes: (om[2] ?? "").length > 0, label: (om[3] ?? "").trim() });
      return;
    }
    const cm = line.match(POLICY_REGION_CLOSE_RE);
    if (cm) events.push({ kind: "close", line: i + 1, spaceAfterSlashes: (cm[2] ?? "").length > 0, label: (cm[3] ?? "").trim() });
  });
  return events;
}

export function policyPairRegionSpans(events: readonly PolicyRegionEvent[]): PolicyRegionSpan[] {
  const stack: PolicyRegionEvent[] = [];
  const spans: PolicyRegionSpan[] = [];
  for (const ev of events) {
    if (ev.kind === "open") {
      stack.push(ev);
      continue;
    }
    const open = stack.pop();
    if (!open) continue;
    spans.push({ label: open.label, closeLabel: ev.label, startLine: open.line, endLine: ev.line });
  }
  return spans;
}



const POLICY_MOD_OPEN_RE = /^\s*pub mod (\w+)\b.*\{\s*$/;

export function policyParseModSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  const stack: { name: string; startLine: number; depth: number }[] = [];
  let depth = 0;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    const modMatch = raw.match(POLICY_MOD_OPEN_RE);
    if (modMatch) stack.push({ name: modMatch[1]!, startLine: i + 1, depth });
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    while (stack.length > 0 && depth <= stack[stack.length - 1]!.depth) {
      const top = stack.pop()!;
      spans.push({ name: top.name, startLine: top.startLine, endLine: i + 1 });
    }
  });
  return spans;
}

export function policyModAtLine(modSpans: readonly PolicyModSpan[], lineNo: number): string {
  const containing = modSpans.filter((s) => s.startLine <= lineNo && lineNo <= s.endLine);
  return containing.sort((a, b) => a.endLine - a.startLine - (b.endLine - b.startLine))[0]?.name ?? "";
}





/** 🏷️Strips a leading non-letter (emoji/sigil) prefix off a region label, e.g. "🔖️Tests" -> "Tests". */
export function policyLabelName(label: string): string {
  return label.replace(/^[^\p{L}]+/u, "").trim();
}
//#endregion 🔧️PolicyRegionParsing

//#region 🔧️PolicyFnParsing
/** 🧱️Extracts a `{ … }` function body starting the brace-scan at/after `fromIdx`, string-literal-safe. */
export function policyExtractFnBody(content: string, fromIdx: number): string {
  const masked = policyMaskLiterals(content);
  const openIdx = masked.indexOf("{", fromIdx);
  if (openIdx === -1) return "";
  let depth = 0;
  for (let i = openIdx; i < masked.length; i++) {
    if (masked[i] === "{") depth++;
    else if (masked[i] === "}") {
      depth--;
      if (depth === 0) return content.slice(openIdx, i + 1);
    }
  }
  return content.slice(openIdx);
}

/** 🐫️PascalCase(app id) + "App", e.g. "gis2d-play" -> "Gis2dPlayApp". */
export function policyPascalAppStructName(id: string): string {
  const parts = id.split(/[-_]+/).filter(Boolean);
  return `${parts.map((p) => p.charAt(0).toUpperCase() + p.slice(1)).join("")}App`;
}
//#endregion 🔧️PolicyFnParsing

/** 🔎️Repo-wide `*.rs` file paths (repo-relative), skipping the same dirs `policyDiscoverCrateDirs` skips. */
export function policyAllRustFiles(repoRoot: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: Dirent[];
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = relDir ? `${relDir}/${ent.name}` : ent.name;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name)) continue;
        walk(childRel);
        continue;
      }
      if (ent.name.endsWith(".rs")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}
//#endregion 🔖️Policy
