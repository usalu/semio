/** 🧱️ Wave 4 plugin consistency policy: cargo artifacts, app coupling, JSON fixtures and ops grammar rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { join, relative, resolve } from "node:path";
import { POLICY_JSON_FIXTURE_ALLOWLIST, POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST, POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST } from "../📋️allowlists/🟦️.ts";
import { type PolicyCrateRef, policyNewSurfacePriority, policyScopeKey } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleCargoArtifacts
/**
 * 📏️V2 rule: no stray `Cargo.lock` checked into a discovered crate dir (the workspace root owns the
 * single lockfile; a nested one is the classic leftover of an isolated verification overlay).
 * `target/` is deliberately NOT checked: the repo-wide `.gitignore` `target/` entry makes an on-disk
 * `target/` a local build product that can never be "checked into" anything, so flagging it would turn
 * every developer's own `cargo build` into a high-priority breach.
 */
export function policyCargoArtifactBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    if (!existsSync(join(repoRoot, crate.dir, "Cargo.lock"))) continue;
    breaches.push({
      id: `stray-cargo-artifact-${crate.dir}-Cargo.lock`,
      summary: `Stray "Cargo.lock" checked into ${crate.dir}/`,
      kind: "app-plugin/stray-cargo-artifact",
      scope: crate.dir,
      priority: policyNewSurfacePriority(crate, "high"),
      reason: "Wave 4 V2 structure audit: crates must not carry their own Cargo.lock (workspace-managed) — a nested one silently pins a second dependency graph.",
      solution: `Remove ${crate.dir}/Cargo.lock (leftover of an isolated verification overlay).`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleCargoArtifacts

//#region 🔧️PolicyRuleAppCoupling
const POLICY_CARGO_DEP_RE = /^([\w.-]+)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$/gm;

/**
 * 📏️V3 rule: a plugin crate's path-dependencies must not reach into another plugin's crate (blocking)
 * or undocumented domain path (tracked). "Always allowed" used to be a hand-maintained ASCII prefix
 * list (`framework/`, `vcs/`, `protocol/`, …) that never matched this repo's real emoji-prefixed
 * directories (dead in the same way the discoverer below it was) — replaced with the structural check
 * that actually expresses the rule's intent: coupling is only a concern for dependencies that resolve
 * *inside* a discovered plugin owner (`policyScopeKey` returns `""` for anything else —
 * framework, hub, and repository infrastructure is always allowed by construction, with no prefix list to keep in sync).
 */
export function policyAppCouplingBreaches(repoRoot: string, crates: readonly PolicyCrateRef[]): BreachRecord[] {
  const crateDirSet = new Set(crates.map((c) => c.dir));
  const breaches: BreachRecord[] = [];
  for (const crate of crates) {
    const { dir, pluginId: selfPluginId } = crate;
    const cargoTomlAbs = join(repoRoot, dir, "Cargo.toml");
    if (!existsSync(cargoTomlAbs)) continue;
    const text = readFileSync(cargoTomlAbs, "utf8");
    let m: RegExpExecArray | null;
    POLICY_CARGO_DEP_RE.lastIndex = 0;
    while ((m = POLICY_CARGO_DEP_RE.exec(text))) {
      const depName = m[1]!;
      const depPath = m[2]!;
      const resolvedAbs = resolve(join(repoRoot, dir), depPath);
      const resolvedRel = relative(repoRoot, resolvedAbs).split("\\").join("/");
      const otherPluginId = policyScopeKey(repoRoot, resolvedRel);
      if (!otherPluginId || otherPluginId === selfPluginId) continue; // outside the plugins tree entirely, or the same plugin's own tree
      if (crateDirSet.has(resolvedRel)) {
        breaches.push({
          id: `app-coupling-plugin-${dir}-${depName}`,
          summary: `${dir} depends directly on another plugin's crate (${depName} -> ${resolvedRel})`,
          kind: "app-plugin/app-coupling",
          scope: dir,
          // 🎫️ W0: downgraded from "high" — the discoverer revival surfaced ~20 real, pre-existing
          // cross-plugin dependencies (✒️writer, 📕️norm's en1992/en1993, 💠️lowpoly, and 15 for
          // 🎪️demonstrator, which the plan explicitly documents as depending on other plugins' crates by
          // design and merges last for exactly that reason — see the master plan's crate-list rulings).
          // None of these are mine to repoint from script.ts; WARN-only until each dependency is
          // resolved by its owning plugin's wave agent, then flip back to "high".
          priority: policyNewSurfacePriority(crate, "medium"),
          reason: "Wave 4 V3 coupling audit: no plugin crate may depend on another plugin's crate.",
          solution: `Remove the "${depName}" dependency from ${dir}/Cargo.toml, or move the shared logic into a neutral domain crate outside any plugin's tree (e.g. under 🧰️framework).`,
        });
        continue;
      }
      if (POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST.has(depName)) continue;
      breaches.push({
        id: `app-coupling-domain-${dir}-${depName}`,
        summary: `${dir} depends on a path under another plugin's tree (${depName} -> ${resolvedRel}) not yet vetted as shared infra`,
        kind: "app-plugin/app-coupling",
        scope: dir,
        priority: "low",
        reason: "Wave 4 V3 coupling audit: dependencies into another plugin's folder are only acceptable for documented neutral shared domain crates (see that plugin's AGENTS.md); everything else needs a look.",
        solution: `If "${depName}" is genuinely a shared domain/library crate (documented via AGENTS.md), add it to POLICY_SHARED_DOMAIN_CRATE_ALLOWLIST citing the ticket; otherwise remove the dependency.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleAppCoupling

//#region 🔧️PolicyRuleNoJsonFixtures
/** 🔎️Repo-wide `.../example/*.json` file paths (repo-relative), skipping the same dirs `policyDiscoverCrateDirs` skips. */
function policyDiscoverExampleJsonFiles(repoRoot: string): string[] {
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
      if (relDir.split("/").pop() === "example" && ent.name.endsWith(".json")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 📏️dsl/ migration lock-step rule: no example/*.json fixture may exist — every technology's example fixtures moved to its own DSL-text extension (`.puzzle2d`, `.flow`, `.draw`, …); a documented allowlist covers not-yet-migrated technologies. */
export function policyJsonFixtureBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverExampleJsonFiles(repoRoot)) {
    if (POLICY_JSON_FIXTURE_ALLOWLIST.has(relPath)) continue;
    if (POLICY_JSON_FIXTURE_PATH_PREFIX_ALLOWLIST.some((p) => relPath.startsWith(p))) continue;
    breaches.push({
      id: `no-json-fixture-${relPath}`,
      summary: `"${relPath}" is a JSON example fixture — technologies now use DSL-text fixtures`,
      kind: "dsl-migration/no-json-fixture",
      scope: relPath,
      priority: "high",
      reason: "The dsl/ derive-engine migration moved every technology's example fixtures from JSON to its own DSL-text extension (e.g. .puzzle2d, .flow, .draw); new */example/*.json files should never be added.",
      solution: `Convert ${relPath} to its technology's DSL-text extension (see an already-migrated sibling crate's *_fixture round-trip test for the pattern), or if this technology is not yet converted, add it to POLICY_JSON_FIXTURE_ALLOWLIST citing the follow-up ticket.`,
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleNoJsonFixtures

//#region 🔧️PolicyRuleOpsGrammar
/** 🔎️Repo-wide `*.ops` file paths (repo-relative). */
function policyDiscoverOpsFiles(repoRoot: string): string[] {
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
      if (ent.name.endsWith(".ops")) found.push(childRel);
    }
  };
  walk("");
  return found.sort();
}

/** 🏷️`.ops` op-log structural line keywords (see `vcs::os_store::text_format::print_document_text`/`parse_document_text`). */
const POLICY_OPS_STRUCTURAL_KEYWORDS = ["@doc", "@edit", "@change", "@checkpoint", "@alternative", "@active"];

/**
 * 📏️dsl/ migration lock-step rule: basic `.ops` op-log grammar sanity — one structural `@doc`/`@edit`/
 * `@change`/`@checkpoint`/`@alternative`/`@active` line, or one 2-space-indented mutation-text (op grammar)
 * line under a preceding `@edit`, per line; never blank. The `.ops` brand is the compact op-log format
 * (not the document `Mutation` trait name). Forward-looking: no `*.ops` fixture exists in the repo yet
 * (`FolderTextStorage`'s `.ops` companion file is additive/unwired — see `vcs/rs/lib.rs`), but the check
 * is ready the day one lands.
 */
export function policyOpsGrammarBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverOpsFiles(repoRoot)) {
    const text = policyReadFileSafe(repoRoot, relPath);
    const rawLines = text.split(/\r?\n/);
    const lines = rawLines.length > 0 && rawLines[rawLines.length - 1] === "" ? rawLines.slice(0, -1) : rawLines;
    lines.forEach((line, i) => {
      const lineNo = i + 1;
      if (line.trim().length === 0) {
        breaches.push({
          id: `ops-grammar-blank-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is blank`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "The .ops op-log grammar is one structural @-line or one indented mutation-text (op grammar) line per line — no blank lines.",
          solution: `Remove the blank line at ${relPath}:${lineNo}.`,
        });
        return;
      }
      const isStructural = POLICY_OPS_STRUCTURAL_KEYWORDS.some((kw) => line.startsWith(kw));
      const isIndentedOp = line.startsWith("  ") && line.trim().length > 0;
      if (!isStructural && !isIndentedOp) {
        breaches.push({
          id: `ops-grammar-line-${relPath}-${lineNo}`,
          summary: `"${relPath}" line ${lineNo} is neither a structural @-line nor a 2-space-indented mutation-text (op grammar) line`,
          kind: "dsl-migration/ops-grammar",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: "Every .ops line must be a recognized @doc/@edit/@change/@checkpoint/@alternative/@active structural line or a 2-space-indented mutation-text line (op grammar) under an @edit block.",
          solution: `Fix ${relPath}:${lineNo} to match the .ops grammar (see vcs::os_store::text_format::print_document_text/parse_document_text).`,
        });
      }
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleOpsGrammar
