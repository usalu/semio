/** 📋️ Window panes, clipboard activation: the clipboard fragment of an effect, the paste action with its retained fragment and the utility activation. */
//#region ShellHelpers
//#region 🧰️WindowActionPane
/** 📋 Fragment carried by a guest `clipboardWrite` host effect, or undefined when the effect is not that variant. */
export function clipboardWriteFragmentFromEffect(effect: unknown): unknown | undefined {
  if (effect === null || typeof effect !== "object" || !("clipboardWrite" in effect)) return undefined;
  const write = (effect as { clipboardWrite?: { fragment?: unknown } }).clipboardWrite;
  return write?.fragment;
}

export type ClipboardActionDescriptor = { readonly action: string; readonly args?: unknown };

/** 📋 Named `fragment` on a paste descriptor, when the args object carries one. */
export function pasteArgsFragment(action: { readonly args?: unknown }): unknown | undefined {
  const args = action.args;
  if (args === undefined || args === null || typeof args !== "object" || Array.isArray(args)) return undefined;
  if (!("fragment" in args)) return undefined;
  return (args as { fragment?: unknown }).fragment;
}

/** 📋 Injects a host-retained copy fragment onto paste when `args.fragment` is missing. */
export function pasteActionWithRetainedFragment<T extends ClipboardActionDescriptor>(action: T, fragment: unknown | undefined): T {
  if (action.action !== "paste" || fragment === undefined) return action;
  if (pasteArgsFragment(action) !== undefined) return action;
  const args = action.args;
  const base = args !== undefined && typeof args === "object" && args !== null && !Array.isArray(args) ? (args as Record<string, unknown>) : {};
  return { ...action, args: { ...base, fragment } };
}

/** 🧰️ Pure P5 activation decision: an empty request, or re-requesting the already-active utility, deactivates (null); otherwise the requested utility becomes active. */
export function resolveUtilityActivation(current: string | null | undefined, requested: string): string | null {
  return requested === "" || (current ?? null) === requested ? null : requested;
}
//#endregion 🧰️WindowActionPane
//#endregion ShellHelpers
