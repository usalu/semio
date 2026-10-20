// #region 🧲️Header
/** 🖼️ Renderer-agnostic interpretation of one `ShardClient.turn()` result — coercing its
 * opaque `unknown` shape, decoding retained-mode `UiPatch` ops onto a kept tree, and translating a
 * raw wire `effect` variant into the shared `kernel::Effect` TS union. Lifted out of
 * `PluginRuntime/🟦️.tsx`'s `🔖️ActorAdapter`/`🔖️RetainedUiPatch` regions (MICROKERNEL-POOLED-
 * ACTOR-PLUGIN-RUNTIME, `wgpu-web-shard`) so a second renderer target does not reimplement this wire
 * parsing independently — the exact "third divergent copy" that packet's own brief warns against.
 * `decodePackValue` is injected rather than imported (`@semio-tech/framework-os`'s own codec) so this
 * package stays free of a hard dependency on that product package, matching the `🎭️actor` crate's own
 * "stay pure, transports/codecs are injected" discipline (📌️important.md naming hazards). `PluginRuntime`
 * predates this module and still carries its own inline copy (outside `wgpu-web-shard`'s lease to
 * edit) — a future packet should point it here too. */
// #endregion 🧲️Header

// #region 🔌️Imports
import type { Effect } from "@semio-tech/framework-effect";
import { jobPlacementFromWireName } from "@semio-tech/framework-effect/spawned-job";
import { videoRenderProgramFromWire } from "@semio-tech/framework-effect/video-render-program";
import { TYPED_OPERATION_ACK_MAGIC, TYPED_OPERATION_LANE_FAULT, TYPED_OPERATION_LANE_TERMINAL, TYPED_OPERATION_PAGE_HEADER_BYTES, TYPED_OPERATION_PAGE_MAGIC, TYPED_OPERATION_RESULT_LANE_MAX, TYPED_OPERATION_RESULT_PAGE_BYTES, TYPED_OPERATION_TOKEN_BYTES, WIRE_SEND_MESSAGE_ROUTED_TARGETS, type WireVariant, coerceWireBytes, isRoutedWireSendMessage, scanTypedOperationPages, shellFrameBytes, shellMessageKind, typedOperationAcknowledgements, typedOperationPageAnswerV1, typedOperationResult, wireSendMessageTargetTag } from "./🔖️turn-result/🟦️.ts";
// #endregion 🔌️Imports

//#region 🔖️EffectWire
/** 🪪️ Decodes the schema-owned nested request without narrowing its u64 identity or interpreting JSON as a pack. */
export function wireExtensionInvocation(effect: WireVariant): Extract<Effect, { readonly invokeExtension: unknown }> {
  const value = effect.val as { readonly req?: unknown; readonly params?: { readonly extensionId?: unknown; readonly capability?: unknown; readonly payload?: unknown } } | undefined;
  const req = value?.req;
  if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("extension.request-id-invalid");
  const params = value?.params;
  if (typeof params?.extensionId !== "string" || !params.extensionId || typeof params.capability !== "string" || !params.capability) throw new Error("extension.request-address-invalid");
  const requestJson = new TextDecoder("utf-8", { fatal: true }).decode(coerceWireBytes(params.payload));
  return { invokeExtension: { req, extensionId: params.extensionId, capability: params.capability, requestJson } };
}

/** ↩️ Decodes `respond-effect` (`🔌️plugin/🧬️schema/📜️.wit` — `{ req, outcome: respond-result }`)
 * without narrowing its u64 identity. `respond` is the ANSWER half of the one inbound-call seam the
 * ABI has (`request-event`); it is addressed by `req` alone, so a host that submitted the request
 * correlates on that id and never on an actor-local reply channel. The outcome keeps the WIT arm
 * names (`ok`/`fault`) rather than Rust's `RequestOutcome::{Ok,Err}` because this is the wire the
 * host reads, and because it is byte-for-byte the shape `captureExtensionCompletion.complete`
 * already takes — one vocabulary for both directions of the same door. */
