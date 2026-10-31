import {createHash} from "node:crypto";
import {setImmediate} from "node:timers/promises";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../../🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json" with {type:"json"};

/** 🎯️ Authenticated factory facts select a real persistent compiler path; this digest issues no authority. */
export type CompilerProductIntentV1=Readonly<{version:1;root:string;manifest:string;mode:"prototype"|"release";command:string;producer:"artifact"|"descriptor-emitter";arguments:readonly string[];toolchain:string}>;

/** 🔑️ Bounded exact spelling preserves path-observable compiler behavior without conflating execution owners. */
export async function compilerProductKeyV1(intent:CompilerProductIntentV1,control:ResourceLeaseControl):Promise<string>{
 checkResourceLeaseControl(control);const captured=Object.freeze({...intent,arguments:Object.freeze([...intent.arguments])});
 if(validateJsonSchemaSubset({...schema,$ref:"#/definitions/ProductIntent"},captured).length)throw Error("Actual bounded compiler product intent required");
 const hash=createHash("sha256");hash.update("semio.cargo.product/v1\n");
 for(const value of[captured.version,captured.root,captured.manifest,captured.mode,captured.command,captured.producer,captured.toolchain,...captured.arguments]){checkResourceLeaseControl(control);hash.update(JSON.stringify(value));hash.update("\n");await setImmediate();}
 checkResourceLeaseControl(control);return hash.digest("hex");
}
