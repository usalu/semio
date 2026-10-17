/** 🔄️ JSON identity helpers and the plugin UI-refresh cache, dirty-scope algebra, coalescer and request builders.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

// #region 🔌️Adapters
import {
  FRAMEWORK_HISTORY_BODY_KEY,
  type PanelTabKind,
  panelTabKindId,
  type PluginUiRefreshRequest,
  type PluginUiRefreshResponse,
  type PluginUiRefreshSectionResponse,
  type PluginViewState,
  resolveUiDirtyScope,
  type UiDirtyScope
} from "@semio-tech/framework";
// #endregion 🔌️Adapters
import { elementIdSegment } from "@semio-tech/ui-react";
import {
  mergeUiDirtyScopes,
  uiDirtyScopeWantsCatalogue,
  uiDirtyScopeWantsPanelBody,
  uiDirtyScopeWantsSection,
  uiDirtyScopeWantsWindowBody,
  type UiDirtySection
} from "@semio-tech/framework";
import { type ExtraWindowInstance } from "../../🐚️Shell/🗃️state-shapes/🟦️.ts";

//#region ShellHelpers
/** 🐢️ Structural equality over plain JSON-shaped values (the shape every `UiNode`/`WindowEngagement`/`WindowMeasure` program payload takes) — no cycles, no non-JSON types. */
function uiJsonDeepEqual(a: unknown, b: unknown): boolean {
  if (a === b) return true;
  if (typeof a !== "object" || typeof b !== "object" || a === null || b === null) return false;
  if (Array.isArray(a) !== Array.isArray(b)) return false;
  if (Array.isArray(a) && Array.isArray(b)) {
    if (a.length !== b.length) return false;
    for (let index = 0; index < a.length; index += 1) {
      if (!uiJsonDeepEqual(a[index], b[index])) return false;
    }
    return true;
  }
  const aRecord = a as Record<string, unknown>;
  const bRecord = b as Record<string, unknown>;
  const aKeys = Object.keys(aRecord);
  const bKeys = Object.keys(bRecord);
  if (aKeys.length !== bKeys.length) return false;
  for (const key of aKeys) {
    if (!Object.prototype.hasOwnProperty.call(bRecord, key)) return false;
    if (!uiJsonDeepEqual(aRecord[key], bRecord[key])) return false;
  }
  return true;
}

/**
 * 🐢️ Reuses `previous`'s object identity when it's structurally equal to `next` — every program
 * `render()`/`utilities()`/`windowEngagements()`/`windowMeasures()` call re-parses a fresh JSON payload
 * every time, even when nothing about that body actually changed (e.g. a camera-only or selection-only
 * action still returns byte-identical panel/utility JSON). Without this, every downstream `React.memo`
 * (see `InterpretedUiNode`) sees a new prop reference every render and can never bail.
 */
export function preserveJsonIdentity<T>(previous: T | undefined, next: T): T {
  return previous !== undefined && uiJsonDeepEqual(previous, next) ? previous : next;
}

/**
 * 🐢️ Builds a `Record<string, V>` from `entries`, reusing `prev`'s per-key value reference where
 * `preserveJsonIdentity` finds no structural change, and reusing `prev` itself (the whole record) when
 * no key actually changed — so a no-operation action's `dispatch` doesn't hand `windowUiByWindowId`/etc. a new
 * object reference and cascade an unmemoizable re-render through every downstream consumer.
 */
export function mergeRecordPreservingIdentity<V>(prev: Readonly<Record<string, V>>, entries: readonly (readonly [string, V])[]): Readonly<Record<string, V>> {
  const next: Record<string, V> = {};
  let changed = Object.keys(prev).length !== entries.length;
  for (const [key, value] of entries) {
    const preserved = preserveJsonIdentity(prev[key], value);
    next[key] = preserved;
    if (preserved !== prev[key]) changed = true;
  }
  return changed ? next : prev;
}

//#region UiRefresh
/** 🐢️ One cached section value keyed by `${section}:${key}` (e.g. `window:2d-overview`, `engagements`) — the hash is what gets sent back to the plugin next time so it can skip re-serializing unchanged content. */
export type UiRefreshCache = Map<string, { readonly hash: string; readonly value: unknown }>;

