import {createHash} from "node:crypto";
import {constants,createBrotliCompress} from "node:zlib";
import {GRAPH_STATE_POLICY_V1} from "../../../../🪪️graph-state/🟦️.ts";
import {rootRewriterV1} from "../../../🕰️generations/🌐️remote-exchange/🧳️portable/🟦️.ts";
import {neutralRowsV1,portableRowsPolicyV1,rowMemoV1,type HostV1,type ObservationRowV1,type RowMemoV1} from "../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🌊️ The declared bounds of the streamed publication of a sealed selected read: the characters one part of a document holds before it is encoded and compressed, and the peak the publication may add to the process for the realistic fixture of its law.
 * @see ./🧬️schema/🔣️.json */
export const SELECTED_STREAM_POLICY_V1=schema.definitions.Policy.const;

const sha=(value:Uint8Array|string):string=>createHash("sha256").update(value).digest("hex"),token=portableRowsPolicyV1.rootToken,workspaceOwner="(workspace)";
const canonicalRow=(row:ObservationRowV1):string=>JSON.stringify([row.kind,row.path,row.fact??null]);
const ownerOfRow=(row:ObservationRowV1,ownerOf:(relative:string)=>string|undefined):string=>{const path=typeof row.path==="string"?row.path:"";return path.startsWith(token+"/")?ownerOf(path.slice(token.length+1))??workspaceOwner:workspaceOwner;};

/** 🧾️ The sealed read the streamed publication reads: the graph text, the rows its owners were sealed for, the admission facts it consumed (as the exchange's `SelectedSealedV1`). */
export type StreamedSealedV1=Readonly<{graph:string;rows:readonly ObservationRowV1[];admission:readonly Readonly<{owner:string;suffix:string;facts:unknown;rows:readonly ObservationRowV1[]}>[]}>;
/** 📦️ One stored blob of a document: `<name>.br` or `<name>.br.<n>`, its digest and size, as the head of the exchange lists it. */
export type StreamedBlobV1=Readonly<{name:string;hash:string;size:number}>;

/** 🧮️ The neutral rows of a read grouped by owner and sorted by their canonical text, owner by owner in name order; the canonical text of every row is computed once (the exchange's group order and owner digests, without a comparator that serialises both rows of every comparison). */
function ownerGroupsV1(rows:readonly ObservationRowV1[],host:HostV1,ownerOf:(relative:string)=>string|undefined,memo:RowMemoV1=rowMemoV1()):[string,{key:string;row:ObservationRowV1}[]][]{
 const groups=new Map<string,{key:string;row:ObservationRowV1}[]>();
 for(const row of neutralRowsV1(rows,host,new Map(),portableRowsPolicyV1,memo)){const owner=ownerOfRow(row,ownerOf),list=groups.get(owner),item={key:canonicalRow(row),row};if(list)list.push(item);else groups.set(owner,[item]);}
 const sorted=[...groups].sort(([left],[right])=>left<right?-1:left>right?1:0);
 for(const [,list] of sorted)list.sort((a,b)=>a.key<b.key?-1:a.key>b.key?1:0);
 return sorted;
}


/** ✂️ A text in slices of `partCharacters`, never cut between the two halves of a surrogate pair (each slice encodes to the bytes of its part of the whole text). */
function* slicesV1(text:string):Generator<string>{
 for(let from=0;from<text.length;){let to=Math.min(text.length,from+SELECTED_STREAM_POLICY_V1.partCharacters);const last=text.charCodeAt(to-1);if(to<text.length&&last>=0xd800&&last<=0xdbff)to--;yield text.slice(from,to);from=to;}
}

/** 🗜️ A document written as a sequence of texts: each part is encoded and fed to one brotli stream (the parameters of `packGraphDocumentV1`) as soon as `partCharacters` gather, so neither the text nor the bytes of the whole document ever exist; the compressed output is cut into blobs of `blobMaximumBytes` and handed to `put` one by one. Refuses past `documentMaximumBytes` or the entry budget. */
async function streamDocumentV1(name:string,texts:Iterable<string>,budget:{left:number},put:(blob:Readonly<{name:string;bytes:Uint8Array}>)=>Promise<boolean>):Promise<readonly StreamedBlobV1[]|Readonly<{refusal:"limit-document"|"limit-entry"|"blob";text:string}>>{
 const brotli=createBrotliCompress({params:{[constants.BROTLI_PARAM_QUALITY]:5,[constants.BROTLI_PARAM_LGWIN]:22}}),output:Buffer[]=[];
 brotli.on("data",(chunk:Buffer)=>{output.push(chunk);});
 const ended=new Promise<void>((done,fail)=>{brotli.once("end",done);brotli.once("error",fail);});
 let part:string[]=[],characters=0,bytes=0,refused:"limit-document"|"limit-entry"|undefined;
 const flush=async()=>{
  if(part.length===0)return;
  const encoded=Buffer.from(part.join(""));part=[];characters=0;bytes+=encoded.length;budget.left-=encoded.length;
  if(bytes>GRAPH_STATE_POLICY_V1.documentMaximumBytes){refused="limit-document";return;}
  if(budget.left<0){refused="limit-entry";return;}
  if(!brotli.write(encoded))await new Promise<void>(done=>brotli.once("drain",()=>done()));
 };
 for(const text of texts){part.push(text);characters+=text.length;if(characters>=SELECTED_STREAM_POLICY_V1.partCharacters){await flush();if(refused)break;}}
 if(!refused)await flush();
 brotli.end();await ended;
 if(refused)return{refusal:refused,text:`the document ${name} passed the ${refused==="limit-document"?"document":"entry"} limit / das Dokument ${name} überschritt die ${refused==="limit-document"?"Dokument":"Eintrags"}grenze`};
 const packed=Buffer.concat(output),cut=GRAPH_STATE_POLICY_V1.blobMaximumBytes,blobs:StreamedBlobV1[]=[];output.length=0;
 for(let from=0,index=0;from<Math.max(1,packed.length);from+=cut,index++){
  const bytes=packed.subarray(from,Math.min(packed.length,from+cut)),blob={name:packed.length<=cut?name+".br":`${name}.br.${index}`,bytes};
  if(!await put(blob))return{refusal:"blob",text:`the blob ${blob.name} was not stored / der Blob ${blob.name} wurde nicht gespeichert`};
  blobs.push({name:blob.name,hash:sha(bytes),size:bytes.length});
 }
 return blobs;
}

