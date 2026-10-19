import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {requireIssuedScriptInvocationV1,type ScriptInvocation} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🎚️ Admits a course intent without changing the original compiler mode. */
export function courseCompilerModeV1(intent:unknown,originalMode:unknown):"prototype"|"release" {
 if(validateJsonSchemaSubset(schema,{intent,originalMode}).length)throw Error("Closed course compiler intent and original mode required");
 if(intent==="release"&&originalMode!=="release")throw Error("Course publication and rehearsal require the original release build mode; select --configuration=release");return originalMode as "prototype"|"release";
}
/** 🪪️ Reads mode only from the actual issued native caller, retaining its original control and resources. */
export function courseOriginalBuildModeV1(invocation:ScriptInvocation,intent:"dev"|"release"):"prototype"|"release" {
 requireIssuedScriptInvocationV1(invocation);return courseCompilerModeV1(intent,readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode);
}
