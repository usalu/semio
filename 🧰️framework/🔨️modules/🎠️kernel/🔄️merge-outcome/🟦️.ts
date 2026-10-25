/** 🔄️ Merge outcomes: merge policy and conflict resolution with their wire order, mutation messages, apply errors, conflicts and dispatch and merge reports.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type Severity } from "../🚨️fault/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️MergeOutcome
/** ⚖️ How strict an authority is about accepting a `MutationOutcome` whose messages reach a
 * given {@link Severity} — TS twin of Rust `MergePolicy` (`📡️spr/🧾️wire/🦀️.rs` region
 * `🔖️Policies`). Declaration order IS `as_u8`/`from_u8`'s 0..2 (`LaissezFaire, Normal, Vigilant`).
 * Unlike {@link Severity}, Rust's `MergePolicy` carries no `#[serde(rename_all)]`, so its
 * pack-decoded JSON form (`MergeReport.policy`/`DispatchReport.policy`) is the bare Rust variant
 * name, not camelCased. Local/authority state only — never carried on a `MutationEnvelope`/
 * `BackboneMessage`, never part of an artifact's shared history. */
export type MergePolicy = "LaissezFaire" | "Normal" | "Vigilant";

/** 🏠️ `#[default]` policy (Rust `MergePolicy::default()`) every fresh instance boots with
 * until a persisted `🛡️change-merge-policy` config triad overrides it or a caller sends
 * `AppChannelClient.setMergePolicy`. */
export const DEFAULT_MERGE_POLICY: MergePolicy = "Normal";

const MERGE_POLICY_ORDER: readonly MergePolicy[] = ["LaissezFaire", "Normal", "Vigilant"];

/** 🔏️ TS twin of Rust `MergePolicy::as_u8` — the ordinal `AppCommand::SetMergePolicy.policy` carries. */
export function mergePolicyAsU8(policy: MergePolicy): number {
  return MERGE_POLICY_ORDER.indexOf(policy);
}

/** 🪞️ TS twin of Rust `MergePolicy::from_u8`; `undefined` for any value outside 0..2. */
export function mergePolicyFromU8(value: number): MergePolicy | undefined {
  return MERGE_POLICY_ORDER[value];
}

/** ✅️❌️ What a human/authority decided to do with an `Open` {@link Conflict} — TS twin of
 * Rust `ConflictResolution` (`📡️spr/⚔️conflict/🦀️.rs`, `#[serde(rename_all =
 * "camelCase")]` unit enum — single-word variants so its JSON form is just lowercase). */
export type ConflictResolution = "accept" | "discard";

const CONFLICT_RESOLUTION_ORDER: readonly ConflictResolution[] = ["accept", "discard"];

/** 🔢️ The ordinal `AppCommand::ResolveConflict.resolution` carries — declaration order 0..1. */
export function conflictResolutionAsU8(resolution: ConflictResolution): number {
  return CONFLICT_RESOLUTION_ORDER.indexOf(resolution);
}

export function conflictResolutionFromU8(value: number): ConflictResolution | undefined {
  return CONFLICT_RESOLUTION_ORDER[value];
}

/** 💬️ One outcome-carried diagnostic from a `Mutation`/`MutationKind::diff` — TS twin of
 * Rust `MutationMessage` (`📡️spr/🎮️command/🦀️.rs` region `🔖️Message`,
 * `#[serde(rename_all = "camelCase")]`). `level` reuses {@link Severity}; `code` is one of the
 * frozen nine `mutation.*` codes (contract-freeze §C2 plus the 2026-09-30 state-dependent Error codes — no per-plugin codes, ever); `message` is
 * English prose (UI localizes by `code`, never by parsing `message`); `target`/`opIndex` are
 * `#[serde(skip_serializing_if)]` on the Rust side, so both are absent (not merely empty) from the
 * pack-decoded JSON when unset. */
export type MutationMessage = {
  readonly level: Severity;
  readonly code: string;
  readonly message: string;
  readonly target?: readonly string[];
  readonly opIndex?: number;
};

/** 🚫️ Schema mirror of Rust `MutationApplyError` (`📡️spr/🎮️command/🦀️.rs`,
 * `#[serde(rename_all = "camelCase")]`). This is the complete cross-implementation contract for
 * a diff rejected against its supplied base: stable machine `code`, diagnostic `message`, and
 * outermost-first `target`. Rust omits an empty target during serialization, so it is optional
 * on decoded wire values and semantically equivalent to `[]`. */