/** 🖼️ How much of the shell one direct browser-actor dispatch dirtied.
 *
 * The actor handoff answers with `{outcome, mutationCount}` and carries NO `UiDirtyScope` — the guest's
 * own scope never crosses it — and a framework-reserved verb that commits inline (`paste`, `undo`, a
 * gumball commit) publishes no `OperationCompleted` frame either, so this route is the one dispatch path
 * with no scope of its own at all. An applied mutation therefore dirties everything: the honest answer
 * when the only thing known is THAT the document changed. `refreshUi` is hash-conditional, so an
 * unchanged section still costs no payload; what this buys is the changed window body being re-taken in
 * the same turn instead of whenever some later, unrelated action happens to refresh (measured at +12 s
 * for `paste` — ticket 26/09/02/PUZZLE-3D-END-TO-END wave B6 §3, fixed in wave B9 lane 4).
 */
export function browserActorDispatchUiScopeV1(result: { readonly outcome: "guest-applied" | "rejected"; readonly mutationCount: number }): UiDirtyScope {
  return result.outcome === "guest-applied" && result.mutationCount > 0 ? { kind: "full" } : { kind: "none" };
}

/** 📏 Window-option verbs publish only the WindowConfig lane, so the actor handoff reports
 * `mutationCount: 0`. Without this list the rail never re-takes measures after a successful toggle. */
export const WINDOW_CONFIG_RAIL_ACTION_IDS: ReadonlySet<string> = new Set([
  "focusSelection",
  "setCamera",
  "setChunkSize",
  "setGridSnapEnabled",
  "setGridSpacing",
  "setGridVisible",
  "setLodAutomatic",
  "setLodDepthVariable",
  "setLodManual",
  "setProjection",
  "setProjectionParam",
  "setProximityRadius",
  "setSelectableKind",
  "setSunAzimuth",
  "setSunElevation",
  "setSunIntensity",
  "setTransformGumballFlag",
  "setVortexDirection",
  "setVortexShow",
  "setVoxelDims",
  "toggleSun",
]);

export function browserActorWindowConfigDispatchUiScopeV1(
  result: { readonly outcome: "guest-applied" | "rejected"; readonly mutationCount: number },
  actionId: string | undefined,
): UiDirtyScope {
  const dirty = browserActorDispatchUiScopeV1(result);
  if (dirty.kind !== "none") return dirty;
  if (result.outcome === "guest-applied" && actionId !== undefined && WINDOW_CONFIG_RAIL_ACTION_IDS.has(actionId)) return { kind: "full" };
  return dirty;
}

/** 🏁️ What ONE typed-operation completion owes the shell — `null` for "owes no pass at all".
 *
 * A typed operation's admitting reply is only an ADMISSION: its edit is staged, published and logged on
 * later continuations, so that reply reports `mutationCount: 0` and carries no scope worth reading. The
 * completion frame (`AppFrame::OperationCompleted`) is the only carrier of the mutation's own
 * `UiDirtyScope` and, since wave B21, of its history patch — so the refresh has to follow the COMPLETION,
 * and it must be the completion's own scope rather than a guess keyed on a verb name (compare
 * {@link WINDOW_CONFIG_RAIL_ACTION_IDS}, which only exists because the browser-actor route publishes no
 * completion at all).
 *
 * The other half of the rule is what a completion does NOT owe. Every retained operation completes,
 * including the View-kind turns a drain poll finishes while the document is untouched: measured on the
 * live puzzle 3d shell at **18 completions in an idle 10 s window, 12 of them `{kind:"none"}`**, each
 * running a host-effect pass and a `refreshUi` call for nothing. A completion whose scope is `none`,
 * which carries no history patch and requests no effect is bookkeeping, and gets no pass.
 *
 * A patch with an empty scope is the one case that still owes a refresh: the row it mints belongs to the
 * reserved history body ({@link FRAMEWORK_HISTORY_BODY_KEY}), and nothing else on the shell moved.
 */
export function typedOperationCompletionRefreshV1(completion: {
  readonly uiScope: UiDirtyScope | undefined;
  readonly historyPatch: unknown;
  readonly requestedEffects: readonly unknown[];
}): UiDirtyScope | null {
  const scope = resolveUiDirtyScope(completion.uiScope);
  if (scope.kind !== "none") return scope;
  if (completion.historyPatch !== undefined) return { kind: "partial", panelBodies: [FRAMEWORK_HISTORY_BODY_KEY] };
  return completion.requestedEffects.length > 0 ? scope : null;
}

