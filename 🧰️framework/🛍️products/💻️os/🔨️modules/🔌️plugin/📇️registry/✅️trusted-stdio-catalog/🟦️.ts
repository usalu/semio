/** @emoji 🗄️ Node-only reader of the generated first-party stdio native-codec catalog. Lives beside
 * `📇️registry/🟦️.ts` rather than inside it because that module is imported by the browser shell
 * (`node:fs` there breaks every Vite boot with "Module node:fs has been externalized"). */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** 🗄️ Reads the generated first-party stdio native-codec catalog written by generate / trusted-catalog-publish. */
export function readTrustedStdioCatalog(): {
  readonly schemaVersion: 1;
  readonly pluginId: "stdio";
  readonly packageId: "semio:stdio";
  readonly nativeCodecs: readonly { readonly artifactKind: string; readonly packSchemaHash: string }[];
  readonly openTargets: readonly { readonly artifactKind: string }[];
  readonly publication: string;
  readonly hubBundle: "withheld";
} {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "..", "🤖️generated", "trusted-stdio-catalog.json"), "utf8"));
}
