import{captureRustUnitInputsV1}from"./📋️plan/🟦️.ts";
import{iterateRustUnitPlansV1}from"./📋️plan/🕸️graph/🟦️.ts";
import{type RustUnitCompilerContextV1}from"./🧭️context/🟦️.ts";

import{isRustUnitPathV1,type RustUnitV1}from"./🧬️definition/🟦️.ts";
export{readRustUnitV1}from"./🧬️definition/🟦️.ts";
export type{RustUnitInputV1,RustUnitV1}from"./🧬️definition/🟦️.ts";
export interface RustUnitPlanV1 {unit:RustUnitV1;context:RustUnitCompilerContextV1;digest:string;outputDirectory:string;dependencyDigests:{unit:string;semanticDigest:string;artifactDigest:string|null}[]}
const refuse=():never=>{throw Error("Complete closed Rust unit required");};
/** 🕸️ Resolves exact dependency artifact identities in a cycle-free compiler DAG. */
export function planRustUnitsV1(values:unknown[],outputRoot:string):RustUnitPlanV1[]{if(!isRustUnitPathV1(outputRoot))return refuse();return Object.freeze([...iterateRustUnitPlansV1(captureRustUnitInputsV1(values),outputRoot)])as unknown as RustUnitPlanV1[];}
