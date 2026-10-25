/** 👁️ The history and host event ids, the plugin view state, the public invocation and view context capacities and the view context projections. */
// #region 🛂️Manifest
//#region 🔌️PluginAndAppContract
//#region PluginRuntime
/** 🕹️ Mirrors `semio_framework_core::history_action_definitions` — the six framework-owned
 * History actions every editor receives, used by the shell to render the same set without a wasm round trip. A viewer
 * receives none of them: `switchAlternative` and `checkoutCheckpoint` commit a shared `Checkout` transition, and its
 * manifest never declares a verb its guard rejects. */
export const HISTORY_ACTION_IDS = ["undo", "redo", "commitCheckpoint", "createAlternative", "switchAlternative", "checkoutCheckpoint"] as const;
/** 🔛️ `switchAlternative`'s alternative id argument — mirrors Rust `SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID`. */
export const SWITCH_ALTERNATIVE_ARG_ALTERNATIVE_ID = "alternativeId";

/** 📨️ The host-forwarded window fact `hostEvent{windowId, kind}` — mirrors Rust `HOST_EVENT_ACTION_ID`: a pane's focus loss,
 * lost pointer capture or closing reaches its program, which ends an open gesture there without a trace. */
export const HOST_EVENT_ACTION_ID = "hostEvent";
/** 🏷️ The kinds a host forwards — mirror Rust `HOST_EVENT_KIND_BLUR`, `HOST_EVENT_KIND_CAPTURE_LOST`, `HOST_EVENT_KIND_RETIRING`. */
export const HOST_EVENT_KINDS = ["blur", "captureLost", "retiring"] as const;
export type HostEventKind = (typeof HOST_EVENT_KINDS)[number];

export type PluginViewState = {
  readonly activeModeId?: string;
  readonly activeWindowKindId?: string;
  /** 🧰️ Per-call overlay: host-owned active utility for the window targeted by this render/action (`windowId`). */
  readonly activeUtilityId?: string;
  /** 🖥️ Host-owned active utility per window instance (never a document field, never a VCS operation). */
  readonly activeUtilityByWindowId?: Readonly<Record<string, string | null>>;
  /** 🛠️ Host-owned active tool of the active mode (never a document field, never a VCS operation) — mutually
   * exclusive with `activeUtilityId`: activating one clears the other. */
  readonly activeToolId?: string;
  /** 📌️ Host-owned panel state, opaque to the guest. Registry
   * contributions are not a view-state field: they are installed into the guest by the paged
   * `setContributions` command run (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`), which the guest folds
   * into its own registry, so a refresh crosses a reference-free, bounded context. */
  readonly panelJson?: string;
  /** 🧩️ Parent-owned input for one embedded surface; never persisted in the contributor document. */
  readonly extensionInputJson?: string;
  /** 🪪️ Current authenticated OS session identity for this call. Plugins consume this
   * ephemeral value directly and never persist an app/document copy. */
  readonly sessionIdentity?: Readonly<{ readonly userId: string; readonly displayName: string }>;
  readonly locale?: string;
  readonly terminology?: string;
  /** 🪟️ The window instance a render/action call targets — programs key per-window option state off this, never off `activeWindowKindId`. */
  readonly windowId?: string;
  /** 🎯️ The window instance the user is LOOKING at — the shell's own last-focused pane, sent on every
   * call and deliberately surviving {@link panelViewContext} while it names a live instance. It is never the render target
   * (`windowId` is); it is what lets an app-level panel that authors per-window settings address the
   * pane the user last touched instead of the roster's first entry. */
  readonly focusedWindowId?: string;
  /** 🔓️ The live set of open window instances (base + spawned/split), so `windowMeasures`/`windowEngagements` can return one entry per instance. */
  readonly windowInstances?: readonly { readonly id: string; readonly windowKindId: string }[];
  /** ⏯️ The tool run trace cursor each window instance's renderer echoes, keyed by window instance id: the
   * next trace page it expects of `run` at `generation`. The guest answers the pages after it inside the
   * scene's `toolRunTrace` lane (`📋️tool-run-contract.md` §3.2). */
  readonly toolRunTraceCursorByWindowId?: Readonly<Record<string, ViewToolRunTraceCursor>>;
  /** 🌳️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
   * source of truth for which containers are open and which rows are on screen. A guest materialises
   * exactly the named windows and keeps no expansion state of its own, which is what lets an open
   * container survive a refresh. */
  readonly treeWindows?: readonly ViewTreeWindowRequest[];
  /** 🌲️ Rows the tallest visible panel body fits — the shared first-paint budget a guest spends in
   * document order over containers the host has not yet seen. */
  readonly treeViewportRows?: number;
};

