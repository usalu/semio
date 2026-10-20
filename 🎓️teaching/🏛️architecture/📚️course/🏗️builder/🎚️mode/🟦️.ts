import schema from "./🧬️schema/🔣️.json";
import messages from "./🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {requireIssuedScriptInvocationV1,type ScriptInvocation} from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

if(validateJsonSchemaSubset(schema.definitions.Messages,messages,schema).length)throw Error("Closed course mode messages required");

/** 🗣️ Names a course mode refusal; every refusal is stated in English and German. */
export type CourseModeRefusalV1=keyof typeof messages.refusals;

/** 💬️ Joins one refusal's English line and German line, English first, for the developer's terminal. */
export function courseModeRefusalMessageV1(code:CourseModeRefusalV1):string {
 return `${messages.refusals[code].en}\n${messages.refusals[code].de}`;
}

/** 🎚️ Admits a course intent without changing the original compiler mode. */
export function courseCompilerModeV1(intent:unknown,originalMode:unknown):"prototype"|"release" {
 if(validateJsonSchemaSubset(schema,{intent,originalMode}).length)throw Error(courseModeRefusalMessageV1("closed-request"));
 if(intent==="release"&&originalMode!=="release")throw Error(courseModeRefusalMessageV1("release-required"));return originalMode as "prototype"|"release";
}
/** 🪪️ Reads mode only from the actual issued native caller of either build tool, retaining its original control and resources. */
export function courseOriginalBuildModeV1(invocation:ScriptInvocation,intent:"dev"|"release"):"prototype"|"release" {
 requireIssuedScriptInvocationV1(invocation);return courseCompilerModeV1(intent,readNativeCallerAuthorityV1(invocation.capabilities).original.buildMode);
}
