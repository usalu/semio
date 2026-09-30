/**
 * ⏪️ TypeScript mirror of the domain-neutral time-travel session (`🦀️.rs`): identity, generation
 * and base stamp, stages, accepted drafts in history order, pending draft, replay report, progress
 * and fault; the pure reducer, its events, effects, refusals and EN/DE labels. u64 values are
 * `bigint`, the content revision is a `Uint8Array`; replication types (`InputReplacement`,
 * `SupersededInput`, `ReplayReport`) come from the replication twin. Schema of record: `🧬️schema/🔣️.json`; law fixture:
 * `🧫️fixtures/🧫️lifecycle-law/🔣️.json`; contract:
 * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4.
 *
 * Driver contract: `begin` carries the target position resolved at the session base; every store
 * generation change is sent as `baseMoved` with the re-resolved positions of every session target
 * (in every stage; an unchanged base is `timeTravel.stale`); every other event except `begin` and
 * `exit` carries the session generation. {@link timeTravelReview} classifies `reviewing`: nothing
 * accepted is "no changes" (the preview is the committed head); drafts without a report (a cancelled
 * or faulted replay) "need a replay", which `rerun` starts. A malformed fault code or alternative
 * name is `timeTravel.illegal`.
 */

import { replayReportBlocksFinalize, type InputReplacement, type ReplayReport, type SupersededInput } from "@semio-tech/framework-replication";

//#region 🔖️Limits
/** 📏️ UTF-8 byte ceiling of a fault code and of an alternative name. */
export const TIME_TRAVEL_TEXT_MAX_BYTES = 256;

/** 🔒️ Refusal code of an artifact-lane app command while a session is not `inactive` (issued by the host). */
export const TIME_TRAVEL_FROZEN_CODE = "timeTravel.frozen";

/** 🧯️ Fault a cancelled replay leaves behind, so hosts can label it and offer `rerun`. */
export const TIME_TRAVEL_CANCELLED_CODE = "timeTravel.cancelled";

const utf8Length = (text: string): number => new TextEncoder().encode(text).length;

/** 🔣️ A fault code is non-empty, free of Unicode `White_Space` and at most {@link TIME_TRAVEL_TEXT_MAX_BYTES}. */
export function isTimeTravelFaultCode(code: string): boolean {
  return code.length > 0 && utf8Length(code) <= TIME_TRAVEL_TEXT_MAX_BYTES && !/\p{White_Space}/u.test(code);
}

/** 🪪️ An alternative name is not blank (Unicode `White_Space`) and at most {@link TIME_TRAVEL_TEXT_MAX_BYTES}. */
export function isTimeTravelAlternativeName(name: string): boolean {
  return utf8Length(name) <= TIME_TRAVEL_TEXT_MAX_BYTES && /[^\p{White_Space}]/u.test(name);
}
//#endregion 🔖️Limits

//#region 🔖️Protocol
/** 🟰️ Structural equality of two replacements (payload bytes compared). */
export function inputReplacementEquals(left: InputReplacement, right: InputReplacement): boolean {
  if (left.kind === "withdrawn" || right.kind === "withdrawn") return left.kind === right.kind;
  return left.schema === right.schema && left.payload.length === right.payload.length && left.payload.every((byte, index) => byte === right.payload[index]);
}

/** 🔤️ UTF-8 byte order (= code point order) of two strings, as Rust's `String: Ord`. */
export function compareCodePoints(left: string, right: string): number {
  let i = 0;
  let j = 0;
  while (i < left.length && j < right.length) {
    const a = left.codePointAt(i)!;
    const b = right.codePointAt(j)!;
    if (a !== b) return a < b ? -1 : 1;
    i += a > 0xffff ? 2 : 1;
    j += b > 0xffff ? 2 : 1;
  }
  return i === left.length ? (j === right.length ? 0 : -1) : 1;
}
//#endregion 🔖️Protocol

//#region 🔖️Identity
/** 🧭️ The store state a session last observed. */
export type TimeTravelBase = { readonly storeGeneration: bigint; readonly contentRevision: Uint8Array };

/** 🎯️ A mutation the session addresses and its applied position at the session base. */
export type TimeTravelTarget = { readonly mutation: string; readonly position: number };

