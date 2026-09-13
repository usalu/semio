/** 🛂️ Supplies strict third-party admission for named Flow wasm fixture contracts. */
import { readFileSync } from "node:fs";
import Ajv from "ajv";

const schemaUrl = new URL("../../../🧬️schema/🔣️.json", import.meta.url);

/** 🔍️ Compiles one named Flow wasm contract export under strict draft-07 Ajv. */
export function flowWasmContract(exportId: string): (value: unknown) => boolean {
  const document = JSON.parse(readFileSync(schemaUrl, "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`Flow wasm contract module has no export ${exportId}`);
  return validate as (value: unknown) => boolean;
}