/** 🧰️ Host effects that rewrite host-owned state the GUEST reads back on its next render. The host's
 * per-window utility map and active tool ride into every `refresh-ui` as `ViewModel
 * .activeUtilityByWindowId`/`activeToolId`, and `setPanel` rewrites `panelJson` — so applying one of
 * these changes what every subsequent render MUST produce, and the pass that applies it owes that render.
 *
 * Everything else an effect pass can do is host chrome or re-enters a route that owns its own scope: a
 * `notify` banner is host React state, `clipboardWrite`/`downloadMediaExport` never reach a projection,
 * `dispatchAction`/`replayShellCommand` come back through shell dispatch with their own `UiDirtyScope`,
 * and `navigate`/`spawnPluginInstance`/`openPluginInstance` switch sessions (a session switch forces a
 * full fetch of its own). Those earn NOTHING here — that is what keeps {@link
 * typedOperationCompletionRefreshV1}'s no-storm property intact. */
const HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS = ["setActiveUtility", "setActiveTool", "setPanel", "loadDocument"] as const;

/**
 * 🧰️ The scope a host-effect pass owes ON TOP of what its dispatch or completion declared.
 *
 * The measured defect (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37): the typed `brush` engagement verb
 * arms `scene.active_utility`, its epilogue emits `Effect::SetActiveUtility`, and the operation completes
 * with no mutation, no history patch and `UiDirtyScope::None` — so {@link
 * typedOperationCompletionRefreshV1} answers `none` (correctly: the GUEST re-took nothing) and
 * `applyHostEffects` handed that `none` straight to `refreshUi`, which asks for nothing at all. Live
 * console, `engagementSubmit` at `effects:1`: the host armed `puzzle3d-main-perspective → brush` and the
 * NEXT `refresh-ui` crossing happened 30 s later on an unrelated keystroke, with both world panes
 * publishing `select` for the whole interval.
 *
 * The scope a completion declares is what the GUEST dirtied. What the HOST dirtied by applying the
 * completion's own effects is this — and the two are unioned, never substituted.
 */
/** 🖼️ Whether this effect pass rewrote a guest render input at all — the same question {@link
 * hostEffectRefreshScopeV1} answers with a scope, asked where a boolean is what is needed: the follow-up
 * must ALSO land in the live built-node stores the mounted surfaces read (`ShellHost`'s
 * `forceReloadLiveUiStoresV1`, gated on `replaceBodies`). Without that the armed body is fetched, cached
 * and dispatched while the mounted world pane keeps publishing the previous `data-interaction-json`, which
 * is the same red one hop further down. */
export function hostEffectsRewriteGuestRenderInputsV1(effects: readonly unknown[]): boolean {
  return effects.some((effect) => typeof effect === "object" && effect !== null && HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS.some((key) => key in effect));
}

export function hostEffectRefreshScopeV1(effects: readonly unknown[], declared: UiDirtyScope, windowBodyKeys: readonly string[]): UiDirtyScope {
  if (!hostEffectsRewriteGuestRenderInputsV1(effects)) return declared;
  const rewriting = effects.filter((effect) => typeof effect === "object" && effect !== null && HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS.some((key) => key in effect));
  // 🪟️ `setPanel`/`loadDocument` name no body of their own — `panelJson` feeds every section, so the only
  // honest answer is the widest one, which a hash-conditional fetch makes cheap for the unchanged parts.
  const widest = rewriting.some((effect) => "setPanel" in (effect as object) || "loadDocument" in (effect as object));
  const earned: UiDirtyScope = widest
    ? { kind: "full" }
    : { kind: "partial", windowBodies: [...windowBodyKeys], panelBodies: [], utilities: true, tools: true, engagements: false, measures: true, labels: false };
  return mergeUiDirtyScopeV1(declared, earned);
}