/** 🧮️ History order: applied position, then mutation id (UTF-8 byte order). */
export function compareTimeTravelTargets(left: TimeTravelTarget, right: TimeTravelTarget): number {
  return left.position !== right.position ? (left.position < right.position ? -1 : 1) : compareCodePoints(left.mutation, right.mutation);
}

function baseEquals(left: TimeTravelBase, right: TimeTravelBase): boolean {
  return left.storeGeneration === right.storeGeneration && left.contentRevision.length === right.contentRevision.length && left.contentRevision.every((byte, index) => byte === right.contentRevision[index]);
}
//#endregion 🔖️Identity

//#region 🔖️Session
export const TIME_TRAVEL_STAGES = ["inactive", "editing", "replaying", "reviewing", "choosing", "finalizing"] as const;
export type TimeTravelStage = (typeof TIME_TRAVEL_STAGES)[number];

export type TimeTravelDraft = { readonly target: TimeTravelTarget; readonly replacement: InputReplacement };
export type TimeTravelPending = { readonly target: TimeTravelTarget; readonly original: InputReplacement; readonly replacement: InputReplacement; readonly returnStage: TimeTravelStage };
export type TimeTravelProgress = { readonly done: number; readonly total: number };

/** 🕰️ Ephemeral local-only session of one app instance; `generation` only grows. */
export type TimeTravelSession = {
  readonly id: bigint;
  readonly generation: number;
  readonly base: TimeTravelBase;
  readonly stage: TimeTravelStage;
  readonly accepted: readonly TimeTravelDraft[];
  readonly pending: TimeTravelPending | null;
  readonly report: ReplayReport | null;
  readonly progress: TimeTravelProgress | null;
  readonly fault: string | null;
};

/** 🔭️ What a `reviewing` session shows: nothing accepted, drafts awaiting a replay, or a report that blocks or allows finalizing. */
export const TIME_TRAVEL_REVIEWS = ["noChanges", "needsReplay", "blocked", "ready"] as const;
export type TimeTravelReview = (typeof TIME_TRAVEL_REVIEWS)[number];

/** 🌱️ A fresh, inactive session observing `base`. */
export function timeTravelSession(base: TimeTravelBase): TimeTravelSession {
  return { id: 0n, generation: 0, base, stage: "inactive", accepted: [], pending: null, report: null, progress: null, fault: null };
}
//#endregion 🔖️Session

//#region 🔖️Events
export type TimeTravelChoice = { readonly kind: "overwrite" } | { readonly kind: "alternative"; readonly name: string };

export type TimeTravelEvent =
  | { readonly type: "begin"; readonly target: TimeTravelTarget; readonly original: InputReplacement }
  | { readonly type: "draft"; readonly generation: number; readonly replacement: InputReplacement }
  | { readonly type: "withdraw" | "accept" | "discard" | "replayCancelled" | "rerun" | "requestFinalize" | "back" | "finalized"; readonly generation: number }
  | { readonly type: "replayProgressed"; readonly generation: number; readonly done: number; readonly total: number }
  | { readonly type: "replayCompleted"; readonly generation: number; readonly report: ReplayReport }
  | { readonly type: "replayFaulted" | "finalizeFaulted"; readonly generation: number; readonly code: string }
  | { readonly type: "choose"; readonly generation: number; readonly choice: TimeTravelChoice }
  | { readonly type: "baseMoved"; readonly base: TimeTravelBase; readonly positions: readonly TimeTravelTarget[] }
  | { readonly type: "exit" };

export const TIME_TRAVEL_EVENT_KEYS = [
  "begin",
  "draft",
  "withdraw",
  "accept",
  "discard",
  "replayProgressed",
  "replayCompleted",
  "replayCancelled",
  "replayFaulted",
  "rerun",
  "requestFinalize",
  "chooseOverwrite",
  "chooseAlternative",
  "back",
  "finalized",
  "finalizeFaulted",
  "baseMoved",
  "exit",
] as const;
export type TimeTravelEventKey = (typeof TIME_TRAVEL_EVENT_KEYS)[number];

/** 🗝️ Lifecycle-law matrix column of an event. */
export function timeTravelEventKey(event: TimeTravelEvent): TimeTravelEventKey {
  if (event.type === "choose") return event.choice.kind === "overwrite" ? "chooseOverwrite" : "chooseAlternative";
  return event.type;
}

