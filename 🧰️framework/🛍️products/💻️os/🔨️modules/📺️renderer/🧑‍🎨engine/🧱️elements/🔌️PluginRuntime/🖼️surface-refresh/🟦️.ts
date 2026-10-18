/** 🖼️ Window, panel and section refresh: retained surfaces per actor, refresh request to surface-visible events, section values, refresh responses and guest presence updates.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | PluginRuntime}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  type Effect,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  SemioFaultError,
  type UiRefreshSection,
  type UiRefreshSectionKey,
  UI_REFRESH_SECTIONS,
  panelViewContext,
  parseResolvedPluginViewState,
  sectionViewContext,
  windowViewContext
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { packedTextLeaf } from "../🧳️packed-text/🟦️.ts";
import {
  decodePackValue,
  encodePackValue
} from "@semio-tech/framework/packs";
import {
  decodeAppFrame,
  decodeFaultFromWire,
  faultDisplayMessage
} from "@semio-tech/framework/app-channel";
import {
  viewContextWireValue
} from "@semio-tech/framework/packs";
import { type BuiltNode } from "@semio-tech/framework";
import { publishGuestPresenceV1, type GuestPresenceMarkV1 } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type ShardEventEnvelope } from "../../../../../../../../🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { decodeActorUiPatchReceipt } from "../../../../../../../../🔨️modules/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch/🟦️.ts";
import {
  coerceWireBytes,
  decodeWirePatchOps,
  retainedSurfaceId,
  shellFrameBytes,
  wirePatchSurfaceId,
  type WireTurnResult,
  type WireUiPatch,
  type WireVariant
} from "../🪝️wire-codec/🟦️.ts";
import {
  applyUiPatchToRetained,
  type RetainedSurface,
  retainedSurfaceHash,
  retainedSurfaceToBuiltNode,
  retainedSurfaceToSnapshot,
  wireEffectToFriendly
} from "../🪟️retained-ui-patch/🟦️.ts";

//#region 🔖️plugin-runtime
//#region 🔖️ActorAdapter
/** 🧪️ Language-neutral retained-patch oracle used by the in-source contract tests. Production
 * rendering is owned exclusively by `OwnedUiInstanceSurface` inside `loadPluginModule`. */
export const retainedWindowByActor = new Map<string, Map<string, RetainedSurface>>();

export function pluginSurfaceRef(instance: number, bodyKey: string): { readonly instance: number; readonly surface: string } {
  return { instance, surface: bodyKey };
}

/** 📌️ Full chrome refresh once leftover InteractionView carries a selection the Inspection body must re-render. */
export function leftoverInspectionRefreshScope(selectedIds: readonly string[]): { readonly kind: "full" } | null {
  return selectedIds.length > 0 ? { kind: "full" } : null;
}

/** 📌️ First leftover pick must omit the cached Inspection hash — B4's `uiRefreshSectionUnchanged` skip kept the empty summary while leftover already published `selectedIds`. */
export function leftoverInspectionPanelHash(selectedIds: readonly string[], cachedHash: string | undefined): string | undefined {
  return selectedIds.length > 0 ? undefined : cachedHash;
}

/** 🪟 Binds each authored window instance the host view actually carries to its own host view.
 *
 * 🐛️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56: this also appended a synthetic
 * `<instance>:window` binding carrying the LAST bound window's body, so that a guest dirty addressed to
 * the bare `window` name would find a host context (wave W-G3 §8.29). That alias mounted a fourteenth
 * surface no pane ever reads, republished the 180-object world body a fourth time per refresh, and —
 * because the reconcile credit ledger admits only a handful of concurrent reservations process-wide —
 * held the reservation whose refusal (`reserve_refusal=1:window:registry-reservation-unavailable`) left
 * every real surface deferred and the reactor answering `more-work` for ever (wave B54 §6.4). Every
 * guest dirty now names a real window instance, so no alias is needed and none is minted. */
export function windowHostContextBindings(
  instanceId: number,
  windows: readonly { readonly key: string; readonly bodyKey?: string }[],
  viewState: Parameters<typeof windowViewContext>[0],
): ReadonlyArray<{ readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string }> {
  const bindings: Array<{ readonly surface: { readonly instance: number; readonly surface: string }; readonly bodyKey: string; readonly windowKey: string }> = [];
  for (const target of windows) {
    if (!target.bodyKey) continue;
    if (!windowViewContext(viewState, target.key)) continue;
    bindings.push({ surface: pluginSurfaceRef(instanceId, target.key), bodyKey: target.bodyKey, windowKey: target.key });
  }
  return bindings;
}