export function wireRespondAnswer(effect: WireVariant): Extract<Effect, { readonly respond: unknown }> {
  const value = effect.val as { readonly req?: unknown; readonly outcome?: WireVariant } | undefined;
  const req = value?.req;
  if (typeof req !== "bigint" || req <= 0n || req > 0xffffffffffffffffn) throw new Error("respond.request-id-invalid");
  const outcome = value?.outcome;
  if (outcome?.tag !== "ok" && outcome?.tag !== "fault") throw new Error("respond.outcome-invalid");
  const bytes = coerceWireBytes(outcome.val);
  return { respond: { req, result: outcome.tag === "ok" ? { ok: bytes } : { fault: bytes } } };
}

/** 🎁️ Unwraps ONE WIT `option<T>` as it really reaches a renderer door. jco's direct binding hands
 * over the bare value (`T | undefined`); the actor boundary hands over the variant record
 * (`{tag: "some", val}` / `{tag: "none"}`). Both shapes are real on the same field, so a decoder that
 * reads only one of them silently loses it — which is exactly what happened to
 * `download-media-export.encoding` and turned every binary export in the repo into base64 TEXT under
 * a binary file name (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3). */
export function wireOptionValue(raw: unknown): unknown {
  if (raw === null || typeof raw !== "object" || !("tag" in (raw as Record<string, unknown>))) return raw;
  const option = raw as { readonly tag?: unknown; readonly val?: unknown };
  if (option.tag === "some") return option.val;
  if (option.tag === "none") return undefined;
  return raw;
}

/** 🎁️ {@link wireOptionValue} narrowed to `option<string>` — anything that is not a string is absent. */
export function wireOptionText(raw: unknown): string | undefined {
  const value = wireOptionValue(raw);
  return typeof value === "string" ? value : undefined;
}

/** ⬇️ The one reader of `download-media-export.encoding`, shared by the React `PluginRuntime` door and
 * the wgpu `plugin-bridge` one. What the string MEANS is the kernel's contract
 * (`kernel::mediaExportBytes` / `kernel::media_export_bytes`); this only recovers it from the wire. */
export const wireMediaExportEncoding = wireOptionText;

/** ⬇️ Decodes `download-media-export-effect` (`🔌️plugin/🧬️schema/📜️.wit`). One decoder for both
 * renderers, for the same reason {@link wireRespondAnswer} is one: the React door used to carry its
 * own copy that read `encoding` flat, and the wgpu door carried no case at all — so the same guest
 * export produced a corrupt file on one target and nothing at all on the other. */
export function wireDownloadMediaExport(effect: WireVariant): Extract<Effect, { readonly downloadMediaExport: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  return {
    downloadMediaExport: {
      filename: String(value.filename ?? ""),
      mimeType: String(value.mimeType ?? value["mime-type"] ?? ""),
      data: String(value.data ?? ""),
      encoding: wireMediaExportEncoding(value.encoding),
    },
  };
}

/** 📤️ Decodes the packed IconRenderExport item envelope shared by both browser renderers. */
export function wireIconRenderExport(effect: WireVariant, decodePackValue: (bytes: Uint8Array) => unknown): Extract<Effect, { readonly iconRenderExport: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  const items = decodePackValue(coerceWireBytes(value.items));
  if (!Array.isArray(items)) throw new Error("icon-export.items-invalid");
  return { iconRenderExport: { items: items.map((item: unknown) => {
    if (item === null || typeof item !== "object" || Array.isArray(item)) throw new Error("icon-export.items-invalid");
    const record = item as Record<string, unknown>;
    if (typeof record.filename !== "string" || !Object.hasOwn(record, "request") || Object.keys(record).some((key) => key !== "filename" && key !== "request")) throw new Error("icon-export.items-invalid");
    return { filename: record.filename, request: record.request };
  }) } };
}

/** 🎥️ Decodes `video-render-export-effect` (`🔌️plugin/🧬️schema/📜️.wit`): `program` crosses as a `pack` and is normalised by
 * `kernel::videoRenderProgramFromWire`. One decoder for both renderer doors, like {@link wireDownloadMediaExport}. */
