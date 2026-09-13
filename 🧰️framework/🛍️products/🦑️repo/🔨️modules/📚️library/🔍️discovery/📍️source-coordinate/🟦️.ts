/** 📍️ Resolves a zero-based source offset to its one-based line coordinate. */
export function policyLineOfIndex(content: string, index: number): number {
  return content.slice(0, index).split("\n").length;
}
