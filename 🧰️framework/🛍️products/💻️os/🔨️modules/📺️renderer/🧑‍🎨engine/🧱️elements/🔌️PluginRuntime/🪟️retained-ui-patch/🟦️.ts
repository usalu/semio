/** 🪟️ The retained per-actor UI document: patch application, snapshot, hash and built-node projection, and the wire-effect to friendly-`Effect` mapping.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type Effect } from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework-os";
import { type BuiltNode, type Component, type UiPatchOp, type UiSnapshot } from "@semio-tech/framework";
import { applyUiPatch, emptyUiDocumentState, type UiDocumentState } from "../../📃️UiDocumentStore/🟦️.tsx";
import type { RetainedUiNodeRecord } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";
import {
  isRoutedWireSendMessage,
  WIRE_SEND_MESSAGE_ROUTED_TARGETS,
  wireDownloadMediaExport,
  wireIconRenderExport,
  wireExtensionInvocation,
  wireOptionValue,
  wireRespondAnswer,
  wireSendMessageTargetTag,
  wireVideoRenderExport
} from "../../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import {
  coerceWireBytes,
  decodeWirePack,
  fnv1aHex,
  wireNatural,
  type WireVariant
} from "../🪝️wire-codec/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
//#region 🔖️RetainedUiPatch
/** 🗄️ The retained per-actor document — a `📃️UiDocumentStore`-shaped state, not the store class itself
 * (this file has no per-node React subscribers to serve; it only needs the flat table to hash/forward
 * to `refreshUi` callers). Reuses `📃️UiDocumentStore`'s own `uiDocumentStateFromSnapshot` so this file
 * and the React tree apply the identical algorithm to the identical wire shape — never a second,
 * drifting reimplementation. */
export type RetainedSurface = UiDocumentState;

/**
 * 🖼️ H1-react (design-runtime.md §1 `SceneStore` / packet brief item 2) — reconciles one
 * `UiPatch`'s ops onto `previous` (the last body this file retained for the surface), so the UI
 * thread reads an already-reconciled tree instead of awaiting a plugin turn. Reuses the transactional
 * `📃️UiDocumentStore` patch applicator so revision checks, graph validation, quotas, and every semantic
 * operation stay identical to the renderer's subscribed document store.
 */
export function applyUiPatchToRetained(
  previous: RetainedSurface | null,
  patch: { readonly surface: string; readonly revision: number | bigint; readonly baseRevision: number | bigint; readonly ops: readonly UiPatchOp[] },
): { readonly surface: RetainedSurface | null; readonly desynced: boolean } {
  const surfaceId = patch.surface || previous?.surface;
  if (!surfaceId) return { surface: previous, desynced: true };
  if (previous && previous.surface !== surfaceId) return { surface: previous, desynced: true };
  const state = previous ?? emptyUiDocumentState(surfaceId);
  const applied = applyUiPatch(state, {
    surface: surfaceId,
    revision: wireNatural(patch.revision, "patch.revision"),
    baseRevision: wireNatural(patch.baseRevision, "patch.baseRevision"),
    ops: [...patch.ops],
  });
  return applied.ok ? { surface: applied.state, desynced: false } : { surface: previous, desynced: true };
}

/** 🔁️ Rebuilds a plain `UiSnapshot` (nodes flattened back to an array) from a retained
 * `UiDocumentState` — needed only where a `Map` must cross a `JSON.stringify`/hash boundary; the
 * store's own consumers read `.nodes` directly and never need this round trip. */
export function retainedSurfaceToSnapshot(surface: RetainedSurface): UiSnapshot {
  return { surface: surface.surface, revision: surface.revision, root: surface.root ?? 0, nodes: [...surface.nodes.values()], layoutEpoch: 0n };
}

export function retainedSurfaceHash(snapshot: UiSnapshot): string {
  const json = JSON.stringify(snapshot, (_key, value) => (typeof value === "bigint" ? value.toString() : value));
  return fnv1aHex(new TextEncoder().encode(json)) + ":" + String(snapshot.revision);
}