/** 🤝️ The scope one refresh pass must cover when a SECOND pass was asked for while the first was
 * still crossing into the guest — the union, never the newer alone.
 *
 * `ShellHost.refreshUi` used to abandon a superseded pass outright: it bumps a generation on every
 * call and drops its own response when the generation moved under an await. That is correct only
 * while a pass is faster than the cadence that triggers passes. It is not: a converging preview's
 * `flowEvalTick` completions each demand a full pass, and one guest crossing during a brep solve was
 * measured at 15-16 s on the served procedural editor while completions arrived every few seconds —
 * so EVERY response was superseded before it could be applied and the flow window kept the previous
 * example's graph for as long as the evaluation ran, with no fault anywhere
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Coalescing onto this union is what turns that livelock
 * into one in-flight pass plus at most one owed follow-up, exactly the way the guest's own
 * `FlowEvalSession` latch admits at most one pending tick per window.
 */
export function mergeUiDirtyScopeV1(first: UiDirtyScope, second: UiDirtyScope): UiDirtyScope {
  return mergeUiDirtyScopes(first, second);
}

/** 🐢️ Whether `inner` asks for nothing `outer` does not already ask for — the ordering that makes
 * "this pass already covers that request" decidable. `none` is covered by anything, `full` covers
 * everything, and two partials compare field by field: every window body, every panel body and every
 * section flag `inner` names must be named by `outer` too.
 *
 * Deliberately conservative in the one direction that matters: an UNKNOWN field, or a `full` inner
 * against a `partial` outer, answers false. Answering true wrongly under-refreshes the shell, which
 * is a stale pane with no fault anywhere; answering false wrongly costs one extra pass. */
export function uiDirtyScopeCoveredByV1(inner: UiDirtyScope, outer: UiDirtyScope): boolean {
  if (inner.kind === "none") return true;
  if (outer.kind === "full") return true;
  if (inner.kind === "full" || outer.kind !== "partial" || inner.kind !== "partial") return false;
  const bodies = (values: readonly string[] | undefined) => values ?? [];
  const outerWindows = new Set(bodies(outer.windowBodies));
  const outerPanels = new Set(bodies(outer.panelBodies));
  if (!bodies(inner.windowBodies).every((body) => outerWindows.has(body))) return false;
  if (!bodies(inner.panelBodies).every((body) => outerPanels.has(body))) return false;
  return UI_DIRTY_SECTIONS_V1.every((section) => inner[section] !== true || outer[section] === true);
}

/** 🔖️ The flag-addressed sections a partial scope can name, read once so a new section cannot be
 * added to the scope type and silently escape {@link uiDirtyScopeCoveredByV1}. */
const UI_DIRTY_SECTIONS_V1 = ["utilities", "tools", "engagements", "measures", "labels"] as const satisfies readonly UiDirtySection[];

/** 🚪️ Whether the pass currently in flight has ALREADY answered `next`, so the lane may join it
 * instead of owing a second guest turn.
 *
 * The measured redundancy (`📓️react-hop-latency-2026-09-14.md` §3.2/§4): one `flowEvalTick` hop paid
 * THREE serialized guest turns — one dispatch and two full refresh passes at ~420 ms each — because a
 * converging preview arms one chain per flow window and each chain's completion demands a pass of its
 * own. The second request arrives a few milliseconds after the first pass has already submitted its
 * guest turn, and both completions are host-side callbacks of turns that had already settled: nothing
 * crossed into the guest in between, so the pass in flight is re-rendering exactly the state the
 * second request wants re-rendered.
 *
 * The four conditions are each a way the join could be WRONG, and every one of them is a refusal:
 *
 * 1. `ingressAtSubmit === ingressNow` — no dispatch and no extension-completion publication has
 *    crossed into this instance since the running pass submitted its own turn
 *    (`PluginRuntime.guestIngressGenerationV1`, which a refresh turn deliberately does not bump).
 * 2. `!next.hostInputs` — a request that carries a HOST-owned render input the guest has not seen
 *    (view state, an armed utility, a locale switch, a newly mounted window) is never covered by a
 *    pass that crossed before it, no matter what the guest did.
 * 3. the scope must be covered ({@link uiDirtyScopeCoveredByV1}) and the running pass must already be
 *    replacing bodies if this one asks to.
 * 4. same instance — two sessions never answer for each other.
 */
export function uiRefreshAlreadyAnsweredV1(
  running: { readonly instanceId: number; readonly scope: UiDirtyScope; readonly replaceBodies: boolean; readonly ingressAtSubmit: number | null },
  next: { readonly instanceId: number; readonly scope: UiDirtyScope; readonly replaceBodies: boolean; readonly hostInputs: boolean },
  ingressNow: number,
): boolean {
  if (running.ingressAtSubmit === null) return false;
  if (running.instanceId !== next.instanceId) return false;
  if (next.hostInputs) return false;
  if (next.replaceBodies && !running.replaceBodies) return false;
  if (running.ingressAtSubmit !== ingressNow) return false;
  return uiDirtyScopeCoveredByV1(next.scope, running.scope);
}