/** 🧭️ View-context form of the tool run `ToolRunTraceCursor` — `run` bounded to exact JavaScript integers. */
export type ViewToolRunTraceCursor = { readonly run: number; readonly generation: number; readonly page: number };

/** 📬️ One tree container's host-known state: whether the user opened or closed it (absent = the
 * author's own default still stands) and the row window on screen, overscan included. The exact twin
 * of `TreeWindowRequest` (`🛂️manifest/🦀️.rs`). */
export type ViewTreeWindowRequest = { readonly bodyKey: string; readonly nodeKey: string; readonly open?: boolean; readonly offset: number; readonly rows: number };

export type ResolvedPluginViewState = PluginViewState & { readonly locale: "en" | "de"; readonly terminology: "native" | "reuse" };

//#region 📏️PublicInvocationCapacity
/** 🎬️ Largest UTF-8 body one structurally addressed action/command JSON invocation may occupy
 * on the way into a plugin process — `🎛️public-invocation/🧬️schema/🔣️.json`'s `maxBodyBytes`, the
 * Rust mirror being `PUBLIC_INVOCATION_BODY_BYTES` (`🛂️manifest/🦀️.rs`). */
export const PUBLIC_INVOCATION_BODY_BYTES = 262_144;

/** 🔂️ Largest single JSON string one invocation may carry — `maxStringBytes`, counted as
 * escaped bytes minus their leading backslashes.
 *
 * This is the bound that actually sizes a host→guest push, and NO tool execution contract can widen
 * it: `validate_public_json_envelope` runs before the addressed tool's own `max_raw_wire_bytes` is
 * ever consulted. Any payload larger than this must be paged by its producer — see
 * {@link publicInvocationStringPages}. */
export const PUBLIC_INVOCATION_STRING_BYTES = 4_096;

/** ⏬️ Deepest object/array nesting one invocation may reach — `maxDepth`. */
export const PUBLIC_INVOCATION_DEPTH = 64;

/** 📐️ What ONE character of a raw string costs against {@link PUBLIC_INVOCATION_STRING_BYTES}
 * once the JSON encoder has written it — the exact accounting the guest performs, which skips a
 * leading `\` and counts every byte after it. Non-ASCII is charged at its `\uXXXX` escape (five per
 * UTF-16 unit), never at its shorter raw UTF-8 form, so a page cut with this function is admitted
 * whether or not the encoder escapes above U+007F. */
export function publicInvocationCharCost(character: string): number {
  const code = character.codePointAt(0) ?? 0;
  if (character === '"' || character === "\\" || character === "\n" || character === "\r" || character === "\t" || code === 0x08 || code === 0x0c) return 1;
  if (code < 0x20) return 5;
  if (code < 0x80) return 1;
  return (code > 0xffff ? 2 : 1) * 5;
}

/** 📄️ Cuts one oversized string into the page run a public invocation can actually carry —
 * each page filled to, and never past, {@link PUBLIC_INVOCATION_STRING_BYTES} as
 * {@link publicInvocationCharCost} measures it, split only on code-point boundaries.
 *
 * The twin of `public_invocation_string_pages` (`🛂️manifest/🦀️.rs`); the two must cut the same
 * payload identically. An empty input yields one empty page, so a producer always sends at least one
 * addressed page. */
export function publicInvocationStringPages(text: string): readonly string[] {
  const pages: string[] = [];
  let page = "";
  let cost = 0;
  for (const character of text) {
    const next = publicInvocationCharCost(character);
    if (cost + next > PUBLIC_INVOCATION_STRING_BYTES) {
      pages.push(page);
      page = "";
      cost = 0;
    }
    page += character;
    cost += next;
  }
  pages.push(page);
  return pages;
}

