/** 📤️ Canonical guest publications and Shell-owned inference intent effects. */
import { decodeAppFrame, decodeInvocationResultPacks, decodePackValue, encodeAppFrame, encodePackValue } from "../../../../../🟦️.ts";
import { BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, parseBrowserActorHostEffectBytesV1 } from "../🟦️.ts";

/** 🕰️ A pack-encoded `HistoryPatch` the guest published with a result, forwarded verbatim to the Shell's History projection. */
export type BrowserActorHistoryPatchBytesV1 = readonly number[] | null;
export type BrowserActorUnsolicitedPublicationV1 =
  | { readonly kind: "ephemeral" }
  | { readonly kind: "merge-report" }
  | { readonly kind: "operation-completed"; readonly historyPatch: BrowserActorHistoryPatchBytesV1 }
  | { readonly kind: "completion"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 };
export type BrowserActorIntentPublicationV1 = { readonly kind: "emit" } | { readonly kind: "error"; readonly reason: string } | BrowserActorUnsolicitedPublicationV1;
type BrowserActorCommandMutationProjectionV1 = ReturnType<typeof decodeInvocationResultPacks>;
export type BrowserActorCommandPublicationV1 = { readonly kind: "invocation"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 } | { readonly kind: "error"; readonly reason: string } | BrowserActorUnsolicitedPublicationV1;
export type BrowserActorCommandBackboneEnvelopeV1 = Readonly<{
  mutation_id: string;
  actor: string;
  dependencies: readonly string[];
  diff: Readonly<{ schema: string; payload: Readonly<Uint8Array> }>;
  inverse: Readonly<{ schema: string; payload: Readonly<Uint8Array> }>;
  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
}>;
export type BrowserActorHostEffectV1 =
  | { readonly requestInferenceProposal: { readonly kind: "gis-map-bounds-region" } }
  | { readonly openExternalUrl: { readonly url: string } };

function exact(value: unknown, fields: readonly string[]): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.getPrototypeOf(value) !== Object.prototype) throw new Error("browser-actor-publication: invalid object");
  const record = value as Readonly<Record<string, unknown>>;
  if (Object.keys(record).length !== fields.length || fields.some((field) => !Object.hasOwn(record, field))) throw new Error("browser-actor-publication: invalid exact fields");
  return record;
}

function canonical(bytes: Uint8Array, encoded: Uint8Array): void {
  if (bytes.length !== encoded.length || bytes.some((byte, index) => byte !== encoded[index])) throw new Error("browser-actor-publication: noncanonical bytes");
}

function equalBytes(left: ArrayLike<number>, right: ArrayLike<number>): boolean {
  return left.length === right.length && Array.from(left).every((byte, index) => byte === right[index]);
}

function equalStrings(left: readonly string[] | undefined, right: readonly string[]): boolean {
  return (left?.length ?? 0) === right.length && (left ?? []).every((value, index) => value === right[index]);
}

function readExactVarint(bytes: ArrayLike<number>, position: [number], field: string): number {
  const start = position[0];
  let value = 0n;
  for (let index = 0; index < 10; index++) {
    if (position[0] >= bytes.length) throw new Error(`browser-actor-publication: truncated ${field}`);
    const byte = bytes[position[0]++]!;
    if (index === 9 && byte > 1) throw new Error(`browser-actor-publication: ${field} overflow`);
    value |= BigInt(byte & 0x7f) << BigInt(index * 7);
    if ((byte & 0x80) === 0) {
      if (position[0] - start > 1 && byte === 0) throw new Error(`browser-actor-publication: noncanonical ${field}`);
      if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error(`browser-actor-publication: ${field} capacity`);
      return Number(value);
    }
  }
  throw new Error(`browser-actor-publication: ${field} overflow`);
}

