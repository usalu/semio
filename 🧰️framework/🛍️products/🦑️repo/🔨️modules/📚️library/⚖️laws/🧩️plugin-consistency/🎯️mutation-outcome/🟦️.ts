/** 🧭️ Wave 4 plugin consistency policy: mutation outcome, merge policy and diff-only laws.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { policyMaskLiterals } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { loadTaxonomy, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { taxonomyRelativePathIsExcluded } from "../../../🔍️discovery/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_MUTATION_PLAN_DIR, POLICY_MUTATIONS_FACET, POLICY_RS_COMPONENT_LEAF_NAME, policyArtifactRootOfMutationsDir } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyListMutationDirs } from "../../../🧹️normalization/🧬️mutation/📇️direct-owner-index/🟦️.ts";
import { existsSync, lstatSync } from "node:fs";
import { join } from "node:path";
import { policyExtractFnBody, policyIsRootScriptSource } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleMutationOutcomeMergePolicy
/**
 * 🎫️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` C2/C4/C10 gates: every
 * `🔺️diff` leaf must report through `protocol::MutationOutcome<D>` using only the 9 frozen codes
 * (`📋️contract-freeze.md` §C2 plus the two state-dependent `Error` codes `mutation.target-referenced` and
 * `mutation.target-mismatch` added 2026-09-30, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING), `validate` is deleted everywhere, `Severity::Hint` is gone, the CRDT
 * merge-strategy/conflict-rule vocabulary reaches zero, `MergePolicy`'s 3 variants mirror across all 4
 * surfaces that spell them, and the dsl derive macro's package glue mounts its one owner implementation.
 * All sixteen rules (the seven above plus the diff-only rules R8–R16 of ticket `26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`) read one shared git-visible inventory (`policyMutationLawInventory`).
 */
/** 📖️The frozen outcome-code vocabulary's language-agnostic document — the one table this gate, the Rust protocol
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
/** 🔎️Every position the rules scan: sources, Python oracles, Gherkin specifications and committed `🎯️outcome` documents. */
const POLICY_MUTATION_LAW_PATHSPECS = ["*.rs", "*.ts", "*.tsx", "*.py", "*.feature", "*🎯️outcome/🔣️.json"] as const;
const POLICY_MUTATION_LAW_INVENTORIES = new Map<string, PolicyMutationLawInventory>();

/**
 * 🗂️The mutation-outcome-law gate's ONE repo walk, memoized per root and shared by every rule of this region: git's own inventory of
 * tracked and unignored files (`git ls-files -co --exclude-standard`), so ignored build trees are never entered and a
 * symlink is never followed — git lists a link as one path and nothing beneath it. Taxonomy `pathExclusions` (opaque
 * prefixes), `POLICY_SKIP_DIRS` segments and the router itself drop before any filesystem access, and `lstat` keeps regular
 * files only, so a link (the root `.tmp-ticket` convenience link) or a deleted tracked path is skipped instead of read.
 */
