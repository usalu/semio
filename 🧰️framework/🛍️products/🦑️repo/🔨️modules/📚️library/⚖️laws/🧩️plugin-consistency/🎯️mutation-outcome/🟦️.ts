import { existsSync, lstatSync } from "node:fs";
import { join } from "node:path";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyIsRootScriptSource } from "../🏛️foundation/🟦️.ts";
import { taxonomyRelativePathIsExcluded } from "../../../🔍️discovery/🔎️rust-source-identities/🟦️.ts";
import { POLICY_MUTATION_PLAN_DIR, POLICY_MUTATIONS_FACET, POLICY_RS_COMPONENT_LEAF_NAME, policyArtifactRootOfMutationsDir } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import type { BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { policyListMutationDirs } from "../../../🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
import { policyMaskLiterals } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";

import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** 🧫️The frozen outcome-code vocabulary's language-agnostic document — the one table this gate, the Rust protocol
 * (`OUTCOME_CODES`, persistence) and the TypeScript twin (`outcomeCodeLevel`) share. */
const POLICY_OUTCOME_CODE_FIXTURE = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/🧫️outcome-code/🔣️.json";

type PolicyOutcomeLevel = "info" | "warning" | "error" | "fatal";

export type PolicyOutcomeVocabulary = Readonly<{ codes: ReadonlyMap<string, PolicyOutcomeLevel>; applyPattern: RegExp; applyLevel: PolicyOutcomeLevel }>;

/** 📖️Reads the frozen vocabulary; a missing or malformed document fails the gate instead of admitting everything. */
export function policyOutcomeVocabulary(repoRoot: string): PolicyOutcomeVocabulary {
  const document = JSON.parse(policyReadFileSafe(repoRoot, POLICY_OUTCOME_CODE_FIXTURE)) as { codes: { code: string; level: PolicyOutcomeLevel }[]; apply: { pattern: string; level: PolicyOutcomeLevel } };
  return { codes: new Map(document.codes.map((row) => [row.code, row.level])), applyPattern: new RegExp(document.apply.pattern), applyLevel: document.apply.level };
}

/** ⚖️The level the vocabulary fixes for `code`; `null` outside it. */
function policyOutcomeCodeLevel(vocabulary: PolicyOutcomeVocabulary, code: string): PolicyOutcomeLevel | null {
  return vocabulary.codes.get(code) ?? (vocabulary.applyPattern.test(code) ? vocabulary.applyLevel : null);
}

/** 🗂️What the mutation-outcome-law rules read: git-visible regular source files and the `🧬️mutations` directories above them. */
type PolicyMutationLawInventory = Readonly<{ files: readonly string[]; mutationsDirs: readonly string[] }>;

/** 📍️Every position the rules scan: sources, Python oracles, Gherkin specifications and committed `🎯️outcome` documents. */
const POLICY_MUTATION_LAW_PATHSPECS = ["*.rs", "*.ts", "*.tsx", "*.py", "*.feature", "*🎯️outcome/🔣️.json"] as const;

const POLICY_MUTATION_LAW_INVENTORIES = new Map<string, PolicyMutationLawInventory>();

/**
 * 🚪️The mutation-outcome-law gate's ONE repo walk, memoized per root and shared by every rule of this region: git's own inventory of
 * tracked and unignored files (`git ls-files -co --exclude-standard`), so ignored build trees are never entered and a
 * symlink is never followed — git lists a link as one path and nothing beneath it. Taxonomy `pathExclusions` (opaque
 * prefixes), `POLICY_SKIP_DIRS` segments and the router itself drop before any filesystem access, and `lstat` keeps regular
 * files only, so a link (the root `.tmp-ticket` convenience link) or a deleted tracked path is skipped instead of read.
 */
export function policyMutationLawInventory(repoRoot: string): PolicyMutationLawInventory {
  const cached = POLICY_MUTATION_LAW_INVENTORIES.get(repoRoot);
  if (cached) return cached;
  const listed = runToolSyncV1("git", ["ls-files", "-co", "--exclude-standard", "-z", "--", ...POLICY_MUTATION_LAW_PATHSPECS], { cwd: repoRoot });
  if (listed.status !== 0) throw new Error(`git-visible mutation-outcome-law inventory failed: ${listed.stderr.toString()}`);
  const taxonomy = loadTaxonomy();
  const files = [...new Set(listed.stdout.toString().split("\0").map((relPath) => relPath.normalize("NFC")))]
    .filter((relPath) => relPath && !policyIsRootScriptSource(relPath) && !relPath.split("/").some((segment) => POLICY_SKIP_DIRS.has(segment)) && !taxonomyRelativePathIsExcluded(relPath, taxonomy))
    .filter((relPath) => {
      try {
        return lstatSync(join(repoRoot, relPath)).isFile();
      } catch {
        return false;
      }
    })
    .sort();
  const mutationsDirs = new Set<string>();
  for (const relPath of files) {
    const segments = relPath.split("/");
    for (let index = 0; index < segments.length - 1; index++) if (segments[index] === POLICY_MUTATIONS_FACET) mutationsDirs.add(segments.slice(0, index + 1).join("/"));
  }
  const inventory: PolicyMutationLawInventory = { files, mutationsDirs: [...mutationsDirs].sort() };
  POLICY_MUTATION_LAW_INVENTORIES.set(repoRoot, inventory);
  return inventory;
}

/** 🦀️The inventory's Rust sources. */
export function policyMutationLawRustFiles(repoRoot: string): string[] {
  return policyMutationLawInventory(repoRoot).files.filter((relPath) => relPath.endsWith(".rs"));
}

/** 🧩️ A direct mutation is composite exactly when it visibly owns an explicit plan facet. */
function policyIsCompositeMutationDir(repoRoot: string, mutationRel: string): boolean {
  return existsSync(join(repoRoot, mutationRel, POLICY_MUTATION_PLAN_DIR, POLICY_RS_COMPONENT_LEAF_NAME));
}

/** 📥️`use protocol::MutationOutcome;` or `use protocol::{.., MutationOutcome, ..};` — the import that lets a leaf spell the contract bare. */
const POLICY_OUTCOME_IMPORT_RE = /\buse\s+protocol::(?:MutationOutcome\b|\{[^}]*\bMutationOutcome\b[^}]*\})/;

