import {createHash} from "node:crypto";
import {lstatSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 🔏️ Actual compiled bytes and their frozen source census belong to one release toolkit. */
export interface NxAuthorizerReceiptV1 {readonly version:1;readonly mode:"release";readonly sourceSha256:string;readonly entry:"semio-authorizers.mjs";readonly outputs:readonly {readonly path:"semio-authorizers.mjs";readonly sha256:string}[];}
const hash=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
function readStable(directory:string,path:string):Uint8Array{
 let parent=directory;for(;;){const status=lstatSync(parent);if(!status.isDirectory()||status.isSymbolicLink())throw Error("Immutable authorizer requires no-follow directory ancestry");const next=dirname(parent);if(next===parent)break;parent=next;}
 const absolute=join(directory,path);if(absolute.length>256)throw Error("Immutable authorizer path exceeds 256 characters");const before=lstatSync(absolute,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>8_388_608n)throw Error("Immutable authorizer requires bounded original file");const bytes=readFileSync(absolute),after=lstatSync(absolute,{bigint:true});
 if(before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs)throw Error("Immutable authorizer bytes changed during verification");return bytes;
}
/** ✅️ The SDK may import only the exact actual bundle named by its original marker. */
export function nxAuthorizerReceiptHoldsV1(directory:string,value:unknown):value is NxAuthorizerReceiptV1{
 try{
  if(validateJsonSchemaSubset({$schema:schema.$schema,$defs:schema.$defs,$ref:"#/$defs/Receipt"},value).length)return false;const receipt=value as NxAuthorizerReceiptV1,source=readStable(directory,".semio-authorizer-source.json"),manifest=JSON.parse(Buffer.from(source).toString("utf8"));
  if(hash(source)!==receipt.sourceSha256||validateJsonSchemaSubset(schema,manifest).length||!manifest.sources.some((row:{path:string})=>row.path===manifest.entry))return false;
  const paths=manifest.sources.map((row:{path:string})=>row.path),sorted=[...paths].sort((left,right)=>Buffer.compare(Buffer.from(left),Buffer.from(right)));if(new Set(paths).size!==paths.length||JSON.stringify(paths)!==JSON.stringify(sorted))return false;
  return receipt.outputs.every(row=>hash(readStable(directory,row.path))===row.sha256);
 }catch{return false;}
}