/** 🧿️ The generation an event is addressed to; `null` for `begin`, `baseMoved` and `exit`. */
export function timeTravelEventGeneration(event: TimeTravelEvent): number | null {
  return "generation" in event ? event.generation : null;
}

/** 🩺️ Whether every fault code and alternative name the event carries is valid. */
export function timeTravelEventWellFormed(event: TimeTravelEvent): boolean {
  if (event.type === "replayFaulted" || event.type === "finalizeFaulted") return isTimeTravelFaultCode(event.code);
  if (event.type === "choose" && event.choice.kind === "alternative") return isTimeTravelAlternativeName(event.choice.name);
  return true;
}
//#endregion 🔖️Events

//#region 🔖️Effects
export type TimeTravelEffect =
  | { readonly type: "showPreview"; readonly target: string; readonly replacement: InputReplacement }
  | { readonly type: "startReplay"; readonly drafts: readonly SupersededInput[]; readonly from: string }
  | { readonly type: "cancelReplay" | "openFinalizePrompt" | "close" }
  | { readonly type: "commitOverwrite"; readonly inputs: readonly SupersededInput[] }
  | { readonly type: "commitAlternative"; readonly name: string; readonly inputs: readonly SupersededInput[] };

export const TIME_TRAVEL_EFFECT_KINDS = ["showPreview", "startReplay", "cancelReplay", "openFinalizePrompt", "commitOverwrite", "commitAlternative", "close"] as const;
export type TimeTravelEffectKind = (typeof TIME_TRAVEL_EFFECT_KINDS)[number];
//#endregion 🔖️Effects

//#region 🔖️Refusals
export const TIME_TRAVEL_REFUSALS = ["timeTravel.illegal", "timeTravel.stale", "timeTravel.blocked", "timeTravel.empty"] as const;
export type TimeTravelRefusal = (typeof TIME_TRAVEL_REFUSALS)[number];

export type TimeTravelApplyResult = { readonly ok: true; readonly session: TimeTravelSession; readonly effects: readonly TimeTravelEffect[] } | { readonly ok: false; readonly rejection: TimeTravelRefusal };
//#endregion 🔖️Refusals

//#region 🔖️Reducer
/** 🔎️ The accepted draft of `mutation`, if any. */
export function timeTravelAcceptedDraft(session: TimeTravelSession, mutation: string): TimeTravelDraft | undefined {
  return session.accepted.find((draft) => draft.target.mutation === mutation);
}

/** 🧾️ The accepted drafts as `Supersede` inputs, in history order. */
export function timeTravelInputs(session: TimeTravelSession): SupersededInput[] {
  return session.accepted.map((draft) => ({ target: draft.target.mutation, replacement: draft.replacement }));
}

/** 🧷️ The value a pending draft started from: the target's accepted draft, else its original input. */
export function timeTravelStartOf(session: TimeTravelSession, pending: TimeTravelPending): InputReplacement {
  return timeTravelAcceptedDraft(session, pending.target.mutation)?.replacement ?? pending.original;
}

/** 🪞️ Whether `pending` still equals the value it started from. */
export function timeTravelUnchanged(session: TimeTravelSession, pending: TimeTravelPending): boolean {
  return inputReplacementEquals(pending.replacement, timeTravelStartOf(session, pending));
}

/** 🚧️ Why `requestFinalize` would be refused, `null` when it would open the prompt. */
export function timeTravelFinalizeRefusal(session: TimeTravelSession): TimeTravelRefusal | null {
  if (session.stage !== "reviewing") return "timeTravel.illegal";
  if (session.accepted.length === 0) return "timeTravel.empty";
  if (session.report === null || replayReportBlocksFinalize(session.report)) return "timeTravel.blocked";
  return null;
}

/** 🔁️ Why `rerun` would be refused, `null` when it would start a replay: it needs `reviewing`, accepted drafts, and either no report or a fault. */
export function timeTravelRerunRefusal(session: TimeTravelSession): TimeTravelRefusal | null {
  if (session.stage !== "reviewing") return "timeTravel.illegal";
  if (session.accepted.length === 0) return "timeTravel.empty";
  if (session.report !== null && session.fault === null) return "timeTravel.illegal";
  return null;
}

