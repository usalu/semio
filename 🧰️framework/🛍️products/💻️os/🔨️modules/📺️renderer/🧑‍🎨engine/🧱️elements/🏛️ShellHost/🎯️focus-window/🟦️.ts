/** 🎯️ The `focusWindow` verb of the React shell: open a window kind when it is closed, raise it when it is open, move the keyboard focus into it and announce it politely in the interface language, with the same semantics as the Rust shell (`ShellDockState::focus_window_of_kind` and `ActionsCtx::framework_focus_window`).
 *
 * 📱️ Device-aware and domain-neutral: a kind the layout of the presented device (desktop, mobile, tablet) does not hold maps to the action's stand-in kind when that layout holds it, otherwise the focus is refused with a localized notice. Pure planning over plain data, so the language-agnostic fixture `🐚️Shell/🧪️tests/🧫️fixtures/🎯️focus-window-of-kind/🔣️.json` drives it without the shell's element graph.
 *
 * 🔗️ Rust twin: `📥️staging/⚓️dock/⌨️keyboard-and-palette/🦀️.rs` (`resolve_device_window_kind`) and `🪟️layouts/🦀️.rs`. */
import { FOCUS_WINDOW_ARG_FALLBACK, FOCUS_WINDOW_ARG_KIND, type ShellLocale } from "@semio-tech/framework";
import { shellLabelTextV1 } from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";

/** 📱️ The layout family the shell presents. */
export type FocusWindowDeviceV1 = "desktop" | "mobile" | "tablet";

/** 🪟️ One named layout reduced to what the verb reads: its id and the window kinds it holds. */
export type FocusWindowDeviceLayoutV1 = Readonly<{ id: string; windowKindIds: readonly string[] }>;

/** 🪟️ One mounted window instance and the kind it renders. */
export type FocusWindowMountedV1 = Readonly<{ windowId: string; windowKindId: string }>;

/** 🎯️ What the verb does: raise an open window, open a closed one, or refuse. */
export type FocusWindowPlanV1 =
  | Readonly<{ outcome: "raised" | "opened"; windowId: string; windowKindId: string }>
  | Readonly<{ outcome: "refused"; windowKindId: string; reason: "undeclared" | "unavailable"; device: FocusWindowDeviceV1 }>;

export const FOCUS_WINDOW_ANNOUNCED_CODE_V1 = "shell.window-focus.announced";
export const FOCUS_WINDOW_REFUSED_CODE_V1 = "shell.window-focus.refused";

type LayoutNodeV1 = Readonly<{ windowKindId?: string; children?: readonly LayoutNodeV1[] }>;

/** 🔎️ Every window kind id a manifest window layout tree references, in tree order. */
export function windowKindIdsOfLayoutV1(layout: Readonly<{ root: unknown }>): readonly string[] {
  const walk = (node: LayoutNodeV1): readonly string[] => (node.windowKindId === undefined ? (node.children ?? []).flatMap(walk) : [node.windowKindId]);
  return walk(layout.root as LayoutNodeV1);
}

/** 🪟️ The window kinds the layout `device` presents holds: the named layout whose id is the device id or ends in `-<device id>` (`norm-visual-mobile`), or `null` when the app declares none for the device, which leaves every declared kind available. */
export function deviceLayoutWindowKindsV1(layouts: readonly FocusWindowDeviceLayoutV1[], device: FocusWindowDeviceV1): readonly string[] | null {
  return layouts.find((layout) => layout.id === device || layout.id.endsWith(`-${device}`))?.windowKindIds ?? null;
}

/** 🎯️ The kind a focus lands on for a device: `requested` when its layout holds it (or declares none), else the `fallback` stand-in when the layout holds that, else `null`. */
export function resolveDeviceWindowKindV1(available: readonly string[] | null, requested: string, fallback: string | undefined): string | null {
  if (available === null || available.includes(requested)) return requested;
  return fallback !== undefined && available.includes(fallback) ? fallback : null;
}

