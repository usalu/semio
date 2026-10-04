/** 🔣️ Lossless tagged JSON admits all intrinsic words, octets and ordered members. */
import{parseIntrinsicValue,type IntrinsicValue}from"../🟦️.ts";
import{parseSchemaRecord}from"../../../../🧬️schema/🧾️record/🟦️.ts";
type Pending={source:unknown;put:(value:IntrinsicValue)=>void}|{close:object};
function decimal(source:unknown,signed:boolean):bigint{if(typeof source!=="string"||!(signed?/^(0|-?[1-9][0-9]*)$/:/^(0|[1-9][0-9]*)$/).test(source)||source.length>20)throw Error("invalid intrinsic decimal");const value=BigInt(source);if(signed?(value< -9223372036854775808n||value>9223372036854775807n):(value>18446744073709551615n))throw Error("intrinsic decimal exceeds its domain");return value;}
function octets(source:unknown):Uint8Array{if(!Array.isArray(source)||source.some(value=>typeof value!=="number"||!Number.isInteger(value)||value<0||value>255))throw Error("invalid intrinsic octets");return Uint8Array.from(source);}
/** 📥️ Decode the declared full intrinsic JSON domain without recursive traversal. */
export function decodeIntrinsicJson(source:unknown):IntrinsicValue{
 let result:IntrinsicValue|undefined;const pending:Pending[]=[{source,put:value=>{result=value}}],active=new Set<object>();
 while(pending.length){const frame=pending.pop()!;if("close"in frame){active.delete(frame.close);continue;}const{source,put}=frame;if(source===null||typeof source!=="object"||!("kind"in source)||typeof source.kind!=="string"||active.has(source))throw Error("invalid intrinsic JSON ownership");const kind=source.kind,row=parseSchemaRecord(source,kind==="null"?["kind"]:kind==="array"?["kind","items"]:kind==="object"?["kind","members"]:["kind","value"]);active.add(source);pending.push({close:source});
  switch(kind){
   case"null":put({kind});break;
   case"boolean":if(typeof row.value!=="boolean")throw Error("invalid intrinsic boolean");put({kind,value:row.value});break;
   case"unsigned":case"signed":put({kind,value:decimal(row.value,kind==="signed")});break;
   case"float":if(typeof row.value!=="string"||!/^[0-9a-f]{16}$/.test(row.value))throw Error("invalid intrinsic binary64 word");put({kind,value:{bits:BigInt(`0x${row.value}`)}});break;
   case"text":if(typeof row.value!=="string")throw Error("invalid intrinsic text");put({kind,value:row.value});break;
   case"bytes":put({kind,value:octets(row.value)});break;
   case"array":{if(!Array.isArray(row.items))throw Error("invalid intrinsic array");const items:IntrinsicValue[]=new Array(row.items.length);put({kind,items});for(let i=row.items.length-1;i>=0;i--)pending.push({source:row.items[i],put:value=>{items[i]=value}});break;}
   case"object":{if(!Array.isArray(row.members))throw Error("invalid intrinsic members");const members:{name:string;value:IntrinsicValue}[]=new Array(row.members.length);put({kind,members});for(let i=row.members.length-1;i>=0;i--){const member=parseSchemaRecord(row.members[i],["name","value"]);if(typeof member.name!=="string")throw Error("invalid intrinsic member name");const name=member.name;pending.push({source:member.value,put:value=>{members[i]={name,value}}});}break;}
   default:throw Error("unknown intrinsic JSON kind");
  }
 }
 return parseIntrinsicValue(result);
}