export function uiRefreshBodyKeys(request: PluginUiRefreshRequest): string[] {
  return [...new Set([...(request.windows ?? []), ...(request.panels ?? [])].flatMap((target) => target.bodyKey ? [target.bodyKey] : []))];
}

/** 🧩️ The reserved section surfaces this request asks for, in contract order. Unlike a window or a
 * panel the shell never names their body: the section's identity IS its reserved body key, so it can
 * never collide with an app-authored one. */
export function uiRefreshSectionTargets(request: PluginUiRefreshRequest): readonly UiRefreshSection[] {
  return UI_REFRESH_SECTIONS.filter((section) => request[section.key] !== undefined);
}

/** 🪞️ Whether the host already holds this section's CURRENT tree, i.e. whether the whole projection
 * may be skipped.
 *
 * `buildUiRefreshRequest` has always stamped every requested entry with the host's own cached hash
 * precisely so "the plugin can omit payloads for sections that didn't change" — and no projector ever
 * read it: both {@link retainedUiRefreshResponse} and the owned projector rebuilt every requested tree
 * on every refresh. That cost twice:
 *
 * 1. the retained walk itself (measured on the Nakagin document switch, ticket 26/09/02 wave B4:
 *    51 projections, 1 275 nodes, 132 ms of main-thread time for ONE switch), and
 * 2. — the larger term — a FRESH `BuiltNode` object for an unchanged body, which defeats
 *    `applyUiRefreshResponseToCache`/`mergeRecordPreservingIdentity` and therefore
 *    `InterpretedUiNode`'s `React.memo`: every refresh re-reconciled the entire shell subtree of a
 *    window nothing had touched.
 *
 * A response entry carrying only `{ key, hash }` is the contract's own shape for "unchanged"
 * (`PluginUiRefreshSectionResponse.value` is optional); the shell then keeps the exact object
 * reference it already dispatched. The hash on both sides is the same surface view hash, so equality
 * is exact, never heuristic. */
export function uiRefreshSectionUnchanged(cached: string | undefined, view: { readonly root: number | null; readonly hash: string | null }): boolean {
  return cached !== undefined && cached.length > 0 && view.root !== null && view.hash === cached;
}

/** 🧩️ Rebuilds one reserved section's value from its retained surface tree: the depth-first
 * concatenation of every text leaf is exactly the canonical JSON the plugin serialized (the tree is
 * an `UI_BUILT_CHILDREN_MAX`-ary carrier of packed `UI_TEXT_MAX_BYTES` leaves (`value` + `dataAttributes`), see `section_component_tree`
 * in `🔌️plugin/🦀️.rs`). No `packValueToExactJson` is involved: the payload crosses as text, so
 * `JSON.parse` reproduces the plugin's own numbers without an integer-carrier projection.
 *
 * 🧯️ A payload that does not parse is a TYPED fault naming the reserved section and the producing
 * actor, never a bare `SyntaxError`. Reading only `value` and dropping the 32 packed
 * `dataAttributes` slices per leaf handed `JSON.parse` an exact 512-byte prefix, and the shell
 * reported it as `Unterminated string in JSON at position 512` from two unrelated-looking call sites
 * (`ShellHost`'s session-refresh effect and its typed-operation completion pass) with nothing naming
 * the carrier — see ticket 26/09/09/PROCEDURAL-3D-END-TO-END `📓️json-512-truncation-2026-09-10.md`. */
export function sectionValueFromBuiltNode(bodyKey: string, node: BuiltNode, producer: string): unknown {
  if (node.key !== bodyKey) throw new Error(`plugin-ui.section-root-mismatch:${node.key}`);
  let payload = "";
  const walk = (current: BuiltNode): void => {
    if (current.component.type === "text") {
      if (typeof current.component.value !== "string") throw new Error(`plugin-ui.section-chunk-not-text:${bodyKey}`);
      payload += packedTextLeaf(current.component.value, current.component.dataAttributes);
    }
    for (const child of current.children) walk(child);
  };
  walk(node);
  try {
    return JSON.parse(payload) as unknown;
  } catch (error) {
    throw new SemioFaultError({
      origin: "renderer",
      code: "plugin-ui.section-payload-not-json",
      severity: "error",
      message: `reserved refresh section ${bodyKey} published by ${producer} carried ${payload.length} bytes that are not valid JSON`,
      scope: { bodyKey, instanceId: producer },
      causes: [{ message: error instanceof Error ? error.message : String(error) }],
      retryable: false,
    });
  }
}