/** 🧭️ Plans one `focusWindow`: the focused instance of the target kind, else its first instance, else a newly opened window whose id is the kind id. */
export function planFocusWindowV1(input: Readonly<{ declaredKindIds: readonly string[]; layouts: readonly FocusWindowDeviceLayoutV1[]; device: FocusWindowDeviceV1; requested: string; fallback: string | undefined; mounted: readonly FocusWindowMountedV1[]; activeWindowId: string | null }>): FocusWindowPlanV1 {
  const { declaredKindIds, layouts, device, requested, fallback, mounted, activeWindowId } = input;
  if (!declaredKindIds.includes(requested)) return { outcome: "refused", windowKindId: requested, reason: "undeclared", device };
  const target = resolveDeviceWindowKindV1(deviceLayoutWindowKindsV1(layouts, device), requested, fallback);
  if (target === null) return { outcome: "refused", windowKindId: requested, reason: "unavailable", device };
  const instances = mounted.filter((window) => window.windowKindId === target);
  const existing = instances.find((window) => window.windowId === activeWindowId) ?? instances[0];
  return existing === undefined ? { outcome: "opened", windowId: target, windowKindId: target } : { outcome: "raised", windowId: existing.windowId, windowKindId: target };
}

/** 🔊️ The polite announcement of a plan in `locale`: the same templates and codes as the Rust shell (`window.focus.*`). `nameOf` answers a window kind's label in that locale. */
export function focusWindowNoticeV1(plan: FocusWindowPlanV1, nameOf: (windowKindId: string, locale: ShellLocale) => string, locale: ShellLocale): Readonly<{ text: string; severity: "info" | "warning"; code: string }> {
  const name = nameOf(plan.windowKindId, locale);
  if (plan.outcome !== "refused") return { text: shellLabelTextV1(plan.outcome === "opened" ? "ui.windowFocus.opened" : "ui.windowFocus.raised", { name })[locale], severity: "info", code: FOCUS_WINDOW_ANNOUNCED_CODE_V1 };
  if (plan.reason === "undeclared") return { text: shellLabelTextV1("ui.windowFocus.refused", { name })[locale], severity: "warning", code: FOCUS_WINDOW_REFUSED_CODE_V1 };
  return { text: shellLabelTextV1("ui.windowFocus.unavailable", { name, device: shellLabelTextV1(`ui.windowFocus.device.${plan.device}`)[locale] })[locale], severity: "warning", code: FOCUS_WINDOW_REFUSED_CODE_V1 };
}

/** 🧾️ The window kind and optional stand-in a `focusWindow` action names; `null` while the kind argument is missing. */
export function focusWindowArgsV1(args: unknown): Readonly<{ windowKindId: string; fallback: string | undefined }> | null {
  const record = typeof args === "object" && args !== null ? (args as Readonly<Record<string, unknown>>) : {};
  const kind = record[FOCUS_WINDOW_ARG_KIND];
  const fallback = record[FOCUS_WINDOW_ARG_FALLBACK];
  return typeof kind === "string" && kind.length > 0 ? { windowKindId: kind, fallback: typeof fallback === "string" ? fallback : undefined } : null;
}

const FOCUSABLE_V1 = 'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

/** ⌨️ Moves the keyboard focus into the body of window `windowId`: the first focusable control of its tab panel, else the panel itself. Waits for the frames a freshly opened window needs to mount; answers whether the focus moved. */
export function focusWindowBodyV1(windowId: string, root: Document | undefined = typeof document === "undefined" ? undefined : document, frames = 6): Promise<boolean> {
  if (root === undefined) return Promise.resolve(false);
  const attempt = (remaining: number): Promise<boolean> => {
    const tab = [...root.querySelectorAll<HTMLElement>('[data-slot="mode-dock-tab"][data-window-id]')].find((element) => element.dataset.windowId === windowId && element.dataset.stackActive === "true");
    const panel = tab?.querySelector('[role="tab"]')?.getAttribute("aria-controls");
    const body = panel === null || panel === undefined ? null : root.getElementById(panel);
    if (body !== null) {
      (body.querySelector<HTMLElement>(FOCUSABLE_V1) ?? body).focus();
      return Promise.resolve(true);
    }
    return remaining <= 0 ? Promise.resolve(false) : new Promise((resolve) => requestAnimationFrame(() => resolve(attempt(remaining - 1))));
  };
  return attempt(frames);
}