export function policyMutationLawInventory(repoRoot: string): PolicyMutationLawInventory {
  const cached = POLICY_MUTATION_LAW_INVENTORIES.get(repoRoot);
  if (cached) return cached;
  const listed = Bun.spawnSync(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", ...POLICY_MUTATION_LAW_PATHSPECS], { cwd: repoRoot });
  if (listed.exitCode !== 0) throw new Error(`git-visible mutation-outcome-law inventory failed: ${listed.stderr.toString()}`);
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
function policyMutationLawRustFiles(repoRoot: string): string[] {
  return policyMutationLawInventory(repoRoot).files.filter((relPath) => relPath.endsWith(".rs"));
}

/** 🧩️ A direct mutation is composite exactly when it visibly owns an explicit plan facet. */
function policyIsCompositeMutationDir(repoRoot: string, mutationRel: string): boolean {
  return existsSync(join(repoRoot, mutationRel, POLICY_MUTATION_PLAN_DIR, POLICY_RS_COMPONENT_LEAF_NAME));
}

/** 🔎️`use protocol::MutationOutcome;` or `use protocol::{.., MutationOutcome, ..};` — the import that lets a leaf spell the contract bare. */
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

/** 🔎️Rust outcome builders with the level they fix: `MutationOutcome::(error|fatal)(..)` and `MutationMessage::(info|warning|error|fatal)(..)`. */
const POLICY_OUTCOME_RUST_BUILDER_RE = /\b(?:MutationOutcome::(error|fatal)|MutationMessage::(info|warning|error|fatal))\s*\(\s*"([^"]*)"/g;
/** 🔎️The chainable `.info(..)`/`.warning(..)` shorthand — checked only inside a `fn` body that builds a `MutationOutcome`, so an unrelated sink's `.warning(..)`/`tracing::warn!` never matches. */
const POLICY_OUTCOME_RUST_CHAIN_RE = /\.(info|warning)\s*\(\s*"([^"]*)"/g;
/** 🔎️The retired `warn` builder spelling: `MutationMessage::warn(..)` anywhere, the chainable `.warn(..)` inside an outcome-building body. */
const POLICY_OUTCOME_RUST_RETIRED_BUILDER_RE = /(?:\bMutationMessage::|\.)warn\s*\(/g;
/** 🔎️Apply-time rejections (`MutationOutcome::apply_to` persists them as `Fatal` messages): `MutationApplyError::new("…")`, `MutationApplyError { code: "…" }`. */
const POLICY_OUTCOME_RUST_APPLY_RE = /\bMutationApplyError(?:::new\s*\(\s*|\s*\{\s*code:\s*)"([^"]*)"/g;
/** 🔎️TypeScript twins: `refuse("<level>", "<code>", ..)` and `{ level: "<level>", code: "mutation.…" }` messages (the retired `warn` spelling is matched so it is reported, never skipped). */
const POLICY_OUTCOME_TS_REFUSE_RE = /\brefuse\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([^"]*)"/g;
const POLICY_OUTCOME_TS_OBJECT_RE = /\blevel:\s*"(info|warn|warning|error|fatal)"\s*,\s*code:\s*"(mutation\.[^"]*)"|\bcode:\s*"(mutation\.[^"]*)"\s*,\s*level:\s*"(info|warn|warning|error|fatal)"/g;
/** 🔎️Python second implementations: `("<level>", "<dotted-code>")` outcome tuples. */
const POLICY_OUTCOME_PY_TUPLE_RE = /\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([a-z0-9]+(?:[.-][a-z0-9]+)+)"\s*\)/g;
/** 🔎️Gherkin outcome messages: `{level: <level>, code: mutation.…}`. */
const POLICY_OUTCOME_FEATURE_MESSAGE_RE = /\blevel:\s*([a-z]+)\s*,\s*code:\s*(mutation\.[a-z0-9][a-z0-9.-]*[a-z0-9])(?![\w.<*-])/g;
/** 🔎️A hand-written Rust level decoder admitting the retired `"warn"` spelling for `Severity::Warning` — the wire name is `warning` only. */
const POLICY_OUTCOME_RUST_LEVEL_ALIAS_RE = /(?:"[a-z]+"\s*\|\s*)*"warn"\s*(?:\|\s*"[a-z]+"\s*)*=>\s*(?:Ok\(|Some\()?(?:[A-Za-z_]\w*::)*Severity::Warning\b/g;
/** 🔎️A complete string literal in the `mutation.` namespace — an outcome code wherever mutation leaves, diffs and oracles spell one. */
const POLICY_OUTCOME_NAMESPACE_LITERAL_RE = /["'`](mutation\.[a-z0-9][a-z0-9.-]*)["'`]/g;
/** 🔎️A `mutation.` token in a Gherkin specification (`mutation.apply.<detail>` placeholders excluded). */
const POLICY_OUTCOME_FEATURE_TOKEN_RE = /(?<![\w.-])(mutation\.[a-z0-9][a-z0-9-]*(?:\.[a-z0-9][a-z0-9-]*)*)(?![\w-]|\.[<*])/g;
/** 🔎️Finds `fn <name>(..` openers so the chainable shorthand is scoped to outcome-building bodies. */
const POLICY_FN_DECL_RE = /\bfn\s+[A-Za-z_]\w*/g;
/** 🔎️Outcome-code positions by domain: mutation leaves, diffs and their oracles. */
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
 * 📏️Rule 2: every outcome-code position spells a code of the frozen vocabulary (`🎮️mutation/🧫️fixtures/🧫️outcome-code`)
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

/** 🔎️CRDT-era vocabulary C10 deletes: `merge_strategy`, `MergeStrategyKind`, `merge_concurrent_diffs`, `ConflictRule`, `ResolutionPlan`, `assert_crdt_*`. */
const POLICY_CRDT_VOCABULARY_TOKENS = ["merge_strategy", "MergeStrategyKind", "merge_concurrent_diffs", "ConflictRule", "ResolutionPlan", "assert_crdt_"] as const;
const POLICY_CRDT_VOCABULARY_RE = new RegExp(`\\b(${POLICY_CRDT_VOCABULARY_TOKENS.join("|")})`, "g");

/**
 * 📏️Rule 3: zero repo-wide occurrences of the CRDT merge-strategy/conflict-rule vocabulary C10 deletes
 * (git-visible files of `policyMutationLawInventory`; `.🧬semio/`, `node_modules`, `target`, `dist` excluded via `POLICY_SKIP_DIRS`). Scoped to `.rs` files —
 * every token is a Rust-only identifier (no TS mirror was ever specified for the deleted CRDT pair).
 */
export function policyNoCrdtVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyMutationLawRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    POLICY_CRDT_VOCABULARY_RE.lastIndex = 0;
    const seenLines = new Set<number>();
    let m: RegExpExecArray | null;
    while ((m = POLICY_CRDT_VOCABULARY_RE.exec(content))) {
      const line = policyLineOfIndex(content, m.index);
      if (seenLines.has(line)) continue;
      seenLines.add(line);
      breaches.push({
        id: `no-crdt-vocabulary-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references CRDT vocabulary "${m[1]}"`,
        kind: "mutation-migration/no-crdt-vocabulary",
        scope: relPath,
        line,
        priority: "high",
        reason: "C10 deletes 📡️spr/🔀️crdt/** and its whole vocabulary in favor of C3's MergePolicy + C5's first-class Conflict — merge_strategy/MergeStrategyKind/merge_concurrent_diffs/ConflictRule/ResolutionPlan/assert_crdt_* must reach zero occurrences repo-wide.",
        solution: `Delete or rewrite this reference — reach for protocol::MergePolicy / the new 📡️spr/⚔️conflict module instead (📋️contract-freeze.md §C3/C5/C10).`,
      });
    }
  }
  return breaches;
}

/** 🔎️A `fn validate(&self, ..)` override — the exact shape C4/C10 delete from `Mutation`/`MutationKind`/`CompositeMutationKind`. */
const POLICY_VALIDATE_OVERRIDE_RE = /\bfn\s+validate\s*\(\s*&self\b/g;
/** 🔎️An `impl (protocol::)?(Composite)?MutationKind<..> for X {` block opener, to scope the second half of rule 4. */
const POLICY_MUTATION_KIND_IMPL_RE = /\bimpl\b[^\n{]*\b(?:CompositeMutationKind|MutationKind)\s*</g;

/**
 * 📏️Rule 4: no `fn validate(&self, ..)` survives — not inside a `🧬️mutations/**` file, and not inside
 * any `impl … MutationKind`/`impl … CompositeMutationKind` block anywhere (hand-written config/presence
 * enums included). Its checks move into the `🔺️diff` leaf as Error/Fatal messages (C4/C10). Note: gltf's
 * `pub fn validate(payload: &P, base: &S) -> Result<(), GltfTopLevelMutationRejection>` is a FREE function
 * with no `&self` receiver — a structurally different shape that this rule never matches.
 */
export function policyNoValidateOverrideBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyMutationLawRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    const reportedLines = new Set<number>();
    const pushBreach = (line: number, detail: string) => {
      if (reportedLines.has(line)) return;
      reportedLines.add(line);
      breaches.push({
        id: `no-validate-override-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still overrides fn validate(&self, ..) ${detail}`,
        kind: "mutation-migration/no-validate-override",
        scope: relPath,
        line,
        priority: "high",
        reason: "C4/C10: validate no longer exists on Mutation/MutationKind/CompositeMutationKind — every override is deleted, its checks moved into the 🔺️diff leaf as Error/Fatal messages.",
        solution: `Delete this fn validate override; move its check into the sibling 🔺️diff leaf as a protocol::MutationOutcome::error(..)/::fatal(..) message.`,
      });
    };

    const underMutations = relPath.replaceAll("\\", "/").includes(`/${POLICY_MUTATIONS_FACET}/`);
    if (underMutations) {
      POLICY_VALIDATE_OVERRIDE_RE.lastIndex = 0;
      let vm: RegExpExecArray | null;
      while ((vm = POLICY_VALIDATE_OVERRIDE_RE.exec(content))) pushBreach(policyLineOfIndex(content, vm.index), `under a 🧬️mutations/ leaf`);
    }

    POLICY_MUTATION_KIND_IMPL_RE.lastIndex = 0;
    let im: RegExpExecArray | null;
    while ((im = POLICY_MUTATION_KIND_IMPL_RE.exec(content))) {
      const body = policyExtractFnBody(content, im.index);
      POLICY_VALIDATE_OVERRIDE_RE.lastIndex = 0;
      const vm = POLICY_VALIDATE_OVERRIDE_RE.exec(body);
      if (!vm) continue;
      pushBreach(policyLineOfIndex(content, im.index + vm.index), `inside an impl …MutationKind block`);
    }
  }
  return breaches;
}

/** 🔎️The inventory's source files (`.rs`/`.ts`/`.tsx`) repo-wide — the canonical root router, whose policy literals must not scan themselves, is already out. */
function policyMutationLawSourceFiles(repoRoot: string): string[] {
  return policyMutationLawInventory(repoRoot).files.filter((relPath) => relPath.endsWith(".rs") || relPath.endsWith(".ts") || relPath.endsWith(".tsx"));
}

const POLICY_SEVERITY_HINT_STRUCT_RE = /\bSeverity(?:::|\.)Hint\b/g;

/**
 * 📏️Rule 5: zero `Severity::Hint`/`Severity.Hint` (C1 declaration order is now `Info, Warning, Error,
 * Fatal` — `derive(Ord)` IS the level order, Hint was folded into Info repo-wide), and zero bare `"hint"`
 * severity-literal on a line that also mentions "severity"/"level" (scoped to avoid flagging the ordinary
 * English word "hint" used for unrelated UI copy/tooltips).
 */
export function policySeverityInfoBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyMutationLawSourceFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content) continue;
    const reportedLines = new Set<number>();
    POLICY_SEVERITY_HINT_STRUCT_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_SEVERITY_HINT_STRUCT_RE.exec(content))) {
      const line = policyLineOfIndex(content, m.index);
      if (reportedLines.has(line)) continue;
      reportedLines.add(line);
      breaches.push({
        id: `severity-hint-${relPath}-${line}`,
        summary: `"${relPath}:${line}" still references Severity::Hint/Severity.Hint`,
        kind: "mutation-migration/no-severity-hint",
        scope: relPath,
        line,
        priority: "high",
        reason: "C1 collapsed Hint into Info repo-wide; Severity's declaration order (Info < Warning < Error < Fatal, 0..3) has no Hint member anymore.",
        solution: `Rewrite this to Severity::Info / "Info" — Hint was merged into Info by C1.`,
      });
    }
    const lines = content.split(/\r?\n/);
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i]!;
      if (!/["']hint["']/i.test(line) || !/severity|level/i.test(line)) continue;
      const lineNo = i + 1;
      if (reportedLines.has(lineNo)) continue;
      reportedLines.add(lineNo);
      breaches.push({
        id: `severity-hint-literal-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" uses a "hint" severity literal`,
        kind: "mutation-migration/no-severity-hint",
        scope: relPath,
        line: lineNo,
        priority: "high",
        reason: "C1 collapsed Hint into Info repo-wide; a serialized/matched severity level literal must never be \"hint\".",
        solution: `Rewrite the literal to "info" — Hint was merged into Info by C1.`,
      });
    }
  }
  return breaches;
}

