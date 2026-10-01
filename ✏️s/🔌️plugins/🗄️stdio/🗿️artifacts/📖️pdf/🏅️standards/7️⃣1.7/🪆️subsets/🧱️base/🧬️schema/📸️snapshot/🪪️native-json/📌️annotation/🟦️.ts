/** 📌️ Explicit native annotation admission into the owned scalar model. */
import type { PdfAnnotation,PdfAnnotationKind,PdfMarkupAnnotation,PdfAppearance,PdfAppearanceEntry,PdfAppearanceState,PdfBorderStyle,PdfDate } from "../../🟦️.ts";
import { record,array,text,boolean,integer,real,reals,four,pdfDictionaryFromNativeJson } from "../🟦️.ts";
import { pdfActionFromNativeJson,pdfDestinationFromNativeJson,pdfFileFromNativeJson } from "../🎯️navigation/🟦️.ts";

const optionalText=(value:unknown)=>value==null?null:text(value);
const optionalReals=(value:unknown)=>value==null?null:reals(value);
const optionalFour=(value:unknown)=>value==null?null:four(value);
function wide(value:unknown):bigint|null{if(value==null)return null;const result=integer(value);if(result<0)throw new Error("PDF annotation index must be unsigned");return BigInt(result);}
function endings(value:unknown):[string,string]|null{if(value==null)return null;const parts=array(value);if(parts.length!==2)throw new Error("PDF line endings require two entries");return [text(parts[0]),text(parts[1])];}
/** 📆️ Admit native date fields without narrowing their signed or unsigned widths. */
export function pdfDateFromNativeJson(input:unknown):PdfDate{const row=record(input);return {year:integer(row.year),month:integer(row.month??1),day:integer(row.day??1),hour:integer(row.hour??0),minute:integer(row.minute??0),second:integer(row.second??0),offsetMinutes:row.offsetMinutes==null?null:integer(row.offsetMinutes)};}
/** 🎭️ Admit an explicitly named appearance state. */
export function pdfAppearanceStateFromNativeJson(input:unknown):PdfAppearanceState{const row=record(input);return {state:text(row.state),form:text(row.form)};}
/** 🎭️ Admit single forms and ordered appearance state entries. */
export function pdfAppearanceEntryFromNativeJson(input:unknown):PdfAppearanceEntry{const row=record(input);switch(text(row.kind)){case "single":return {kind:"single",form:text(row.form)};case "states":return {kind:"states",states:array(row.states).map(pdfAppearanceStateFromNativeJson)};default:throw new Error("Unknown PDF appearance entry");}}
/** 🎭️ Admit the complete normal, rollover and down appearance ownership. */
export function pdfAppearanceFromNativeJson(input:unknown):PdfAppearance{const row=record(input);return {normal:pdfAppearanceEntryFromNativeJson(row.normal),rollover:row.rollover==null?null:pdfAppearanceEntryFromNativeJson(row.rollover),down:row.down==null?null:pdfAppearanceEntryFromNativeJson(row.down)};}
/** 🖊️ Admit border coordinates and optional dash/radii distinctions. */
export function pdfBorderFromNativeJson(input:unknown):PdfBorderStyle{const row=record(input);const radii=row.radii==null?null:reals(row.radii);if(radii!==null&&radii.length!==2)throw new Error("PDF border requires two radii");return {width:real(row.width),style:optionalText(row.style),dash:optionalReals(row.dash),radii:radii===null?null:[radii[0]!,radii[1]!]};}
/** 💬️ Admit all markup fields and owned wide annotation references. */
export function pdfMarkupFromNativeJson(input:unknown):PdfMarkupAnnotation{const row=record(input);return {title:optionalText(row.title),popup:wide(row.popup),opacity:row.opacity==null?null:real(row.opacity),richContents:optionalText(row.richContents),creationDate:row.creationDate==null?null:pdfDateFromNativeJson(row.creationDate),inReplyTo:wide(row.inReplyTo),subject:optionalText(row.subject),replyType:optionalText(row.replyType),intent:optionalText(row.intent)};}
/** 🏷️ Admit every explicitly authored annotation subtype. */
export function pdfAnnotationKindFromNativeJson(input:unknown):PdfAnnotationKind{
  const row=record(input);switch(text(row.kind)){
    case "text":return {kind:"text",open:boolean(row.open),icon:optionalText(row.icon),state:optionalText(row.state),stateModel:optionalText(row.stateModel)};
    case "link":return {kind:"link",action:row.action==null?null:pdfActionFromNativeJson(row.action),destination:row.destination==null?null:pdfDestinationFromNativeJson(row.destination),highlight:optionalText(row.highlight),quadPoints:reals(row.quadPoints)};
    case "freeText":return {kind:"freeText",defaultAppearance:text(row.defaultAppearance),quadding:integer(row.quadding),callout:optionalReals(row.callout),lineEnding:optionalText(row.lineEnding),richText:optionalText(row.richText)};
    case "line":return {kind:"line",points:four(row.points),lineEndings:endings(row.lineEndings),interiorColor:optionalReals(row.interiorColor),leaderLength:row.leaderLength==null?null:real(row.leaderLength),caption:boolean(row.caption)};
    case "square":return {kind:"square",interiorColor:optionalReals(row.interiorColor),rectDifferences:optionalFour(row.rectDifferences)};
    case "circle":return {kind:"circle",interiorColor:optionalReals(row.interiorColor),rectDifferences:optionalFour(row.rectDifferences)};
    case "polygon":return {kind:"polygon",vertices:reals(row.vertices),interiorColor:optionalReals(row.interiorColor)};
    case "polyLine":return {kind:"polyLine",vertices:reals(row.vertices),lineEndings:endings(row.lineEndings),interiorColor:optionalReals(row.interiorColor)};
    case "highlight":return {kind:"highlight",quadPoints:reals(row.quadPoints)};
    case "underline":return {kind:"underline",quadPoints:reals(row.quadPoints)};
    case "squiggly":return {kind:"squiggly",quadPoints:reals(row.quadPoints)};
    case "strikeOut":return {kind:"strikeOut",quadPoints:reals(row.quadPoints)};
    case "stamp":return {kind:"stamp",icon:optionalText(row.icon)};
    case "caret":return {kind:"caret",rectDifferences:optionalFour(row.rectDifferences),symbol:optionalText(row.symbol)};
    case "ink":return {kind:"ink",paths:array(row.paths).map(reals)};
    case "popup":return {kind:"popup",parent:wide(row.parent),open:boolean(row.open)};
    case "fileAttachment":return {kind:"fileAttachment",file:pdfFileFromNativeJson(row.file),icon:optionalText(row.icon)};
    case "sound":return {kind:"sound",sound:pdfDictionaryFromNativeJson(row.sound),icon:optionalText(row.icon)};
    case "movie":return {kind:"movie",title:optionalText(row.title),movie:pdfDictionaryFromNativeJson(row.movie),activation:row.activation==null?null:pdfDictionaryFromNativeJson(row.activation)};
    case "widget":return {kind:"widget",field:optionalText(row.field),highlight:optionalText(row.highlight),characteristics:pdfDictionaryFromNativeJson(row.characteristics),action:row.action==null?null:pdfActionFromNativeJson(row.action),additionalActions:pdfDictionaryFromNativeJson(row.additionalActions)};
    case "screen":return {kind:"screen",title:optionalText(row.title),characteristics:pdfDictionaryFromNativeJson(row.characteristics),action:row.action==null?null:pdfActionFromNativeJson(row.action),additionalActions:pdfDictionaryFromNativeJson(row.additionalActions)};
    case "printerMark":return {kind:"printerMark",markStyle:optionalText(row.markStyle),colorants:pdfDictionaryFromNativeJson(row.colorants)};
    case "trapNet":return {kind:"trapNet",entries:pdfDictionaryFromNativeJson(row.entries)};
    case "watermark":return {kind:"watermark",fixedPrint:row.fixedPrint==null?null:pdfDictionaryFromNativeJson(row.fixedPrint)};
    case "threeD":return {kind:"threeD",entries:pdfDictionaryFromNativeJson(row.entries)};
    case "redact":return {kind:"redact",quadPoints:reals(row.quadPoints),interiorColor:optionalReals(row.interiorColor),overlayText:optionalText(row.overlayText),repeat:boolean(row.repeat),defaultAppearance:optionalText(row.defaultAppearance),quadding:integer(row.quadding)};
    case "unknown":return {kind:"unknown",subtype:text(row.subtype),entries:pdfDictionaryFromNativeJson(row.entries)};
    default:throw new Error("Unknown PDF annotation subtype");
  }
}
/** 📌️ Admit the complete common annotation domain and nested ownership. */
export function pdfAnnotationFromNativeJson(input:unknown):PdfAnnotation{const row=record(input);return {rect:four(row.rect),kind:pdfAnnotationKindFromNativeJson(row.kind),contents:optionalText(row.contents),name:optionalText(row.name),modified:optionalText(row.modified),flags:integer(row.flags??0),border:row.border==null?null:pdfBorderFromNativeJson(row.border),color:reals(row.color??[]),appearance:row.appearance==null?null:pdfAppearanceFromNativeJson(row.appearance),appearanceState:optionalText(row.appearanceState),markup:row.markup==null?null:pdfMarkupFromNativeJson(row.markup),optionalContent:optionalText(row.optionalContent),structParent:row.structParent==null?null:integer(row.structParent),extra:pdfDictionaryFromNativeJson(row.extra??[])};}
