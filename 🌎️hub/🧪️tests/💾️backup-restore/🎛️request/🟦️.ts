import {readScriptProcessJsonV1} from "@semio-tech/process/routing/invocation/process/json";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "../🧬️schema/🔣️.json";
export type BackupRestoreDrillOptions=Readonly<{version:1;kind:string;edits:number;rounds:number;heavyKinds:readonly string[]}>;
export type ShutdownDrillOptions=BackupRestoreDrillOptions;
/** 📋️ Captures finite drill DATA without accepting executable, catalog or directory authority. */
export function readHubDrillRequestV1(value:unknown):BackupRestoreDrillOptions{const copied=readScriptProcessJsonV1(value);if(validateJsonSchemaSubset(schema,copied).length)throw Error("Closed finite Hub drill request required");return copied as BackupRestoreDrillOptions;}