/** 🔎️The 3 frozen MergePolicy variants (C3) spelled at exactly these 4 surfaces. */
const POLICY_MERGE_POLICY_VARIANTS = ["LaissezFaire", "Normal", "Vigilant"] as const;
/** 🔡️Idiomatic per-language spelling accepted for each variant — Rust surfaces spell it exactly like the enum (`LaissezFaire`); TS surfaces (kernel types, label contract, i18n bundles) may use either the PascalCase mirror or the camelCase object-key form (`laissezFaire`) that idiomatic TS reaches for. Presence-only, per surface's own convention — this is a mirror-existence check, not a shape check. */
const POLICY_MERGE_POLICY_VARIANT_SPELLINGS: Readonly<Record<(typeof POLICY_MERGE_POLICY_VARIANTS)[number], readonly string[]>> = {
  LaissezFaire: ["LaissezFaire", "laissezFaire"],
  Normal: ["Normal", "normal"],
  Vigilant: ["Vigilant", "vigilant"],
};
/** 🗺️The surfaces that SPELL the variants. The host codec (`💻️os/🟦️.ts`) carries only the ordinal through the kernel twin's `mergePolicyAsU8`, so the kernel types stand for it. */
const POLICY_MERGE_POLICY_SURFACES: readonly { label: string; relPath: string }[] = [
  { label: "Rust spine (protocol::MergePolicy, 📡️replication/🧾️wire)", relPath: "🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs" },
  { label: "TS kernel types (🎠️kernel/🔄️merge-outcome)", relPath: "🧰️framework/🔨️modules/🎠️kernel/🔄️merge-outcome/🟦️.ts" },
  { label: "UI label contract (📚️I18n policy.*)", relPath: "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx" },
  { label: "i18n bundles (en+de, ⚛️react/🌐️i18n)", relPath: "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts" },
];

/**
 * 📏️Rule 6: the three `MergePolicy` variants must be present in all 4 surfaces that spell them (C3/C9),
 * each under whatever idiomatic spelling that surface's language uses (`POLICY_MERGE_POLICY_VARIANT_
 * SPELLINGS` — as substrings, this is a mirror-existence check, not a shape check); one breach per surface
 * that is missing at least one variant under any of its spellings, or that no longer exists at all.
 */
export function policyMergePolicyParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const surface of POLICY_MERGE_POLICY_SURFACES) {
    const content = policyReadFileSafe(repoRoot, surface.relPath);
    const missing = content ? POLICY_MERGE_POLICY_VARIANTS.filter((v) => !POLICY_MERGE_POLICY_VARIANT_SPELLINGS[v].some((spelling) => content.includes(spelling))) : [...POLICY_MERGE_POLICY_VARIANTS];
    if (missing.length === 0) continue;
    breaches.push({
      id: `merge-policy-parity-${surface.relPath}`,
      summary: content ? `"${surface.relPath}" (${surface.label}) is missing MergePolicy variant(s): ${missing.join(", ")}` : `"${surface.relPath}" (${surface.label}) does not exist — the surface moved and this gate lists a stale path`,
      kind: "mutation-migration/merge-policy-parity",
      scope: surface.relPath,
      priority: "high",
      reason: "C3/C9: MergePolicy {LaissezFaire, Normal, Vigilant} must mirror across all 4 surfaces that spell it (Rust spine, TS kernel types, UI label contract, en+de i18n bundles) or the merge-policy setting silently desyncs across a surface.",
      solution: content ? `Add the missing MergePolicy variant name(s) to ${surface.relPath} (see 📋️contract-freeze.md §C3/C9 for the exact shape expected at this surface).` : `Point POLICY_MERGE_POLICY_SURFACES at the file that now spells this surface.`,
    });
  }
  return breaches;
}

const POLICY_DERIVE_OWNER = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs";
const POLICY_DERIVE_GLUE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs";
/** 🔎️The glue's one mount of the owner implementation. */
const POLICY_DERIVE_GLUE_MOUNT_RE = /#\[path\s*=\s*"\.\.\/\.\.\/🦀️\.rs"\]\s*mod\s+component\s*;/;

/** 📏️Rule 7: the dsl derive macro has ONE implementation — the proc-macro package glue mounts the owner `🦀️.rs` (`#[path = "../../🦀️.rs"] mod component;`) and only forwards to it, so no build shape can run a stale copy of the derive logic. */
export function policyDeriveGlueMountBreaches(repoRoot: string): BreachRecord[] {
  const glue = policyReadFileSafe(repoRoot, POLICY_DERIVE_GLUE);
  if (policyReadFileSafe(repoRoot, POLICY_DERIVE_OWNER) && POLICY_DERIVE_GLUE_MOUNT_RE.test(glue) && !/\bfn\s+expand_/.test(glue)) return [];
  return [
    {
      id: `derive-glue-mount-${POLICY_DERIVE_GLUE}`,
      summary: `"${POLICY_DERIVE_GLUE}" does not mount the derive owner "${POLICY_DERIVE_OWNER}" as its one implementation`,
      kind: "mutation-migration/derive-glue-mount",
      scope: POLICY_DERIVE_GLUE,
      priority: "high",
      reason: "The dsl derive macro's proc-macro package is glue: it mounts the owner implementation instead of carrying a copy, so the macro body exists once and no build shape runs stale derive logic.",
      solution: `Make ${POLICY_DERIVE_GLUE} mount the owner with #[path = "../../🦀️.rs"] mod component; and forward each #[proc_macro_derive] entry to component::expand_*; move any expand_* body into ${POLICY_DERIVE_OWNER}.`,
    },
  ];
}

type PolicyDiffOnlyRuleId = "R8" | "R9" | "R10" | "R11" | "R12" | "R13" | "R14" | "R15" | "R16";
type PolicyDiffOnlyRule = Readonly<{ code: string; slug: string; reason: string; solution: string }>;

