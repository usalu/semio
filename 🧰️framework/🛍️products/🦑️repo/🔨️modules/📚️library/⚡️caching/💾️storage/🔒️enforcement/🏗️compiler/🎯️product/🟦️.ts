import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {setImmediate} from "node:timers/promises";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../../🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json" with {type:"json"};

/** 🎯️ Authenticated factory facts select a real persistent compiler path; this digest issues no authority. */
export type CompilerProductIntentV1=Readonly<{version:1;root:string;manifest:string;mode:"prototype"|"release";command:string;producer:"artifact"|"descriptor-emitter";arguments:readonly string[];toolchain:string}>;

/** 🔑️ Bounded compiler arguments select the lane while the caller retains Cargo's execution options. */
export async function compilerProductKeyV1(intent:CompilerProductIntentV1,control:ResourceLeaseControl):Promise<string>{
 checkResourceLeaseControl(control);const captured=Object.freeze({...intent,arguments:Object.freeze([...intent.arguments])});
 if(validateJsonSchemaSubset({...schema,$ref:"#/definitions/ProductIntent"},captured).length)throw Error("Actual bounded compiler product intent required");
 const separator=captured.arguments.indexOf("--");
 const productArguments=captured.command==="test"
  ? captured.arguments.filter((argument,index)=>argument!=="--no-fail-fast"||(separator!==-1&&index>=separator))
  : captured.arguments;
 const hash=createHash("sha256");hash.update("semio.cargo.product/v1\n");
 for(const value of[captured.version,captured.root,captured.manifest,captured.mode,captured.command,captured.producer,captured.toolchain,...productArguments]){checkResourceLeaseControl(control);hash.update(JSON.stringify(value));hash.update("\n");await setImmediate();}
 checkResourceLeaseControl(control);return hash.digest("hex");
}

/** 🧬️ The toolchain identity of a product lane, derived from the pinned `rust-toolchain.toml` and the host alone, so the lane is known before any store, image or toolchain generation is open. Like the product key it selects a path and issues no authority. */
export function compilerProductToolchainDigestV1(workspace:string):string{
 return createHash("sha256").update("semio.cargo.product-toolchain/v1\n").update(readFileSync(join(workspace,"rust-toolchain.toml"))).update("\n"+process.platform+"\n"+process.arch).digest("hex");
}