/** 🤝️ One ui-refresh lane: at most ONE pass running, at most ONE owed follow-up carrying the union of
 * everything asked for while it ran, and a follow-up that is NEVER lost — whatever the pass did.
 *
 * The three properties, and the live defect each one answers:
 *
 * 1. **Every request is owed until a pass covers it.** The starter is not a special case: it registers
 *    into the same owed slot every joiner does, and the drain loop takes it from there. A caller's
 *    promise settles when the pass that covered ITS request settled, so `await refreshUi(…)` means "my
 *    scope has been re-taken", not "somebody else's pass finished".
 * 2. **A failed pass never ends the lane.** The rejection reaches exactly the requests that pass
 *    covered (the boot refresh turns one into the session's fault card), and the loop still runs
 *    whatever was owed behind it. The predecessor `while` loop read its owed slot AFTER `await pass`,
 *    so one rejected pass dropped the follow-up on the floor — invisibly, since the owed entry is then
 *    cleared by the next unrelated starter.
 * 3. **A pass may ASK for another pass; it may never WAIT for one.** A host effect applied from inside
 *    a pass (`ShellHost`'s `pendingRefreshEffects`) re-enters this lane, and its request is served by
 *    the NEXT iteration — so the pass itself must never await that re-entrant work, or it is waiting on
 *    itself. The predecessor made the re-entrant caller `await` the very promise the pass resolves: the
 *    lane wedged with its in-flight marker pinned forever and NOTHING repainted again, which is the
 *    shape measured on the served puzzle 3d editor as an armed `SetActiveUtility` whose world pane still
 *    published `select` 30 s later (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B37). `ShellHost` keeps
 *    its half by applying a pass's own owed effects OUTSIDE the pass.
 *
 * `merge` is injected ({@link mergeUiDirtyScopeV1} plus whatever else the caller's request record
 * carries) so this lane owns sequencing only and knows nothing about sessions or window instances.
 */
export interface UiRefreshCoalescerV1<TRequest> {
  /** 🤝️ Asks for a pass covering `next`; resolves once a pass that covered it has settled. A `none` scope asks for nothing and resolves at once. */
  readonly request: (next: TRequest) => Promise<void>;
  /** 🩺️ Whether the drain loop owns the lane right now. */
  readonly busy: () => boolean;
  /** 🩺️ The scope owed to the next iteration, `null` when nothing is. */
  readonly owedScope: () => UiDirtyScope | null;
  /** 🩺️ Passes this lane has run, ever — the no-storm counter every coalescing law measures. */
  readonly passes: () => number;
  /** 🩺️ Requests joined onto a pass already in flight because it had provably already answered them. */
  readonly answered: () => number;
}

