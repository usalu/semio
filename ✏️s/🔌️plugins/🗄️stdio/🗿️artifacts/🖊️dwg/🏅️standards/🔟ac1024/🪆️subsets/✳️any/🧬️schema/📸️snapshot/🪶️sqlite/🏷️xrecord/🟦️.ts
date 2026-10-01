/** 🏷️ Typed DWG XRecord primitives, exact IEEE coordinates and ordered binary octets. */
import type { DwgXRecordValue } from "../../../🟦️.ts";
import { DwgProjection,DwgNumberRow,dwgInteger,dwgBoolean,dwgUnsignedWords,type DwgCell } from "../🔢️number/🟦️.ts";
import { DwgReader,dwgSignedWord,dwgSignedByte,dwgSignedInteger,dwgBoolean as readBoolean,dwgByte,dwgFullUnsigned } from "../🫳️reader/🟦️.ts";

export function dwgRequireNull(row:DwgNumberRow,columns:readonly number[]):void{for(const column of columns)if(row.value(column)!==null)throw new Error("DWG typed variant has a non-NULL unrelated field");}

export async function dwgProjectValues(p:DwgProjection,extended:bigint|undefined,xrecord:bigint|undefined,values:readonly DwgXRecordValue[]):Promise<void>{
  if((extended===undefined)===(xrecord===undefined))throw new Error("DWG XRecord values require exactly one typed owner");
  for(const[index,value]of values.entries()){
    let kind:string;let text:DwgCell=null,real:DwgCell=null,integer:DwgCell=null,x:DwgCell=null,y:DwgCell=null,z:DwgCell=null,high:DwgCell=null,low:DwgCell=null;
    switch(value.kind){
      case"string":kind="string";text=value.value;break;
      case"real":kind="real";real=value.value;break;
      case"boolean":kind="boolean";integer=dwgBoolean(value.value);break;
      case"integer8":kind="integer8";integer=dwgInteger(value.value,-128,127);break;
      case"integer16":kind="integer16";integer=dwgInteger(value.value,-32768,32767);break;
      case"integer32":kind="integer32";integer=dwgInteger(value.value,-2147483648,2147483647);break;
      case"integer64":kind="integer64";integer=value.value;if(typeof integer!=="bigint"||integer< -9223372036854775808n||integer>9223372036854775807n)throw new Error("DWG XRecord signed64 width");break;
      case"point3d":kind="point3d";if(value.value.length!==3)throw new Error("DWG XRecord point requires three coordinates");[x,y,z]=value.value;break;
      case"binary":kind="binary";break;
      case"handle":kind="handle";[high,low]=dwgUnsignedWords(value.value);break;
      case"objectId":kind="object_id";[high,low]=dwgUnsignedWords(value.absoluteValue);break;
      default:throw new Error("DWG XRecord primitive kind is unknown");
    }
    const id=await p.insert("dwg_xrecord_value",[extended??null,xrecord??null,BigInt(index),dwgInteger(value.groupCode,-32768,32767),kind,text,real,integer,x,y,z,high,low]);
    if(value.kind==="binary")for(const[ordinal,octet]of value.octets.entries())await p.insert("dwg_xrecord_binary_octet",[id,BigInt(ordinal),dwgInteger(octet,0,255)]);
  }
}

export async function dwgReconstructValues(reader:DwgReader,extended:bigint|undefined,xrecord:bigint|undefined):Promise<DwgXRecordValue[]>{
  if((extended===undefined)===(xrecord===undefined))throw new Error("DWG XRecord values require exactly one typed owner");
  const ownerColumn=extended===undefined?2:1,owner=extended??xrecord!;const result:DwgXRecordValue[]=[];
  for(const row of await reader.list("dwg_xrecord_value",ownerColumn,owner,3)){
    dwgRequireNull(row,[ownerColumn===1?2:1]);const groupCode=dwgSignedWord(row,4),kind=row.text(5);
    const children=await reader.list("dwg_xrecord_binary_octet",1,row.rowid,2);
    if(kind!=="binary"&&children.length!==0)throw new Error("DWG non-binary XRecord has binary octets");
    switch(kind){
      case"string":dwgRequireNull(row,[7,8,9,10,11,12,13]);result.push({kind:"string",groupCode,value:row.text(6)});break;
      case"real":dwgRequireNull(row,[6,8,9,10,11,12,13]);result.push({kind:"real",groupCode,value:row.real(7)});break;
      case"boolean":dwgRequireNull(row,[6,7,9,10,11,12,13]);result.push({kind:"boolean",groupCode,value:readBoolean(row,8)});break;
      case"integer8":dwgRequireNull(row,[6,7,9,10,11,12,13]);result.push({kind:"integer8",groupCode,value:dwgSignedByte(row,8)});break;
      case"integer16":dwgRequireNull(row,[6,7,9,10,11,12,13]);result.push({kind:"integer16",groupCode,value:dwgSignedWord(row,8)});break;
      case"integer32":dwgRequireNull(row,[6,7,9,10,11,12,13]);result.push({kind:"integer32",groupCode,value:dwgSignedInteger(row,8)});break;
      case"integer64":dwgRequireNull(row,[6,7,9,10,11,12,13]);result.push({kind:"integer64",groupCode,value:row.integer(8)});break;
      case"point3d":dwgRequireNull(row,[6,7,8,12,13]);result.push({kind:"point3d",groupCode,value:[row.real(9),row.real(10),row.real(11)]});break;
      case"binary":dwgRequireNull(row,[6,7,8,9,10,11,12,13]);result.push({kind:"binary",groupCode,octets:children.map(child=>dwgByte(child,3))});break;
      case"handle":dwgRequireNull(row,[6,7,8,9,10,11]);result.push({kind:"handle",groupCode,value:dwgFullUnsigned(row,12,13)});break;
      case"object_id":dwgRequireNull(row,[6,7,8,9,10,11]);result.push({kind:"objectId",groupCode,absoluteValue:dwgFullUnsigned(row,12,13)});break;
      default:throw new Error("DWG XRecord primitive kind is unknown");
    }
  }
  return result;
}