export function retainedSurfaceToBuiltNode(surface: RetainedSurface): BuiltNode | null {
  if (surface.root === null) return null;
  const build = (id: number): BuiltNode => {
    const record = surface.nodes.get(id);
    if (!record) throw new Error(`retained UI surface ${surface.surface} references missing node ${id}`);
    return {
      key: record.key,
      component: record.component,
      layout: record.layout,
      style: record.style,
      activity: record.activity,
      disabled: record.disabled,
      accessibility: record.accessibility,
      bindings: record.bindings,
      menu: record.menu,
      children: record.children.map(build),
    };
  };
  return build(surface.root);
}

/** 🪟️ Copies only an opaque surface payload whose native byte view cannot cross the host response boundary. */
export function ownedUiComponentToBuilt(component: RetainedUiNodeRecord["component"]): Component {
  if (component.type !== "surface") return component;
  return { ...component, doc: { bytes: Array.from({ length: component.doc.bytes.length }, (_, index) => component.doc.bytes.byteAt(index)) } };
}
//#endregion 🔖️RetainedUiPatch

/** 🚧️ Best-effort conversion of a raw WIT `effect` variant (`{tag, val}`, see `WireVariant`'s doc for
 * the unverified-boundary caveat) into the friendly `Effect` union `🎠️kernel/🟦️.ts` already
 * declares — Rust `kernel::Effect`'s externally-tagged serde shape (`{effectName: {...fields}}` /
 * `"requestSync"`), which is what every downstream consumer (`applyHostEffects` and friends) already
 * expects. Covers the effect kinds this renderer actually branches on; an effect kind with no case
 * here degrades to an honest `[TRACE]`-logged drop rather than guessing an unverified shape. */
