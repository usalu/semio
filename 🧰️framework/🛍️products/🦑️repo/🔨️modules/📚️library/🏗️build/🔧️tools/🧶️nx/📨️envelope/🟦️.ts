import {resolve} from "node:path";
import {refuseBuildV1} from "../../../📖️texts/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {readIssuedScriptProcessEnvelopeV1,readScriptProcessEnvelope,type ScriptInvocation,type ScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import nxCallerSchema from "../../../../⚡️caching/🚀️bootstrap/🔌️nx-plugin/📤️arguments/🧬️schema/📥️native-caller.json" with {type:"json"};
import schema from "../../../🧬️schema/📥️caller.json" with {type:"json"};
import type { NativeBuildCallerCapabilitiesV1 } from "../../../📥️caller/📋️contract/🟦️.ts";
import {isStoreRepairSelectionV1} from "../../../🧾️store-receipt/🟨️.mjs";
import {declaredCapabilitiesV1} from "../../../📥️caller/🛰️service/🟦️.ts";

/** 🧮️ Drops the parallelism grant from an Nx launch: capacity chooses the jobs after birth, so only the remaining arguments identify a declared child. */
export function nxLaunchWithoutParallelismV1(launch:readonly string[]):string[]{
 const boundary=launch.indexOf("--"),head=launch.slice(0,boundary<0?undefined:boundary),tail=boundary<0?[]:launch.slice(boundary),kept:string[]=[];
 for(let index=0;index<head.length;index++){const argument=head[index]!;if(argument==="--parallel"){index++;continue;}if(!argument.startsWith("--parallel="))kept.push(argument);}
 return [...kept,...tail];
}

/** 🧭️ Answers the launches a selected Nx child carries: the declared ones when it is declared exactly, those plus the granted-parallelism form when it is declared up to its parallelism grant, `undefined` when it is undeclared. */
export function nxChildLaunchesV1(launches:readonly(readonly string[])[],selection:readonly string[]):readonly(readonly string[])[]|undefined{
 const same=(left:readonly string[],right:readonly string[])=>JSON.stringify(left)===JSON.stringify(right);
 if(launches.some(launch=>same(launch,selection)))return launches;
 const bare=nxLaunchWithoutParallelismV1(selection);
 return launches.some(launch=>same(nxLaunchWithoutParallelismV1(launch),bare))?[...launches,[...selection]]:undefined;
}

/** 📨️ Serializes only predeclared Nx child data while retaining the exact private parent invocation. */
export function projectOwnedNxProcessEnvelopeV1(root:string,invocation:ScriptInvocation,selected:readonly string[]):ScriptProcessEnvelope{
 const original=readIssuedScriptProcessEnvelopeV1(invocation),role=declaredCapabilitiesV1(original.capabilities as NativeBuildCallerCapabilitiesV1),selection=["nx",...selected];let capabilities:object;
 if(role.tool!==undefined){if(validateJsonSchemaSubset(schema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="repo")throw refuseBuildV1("refused","nx.caller-required");const declared=role.launches.filter(launch=>launch[0]==="nx"),launches=declared.length?nxChildLaunchesV1(declared,selection)??(isStoreRepairSelectionV1(selected)?declared:undefined):undefined;if(!launches)throw refuseBuildV1("refused","nx.child-selection-undeclared",{selection:JSON.stringify(selection)});capabilities={workspaceRoot:role.workspaceRoot,arguments:launches[0],selection,buildMode:role.buildMode,native:role.native,launches};}
 else{const role=original.capabilities as import("../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts").NativeNxCallerCapabilitiesV1;const launches=validateJsonSchemaSubset(nxCallerSchema,role).length||resolve(role.workspaceRoot)!==resolve(root)||role.arguments[0]!=="nx"||!role.launches?undefined:nxChildLaunchesV1(role.launches,selection);if(!launches)throw refuseBuildV1("refused","nx.original-child-selection-undeclared",{selection:JSON.stringify(selection)});capabilities={...role,selection,launches};}
 if(validateJsonSchemaSubset(nxCallerSchema,capabilities).length)throw refuseBuildV1("refused","nx.child-data-open");return readScriptProcessEnvelope({...original,capabilities},Date.now());
}
