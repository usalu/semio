/** 🧲️ The keyboard model of a placement: a pure map from a key press to what it does.
 *
 * Left and Right move 0.1 m (Shift 1 m, Alt 0.01 m), Home and End go to the ends of the ground, Delete or Backspace removes, D duplicates, `[` and `]` select the previous and next placement.
 *
 * @see ../🟦️.tsx — where the action becomes an event
 */

/** 🎬️ What a key press asks for. */
export type KeyAction = { readonly kind: "move"; readonly deltaM: number } | { readonly kind: "to"; readonly end: "start" | "end" } | { readonly kind: "remove" } | { readonly kind: "duplicate" } | { readonly kind: "select"; readonly step: -1 | 1 };

/** ⌨️ The action of a key press, or null when the key means nothing here. */
export function keyAction(event: { readonly key: string; readonly shiftKey: boolean; readonly altKey: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }): KeyAction | null {
  if (event.ctrlKey === true || event.metaKey === true) return null;
  const size = event.altKey ? 0.01 : event.shiftKey ? 1 : 0.1;
  switch (event.key) {
    case "ArrowLeft":
      return { kind: "move", deltaM: -size };
    case "ArrowRight":
      return { kind: "move", deltaM: size };
    case "Home":
      return { kind: "to", end: "start" };
    case "End":
      return { kind: "to", end: "end" };
    case "Delete":
    case "Backspace":
      return { kind: "remove" };
    case "d":
    case "D":
      return { kind: "duplicate" };
    case "[":
      return { kind: "select", step: -1 };
    case "]":
      return { kind: "select", step: 1 };
    default:
      return null;
  }
}
