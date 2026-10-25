import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { policyMutationLawSourceFiles, policyMutationLawInventory, policyMutationLawRustFiles, policyMutationMessageCodeBreaches, policyMutationOutcomeBreaches } from "../🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLineOfIndex } from "../../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_MUTATIONS_FACET } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyExtractFnBody } from "../../🏛️foundation/🟦️.ts";
import { policyHostApplyBreaches, policyRowEmitterBreaches, policyBaseCloneDiffBreaches, policyDiffDerivedInverseBreaches, policyInverseSumLawUntestedBreaches, policyLeafBetweenBreaches, policyLeafCentralApplyBreaches, policyLeafMutableBaseBreaches, policyOutcomeApplyToBreaches, policyRestoreInverseBreaches, policyWholeStateDiffBreaches } from "../🧪️diff-only-law/🟦️.ts";

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

/** ✅️A `fn validate(&self, ..)` override — the exact shape C4/C10 delete from `Mutation`/`MutationKind`/`CompositeMutationKind`. */
const POLICY_VALIDATE_OVERRIDE_RE = /\bfn\s+validate\s*\(\s*&self\b/g;

/** 🧱️An `impl (protocol::)?(Composite)?MutationKind<..> for X {` block opener, to scope the second half of rule 4. */
const POLICY_MUTATION_KIND_IMPL_RE = /\bimpl\b[^\n{]*\b(?:CompositeMutationKind|MutationKind)\s*</g;

/**
 * 🚫️Rule 4: no `fn validate(&self, ..)` survives — not inside a `🧬️mutations/**` file, and not inside
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

/** 💡️The inventory's source files (`.rs`/`.ts`/`.tsx`) repo-wide — the canonical root router, whose policy literals must not scan themselves, is already out. */


const POLICY_SEVERITY_HINT_STRUCT_RE = /\bSeverity(?:::|\.)Hint\b/g;

/**
 * ℹ️Rule 5: zero `Severity::Hint`/`Severity.Hint` (C1 declaration order is now `Info, Warning, Error,
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

/** 🔀️The 3 frozen MergePolicy variants (C3) spelled at exactly these 4 surfaces. */
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
  { label: "i18n bundles (en+de, ⚛️react/🌐️i18n)", relPath: "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🤝️collaboration/🟦️.ts" },
];

/**
 * 🪞️Rule 6: the three `MergePolicy` variants must be present in all 4 surfaces that spell them (C3/C9),
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

/** 🧬️The glue's one mount of the owner implementation. */
const POLICY_DERIVE_GLUE_MOUNT_RE = /#\[path\s*=\s*"\.\.\/\.\.\/🦀️\.rs"\]\s*mod\s+component\s*;/;

/** 🔩️Rule 7: the dsl derive macro has ONE implementation — the proc-macro package glue mounts the owner `🦀️.rs` (`#[path = "../../🦀️.rs"] mod component;`) and only forwards to it, so no build shape can run a stale copy of the derive logic. */
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
      solution: `Make ${POLICY_DERIVE_GLUE} mount the owner through a #[path] attribute naming "../../🦀️.rs" on mod component; and forward each #[proc_macro_derive] entry to component::expand_*; move any expand_* body into ${POLICY_DERIVE_OWNER}.`,
    },
  ];
}

/** ⚖️Aggregates this ticket's 7 mutation-outcome / merge-policy / no-CRDT / no-validate / derive-glue gates plus the eleven diff-only mutation rules (R8–R18) over ONE shared inventory (`policyMutationLawInventory`) — the bundle both `policy` (below) and `VerifyScript.runGate`/`verify mutation-outcome-law` share. */
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
    ...policyHostApplyBreaches(repoRoot),
    ...policyRowEmitterBreaches(repoRoot),
  ];
}
