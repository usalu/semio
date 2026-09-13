/** 🧩️ Semantic contract validation owner. */

import { constants as fsConstants, createReadStream, createWriteStream, copyFileSync, cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, realpathSync, renameSync, rmSync, rmdirSync, statSync, unlinkSync, watch, writeFileSync } from "node:fs";



//#region 🧬️Contracts
/** 🧬️ Owned draft-07 contract module for every development product fixture.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🔣️.json */
export const DEV_SCHEMA_URL = new URL("../🔣️.json", import.meta.url);

/** 🔍️ Compiles one named export of the development contract module under strict draft-07 Ajv. */
export async function devContract(exportId: string): Promise<(value: unknown) => boolean> {
  const { default: Ajv } = await import("ajv");
  const document = JSON.parse(readFileSync(DEV_SCHEMA_URL, "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`Development contract module has no export ${exportId}`);
  return validate as (value: unknown) => boolean;
}