export function createUiRefreshCoalescerV1<TRequest extends { readonly scope: UiDirtyScope }>(
  run: (request: TRequest) => Promise<void>,
  merge: (owed: TRequest, next: TRequest) => TRequest,
  onDecision?: (decision: "owed" | "merged" | "pass" | "failed" | "answered", scope: UiDirtyScope, passes: number) => void,
  alreadyAnswered?: (running: TRequest, next: TRequest) => boolean,
): UiRefreshCoalescerV1<TRequest> {
  let owed: TRequest | null = null;
  let waiters: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  let running: TRequest | null = null;
  let runningWaiters: { resolve: () => void; reject: (error: unknown) => void }[] = [];
  let draining = false;
  let passes = 0;
  let answered = 0;
  const drain = async (): Promise<void> => {
    while (owed) {
      const pending = owed;
      owed = null;
      running = pending;
      runningWaiters = waiters;
      waiters = [];
      passes += 1;
      onDecision?.("pass", pending.scope, passes);
      let failure: { error: unknown } | null = null;
      try {
        await run(pending);
      } catch (error) {
        failure = { error };
        onDecision?.("failed", pending.scope, passes);
      }
      // 🤝️ Read AFTER the pass: a request the gate joined while this pass ran settles with it, and
      // both the success and the failure path must reach the joiners too — a joiner was told this
      // pass answers for it, so it inherits this pass's verdict, whichever it is.
      const covered = runningWaiters;
      running = null;
      runningWaiters = [];
      for (const waiter of covered) (failure ? waiter.reject(failure.error) : waiter.resolve());
    }
  };
  return {
    request: (next: TRequest): Promise<void> => {
      if (next.scope.kind === "none") return Promise.resolve();
      // 🚪️ The pass in flight may have already answered this request — see
      // {@link uiRefreshAlreadyAnsweredV1} for the four conditions and what each one refuses. Joining
      // it is what turns a converging hop's three serialized guest turns into two.
      if (running && alreadyAnswered?.(running, next)) {
        answered += 1;
        onDecision?.("answered", next.scope, passes);
        return new Promise<void>((resolve, reject) => runningWaiters.push({ resolve, reject }));
      }
      onDecision?.(owed ? "merged" : "owed", next.scope, passes);
      owed = owed ? merge(owed, next) : next;
      const joined = new Promise<void>((resolve, reject) => waiters.push({ resolve, reject }));
      // 🔁️ The flag is raised BEFORE the loop is entered, never after: `run`'s own synchronous prologue is
      // allowed to re-enter `request` (that is property 3), and a marker written after the call would let
      // that re-entrant request start a SECOND drain — two passes crossing into the same guest at once.
      if (!draining) {
        draining = true;
        void drain().finally(() => {
          draining = false;
        });
      }
      return joined;
    },
    busy: () => draining,
    owedScope: () => owed?.scope ?? null,
    passes: () => passes,
    answered: () => answered,
  };
}

/** 🐢️ The four selection predicates are the KERNEL's (`🎠️kernel`), whose Rust twin the wgpu
 * shell's `refresh_ui` reads from the same fixture — one implementation per language, never one per
 * renderer, so the two shells cannot drift into different ideas of what a settle dirtied. */
const uiRefreshWantsWindow = uiDirtyScopeWantsWindowBody;
const uiRefreshWantsPanel = uiDirtyScopeWantsPanelBody;
const uiRefreshWantsFlag = uiDirtyScopeWantsSection;
const uiRefreshWantsCatalogue = uiDirtyScopeWantsCatalogue;

/**
 * 🪟️ Every live window instance for a session — one per base `AppDefinition.windowKinds` entry (id ==
 * kind id) plus one per split/spawned extra — so `refreshUi` fetches and the plugin returns state for
 * every actual window, never collapsing two same-kind instances (e.g. split top/perspective panes) onto
 * one shared entry.
 */
export function sessionWindowInstances(
  app: { readonly windowKinds: readonly { readonly id: string; readonly bodyKey: string }[] },
  extraWindowInstances: readonly Pick<ExtraWindowInstance, "id" | "windowKindId">[],
): readonly { readonly id: string; readonly bodyKey: string; readonly windowKindId: string }[] {
  const kindById = new Map(app.windowKinds.map((kind) => [kind.id, kind] as const));
  const base = app.windowKinds.map((kind) => ({ id: kind.id, bodyKey: kind.bodyKey, windowKindId: kind.id }));
  const extra = extraWindowInstances.flatMap((instance) => {
    const kind = kindById.get(instance.windowKindId);
    return kind ? [{ id: instance.id, bodyKey: kind.bodyKey, windowKindId: instance.windowKindId }] : [];
  });
  return [...base, ...extra];
}

/** 🎓️ Kind-level introduction targets must also match live window *instances* of that kind
 * (`puzzle3d-main-top` / `puzzle3d-main-perspective` for kind `puzzle3d-main`) — otherwise force-unfold
 * of the utility bar / Actions rail never reaches the panes the user actually sees. `targetKindId` is a
 * raw window-kind id; `targetSegment` is an already-normalized `elementIdSegment` (e.g. from a
 * `framework.window.{segment}.action.*` introduce id). */
export function introductionTargetsWindow(
  windowId: string,
  windowKindId: string,
  targetKindId: string | null,
  targetSegment: string | null = null,
): boolean {
  if (targetKindId && (elementIdSegment(windowId) === elementIdSegment(targetKindId) || elementIdSegment(windowKindId) === elementIdSegment(targetKindId))) return true;
  if (targetSegment && (elementIdSegment(windowId) === targetSegment || elementIdSegment(windowKindId) === targetSegment)) return true;
  return false;
}