//#endregion 📏️PublicInvocationCapacity
//#region 📏️ViewContextCapacity
/** 🔣️ `panelJson` capacity in Unicode characters — the TypeScript mirror of the ONE neutral
 * declaration `🪟️view-context/🧬️schema/🔣️.json` (`maxLength`), twinned in Rust by
 * `VIEW_CONTEXT_LONG_STRING_CHARS` (`🛂️manifest/🦀️.rs`) and pinned against the schema by
 * `🧪️tests/🔬️view-context-capacity/🦀️.rs`. */
export const VIEW_CONTEXT_LONG_STRING_CHARS = 65_536;
/** 📏️ Bounded opaque inputs: panel state and one embedded surface's parameters. */
export const VIEW_CONTEXT_LONG_STRING_FIELDS = ["panelJson", "extensionInputJson"] as const;

//#endregion 📏️ViewContextCapacity
/** 🔌️ Admits an explicit host projection before it crosses a process boundary. */
export function parseResolvedPluginViewState(value: unknown): ResolvedPluginViewState {
  const object = (input: unknown): Record<string, unknown> => {
    if (input === null || typeof input !== "object" || Array.isArray(input)) throw new Error("view context: expected object");
    return input as Record<string, unknown>;
  };
  const identifier = (input: unknown): string => {
    if (typeof input !== "string" || input.length === 0 || Array.from(input).length > 256 || /[\u0000-\u001f\u007f]/u.test(input)) throw new Error("view context: invalid identifier");
    return input;
  };
  const row = Object.fromEntries(Object.entries(object(value)).filter(([, item]) => item !== null));
  const short = ["activeModeId", "activeWindowKindId", "activeUtilityId", "activeToolId", "windowId", "focusedWindowId"];
  const long = VIEW_CONTEXT_LONG_STRING_FIELDS;
  const allowed = new Set([...short, ...long, "locale", "terminology", "sessionIdentity", "activeUtilityByWindowId", "windowInstances", "toolRunTraceCursorByWindowId", "treeWindows", "treeViewportRows"]);
  if (Object.keys(row).some((key) => !allowed.has(key)) || !["en", "de"].includes(row.locale as string) || !["native", "reuse"].includes(row.terminology as string)) throw new Error("view context: explicit supported preferences required");
  for (const key of short) if (row[key] !== undefined) identifier(row[key]);
  for (const key of long) if (row[key] !== undefined && (typeof row[key] !== "string" || Array.from(row[key]).length > VIEW_CONTEXT_LONG_STRING_CHARS)) throw new Error(`view context: invalid panel data at ${key}`);
  if (row.sessionIdentity !== undefined) {
    const identity = object(row.sessionIdentity);
    if (Object.keys(identity).sort().join(",") !== "displayName,userId") throw new Error("view context: invalid session identity fields");
    identifier(identity.userId);
    identifier(identity.displayName);
  }
  if (row.activeUtilityByWindowId !== undefined) {
    const entries = Object.entries(object(row.activeUtilityByWindowId));
    if (entries.length > 64) throw new Error("view context: utility capacity exceeded");
    for (const [key, item] of entries) { identifier(key); if (item !== null) identifier(item); }
  }
  if (row.windowInstances !== undefined) {
    if (!Array.isArray(row.windowInstances) || row.windowInstances.length > 64) throw new Error("view context: window capacity exceeded");
    const ids = new Set<string>();
    for (const item of row.windowInstances) {
      const window = object(item);
      if (Object.keys(window).sort().join(",") !== "id,windowKindId") throw new Error("view context: invalid window fields");
      const id = identifier(window.id);
      identifier(window.windowKindId);
      if (ids.has(id)) throw new Error("view context: repeated window instance");
      ids.add(id);
    }
  }
  if (row.toolRunTraceCursorByWindowId !== undefined) {
    const entries = Object.entries(object(row.toolRunTraceCursorByWindowId));
    if (entries.length > 64) throw new Error("view context: trace cursor capacity exceeded");
    const bounded = (input: unknown, maximum: number): boolean => Number.isInteger(input) && (input as number) >= 0 && (input as number) <= maximum;
    for (const [key, item] of entries) {
      identifier(key);
      const cursor = object(item);
      if (Object.keys(cursor).sort().join(",") !== "generation,page,run" || !bounded(cursor.run, Number.MAX_SAFE_INTEGER) || !bounded(cursor.generation, 0xffff_ffff) || !bounded(cursor.page, 0xffff_ffff)) throw new Error("view context: invalid tool run trace cursor");
    }
  }
  const boundedInteger = (input: unknown, maximum: number): boolean => Number.isInteger(input) && (input as number) >= 0 && (input as number) <= maximum;
  if (row.treeWindows !== undefined) {
    if (!Array.isArray(row.treeWindows) || row.treeWindows.length > 128) throw new Error("view context: tree window capacity exceeded");
    for (const item of row.treeWindows) {
      const request = object(item);
      const keys = Object.keys(request).sort().join(",");
      if (keys !== "bodyKey,nodeKey,offset,rows" && keys !== "bodyKey,nodeKey,offset,open,rows") throw new Error("view context: invalid tree window fields");
      identifier(request.bodyKey);
      identifier(request.nodeKey);
      if (request.open !== undefined && typeof request.open !== "boolean") throw new Error("view context: invalid tree window open state");
      if (!boundedInteger(request.offset, 0xffff_ffff) || !boundedInteger(request.rows, 0xffff_ffff)) throw new Error("view context: invalid tree window");
    }
  }
  if (row.treeViewportRows !== undefined && !boundedInteger(row.treeViewportRows, 0xffff_ffff)) throw new Error("view context: invalid tree viewport rows");
  return structuredClone(row) as ResolvedPluginViewState;
}

