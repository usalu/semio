/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
/** 🎞️ Individually authored GIF89a animation and extension relationships. */
import type { GifDisposal,GifPlainText,GifSnapshot } from "../🟦️.ts";
import { ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteBoolean,artifactSqliteOrderedRows,artifactSqliteText,type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase,SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { gifInteger,gifFlag,gifUnsigned,gifEntities,gifSingletons,gifGroups,gifWritePalette,gifReadPalette,gifReadPixels } from "../../../../../../🪶️sqlite/🟦️.ts";
export const GIF89_SQLITE_SCHEMA:string=sql;
const disposalClasses=["unspecified","do_not_dispose","restore_to_background","restore_to_previous"]as const;
const disposalValues:readonly GifDisposal[]=["unspecified","doNotDispose","restoreToBackground","restoreToPrevious"];
function readOptional(row:SqliteRow,column:number,maximum:number):number|null{return row.values[column]===null?null:gifUnsigned(row,column,maximum);}
function readPlainText(row:SqliteRow|undefined):GifPlainText|null{return row?{left:gifUnsigned(row,1),top:gifUnsigned(row,2),width:gifUnsigned(row,3),height:gifUnsigned(row,4),cellWidth:gifUnsigned(row,5,255),cellHeight:gifUnsigned(row,6,255),fgColorIndex:gifUnsigned(row,7,255),bgColorIndex:gifUnsigned(row,8,255),text:artifactSqliteText(row,9)}:null;}

/** 📤️ Projects all owned animation, text and application fields without native packing. */
export async function gifSnapshotToSqliteDatabase(snapshot:GifSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(GIF89_SQLITE_SCHEMA,options);
 await out.insert("gif89_document",[snapshot.schema,gifInteger(snapshot.width),gifInteger(snapshot.height),gifInteger(snapshot.backgroundColorIndex,255),gifInteger(snapshot.pixelAspectRatio,255),snapshot.loopCount===null?null:gifInteger(snapshot.loopCount,65535)]);
 await gifWritePalette(out,"gif89_global_palette","gif89_global_color",1n,snapshot.gct);
 for(let ordinal=0;ordinal<snapshot.frames.length;ordinal++){const frame=snapshot.frames[ordinal]!;const width=gifInteger(frame.width),height=gifInteger(frame.height);const disposal=disposalValues.indexOf(frame.disposal);if(disposal<0)throw new Error("Unknown GIF89 disposal policy");
  const id=await out.insert("gif89_frame",[1n,BigInt(ordinal),gifInteger(frame.left),gifInteger(frame.top),width,height,gifFlag(frame.interlace),gifInteger(frame.delayCs,65535),disposalClasses[disposal]!,frame.transparentIndex===null?null:gifInteger(frame.transparentIndex,255),gifFlag(frame.userInput)]);
  await gifWritePalette(out,"gif89_local_palette","gif89_local_color",id,frame.lct);
  for(let position=0;position<frame.indices.length;position++)await out.insert("gif89_pixel",[id,BigInt(position),gifInteger(frame.indices[position]!,255)]);
  const text=frame.plainText;if(text!==null)await out.insert("gif89_plain_text",[gifInteger(text.left),gifInteger(text.top),gifInteger(text.width),gifInteger(text.height),gifInteger(text.cellWidth,255),gifInteger(text.cellHeight,255),gifInteger(text.fgColorIndex,255),gifInteger(text.bgColorIndex,255),text.text],id);
 }
 for(let ordinal=0;ordinal<snapshot.comments.length;ordinal++)await out.insert("gif89_comment",[1n,BigInt(ordinal),snapshot.comments[ordinal]!]);
 for(let ordinal=0;ordinal<snapshot.appExtensions.length;ordinal++){const application=snapshot.appExtensions[ordinal]!,a=application.identifier,b=application.authCode;if(a.length!==8||b.length!==3)throw new Error("GIF89 application identifiers require eight and three octets");const id=await out.insert("gif89_application",[1n,BigInt(ordinal),gifInteger(a[0],255),gifInteger(a[1],255),gifInteger(a[2],255),gifInteger(a[3],255),gifInteger(a[4],255),gifInteger(a[5],255),gifInteger(a[6],255),gifInteger(a[7],255),gifInteger(b[0],255),gifInteger(b[1],255),gifInteger(b[2],255)]);for(let ordinal=0;ordinal<application.data.length;ordinal++)await out.insert("gif89_application_byte",[id,BigInt(ordinal),gifInteger(application.data[ordinal]!,255)]);}
 return out.finish();
}

/** 📥️ Reconstructs typed frame controls and ordered extension ownership. */
export async function gifSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<GifSnapshot>{
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
 const [documents,globalRows,globalColors,frameRows,localRows,localColors,pixelRows,textRows,commentRows,applicationRows,byteRows]=await artifactSqliteTables(database,GIF89_SQLITE_SCHEMA,options);
 const document=artifactSqliteDocument(documents!);const frameIds=await gifEntities(frameRows!,options);const global=await gifSingletons(globalRows!,new Set([1n]),options);const globalEntries=await gifGroups(globalColors!,new Set(global.keys()),true,options);const local=await gifSingletons(localRows!,frameIds,options);const localEntries=await gifGroups(localColors!,new Set(local.keys()),true,options);const pixels=await gifGroups(pixelRows!,frameIds,true,options);const texts=await gifSingletons(textRows!,frameIds,options);
 const frames:GifSnapshot["frames"]=[];
 for(const row of artifactSqliteOrderedRows(frameRows!,2)){artifactSqliteDocumentReference(row,1);const width=gifUnsigned(row,5),height=gifUnsigned(row,6),disposal=disposalClasses.indexOf(artifactSqliteText(row,9)as typeof disposalClasses[number]);if(disposal<0)throw new Error("Unknown GIF89 disposal policy");frames.push({left:gifUnsigned(row,3),top:gifUnsigned(row,4),width,height,interlace:artifactSqliteBoolean(row,7),lct:await gifReadPalette(local.get(row.rowid),localEntries.get(row.rowid),options),indices:await gifReadPixels(pixels.get(row.rowid),width,height,options),delayCs:gifUnsigned(row,8,65535),disposal:disposalValues[disposal]!,transparentIndex:readOptional(row,10,255),userInput:artifactSqliteBoolean(row,11),plainText:readPlainText(texts.get(row.rowid))});if(frames.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",frames.length,frameRows!.length);}
 await gifEntities(commentRows!,options);const comments:string[]=[];for(const row of artifactSqliteOrderedRows(commentRows!,2)){artifactSqliteDocumentReference(row,1);comments.push(artifactSqliteText(row,3));if(comments.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",comments.length,commentRows!.length);}
 const applicationIds=await gifEntities(applicationRows!,options);const bytes=await gifGroups(byteRows!,applicationIds,true,options);const appExtensions:GifSnapshot["appExtensions"]=[];
 for(const row of artifactSqliteOrderedRows(applicationRows!,2)){artifactSqliteDocumentReference(row,1);const data:number[]=[];for(const value of bytes.get(row.rowid)??[]){data.push(gifUnsigned(value,3,255));if(data.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",data.length,0);}appExtensions.push({identifier:[gifUnsigned(row,3,255),gifUnsigned(row,4,255),gifUnsigned(row,5,255),gifUnsigned(row,6,255),gifUnsigned(row,7,255),gifUnsigned(row,8,255),gifUnsigned(row,9,255),gifUnsigned(row,10,255)],authCode:[gifUnsigned(row,11,255),gifUnsigned(row,12,255),gifUnsigned(row,13,255)],data});if(appExtensions.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",appExtensions.length,applicationRows!.length);}
 const result:GifSnapshot={schema:artifactSqliteText(document,1),width:gifUnsigned(document,2),height:gifUnsigned(document,3),gct:await gifReadPalette(global.get(1n),globalEntries.get(1n),options),backgroundColorIndex:gifUnsigned(document,4,255),pixelAspectRatio:gifUnsigned(document,5,255),loopCount:readOptional(document,6,65535),frames,comments,appExtensions};await artifactSqliteCheckpoint(options,"reconstructSnapshot",frameRows!.length,frameRows!.length);return result;
}
