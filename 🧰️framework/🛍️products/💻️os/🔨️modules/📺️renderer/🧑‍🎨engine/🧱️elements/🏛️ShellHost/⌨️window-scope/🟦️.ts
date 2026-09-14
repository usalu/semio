// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellHost/window-scope/component.ts
/** @emoji ⌨️ Which window a mode opens ACTIVE, and which window an app-wide chord addresses — as an
 * OWNED unit: no React, no shell imports, so a law can drive both decisions without building the
 * shell's element graph (the same separation `🔀️surface-switch` keeps).
 *
 * 🏁️ What this exists for: a seeded mode layout left `activeWindowId` at `null` until the user
 * clicked, and the keybinding loop resolved a chord against the FOCUSED window kind alone. Together
 * those made every window-owned verb unreachable by keyboard — `mod+shift+g` (Add Generation) on
 * `generation3d` reached nothing at boot in either mode, because no window was active AND, once one
 * was, the active window was not the one that owns the verb.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. */
// #endregion 🧲️Header

//#region 🪟️DockSeed
/** 🪟️ One tab stack of a seeded mode layout, in layout order: the window instance ids it holds and
 * the one its tab bar shows. Deliberately a projection and not the renderer's `WindowLayoutNode`, so
 * the wgpu shell's Rust twin and a fixture row describe the same thing. */
export type WindowScopeStackV1 = { readonly windowIds: readonly string[]; readonly activeWindowId?: string | null };

/** 🪟️ The layout shapes {@link modeLayoutStacksV1} walks — structurally the renderer's
 * `WindowLayoutNode` (`🖱️ui/🎯️targets/⚛️react`), restated here so this unit imports no UI target. */
export type WindowScopeLayoutNodeV1 =
  | { readonly kind: "window"; readonly id: string }
  | { readonly kind: "stack"; readonly activeId?: string; readonly children: readonly { readonly id: string }[] }
  | { readonly kind: "row" | "column"; readonly children: readonly WindowScopeLayoutNodeV1[] };

/** 🪟️ Flattens a mode layout into its tab stacks, in layout order. A bare window leaf counts as a
 * one-window stack, which is what `normalizeLayoutToStacks` makes of it downstream. */
export function modeLayoutStacksV1(node: WindowScopeLayoutNodeV1 | null | undefined): readonly WindowScopeStackV1[] {
  if (!node) return [];
  if (node.kind === "window") return [{ windowIds: [node.id], activeWindowId: node.id }];
  if (node.kind === "stack") return [{ windowIds: node.children.map((child) => child.id), activeWindowId: node.activeId ?? node.children[0]?.id ?? null }];
  return node.children.flatMap((child) => modeLayoutStacksV1(child));
}

/** 🪟️ The window a freshly seeded mode layout opens ACTIVE, or `null` when the seed must be left
 * alone: a mode with windows always has exactly one active window, so that every window-scoped verb,
 * chord badge and Actions rail has an owner before the user's first click. Answers `null` — meaning
 * "keep what you have" — when the layout is empty or when `activeWindowId` already names one of its
 * windows, which is what keeps an explicit canvas-background deactivation from being re-seeded. */
export function dockSeedActiveWindowIdV1(stacks: readonly WindowScopeStackV1[], activeWindowId: string | null | undefined): string | null {
  const windowIds = stacks.flatMap((stack) => [...stack.windowIds]);
  if (windowIds.length === 0) return null;
  if (activeWindowId != null && windowIds.includes(activeWindowId)) return null;
  for (const stack of stacks) {
    const active = stack.activeWindowId ?? stack.windowIds[0];
    if (active != null && stack.windowIds.includes(active)) return active;
  }
  return windowIds[0] ?? null;
}
//#endregion 🪟️DockSeed

//#region ⌨️KeybindingTarget
/** ⌨️ A live window instance of the active mode, with the kind whose declaration owns its verbs. */
export type WindowScopeInstanceV1 = { readonly id: string; readonly windowKindId: string };

/** ⌨️ The action ids ONE window kind declares (`window_kind_action_refs` in a plugin manifest). */
export type WindowScopeKindV1 = { readonly id: string; readonly actionIds: readonly string[] };

/** ⌨️ Where an app-wide chord lands.
 * - `focused` — the focused window's kind declares the verb; nothing moves.
 * - `owner` — another window MOUNTED IN THE ACTIVE MODE declares it; the chord addresses (and
 *   activates) that window, whether or not it is focused. This is what makes a chord table that
 *   names an app-wide key usable for a window-owned verb without naming the window twice.
 * - `unowned` — no window of the active mode declares it. The chord is a deliberate no-op with a
 *   visible hint; it must never be a silently dead key. */