/** 🌊️ The three documents of a sealed selected read (`graph`, `owners`, `admission`, the bytes the exchange's `buildSelectedGenerationV1` produces before compression) written one after the other and handed to `put` blob by blob, so a publication never holds a whole document as text, never holds two documents and converts every row to its neutral form once (one row memo for the owner groups and every admission entry): the owner groups give the owner digests and the compressed `owners` document first, `current` may then stop the publication before anything is written (a held head with the same owners), the admission rows are indexed by row and by their text (the exchange's dedupe), only the distinct row texts and the index lists stay until the document is written, and the machine root is replaced per row and per entry text (a root is a path prefix inside one JSON string, so the result equals the replacement over the whole text). Answers the owner digests and the blobs in head order, or the refusal.
 * @see ../🟦️.ts `buildSelectedGenerationV1`
 * @see ./🧪️tests/🟦️.ts */
export async function streamSelectedDocumentsV1(input:Readonly<{host:HostV1;sealed:StreamedSealedV1;ownerOf:(relative:string)=>string|undefined;current?:(owners:Readonly<Record<string,string>>)=>Promise<boolean>;put:(blob:Readonly<{name:string;bytes:Uint8Array}>)=>Promise<boolean>}>):Promise<Readonly<{owners:Readonly<Record<string,string>>;blobs:readonly StreamedBlobV1[];current?:true}>|Readonly<{refusal:"limit-document"|"limit-entry"|"blob";text:string}>>{
 const budget={left:GRAPH_STATE_POLICY_V1.entryMaximumBytes},neutral=rootRewriterV1(input.host.root,"neutralize"),memo=rowMemoV1(),blobs:StreamedBlobV1[]=[],held:{name:string;bytes:Uint8Array}[]=[];
 let groups:[string,{key:string;row:ObservationRowV1}[]][]|undefined=ownerGroupsV1(input.sealed.rows,input.host,input.ownerOf,memo);
 const owners=Object.fromEntries(groups.map(([owner,list])=>[owner,sha(list.map(item=>item.key).join("\n"))])),ordered=groups;
 const ownersDocument=await streamDocumentV1("owners",(function*(){
  yield '{"version":1,"rows":{';
  for(let at=0;at<ordered.length;at++){const [owner,list]=ordered[at]!;yield (at?",":"")+JSON.stringify(owner)+":[";for(let row=0;row<list.length;row++)yield (row?",":"")+JSON.stringify(list[row]!.row);yield "]";}
  yield "}}";
 })(),budget,async blob=>{held.push(blob);return true;});
 groups=undefined;ordered.length=0;
 if(!Array.isArray(ownersDocument))return ownersDocument as never;
 if(await input.current?.(owners))return{owners,blobs:[],current:true};
 const graph=await streamDocumentV1("graph",slicesV1(neutral(input.sealed.graph)),budget,input.put);
 if(!Array.isArray(graph))return graph as never;blobs.push(...graph);
 for(const blob of held.splice(0))if(!await input.put(blob))return{refusal:"blob",text:`the blob ${blob.name} was not stored / der Blob ${blob.name} wurde nicht gespeichert`};
 blobs.push(...ownersDocument);
 const byRow=new Map<object,number>(),byText=new Map<string,number>(),texts:string[]=[],entries:Int32Array[]=[];
 for(const entry of input.sealed.admission){
  const rows=neutralRowsV1(entry.rows,input.host,new Map(),portableRowsPolicyV1,memo),positions=new Int32Array(rows.length);
  for(let at=0;at<rows.length;at++){const row=rows[at]!;let place=byRow.get(row);if(place===undefined){const key=JSON.stringify(row);place=byText.get(key);if(place===undefined){place=texts.length;byText.set(key,place);texts.push(key);}byRow.set(row,place);}positions[at]=place;}
  entries.push(positions);
 }
 byRow.clear();byText.clear();
 const admission=await streamDocumentV1("admission",(function*(){
  yield '{"version":2,"rows":[';for(let at=0;at<texts.length;at++)yield (at?",":"")+neutral(texts[at]!);
  yield '],"entries":[';
  for(let at=0;at<input.sealed.admission.length;at++){const entry=input.sealed.admission[at]!;yield (at?",":"")+neutral(JSON.stringify({owner:entry.owner,suffix:entry.suffix,facts:entry.facts,rows:Array.from(entries[at]!)}));}
  yield "]}";
 })(),budget,input.put);
 if(!Array.isArray(admission))return admission as never;blobs.push(...admission);
 return{owners,blobs};
}
