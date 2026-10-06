/** 🎯️ Explicit native JSON navigation admission into the canonical owned model. */
import type { PdfDestination,PdfDestinationFit,PdfFileSpecification,PdfAction,PdfActionKind,PdfOutlineItem,PdfOpenAction,PdfNamedDestination } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { record,array,text,boolean,integer,real,four,three,pdfDictionaryFromNativeJson } from "../🟦️.ts";

const optionalText=(value:unknown)=>value==null?null:text(value);
const optionalBoolean=(value:unknown)=>value==null?null:boolean(value);
const optionalReal=(value:unknown)=>value==null?null:real(value);
/** 📍️ Admit every destination fit's owned IEEE coordinates. */
export function pdfFitFromNativeJson(input:unknown):PdfDestinationFit{const row=record(input);switch(text(row.kind)){case "xyz":return {kind:"xyz",left:optionalReal(row.left),top:optionalReal(row.top),zoom:optionalReal(row.zoom)};case "fit":return {kind:"fit"};case "fitBoundingBox":return {kind:"fitBoundingBox"};case "fitHorizontal":return {kind:"fitHorizontal",top:optionalReal(row.top)};case "fitVertical":return {kind:"fitVertical",left:optionalReal(row.left)};case "fitRectangle":return {kind:"fitRectangle",rect:four(row.rect)};case "fitBoundingBoxHorizontal":return {kind:"fitBoundingBoxHorizontal",top:optionalReal(row.top)};case "fitBoundingBoxVertical":return {kind:"fitBoundingBoxVertical",left:optionalReal(row.left)};default:throw new Error("Unknown PDF destination fit");}}
/** 📍️ Admit local, remote and named native destinations. */
export function pdfDestinationFromNativeJson(input:unknown):PdfDestination{const row=record(input);switch(text(row.kind)){case "page":return {kind:"page",page:integer(row.page),fit:pdfFitFromNativeJson(row.fit)};case "remotePage":return {kind:"remotePage",page:integer(row.page),fit:pdfFitFromNativeJson(row.fit)};case "named":return {kind:"named",name:text(row.name)};default:throw new Error("Unknown PDF destination");}}
/** 📂️ Admit explicit native paths and attachment references. */
export function pdfFileFromNativeJson(input:unknown):PdfFileSpecification{const row=record(input);switch(text(row.kind)){case "path":return {kind:"path",path:text(row.path)};case "embedded":return {kind:"embedded",file:text(row.file)};default:throw new Error("Unknown PDF file specification");}}
/** ⚡️ Admit all nineteen action payloads with their actual owner fields. */
export function pdfActionKindFromNativeJson(input:unknown):PdfActionKind{
  const row=record(input);switch(text(row.kind)){
    case "goTo":return {kind:"goTo",destination:pdfDestinationFromNativeJson(row.destination)};
    case "goToRemote":return {kind:"goToRemote",destination:pdfDestinationFromNativeJson(row.destination),file:pdfFileFromNativeJson(row.file),newWindow:optionalBoolean(row.newWindow)};
    case "goToEmbedded":return {kind:"goToEmbedded",destination:pdfDestinationFromNativeJson(row.destination),newWindow:optionalBoolean(row.newWindow)};
    case "launch":return {kind:"launch",file:pdfFileFromNativeJson(row.file),newWindow:optionalBoolean(row.newWindow)};
    case "thread":return {kind:"thread",file:row.file==null?null:pdfFileFromNativeJson(row.file),thread:integer(row.thread)};
    case "uri":return {kind:"uri",uri:text(row.uri),isMap:boolean(row.isMap)};
    case "sound":return {kind:"sound",sound:text(row.sound),volume:optionalReal(row.volume),synchronous:boolean(row.synchronous),repeat:boolean(row.repeat),mix:boolean(row.mix)};
    case "movie":return {kind:"movie",annotation:optionalText(row.annotation),operation:optionalText(row.operation)};
    case "hide":return {kind:"hide",annotations:array(row.annotations).map(text),hide:boolean(row.hide)};
    case "named":return {kind:"named",name:text(row.name)};
    case "submitForm":return {kind:"submitForm",url:text(row.url),fields:array(row.fields).map(text),flags:integer(row.flags)};
    case "resetForm":return {kind:"resetForm",fields:array(row.fields).map(text),flags:integer(row.flags)};
    case "importData":return {kind:"importData",file:pdfFileFromNativeJson(row.file)};
    case "javaScript":return {kind:"javaScript",script:text(row.script)};
    case "setOptionalContentState":return {kind:"setOptionalContentState",states:pdfDictionaryFromNativeJson(row.states),preserveRadioButtons:boolean(row.preserveRadioButtons)};
    case "rendition":return {kind:"rendition",entries:pdfDictionaryFromNativeJson(row.entries)};
    case "transition":return {kind:"transition",entries:pdfDictionaryFromNativeJson(row.entries)};
    case "goTo3dView":return {kind:"goTo3dView",entries:pdfDictionaryFromNativeJson(row.entries)};
    case "unknown":return {kind:"unknown",subtype:text(row.subtype),entries:pdfDictionaryFromNativeJson(row.entries)};
    default:throw new Error("Unknown PDF action");
  }
}
/** 🔗️ Admit ordered action trees without recursive model conversion. */
export function pdfActionFromNativeJson(input:unknown):PdfAction{const row=record(input);const root:PdfAction={kind:pdfActionKindFromNativeJson(row.kind),next:[]};const active=new WeakSet<object>([row]);const pending=[{input:row,value:root,children:array(row.next??[]),ordinal:0}];while(pending.length){const frame=pending[pending.length-1]!;if(frame.ordinal<frame.children.length){const input=record(frame.children[frame.ordinal++]);if(active.has(input))throw new Error("Cyclic native PDF action");active.add(input);const value:PdfAction={kind:pdfActionKindFromNativeJson(input.kind),next:[]};frame.value.next!.push(value);pending.push({input,value,children:array(input.next??[]),ordinal:0});}else{active.delete(frame.input);pending.pop();}}return root;}
function outlineShallow(row:Record<string,unknown>):PdfOutlineItem{return {title:text(row.title),destination:row.destination==null?null:pdfDestinationFromNativeJson(row.destination),action:row.action==null?null:pdfActionFromNativeJson(row.action),color:row.color==null?null:three(row.color),italic:boolean(row.italic??false),bold:boolean(row.bold??false),open:boolean(row.open??false),children:[],extra:pdfDictionaryFromNativeJson(row.extra??[])};}
/** 🌳️ Admit exact bookmark coordinates and ordered child trees iteratively. */
export function pdfOutlineFromNativeJson(input:unknown):PdfOutlineItem{const row=record(input);const root=outlineShallow(row);const active=new WeakSet<object>([row]);const pending=[{input:row,value:root,children:array(row.children??[]),ordinal:0}];while(pending.length){const frame=pending[pending.length-1]!;if(frame.ordinal<frame.children.length){const input=record(frame.children[frame.ordinal++]);if(active.has(input))throw new Error("Cyclic native PDF outline");active.add(input);const value=outlineShallow(input);frame.value.children!.push(value);pending.push({input,value,children:array(input.children??[]),ordinal:0});}else{active.delete(frame.input);pending.pop();}}return root;}
/** 🚪️ Admit the declared open destination or action. */
export function pdfOpenActionFromNativeJson(input:unknown):PdfOpenAction{const row=record(input);switch(text(row.kind)){case "destination":return {kind:"destination",destination:pdfDestinationFromNativeJson(row.destination)};case "action":return {kind:"action",action:pdfActionFromNativeJson(row.action)};default:throw new Error("Unknown PDF open action");}}
/** 🏷️ Admit complete named destination ownership. */
export function pdfNamedDestinationFromNativeJson(input:unknown):PdfNamedDestination{const row=record(input);return {name:text(row.name),destination:pdfDestinationFromNativeJson(row.destination)};}