/** 🧾️Whether a `🔺️diff` leaf reports through the frozen contract: `protocol::MutationOutcome<..>` spelled qualified, or bare after importing it from `protocol`. */
function policyDiffReturnsOutcome(content: string): boolean {
  return /protocol::MutationOutcome\s*</.test(content) || (POLICY_OUTCOME_IMPORT_RE.test(content) && /->\s*MutationOutcome\s*</.test(content));
}

/**
 * 📏️Rule 1: every `🧬️mutations/<slug>/🔺️diff/🦀️.rs` must return `protocol::MutationOutcome<` (qualified, or bare after
 * `use protocol::MutationOutcome`) and reference at least one of the 9 frozen codes. Composite mutation dirs (own `🧩️plan`, not `🔺️diff`) are out of scope — their outcome folds from
 * the plan. A leaf whose `🔺️diff` doesn't exist yet is out of scope here: the rule judges only the diff leaves that exist.
 */
export function policyMutationOutcomeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const vocabulary = policyOutcomeVocabulary(repoRoot);
  for (const mutationsRel of policyMutationLawInventory(repoRoot).mutationsDirs) {
    const artRel = policyArtifactRootOfMutationsDir(mutationsRel);
    for (const mutName of policyListMutationDirs(repoRoot, mutationsRel)) {
      const mutRel = `${mutationsRel}/${mutName}`;
      if (policyIsCompositeMutationDir(repoRoot, mutRel)) continue;
      const diffRel = `${mutRel}/🔺️diff/${POLICY_RS_COMPONENT_LEAF_NAME}`;
      if (!existsSync(join(repoRoot, diffRel))) continue;
      const content = policyReadFileSafe(repoRoot, diffRel);
      if (!policyDiffReturnsOutcome(content)) {
        breaches.push({
          id: `mutation-outcome-missing-type-${diffRel}`,
          summary: `"${diffRel}" does not return protocol::MutationOutcome<..>`,
          kind: "mutation-migration/outcome",
          scope: artRel,
          priority: "high",
          reason: "C2/C4: every 🔺️diff leaf must report through the frozen MutationOutcome<D> contract.",
          solution: `Change diff's signature to -> protocol::MutationOutcome<XDiff> and wrap the success path in MutationOutcome::new(..) (see 📋️contract-freeze.md's fan-out recipe, or the already-converted 🕸️dag/📐️cad/💠️lowpoly facets).`,
        });
        continue;
      }
      const hasCode = [...vocabulary.codes.keys()].some((code) => content.includes(code) || content.includes(`OutcomeCode::${code.replace(/^mutation\./, "").replace(/(?:^|-)([a-z])/g, (_, letter: string) => letter.toUpperCase())}`));
      if (hasCode) continue;
      breaches.push({
        id: `mutation-outcome-missing-code-${diffRel}`,
        summary: `"${diffRel}" returns protocol::MutationOutcome<..> but never references one of the 9 frozen message codes`,
        kind: "mutation-migration/outcome",
        scope: artRel,
        priority: "high",
        reason: "The verb-family table requires real Error/Warning/Fatal/Info detection per verb family; no mutation architecture is exempt.",
        solution: `Add the real detection this verb family requires (target missing ⇒ ::error("mutation.target-missing", ..), idempotent ⇒ .warning("mutation.no-op", ..), duplicate id / invariant ⇒ ::fatal(..), cascade ⇒ .info("mutation.cascade", ..)).`,
      });
    }
  }
  return breaches;
}