/** 🧩️ One reserved section's value from its retained document (`undefined` while it has no root) — the one decoder the
 * local refresh and an actor-bound document's retained section stores share (C13 P1). */
export function retainedSectionValueV1(bodyKey: string, surface: RetainedSurface, producer: string): unknown {
  const built = retainedSurfaceToBuiltNode(surface);
  return built === null ? undefined : sectionValueFromBuiltNode(bodyKey, built, producer);
}

/** 🪟️ The ONE host→guest view-context gate — the FULL schema admission
 * (`parseResolvedPluginViewState`, `🛂️manifest/🟦️.ts`), every field and every capacity.
 *
 * The guest's `ViewModel` (`🛂️manifest/🦀️.rs`) declares `locale`/`terminology` NON-optional, so a
 * projection that never passed through the admission decodes there as the anonymous
 * `missing field \`locale\`` app fault — an `AppFrame::Error` that surfaces as an unhandled
 * `SemioFaultError` naming neither the caller nor the field's owner
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, deferred `flowEvalTick` re-arms). Admitting here makes
 * the same drift a loud, host-side throw that names the crossing.
 *
 * 📏️ This gate was temporarily narrowed to a required-FIELD check because the shell rode its own
 * aggregated `contributionsJson` closure (248 635 characters) inside every refresh's view state,
 * against the schema's 65 536-character long-field bound. That carriage is gone: contributions are
 * installed by the paged `setContributions` run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`) and the
 * guest folds them into its own registry, so the capacity admission is answerable here again and
 * `panelJson` is the only long field a context carries. */
export function admitCrossingViewContext<T>(where: string, view: T): T {
  try {
    parseResolvedPluginViewState(view);
  } catch (error) {
    throw new Error(`view context rejected at ${where}: ${error instanceof Error ? error.message : String(error)} — resolve it through parseResolvedPluginViewState before crossing`);
  }
  return view;
}

/** 🪟️ Binds each concrete host surface to its authored body and projected ViewModel. */
export function uiRefreshSurfaceEvents(instanceId: number, request: PluginUiRefreshRequest) {
  admitCrossingViewContext("refresh-ui", request.viewState);
  const windows = windowHostContextBindings(instanceId, request.windows ?? [], request.viewState).flatMap((binding) => {
    const viewState = windowViewContext(request.viewState, binding.windowKey);
    if (!viewState) return [];
    return [{
      kind: "surface-visible",
      payload: { surface: binding.surface, bodyKey: binding.bodyKey, viewState: encodePackValue(viewContextWireValue(viewState)) },
    } satisfies ShardEventEnvelope];
  });
  const panels = (request.panels ?? []).flatMap((target) => target.bodyKey ? [{
      kind: "surface-visible",
      payload: {
        surface: pluginSurfaceRef(instanceId, target.key),
        bodyKey: target.bodyKey,
        viewState: encodePackValue(viewContextWireValue(panelViewContext(request.viewState))),
      },
    } satisfies ShardEventEnvelope] : []);
  const sections = uiRefreshSectionTargets(request).map((section) => ({
    kind: "surface-visible",
    payload: {
      surface: pluginSurfaceRef(instanceId, section.bodyKey),
      bodyKey: section.bodyKey,
      viewState: encodePackValue(viewContextWireValue(sectionViewContext(request.viewState))),
    },
  } satisfies ShardEventEnvelope));
  return [...windows, ...panels, ...sections];
}

/** 🖼️ The typed fault one MOUNTED SURFACE publishes when its own retained render terminated — the
 * guest's `PatchTracker::take_render_fault` reported through `shell_fault_effect`
 * (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`, `SURFACE_RENDER_FAULT_CODE`). Its message is
 * `"<instance>:<surface>: <fault>"`, so it names its own owner and nobody else's. */
export const SURFACE_RENDER_FAULT = "ui.surface-render";

/** 🏛️ Projects one turn's host effects without borrowing any retained UI owner.
 *
 * 🧯️ A {@link SURFACE_RENDER_FAULT} is scoped to the ONE surface it names and never becomes this
 * turn's answer. The guest emits it on whatever turn its reconciler happened to terminate on, which
 * is almost never the turn that caused it: a `DuplicateSiblingKey` in the framework ToolRun panel
 * aborted `applyHostEffects` for every unrelated `flow-extension-brep` completion, so no evaluation
 * result ever landed and the preview never converged (2026-09-14, boot console
 * `🗑️generated/react-s5/boot`). The surface's own terminal is already closed by the guest and the
 * surface re-produces on its next dirty pass, so the honest host answer is to report it and keep
 * the turn's remaining effects. Every OTHER `Error` frame IS this turn's answer and still throws. */
