/** 🏷️ Individually authored PDF annotation subtype entities and geometric relationships. */
import type { PdfAnnotationKind } from "../../../🟦️.ts";
import { PdfProjection,PdfReader,pdfInteger,pdfNumber,pdfBoolean,type PdfCell,type Binary64,type SqliteRow } from "../../🧩️entity/🟦️.ts";
import { writePdfDictionary,readPdfDictionary } from "../../🧩️cos/🟦️.ts";
import { writePdfAction,readPdfAction,writePdfDestination,readPdfDestination,writePdfFileSpecification,readPdfFileSpecification } from "../../🎯️navigation/🟦️.ts";
import { artifactSqliteInteger,artifactSqliteText } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";

const O={state:2,stateModel:3,action:4,destination:5,highlight:6,defaultAppearance:7,quadding:8,lineEnding:9,richText:10,endingStart:11,endingEnd:12,leaderLength:13,caption:14,icon:15,symbol:16,parentHigh:17,parentLow:18,open:19,file:20,sound:21,title:22,movie:23,activation:24,field:25,characteristics:26,additionalActions:27,markStyle:28,colorants:29,entries:30,fixedPrint:31,overlayText:32,repeat:33,subtype:34,interiorPresent:35,calloutPresent:36,point0:37,point1:38,point2:39,point3:40}as const;
/** 🌐️ Project the full native index as explicitly declared unsigned high and low words. */
export function pdfWideCells(value:bigint|null|undefined):[bigint|null,bigint|null]{if(value==null)return [null,null];if(typeof value!=="bigint")throw new Error("PDF owned index requires bigint");const integer=pdfInteger(value,64);return [integer>>32n,integer&0xffff_ffffn];}
/** 📥️ Reject absent or overflowing words and restore a full owned unsigned index. */
export function readPdfWide(row:SqliteRow,column:number):bigint|null{if(row.values[column]===null&&row.values[column+1]===null)return null;if(row.values[column]===null||row.values[column+1]===null)throw new Error("PDF wide index has an absent word");return (BigInt(pdfNumber(row,column))<<32n)|BigInt(pdfNumber(row,column+1));}
async function writeReals(out:PdfProjection,key:bigint,role:string,values:readonly Binary64[]):Promise<void>{out.checkRowsAdditional(values.length);for(const[ordinal,value]of values.entries())await out.insert("pdf_annotation_kind_real",[key,BigInt(ordinal),role,value]);}
async function readReals(reader:PdfReader,key:bigint,role:string):Promise<Binary64[]>{const values:Binary64[]=[];for(const child of await reader.children("pdf_annotation_kind_real",1,2,key,3,role)){const row=await reader.take("pdf_annotation_kind_real",child.rowid,5);values.push(reader.real("pdf_annotation_kind_real",row,4));}return values;}
async function readOptionalReals(reader:PdfReader,row:SqliteRow,key:bigint,column:number,role:string):Promise<Binary64[]|null>{return pdfBoolean(row,column)?readReals(reader,key,role):null;}

