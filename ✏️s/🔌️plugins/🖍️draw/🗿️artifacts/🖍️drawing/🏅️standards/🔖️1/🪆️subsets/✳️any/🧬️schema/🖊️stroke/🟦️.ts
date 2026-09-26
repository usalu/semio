/** 🖊️ Parse a bounded sequence of nonnegative lengths separated by spaces. */
export function parseStrokeDash(value: string): number[] | null {
  if (value.length > 128 || !/^[ \t\r\n]*(?:(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[ \t\r\n]+(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+))*)?[ \t\r\n]*$/.test(value)) throw new Error("Invalid dash pattern");
  const dash = value.trim() ? value.trim().split(/[ \t\r\n]+/).map(Number) : [];
  return dash.some(length => length > 0) ? dash : null;
}