/** 🎞️ Shell frames a REFRESH settle produced, handed to the channel instead of dropped.
 *
 * 🐛️ Every non-`Error` shell frame used to be decoded and discarded here. A long typed operation
 * publishes its process as unsolicited `Invocation` frames (`in_reply_to: 0`, carrying the dirty
 * scope), and a refresh settle that runs DURING that operation is exactly where those frames land —
 * so a live tool run's progress reached `AppChannelClient.publishOperationProgress` only until the
 * first refresh took the actor, and then never again while the run continued (measured on release
 * :6013, 100-object fill: progress frames stop at +442 ms, the run completes at ~2.1 s, the scene
 * stays at 36–40 of 100 provisional objects 20 s later — ticket
 * 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, `🔍️w5-fill-timeline-probe.ts --settle=20000`). A frame
 * is the channel's to route, whichever settle produced it. */
export function retainedUiRefreshEffects(instanceId: number, effects: readonly WireVariant[], publishShellFrames: (frames: readonly Uint8Array[]) => void = () => {}): Effect[] {
  const requestedEffects: Effect[] = [];
  const shellFrames: Uint8Array[] = [];
  for (const effect of effects) {
    const bytes = shellFrameBytes(effect, instanceId);
    if (bytes) {
      const frame = decodeAppFrame(bytes);
      if (!("Error" in frame)) shellFrames.push(bytes);
      if ("Error" in frame) {
        const fault = decodeFaultFromWire(frame.Error.fault, decodePackValue);
        if (fault?.code === SURFACE_RENDER_FAULT) {
          console.error(`retained surface render fault, scoped to that surface: ${fault.message}`);
          continue;
        }
        if (fault) throw new SemioFaultError(fault);
        throw new Error(`refresh failed: ${faultDisplayMessage(frame.Error.fault, decodePackValue)}`);
      }
    } else {
      const requested = wireEffectToFriendly(effect);
      if (requested) requestedEffects.push(requested);
    }
  }
  if (shellFrames.length > 0) publishShellFrames(shellFrames);
  return requestedEffects;
}

/** 📬️ Projects retained bodies back to their requested window and panel keys. */
export function retainedUiRefreshResponse(instanceId: number, request: PluginUiRefreshRequest, retained: ReadonlyMap<string, RetainedSurface>, effects: readonly WireVariant[] = []): PluginUiRefreshResponse {
  const project = (targets: PluginUiRefreshRequest["windows"]): PluginUiRefreshSectionResponse[] => (targets ?? []).flatMap((target) => {
    const surface = retained.get(retainedSurfaceId(instanceId, target.key));
    if (!surface) return [];
    const hash = retainedSurfaceHash(retainedSurfaceToSnapshot(surface));
    if (uiRefreshSectionUnchanged(target.hash, { root: surface.root, hash })) return [{ key: target.key, hash }];
    const value = retainedSurfaceToBuiltNode(surface);
    return value ? [{ key: target.key, hash, value }] : [];
  });
  const sections: { [key in UiRefreshSectionKey]?: PluginUiRefreshSectionResponse } = {};
  for (const section of uiRefreshSectionTargets(request)) {
    const surface = retained.get(retainedSurfaceId(instanceId, section.bodyKey));
    if (!surface) continue;
    const hash = retainedSurfaceHash(retainedSurfaceToSnapshot(surface));
    if (uiRefreshSectionUnchanged(request[section.key]?.hash, { root: surface.root, hash })) {
      sections[section.key] = { key: section.key, hash };
      continue;
    }
    const value = retainedSectionValueV1(section.bodyKey, surface, `instance ${instanceId}`);
    if (value !== undefined) sections[section.key] = { key: section.key, hash, value };
  }
  return { windows: project(request.windows), panels: project(request.panels), ...sections, requestedEffects: retainedUiRefreshEffects(instanceId, effects) };
}

export function retainedSurfacesForActor(actorId: string): Map<string, RetainedSurface> {
  const existing = retainedWindowByActor.get(actorId);
  if (existing) return existing;
  const created = new Map<string, RetainedSurface>();
  retainedWindowByActor.set(actorId, created);
  return created;
}
//#endregion 🔖️ActorAdapter

