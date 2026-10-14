import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
/** 🎟️ Binds one actual admitted allocation to the output slot whose prior bytes must retire first. */
export interface RustUnitChargeV1 {readonly version:1;readonly unit:string;readonly semanticDigest:string;readonly allocation:string;readonly owner:string;readonly bytes:number;readonly output:string}
/** 🛂️ Refuses foreign allocation claims before original storage recovery is attempted. */
export function readRustUnitChargeV1(value:unknown,unit:string,output:string):RustUnitChargeV1{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed Rust unit output charge required");const charge=value as RustUnitChargeV1;if(charge.unit!==unit||charge.output!==output||charge.owner!=="rust-unit:"+unit+":"+charge.semanticDigest)throw Error("Original Rust unit output charge owner required");return charge;}