/** 🔢️ Rewrites every EXACT-INTEGER view-context field through `mint`, so a transport whose own
 * number type is an IEEE double still hands the guest the integer carrier its `u64`/`u32` fields
 * decode from. The schema knows WHICH fields are integers; the crossing knows what an integer
 * carrier looks like on ITS wire — this function is the seam between the two, and the ONE place the
 * integer-typed fields of a view context are written down.
 *
 * `toolRunTraceCursorByWindowId.<window>.{run,generation,page}` is the first such field a view
 * context ever carried: `locale`, `terminology`, `windowInstances`, `activeUtilityByWindowId` and
 * `panelJson` are all strings, so no crossing had ever had to carry an integer and every one of them
 * widened it silently. The guest's `FromValue` refuses a widened float by design
 * (`🌱️value/🔁️codec/🦀️.rs`), so the WHOLE view state failed to decode and took every dispatch with
 * it — `toolRunTraceCursorByWindowId.procedural-preview.run.expected an exact u64 integer, found
 * Float(1.0)`, on both renderer doors (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * Call it on an ALREADY-admitted context ({@link parseResolvedPluginViewState}); it projects, it
 * does not validate. A context that carries no integer field is returned untouched, and `mint`
 * receives whatever the field actually holds — a crossing whose context is ALREADY in its own wire
 * form (the wgpu bridge decodes one straight out of pack) recognises its carriers and passes them
 * through rather than minting them twice. Pinned by
 * `🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json` and its Rust/TypeScript twins. */
export function viewContextWithIntegerCarriers(view: PluginViewState, mint: (value: unknown) => unknown): Record<string, unknown> {
  const cursors = view.toolRunTraceCursorByWindowId;
  const windows = view.treeWindows;
  const viewportRows = view.treeViewportRows;
  if (cursors === undefined && windows === undefined && viewportRows === undefined) return view as unknown as Record<string, unknown>;
  const projected: Record<string, unknown> = { ...view };
  if (cursors !== undefined) {
    const carried: Record<string, unknown> = {};
    for (const [windowId, cursor] of Object.entries(cursors as Readonly<Record<string, Readonly<Record<string, unknown>>>>)) {
      carried[windowId] = { run: mint(cursor.run), generation: mint(cursor.generation), page: mint(cursor.page) };
    }
    projected.toolRunTraceCursorByWindowId = carried;
  }
  if (windows !== undefined) {
    projected.treeWindows = windows.map((request) => (request.open === undefined ? { bodyKey: request.bodyKey, nodeKey: request.nodeKey, offset: mint(request.offset), rows: mint(request.rows) } : { bodyKey: request.bodyKey, nodeKey: request.nodeKey, open: request.open, offset: mint(request.offset), rows: mint(request.rows) }));
  }
  if (viewportRows !== undefined) projected.treeViewportRows = mint(viewportRows);
  return projected;
}

