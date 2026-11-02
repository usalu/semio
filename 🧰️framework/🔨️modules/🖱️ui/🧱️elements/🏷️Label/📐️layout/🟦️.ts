/** 🧮️ Property-row fit depends on intrinsic content, never allocated grid tracks. */
export type PropertyLayout = "auto" | "inline" | "stacked";

/** 📐️ The control minimum reserves one label-track width for short editable values by default. */
export function resolvePropertyRowStacked(input: { width: number; label: number; track: number; control: number; gap: number; hysteresis: number; wasStacked: boolean; mode: PropertyLayout }): boolean {
  if (input.mode !== "auto") return input.mode === "stacked";
  return Math.max(input.label, input.track) + input.control + input.gap + (input.wasStacked ? input.hysteresis : 0) > input.width;
}