/** 🩹️ A patch acknowledgement carries the guest's own publication receipt: the guest rejects an ack
 * whose `(lifetime, patch-sequence)` authority it never issued, so the ack is only meaningful for the
 * very turn whose `uiPatchReceipt` produced these patches. */
export function patchAckEvents(turn: WireTurnResult, uiPatches: readonly WireUiPatch[]): ShardEventEnvelope[] {
  if (uiPatches.length === 0) return [];
  const receipt = turn.uiPatchReceipt === undefined ? undefined : decodeActorUiPatchReceipt(turn.uiPatchReceipt);
  if (!receipt) return [];
  return uiPatches.flatMap((patch) => (patch.surface ? [{ kind: "patch-ack", payload: { receipt, surface: patch.surface, revision: patch.revision ?? 0 } }] : []));
}

export function applyRetainedWindowPatches(actorId: string, uiPatches: readonly WireUiPatch[]): WireUiPatch[] {
  const retained = retainedSurfacesForActor(actorId);
  const accepted: WireUiPatch[] = [];
  for (const patch of uiPatches) {
    const ops = decodeWirePatchOps(patch.ops ?? []);
    const surfaceId = wirePatchSurfaceId(patch);
    if (!surfaceId) {
      continue;
    }
    const previous = retained.get(surfaceId) ?? null;
    const { surface, desynced } = applyUiPatchToRetained(previous, { surface: surfaceId, revision: patch.revision ?? 0, baseRevision: patch.baseRevision ?? 0, ops });
    if (desynced) {
      continue;
    }
    if (surface) {
      retained.set(surfaceId, surface);
      accepted.push(patch);
    }
  }
  return accepted;
}

/** 🪟️ Captures every render-producing turn, including instance-open: a later
 * surface-visible probe may legitimately emit no patch when the guest tree is unchanged, so
 * dropping the first turn's patch would leave the shell on its loading placeholder forever. */
export function retainTurnUiPatches(actorId: string, result: Pick<WireTurnResult, "uiPatches">): WireUiPatch[] {
  return result.uiPatches.length > 0 ? applyRetainedWindowPatches(actorId, result.uiPatches) : [];
}

/** 👥️ One WIT `presence-update` decoded into the host's own mark, or `null` for a carrier this host
 * cannot read. Every field the guest omits is its serde default — `OwnPresence` skips its own `false`
 * flags on the wire, so an ABSENT flag is idle, never unknown. */
export function decodeGuestPresenceUpdateV1(decoded: unknown): { readonly surface: string; readonly nodeKey: string; readonly own: GuestPresenceMarkV1 } | null {
  if (decoded === null || typeof decoded !== "object") return null;
  const record = decoded as Record<string, unknown>;
  const surface = record.surface;
  const nodeKey = record.nodeKey;
  if (typeof surface !== "string" || typeof nodeKey !== "string" || nodeKey.length === 0) return null;
  const own = record.own !== null && typeof record.own === "object" ? (record.own as Record<string, unknown>) : {};
  const color = typeof own.color === "number" ? own.color : typeof own.color === "bigint" ? Number(own.color) : null;
  return { surface, nodeKey, own: { selected: own.selected === true, hovered: own.hovered === true, previewed: own.previewed === true, color } };
}

/**
 * 👥️ Hands one turn's `presence` array to the renderer's presence table.
 *
 * Selection and hover deliberately never ride a `UiPatch` (they change at input frequency, and a patch
 * is revisioned), so this array is the ONLY channel by which a guest-owned selection reaches a rendered
 * row. Before this call site existed the host read `turn-result.presence` in exactly one place — the
 * "did this turn carry anything" predicate — and then dropped it, so `UiPresenceOverlayContext` had no
 * production provider at all and every retained tree row rendered `aria-selected="false"` however the
 * guest's `graph` domain moved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap F1).
 */
export function retainTurnPresence(result: { readonly presence?: unknown }): void {
  if (!Array.isArray(result.presence) || result.presence.length === 0) return;
  const updates: { readonly surface: string; readonly nodeKey: string; readonly own: GuestPresenceMarkV1 }[] = [];
  for (const entry of result.presence) {
    const packed = entry !== null && typeof entry === "object" ? (entry as { readonly update?: unknown }).update : undefined;
    if (packed === undefined) continue;
    let decoded: unknown;
    try {
      decoded = decodePackValue(coerceWireBytes(packed));
    } catch {
      continue;
    }
    const update = decodeGuestPresenceUpdateV1(decoded);
    if (update) updates.push(update);
  }
  if (updates.length > 0) publishGuestPresenceV1(updates);
}
//#endregion 🔖️plugin-runtime
