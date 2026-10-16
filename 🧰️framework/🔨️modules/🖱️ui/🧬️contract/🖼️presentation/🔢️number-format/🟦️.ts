/** 🔢️ Twelve significant digits shared by visible and accessible numeric controls. */
export function formatUiNumber(value: number): string {
  return Number.isFinite(value) ? Number.parseFloat(value.toPrecision(12)).toString() : "";
}

/** 🎯️ Exactly `precision` fraction digits, ties away from zero (`toFixed`), unsigned zero; the twin of `format_ui_number_fixed`. */
export function formatUiNumberFixed(value: number, precision: number): string {
  if (!Number.isFinite(value)) return "";
  if (Math.abs(value) >= 1e21) return formatUiNumber(value);
  const text = value.toFixed(Math.min(Math.max(0, Math.trunc(precision)), 15));
  return /^-[0.]*$/.test(text) ? text.slice(1) : text;
}

/** 🧮️ `value` rounded to `precision` fraction digits by the law {@link formatUiNumberFixed} prints; the twin of `round_ui_number`. */
export function roundUiNumber(value: number, precision: number): number {
  return !Number.isFinite(value) || Math.abs(value) >= 1e21 ? value : Number(formatUiNumberFixed(value, precision));
}
