/** 🔢️ Twelve significant digits shared by visible and accessible numeric controls. */
export function formatUiNumber(value: number): string {
  return Number.isFinite(value) ? Number.parseFloat(value.toPrecision(12)).toString() : "";
}