/** 🧰️ Projects resolved window arms, preserving an explicit clear as null. */
export function buildActiveUtilityByWindowId(activeUtilityByWindowId: Readonly<Record<string, string | null>>): Record<string, string | null> {
  return { ...activeUtilityByWindowId };
}

/**
 * 🐢️ Builds one batched `refresh-ui` request restricted to `scope` — `null` when the scope
 * resolves to nothing worth fetching (`none`, or a `partial` whose fields all miss this app's actual
 * bodies/instances). Every requested entry carries the host's cached hash so the plugin can omit payloads
 * for sections that didn't change. `windowInstances` is keyed by window INSTANCE id (base windows plus any
 * split/spawned extras) — never by window kind — so two instances of the same kind get independent
 * cache entries and independent rendered bodies.
 */
export function buildUiRefreshRequest(
  scope: UiDirtyScope,
  windowInstances: readonly { readonly id: string; readonly bodyKey: string }[],
  panelTabLeaves: readonly { readonly kind: PanelTabKind; readonly bodyKey?: string }[],
  viewState: PluginViewState,
  cache: UiRefreshCache,
): PluginUiRefreshRequest | null {
  if (scope.kind === "none") return null;
  const windows = windowInstances.filter((instance) => uiRefreshWantsWindow(scope, instance.bodyKey)).map((instance) => ({ key: instance.id, bodyKey: instance.bodyKey, hash: cache.get(`window:${instance.id}`)?.hash }));
  const panels = panelTabLeaves
    .filter((tab): tab is { readonly kind: PanelTabKind; readonly bodyKey: string } => Boolean(tab.bodyKey) && uiRefreshWantsPanel(scope, tab.bodyKey!))
    .map((tab) => ({ key: panelTabKindId(tab.kind), bodyKey: tab.bodyKey, hash: cache.get(`panel:${panelTabKindId(tab.kind)}`)?.hash }));
  const engagements = uiRefreshWantsFlag(scope, "engagements") ? { hash: cache.get("engagements")?.hash } : undefined;
  const measures = uiRefreshWantsFlag(scope, "measures") ? { hash: cache.get("measures")?.hash } : undefined;
  const tools = uiRefreshWantsFlag(scope, "tools") ? { hash: cache.get("tools")?.hash } : undefined;
  const labels = uiRefreshWantsFlag(scope, "labels") ? { hash: cache.get("labels")?.hash } : undefined;
  const catalogue = uiRefreshWantsCatalogue(scope) ? { hash: cache.get("catalogue")?.hash } : undefined;
  if (windows.length === 0 && panels.length === 0 && !engagements && !measures && !tools && !labels && !catalogue) return null;
  return { viewState, windows, panels, engagements, measures, tools, catalogue, labels };
}

/** 🐢️ Writes every changed section (`value !== undefined`) from a `refresh-ui` response into `cache`; unchanged sections are left as-is since the cached value is still current. */
function applyUiRefreshSectionsToCache(cache: UiRefreshCache, prefix: string, entries: readonly PluginUiRefreshSectionResponse[] | undefined): void {
  for (const entry of entries ?? []) {
    if (entry.value !== undefined) cache.set(`${prefix}:${entry.key}`, { hash: entry.hash, value: entry.value });
  }
}

export function applyUiRefreshResponseToCache(cache: UiRefreshCache, response: PluginUiRefreshResponse): void {
  applyUiRefreshSectionsToCache(cache, "window", response.windows);
  applyUiRefreshSectionsToCache(cache, "panel", response.panels);
  if (response.engagements?.value !== undefined) cache.set("engagements", { hash: response.engagements.hash, value: response.engagements.value });
  if (response.measures?.value !== undefined) cache.set("measures", { hash: response.measures.hash, value: response.measures.value });
  if (response.tools?.value !== undefined) {
    cache.set("tools", { hash: response.tools.hash, value: response.tools.value });
    const tools = response.tools.value as Record<string, unknown> | undefined;
  }
  if (response.catalogue?.value !== undefined) cache.set("catalogue", { hash: response.catalogue.hash, value: response.catalogue.value });
  if (response.labels?.value !== undefined) cache.set("labels", { hash: response.labels.hash, value: response.labels.value });
}
//#endregion UiRefresh
//#endregion ShellHelpers
