/** 🎯️ Handwritten PDF action variants and iterative ordered next containment. */
import type { PdfAction,PdfActionKind } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type PdfCell,type SqliteRow } from "../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../🧩️cos/🟦️.ts";
import { writePdfDestination,readPdfDestination,writePdfFileSpecification,readPdfFileSpecification } from "./📍️destination/🟦️.ts";
import { pdfResourceNumberColumns } from "../🖼️resource/🔢️number/🟦️.ts";
import { artifactSqliteInteger,artifactSqliteText } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { Ieee754Column } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export { writePdfDestination,readPdfDestination,writePdfFileSpecification,readPdfFileSpecification };

/** 🔢️ Exact native IEEE column contracts for navigation and bookmark entities. */
export function pdfNavigationNumberColumns(table:string):readonly Ieee754Column[]{switch(table){case "pdf_destination":return [{index:5,width:64},{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:11,width:64}];case "pdf_action":return [{index:9,width:64}];case "pdf_outline":return [{index:4,width:64},{index:5,width:64},{index:6,width:64}];default:return pdfResourceNumberColumns(table);}}
function bool(value:boolean|null|undefined):bigint|null{return value==null?null:value?1n:0n;}
function optionalBoolean(row:SqliteRow,column:number):boolean|null{return row.values[column]===null?null:pdfBoolean(row,column);}

async function writeShallow(out:PdfProjection,action:PdfAction):Promise<bigint>{
  const fields:PdfCell[]=Array(21).fill(null);const value=action.kind;
  switch(value.kind){
    case "goTo":fields[0]=await writePdfDestination(out,value.destination);break;
    case "goToRemote":fields[0]=await writePdfDestination(out,value.destination);fields[1]=await writePdfFileSpecification(out,value.file);fields[2]=bool(value.newWindow);break;
    case "goToEmbedded":fields[0]=await writePdfDestination(out,value.destination);fields[2]=bool(value.newWindow);break;
    case "launch":fields[1]=await writePdfFileSpecification(out,value.file);fields[2]=bool(value.newWindow);break;
    case "thread":fields[1]=value.file==null?null:await writePdfFileSpecification(out,value.file);fields[3]=pdfInteger(value.thread);break;
    case "uri":fields[4]=value.uri;fields[5]=value.isMap?1n:0n;break;
    case "sound":fields[6]=value.sound;fields[7]=value.volume??null;fields[8]=value.synchronous?1n:0n;fields[9]=value.repeat?1n:0n;fields[10]=value.mix?1n:0n;break;
    case "movie":fields[11]=value.annotation??null;fields[12]=value.operation??null;break;
    case "hide":fields[13]=value.hide?1n:0n;break;
    case "named":fields[14]=value.name;break;
    case "submitForm":fields[15]=value.url;fields[16]=pdfInteger(value.flags);break;
    case "resetForm":fields[16]=pdfInteger(value.flags);break;
    case "importData":fields[1]=await writePdfFileSpecification(out,value.file);break;
    case "javaScript":fields[17]=value.script;break;
    case "setOptionalContentState":fields[18]=await writePdfDictionary(out,value.states);fields[19]=value.preserveRadioButtons?1n:0n;break;
    case "rendition":case "transition":case "goTo3dView":fields[18]=await writePdfDictionary(out,value.entries);break;
    case "unknown":fields[18]=await writePdfDictionary(out,value.entries);fields[20]=value.subtype;break;
    default:throw new Error("Unknown PDF action");
  }
  const key=await out.insert("pdf_action",[value.kind,...fields]);const names=value.kind==="hide"?value.annotations:value.kind==="submitForm"||value.kind==="resetForm"?value.fields:null;
  if(names!=null){out.checkRowsAdditional(names.length);for(const[ordinal,name]of names.entries())await out.insert("pdf_action_name",[key,BigInt(ordinal),name]);}return key;
}
async function names(reader:PdfReader,key:bigint):Promise<string[]>{const values:string[]=[];for(const child of await reader.children("pdf_action_name",1,2,key)){const row=await reader.take("pdf_action_name",child.rowid,4);values.push(await reader.text(row,3));}return values;}
async function readShallow(reader:PdfReader,key:bigint):Promise<PdfAction>{
  const row=await reader.take("pdf_action",key,23);let kind:PdfActionKind;let present:number[];
  switch(artifactSqliteText(row,1)){
    case "goTo":kind={kind:"goTo",destination:await readPdfDestination(reader,artifactSqliteInteger(row,2))};present=[2];break;
    case "goToRemote":kind={kind:"goToRemote",destination:await readPdfDestination(reader,artifactSqliteInteger(row,2)),file:await readPdfFileSpecification(reader,artifactSqliteInteger(row,3)),newWindow:optionalBoolean(row,4)};present=[2,3,4];break;
    case "goToEmbedded":kind={kind:"goToEmbedded",destination:await readPdfDestination(reader,artifactSqliteInteger(row,2)),newWindow:optionalBoolean(row,4)};present=[2,4];break;
    case "launch":kind={kind:"launch",file:await readPdfFileSpecification(reader,artifactSqliteInteger(row,3)),newWindow:optionalBoolean(row,4)};present=[3,4];break;
    case "thread":kind={kind:"thread",file:row.values[3]===null?null:await readPdfFileSpecification(reader,artifactSqliteInteger(row,3)),thread:pdfNumber(row,5)};present=[3,5];break;
    case "uri":kind={kind:"uri",uri:await reader.text(row,6),isMap:pdfBoolean(row,7)};present=[6,7];break;
    case "sound":kind={kind:"sound",sound:await reader.text(row,8),volume:reader.isNull("pdf_action",row,9)?null:reader.real("pdf_action",row,9),synchronous:pdfBoolean(row,10),repeat:pdfBoolean(row,11),mix:pdfBoolean(row,12)};present=[8,9,10,11,12];break;
    case "movie":kind={kind:"movie",annotation:await reader.optionalText(row,13),operation:await reader.optionalText(row,14)};present=[13,14];break;
    case "hide":kind={kind:"hide",annotations:await names(reader,key),hide:pdfBoolean(row,15)};present=[15];break;
    case "named":kind={kind:"named",name:await reader.text(row,16)};present=[16];break;
    case "submitForm":kind={kind:"submitForm",url:await reader.text(row,17),fields:await names(reader,key),flags:pdfNumber(row,18)};present=[17,18];break;
    case "resetForm":kind={kind:"resetForm",fields:await names(reader,key),flags:pdfNumber(row,18)};present=[18];break;
    case "importData":kind={kind:"importData",file:await readPdfFileSpecification(reader,artifactSqliteInteger(row,3))};present=[3];break;
    case "javaScript":kind={kind:"javaScript",script:await reader.text(row,19)};present=[19];break;
    case "setOptionalContentState":kind={kind:"setOptionalContentState",states:await readPdfDictionary(reader,artifactSqliteInteger(row,20)),preserveRadioButtons:pdfBoolean(row,21)};present=[20,21];break;
    case "rendition":kind={kind:"rendition",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,20))};present=[20];break;
    case "transition":kind={kind:"transition",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,20))};present=[20];break;
    case "goTo3dView":kind={kind:"goTo3dView",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,20))};present=[20];break;
    case "unknown":kind={kind:"unknown",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,20)),subtype:await reader.text(row,22)};present=[20,22];break;
    default:throw new Error("Unknown PDF action");
  }
  reader.nullExcept("pdf_action",row,2,23,present);return {kind,next:[]};
}

