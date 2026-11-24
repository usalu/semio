import { boundedServicePayloadV1 } from "../../../💡️inference/🔌️service/🟦️.ts";
import type { Effect, ServiceOperationPayloadV1 } from "@semio-tech/framework";
import { wireOptionValue } from "../../../🎭️actor/🖼️wire-turn/🟦️.ts";
/** 📤️ Canonical guest publications and Shell-owned inference intent effects. */
import { decodeAppFrame, encodeAppFrame } from "../../../🧾️app-channel/🎞️frame-codec/🟦️.ts";
import { faultDisplayMessage } from "../../../🧾️app-channel/📋️report-decoders/🟦️.ts";
import { decodeInvocationResultPacks, decodePackValue, encodePackValue, packValueToExactJson } from "../../../🎁️packs/🟦️.ts";
import { BROWSER_ACTOR_ACTION_MUTATION_MAXIMUM, BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES, parseBrowserActorHostEffectBytesV1 } from "../🟦️.ts";

/** 🕰️ A pack-encoded `HistoryPatch` the guest published with a result, forwarded verbatim to the Shell's History projection. */
export type BrowserActorHistoryPatchBytesV1 = readonly number[] | null;
/** 👥️ The guest's own presence pack, declared-broadcast interaction slice and tool-run and history-edit summaries, as its
 * last `AppFrame::Ephemeral` carried them (contract-freeze §C7.6) — what a presence heartbeat of the actor-bound document
 * publishes for this human. */
export type BrowserActorEphemeralSnapshotV1 = Readonly<{ presence: readonly number[]; presenceGeneration: number; transientGeneration: number; interaction: readonly number[]; toolRun: readonly number[]; historyEdit: readonly number[] }>;
export type BrowserActorUnsolicitedPublicationV1 =
  | { readonly kind: "ephemeral"; readonly snapshot: BrowserActorEphemeralSnapshotV1 }
  | { readonly kind: "merge-report" }
  | { readonly kind: "operation-completed"; readonly operation: bigint; readonly revision: bigint; readonly historyPatch: BrowserActorHistoryPatchBytesV1 }
  | { readonly kind: "completion"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 };
export type BrowserActorIntentPublicationV1 = { readonly kind: "emit" } | { readonly kind: "error"; readonly reason: string; readonly detail: string } | BrowserActorUnsolicitedPublicationV1;
type BrowserActorCommandMutationProjectionV1 = ReturnType<typeof decodeInvocationResultPacks>;
export type BrowserActorCommandPublicationV1 = { readonly kind: "invocation"; readonly projection: BrowserActorCommandMutationProjectionV1; readonly historyPatch: BrowserActorHistoryPatchBytesV1 } | { readonly kind: "error"; readonly reason: string; readonly detail: string } | BrowserActorUnsolicitedPublicationV1;
export type BrowserActorCommandBackboneEnvelopeV1 = Readonly<{
  mutation_id: string;
  actor: string;
  dependencies: readonly string[];
  diff: Readonly<{ schema: string; payload: Readonly<Uint8Array> }>;
  inverse: Readonly<{ schema: string; payload: Readonly<Uint8Array> }>;
  timestamp: Readonly<{ actor: bigint; physical_ms: bigint; logical: bigint }>;
}>;
type BrowserActorHostEffectKeyV1 = "requestServiceOperation" | "openExternalUrl" | "replayShellCommand" | "openPluginInstance"
  | "setPanel" | "setActiveUtility" | "setActiveTool" | "notify" | "clipboardWrite" | "openDialog" | "requestFileOpen" | "downloadMediaExport";
export type BrowserActorHostEffectV1 = {
  [Key in BrowserActorHostEffectKeyV1]: Extract<Effect, Readonly<Record<Key, unknown>>>;
}[BrowserActorHostEffectKeyV1];

function record(value: unknown): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value) || Object.getPrototypeOf(value) !== Object.prototype) throw new Error("browser-actor-publication: invalid object");
  return value as Readonly<Record<string, unknown>>;
}