/** 📌️ Project all 27 subtype payloads into their hand-declared scalar and child fields. */
export async function writePdfAnnotationKind(out:PdfProjection,value:PdfAnnotationKind):Promise<bigint>{
  const fields:PdfCell[]=Array(39).fill(null);const put=(column:number,value:PdfCell)=>{fields[column-2]=value;};let points:readonly Binary64[]|null=null;
  switch(value.kind){
    case "text":put(O.open,value.open?1n:0n);put(O.icon,value.icon??null);put(O.state,value.state??null);put(O.stateModel,value.stateModel??null);break;
    case "link":put(O.action,value.action==null?null:await writePdfAction(out,value.action));put(O.destination,value.destination==null?null:await writePdfDestination(out,value.destination));put(O.highlight,value.highlight??null);break;
    case "freeText":put(O.defaultAppearance,value.defaultAppearance);put(O.quadding,pdfInteger(value.quadding));put(O.calloutPresent,value.callout==null?0n:1n);put(O.lineEnding,value.lineEnding??null);put(O.richText,value.richText??null);break;
    case "line":points=value.points;if(value.lineEndings!=null){if(value.lineEndings.length!==2)throw new Error("PDF line endings require two values");put(O.endingStart,value.lineEndings[0]);put(O.endingEnd,value.lineEndings[1]);}put(O.interiorPresent,value.interiorColor==null?0n:1n);put(O.leaderLength,value.leaderLength??null);put(O.caption,value.caption?1n:0n);break;
    case "square":case "circle":points=value.rectDifferences??null;put(O.interiorPresent,value.interiorColor==null?0n:1n);break;
    case "polygon":put(O.interiorPresent,value.interiorColor==null?0n:1n);break;
    case "polyLine":if(value.lineEndings!=null){if(value.lineEndings.length!==2)throw new Error("PDF line endings require two values");put(O.endingStart,value.lineEndings[0]);put(O.endingEnd,value.lineEndings[1]);}put(O.interiorPresent,value.interiorColor==null?0n:1n);break;
    case "highlight":case "underline":case "squiggly":case "strikeOut":case "ink":break;
    case "stamp":put(O.icon,value.icon??null);break;
    case "caret":points=value.rectDifferences??null;put(O.symbol,value.symbol??null);break;
    case "popup":{const words=pdfWideCells(value.parent);put(O.parentHigh,words[0]);put(O.parentLow,words[1]);put(O.open,value.open?1n:0n);break;}
    case "fileAttachment":put(O.file,await writePdfFileSpecification(out,value.file));put(O.icon,value.icon??null);break;
    case "sound":put(O.sound,await writePdfDictionary(out,value.sound));put(O.icon,value.icon??null);break;
    case "movie":put(O.title,value.title??null);put(O.movie,await writePdfDictionary(out,value.movie));put(O.activation,value.activation==null?null:await writePdfDictionary(out,value.activation));break;
    case "widget":put(O.field,value.field??null);put(O.highlight,value.highlight??null);put(O.characteristics,await writePdfDictionary(out,value.characteristics));put(O.action,value.action==null?null:await writePdfAction(out,value.action));put(O.additionalActions,await writePdfDictionary(out,value.additionalActions));break;
    case "screen":put(O.title,value.title??null);put(O.characteristics,await writePdfDictionary(out,value.characteristics));put(O.action,value.action==null?null:await writePdfAction(out,value.action));put(O.additionalActions,await writePdfDictionary(out,value.additionalActions));break;
    case "printerMark":put(O.markStyle,value.markStyle??null);put(O.colorants,await writePdfDictionary(out,value.colorants));break;
    case "trapNet":case "threeD":put(O.entries,await writePdfDictionary(out,value.entries));break;
    case "watermark":put(O.fixedPrint,value.fixedPrint==null?null:await writePdfDictionary(out,value.fixedPrint));break;
    case "redact":put(O.interiorPresent,value.interiorColor==null?0n:1n);put(O.overlayText,value.overlayText??null);put(O.repeat,value.repeat?1n:0n);put(O.defaultAppearance,value.defaultAppearance??null);put(O.quadding,pdfInteger(value.quadding));break;
    case "unknown":put(O.subtype,value.subtype);put(O.entries,await writePdfDictionary(out,value.entries));break;
    default:throw new Error("Unknown PDF annotation kind");
  }
  if(points!=null){if(points.length!==4)throw new Error("PDF annotation rectangle requires four coordinates");put(O.point0,points[0]!);put(O.point1,points[1]!);put(O.point2,points[2]!);put(O.point3,points[3]!);}
  const key=await out.insert("pdf_annotation_detail",[value.kind,...fields]);
  switch(value.kind){case "link":case "highlight":case "underline":case "squiggly":case "strikeOut":case "redact":await writeReals(out,key,"quad",value.quadPoints);break;case "polygon":case "polyLine":await writeReals(out,key,"vertices",value.vertices);break;case "freeText":if(value.callout!=null)await writeReals(out,key,"callout",value.callout);break;case "ink":out.checkRowsAdditional(value.paths.length);for(const[ordinal,path]of value.paths.entries()){const owner=await out.insert("pdf_annotation_ink_path",[key,BigInt(ordinal)]);out.checkRowsAdditional(path.length);for(const[index,coordinate]of path.entries())await out.insert("pdf_annotation_ink_coordinate",[owner,BigInt(index),coordinate]);}break;}
  switch(value.kind){case "line":case "square":case "circle":case "polygon":case "polyLine":case "redact":if(value.interiorColor!=null)await writeReals(out,key,"interior",value.interiorColor);break;}
  return key;
}

