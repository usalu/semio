/** ✂️ Removes an executable header before Vite prepends imports to TypeScript source. */
export function stripExecutableShebang(source: string): string {
  return source.replace(/^#![^\r\n]*(?:\r?\n|$)/u, "");
}
