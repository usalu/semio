/** 🧾️ Incremental glTF JSON egress preserves schema roles and exact owned scalar meanings. */
import type {GltfDocument,GltfJson} from "../../../../../../🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🟦️.ts";
import {ifcNumber} from "./step.ts";
import {captureIfcIoControl,ifcCheckpoint,IfcStepError,type IfcIoControl} from "./control.ts";

/** 🧩️ Completed bounded UTF8 fragments retain their exact cumulative framed length. */
export interface IfcGltfJsonFragments {chunks:Uint8Array[];byteLength:number}
type Steps<T> = Generator<void,T>;
const own=(target:object,key:string,value:unknown):void=>{Object.defineProperty(target,key,{value,writable:true,configurable:true,enumerable:true});};
const number=(value:{readonly bits:bigint}):number=>ifcNumber({kind:"real",value});
const depth=(value:number,control:IfcIoControl):void=>{if(value>control.maxDepth!)throw new IfcStepError("glb.limit","JSON output depth exceeds admission");};

function* jsonData(value:GltfJson,level:number,control:IfcIoControl):Steps<unknown> {
 yield;depth(level,control);
 switch(value.kind){
  case "null":return null;
  case "boolean":case "string":return value.value;
  case "number":return number(value.value);
  case "array":{const items:unknown[]=[];for(const item of value.values)items.push(yield* jsonData(item,level+1,control));return items;}
  case "object":{const object=Object.create(null);for(const [name,item] of value.members){yield;own(object,name,yield* jsonData(item,level+1,control));}return object;}
  default:throw new IfcStepError("glb.json","Closed glTF JSON value required");
 }
}

function* nativeData(value:unknown,key:string,level:number,control:IfcIoControl):Steps<unknown> {
 yield;depth(level,control);
 if(typeof value==="bigint"){
  const numeric=Number(value);if(!Number.isSafeInteger(numeric))throw new IfcStepError("gltf.index","Index exceeds native JSON range");return numeric;
 }
 if(value&&typeof value==="object"&&Object.hasOwn(value,"bits"))return number(value as {bits:bigint});
 if(key==="extras"||key==="extensions")return yield* jsonData(value as GltfJson,level,control);
 if(key==="attributes"){
  const object=Object.create(null);
  for(const [name,item] of value as [string,bigint][]){yield;own(object,name,yield* nativeData(item,"",level+1,control));}
  return object;
 }
 if(key==="targets"){const items:unknown[]=[];for(const item of value as unknown[])items.push(yield* nativeData(item,"attributes",level+1,control));return items;}
 if(Array.isArray(value)){const items:unknown[]=[];for(const item of value)items.push(yield* nativeData(item,"",level+1,control));return items;}
 if(value&&typeof value==="object"){
  const object=Object.create(null),record=value as Record<string,unknown>;
  for(const name in record){
   if(!Object.hasOwn(record,name))continue;
   yield;const item=record[name];if(item===undefined||(Array.isArray(item)&&item.length===0))continue;
   own(object,name,yield* nativeData(item,name,level+1,control));
  }
  return object;
 }
 return value;
}

function* quoted(value:string,chunk:number):Generator<string> {
 yield '"';let part="";
 for(let index=0;index<value.length;index++){
  const code=value.charCodeAt(index),next=value.charCodeAt(index+1);let token:string;
  if(code===34)token='\\"';
  else if(code===92)token="\\\\";
  else if(code===8)token="\\b";
  else if(code===9)token="\\t";
  else if(code===10)token="\\n";
  else if(code===12)token="\\f";
  else if(code===13)token="\\r";
  else if(code<32||(code>=0xdc00&&code<=0xdfff)||(code>=0xd800&&code<=0xdbff&&!(next>=0xdc00&&next<=0xdfff)))token="\\u"+code.toString(16).padStart(4,"0");
  else if(code>=0xd800&&code<=0xdbff){token=value.slice(index,index+2);index++;}
  else token=value[index]!;
  if(part.length&&part.length+token.length>chunk){yield part;part="";}
  part+=token;
 }
 if(part)yield part;
 yield '"';
}

function* fragments(value:unknown,chunk:number):Generator<string> {
 if(value===null||value===undefined){yield "null";return;}
 if(typeof value==="string"){yield* quoted(value,chunk);return;}
 if(typeof value==="number"){if(!Number.isFinite(value))throw new IfcStepError("glb.json","Finite JSON number required");yield String(value);return;}
 if(typeof value==="boolean"){yield value?"true":"false";return;}
 if(Array.isArray(value)){
  yield "[";let first=true;for(const item of value){if(!first)yield ",";first=false;yield* fragments(item,chunk);}yield "]";return;
 }
 if(typeof value==="object"){
  yield "{";let first=true;const record=value as Record<string,unknown>;
  for(const key in record){if(!Object.hasOwn(record,key))continue;if(!first)yield ",";first=false;yield* quoted(key,chunk);yield ":";yield* fragments(record[key],chunk);}
  yield "}";return;
 }
 throw new IfcStepError("glb.json","JSON data value required");
}

/** ✍️ Convert one typed node per turn, then encode bounded strings without a complete JSON text or byte copy. */
export async function encodeIfcGltfJson(document:GltfDocument,control:IfcIoControl,maximumBytes:number):Promise<IfcGltfJsonFragments> {
 control=captureIfcIoControl(control);
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>0xffffffff)throw new IfcStepError("glb.limit","Finite framed JSON byte ceiling required");
 const batch=control.batchSize!,maximum=control.maxValues!,byteMaximum=Math.min(maximum,maximumBytes),steps=nativeData(document,"",1,control);let work=0,value:unknown;
 await ifcCheckpoint(control,"glb-json-values",0,maximum);
 for(;;){
  const step=steps.next();if(step.done){value=step.value;break;}
  if(++work>maximum)throw new IfcStepError("glb.limit","JSON output values exceed admission");
  if(work%batch===0)await ifcCheckpoint(control,"glb-json-values",work,maximum);
 }
 await ifcCheckpoint(control,"glb-json-values",work,work);
 const encoder=new TextEncoder(),chunks:Uint8Array[]=[];let byteLength=0,next=batch,pending="";
 const emit=async(text:string):Promise<void>=>{
  const bytes=encoder.encode(text);if(bytes.length>byteMaximum-byteLength)throw new IfcStepError("glb.limit","Encoded JSON bytes exceed admission");
  byteLength+=bytes.length;chunks.push(bytes);
  if(byteLength>=next){await ifcCheckpoint(control,"glb-json-encode",byteLength,byteMaximum);next=byteLength+batch;}
 };
 await ifcCheckpoint(control,"glb-json-encode",0,byteMaximum);
 for(const fragment of fragments(value,batch)){
  if(pending&&pending.length+fragment.length>batch){await emit(pending);pending="";}
  pending+=fragment;if(pending.length>=batch){await emit(pending);pending="";}
 }
 if(pending)await emit(pending);
 await ifcCheckpoint(control,"glb-json-encode",byteLength,byteLength);
 return{chunks,byteLength};
}
