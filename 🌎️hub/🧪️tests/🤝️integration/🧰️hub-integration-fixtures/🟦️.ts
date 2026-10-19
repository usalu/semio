/** 🧰️ The shared hub integration test fixtures: the end to end switch, the test timeout and the scope export validator. */
import { hubSchemaExport } from "../../../📦️packages/🟦️typescript/🟦️.ts";

/** 🧬️ Resolves every export of one scope through the shared catalog-backed draft-07 validator. */
export function scopeExport(root: string, scope: string): (exportId: string) => (value: unknown) => boolean {
  return (exportId: string) => hubSchemaExport(root, `schema://${scope}/${exportId}`);
}