/** 📜️The diff-only mutation law's rules (`26/10/08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES` design L1–L5, violation codes V1–V4): one reason and one remedy per rule, keyed by rule id. */
const POLICY_DIFF_ONLY_RULES: Readonly<Record<PolicyDiffOnlyRuleId, PolicyDiffOnlyRule>> = {
  R8: { code: "V3-LEAF-APPLY", slug: "leaf-mutable-base", reason: "a scanned diff/inverse/helper fn takes or creates a `&mut`", solution: "Build the sparse diff or concrete inverse declaratively from the payload and reads of `base`; only the central applier writes into a snapshot." },
  R9: { code: "V3-LEAF-APPLY", slug: "leaf-applies-diff", reason: "a diff is applied outside the central applier (`apply_diff`, `ApplyCapability`, `.apply(`)", solution: "Remove the apply: leaves never turn a diff into a snapshot; callers use `protocol::apply_diff`, which alone mints the `ApplyCapability`." },
  R10: { code: "V1-SNAPSHOT-DIFF", slug: "leaf-between", reason: "a diff is built by differencing or simulating snapshots (`between`, `*_replacing`, `state_after`, `value_diff*`, …)", solution: "Name the changed entities and fields with their new values instead; `between` is for sync and import only." },
  R11: { code: "V2-DIFF-DERIVED-INVERSE", slug: "diff-derived-inverse", reason: "an inverse is derived from the forward diff or a simulation of it (`diff(`, `negative`, `state_after`, …)", solution: "Build the inverse mutations from the payload and reads of `base` inside the leaf's own inverse, as absolute setters of the base values." },
  R12: { code: "V1-SNAPSHOT-DIFF", slug: "base-clone-diff", reason: "a fn copies a reference parameter (`clone`/`to_vec`/`to_owned`/`iter().cloned().collect()`) into a mutable binding", solution: "Read `base` and emit the sparse diff rows directly; never edit a copy of the snapshot and difference it." },
  R13: { code: "V1-SNAPSHOT-DIFF", slug: "whole-state-diff", reason: "the diff type is the whole snapshot it applies to", solution: "Replace it with a sparse typed diff (keyed added/removed/modified rows) with a concrete `DiffAlgebra::inverse`." },
  R14: { code: "V2-RESTORE-INVERSE", slug: "restore-inverse", reason: "an inverse returns a whole-snapshot or whole-record restore (`Restore…`, `Replace…`, `*Snapshot` variants, `*_state`/`*_content` fields)", solution: "Return the concrete mutations that put each touched value back to its exact base value." },
  R15: { code: "V4-LAW-UNTESTED", slug: "inverse-sum-law-untested", reason: "no test under the leaf's own `🧪️tests` calls `assert_mutation_inverse_sum_law`", solution: "Add a test under the leaf's `🧪️tests` that calls `assert_mutation_inverse_sum_law(&mutation, &before)` on each applied fixture." },
  R16: { code: "V3-LEAF-APPLY", slug: "outcome-apply-to", reason: "`MutationOutcome::apply_to` is gone: a diff is applied only by the central applier", solution: "Call `protocol::apply_diff(outcome.diff(), &base)` instead." },
};

