/** 🧰️ The shared node graph host test fixtures: the peer schema export validator. */
import { type ValidateFunction } from "ajv";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";

/** 🧬️ Compiles one named export of the `os.renderer` schema module. */
/** 🧬️ Compiles one named `$defs` export of a peer scope's `🧬️schema/` module. */
export const peerExport = (module: { $id: string }, exportId: string): ValidateFunction => semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(module).getSchema(`${module.$id}#/$defs/${exportId}`) as ValidateFunction;