export function wireVideoRenderExport(effect: WireVariant, decodePackValue: (bytes: Uint8Array) => unknown): Extract<Effect, { readonly videoRenderExport: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  const decoded = value.program !== null && typeof value.program === "object" && !ArrayBuffer.isView(value.program) && !Array.isArray(value.program) && "schema" in value.program;
  const program = decoded ? value.program : decodePackValue(coerceWireBytes(value.program));
  return { videoRenderExport: { filename: String(value.filename ?? ""), program: videoRenderProgramFromWire(program) } };
}

/** 🧵️ Decodes `spawn-job-effect` (`🔌️plugin/🧬️schema/📜️.wit`) without narrowing its u64 identity.
 * `job` IS the guest's parked request id, so a `number` here would mis-resolve a long-lived actor's
 * futures and the `start-job`/`step-job` door rejects one outright. `placement` crosses as a BARE
 * string (jco lowers a WIT `enum` that way, measured on 6118: `placement: "isolated"`), and an
 * unknown spelling is refused rather than defaulted.
 *
 * This is the effect EVERY framework reserved tool verb is delivered as — `interactionSelect`,
 * `interactionHover`, `clearSelection` — so a renderer door without this case publishes every
 * interaction perfectly and applies none (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-world3d-interaction-2026-09-13.md` §7). */
export function wireSpawnJob(effect: WireVariant): Extract<Effect, { readonly spawnJob: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  const job = wireJobIdentity(value.job, "spawn-job");
  const kind = wireOptionValue(value.kind);
  if (typeof kind !== "string" || !kind) throw new Error("spawn-job.kind-invalid");
  const placement = jobPlacementFromWireName(wireOptionValue(value.placement));
  if (placement === undefined) throw new Error(`spawn-job.placement-invalid ${JSON.stringify(value.placement)?.slice(0, 60)}`);
  return { spawnJob: { job, kind, input: coerceWireBytes(wireOptionValue(value.input) ?? []), placement } };
}

/** 🧵️ Decodes `cancel-job-effect` — the same u64 identity, nothing else. */
export function wireCancelJob(effect: WireVariant): Extract<Effect, { readonly cancelJob: unknown }> {
  const value = (effect.val ?? {}) as Record<string, unknown>;
  return { cancelJob: { job: wireJobIdentity(value.job, "cancel-job") } };
}

/** 🪪️ One `u64` job identity off the wire. jco hands a `bigint`; a decimal string is the shape a
 * fixture (and `structuredClone`-free transport) can carry, and both must mean the same id. */
export function wireJobIdentity(raw: unknown, effectTag: string): bigint {
  const value = wireOptionValue(raw);
  const job = typeof value === "bigint" ? value : typeof value === "string" && /^\d+$/.test(value) ? BigInt(value) : typeof value === "number" && Number.isSafeInteger(value) && value >= 0 ? BigInt(value) : null;
  if (job === null || job <= 0n || job > 0xffffffffffffffffn) throw new Error(`${effectTag}.job-identity-invalid`);
  return job;
}


/** 🚧️ Best-effort conversion of a raw WIT `effect` variant into the friendly `Effect` union
 * `🎠️kernel` already declares — Rust `kernel::Effect`'s externally-tagged serde shape,
 * which every downstream consumer already expects. Covers the effect kinds a renderer commonly
 * branches on; an effect kind with no case here degrades to an honest console-logged drop rather
 * than guessing an unverified shape. */