export function wireEffectToFriendly(effect: WireVariant): Effect | null {
  const val = (effect.val ?? {}) as Record<string, unknown>;
  // 🧬️ Every `request-id`-carrying effect nests its payload in a `*-params` record (`🔌️plugin/🧬️schema/📜️.wit`,
  // lowered by `⚛️reactor/🦀️.rs`'s `wit::Effect::…({ req, params })`); the request-less effects keep their
  // fields at the top level. `wireExtensionInvocation` already reads the nested shape — the cases below
  // used to read those same records flatly, which silently produced empty ids: a `dispatch-action` with
  // `action: ""` reached the plugin as `window kind procedural-main does not own action ""`.
  const params = (val.params && typeof val.params === "object" ? val.params : {}) as Record<string, unknown>;
  const str = (key: string): string => String(val[key] ?? "");
  const num = (key: string): number => Number(val[key] ?? 0);
  // 🎁️ ONE WIT `option<T>` reader, shared with `🖼️wire-turn.ts` rather than restated here.
  const optionValue = wireOptionValue;
  const packField = (key: string): unknown => {
    const raw = optionValue(val[key]);
    return raw !== undefined ? decodeWirePack(raw, `wire.${key}`) : undefined;
  };
  const paramStr = (key: string): string => String(params[key] ?? "");
  const paramNum = (key: string): number => Number(params[key] ?? 0);
  const paramText = (key: string): string | undefined => (typeof params[key] === "string" ? (params[key] as string) : undefined);
  const paramPack = (key: string): unknown => {
    const raw = optionValue(params[key]);
    return raw !== undefined ? decodeWirePack(raw, `wire.params.${key}`) : undefined;
  };
  switch (effect.tag) {
    case "request-sync":
      return "requestSync";
    case "load-document":
      return { loadDocument: { pack: Array.from(coerceWireBytes(val.docPack)), spr: Array.from(coerceWireBytes(val.spr)) } };
    // ⬇️ ONE decoder for both renderers (`🖼️wire-turn.ts`) — this door used to read `encoding` flat,
    // and a WIT `option<string>` reaches it as `{tag:"some", val}`, so `typeof … === "string"` was
    // always false and every binary export was saved as base64 TEXT under a binary file name.
    case "download-media-export":
      return wireDownloadMediaExport(effect);
    case "icon-render-export":
      return wireIconRenderExport(effect, decodePackWire);
    case "video-render-export":
      return wireVideoRenderExport(effect, (bytes) => decodePackWire(bytes, "$.program"));
    case "notify":
      return { notify: { message: str("message") } };
    case "navigate":
      return { navigate: { uri: str("uri") } };
    case "open-external-url":
      return { openExternalUrl: { url: str("url") } };
    case "set-panel":
      return { setPanel: { panelJson: str("panelJson") } };
    case "clipboard-write":
      return { clipboardWrite: { fragment: packField("fragment") } };
    case "replay-shell-command":
      return { replayShellCommand: { actionId: str("actionId"), args: packField("args") } };
    case "request-file-open": {
      const readAsRaw = optionValue(params["read-as"] ?? params.readAs);
      const mapped = {
        requestFileOpen: {
          req: num("req"),
          accept: paramStr("accept"),
          readAs: typeof readAsRaw === "string" ? readAsRaw : undefined,
          importAction: paramStr("import-action") || paramStr("importAction"),
          multiple: Boolean(optionValue(params.multiple) ?? params.multiple),
        },
      };
      return mapped;
    }
    case "set-active-utility":
      return { setActiveUtility: { windowId: str("windowId"), utilityId: str("utilityId") } };
    case "set-active-tool":
      return { setActiveTool: { toolId: str("toolId") } };
    case "open-window":
      return { openWindow: { req: num("req"), kind: paramStr("kind"), params: paramPack("params") } };
    case "close-window":
      return { closeWindow: { window: num("window") } };
    case "dispatch-action": {
      const action = paramStr("action");
      // 🚫️ An id-less self re-dispatch can only ever be rejected by the owning window kind; drop it at
      // the boundary that decoded it rather than letting `makeEffectDispatchOne` turn it into an
      // unhandled `plugin.internal` rejection with an empty action id in its message.
      if (!action) {
        return null;
      }
      return { dispatchAction: { req: num("req"), action, args: paramPack("args"), delayMs: paramNum("delayMs") } };
    }
    case "open-dialog":
      return { openDialog: { req: num("req"), dialogId: paramStr("dialogId"), args: paramPack("args") as Record<string, unknown> | undefined } };
    case "invoke-extension":
      return wireExtensionInvocation(effect);
    // ↩️ ONE rule for the answer half of the request seam, shared with `🖼️wire-turn.ts`'s own copy of
    // this mapping rather than re-decoded here — the divergence that file's header warns about.
    case "respond":
      return wireRespondAnswer(effect);
    case "spawn-plugin-instance":
      return { spawnPluginInstance: { req: num("req"), pluginId: paramStr("pluginId"), appId: paramStr("appId"), osInstanceId: paramText("osInstanceId"), label: paramText("label"), artifactJson: paramText("artifactJson") } };
    case "open-plugin-instance":
      return { openPluginInstance: { pluginId: str("pluginId"), appId: str("appId"), osInstanceId: val.osInstanceId as string | undefined } };
    // 📨️ Transport, never a friendly `Effect` — see `WIRE_SEND_MESSAGE_ROUTED_TARGETS`. A
    // `Shell{instance}` send-message IS an `AppFrame` reply: `runQueuedTurn` splits the ones that
    // land on the call's own turn through `shellFrameBytes`, and `invocationFromFrames` applies the
    // ones a later turn produced (a reserved tool job's completion, e.g. `interactionSelect`'s)
    // through `leftoverShellInvocationFrames` — BEFORE this mapping runs. A `Backbone{uri}` one
    // already went out the document port in `routeHostEffects`. Anything else has no host route and
    // stays loud.
    case "send-message": {
      if (isRoutedWireSendMessage(effect)) return null;
      console.warn(`wireEffectToFriendly: send-message to "${wireSendMessageTargetTag(effect) || "no endpoint"}" has no host route — only ${WIRE_SEND_MESSAGE_ROUTED_TARGETS.join("/")} are consumed`);
      return null;
    }
    default:
      return null;
  }
}
//#endregion 🔖️ActorAdapter
//#endregion 🔖️plugin-runtime
