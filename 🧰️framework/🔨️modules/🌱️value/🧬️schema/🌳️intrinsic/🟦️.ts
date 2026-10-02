/** 🌳️ Complete framework-owned intrinsic variants, independent of JSON projection. */
import{parseSchemaRecord}from"../../../🧬️schema/🧾️record/🟦️.ts";
import{parseBinary64,type Binary64}from"../../../🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type{NativeDecodeControl}from"../../🛬️decode/🟦️.ts";
export type IntrinsicValue=
 |{kind:"null"}|{kind:"boolean";value:boolean}|{kind:"unsigned";value:bigint}|{kind:"signed";value:bigint}
 |{kind:"float";value:Binary64}|{kind:"text";value:string}|{kind:"bytes";value:Uint8Array}
 |{kind:"array";items:IntrinsicValue[]}|{kind:"object";members:IntrinsicMember[]};
export interface IntrinsicMember{name:string;value:IntrinsicValue}
type Admission={kind:"slots";count:number;width:number}|{kind:"text";value:string}|{kind:"bytes";value:Uint8Array}|{kind:"step"};
type Pending={source:unknown;put:(value:IntrinsicValue)=>void}|{close:object};
function* construct(source:unknown):Generator<Admission,IntrinsicValue,string|Uint8Array|undefined>{
 yield{kind:"slots",count:1,width:32};
 let result:IntrinsicValue|undefined;const pending:Pending[]=[{source,put:value=>{result=value}}],active=new Set<object>();
 while(pending.length){
  const frame=pending.pop()!;if("close"in frame){active.delete(frame.close);continue;}
  yield{kind:"step"};const{source,put}=frame;
  if(source===null||typeof source!=="object"||!("kind"in source)||typeof source.kind!=="string"||active.has(source))throw Error("invalid intrinsic variant or cyclic ownership");
  const kind=source.kind,row=parseSchemaRecord(source,kind==="null"?["kind"]:kind==="array"?["kind","items"]:kind==="object"?["kind","members"]:["kind","value"]);
  yield{kind:"slots",count:1,width:64};active.add(source);pending.push({close:source});
  switch(kind){
   case"null":put({kind});break;
   case"boolean":if(typeof row.value!=="boolean")throw Error("invalid intrinsic boolean");put({kind,value:row.value});break;
   case"unsigned":case"signed":if(typeof row.value!=="bigint"||(kind==="unsigned"?(row.value<0n||row.value>18446744073709551615n):(row.value< -9223372036854775808n||row.value>9223372036854775807n)))throw Error("invalid intrinsic integer");put({kind,value:row.value});break;
   case"float":put({kind,value:parseBinary64(row.value)});break;
   case"text":if(typeof row.value!=="string")throw Error("invalid intrinsic text");put({kind,value:(yield{kind:"text",value:row.value}) as string});break;
   case"bytes":if(!(row.value instanceof Uint8Array))throw Error("invalid intrinsic octets");put({kind,value:(yield{kind:"bytes",value:row.value}) as Uint8Array});break;
   case"array":{if(!Array.isArray(row.items))throw Error("invalid intrinsic array");yield{kind:"slots",count:row.items.length,width:40};const items:IntrinsicValue[]=new Array(row.items.length);put({kind,items});for(let i=row.items.length-1;i>=0;i--){yield{kind:"step"};pending.push({source:row.items[i],put:value=>{items[i]=value}});}break;}
   case"object":{if(!Array.isArray(row.members))throw Error("invalid intrinsic members");yield{kind:"slots",count:row.members.length,width:64};const members:IntrinsicMember[]=new Array(row.members.length);put({kind,members});for(let i=row.members.length-1;i>=0;i--){const member=parseSchemaRecord(row.members[i],["name","value"]);if(typeof member.name!=="string")throw Error("invalid intrinsic member name");const name=(yield{kind:"text",value:member.name}) as string;pending.push({source:member.value,put:value=>{members[i]={name,value}}});}break;}
   default:throw Error("unknown intrinsic variant");
  }
 }
 return result!;
}
function scalarBytes(text:string,index:number):[number,number]{
 const word=text.charCodeAt(index);if(word<0x80)return[1,1];if(word<0x800)return[2,1];
 if(word>=0xd800&&word<=0xdbff){const next=text.charCodeAt(index+1);if(!(next>=0xdc00&&next<=0xdfff))throw Error("intrinsic text has an unpaired surrogate");return[4,2];}
 if(word>=0xdc00&&word<=0xdfff)throw Error("intrinsic text has an unpaired surrogate");return[3,1];
}
function textBytes(text:string):number{let bytes=0;for(let i=0;i<text.length;){const[size,units]=scalarBytes(text,i);bytes+=size;i+=units;}return bytes;}
/** 🛂️ Validate and own an acyclic intrinsic tree without recursive JavaScript calls. */
export function parseIntrinsicValue(source:unknown):IntrinsicValue{
 const cursor=construct(source);let input:string|Uint8Array|undefined;
 for(;;){const next=cursor.next(input);if(next.done)return next.value;const instruction=next.value;input=undefined;if(instruction.kind==="text"){textBytes(instruction.value);input=instruction.value;}else if(instruction.kind==="bytes")input=instruction.value.slice();}
}
/** 🛬️ Admit every frontier, collection, text and octet owner before materializing it. */
export async function parseIntrinsicValueControlled(source:unknown,control:NativeDecodeControl):Promise<IntrinsicValue>{
 return control.scopedStage(async()=>{
  await control.beginStage(0);const cursor=construct(source);let input:string|Uint8Array|undefined;
  try{for(;;){const next=cursor.next(input);if(next.done){await control.checkpoint();return next.value;}const instruction=next.value;input=undefined;
   if(instruction.kind==="slots")await control.admitSlots(instruction.count,instruction.width);
   else if(instruction.kind==="step")await control.step();
   else if(instruction.kind==="bytes")input=await control.copyBytes(instruction.value);
   else{await control.scopedStage(async()=>{await control.beginStage(instruction.value.length);let bytes=0;for(let i=0;i<instruction.value.length;){const[size,units]=scalarBytes(instruction.value,i);bytes+=size;i+=units;await control.advance(units);}await control.charge(bytes);});input=instruction.value;}
  }}finally{cursor.return(undefined as never);}
 });
}