/** 🧐️ Classification of a `reviewing` session; `null` in every other stage. */
export function timeTravelReview(session: TimeTravelSession): TimeTravelReview | null {
  if (session.stage !== "reviewing") return null;
  if (session.accepted.length === 0) return "noChanges";
  if (session.report === null) return "needsReplay";
  return replayReportBlocksFinalize(session.report) ? "blocked" : "ready";
}

/** 🧪️ First violated session invariant (law fixture `invariants`), `null` when coherent. */
export function timeTravelInvariantViolation(session: TimeTravelSession): string | null {
  const { stage, pending, accepted, report } = session;
  if ((stage === "editing") !== (pending !== null)) return "pending-iff-editing";
  if (pending !== null && pending.returnStage !== "inactive" && pending.returnStage !== "reviewing") return "return-stage-inactive-or-reviewing";
  if (pending !== null && pending.returnStage === "inactive" && accepted.length > 0) return "return-inactive-means-nothing-accepted";
  if (stage === "inactive" && (accepted.length > 0 || report !== null || session.fault !== null)) return "inactive-holds-nothing";
  if ((stage === "replaying" || stage === "choosing" || stage === "finalizing") && accepted.length === 0) return "work-needs-accepted-drafts";
  if (session.progress !== null && stage !== "replaying") return "progress-only-while-replaying";
  if (stage === "replaying" && report !== null) return "replaying-has-no-report";
  if ((stage === "choosing" || stage === "finalizing") && (report === null || replayReportBlocksFinalize(report))) return "finalize-needs-a-clean-report";
  if (accepted.some((draft, index) => index > 0 && compareTimeTravelTargets(accepted[index - 1]!.target, draft.target) >= 0)) return "accepted-in-history-order";
  if (accepted.some((draft, index) => accepted.slice(0, index).some((earlier) => earlier.target.mutation === draft.target.mutation))) return "accepted-targets-unique";
  if (session.fault !== null && accepted.length === 0) return "fault-needs-drafts";
  return null;
}

type Step = { readonly session: TimeTravelSession; readonly effects: readonly TimeTravelEffect[] };

const bump = (generation: number): number => (generation + 1) >>> 0;

function begin(session: TimeTravelSession, target: TimeTravelTarget, original: InputReplacement, returnStage: TimeTravelStage, id: bigint): Step {
  const replacement = timeTravelAcceptedDraft(session, target.mutation)?.replacement ?? original;
  return { session: { ...session, id, stage: "editing", generation: bump(session.generation), pending: { target, original, replacement, returnStage } }, effects: [{ type: "showPreview", target: target.mutation, replacement }] };
}

function settle(session: TimeTravelSession): Step {
  const cleared = { ...session, report: null, progress: null };
  const first = session.accepted[0];
  if (first === undefined) return { session: { ...cleared, stage: "reviewing", fault: null }, effects: [] };
  const next = { ...cleared, stage: "replaying" as const, fault: null, generation: bump(session.generation) };
  return { session: next, effects: [{ type: "startReplay", drafts: timeTravelInputs(next), from: first.target.mutation }] };
}

function close(session: TimeTravelSession, effects: readonly TimeTravelEffect[]): Step {
  return { session: { ...session, stage: "inactive", generation: bump(session.generation), accepted: [], pending: null, report: null, progress: null, fault: null }, effects: [...effects, { type: "close" }] };
}

function resume(session: TimeTravelSession, returnStage: TimeTravelStage): Step {
  if (returnStage !== "reviewing") return close(session, []);
  if (session.report === null && session.accepted.length > 0) return settle(session);
  return { session: { ...session, stage: "reviewing" }, effects: [] };
}

function accept(session: TimeTravelSession, pending: TimeTravelPending): Step {
  if (timeTravelUnchanged(session, pending)) return resume(session, pending.returnStage);
  const kept = session.accepted.filter((draft) => draft.target.mutation !== pending.target.mutation);
  const accepted = inputReplacementEquals(pending.replacement, pending.original) ? kept : [...kept, { target: pending.target, replacement: pending.replacement }].sort((left, right) => compareTimeTravelTargets(left.target, right.target));
  return settle({ ...session, accepted });
}