export type WindowScopeTargetKindV1 = "focused" | "owner" | "unowned";
export type WindowScopeTargetV1 = { readonly kind: WindowScopeTargetKindV1; readonly windowId: string | null };

export type WindowScopeQueryV1 = {
  readonly kinds: readonly WindowScopeKindV1[];
  readonly mounted: readonly WindowScopeInstanceV1[];
  readonly focusedWindowId: string | null | undefined;
  readonly actionId: string;
};

/** ⌨️ Resolves the window an app-wide chord addresses. `mounted` is the active mode's window
 * instances in layout order — NOT every window kind the app declares, because a verb owned by a
 * window the current mode does not mount has no reachable owner and must say so. */
export function resolveKeybindingTargetWindowV1(query: WindowScopeQueryV1): WindowScopeTargetV1 {
  const actionIdsByKind = new Map(query.kinds.map((kind) => [kind.id, new Set(kind.actionIds)] as const));
  const declares = (windowKindId: string): boolean => actionIdsByKind.get(windowKindId)?.has(query.actionId) === true;
  const focused = query.focusedWindowId == null ? undefined : query.mounted.find((instance) => instance.id === query.focusedWindowId);
  if (focused && declares(focused.windowKindId)) return { kind: "focused", windowId: focused.id };
  const owner = query.mounted.find((instance) => declares(instance.windowKindId));
  if (owner) return { kind: "owner", windowId: owner.id };
  return { kind: "unowned", windowId: null };
}
//#endregion ⌨️KeybindingTarget

//#region 🛡️ReservedShellChords
/** 🛡️ Whether a chord carries the platform ACCELERATOR (`mod`/`ctrl`/`meta`). The shell reserves only
 * accelerator-carrying chords from app dispatch, which is the rule the wgpu shell's
 * `is_reserved_shell_chord` already applies: a bare key (`escape`, `enter`, `arrowleft`) belongs to
 * whichever surface has focus — it is what lets a node-graph canvas own bare arrows — while an
 * accelerator chord the shell's own chrome answers is never an app's to shadow. */
export function chordCarriesAcceleratorV1(chord: string): boolean {
  return chord
    .toLowerCase()
    .split("+")
    .slice(0, -1)
    .some((segment) => segment === "mod" || segment === "ctrl" || segment === "meta" || segment === "cmd");
}

/** 🛡️ The chords the shell's own chrome owns, as the app-keybinding loop must see them: every
 * accelerator-carrying alternative of every SHELL control binding, after the user's overrides.
 *
 * 🏁️ Why this exists: `build_definition` mints a keybinding for each framework-reserved action a
 * declared tool run injects, and `toolRunStep`'s is `mod+alt+arrowright` — the SAME chord as the
 * navbar's `ui.shell.mode.next`. With no window active the app loop never resolved it and the
 * collision was invisible; the moment a mode seeds an active window (see
 * {@link dockSeedActiveWindowIdV1}) the app half swallowed every mode switch. Two owners for one
 * chord is a framework defect either way: the shell's chrome wins, exactly as it does on wgpu. */
export function reservedShellChordsV1(shellTable: Readonly<Record<string, string>>, overrides: Readonly<Record<string, string>> = {}): ReadonlySet<string> {
  const reserved = new Set<string>();
  for (const controlId of Object.keys(shellTable)) {
    const keys = overrides[controlId] ?? shellTable[controlId] ?? "";
    for (const chord of keys.split(",").map((key) => key.trim().toLowerCase()).filter(Boolean)) {
      if (chordCarriesAcceleratorV1(chord)) reserved.add(chord);
    }
  }
  return reserved;
}
//#endregion 🛡️ReservedShellChords

//#region 🗣️UnownedHint
/** 🚦️ The hint the user SEES when a chord's verb is owned by no window of the active mode. Authored
 * here, in the unit that decides the refusal, rather than in the shell's i18n dictionary: no shell
 * import, so the fixture asserts both languages, and there is no default language — `locale` picks,
 * and nothing falls back to English except an unknown locale, which is the only case where a choice
 * has not been made. */
export type WindowScopeLabelV1 = { readonly en: string; readonly de: string };
export const KEYBINDING_UNOWNED_LABEL: WindowScopeLabelV1 = { en: "not available in this mode", de: "in diesem Modus nicht verfügbar" };
export const KEYBINDING_UNOWNED_CODE = "shell.window-scope.unowned-chord";

/** 🚦️ `⌘⇧G · Add Generation — not available in this mode`, in the reader's language. `label` is the
 * verb's already-localized label; `chord` is the chord as the app declared it. */
export function keybindingUnownedTextV1(locale: string, chord: string, label: string): string {
  return `${chord} · ${label} — ${locale === "de" ? KEYBINDING_UNOWNED_LABEL.de : KEYBINDING_UNOWNED_LABEL.en}`;
}
//#endregion 🗣️UnownedHint
