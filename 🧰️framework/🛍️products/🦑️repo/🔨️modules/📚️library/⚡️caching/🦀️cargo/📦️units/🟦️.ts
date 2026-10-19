import{captureRustUnitInputsV1,type RustUnitPlanV1}from"./📋️plan/🟦️.ts";
import{iterateRustUnitPlansV1}from"./📋️plan/🕸️graph/🟦️.ts";

import{isRustUnitPathV1}from"./🧬️definition/🟦️.ts";
const refuse=():never=>{throw Error("Complete closed Rust unit required");};
/** 🕸️ Resolves exact dependency artifact identities in a cycle-free compiler DAG. */
export function planRustUnitsV1(values:unknown[],outputRoot:string):RustUnitPlanV1[]{if(!isRustUnitPathV1(outputRoot))return refuse();return Object.freeze([...iterateRustUnitPlansV1(captureRustUnitInputsV1(values),outputRoot)])as unknown as RustUnitPlanV1[];}