function reposition(session: TimeTravelSession, positions: readonly TimeTravelTarget[]): TimeTravelSession {
  const moved = (target: TimeTravelTarget): TimeTravelTarget => positions.reduce((current, candidate) => (candidate.mutation === current.mutation ? { mutation: current.mutation, position: candidate.position } : current), target);
  const accepted = session.accepted.map((draft) => ({ ...draft, target: moved(draft.target) })).sort((left, right) => compareTimeTravelTargets(left.target, right.target));
  return { ...session, accepted, pending: session.pending === null ? null : { ...session.pending, target: moved(session.pending.target) } };
}

const refuse = (rejection: TimeTravelRefusal): TimeTravelApplyResult => ({ ok: false, rejection });
const admit = (step: Step): TimeTravelApplyResult => ({ ok: true, session: step.session, effects: step.effects });

/** ⚖️ Pure §4 reducer: generation fence first, then the law; the input session is never mutated. */
export function applyTimeTravel(session: TimeTravelSession, event: TimeTravelEvent): TimeTravelApplyResult {
  const generation = timeTravelEventGeneration(event);
  if (generation !== null && generation !== session.generation) return refuse("timeTravel.stale");
  if (!timeTravelEventWellFormed(event)) return refuse("timeTravel.illegal");
  const { stage, pending } = session;
  switch (event.type) {
    case "begin":
      if (stage === "inactive") return admit(begin(session, event.target, event.original, "inactive", (session.id + 1n) & 0xffff_ffff_ffff_ffffn));
      if (stage === "reviewing") return admit(begin(session, event.target, event.original, "reviewing", session.id));
      if (stage === "editing" && pending !== null) return timeTravelUnchanged(session, pending) ? admit(begin(session, event.target, event.original, pending.returnStage, session.id)) : refuse("timeTravel.blocked");
      return refuse("timeTravel.illegal");
    case "draft":
    case "withdraw": {
      if (stage !== "editing" || pending === null) return refuse("timeTravel.illegal");
      const replacement: InputReplacement = event.type === "draft" ? event.replacement : { kind: "withdrawn" };
      return admit({ session: { ...session, pending: { ...pending, replacement } }, effects: [{ type: "showPreview", target: pending.target.mutation, replacement }] });
    }
    case "accept":
      return stage === "editing" && pending !== null ? admit(accept({ ...session, pending: null }, pending)) : refuse("timeTravel.illegal");
    case "discard":
      return stage === "editing" && pending !== null ? admit(resume({ ...session, pending: null }, pending.returnStage)) : refuse("timeTravel.illegal");
    case "replayProgressed":
      return stage === "replaying" ? admit({ session: { ...session, progress: { done: event.done, total: event.total } }, effects: [] }) : refuse("timeTravel.illegal");
    case "replayCompleted":
      return stage === "replaying" ? admit({ session: { ...session, stage: "reviewing", report: event.report, progress: null }, effects: [] }) : refuse("timeTravel.illegal");
    case "replayCancelled":
      return stage === "replaying" ? admit({ session: { ...session, stage: "reviewing", progress: null, fault: TIME_TRAVEL_CANCELLED_CODE }, effects: [] }) : refuse("timeTravel.illegal");
    case "replayFaulted":
      return stage === "replaying" ? admit({ session: { ...session, stage: "reviewing", progress: null, fault: event.code }, effects: [] }) : refuse("timeTravel.illegal");
    case "rerun": {
      const refusal = timeTravelRerunRefusal(session);
      return refusal === null ? admit(settle(session)) : refuse(refusal);
    }
    case "requestFinalize": {
      const refusal = timeTravelFinalizeRefusal(session);
      return refusal === null ? admit({ session: { ...session, stage: "choosing" }, effects: [{ type: "openFinalizePrompt" }] }) : refuse(refusal);
    }
    case "choose": {
      if (stage !== "choosing") return refuse("timeTravel.illegal");
      const inputs = timeTravelInputs(session);
      const effect: TimeTravelEffect = event.choice.kind === "overwrite" ? { type: "commitOverwrite", inputs } : { type: "commitAlternative", name: event.choice.name, inputs };
      return admit({ session: { ...session, stage: "finalizing", generation: bump(session.generation) }, effects: [effect] });
    }
    case "back":
      return stage === "choosing" ? admit({ session: { ...session, stage: "reviewing" }, effects: [] }) : refuse("timeTravel.illegal");
    case "finalized":
      return stage === "finalizing" ? admit(close(session, [])) : refuse("timeTravel.illegal");
    case "finalizeFaulted": {
      if (stage !== "finalizing") return refuse("timeTravel.illegal");
      const step = settle(session);
      return admit({ session: { ...step.session, fault: event.code }, effects: step.effects });
    }
    case "baseMoved": {
      if (baseEquals(event.base, session.base)) return refuse("timeTravel.stale");
      const next = reposition({ ...session, base: event.base }, event.positions);
      switch (stage) {
        case "inactive":
        case "finalizing":
          return admit({ session: next, effects: [] });
        case "editing":
          return admit({ session: { ...next, report: null }, effects: next.pending === null ? [] : [{ type: "showPreview", target: next.pending.target.mutation, replacement: next.pending.replacement }] });
        default:
          return admit(settle(next));
      }
    }
    case "exit":
      if (stage === "inactive") return refuse("timeTravel.illegal");
      if (stage === "finalizing") return refuse("timeTravel.blocked");
      return admit(close(session, stage === "replaying" ? [{ type: "cancelReplay" }] : []));
  }
}
//#endregion 🔖️Reducer

