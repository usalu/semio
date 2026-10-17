/** ⌨️ Settings chrome panel, Keybindings tab: the keybinding capture step and the keybindings settings tree. */
import { Button, type TreePanelConfig, formatKeybindingShortcut, humanizeControlId, parseKeybindingChords, resolveControlLabelId, uiDataLabel } from "@semio-tech/ui-react";
import { shellLabel, shellLabelIfDefined } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import type { SettingsHostApi } from "../⚙️settings-general/🟦️.tsx";

//#region 🔖️os-chrome-panels
//#region SettingsPanel
/** ⌨️ The keys of one keydown a keybinding capture reads — a DOM `KeyboardEvent` satisfies it. */
export type KeybindingCaptureKeyV1 = Pick<globalThis.KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "shiftKey">;

/** ⌨️ What one keydown does to a running keybinding capture: Escape cancels it, a modifier on its own keeps it waiting
 * for the rest of the chord, anything else completes it as `modifiers+key` in the dispatcher's own spelling.
 *
 * A modifier's own keydown used to end the capture — every chord a hand types starts with one — so no chord with a
 * modifier could ever be recorded: pressing ⌃⌥K on "Undo" left it at ⌘Z (ticket 26/09/23 U5, measured with Playwright's
 * keyboard). Rows: `🧫️fixtures/⌨️keybinding-capture/🔣️.json`. */
export function keybindingCaptureStepV1(event: KeybindingCaptureKeyV1): { readonly kind: "cancel" } | { readonly kind: "pending" } | { readonly kind: "chord"; readonly chord: string } {
  if (event.key === "Escape") return { kind: "cancel" };
  const key = event.key.toLowerCase();
  if (key === "control" || key === "meta" || key === "alt" || key === "shift" || key === "os" || key === "altgraph") return { kind: "pending" };
  const parts: string[] = [];
  if (event.ctrlKey) parts.push("ctrl");
  if (event.metaKey) parts.push("meta");
  if (event.altKey) parts.push("alt");
  if (event.shiftKey) parts.push("shift");
  parts.push(key === " " ? "space" : key);
  return { kind: "chord", chord: parts.join("+") };
}

export function buildSettingsKeybindingsTree(host: SettingsHostApi): TreePanelConfig {
  const chordOwners = new Map<string, string>();
  for (const [controlId, keys] of host.controlKeybindings) {
    const chord = parseKeybindingChords(keys)[0];
    if (chord) chordOwners.set(chord, controlId);
  }
  const rows = [...host.controlKeybindings.entries()].sort(([left], [right]) => left.localeCompare(right));
  return {
    sections: [
      {
        id: "framework.settings.keybindings.list",
        label: shellLabel("ui.settings.tab.keybindings"),
        defaultOpen: true,
        items: rows.map(([controlId, keys]) => {
          const chord = parseKeybindingChords(keys)[0];
          const conflict = chord ? chordOwners.get(chord) !== controlId : false;
          const labelId = resolveControlLabelId(controlId);
          const named = host.controlKeybindingLabel(controlId);
          const label = named !== null ? uiDataLabel(named) : shellLabelIfDefined(labelId) ?? uiDataLabel(humanizeControlId(labelId));
          const capturing = host.keybindingCaptureControlId === controlId;
          return {
            id: `framework.settings.keybindings.${controlId}`,
            label: conflict ? `${label} (${shellLabel("settings.keybindings.conflict")})` : label,
            control: (
              <div className="flex items-center gap-single">
                <span className="min-w-[5rem] font-mono text-xs text-muted-foreground">{formatKeybindingShortcut(keys)}</span>
                <Button
                  id={`framework.settings.keybindings.capture.${controlId}`}
                  icon={{ kind: "emoji", emoji: "⌨️" }}
                  text={capturing ? shellLabel("settings.keybindings.pressKeys") : shellLabel("settings.keybindings.capture")}
                  data-keybinding-capture={controlId}
                  onClick={() => host.setKeybindingCaptureControlId(capturing ? null : controlId)}
                  onKeyDown={(event) => {
                    if (!capturing) return;
                    event.preventDefault();
                    event.stopPropagation();
                    const step = keybindingCaptureStepV1(event.nativeEvent);
                    if (step.kind === "pending") return;
                    if (step.kind === "chord") host.setKeybindingOverride(controlId, step.chord);
                    host.setKeybindingCaptureControlId(null);
                  }}
                />
                <Button id={`framework.settings.keybindings.reset.${controlId}`} variant="ghost" icon="rotate-ccw" text={shellLabel("settings.keybindings.reset")} onClick={() => host.resetKeybindingOverride(controlId)} />
              </div>
            ),
          };
        }),
      },
    ],
  };
}
//#endregion SettingsPanel
//#endregion 🔖️os-chrome-panels
