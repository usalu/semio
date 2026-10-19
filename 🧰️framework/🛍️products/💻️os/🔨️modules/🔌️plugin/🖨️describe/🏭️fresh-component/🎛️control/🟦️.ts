import { parseCargoCommandEntryPolicyV1, type CargoCommandEntryPolicyV1, parseCargoPreparationStorageV1, type CargoPreparationStorageV1 } from "@semio-tech/repo-lib/cargo";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {FreshBuildControlV1} from "../../🧾️source-epoch/🟦️.ts";

export type FreshProcessPolicyV1=Readonly<{version:1;storage:CargoPreparationStorageV1;command:CargoCommandEntryPolicyV1}>;
export type FreshProcessControlV1=FreshBuildControlV1&Readonly<{process:FreshProcessPolicyV1;invocation:ScriptInvocation}>;
/** 🧬️ Requires complete original finite process authority before acquiring a child or diagnostic trace. */
export function parseFreshProcessPolicyV1(value:unknown):FreshProcessPolicyV1 {
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join(",")!=="command,storage,version")throw Error("Fresh process policy required");const row=value as Record<string,unknown>;if(row.version!==1)throw Error("Fresh process policy required");return Object.freeze({version:1,storage:Object.freeze(parseCargoPreparationStorageV1(row.storage)),command:Object.freeze(parseCargoCommandEntryPolicyV1(row.command))});
}