/** 🔗️ Project ordered action trees iteratively with row budgets and cycle rejection. */
export async function writePdfAction(out:PdfProjection,action:PdfAction):Promise<bigint>{
  const root=await writeShallow(out,action);const active=new WeakSet<object>([action]);const pending=[{key:root,value:action,ordinal:0}];
  while(pending.length){const frame=pending[pending.length-1]!;const children=frame.value.next??[];out.checkRowsAdditional(2*(children.length-frame.ordinal));if(frame.ordinal<children.length){const ordinal=frame.ordinal++;const child=children[ordinal]!;if(active.has(child))throw new Error("Cyclic PDF action containment");active.add(child);const key=await writeShallow(out,child);await out.insert("pdf_action_next",[frame.key,BigInt(ordinal),key]);pending.push({key,value:child,ordinal:0});}else{active.delete(frame.value);pending.pop();}}
  return root;
}
/** 📥️ Reconstruct action trees iteratively and reject reused or cyclic entity ownership. */
export async function readPdfAction(reader:PdfReader,key:bigint):Promise<PdfAction>{
  const root=await readShallow(reader,key);const pending=[{value:root,children:await reader.children("pdf_action_next",1,2,key),ordinal:0}];
  while(pending.length){const frame=pending[pending.length-1]!;if(frame.ordinal<frame.children.length){const child=frame.children[frame.ordinal++]!;const relation=await reader.take("pdf_action_next",child.rowid,4);const key=artifactSqliteInteger(relation,3);const value=await readShallow(reader,key);frame.value.next!.push(value);pending.push({value,children:await reader.children("pdf_action_next",1,2,key),ordinal:0});}else pending.pop();}
  return root;
}
