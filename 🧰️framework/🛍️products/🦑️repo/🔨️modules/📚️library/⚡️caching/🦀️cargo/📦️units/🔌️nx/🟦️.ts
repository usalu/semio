/** 🔌️ Nx: module exporting `rustUnitNxProjectV1`, `rustUnitNxProjectsV1`, `rustUnitDigestEnvironmentV1`. */
import{createHash}from"node:crypto";
import{dirname}from"node:path";
import{ planRustUnitsV1 }from"../🟦️.ts";
import{ requireRustUnitPlanV1, type RustUnitPlanV1 }from"../📋️plan/🟦️.ts";
import{isRustUnitPathV1}from"../🧬️definition/🟦️.ts";
const refuse=():never=>{throw Error("Complete closed Rust Nx projection required");};

/** 🎯️ Projects one owned plan without rebuilding its predecessor graph. */
export function rustUnitNxProjectV1(plan:RustUnitPlanV1,ownerScript:string,descriptorFile:string,boundedCache:boolean){
 requireRustUnitPlanV1(plan);if(!isRustUnitPathV1(ownerScript)||!isRustUnitPathV1(descriptorFile)||typeof boundedCache!=="boolean")return refuse();return {name:"rust-unit-"+plan.unit.id,root:dirname(plan.unit.manifest).replaceAll("\\","/"),projectType:"library",metadata:{nativeRoot:dirname(plan.unit.manifest).replaceAll("\\","/")},targets:{compile:{executor:"repo:script",cache:boundedCache,inputs:[...plan.unit.inputs.map(input=>"{workspaceRoot}/"+input.path),{env:rustUnitDigestEnvironmentV1(plan.unit.id)},{dependentTasksOutputFiles:"**/receipt.json",transitive:false}],outputs:["{workspaceRoot}/"+plan.outputDirectory],dependsOn:plan.unit.dependencies.map(dependency=>"rust-unit-"+dependency.unit+":compile"),metadata:{semio:{execution:{version:1,kind:"native",scope:"production"},rustUnit:{version:1,id:plan.unit.id,digest:plan.digest,mode:plan.unit.mode,output:plan.outputDirectory}}},options:{cwd:".",command:"repo "+JSON.stringify(ownerScript)+" unit compile --descriptor "+JSON.stringify(descriptorFile)+" --unit "+JSON.stringify(plan.unit.id),forwardAllArgs:false,env:{[rustUnitDigestEnvironmentV1(plan.unit.id)]:plan.digest,SEMIO_BUILD_MODE:plan.unit.mode,SEMIO_RUST_UNIT_DIGEST:plan.digest,SEMIO_RUST_UNIT_OUTPUT:plan.outputDirectory}}}}};
}
/** 📦️ Projects a supplied unit graph once; bounded cache authorization remains explicit. */
export function rustUnitNxProjectsV1(values:unknown[],ownerScript:string,descriptorFile:string,boundedCache:boolean,outputRoot:string){
 if(!isRustUnitPathV1(ownerScript)||!isRustUnitPathV1(descriptorFile)||typeof boundedCache!=="boolean")return refuse();return Object.fromEntries(planRustUnitsV1(values,outputRoot).map(plan=>{const project=rustUnitNxProjectV1(plan,ownerScript,descriptorFile,boundedCache);return[project.name,project]as const;}));
}

/** 🧷️ Gives each physical compiler variant its explicit pre-hash environment binding. */
export function rustUnitDigestEnvironmentV1(id:string):string{if(!/^[a-z][a-z0-9_-]{0,79}$/u.test(id))return refuse();return"SEMIO_RUST_UNIT_"+createHash("sha256").update(id).digest("hex").toUpperCase();}
