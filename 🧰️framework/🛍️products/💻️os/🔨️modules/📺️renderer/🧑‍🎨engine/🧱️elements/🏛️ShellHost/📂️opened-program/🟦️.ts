/** 📂️ Where the program of a document the user opened is shown, the rule `🔣️.json` states case by case. */

export type OpenedProgramRefV1 = Readonly<{ pluginId: string; appId: string; instanceId: number }>;

/** 🧭️ `focus-tab`: a host shows it as its focused tab; `adopt-primary`: a standalone program host replaces its one program with it; `already-primary`: it already is that program. */
export type OpenedProgramPublicationV1 = "focus-tab" | "already-primary" | "adopt-primary";

/** 🪟️ Decides how an opened document's program takes the canvas, so every window of the layout follows the opened document.
 * @see ./🔣️.json */
export function openedProgramPublicationV1(hostMode: boolean, primary: OpenedProgramRefV1 | null, opened: OpenedProgramRefV1): OpenedProgramPublicationV1 {
  if (hostMode) return "focus-tab";
  return primary !== null && primary.pluginId === opened.pluginId && primary.appId === opened.appId && primary.instanceId === opened.instanceId ? "already-primary" : "adopt-primary";
}