function exact(value: unknown, required: readonly string[], optional: readonly string[] = []): Readonly<Record<string, unknown>> {
  const fields = record(value);
  if (required.some(field => !Object.hasOwn(fields, field)) || Object.keys(fields).some(field => !required.includes(field) && !optional.includes(field))) {
    throw new Error("browser-actor-publication: invalid exact fields");
  }
  return fields;
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

function text(value: unknown): string {
  if (typeof value !== "string" || new TextEncoder().encode(value).length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: invalid effect text");
  return value;
}

function identity(value: unknown, maximum = 256): string {
  const id = text(value);
  if (!id || new TextEncoder().encode(id).length > maximum || /\p{Cc}/u.test(id)) throw new Error("browser-actor-publication: invalid effect identity");
  return id;
}

function integer(value: unknown, maximum = Number.MAX_SAFE_INTEGER): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value > maximum) throw new Error("browser-actor-publication: invalid effect integer");
  return value;
}

function clipboardFragment(value: unknown): unknown {
  const { schema, mediaType, dslText, packBytes, sourceApp, label } = exact(value, ["schema", "mediaType", "dslText", "sourceApp", "label"], ["packBytes"]);
  const { class: mediaClass, form } = exact(mediaType, ["class", "form"]);
  if (packBytes !== undefined && (!Array.isArray(packBytes) || packBytes.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES)) throw new Error("browser-actor-publication: invalid clipboard bytes");
  return {
    schema: identity(schema), mediaType: { class: identity(mediaClass), form: identity(form) }, dslText: text(dslText),
    ...(packBytes === undefined ? {} : { packBytes: packBytes.map(byte => integer(byte, 255)) }),
    sourceApp: identity(sourceApp), label: text(label),
  };
}