async function endings(reader:PdfReader,row:SqliteRow):Promise<[string,string]|null>{const start=await reader.optionalText(row,O.endingStart);const end=await reader.optionalText(row,O.endingEnd);if(start===null&&end===null)return null;if(start===null||end===null)throw new Error("PDF line endings must both exist");return [start,end];}
function rectangle(reader:PdfReader,row:SqliteRow):[Binary64,Binary64,Binary64,Binary64]|null{const present=!reader.isNull("pdf_annotation_detail",row,O.point0);for(const column of [O.point1,O.point2,O.point3])if(reader.isNull("pdf_annotation_detail",row,column)===present)throw new Error("PDF annotation rectangle requires four coordinates");if(!present)return null;return [reader.real("pdf_annotation_detail",row,O.point0),reader.real("pdf_annotation_detail",row,O.point1),reader.real("pdf_annotation_detail",row,O.point2),reader.real("pdf_annotation_detail",row,O.point3)];}

/** 📥️ Reconstruct exact subtype ownership and reject every foreign scalar or IEEE word. */
export async function readPdfAnnotationKind(reader:PdfReader,key:bigint):Promise<PdfAnnotationKind>{
  const row=await reader.take("pdf_annotation_detail",key,41);let value:PdfAnnotationKind;let present:readonly number[];
  const optionalAction=()=>row.values[O.action]===null?null:readPdfAction(reader,artifactSqliteInteger(row,O.action));const interior=()=>readOptionalReals(reader,row,key,O.interiorPresent,"interior");const real=(column:number)=>reader.isNull("pdf_annotation_detail",row,column)?null:reader.real("pdf_annotation_detail",row,column);
  switch(artifactSqliteText(row,1)){
    case "text":value={kind:"text",open:pdfBoolean(row,O.open),icon:await reader.optionalText(row,O.icon),state:await reader.optionalText(row,O.state),stateModel:await reader.optionalText(row,O.stateModel)};present=[O.open,O.icon,O.state,O.stateModel];break;
    case "link":value={kind:"link",action:await optionalAction(),destination:row.values[O.destination]===null?null:await readPdfDestination(reader,artifactSqliteInteger(row,O.destination)),highlight:await reader.optionalText(row,O.highlight),quadPoints:await readReals(reader,key,"quad")};present=[O.action,O.destination,O.highlight];break;
    case "freeText":value={kind:"freeText",defaultAppearance:await reader.text(row,O.defaultAppearance),quadding:pdfNumber(row,O.quadding),callout:await readOptionalReals(reader,row,key,O.calloutPresent,"callout"),lineEnding:await reader.optionalText(row,O.lineEnding),richText:await reader.optionalText(row,O.richText)};present=[O.defaultAppearance,O.quadding,O.calloutPresent,O.lineEnding,O.richText];break;
    case "line":{const points=rectangle(reader,row);if(points===null)throw new Error("Missing PDF line coordinates");value={kind:"line",points,lineEndings:await endings(reader,row),interiorColor:await interior(),leaderLength:real(O.leaderLength),caption:pdfBoolean(row,O.caption)};present=[O.point0,O.point1,O.point2,O.point3,O.endingStart,O.endingEnd,O.interiorPresent,O.leaderLength,O.caption];break;}
    case "square":value={kind:"square",interiorColor:await interior(),rectDifferences:rectangle(reader,row)};present=[O.interiorPresent,O.point0,O.point1,O.point2,O.point3];break;
    case "circle":value={kind:"circle",interiorColor:await interior(),rectDifferences:rectangle(reader,row)};present=[O.interiorPresent,O.point0,O.point1,O.point2,O.point3];break;
    case "polygon":value={kind:"polygon",vertices:await readReals(reader,key,"vertices"),interiorColor:await interior()};present=[O.interiorPresent];break;
    case "polyLine":value={kind:"polyLine",vertices:await readReals(reader,key,"vertices"),lineEndings:await endings(reader,row),interiorColor:await interior()};present=[O.endingStart,O.endingEnd,O.interiorPresent];break;
    case "highlight":value={kind:"highlight",quadPoints:await readReals(reader,key,"quad")};present=[];break;
    case "underline":value={kind:"underline",quadPoints:await readReals(reader,key,"quad")};present=[];break;
    case "squiggly":value={kind:"squiggly",quadPoints:await readReals(reader,key,"quad")};present=[];break;
    case "strikeOut":value={kind:"strikeOut",quadPoints:await readReals(reader,key,"quad")};present=[];break;
    case "stamp":value={kind:"stamp",icon:await reader.optionalText(row,O.icon)};present=[O.icon];break;
    case "caret":value={kind:"caret",rectDifferences:rectangle(reader,row),symbol:await reader.optionalText(row,O.symbol)};present=[O.point0,O.point1,O.point2,O.point3,O.symbol];break;
    case "ink":{const paths:Binary64[][]=[];for(const child of await reader.children("pdf_annotation_ink_path",1,2,key)){const path=await reader.take("pdf_annotation_ink_path",child.rowid,3);const coordinates:Binary64[]=[];for(const child of await reader.children("pdf_annotation_ink_coordinate",1,2,path.rowid)){const row=await reader.take("pdf_annotation_ink_coordinate",child.rowid,4);coordinates.push(reader.real("pdf_annotation_ink_coordinate",row,3));}paths.push(coordinates);}value={kind:"ink",paths};present=[];break;}
    case "popup":value={kind:"popup",parent:readPdfWide(row,O.parentHigh),open:pdfBoolean(row,O.open)};present=[O.parentHigh,O.parentLow,O.open];break;
    case "fileAttachment":value={kind:"fileAttachment",file:await readPdfFileSpecification(reader,artifactSqliteInteger(row,O.file)),icon:await reader.optionalText(row,O.icon)};present=[O.file,O.icon];break;
    case "sound":value={kind:"sound",sound:await readPdfDictionary(reader,artifactSqliteInteger(row,O.sound)),icon:await reader.optionalText(row,O.icon)};present=[O.sound,O.icon];break;
    case "movie":value={kind:"movie",title:await reader.optionalText(row,O.title),movie:await readPdfDictionary(reader,artifactSqliteInteger(row,O.movie)),activation:row.values[O.activation]===null?null:await readPdfDictionary(reader,artifactSqliteInteger(row,O.activation))};present=[O.title,O.movie,O.activation];break;
    case "widget":value={kind:"widget",field:await reader.optionalText(row,O.field),highlight:await reader.optionalText(row,O.highlight),characteristics:await readPdfDictionary(reader,artifactSqliteInteger(row,O.characteristics)),action:await optionalAction(),additionalActions:await readPdfDictionary(reader,artifactSqliteInteger(row,O.additionalActions))};present=[O.field,O.highlight,O.characteristics,O.action,O.additionalActions];break;
    case "screen":value={kind:"screen",title:await reader.optionalText(row,O.title),characteristics:await readPdfDictionary(reader,artifactSqliteInteger(row,O.characteristics)),action:await optionalAction(),additionalActions:await readPdfDictionary(reader,artifactSqliteInteger(row,O.additionalActions))};present=[O.title,O.characteristics,O.action,O.additionalActions];break;
    case "printerMark":value={kind:"printerMark",markStyle:await reader.optionalText(row,O.markStyle),colorants:await readPdfDictionary(reader,artifactSqliteInteger(row,O.colorants))};present=[O.markStyle,O.colorants];break;
    case "trapNet":value={kind:"trapNet",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,O.entries))};present=[O.entries];break;
    case "threeD":value={kind:"threeD",entries:await readPdfDictionary(reader,artifactSqliteInteger(row,O.entries))};present=[O.entries];break;
    case "watermark":value={kind:"watermark",fixedPrint:row.values[O.fixedPrint]===null?null:await readPdfDictionary(reader,artifactSqliteInteger(row,O.fixedPrint))};present=[O.fixedPrint];break;
    case "redact":value={kind:"redact",quadPoints:await readReals(reader,key,"quad"),interiorColor:await interior(),overlayText:await reader.optionalText(row,O.overlayText),repeat:pdfBoolean(row,O.repeat),defaultAppearance:await reader.optionalText(row,O.defaultAppearance),quadding:pdfNumber(row,O.quadding)};present=[O.interiorPresent,O.overlayText,O.repeat,O.defaultAppearance,O.quadding];break;
    case "unknown":value={kind:"unknown",subtype:await reader.text(row,O.subtype),entries:await readPdfDictionary(reader,artifactSqliteInteger(row,O.entries))};present=[O.subtype,O.entries];break;
    default:throw new Error("Unknown PDF annotation kind");
  }
  reader.nullExcept("pdf_annotation_detail",row,2,41,present);return value;
}
