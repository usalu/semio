import{readPhysicalCargoSourceV1}from"../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import {createHash} from "node:crypto";
import {lstatSync,readFileSync,readdirSync} from "node:fs";
import {join,resolve,relative,dirname} from "node:path";
import {checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
import type {RustUnitPlanV1} from "../🟦️.ts";
export interface RustUnitReceiptV1 {version:1;unit:string;semanticDigest:string;mode:"prototype"|"release";dependencyDigests:{unit:string;semanticDigest:string;artifactDigest:string}[];artifacts:{path:string;digest:string;bytes:number}[]}
function ownedFile(root:string,path:string):string {
 const file=requireMaterializedPathV1(resolve(root,path));if(relative(root,file).startsWith("..")||relative(root,file)==="")throw Error("Rust unit file escapes owner");
 const ancestors=[];for(let directory=dirname(file);;directory=dirname(directory)){ancestors.push(directory);if(directory===root)break;if(directory===dirname(directory))throw Error("Rust unit file has no physical owner");}for(const directory of ancestors.reverse()){const facts=lstatSync(directory);if(facts.isSymbolicLink()||!facts.isDirectory())throw Error("Rust unit source ancestry refused");}const facts=lstatSync(file);if(facts.isSymbolicLink()||!facts.isFile())throw Error("Rust unit physical file refused");return file;
}
/** 🔎️ Checks current physical input bytes before a compiler unit can acquire output custody. */
export async function verifyRustUnitInputsV1(root:string,plan:RustUnitPlanV1,control:ResourceLeaseControl):Promise<void> {
 root=resolve(root);const inputs=plan.unit.inputs.map(input=>Object.freeze({path:input.path,bytes:input.bytes,digest:input.digest}));for(const input of inputs){checkResourceLeaseControl(control);const bytes=await readPhysicalCargoSourceV1(resolve(root,input.path),{version:1,root,maximumBytes:input.bytes,expectedBytes:input.bytes,remainingFiles:1},control);if(bytes===null||createHash("sha256").update(bytes).digest("hex")!==input.digest)throw Error("Rust unit original source changed: "+input.path);}checkResourceLeaseControl(control);
}
/** 🧾️ Captures only exact immutable output files, excluding mutable compiler trees and undeclared artifacts. */
export function captureRustUnitReceiptV1(directory:string,plan:RustUnitPlanV1,control:ResourceLeaseControl,storedReceipt?:RustUnitReceiptV1):RustUnitReceiptV1 {
 const root=resolve(directory),observed:string[]=[];const walk=(path:string):void=>{checkResourceLeaseControl(control);for(const row of readdirSync(path,{withFileTypes:true})){const selected=requireMaterializedPathV1(join(path,row.name));if(row.isSymbolicLink())throw Error("Rust unit artifact symlink refused");if(row.isDirectory())walk(selected);else if(row.isFile())observed.push(relative(root,selected).replaceAll("\\","/"));else throw Error("Rust unit artifact type refused");}};walk(root);
 if(storedReceipt){const actual=readRustUnitReceiptV1(JSON.parse(readFileSync(ownedFile(root,"receipt.json"),"utf8")),plan);if(JSON.stringify(actual)!==JSON.stringify(storedReceipt))throw Error("Rust unit stored receipt changed");observed.splice(observed.indexOf("receipt.json"),1);}if(JSON.stringify(observed.sort())!==JSON.stringify([...plan.unit.outputs].sort()))throw Error("Rust unit output inventory differs from exact declared artifacts");let total=0;
 const artifacts=observed.map(path=>{checkResourceLeaseControl(control);const file=ownedFile(root,path),facts=lstatSync(file);if(facts.size>plan.unit.limits.maximumArtifactBytes-total)throw Error("Rust unit artifact boundary exceeds its declared limit");const bytes=readFileSync(file);total+=bytes.length;if(!Number.isSafeInteger(total)||total>plan.unit.limits.maximumArtifactBytes)throw Error("Rust unit artifact boundary exceeds its declared limit");return{path,digest:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length};});checkResourceLeaseControl(control);return{version:1,unit:plan.unit.id,semanticDigest:plan.digest,mode:plan.unit.mode,dependencyDigests:plan.dependencyDigests.map(dependency=>{if(dependency.artifactDigest===null)throw Error("Current Rust dependency artifact must be bound before capture");return{...dependency,artifactDigest:dependency.artifactDigest};}),artifacts};
}

/** 📥️ Accepts a receipt only for its current complete unit and dependency artifacts. */
export function readRustUnitReceiptV1(value:unknown,plan:RustUnitPlanV1):RustUnitReceiptV1 {
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Complete Rust unit artifact receipt required");const receipt=value as RustUnitReceiptV1;
 if(receipt.unit!==plan.unit.id||receipt.semanticDigest!==plan.digest||receipt.mode!==plan.unit.mode||receipt.dependencyDigests.length!==plan.dependencyDigests.length||plan.dependencyDigests.some((expected,index)=>{const actual=receipt.dependencyDigests[index];return!actual||actual.unit!==expected.unit||actual.semanticDigest!==expected.semanticDigest||expected.artifactDigest!==null&&actual.artifactDigest!==expected.artifactDigest;})||JSON.stringify(receipt.artifacts.map(artifact=>artifact.path).sort())!==JSON.stringify([...plan.unit.outputs].sort())||receipt.artifacts.reduce((sum,artifact)=>sum+artifact.bytes,0)>plan.unit.limits.maximumArtifactBytes)throw Error("Rust unit receipt differs from current compiler identity");return receipt;
}

/** 🔗️ Binds a future predecessor only from its actual current artifact receipt. */
export function bindRustUnitDependencyArtifactsV1(plan:RustUnitPlanV1,dependencies:ReadonlyMap<string,Readonly<{plan:RustUnitPlanV1;receipt:RustUnitReceiptV1}>>):RustUnitPlanV1 {
 return{...plan,dependencyDigests:plan.dependencyDigests.map(required=>{const dependency=dependencies.get(required.unit);if(!dependency||dependency.plan.digest!==required.semanticDigest)throw Error("Missing current Rust dependency semantic identity");readRustUnitReceiptV1(dependency.receipt,dependency.plan);const artifactDigest=createHash("sha256").update(JSON.stringify(dependency.receipt)).digest("hex");if(required.artifactDigest!==null&&required.artifactDigest!==artifactDigest)throw Error("Rust predecessor artifact differs from original bound receipt");return{...required,artifactDigest};})};
}
