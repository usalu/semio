/** 🎯️ The UI dirty scope: which window bodies, panel bodies and sections a refresh must rebuild, window layout ids and the merge of two scopes.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region InvocationResponse
/**
 * 🐢️ Mirrors the Rust `UiDirtyScope` — which rendered UI sections an action actually
 * invalidates. Absent (`undefined`) on an `InvocationResponse` means the same as the Rust side's missing
 * field: treat as `{kind: "full"}` (see {@link resolveUiDirtyScope}) — every program that doesn't emit
 * this yet keeps today's whole-shell-refresh behavior.
 */
export type UiDirtyScope =
  | { readonly kind: "full" }
  | { readonly kind: "none" }
  | {
      readonly kind: "partial";
      readonly windowBodies?: readonly string[];
      readonly panelBodies?: readonly string[];
      readonly utilities?: boolean;
      readonly tools?: boolean;
      readonly engagements?: boolean;
      readonly measures?: boolean;
      readonly labels?: boolean;
    };

/** 🔎️ Normalizes a possibly-absent `UiDirtyScope` — missing (older program, or a response built without one) means `full`. */
export function resolveUiDirtyScope(scope: UiDirtyScope | undefined): UiDirtyScope {
  return scope ?? { kind: "full" };
}

/** 🔖️ One flag-addressed section of a batched `refresh-ui`, mirrored from Rust `UiDirtySection`. */
export type UiDirtySection = "utilities" | "tools" | "engagements" | "measures" | "labels";

/**
 * ⚖️ The selection law both shells answer to — the TypeScript twin of Rust
 * `UiDirtyScope::wants_window_body` and siblings, driven by the same fixture
 * `🧫️fixtures/🐢️ui-dirty-scope/🔣️.json`.
 *
 * React has always read a scope; the wgpu shell's `refresh_ui` walked every window and panel leaf on
 * every settle and threw `InvocationResult.uiScope` away — 116 of 137 renders per converging edit
 * answered `patched=0` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
 */
export function uiDirtyScopeWantsWindowBody(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.windowBodies ?? []).includes(bodyKey));
}

/** 📑️ Twin of Rust `UiDirtyScope::wants_panel_body`. */
export function uiDirtyScopeWantsPanelBody(scope: UiDirtyScope, bodyKey: string): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && (scope.panelBodies ?? []).includes(bodyKey));
}

/** 🧩️ Twin of Rust `UiDirtyScope::wants_section`. */
export function uiDirtyScopeWantsSection(scope: UiDirtyScope, section: UiDirtySection): boolean {
  return scope.kind === "full" || (scope.kind === "partial" && scope[section] === true);
}

/** 🛍️ Twin of Rust `UiDirtyScope::wants_catalogue` — the app-static catalogue carries no flag of its own, so only a full scope asks the guest for it. */
export function uiDirtyScopeWantsCatalogue(scope: UiDirtyScope): boolean {
  return scope.kind === "full";
}

/** 🚫️ Twin of Rust `UiDirtyScope::asks_for_nothing` — no pass may be opened at all. */
export function uiDirtyScopeAsksForNothing(scope: UiDirtyScope): boolean {
  return scope.kind === "none";
}

/** 🏗️ Structural shape of a mode-layout node, read here so a caller that only asks "which windows are
 * mounted?" does not take a dependency on a renderer's whole layout type surface. */
type ModeLayoutNodeLikeV1 = { readonly kind: string; readonly id?: string; readonly children?: readonly ModeLayoutNodeLikeV1[] };

/** 🪟️ Every window id the mode layout actually mounts, background tabs of a stack included.
 *
 * 🐢️ This is the difference between the windows an app DECLARES and the windows the user is looking
 * at. Asking the guest to re-render all of the declared ones on every `refresh-ui` made a converging
 * generation3d edit re-render the three generate-mode windows — the generate preview's mesh payload
 * included — twice per `flowEvalTick` hop while the user sat in edit mode
 * (`📓️react-hop-latency-2026-09-14.md`). It lives beside the dirty-scope predicates because both
 * shells owe the same rule: the wgpu `refresh_ui` window walk has the identical choice to make. */
export function windowLayoutWindowIdsV1(layout: unknown): ReadonlySet<string> {
  const ids = new Set<string>();
  const walk = (node: ModeLayoutNodeLikeV1 | null | undefined): void => {
    if (!node || typeof node !== "object") return;
    if (node.kind === "window" && typeof node.id === "string") {
      ids.add(node.id);
      return;
    }
    for (const child of node.children ?? []) walk(child);
  };
  walk(layout as ModeLayoutNodeLikeV1 | null | undefined);
  return ids;
}

/** ✂️ Splits the declared window instances into the ones a refresh pass must fetch and the ones
 * the layout does not mount. A skipped window's CACHED body must be dropped by the caller, and that is
 * what makes the skip safe: nothing can later serve a stale body, and the pass that fetches the window
 * once it IS mounted asks with no hash and gets a whole one back. An empty `mounted` set means the
 * layout is not known yet (first pass, session switch) — then everything is fetched. */
export function partitionRefreshWindowInstancesV1<T extends { readonly id: string }>(
  windowInstances: readonly T[],
  mounted: ReadonlySet<string>,
): { readonly fetched: readonly T[]; readonly skipped: readonly T[] } {
  if (mounted.size === 0) return { fetched: windowInstances, skipped: [] };
  const fetched: T[] = [];
  const skipped: T[] = [];
  for (const instance of windowInstances) (mounted.has(instance.id) ? fetched : skipped).push(instance);
  return { fetched, skipped };
}

/** 🤝️ Twin of Rust `UiDirtyScope::merged_with` — the union one coalesced pass owes, in first-seen body-key order. */
export function mergeUiDirtyScopes(first: UiDirtyScope, second: UiDirtyScope): UiDirtyScope {
  if (first.kind === "full" || second.kind === "full") return { kind: "full" };
  if (first.kind === "none") return second;
  if (second.kind === "none") return first;
  return {
    kind: "partial",
    windowBodies: [...new Set([...(first.windowBodies ?? []), ...(second.windowBodies ?? [])])],
    panelBodies: [...new Set([...(first.panelBodies ?? []), ...(second.panelBodies ?? [])])],
    utilities: Boolean(first.utilities || second.utilities),
    tools: Boolean(first.tools || second.tools),
    engagements: Boolean(first.engagements || second.engagements),
    measures: Boolean(first.measures || second.measures),
    labels: Boolean(first.labels || second.labels),
  };
}
//#endregion InvocationResponse
// #endregion 🎠️Kernel