//#region 🔖️Json
/** 🔢️ Lowercase hex of `bytes`. */
export function timeTravelBytesToHex(bytes: ArrayLike<number>): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🔡️ Bytes of a lowercase hex string. */
export function timeTravelHexToBytes(hex: string): Uint8Array {
  return Uint8Array.from({ length: hex.length / 2 }, (_, index) => Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16));
}

type Json = any;

const baseFromJson = (json: Json): TimeTravelBase => ({ storeGeneration: BigInt(json.storeGeneration), contentRevision: timeTravelHexToBytes(json.contentRevision) });
const baseToJson = (base: TimeTravelBase): Json => ({ storeGeneration: Number(base.storeGeneration), contentRevision: timeTravelBytesToHex(base.contentRevision) });
const draftFromJson = (json: Json): TimeTravelDraft => ({ target: { ...json.target }, replacement: json.replacement });
const draftToJson = (draft: TimeTravelDraft): Json => ({ target: { ...draft.target }, replacement: draft.replacement });
const inputToJson = (input: SupersededInput): Json => ({ target: input.target, replacement: input.replacement });

export function timeTravelSessionFromJson(json: Json): TimeTravelSession {
  return {
    id: BigInt(json.id),
    generation: json.generation,
    base: baseFromJson(json.base),
    stage: json.stage,
    accepted: json.accepted.map(draftFromJson),
    pending: json.pending === null ? null : { target: { ...json.pending.target }, original: json.pending.original, replacement: json.pending.replacement, returnStage: json.pending.returnStage },
    report: json.report,
    progress: json.progress,
    fault: json.fault,
  };
}

export function timeTravelSessionToJson(session: TimeTravelSession): Json {
  const { pending } = session;
  return {
    id: Number(session.id),
    generation: session.generation,
    base: baseToJson(session.base),
    stage: session.stage,
    accepted: session.accepted.map(draftToJson),
    pending: pending === null ? null : { target: { ...pending.target }, original: pending.original, replacement: pending.replacement, returnStage: pending.returnStage },
    report: session.report,
    progress: session.progress,
    fault: session.fault,
  };
}

export function timeTravelEventFromJson(json: Json): TimeTravelEvent {
  switch (json.type) {
    case "begin":
      return { type: "begin", target: { ...json.target }, original: json.original };
    case "baseMoved":
      return { type: "baseMoved", base: baseFromJson(json.base), positions: json.positions.map((target: Json) => ({ ...target })) };
    default:
      return { ...json };
  }
}

export function timeTravelEffectToJson(effect: TimeTravelEffect): Json {
  switch (effect.type) {
    case "showPreview":
      return { type: effect.type, target: effect.target, replacement: effect.replacement };
    case "startReplay":
      return { type: effect.type, drafts: effect.drafts.map(inputToJson), from: effect.from };
    case "commitOverwrite":
      return { type: effect.type, inputs: effect.inputs.map(inputToJson) };
    case "commitAlternative":
      return { type: effect.type, name: effect.name, inputs: effect.inputs.map(inputToJson) };
    default:
      return { type: effect.type };
  }
}
//#endregion 🔖️Json