function projection(value: unknown): BrowserActorHostEffectV1 {
  const effect = record(value);
  const [key] = Object.keys(effect);
  if (key === undefined) throw new Error("browser-actor-publication: empty host effect");
  const data = exact(effect, [key])[key];
  switch (key) {
    case "requestServiceOperation": {
      const operation = exact(data, ["owner", "serviceId", "action", "payload"]);
      return { requestServiceOperation: {
        owner: identity(operation.owner, 128), serviceId: identity(operation.serviceId), action: identity(operation.action, 64),
        payload: boundedServicePayloadV1(operation.payload) as ServiceOperationPayloadV1,
      } };
    }
    case "openExternalUrl": {
      const urlText = identity(exact(data, ["url"]).url, 2048);
      if (!/^https?:\/\//u.test(urlText) || /[\u0000-\u0020\u007f]/u.test(urlText)) throw new Error("browser-actor-publication: invalid external URL");
      const url = new URL(urlText);
      if (!url.hostname || url.username || url.password) throw new Error("browser-actor-publication: invalid external URL");
      return { openExternalUrl: { url: urlText } };
    }
    case "replayShellCommand": {
      const { actionId, args } = exact(data, ["actionId"], ["args"]);
      return { replayShellCommand: { actionId: identity(actionId), ...(args === undefined ? {} : { args }) } };
    }
    case "openPluginInstance": {
      const { pluginId, appId, osInstanceId, document } = exact(data, ["pluginId", "appId"], ["osInstanceId", "document"]);
      const target = document === undefined ? undefined : exact(document, ["documentId", "schema"], ["spaceId"]);
      return { openPluginInstance: {
        pluginId: identity(pluginId), appId: identity(appId),
        ...(osInstanceId === undefined ? {} : { osInstanceId: identity(osInstanceId) }),
        ...(target === undefined ? {} : { document: {
          documentId: identity(target.documentId), schema: identity(target.schema),
          ...(target.spaceId === undefined ? {} : { spaceId: identity(target.spaceId) }),
        } }),
      } };
    }
    case "setPanel":
      return { setPanel: { panelJson: text(exact(data, ["panelJson"]).panelJson) } };
    case "setActiveUtility": {
      const { windowId, utilityId } = exact(data, ["windowId", "utilityId"]);
      return { setActiveUtility: { windowId: identity(windowId), utilityId: text(utilityId) } };
    }
    case "setActiveTool":
      return { setActiveTool: { toolId: text(exact(data, ["toolId"]).toolId) } };
    case "notify":
      return { notify: { message: text(exact(data, ["message"]).message) } };
    case "clipboardWrite":
      return { clipboardWrite: { fragment: clipboardFragment(exact(data, ["fragment"]).fragment) } };
    case "openDialog": {
      const { req, dialogId, args } = exact(data, ["req", "dialogId"], ["args"]);
      return { openDialog: { req: integer(req), dialogId: identity(dialogId), ...(args === undefined ? {} : { args: record(args) }) } };
    }
    case "requestFileOpen": {
      const { req, accept, readAs, importAction, args, multiple, maxFileBytes } = exact(data, ["req", "accept", "importAction", "multiple"], ["readAs", "args", "maxFileBytes"]);
      if (typeof multiple !== "boolean") throw new Error("browser-actor-publication: invalid picker multiplicity");
      return { requestFileOpen: {
        req: integer(req), accept: text(accept), importAction: identity(importAction), multiple,
        ...(readAs === undefined ? {} : { readAs: identity(readAs) }),
        ...(args === undefined ? {} : { args }),
        ...(maxFileBytes === undefined ? {} : { maxFileBytes: integer(maxFileBytes, 0xffffffff) }),
      } };
    }
    case "downloadMediaExport": {
      const { filename, mimeType, data: contents, encoding } = exact(data, ["filename", "mimeType", "data"], ["encoding"]);
      return { downloadMediaExport: {
        filename: identity(filename), mimeType: identity(mimeType), data: text(contents),
        ...(encoding === undefined ? {} : { encoding: identity(encoding) }),
      } };
    }
    default:
      throw new Error(`browser-actor-publication: unsupported host effect ${key.slice(0, 64)}`);
  }
}

function ephemeralPublication(frame: Extract<ReturnType<typeof decodeAppFrame>, { readonly Ephemeral: unknown }>["Ephemeral"]): Extract<BrowserActorUnsolicitedPublicationV1, { readonly kind: "ephemeral" }> {
  return { kind: "ephemeral", snapshot: { presence: Array.from(frame.presence), presenceGeneration: frame.presence_generation, transientGeneration: frame.transient_generation, interaction: Array.from(frame.interaction), toolRun: Array.from(frame.tool_run), historyEdit: Array.from(frame.history_edit) } };
}

/** 📬️ Accepts only ordinary, unsolicited intent completion frames, the `Ephemeral` snapshot every guest exchange appends
 * (contract-freeze §C7.6) and a typed operation's unsolicited UI progress; operation bytes remain guest-owned. */
export function decodeBrowserActorIntentPublicationV1(bytes: Uint8Array): BrowserActorIntentPublicationV1 {
  if (bytes.length === 0 || bytes.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: invalid frame size");
  const frame = decodeAppFrame(bytes);
  canonical(bytes, encodeAppFrame(frame));
  if ("Emit" in frame && frame.Emit.in_reply_to === 0) return { kind: "emit" };
  if ("Error" in frame && frame.Error.in_reply_to === null) return { kind: "error", reason: "action-guest-refused", detail: faultDisplayMessage(frame.Error.fault, decodePackValue) };
  if ("Ephemeral" in frame) return ephemeralPublication(frame.Ephemeral);
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
    return { kind: "operation-completed", operation: frame.OperationCompleted.operation, revision: frame.OperationCompleted.revision, historyPatch: historyPatchBytes(frame.OperationCompleted.history_patch) };
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
  if ("Ephemeral" in frame) return ephemeralPublication(frame.Ephemeral);
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
  if ("Error" in frame && frame.Error.in_reply_to === actionSequence) return { kind: "error", reason: "action-guest-refused", detail: faultDisplayMessage(frame.Error.fault, decodePackValue) };
  if ("Ephemeral" in frame) return ephemeralPublication(frame.Ephemeral);
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

function exactPack(value: unknown, maximum: number): unknown {
  if (!(value instanceof Uint8Array) && !Array.isArray(value)) throw new Error("browser-actor-publication: invalid effect pack");
  if (value.length === 0 || value.length > maximum || Array.from(value).some(byte => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error("browser-actor-publication: invalid effect pack");
  const raw = Uint8Array.from(value);
  const decoded = decodePackValue(raw);
  canonical(raw, encodePackValue(decoded));
  return packValueToExactJson(decoded);
}

function optionalPack(value: unknown): unknown {
  const unpacked = wireOptionValue(value);
  return unpacked === undefined ? undefined : exactPack(unpacked, BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES);
}

function wireRequestId(value: unknown): number {
  return integer(typeof value === "bigint" ? Number(value) : value);
}

function projectWireEffect(value: unknown): unknown {
  const { tag, val } = exact(value, ["tag", "val"]);
  switch (tag) {
    case "request-service-operation": {
      const operation = exact(val, ["owner", "serviceId", "action", "payload"]);
      return { requestServiceOperation: { ...operation, payload: exactPack(operation.payload, 16384) } };
    }
    case "open-external-url":
      return { openExternalUrl: val };
    case "replay-shell-command": {
      const { actionId, args } = exact(val, ["actionId"], ["args"]);
      return { replayShellCommand: { actionId, args: optionalPack(args) } };
    }
    case "open-plugin-instance": {
      const { pluginId, appId, osInstanceId, document } = exact(val, ["pluginId", "appId"], ["osInstanceId", "document"]);
      const target = wireOptionValue(document);
      const bound = target === undefined ? undefined : exact(target, ["documentId", "schema"], ["spaceId"]);
      return { openPluginInstance: {
        pluginId, appId, osInstanceId: wireOptionValue(osInstanceId),
        document: bound === undefined ? undefined : { ...bound, spaceId: wireOptionValue(bound.spaceId) },
      } };
    }
    case "set-panel":
      return { setPanel: val };
    case "set-active-utility":
      return { setActiveUtility: val };
    case "set-active-tool":
      return { setActiveTool: val };
    case "notify":
      return { notify: val };
    case "clipboard-write":
      return { clipboardWrite: { fragment: exactPack(exact(val, ["fragment"]).fragment, BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) } };
    case "open-dialog": {
      const { req, params } = exact(val, ["req", "params"]);
      const { dialogId, args } = exact(params, ["dialogId"], ["args"]);
      return { openDialog: { req: wireRequestId(req), dialogId, args: optionalPack(args) } };
    }
    case "request-file-open": {
      const { req, params } = exact(val, ["req", "params"]);
      const { accept, readAs, importAction, args, multiple, maxFileBytes } = exact(params, ["accept", "importAction", "multiple"], ["readAs", "args", "maxFileBytes"]);
      return { requestFileOpen: {
        req: wireRequestId(req), accept, readAs: wireOptionValue(readAs), importAction,
        args: optionalPack(args), multiple, maxFileBytes: wireOptionValue(maxFileBytes),
      } };
    }
    case "download-media-export": {
      const { filename, mimeType, data, encoding } = exact(val, ["filename", "mimeType", "data"], ["encoding"]);
      return { downloadMediaExport: { filename, mimeType, data, encoding: wireOptionValue(encoding) } };
    }
    default:
      throw new Error(`browser-actor-publication: unsupported host effect ${String(tag).slice(0, 64)}`);
  }
}

/** 💡️ Converts exact WIT intents into host effects without guest callback or document authority. */
export function encodeBrowserActorHostEffectV1(value: unknown): readonly number[] {
  const encoded = encodePackValue(projection(projectWireEffect(value)));
  if (encoded.length > BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES) throw new Error("browser-actor-publication: oversized host effect");
  return Array.from(encoded);
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