/** ➡️ Projects host-owned context onto one concrete window, preserving its own utility selection. */
export function windowViewContext(view: PluginViewState, windowId: string): PluginViewState | undefined {
  const window = view.windowInstances?.find((window) => window.id === windowId);
  if (!window) return undefined;
  const utility = view.activeUtilityByWindowId;
  return { ...view, windowId: window.id, activeWindowKindId: window.windowKindId ?? undefined, activeUtilityId: utility && Object.hasOwn(utility, windowId) ? utility[windowId] ?? undefined : undefined };
}

/** 🎛️ Projects app-level panels without binding their controls to a window. `focusedWindowId`
 * deliberately survives: the panel is not RENDERED FOR a window, but a panel that authors per-window
 * settings still has to know which pane the user is looking at, or its controls silently retune the
 * roster's first entry (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B12 §5.1 measured exactly that).
 *
 * 🎯️ It survives only while it names a pane the roster actually carries. A shell publishes the new
 * mode's window roster before it refocuses, and the guest's window-config capture faults on a focused
 * pane the roster does not list — BEFORE the panel's body key is matched — so every app panel
 * published nothing at all (`wgpu-ui.surface-not-published:framework.panel.*` on the wgpu shell in
 * generate mode, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Exact twin of `ViewModel::for_panel`
 * (`🛂️manifest/🦀️.rs`). */
export function panelViewContext(view: PluginViewState): PluginViewState {
  const focused = view.windowInstances?.some((window) => window.id === view.focusedWindowId) ? view.focusedWindowId : undefined;
  return { ...view, windowId: undefined, activeWindowKindId: undefined, activeUtilityId: undefined, focusedWindowId: focused };
}

/** 🔧️ Overlays the host-owned mode tool, then binds the view to a window or the panel.
 * Windowed `handleAction` must use this — {@link windowViewContext} alone keeps a stale/absent
 * session `activeToolId`, so retained tool jobs (`fillBuildTick`) never see the armed tool. */
export function hostArmedViewContext(view: PluginViewState, hostActiveToolId: string | null | undefined, windowId?: string): PluginViewState | undefined {
  const activeToolId = hostActiveToolId ?? undefined;
  const armed = view.activeToolId === activeToolId ? view : { ...view, activeToolId };
  return windowId ? windowViewContext(armed, windowId) : panelViewContext(armed);
}
//#endregion PluginRuntime
//#endregion 🔌️PluginAndAppContract
// #endregion 🛂️Manifest

/** 🎚️ Resolves utilities through exact window ownership; kind requests need an addressed or unique live instance. */
export function windowUtility(view: PluginViewState, windowId?: string): string | undefined {
  const requested = windowId ?? view.windowId;
  if (requested === undefined) return view.activeUtilityId || undefined;
  const roster = view.windowInstances ?? [];
  let id = requested;
  if (!roster.some(window => window.id === requested)) {
    const candidates = roster.filter(window => window.windowKindId === requested);
    if (candidates.length) {
      const instance = candidates.find(window => window.id === view.windowId) ?? candidates.find(window => window.id === view.focusedWindowId) ?? (candidates.length === 1 ? candidates[0] : undefined);
      if (!instance) return undefined;
      id = instance.id;
    }
  }
  const utilities = view.activeUtilityByWindowId;
  return (utilities && Object.hasOwn(utilities, id) ? utilities[id] : view.windowId === id ? view.activeUtilityId : undefined) || undefined;
}