/** 🔨️Rust outcome builders with the level they fix: `MutationOutcome::(error|fatal)(..)` and `MutationMessage::(info|warning|error|fatal)(..)`. */
const POLICY_OUTCOME_RUST_BUILDER_RE = /\b(?:MutationOutcome::(error|fatal)|MutationMessage::(info|warning|error|fatal))\s*\(\s*"([^"]*)"/g;

/** ⛓️The chainable `.info(..)`/`.warning(..)` shorthand — checked only inside a `fn` body that builds a `MutationOutcome`, so an unrelated sink's `.warning(..)`/`tracing::warn!` never matches. */
const POLICY_OUTCOME_RUST_CHAIN_RE = /\.(info|warning)\s*\(\s*"([^"]*)"/g;

/** 🪦️The retired `warn` builder spelling: `MutationMessage::warn(..)` anywhere, the chainable `.warn(..)` inside an outcome-building body. */
const POLICY_OUTCOME_RUST_RETIRED_BUILDER_RE = /(?:\bMutationMessage::|\.)warn\s*\(/g;

/** ⌛️Apply-time rejections (`MutationOutcome::apply_to` persists them as `Fatal` messages): `MutationApplyError::new("…")`, `MutationApplyError { code: "…" }`. */
const POLICY_OUTCOME_RUST_APPLY_RE = /\bMutationApplyError(?:::new\s*\(\s*|\s*\{\s*code:\s*)"([^"]*)"/g;

/** 🛑️TypeScript twins: `refuse("<level>", "<code>", ..)` and `{ level: "<level>", code: "mutation.…" }` messages (the retired `warn` spelling is matched so it is reported, never skipped). */
const POLICY_OUTCOME_TS_REFUSE_RE = /\brefuse\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([^"]*)"/g;

const POLICY_OUTCOME_TS_OBJECT_RE = /\blevel:\s*"(info|warn|warning|error|fatal)"\s*,\s*code:\s*"(mutation\.[^"]*)"|\bcode:\s*"(mutation\.[^"]*)"\s*,\s*level:\s*"(info|warn|warning|error|fatal)"/g;

/** 🐍️Python second implementations: `("<level>", "<dotted-code>")` outcome tuples. */
const POLICY_OUTCOME_PY_TUPLE_RE = /\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([a-z0-9]+(?:[.-][a-z0-9]+)+)"\s*\)/g;

/** 📨️Gherkin outcome messages: `{level: <level>, code: mutation.…}`. */
const POLICY_OUTCOME_FEATURE_MESSAGE_RE = /\blevel:\s*([a-z]+)\s*,\s*code:\s*(mutation\.[a-z0-9][a-z0-9.-]*[a-z0-9])(?![\w.<*-])/g;

/** 🎚️A hand-written Rust level decoder admitting the retired `"warn"` spelling for `Severity::Warning` — the wire name is `warning` only. */
const POLICY_OUTCOME_RUST_LEVEL_ALIAS_RE = /(?:"[a-z]+"\s*\|\s*)*"warn"\s*(?:\|\s*"[a-z]+"\s*)*=>\s*(?:Ok\(|Some\()?(?:[A-Za-z_]\w*::)*Severity::Warning\b/g;

/** ✅️A complete string literal in the `mutation.` namespace — an outcome code wherever mutation leaves, diffs and oracles spell one. */
const POLICY_OUTCOME_NAMESPACE_LITERAL_RE = /["'`](mutation\.[a-z0-9][a-z0-9.-]*)["'`]/g;

/** 🎨️A `mutation.` token in a Gherkin specification (`mutation.apply.<detail>` placeholders excluded). */
const POLICY_OUTCOME_FEATURE_TOKEN_RE = /(?<![\w.-])(mutation\.[a-z0-9][a-z0-9-]*(?:\.[a-z0-9][a-z0-9-]*)*)(?![\w-]|\.[<*])/g;

/** 🔎️Finds `fn <name>(..` openers so the chainable shorthand is scoped to outcome-building bodies. */
const POLICY_FN_DECL_RE = /\bfn\s+[A-Za-z_]\w*/g;

/** 🍃️Outcome-code positions by domain: mutation leaves, diffs and their oracles. */
const POLICY_OUTCOME_LEAF_SEGMENTS = ["/🧬️mutations/", "/🔺️diff/"] as const;

/** 🧱️Spans of every `fn` body that mentions `MutationOutcome`, found with ONE literal-masked brace pass per file. */
function policyOutcomeFnBodyMask(content: string): Uint8Array {
  const masked = policyMaskLiterals(content);
  const close = new Int32Array(masked.length).fill(-1);
  const stack: number[] = [];
  for (let i = 0; i < masked.length; i++) {
    const ch = masked.charCodeAt(i);
    if (ch === 123) stack.push(i);
    else if (ch === 125) {
      const open = stack.pop();
      if (open !== undefined) close[open] = i;
    }
  }
  const mask = new Uint8Array(content.length);
  POLICY_FN_DECL_RE.lastIndex = 0;
  let fm: RegExpExecArray | null;
  while ((fm = POLICY_FN_DECL_RE.exec(masked))) {
    const open = masked.indexOf("{", fm.index);
    const end = open === -1 ? -1 : close[open]!;
    if (end !== -1 && content.slice(open, end + 1).includes("MutationOutcome")) mask.fill(1, open, end + 1);
  }
  return mask;
}

/**
 * 💬️Rule 2: every outcome-code position spells a code of the frozen vocabulary (`🎮️mutation/🧫️fixtures/🧫️outcome-code`)
 * at the one level it fixes — persistence (`🏪️store` `expected_mutation_message_level`) refuses anything else. Positions:
 * Rust builders and the chainable shorthand inside outcome-building bodies (code + level), `MutationApplyError` codes
 * (`mutation.apply.<detail>`), `mutation.` literals in mutation leaves/diffs (Rust, TypeScript) and Python oracles, the
 * TypeScript twins' `refuse(..)`/`{ level, code }` messages, Python `(level, code)` tuples, committed `🎯️outcome` documents
 * (code + level), Gherkin specifications (codes and `{level, code}` messages) and hand-written Rust level decoders. A level
 * has one spelling (`warning`, never `warn`). Non-outcome fault, diagnostic and command codes live in their own namespaces.
 */
export function policyMutationMessageCodeBreaches(repoRoot: string): BreachRecord[] {
  const vocabulary = policyOutcomeVocabulary(repoRoot);
  return policyMutationLawInventory(repoRoot).files.flatMap((relPath) => policyOutcomeCodeFileBreaches(vocabulary, relPath, policyReadFileSafe(repoRoot, relPath)));
}

/**
 * 🔬️Rule 2 over ONE file, pure: the outcome-code positions of `content` (classified by `relPath`'s extension and domain
 * segments) checked against `vocabulary`. A scanned level is whatever the source spells (`warn` included), so a retired or
 * foreign spelling is reported instead of being cast into the vocabulary's level type. The planted-violation law
 * (`🧪️tests/🧪️outcome-law-gate`) drives exactly this function.
 */
export function policyOutcomeCodeFileBreaches(vocabulary: PolicyOutcomeVocabulary, relPath: string, content: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  if (!content || (relPath.endsWith(".py") && !relPath.includes("🧪️tests/"))) return breaches;
  const reported = new Set<string>();
  const check = (code: string, level: string | null, index: number, apply = false) => {
    const expected = policyOutcomeCodeLevel(vocabulary, code);
    const fault = apply ? (vocabulary.applyPattern.test(code) ? null : "is not a mutation.apply.<detail> apply-rejection code") : expected === null ? "is not in the frozen outcome vocabulary" : level !== null && level !== expected ? `is fixed at ${expected}, not ${level}` : null;
    if (fault === null) return;
    const line = policyLineOfIndex(content, index);
    if (reported.has(`${line}:${code}`)) return;
    reported.add(`${line}:${code}`);
    breaches.push({
      id: `mutation-message-code-${relPath}-${line}-${code}`,
      summary: `"${relPath}:${line}" outcome code "${code}"${level === null ? "" : ` at ${level}`} ${fault}`,
      kind: "mutation-migration/message-code",
      scope: relPath,
      line,
      priority: "high",
      reason: "Persistence admits exactly the frozen outcome vocabulary, each code at its one level (`🎮️mutation/🧫️fixtures/🧫️outcome-code`); any other code or level makes the history unpersistable. There are no per-plugin codes.",
      solution: "Map the refusal by meaning: missing → mutation.target-missing, still referenced → mutation.target-referenced, inconsistent with the current state → mutation.target-mismatch, duplicate identity → mutation.duplicate-id, payload-intrinsic → mutation.invariant (+ schema bound or x-semio-invariant), apply-time failure → mutation.apply.<detail>; use the level the vocabulary fixes (MutationOutcome::refuse picks it for runtime codes).",
    });
  };
  const scan = (re: RegExp, visit: (m: RegExpExecArray) => void) => {
    re.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = re.exec(content))) visit(m);
  };
  const leaf = POLICY_OUTCOME_LEAF_SEGMENTS.some((segment) => `/${relPath}`.includes(segment));
  if (relPath.endsWith(".rs")) {
    scan(POLICY_OUTCOME_RUST_BUILDER_RE, (m) => check(m[3]!, (m[1] ?? m[2])!, m.index));
    scan(POLICY_OUTCOME_RUST_APPLY_RE, (m) => check(m[1]!, null, m.index, true));
    if (leaf) scan(POLICY_OUTCOME_NAMESPACE_LITERAL_RE, (m) => check(m[1]!, null, m.index));
    const bodies = content.includes("MutationOutcome") ? policyOutcomeFnBodyMask(content) : new Uint8Array(content.length);
    scan(POLICY_OUTCOME_RUST_CHAIN_RE, (m) => {
      if (bodies[m.index] === 1) check(m[2]!, m[1]!, m.index);
    });
    scan(POLICY_OUTCOME_RUST_RETIRED_BUILDER_RE, (m) => {
      if (m[0].startsWith(".") && bodies[m.index] !== 1) return;
      const line = policyLineOfIndex(content, m.index);
      breaches.push({
        id: `mutation-message-retired-builder-${relPath}-${line}-${m.index}`,
        summary: `"${relPath}:${line}" outcome builder "warn" is a retired spelling of "warning"`,
        kind: "mutation-migration/message-code",
        scope: relPath,
        line,
        priority: "high",
        reason: "An outcome level has one spelling in the API as on the wire: `MutationMessage::warning` / `MutationOutcome::warning` build the `warning` level.",
        solution: "Call `MutationMessage::warning(..)` / `.warning(..)`.",
      });
    });
    scan(POLICY_OUTCOME_RUST_LEVEL_ALIAS_RE, (m) => {
      const line = policyLineOfIndex(content, m.index);
      breaches.push({
        id: `mutation-message-level-alias-${relPath}-${line}`,
        summary: `"${relPath}:${line}" outcome level "warn" is a retired spelling of "warning"`,
        kind: "mutation-migration/message-code",
        scope: relPath,
        line,
        priority: "high",
        reason: "An outcome level has one wire spelling (`info`, `warning`, `error`, `fatal` — `Severity`'s own encoding); a decoder that also admits `warn` lets a non-canonical document pass as valid.",
        solution: 'Decode only "warning" (or use the `Severity` value decoder) and write "warning" in every committed outcome.',
      });
    });
  } else if (relPath.endsWith(".ts") || relPath.endsWith(".tsx")) {
    scan(POLICY_OUTCOME_TS_REFUSE_RE, (m) => check(m[2]!, m[1]!, m.index));
    scan(POLICY_OUTCOME_TS_OBJECT_RE, (m) => check((m[2] ?? m[3])!, (m[1] ?? m[4])!, m.index));
    if (leaf) scan(POLICY_OUTCOME_NAMESPACE_LITERAL_RE, (m) => check(m[1]!, null, m.index));
  } else if (relPath.endsWith(".py")) {
    scan(POLICY_OUTCOME_PY_TUPLE_RE, (m) => check(m[2]!, m[1]!, m.index));
    scan(POLICY_OUTCOME_NAMESPACE_LITERAL_RE, (m) => check(m[1]!, null, m.index));
  } else if (relPath.endsWith(".feature")) {
    scan(POLICY_OUTCOME_FEATURE_TOKEN_RE, (m) => {
      if (!/\.(json|rs|ts|tsx|py|md|txt)$/.test(m[1]!)) check(m[1]!, null, m.index);
    });
    scan(POLICY_OUTCOME_FEATURE_MESSAGE_RE, (m) => check(m[2]!, m[1]!, m.index));
  } else {
    let outcome: unknown;
    try {
      outcome = JSON.parse(content);
    } catch {
      return breaches;
    }
    if (outcome === null || typeof outcome !== "object") return breaches;
    const level = (value: unknown): string | null => (typeof value === "string" ? value : null);
    const record = outcome as { code?: unknown; level?: unknown; messages?: unknown };
    let cursor = 0;
    const locate = (code: string): number => {
      const at = content.indexOf(JSON.stringify(code), cursor);
      cursor = at === -1 ? cursor : at + 1;
      return at === -1 ? 0 : at;
    };
    for (const message of [record, ...(Array.isArray(record.messages) ? (record.messages as { code?: unknown; level?: unknown }[]) : [])]) {
      if (message !== null && typeof message === "object" && typeof message.code === "string") check(message.code, level(message.level), locate(message.code));
    }
  }
  return breaches;
}

export function policyMutationLawSourceFiles(repoRoot: string): string[] {
  return policyMutationLawInventory(repoRoot).files.filter((relPath) => relPath.endsWith(".rs") || relPath.endsWith(".ts") || relPath.endsWith(".tsx"));
}
