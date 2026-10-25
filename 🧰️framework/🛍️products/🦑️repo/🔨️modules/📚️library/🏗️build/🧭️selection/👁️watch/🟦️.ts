import {join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {compileJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { isRuntimeExecutableV1 } from "../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

const argumentsContract=compileJsonSchemaSubset(schema.definitions.Arguments),changesContract=compileJsonSchemaSubset(schema.definitions.Changes);
export const watchBuildChangeLimitsV1=Object.freeze({files:schema.definitions.Changes.properties.files.maxItems,path:schema.definitions.Changes.properties.files.items.maxLength});
export const maximumWatchBuildDescendantsV1=schema.definitions.Policy.const.maximumDescendants;
export type BuildWatchChangesV1=Readonly<{files:readonly string[];projects:readonly string[]}>;

/** 🔌️ Owned child callback ports the composition root hands to the watcher so selection never imports the caller or its child runner. */
export interface BuildWatchChildPortV1 {
 readOwnedBuildInvocationV1(root:string,invocation:ScriptInvocation,args:readonly string[]):unknown;
 runOwnedBuildChildInvocationV1(root:string,args:readonly string[],parent:ScriptInvocation,phaseControl:ResourceLeaseControl,changes?:BuildWatchChangesV1):Promise<void>;
}

/** 👁️ Recognizes only the literal owned Repo entry among watch callback commands. */
export function watchBuildChildArgumentsV1(root:string,command:readonly string[]):string[]|undefined {
 if(argumentsContract.validate(command).length)throw Error("Bounded literal watch command required");
 if(!isRuntimeExecutableV1(command[0]??""))return undefined;
 const offset=command[1]==="repo"?2:command[1]===join(resolve(root),"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts")&&command[2]==="repo"?3:undefined;
 if(offset===undefined)return undefined;
 const args=command.slice(offset);if(!args.length)throw Error("Owned watch callback selection required");return args;
}

/** 📨️ Encodes bounded source-event labels without granting process authority. */
export function buildWatchChangesEnvironmentV1(value:BuildWatchChangesV1):NodeJS.ProcessEnv {
 if(changesContract.validate(value).length)throw Error("Closed watch source changes required");
 return {SEMIO_BUILD_CHANGED_FILES:JSON.stringify(value.files),SEMIO_BUILD_CHANGED_PROJECTS:JSON.stringify(value.projects)};
}