//#region 🔖️Labels
/** 🗣️ Framework-owned EN/DE text of history editing, no default locale. */
export const TIME_TRAVEL_LABELS = {
  stageInactive: { en: "Not editing history", de: "Verlauf wird nicht bearbeitet" },
  stageEditing: { en: "Editing a mutation", de: "Mutation wird bearbeitet" },
  stageReplaying: { en: "Replaying later mutations", de: "Spätere Mutationen werden neu angewendet" },
  stageReviewing: { en: "Reviewing the edited history", de: "Bearbeiteter Verlauf wird geprüft" },
  stageChoosing: { en: "Choose how to finalize", de: "Art des Abschlusses wählen" },
  stageFinalizing: { en: "Finalizing the history edit", de: "Verlaufsbearbeitung wird abgeschlossen" },
  refusalIllegal: { en: "Not possible right now", de: "Derzeit nicht möglich" },
  refusalStale: { en: "Outdated request ignored", de: "Veraltete Anfrage ignoriert" },
  refusalBlocked: { en: "Blocked: resolve the pending change or the errors first", de: "Blockiert: zuerst die offene Änderung oder die Fehler auflösen" },
  refusalEmpty: { en: "Nothing to finalize: no accepted changes", de: "Nichts abzuschließen: keine übernommenen Änderungen" },
  frozen: { en: "Editing is paused while history is being edited", de: "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird" },
  choiceOverwrite: { en: "Overwrite history", de: "Verlauf überschreiben" },
  choiceOverwriteDescription: { en: "Replaces the inputs in every alternative that contains these mutations", de: "Ersetzt die Eingaben in jeder Alternative, die diese Mutationen enthält" },
  choiceAlternative: { en: "New alternative", de: "Neue Alternative" },
  choiceAlternativeDescription: { en: "Keeps the original history and continues in a new alternative", de: "Behält den ursprünglichen Verlauf und arbeitet in einer neuen Alternative weiter" },
  alternativeNameDefault: { en: "Edited history", de: "Bearbeiteter Verlauf" },
  noChanges: { en: "No changes: showing the current history", de: "Keine Änderungen: aktueller Verlauf wird angezeigt" },
  needsReplay: { en: "Replay needed: later mutations are not checked yet", de: "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft" },
  reportBlocking: { en: "Errors in later mutations block finalizing", de: "Fehler in späteren Mutationen verhindern den Abschluss" },
  readyToFinalize: { en: "Ready to finalize", de: "Bereit zum Abschließen" },
  replayCancelled: { en: "Replay cancelled", de: "Neu anwenden abgebrochen" },
  actionRerun: { en: "Replay again", de: "Erneut anwenden" },
  replayProgressValueText: { en: "Replaying {done} of {total} mutations", de: "{done} von {total} Mutationen werden neu angewendet" },
} as const satisfies Record<string, { en: string; de: string }>;
export type TimeTravelLabelKey = keyof typeof TIME_TRAVEL_LABELS;

/** 💬️ Status label key of a stage. */
export function timeTravelStageLabel(stage: TimeTravelStage): TimeTravelLabelKey {
  return `stage${stage[0]!.toUpperCase()}${stage.slice(1)}` as TimeTravelLabelKey;
}

/** 📛️ Status label key of a review. */
export function timeTravelReviewLabel(review: TimeTravelReview): TimeTravelLabelKey {
  return ({ noChanges: "noChanges", needsReplay: "needsReplay", blocked: "reportBlocking", ready: "readyToFinalize" } as const)[review];
}

/** 🩹️ Label key of a framework-owned fault code (`timeTravel.cancelled`); driver fault codes have none. */
export function timeTravelFaultLabel(code: string): TimeTravelLabelKey | undefined {
  return code === TIME_TRAVEL_CANCELLED_CODE ? "replayCancelled" : undefined;
}

/** 🙅️ Label key of a refusal. */
export function timeTravelRefusalLabel(refusal: TimeTravelRefusal): TimeTravelLabelKey {
  const name = refusal.slice("timeTravel.".length);
  return `refusal${name[0]!.toUpperCase()}${name.slice(1)}` as TimeTravelLabelKey;
}
//#endregion 🔖️Labels
