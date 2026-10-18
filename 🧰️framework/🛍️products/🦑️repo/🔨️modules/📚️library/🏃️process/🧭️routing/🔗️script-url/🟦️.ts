import { fileURLToPath } from "node:url";

/** 🔗️Resolves `import.meta.url` of the bundle `script.ts`. */
export function scriptPathFromUrl(scriptUrl: string): string {
  return fileURLToPath(scriptUrl);
}