export function wireEffectToFriendly(effect: WireVariant, decodePackValue: (bytes: Uint8Array) => unknown): Effect | null {
  const val = (effect.val ?? {}) as Record<string, unknown>;
  const str = (key: string): string => String(val[key] ?? "");
  const num = (key: string): number => Number(val[key] ?? 0);
  const packField = (key: string): unknown => (val[key] !== undefined ? decodePackValue(coerceWireBytes(val[key])) : undefined);
  const params = (val.params ?? {}) as Record<string, unknown>;
  const some = wireOptionValue;
  const pstr = (key: string): string => String(some(params[key]) ?? "");
  const pnum = (key: string): number => Number(some(params[key]) ?? 0);
  const poptstr = (key: string): string | undefined => {
    const value = some(params[key]);
    return typeof value === "string" ? value : undefined;
  };
  const ppack = (key: string): unknown => {
    const value = some(params[key]);
    return value === undefined ? undefined : decodePackValue(coerceWireBytes(value));
  };
  switch (effect.tag) {
    case "invoke-extension":
      return wireExtensionInvocation(effect);
    case "respond":
      return wireRespondAnswer(effect);
    case "request-sync":
      return "requestSync";
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "download-media-export":
      return wireDownloadMediaExport(effect);
    case "icon-render-export":
      return wireIconRenderExport(effect, decodePackValue);
    case "video-render-export":
      return wireVideoRenderExport(effect, decodePackValue);
    case "spawn-job":
      return wireSpawnJob(effect);
    case "cancel-job":
      return wireCancelJob(effect);
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: pstr("kind"), params: ppack("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "spawn-plugin-instance":
      return {
        spawnPluginInstance: { req: num("req"), pluginId: pstr("pluginId"), appId: pstr("appId"), osInstanceId: poptstr("osInstanceId"), label: poptstr("label"), artifactJson: poptstr("artifactJson") },
      };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined } };
    case "request-file-open": {
      const readAs = poptstr("readAs") ?? poptstr("read-as");
      return {
        requestFileOpen: {
          req: num("req"),
          accept: pstr("accept"),
          ...(readAs === undefined ? {} : { readAs }),
          importAction: pstr("importAction") || pstr("import-action"),
          ...(some(params.args) == null ? {} : { args: ppack("args") }),
          multiple: Boolean(some(params.multiple) ?? false),
        },
      };
    }
    case "dispatch-action":
      return { dispatchAction: { req: num("req"), action: pstr("action"), args: ppack("args"), delayMs: pnum("delayMs") } };
    case "send-message": {
      if (isRoutedWireSendMessage(effect)) return null;
      console.warn(`wireEffectToFriendly: send-message to "${wireSendMessageTargetTag(effect) || "no endpoint"}" has no host route — only ${WIRE_SEND_MESSAGE_ROUTED_TARGETS.join("/")} are consumed`);
      return null;
    }
    default:
      console.warn(`wireEffectToFriendly: unmapped effect "${effect.tag}" dropped — unverified wasm-boundary conversion`);
      return null;
  }
}
//#endregion 🔖️EffectWire
//#region 🧪️Tests
if (import.meta.vitest) {
  const { registerTests1 } = await import("../🧪️tests/📨️effect-wire-routes/🟦️.ts");
  await registerTests1(import.meta.vitest, { shellFrameBytes, wireEffectToFriendly, isRoutedWireSendMessage, wireSendMessageTargetTag, WIRE_SEND_MESSAGE_ROUTED_TARGETS }, { url: import.meta.url });
  const { registerTests1: registerEffectParamsNestingTests } = await import("../🧪️tests/🎁️effect-params-nesting/🟦️.ts");
  await registerEffectParamsNestingTests(import.meta.vitest, { wireEffectToFriendly }, { url: import.meta.url });
  const { registerTests1: registerTypedOperationPageTests } = await import("../🧪️tests/🗞️typed-operation-page/🟦️.ts");
  await registerTypedOperationPageTests(
    import.meta.vitest,
    {
      scanTypedOperationPages,
      shellFrameBytes,
      shellMessageKind,
      typedOperationAcknowledgements,
      typedOperationPageAnswerV1,
      typedOperationResult,
      TYPED_OPERATION_ACK_MAGIC,
      TYPED_OPERATION_LANE_FAULT,
      TYPED_OPERATION_LANE_TERMINAL,
      TYPED_OPERATION_PAGE_HEADER_BYTES,
      TYPED_OPERATION_PAGE_MAGIC,
      TYPED_OPERATION_RESULT_LANE_MAX,
      TYPED_OPERATION_RESULT_PAGE_BYTES,
      TYPED_OPERATION_TOKEN_BYTES,
    },
  );
}
//#endregion 🧪️Tests