/** 🧰️`MutationDiff` methods that are the central applier's own machinery, never diff or inverse logic: the only fns of a diff type exempt from the diff-only rules. */
const POLICY_DIFF_ONLY_DIFF_TYPE_MACHINERY: ReadonlySet<string> = new Set(["apply", "absorb", "retire_cold", "retire_projection", "is_empty"]);
/** 🔎️Raw cue that a file may hold a hand-written `impl … MutationKind<` or `impl … Mutation<` block. */
const POLICY_DIFF_ONLY_IMPL_PROBE_RE = /\bimpl\b[^{;]*\bMutation(?:Kind)?\s*</;
/** 🔎️Raw cue that a file may hold an `impl … MutationDiff<` block. */
const POLICY_DIFF_ONLY_DIFF_IMPL_PROBE_RE = /\bimpl\b[^{;]*\bMutationDiff\s*</;
const POLICY_RUST_RAW_STRING_RE = /b?r(#*)"/y;
const POLICY_RUST_TEST_ITEM_RE = /#\[\s*(?:cfg\s*\(\s*test\s*\)|test)\s*\]/g;
const POLICY_RUST_FN_RE = /\bfn\s+([A-Za-z_]\w*)/g;
const POLICY_RUST_IMPL_RE = /\bimpl\b/g;
const POLICY_RUST_USE_RE = /\buse\s[^;]*;/g;
const POLICY_DIFF_ONLY_APPLY_RE = /(?:\.|::)apply\s*\(|(?<!\bfn\s+)\bapply_diff\s*\(|\bApplyCapability\b/g;
/** 🔎️Snapshot-differencing and simulation helpers under any spelling: `*between*`, `*_replacing`, `*_state_diff`, `state_after*`, `value_diff*`. */
const POLICY_DIFF_ONLY_BETWEEN_RE = /(?<!\bfn\s+)(?<!\w)(?:\w*between\w*|\w*_replacing|\w*_state_diff|state_after\w*|value_diff\w*)\s*(?:::\s*<[^>(]*>)?\s*\(/g;
/** 🔎️Fns that build diff-like or whole-state values from the forward payload: negative simulators and derived-inverse helpers. */
const POLICY_DIFF_ONLY_SIMULATOR_RE = /(?<!\bfn\s+)(?<!\w)(?:negative|state_after\w*|inverse_of|value_diff\w*)\s*\(|\.negat(?:e|ion)\w*\s*\(/g;
/** 🔎️Raw cue that a plugin file defines a fn named like a diff or inverse builder. */
const POLICY_DIFF_ONLY_DIFFISH_FN_PROBE_RE = /\bfn\s+\w*(?:diff|inverse|negative|negation|replacing|state_after)\w*/;
const POLICY_DIFF_ONLY_DIFFISH_NAME_RE = /(?:^|_)(?:diff|inverse|negative|negation|replacing)(?:_|$)|state_after/;
const POLICY_DIFF_ONLY_INVERSEISH_NAME_RE = /inverse|negative|negation/;
const POLICY_DIFF_ONLY_PAYLOAD_PARAM_RE = /^(?:payload|mutation|op|edit|self)$/;
const POLICY_DIFF_ONLY_MUT_REF_RE = /&\s*mut\b/g;
/** 🔎️A `&mut` parameter or value typed as a whole artifact state: the one `&mut` a non-builder helper must never take. */
const POLICY_DIFF_ONLY_MUT_STATE_RE = /&\s*mut\s+(?:\w+::)*\w*(?:Snapshot|Document|State)\b/g;
/** 🚪️Infrastructure directories (and any directory ending `-internals`): io codecs, retained prepare state machines, encoders and engine internals are not diff logic and may apply diffs. */
const POLICY_DIFF_ONLY_INFRASTRUCTURE_DIRS: ReadonlySet<string> = new Set(["🚪️io", "🎮️prepare", "📦️codec"]);
const POLICY_DIFF_ONLY_DIFF_CALL_RE = /(?<!\w)diff\s*\(/g;
const POLICY_DIFF_ONLY_INVERSE_HELPER_RE = /(?<![\w.])(\w*_inverse|\w*inverse_\w*)\s*\(/g;
const POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE = /\b\w*[dD]iff\w*\b|\boutcome\b/;
const POLICY_DIFF_ONLY_RESTORE_RE = /\b(?:SetSnapshot|PatchSnapshot|ReplaceDocument|ReplaceSnapshot|ReplaceConfig|ReplaceState|SetDocument|Restore\w*)\b|\b(?:Self|[A-Z]\w*)::\w*Snapshot\b|(?<!\blet\s)(?<!\bmut\s)\b\w+_(?:state|content|snapshot|document)\s*:(?!:)/g;
const POLICY_DIFF_ONLY_APPLY_TO_RE = /(?:\.|::)apply_to\s*\(/g;
const POLICY_DIFF_ONLY_TYPE_DIFF_RE = /\btype\s+Diff\s*=\s*([^;]+);/;

/** 🩻️`content` with comments, string literals, raw strings and char literals blanked to spaces — same length, newlines kept — so every index and line number still addresses the original source. */
function policyMaskRustSource(content: string): string {
  const size = content.length;
  const pieces: string[] = [];
  let kept = 0;
  let index = 0;
  const blank = (from: number, to: number) => {
    pieces.push(content.slice(kept, from), content.slice(from, to).replace(/[^\n]/g, " "));
    kept = to;
    index = to;
  };
  while (index < size) {
    const ch = content[index]!;
    const next = content[index + 1];
    if (ch === "/" && next === "/") {
      const eol = content.indexOf("\n", index);
      blank(index, eol === -1 ? size : eol);
    } else if (ch === "/" && next === "*") {
      let depth = 1;
      let cursor = index + 2;
      while (cursor < size && depth > 0) {
        const pair = content.slice(cursor, cursor + 2);
        if (pair === "/*") {
          depth++;
          cursor += 2;
        } else if (pair === "*/") {
          depth--;
          cursor += 2;
        } else cursor++;
      }
      blank(index, cursor);
    } else if (ch === '"') {
      let cursor = index + 1;
      while (cursor < size && content[cursor] !== '"') cursor += content[cursor] === "\\" ? 2 : 1;
      blank(index, Math.min(cursor + 1, size));
    } else if ((ch === "r" || ch === "b") && !/\w/.test(content[index - 1] ?? " ")) {
      POLICY_RUST_RAW_STRING_RE.lastIndex = index;
      const raw = POLICY_RUST_RAW_STRING_RE.exec(content);
      if (raw) {
        const terminator = `"${raw[1]}`;
        const end = content.indexOf(terminator, index + raw[0].length);
        blank(index, end === -1 ? size : end + terminator.length);
      } else index++;
    } else if (ch === "'") {
      if (next === "\\") {
        const end = content.indexOf("'", index + 3);
        blank(index, end === -1 ? index + 1 : end + 1);
      } else {
        const width = (content.codePointAt(index + 1) ?? 0) > 0xffff ? 2 : 1;
        if (content[index + 1 + width] === "'") blank(index, index + 2 + width);
        else index++;
      }
    } else index++;
  }
  return pieces.join("") + content.slice(kept);
}

/** 🪢️For every `{` of `masked` the index of its matching `}` (`-1` when unmatched); braces inside masked comments and literals never count. */
function policyRustBraceClose(masked: string): Int32Array {
  const close = new Int32Array(masked.length).fill(-1);
  const stack: number[] = [];
  for (let index = 0; index < masked.length; index++) {
    const ch = masked.charCodeAt(index);
    if (ch === 123) stack.push(index);
    else if (ch === 125) {
      const open = stack.pop();
      if (open !== undefined) close[open] = index;
    }
  }
  return close;
}

/** 🧽️`masked` with every `#[cfg(test)]`/`#[test]` item blanked: unit-test modules and test functions are not mutation leaves. */
function policyRustBlankTestItems(masked: string, close: Int32Array): string {
  const pieces: string[] = [];
  let kept = 0;
  for (const match of masked.matchAll(POLICY_RUST_TEST_ITEM_RE)) {
    if (match.index < kept) continue;
    let cursor = match.index + match[0].length;
    while (cursor < masked.length && masked[cursor] !== "{" && masked[cursor] !== ";") cursor++;
    const end = masked[cursor] === "{" && close[cursor]! >= 0 ? close[cursor]! + 1 : Math.min(cursor + 1, masked.length);
    pieces.push(masked.slice(kept, match.index), masked.slice(match.index, end).replace(/[^\n]/g, " "));
    kept = end;
  }
  return pieces.join("") + masked.slice(kept);
}

/** ➰️The index just past the `>` closing the `<` at `open` (a `->` arrow never closes). */
function policyRustSkipAngles(text: string, open: number): number {
  let depth = 0;
  for (let index = open; index < text.length; index++) {
    const ch = text[index];
    if (ch === "<") depth++;
    else if (ch === ">" && text[index - 1] !== "-" && --depth === 0) return index + 1;
  }
  return text.length;
}

/** 🔗️The index of the `)` matching the `(` at `open`. */
function policyRustParenEnd(text: string, open: number): number {
  let depth = 0;
  for (let index = open; index < text.length; index++) {
    if (text[index] === "(") depth++;
    else if (text[index] === ")" && --depth === 0) return index;
  }
  return text.length;
}

/** ✂️Splits `text` at its top-level commas (angle, paren, bracket and brace depth zero). */
function policyRustSplitTop(text: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let from = 0;
  for (let index = 0; index < text.length; index++) {
    const ch = text[index]!;
    if (ch === "<" || ch === "(" || ch === "[" || ch === "{") depth++;
    else if (ch === ")" || ch === "]" || ch === "}" || (ch === ">" && text[index - 1] !== "-")) depth--;
    else if (ch === "," && depth === 0) {
      parts.push(text.slice(from, index));
      from = index + 1;
    }
  }
  parts.push(text.slice(from));
  return parts.map((part) => part.trim()).filter(Boolean);
}

type PolicyRustFn = Readonly<{ name: string; start: number; bodyStart: number; end: number; baseParam: string; refParams: readonly string[] }>;
type PolicyRustImplKind = "mutation" | "diff" | "algebra" | "outcome" | "other";
type PolicyRustImpl = Readonly<{ kind: PolicyRustImplKind; args: readonly string[]; self: string; traitImpl: boolean; start: number; bodyStart: number; end: number }>;

/** 🔩️Every `fn` of `masked` that has a body, with the name of its last parameter (the `base` of a `diff`/`inverse`). */
function policyRustFns(masked: string, close: Int32Array): PolicyRustFn[] {
  const fns: PolicyRustFn[] = [];
  const skipSpace = (from: number) => {
    let cursor = from;
    while (cursor < masked.length && /\s/.test(masked[cursor]!)) cursor++;
    return cursor;
  };
  for (const match of masked.matchAll(POLICY_RUST_FN_RE)) {
    let cursor = skipSpace(match.index + match[0].length);
    if (masked[cursor] === "<") cursor = skipSpace(policyRustSkipAngles(masked, cursor));
    if (masked[cursor] !== "(") continue;
    const paramsEnd = policyRustParenEnd(masked, cursor);
    let open = paramsEnd + 1;
    let depth = 0;
    for (; open < masked.length; open++) {
      const ch = masked[open];
      if (ch === "(") depth++;
      else if (ch === ")") depth--;
      else if (depth === 0 && (ch === "{" || ch === ";")) break;
    }
    const end = masked[open] === "{" ? close[open]! : -1;
    if (end < 0) continue;
    const params = policyRustSplitTop(masked.slice(cursor + 1, paramsEnd));
    const last = params.at(-1) ?? "";
    const refParams = params.flatMap((param) => {
      const named = /^(?:mut\s+)?([A-Za-z_]\w*)\s*:\s*(&(?!\s*mut\b))?/.exec(param);
      return named?.[2] && !/\w*(?:Diff|Delta|Patch|Mutation)\w*/.test(param.slice(param.indexOf(":"))) ? [named[1]!] : [];
    });
    fns.push({ name: match[1]!, start: match.index, bodyStart: open, end, baseParam: /^(?:mut\s+)?([A-Za-z_]\w*)\s*:/.exec(last)?.[1] ?? "base", refParams });
  }
  return fns;
}

/** 🧩️Every item-level `impl` block of `masked` with the trait it implements, that trait's generic arguments and the implementing type. */
function policyRustImpls(masked: string, close: Int32Array): PolicyRustImpl[] {
  const impls: PolicyRustImpl[] = [];
  for (const match of masked.matchAll(POLICY_RUST_IMPL_RE)) {
    let before = match.index - 1;
    while (before >= 0 && /\s/.test(masked[before]!)) before--;
    if (before >= 0 && !";}{]".includes(masked[before]!)) continue;
    let open = match.index;
    while (open < masked.length && masked[open] !== "{" && masked[open] !== ";") open++;
    const end = masked[open] === "{" ? close[open]! : -1;
    if (end < 0) continue;
    const header = masked.slice(match.index + 4, open).replace(/\s+/g, " ").trim();
    let cursor = 0;
    if (header[0] === "<") cursor = policyRustSkipAngles(header, 0);
    const rest = header.slice(cursor).replace(/^\s+/, "").replace(/^!/, "");
    const trait = /^([\w:]+)\s*</.exec(rest);
    let kind: PolicyRustImplKind = "other";
    let args: string[] = [];
    let self = "";
    if (trait) {
      const name = trait[1]!.split("::").pop()!;
      const angleOpen = trait[0].length - 1;
      const angleEnd = policyRustSkipAngles(rest, angleOpen);
      args = policyRustSplitTop(rest.slice(angleOpen + 1, angleEnd - 1));
      self = /^\s*for\s+(.+?)(?:\s+where\b.*)?$/.exec(rest.slice(angleEnd))?.[1] ?? "";
      kind = name === "MutationKind" || name === "Mutation" ? "mutation" : name === "MutationDiff" ? "diff" : name === "DiffAlgebra" ? "algebra" : name === "MutationOutcome" && self === "" ? "outcome" : "other";
    }
    impls.push({ kind, args, self, traitImpl: /(?:^|[\s>])for\s/.test(rest), start: match.index, bodyStart: open, end });
  }
  return impls;
}

/** 🏷️The last path segment of a Rust type with its generics dropped (`protocol::Foo<Bar>` → `Foo`). */
function policyRustTypeName(type: string): string {
  return type.replace(/<.*$/s, "").replace(/^[&\s]+/, "").trim().split("::").pop()!.trim();
}

/** 📍️Maps a character index of `content` to its 1-based line by binary search over the line starts. */
function policyRustLineLookup(content: string): (index: number) => number {
  const starts = [0];
  for (let at = content.indexOf("\n"); at !== -1; at = content.indexOf("\n", at + 1)) starts.push(at + 1);
  return (index) => {
    let low = 0;
    let high = starts.length - 1;
    while (low < high) {
      const mid = (low + high + 1) >> 1;
      if (starts[mid]! <= index) low = mid;
      else high = mid - 1;
    }
    return low + 1;
  };
}

/** 🧾️One breach record of a diff-only rule: `file:line rule:code reason (token)` is the summary the gate prints. */
function policyDiffOnlyBreach(rule: PolicyDiffOnlyRuleId, relPath: string, line: number, token: string): BreachRecord {
  const spec = POLICY_DIFF_ONLY_RULES[rule];
  return {
    id: `diff-only-${rule.toLowerCase()}-${relPath}-${line}`,
    summary: `${relPath}:${line} ${rule}:${spec.code} ${spec.reason} (\`${token}\`)`,
    kind: `diff-only-mutation/${spec.slug}`,
    scope: relPath,
    line,
    priority: "high",
    reason: `${spec.code}: ${spec.reason}.`,
    solution: spec.solution,
  };
}

/**
 * 🔬️Rules R8–R14 and R16 over ONE Rust file, pure. Comments, literals and `#[cfg(test)]` items never count.
 * Scope: non-test files under `🧬️mutations`, under `🧬️mutation-support` and under any `🔺️diff`/`↩️inverse` directory of a
 * `🧬️schema` are scanned in EVERY fn (except under the infrastructure directories `🚪️io`, `🎮️prepare`, `📦️codec`, which are treated like any other file); other non-test files holding an `impl … Mutation<`/`MutationKind<`/`MutationDiff<`/
 * `DiffAlgebra<` block are scanned in the fns inside those impls and in fns named like a diff or inverse builder
 * (`*diff*`, `*inverse*`, `negative`, `*_replacing`, `state_after`), so artifact-root helpers are covered too. R8 flags any `&mut`
 * in a fn named like a builder (`diff`, `*inverse*`, …) and, in every other scanned fn, only a `&mut` typed `…Snapshot`/`…Document`/`…State`. Never scanned:
 * fns of non-mutation trait impls (codecs, `Default`, …), fns named `*between*` (sync machinery) and the diff type's own
 * `apply`/`absorb`/`retire_*`/`is_empty` — plus every fn outside `🧬️mutations` that receives an `ApplyCapability` and, in a file that implements a
 * diff type outside `🧬️mutations`, the `*apply*` helpers its `apply` calls. A diff type's own `inverse` legitimately reads
 * its rows: R11 flags only simulators (`negative`, `state_after`, …) there, and `diff(`/`*_inverse(…diff…)` only on the
 * mutation side. Every `DiffAlgebra::inverse` body IS scanned. R9 additionally covers the whole
 * file under any `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` directory. R13's `impl MutationDiff<X> for X` is judged in
 * every non-test Rust file. R13 and R14 apply to every lane and mutation type, including presence and transient roots:
 * diffs remain sparse and inverses restore touched values through concrete mutations. R16 scans every Rust file.
 * The planted-violation law (`🧪️tests/🧪️diff-only-law-gate`) drives exactly this function.
 */
export function policyDiffOnlyFileBreaches(relPath: string, content: string): BreachRecord[] {
  if (!relPath.endsWith(".rs")) return [];
  const directories = relPath.split("/").slice(0, -1);
  const underMutations = directories.includes(POLICY_MUTATIONS_FACET);
  const underSchema = directories.includes("🧬️schema");
  const underSupport = directories.includes("🧬️mutation-support");
  const testPath = directories.some((segment) => segment === "🧪️tests" || segment === "🧫️fixtures");
  const infrastructure = directories.some((segment) => POLICY_DIFF_ONLY_INFRASTRUCTURE_DIRS.has(segment) || segment.endsWith("-internals"));
  const everyFn = !testPath && !infrastructure && (underMutations || underSupport || (underSchema && directories.some((segment) => segment === "🔺️diff" || segment === "↩️inverse")));
  const leafCandidate = !testPath && (everyFn || underSchema || POLICY_DIFF_ONLY_IMPL_PROBE_RE.test(content) || POLICY_DIFF_ONLY_DIFF_IMPL_PROBE_RE.test(content) || (relPath.startsWith("✏️s/") && relPath.includes("/🗿️artifacts/") && POLICY_DIFF_ONLY_DIFFISH_FN_PROBE_RE.test(content)));
  const applyToCandidate = content.includes("apply_to");
  if (!leafCandidate && !applyToCandidate) return [];
  const all = policyMaskRustSource(content);
  const close = policyRustBraceClose(all);
  const lineOf = policyRustLineLookup(content);
  const breaches: BreachRecord[] = [];
  const seen = new Set<string>();
  const add = (rule: PolicyDiffOnlyRuleId, index: number, token: string) => {
    const line = lineOf(index);
    const key = `${rule}|${line}`;
    if (seen.has(key)) return;
    seen.add(key);
    breaches.push(policyDiffOnlyBreach(rule, relPath, line, token.replace(/\s+/g, " ").trim()));
  };
  if (applyToCandidate) {
    for (const match of all.matchAll(POLICY_DIFF_ONLY_APPLY_TO_RE)) add("R16", match.index, match[0]);
    if (all.includes("MutationOutcome"))
      for (const outcome of policyRustImpls(all, close).filter((impl) => impl.kind === "outcome"))
        for (const match of all.slice(outcome.bodyStart, outcome.end).matchAll(/\bfn\s+apply_to\b/g)) add("R16", outcome.bodyStart + match.index, match[0]);
  }
  if (!leafCandidate) return breaches;
  const code = policyRustBlankTestItems(all, close);
  const impls = policyRustImpls(code, close);
  for (const impl of impls)
    if (impl.kind === "diff" && impl.args.length === 1 && policyRustTypeName(impl.args[0]!) === policyRustTypeName(impl.self)) add("R13", impl.start, `impl MutationDiff<${policyRustTypeName(impl.self)}> for ${policyRustTypeName(impl.self)}`);
  const mutationImpls = impls.filter((impl) => impl.kind === "mutation");
  const hasDiffImpl = impls.some((impl) => impl.kind === "diff");
  const within = (spans: readonly { bodyStart: number; end: number }[], index: number) => spans.some((span) => span.bodyStart <= index && index <= span.end);
  const innermost = (index: number) => impls.filter((impl) => impl.bodyStart <= index && index <= impl.end).sort((a, b) => b.bodyStart - a.bodyStart)[0];
  const classified = policyRustFns(code, close).map((fn) => {
    const impl = innermost(fn.start);
    const logicImpl = impl !== undefined && (impl.kind === "mutation" || impl.kind === "diff" || impl.kind === "algebra");
    const machinery = impl !== undefined && (impl.kind === "diff" || impl.kind === "algebra") && POLICY_DIFF_ONLY_DIFF_TYPE_MACHINERY.has(fn.name);
    const applier = (impl?.kind === "diff" && fn.name === "apply") || (hasDiffImpl && !underMutations && fn.name.includes("apply")) || (!underMutations && code.slice(fn.start, fn.bodyStart).includes("ApplyCapability"));
    const exempt = machinery || applier || fn.name.includes("between") || (impl !== undefined && impl.kind === "other" && impl.traitImpl);
    return { fn, applier, scanned: !exempt && (everyFn || logicImpl || (!infrastructure && POLICY_DIFF_ONLY_DIFFISH_NAME_RE.test(fn.name))) };
  });
  const scanned = classified.filter((row) => row.scanned).map((row) => row.fn);
  const appliers = classified.filter((row) => row.applier).map((row) => ({ bodyStart: row.fn.start, end: row.fn.end }));
  const inverseFns = scanned.filter((fn) => POLICY_DIFF_ONLY_INVERSEISH_NAME_RE.test(fn.name));
  const diffTypeSide = (fn: PolicyRustFn) => {
    const impl = innermost(fn.start);
    return (impl !== undefined && (impl.kind === "diff" || impl.kind === "algebra")) || (underSchema && !underMutations);
  };
  const scannedSpans = scanned.map((fn) => ({ bodyStart: fn.start, end: fn.end }));
  for (const fn of scanned) {
    const builder = fn.name === "diff" || POLICY_DIFF_ONLY_DIFFISH_NAME_RE.test(fn.name);
    for (const match of code.slice(fn.start, fn.end + 1).matchAll(builder ? POLICY_DIFF_ONLY_MUT_REF_RE : POLICY_DIFF_ONLY_MUT_STATE_RE)) add("R8", fn.start + match.index, "&mut");
  }
  const applyEverywhere = !infrastructure && (underMutations || underSchema || underSupport);
  const withoutUses = code.replace(POLICY_RUST_USE_RE, (statement) => statement.replace(/[^\n]/g, " "));
  for (const match of withoutUses.matchAll(POLICY_DIFF_ONLY_APPLY_RE))
    if (!within(appliers, match.index) && (applyEverywhere || within(scannedSpans, match.index) || within(mutationImpls, match.index))) add("R9", match.index, match[0]);
  for (const match of code.matchAll(POLICY_DIFF_ONLY_BETWEEN_RE)) if (within(scannedSpans, match.index)) add("R10", match.index, match[0]);
  for (const fn of inverseFns) {
    const body = code.slice(fn.bodyStart, fn.end + 1);
    for (const match of body.matchAll(POLICY_DIFF_ONLY_SIMULATOR_RE)) add("R11", fn.bodyStart + match.index, match[0]);
    if (!diffTypeSide(fn)) {
      for (const match of body.matchAll(POLICY_DIFF_ONLY_DIFF_CALL_RE)) add("R11", fn.bodyStart + match.index, match[0]);
      for (const match of body.matchAll(POLICY_DIFF_ONLY_INVERSE_HELPER_RE)) {
        const open = match.index + match[0].length - 1;
        const args = body.slice(open + 1, policyRustParenEnd(body, open));
        if (/diff/i.test(match[1]!) || POLICY_DIFF_ONLY_DIFF_ARGUMENT_RE.test(args)) add("R11", fn.bodyStart + match.index, `${match[1]}(`);
      }
    }
    for (const match of body.matchAll(POLICY_DIFF_ONLY_RESTORE_RE)) add("R14", fn.bodyStart + match.index, match[0]);
  }
  for (const fn of scanned) {
    const roots = fn.refParams.filter((name) => !POLICY_DIFF_ONLY_PAYLOAD_PARAM_RE.test(name));
    if (roots.length === 0) continue;
    const body = code.slice(fn.bodyStart, fn.end + 1);
    if (!body.includes("let mut")) continue;
    const copy = new RegExp(`\\blet\\s+mut\\s+\\w+\\s*(?::[^=;]*)?=\\s*(?:\\(\\s*\\*\\s*)?(?:${roots.join("|")})\\b[^;{}]*?(?:\\.(?:clone|to_owned|to_vec)\\s*\\(\\s*\\)|\\.iter\\s*\\(\\s*\\)\\s*\\.cloned\\s*\\(\\s*\\)\\s*\\.collect[^;]*)\\s*\\)?\\s*;`, "g");
    for (const match of body.matchAll(copy)) add("R12", fn.bodyStart + match.index, "let mut … = <ref param>….clone()");
  }
  for (const impl of mutationImpls) {
    const declared = POLICY_DIFF_ONLY_TYPE_DIFF_RE.exec(code.slice(impl.bodyStart, impl.end));
    if (!declared) continue;
    const name = policyRustTypeName(declared[1]!);
    if (name === "Self" || name === policyRustTypeName(impl.self) || (impl.args[0] !== undefined && name === policyRustTypeName(impl.args[0]))) add("R13", impl.bodyStart + declared.index, `type Diff = ${name}`);
  }
  return breaches;
}

/**
 * 🧪️Rule R15, pure: every direct mutation leaf (a directory below a `🧬️mutations` directory that owns a `🔺️diff` and no
 * `🧩️plan`) needs a Rust test under its OWN `🧪️tests` that calls the framework's `assert_mutation_inverse_sum_law`
 * (L3: the inverse diffs sum to the negative diff). `files` is the git-visible inventory, `read` returns a file's content.
 */
export function policyInverseSumLawUntestedLeafBreaches(files: readonly string[], read: (relPath: string) => string): BreachRecord[] {
  const leaves = new Map<string, string>();
  const composites = new Set<string>();
  const tested = new Set<string>();
  for (const relPath of files) {
    const segments = relPath.split("/");
    const facet = segments.indexOf(POLICY_MUTATIONS_FACET);
    if (facet === -1) continue;
    const planAt = segments.indexOf(POLICY_MUTATION_PLAN_DIR, facet + 1);
    if (planAt !== -1) composites.add(segments.slice(0, planAt).join("/"));
    const testsAt = segments.indexOf("🧪️tests", facet + 1);
    if (testsAt !== -1) {
      if (relPath.endsWith(".rs") && read(relPath).includes("assert_mutation_inverse_sum_law")) tested.add(segments.slice(0, testsAt).join("/"));
      continue;
    }
    const diffAt = segments.indexOf("🔺️diff", facet + 1);
    if (diffAt !== -1 && relPath.endsWith(".rs")) {
      const leaf = segments.slice(0, diffAt).join("/");
      if (!leaves.has(leaf)) leaves.set(leaf, relPath);
    }
  }
  return [...leaves]
    .filter(([leaf]) => !composites.has(leaf) && !tested.has(leaf))
    .map(([leaf, diffRel]) => policyDiffOnlyBreach("R15", diffRel, 1, `${leaf.split("/").at(-1)}/🧪️tests`));
}

const POLICY_DIFF_ONLY_SCANS = new Map<string, Map<string, BreachRecord[]>>();

/** 🗂️The per-file rules over the shared inventory's Rust sources, scanned ONCE per root and grouped by rule slug. */
function policyDiffOnlyScan(repoRoot: string): Map<string, BreachRecord[]> {
  const cached = POLICY_DIFF_ONLY_SCANS.get(repoRoot);
  if (cached) return cached;
  const byKind = new Map<string, BreachRecord[]>();
  for (const relPath of policyMutationLawRustFiles(repoRoot))
    for (const breach of policyDiffOnlyFileBreaches(relPath, policyReadFileSafe(repoRoot, relPath))) {
      const rows = byKind.get(breach.kind);
      if (rows) rows.push(breach);
      else byKind.set(breach.kind, [breach]);
    }
  POLICY_DIFF_ONLY_SCANS.set(repoRoot, byKind);
  return byKind;
}

/** 📏️Rule R8: no `&mut` in the signature or body of any scanned diff, inverse or helper fn (L4). */
export function policyLeafMutableBaseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R8.slug}`) ?? [];
}

/** 📏️Rule R9: no `.apply(`, `apply_diff(` or `ApplyCapability` anywhere under `🧬️schema`/`🧬️mutations`/`🧬️mutation-support` or in scanned fns (L4/L5); only a diff type's own `apply` (and the `*apply*` helpers it calls) may forward its capability. */
export function policyLeafCentralApplyBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R9.slug}`) ?? [];
}

/** 📏️Rule R10: no `*between*`, `*_replacing`, `*_state_diff`, `state_after*` or `value_diff*` call in a scanned fn (L1: the diff is declarative, never a snapshot difference or simulation). */
export function policyLeafBetweenBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R10.slug}`) ?? [];
}

/** 📏️Rule R11: an inverse never calls `diff(`, `.diff()`, `negative`, `state_after`, `inverse_of`, `value_diff*` or a `*_inverse(…diff…)` helper (L2/AMB-3). */
export function policyDiffDerivedInverseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R11.slug}`) ?? [];
}

/** 📏️Rule R12: no scanned fn copies a reference parameter into a mutable binding (L1). */
export function policyBaseCloneDiffBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R12.slug}`) ?? [];
}

/** 📏️Rule R13: the diff type is never the snapshot it applies to (`type Diff = <snapshot>`, `impl MutationDiff<X> for X`) (L1). */
export function policyWholeStateDiffBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R13.slug}`) ?? [];
}

/** 📏️Rule R14: an inverse never returns a `SetSnapshot`/`PatchSnapshot`/`Replace…`/`Restore…`/`*Snapshot` variant nor a whole `*_state`/`*_content` record (L2). */
export function policyRestoreInverseBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R14.slug}`) ?? [];
}

/** 📏️Rule R15: every direct mutation leaf has a test under its own `🧪️tests` calling `assert_mutation_inverse_sum_law` (L3). */
export function policyInverseSumLawUntestedBreaches(repoRoot: string): BreachRecord[] {
  return policyInverseSumLawUntestedLeafBreaches(policyMutationLawInventory(repoRoot).files, (relPath) => policyReadFileSafe(repoRoot, relPath));
}

/** 📏️Rule R16: `MutationOutcome::apply_to` and `.apply_to(` are gone from every Rust file (L4). */
export function policyOutcomeApplyToBreaches(repoRoot: string): BreachRecord[] {
  return policyDiffOnlyScan(repoRoot).get(`diff-only-mutation/${POLICY_DIFF_ONLY_RULES.R16.slug}`) ?? [];
}

/** ⚖️Aggregates this ticket's 7 mutation-outcome / merge-policy / no-CRDT / no-validate / derive-glue gates plus the nine diff-only mutation rules (R8–R16) over ONE shared inventory (`policyMutationLawInventory`) — the bundle both `policy` (below) and `VerifyScript.runGate`/`verify mutation-outcome-law` share. */
export function policyMutationOutcomeMergePolicyBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyMutationOutcomeBreaches(repoRoot),
    ...policyMutationMessageCodeBreaches(repoRoot),
    ...policyNoCrdtVocabularyBreaches(repoRoot),
    ...policyNoValidateOverrideBreaches(repoRoot),
    ...policySeverityInfoBreaches(repoRoot),
    ...policyMergePolicyParityBreaches(repoRoot),
    ...policyDeriveGlueMountBreaches(repoRoot),
    ...policyLeafMutableBaseBreaches(repoRoot),
    ...policyLeafCentralApplyBreaches(repoRoot),
    ...policyLeafBetweenBreaches(repoRoot),
    ...policyDiffDerivedInverseBreaches(repoRoot),
    ...policyBaseCloneDiffBreaches(repoRoot),
    ...policyWholeStateDiffBreaches(repoRoot),
    ...policyRestoreInverseBreaches(repoRoot),
    ...policyInverseSumLawUntestedBreaches(repoRoot),
    ...policyOutcomeApplyToBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleMutationOutcomeMergePolicy
