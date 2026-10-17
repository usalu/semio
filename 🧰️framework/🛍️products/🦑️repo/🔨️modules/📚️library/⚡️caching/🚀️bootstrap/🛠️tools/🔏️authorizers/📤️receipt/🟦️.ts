import {createHash} from "node:crypto";
import {lstatSync,readFileSync,writeSync as writeNxAuthorizerReceiptDiagnosticV1} from "node:fs";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {compileJsonSchemaSubset,validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🔏️ Actual compiled bytes and their frozen source census belong to one release toolkit. */
export interface NxAuthorizerReceiptV1 {readonly version:1;readonly mode:"release";readonly sourceSha256:string;readonly entry:"semio-authorizers.mjs";readonly outputs:readonly {readonly path:"semio-authorizers.mjs";readonly sha256:string}[];}
const receiptPlan=compileJsonSchemaSubset({$ref:"#/$defs/Receipt"},schema),manifestPlan=compileJsonSchemaSubset(schema);
const hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
function readStable(directory:string,path:string):Uint8Array{
 let parent=directory;for(;;){const status=lstatSync(parent);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Immutable authorizer requires no-follow directory ancestry");const next=dirname(parent);if(next===parent)break;parent=next;}
 const absolute=join(directory,path);if(absolute.length>256)throw Error("Immutable authorizer path exceeds 256 characters");const before=lstatSync(absolute,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>8_388_608n)throw Error("Immutable authorizer requires bounded original file");const bytes=readFileSync(absolute),after=lstatSync(absolute,{bigint:true});
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Immutable authorizer bytes changed during verification");return bytes;
}
/** 🔍️ Reports only the first original boolean refusal without observing any additional protected bytes. */
function reportNxAuthorizerReceiptFailureV1(directory:string,reason:string,issues:readonly string[]=[]):void{try{const value={version:1,purpose:"unqualified-authorizer-receipt-data",directory,reason,validation:{count:issues.length,sample:issues.slice(0,8).map(issue=>issue.slice(0,512))},authority:false,publicationRetirementDispatches:0};if(validateJsonSchemaSubset(schema.$defs.ReceiptFailureDiagnostic,value).length)return;const text="[DATA] Authorizer receipt refusal "+JSON.stringify(value)+"\n";if(Buffer.byteLength(text)>4096)return;writeNxAuthorizerReceiptDiagnosticV1(2,text);}catch{}}
/** ✅️ The SDK may import only the exact actual bundle named by its original marker. */
export function nxAuthorizerReceiptHoldsV1(directory:string,value:unknown):value is NxAuthorizerReceiptV1{
 let reason="receipt-schema";
 try{
  const receiptErrors=receiptPlan.validate(value);if(receiptErrors.length){reportNxAuthorizerReceiptFailureV1(directory,reason,receiptErrors);return false;}const receipt=value as NxAuthorizerReceiptV1;reason="source-read";const source=readStable(directory,".semio-authorizer-source.json");reason="source-json";const manifest=JSON.parse(Buffer.from(source).toString("utf8"));
  reason="source-digest";if(hash(source)!==receipt.sourceSha256){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}reason="source-manifest";const manifestErrors=manifestPlan.validate(manifest);if(manifestErrors.length){reportNxAuthorizerReceiptFailureV1(directory,reason,manifestErrors);return false;}reason="manifest-entry";if(!manifest.sources.some((row:{path:string})=>row.path===manifest.entry)){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
  reason="manifest-order";const paths=manifest.sources.map((row:{path:string})=>row.path),sorted=[...paths].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));if(new Set(paths).size!==paths.length||JSON.stringify(paths)!==JSON.stringify(sorted)){reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
  return receipt.outputs.every(row=>{reason="output-read";const bytes=readStable(directory,row.path);reason="output-digest";if(hash(bytes)===row.sha256)return true;reportNxAuthorizerReceiptFailureV1(directory,reason);return false;});
 }catch{reportNxAuthorizerReceiptFailureV1(directory,reason);return false;}
}

/** 🧊️ Checks only closed current receipt byte integrity for retirement; opaque Source never drives IO or execution. */
export function nxAuthorizerRetirementIntegrityHoldsV1(value:unknown,source:Uint8Array,output:Uint8Array):value is NxAuthorizerReceiptV1{try{if(receiptPlan.validate(value).length)return false;const receipt=value as NxAuthorizerReceiptV1;return hash(source)===receipt.sourceSha256&&receipt.outputs.every(row=>hash(output)===row.sha256);}catch{return false;}}
