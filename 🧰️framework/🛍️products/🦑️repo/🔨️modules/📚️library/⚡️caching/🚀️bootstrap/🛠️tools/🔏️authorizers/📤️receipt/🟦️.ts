import{descriptorRefusalV1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧷️descriptor/🟦️.ts";
import {createHash} from "node:crypto";
import {closeSync,constants,fstatSync,lstatSync,openSync,readFileSync,readSync,writeSync as writeNxAuthorizerReceiptDiagnosticV1,type BigIntStats} from "node:fs";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import snapshotPolicy from "../📦️snapshot/🔣️policy.json" with {type:"json"};
import {compileJsonSchemaSubset,validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🔏️ Actual compiled bytes and their frozen source census belong to one release toolkit. */
export interface NxAuthorizerReceiptV1 {readonly version:1;readonly mode:"release";readonly sourceSha256:string;readonly entry:"semio-authorizers.mjs";readonly outputs:readonly {readonly path:"semio-authorizers.mjs";readonly sha256:string}[];}
const receiptPlan=compileJsonSchemaSubset({$ref:"#/$defs/Receipt"},schema),manifestPlan=compileJsonSchemaSubset(schema);
/** 📏️ Bounds the canonical authorizer digest manifests shared by the generation writer and readers. */
export const nxAuthorizerDigestManifestPolicyV1=schema.$defs.GenerationDigestManifestPolicy.const;
const hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
function originalAuthorizerPath(directory:string,path:string):string{
 let parent=directory;for(;;){const status=lstatSync(parent);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Immutable authorizer requires no-follow directory ancestry");const next=dirname(parent);if(next===parent)break;parent=next;}
 const absolute=join(directory,path);if(absolute.length>256)throw Error("Immutable authorizer path exceeds 256 characters");return absolute;
}
function readStable(directory:string,path:string,maximumBytes:number):Uint8Array{
 const absolute=originalAuthorizerPath(directory,path),before=lstatSync(absolute,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(maximumBytes))throw Error("Immutable authorizer requires bounded original file");const bytes=readFileSync(absolute),after=lstatSync(absolute,{bigint:true});
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Immutable authorizer bytes changed during verification");return bytes;
}
const authorizerFileStamp=(status:BigIntStats)=>[status.dev,status.ino,status.mode,status.nlink,status.size,status.mtimeNs,status.ctimeNs].join(":");
/** 🔐️ Streams only the compiler-admitted original authorizer body and rechecks its descriptor and name. */
export function observeNxAuthorizerBundleV1(directory:string,check:()=>void=()=>{}):Readonly<{sha256:string;stamp:string}>{
 check();const path=originalAuthorizerPath(directory,"semio-authorizers.mjs"),before=lstatSync(path,{bigint:true});
 if(!before.isFile()||before.isSymbolicLink()||before.size>BigInt(snapshotPolicy.maximumOutputBytes))throw Error("Original bounded authorizer module required");
 const stamp=authorizerFileStamp(before),fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)|(constants.O_NONBLOCK??0));
 try{
  check();if(authorizerFileStamp(fstatSync(fd,{bigint:true}))!==stamp)throw Error("Original authorizer descriptor required");
  const digest=createHash("sha256"),bytes=Buffer.allocUnsafe(Math.min(Number(before.size),65536));
  for(let offset=0;offset<Number(before.size);){check();const count=readSync(fd,bytes,0,Math.min(bytes.length,Number(before.size)-offset),offset);if(count===0)throw descriptorRefusalV1("immutable",path,{en:"Authorizer module",de:"Autorisierungsmodul"});digest.update(bytes.subarray(0,count));offset+=count;}
  check();if(authorizerFileStamp(fstatSync(fd,{bigint:true}))!==stamp||authorizerFileStamp(lstatSync(path,{bigint:true}))!==stamp)throw descriptorRefusalV1("immutable",path,{en:"Authorizer module",de:"Autorisierungsmodul"});return{sha256:digest.digest("hex"),stamp};
 }finally{closeSync(fd);}
}
/** 🔍️ Reports only the first original boolean refusal without observing any additional protected bytes. */
function reportNxAuthorizerReceiptFailureV1(directory:string,reason:string,issues:readonly string[]=[]):void{try{const value={version:1,purpose:"unqualified-authorizer-receipt-data",directory,reason,validation:{count:issues.length,sample:issues.slice(0,8).map(issue=>issue.slice(0,512))},authority:false,publicationRetirementDispatches:0};if(validateJsonSchemaSubset(schema.$defs.ReceiptFailureDiagnostic,value).length)return;const text="[DATA] Authorizer receipt refusal "+JSON.stringify(value)+"\n";if(Buffer.byteLength(text)>4096)return;writeNxAuthorizerReceiptDiagnosticV1(2,text);}catch{}}
/** ✅️ The SDK may import only the exact actual bundle named by its original marker. */
export function nxAuthorizerReceiptHoldsV1(directory:string,value:unknown):value is NxAuthorizerReceiptV1{
 let reason="receipt-schema";
 try{
  const receiptErrors=receiptPlan.validate(value);if(receiptErrors.length){reportNxAuthorizerReceiptFailureV1(directory,reason,receiptErrors);return false;}const receipt=value as NxAuthorizerReceiptV1;reason="source-read";const source=readStable(directory,".semio-authorizer-source.json",nxAuthorizerDigestManifestPolicyV1.maximumBytes);reason="source-json";const manifest=JSON.parse(Buffer.from(source).toString("utf8"));
  reason="source-digest";if(hash(source)!==receipt.sourceSha256){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}reason="source-manifest";const manifestErrors=manifestPlan.validate(manifest);if(manifestErrors.length){reportNxAuthorizerReceiptFailureV1(directory,reason,manifestErrors);return false;}reason="manifest-entry";if(!manifest.sources.some((row:{path:string})=>row.path===manifest.entry)){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
  reason="manifest-order";const paths=manifest.sources.map((row:{path:string})=>row.path),sorted=[...paths].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));if(new Set(paths).size!==paths.length||JSON.stringify(paths)!==JSON.stringify(sorted)){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
  reason="output-read";const {sha256}=observeNxAuthorizerBundleV1(directory);reason="output-digest";if(sha256===receipt.outputs[0]!.sha256)return true;reportNxAuthorizerReceiptFailureV1(directory,reason);return false;
 }catch{reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
}

/** 🧊️ Checks only closed current receipt byte integrity for retirement; opaque Source never drives IO or execution. */
export function nxAuthorizerRetirementIntegrityHoldsV1(value:unknown,source:Uint8Array,output:Uint8Array):value is NxAuthorizerReceiptV1{try{if(output.byteLength>snapshotPolicy.maximumOutputBytes||receiptPlan.validate(value).length)return false;const receipt=value as NxAuthorizerReceiptV1;return hash(source)===receipt.sourceSha256&&receipt.outputs.every(row=>hash(output)===row.sha256);}catch{return false;}}