export type MutationApplyError = {
  readonly code: string;
  readonly message: string;
  readonly target?: readonly string[];
};

/** 🧬️ Runtime JSON Schema for the Rust/TypeScript `MutationApplyError` wire shape. */
export const MUTATION_APPLY_ERROR_SCHEMA = {
  type: "object",
  additionalProperties: false,
  required: ["code", "message"],
  properties: {
    code: { type: "string" },
    message: { type: "string" },
    target: { type: "array", items: { type: "string" } },
  },
} as const;

/** 🔁️ Shared Rust/TypeScript parity vector for the exact serialized apply-error shape. */
export const MUTATION_APPLY_ERROR_WIRE_PARITY_VECTOR = {
  json: '{"code":"mutation.apply.invalid-index","message":"index 4 exceeds length 2","target":["slides","4"]}',
  value: {
    code: "mutation.apply.invalid-index",
    message: "index 4 exceeds length 2",
    target: ["slides", "4"],
  } satisfies MutationApplyError,
} as const;

/** 🆔️ Content-addressed conflict identity — TS twin of Rust `ConflictId`
 * (`#[serde(transparent)]`, decodes to a bare string: `conflict-<blake3 hex>`). */
export type ConflictId = string;

/** 🚧️ What kind of conflict this is — TS twin of Rust `ConflictKind`
 * (`#[serde(tag = "kind", rename_all = "camelCase")]`, internally tagged). `rename_all` on an enum
 * renames only the `kind` discriminant, not a struct variant's own fields, so `edit_ids` stays
 * snake_case exactly as Rust declared it. `envelopes` is `Vec<MutationEnvelope>` serialized through
 * the generic DSL-value bridge (NOT the dedicated causal-envelope wire codec `AppCommand::
 * ApplyEnvelopes`/🧰️framework/🔨️modules/🎠️kernel`AppFrame::DocumentChanged` use) — left as `unknown` here; no consumer needs a
 * typed shape for it yet. */
export type ConflictKind = { readonly kind: "quarantined"; readonly envelopes: readonly unknown[] } | { readonly kind: "degraded"; readonly edit_ids: readonly string[] };

/** 🚦️ A conflict's own lifecycle, independent of the `MutationMessage`s it carries — TS twin
 * of Rust `ConflictStatus` (`#[serde(rename_all = "camelCase")]`). */
export type ConflictStatus = "open" | "accepted" | "discarded";

/** ⚔️ One first-class conflict — TS twin of Rust `Conflict` (`📡️spr/⚔️conflict/
 * 🦀️.rs`, `#[serde(rename_all = "camelCase")]`). `timestamp` mirrors
 * `HybridLogicalTimestamp` from `📡️spr/🆔️ids/🦀️.rs` (a DIFFERENT shape than this file's
 * own wall/counter {@link HybridLogicalTimestamp} above — that one is the kernel operation clock,
 * this one the SPR authority clock — so it's inlined rather than reusing the name). */
export type Conflict = {
  readonly id: ConflictId;
  readonly kind: ConflictKind;
  readonly status: ConflictStatus;
  readonly messages: readonly MutationMessage[];
  readonly actors: readonly string[];
  readonly timestamp: { readonly actor: number; readonly physical_ms: number; readonly logical: number };
};

/** 📨️ One edit's worth of `MutationMessage`s — TS twin of Rust `EditMessages`. */
export type EditMessages = { readonly edit_id: string; readonly messages: readonly MutationMessage[] };

/** 📤️ The report one LOCAL dispatch produces — TS twin of Rust `DispatchReport`. Packed onto
 * the wire as `AppFrame::Invocation.messages` (successful dispatch) and `AppFrame::Error.report`
 * (rejected dispatch, `Fault.code == "app.command.rejected"`). */
export type DispatchReport = {
  readonly policy: MergePolicy;
  readonly worst: Severity | null;
  readonly messages: readonly MutationMessage[];
};

/** 🔀️ The report one `ingest_remote`/`resolve_conflict` merge
 * produces — TS twin of Rust `MergeReport`. Packed onto the wire as `AppFrame::MergeReport.report`,
 * pushed unsolicited after every ingest alongside `DocumentChanged`. */
export type MergeReport = {
  readonly policy: MergePolicy;
  readonly accepted: boolean;
  readonly insertionIndex: number;
  readonly replayed: readonly EditMessages[];
  readonly worst: Severity | null;
  readonly conflict: ConflictId | null;
};
//#endregion 🔖️MergeOutcome
// #endregion 🎠️Kernel