function decodeExactOpsVector(bytes: ArrayLike<number>): readonly Uint8Array[] {
  const position: [number] = [0],
    count = readExactVarint(bytes, position, "inverse count");
  if (count > BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM) throw new Error("browser-actor-publication: inverse count capacity");
  const operations: Uint8Array[] = [];
  for (let index = 0; index < count; index++) {
    const length = readExactVarint(bytes, position, "inverse length"),
      end = position[0] + length;
    if (end > bytes.length) throw new Error("browser-actor-publication: truncated inverse operation");
    const operation = new Uint8Array(length);
    for (let byteIndex = 0; byteIndex < length; byteIndex++) operation[byteIndex] = bytes[position[0] + byteIndex]!;
    operations.push(operation);
    position[0] = end;
  }
  if (position[0] !== bytes.length) throw new Error("browser-actor-publication: trailing inverse bytes");
  return operations;
}

function projection(value: unknown): BrowserActorHostEffectV1 {
  if (value !== null && typeof value === "object" && Object.hasOwn(value, "openExternalUrl")) {
    const effect = exact(value, ["openExternalUrl"]);
    const target = exact(effect.openExternalUrl, ["url"]);
    if (typeof target.url !== "string" || new TextEncoder().encode(target.url).length > 2048 || !/^https?:\/\//u.test(target.url) || /[\u0000-\u0020\u007f]/u.test(target.url)) throw new Error("browser-actor-publication: invalid external URL");
    const url = new URL(target.url);
    if (!url.hostname || url.username || url.password) throw new Error("browser-actor-publication: invalid external URL");
    return { openExternalUrl: { url: target.url } };
  }
  const effect = exact(value, ["requestInferenceProposal"]);
  const proposal = exact(effect.requestInferenceProposal, ["kind"]);
  if (proposal.kind !== "gis-map-bounds-region") throw new Error("browser-actor-publication: unsupported inference kind");
  return { requestInferenceProposal: { kind: proposal.kind } };
}

/** 📬️ Accepts only ordinary, unsolicited intent completion frames, the `Ephemeral` snapshot every guest exchange appends
 * (contract-freeze §C7.6) and a typed operation's unsolicited UI progress; operation bytes remain guest-owned. */
export function decodeBrowserActorIntentPublicationV1(bytes: Uint8Array): BrowserActorIntentPublicationV1 {
  if (bytes.length === 0 || bytes.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: invalid frame size");
  const frame = decodeAppFrame(bytes);
  canonical(bytes, encodeAppFrame(frame));
  if ("Emit" in frame && frame.Emit.in_reply_to === 0) return { kind: "emit" };
  if ("Error" in frame && frame.Error.in_reply_to === null) return { kind: "error", reason: "action-guest-refused" };
  if ("Ephemeral" in frame) return { kind: "ephemeral" };
  if (("MergeReport" in frame && frame.MergeReport.in_reply_to === null) || ("Conflicts" in frame && frame.Conflicts.in_reply_to === null)) return { kind: "merge-report" };
  const completion = unsolicitedCompletion(frame);
  if (completion !== null) return completion;
  throw new Error("browser-actor-publication: foreign frame");
}

/** ⏩️ An unsolicited `Invocation { in_reply_to: 0 }` — a spawned job's completion (a reserved tool verb's real result) or a
 * typed operation's UI progress, retaining its exact mutation projection — or the typed operation's terminal
 * `OperationCompleted`; each with the history patch it carries. */
function unsolicitedCompletion(frame: ReturnType<typeof decodeAppFrame>): Exclude<BrowserActorUnsolicitedPublicationV1, { readonly kind: "ephemeral" | "merge-report" }> | null {
  if ("OperationCompleted" in frame) {
    return { kind: "operation-completed", historyPatch: historyPatchBytes(frame.OperationCompleted.history_patch) };
  }
  if (!("Invocation" in frame) || frame.Invocation.in_reply_to !== 0) return null;
  return { kind: "completion", projection: decodeInvocationResultPacks(frame.Invocation), historyPatch: historyPatchBytes(frame.Invocation.history_patch) };
}

/** 🕳️ A frame's `history_patch` is a pack-encoded `Option<HistoryPatch>`: empty bytes and a canonical pack `null` are
 * both "no patch" (the guest's invocation builders emit the latter); a present patch must be a canonical pack object. */
function historyPatchBytes(bytes: ArrayLike<number>): BrowserActorHistoryPatchBytesV1 {
  if (bytes.length === 0) return null;
  const raw = Uint8Array.from(bytes);
  const value = decodePackValue(raw);
  canonical(raw, encodePackValue(value));
  if (value === null) return null;
  if (typeof value !== "object" || Array.isArray(value)) throw new Error("browser-actor-publication: invalid history patch");
  return Array.from(raw);
}

/** 📭️ Accepts only what a guest publishes on a turn no action owns (remote ingest, wake, view refresh): the appended
 * `Ephemeral` snapshot, the unsolicited `MergeReport`/`Conflicts` of an ingested remote batch, unsolicited completions and
 * `OperationCompleted`. */
export function decodeBrowserActorUnsolicitedPublicationV1(bytes: Uint8Array): BrowserActorUnsolicitedPublicationV1 {
  if (bytes.length === 0 || bytes.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: invalid frame size");
  const frame = decodeAppFrame(bytes);
  canonical(bytes, encodeAppFrame(frame));
  if ("Ephemeral" in frame) return { kind: "ephemeral" };
  if (("MergeReport" in frame && frame.MergeReport.in_reply_to === null) || ("Conflicts" in frame && frame.Conflicts.in_reply_to === null)) return { kind: "merge-report" };
  const completion = unsolicitedCompletion(frame);
  if (completion !== null) return completion;
  throw new Error("browser-actor-publication: foreign frame");
}

/** 🎛️ Accepts one matching command result with its exact mutation projection and history patch, plus unsolicited publications. */
export function decodeBrowserActorCommandPublicationV1(bytes: Uint8Array, actionSequence: number): BrowserActorCommandPublicationV1 {
  if (bytes.length === 0 || bytes.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: invalid frame size");
  const frame = decodeAppFrame(bytes);
  canonical(bytes, encodeAppFrame(frame));
  if ("Error" in frame && frame.Error.in_reply_to === actionSequence) return { kind: "error", reason: "action-guest-refused" };
  if ("Ephemeral" in frame) return { kind: "ephemeral" };
  if (("MergeReport" in frame && frame.MergeReport.in_reply_to === null) || ("Conflicts" in frame && frame.Conflicts.in_reply_to === null)) return { kind: "merge-report" };
  const completion = unsolicitedCompletion(frame);
  if (completion !== null) return completion;
  if (!("Invocation" in frame) || frame.Invocation.in_reply_to !== actionSequence) throw new Error("browser-actor-publication: foreign frame");
  return { kind: "invocation", projection: decodeInvocationResultPacks(frame.Invocation), historyPatch: historyPatchBytes(frame.Invocation.history_patch) };
}

/** 🪢️ Proves a redundant Invocation projection names the exact same operations already sent by the bound native backbone.
 * A zero-mutation invocation still names itself (`{verb}:{instance}`), so only its member lists must be empty. */
export function requireBrowserActorCommandBackboneProjectionV1(publication: Readonly<{ projection: BrowserActorCommandMutationProjectionV1 }>, envelopes: readonly BrowserActorCommandBackboneEnvelopeV1[]): void {
  const { mutations, inverseGroup } = publication.projection;
  if (mutations.length !== envelopes.length || mutations.length > BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM) throw new Error("action-publication-unprojected");
  if (mutations.length === 0) {
    if (inverseGroup.mutations.length !== 0 || inverseGroup.inverseMutations.length !== 0 || (inverseGroup.memberEdits?.length ?? 0) !== 0) throw new Error("action-publication-unprojected");
    return;
  }
  const invocationId = mutations[0]!.invocationId,
    inverseOperations = decodeExactOpsVector(mutations[0]!.inverse.inverseDiff.payload);
  if (
    invocationId.length === 0 ||
    inverseGroup.invocationId !== invocationId ||
    inverseGroup.mutations.length !== mutations.length ||
    inverseGroup.inverseMutations.length !== mutations.length ||
    (inverseGroup.memberEdits?.length ?? 0) !== 0 ||
    inverseOperations.length !== envelopes.length
  ) throw new Error("action-publication-unprojected");
  for (let index = 0; index < mutations.length; index++) {
    const mutation = mutations[index]!,
      envelope = envelopes[index]!,
      inverse = inverseGroup.inverseMutations[index]!;
    if (
      mutation.id !== envelope.mutation_id ||
      mutation.invocationId !== invocationId ||
      mutation.author !== envelope.actor ||
      !equalStrings(mutation.dependencies, envelope.dependencies) ||
      mutation.diff.schema !== `${envelope.diff.schema}.operation` ||
      mutation.inverse.inverseDiff.schema !== `${envelope.inverse.schema}.operation.inverse` ||
      mutation.inverse.targetMutation !== mutation.id ||
      inverseGroup.mutations[index] !== mutation.id ||
      inverse.targetMutation !== mutation.inverse.targetMutation ||
      inverse.baseVersion !== mutation.inverse.baseVersion ||
      inverse.undoPolicy !== mutation.inverse.undoPolicy ||
      !equalStrings(inverse.dependencies, mutation.inverse.dependencies ?? []) ||
      !equalBytes(mutation.diff.payload, envelope.diff.payload) ||
      !equalBytes(inverseOperations[index]!, envelope.inverse.payload) ||
      !equalBytes(mutation.inverse.inverseDiff.payload, mutations[0]!.inverse.inverseDiff.payload) ||
      !equalBytes(inverse.inverseDiff.payload, mutation.inverse.inverseDiff.payload) ||
      BigInt(mutation.timestamp.actor) !== envelope.timestamp.actor ||
      BigInt(mutation.timestamp.physical_ms) !== envelope.timestamp.physical_ms ||
      BigInt(mutation.timestamp.logical) !== envelope.timestamp.logical
    ) throw new Error("action-publication-unprojected");
  }
}

/** 💡️ Converts exact WIT intents into host effects without guest callback or document authority. */
export function encodeBrowserActorHostEffectV1(value: unknown): readonly number[] {
  const effect = exact(value, ["tag", "val"]);
  const projected = effect.tag === "request-inference-proposal" ? projection({ requestInferenceProposal: effect.val })
    : effect.tag === "open-external-url" ? projection({ openExternalUrl: effect.val }) : null;
  if (projected === null) throw new Error("browser-actor-publication: unsupported host effect");
  return Array.from(encodePackValue(projected));
}

/** 🛂️ Validates every effect before publishing any member of a retained action result. */
export function decodeBrowserActorHostEffectsV1(value: unknown): readonly BrowserActorHostEffectV1[] {
  return parseBrowserActorHostEffectBytesV1(value).map((bytes) => {
    const raw = new Uint8Array(bytes);
    const effect = projection(decodePackValue(raw));
    canonical(raw, encodePackValue(effect));
    return effect;
  });
}

/** 🪪️ A replaced owner cannot publish, including replacement caused by an earlier effect in this batch. */
export async function publishBrowserActorHostEffectsV1(value: unknown, current: () => boolean, publish: (effect: BrowserActorHostEffectV1) => void | Promise<void>): Promise<void> {
  const effects = decodeBrowserActorHostEffectsV1(value);
  for (const effect of effects) {
    if (!current()) throw new Error("browser-actor-publication: owner retired");
    await publish(effect);
  }
}
