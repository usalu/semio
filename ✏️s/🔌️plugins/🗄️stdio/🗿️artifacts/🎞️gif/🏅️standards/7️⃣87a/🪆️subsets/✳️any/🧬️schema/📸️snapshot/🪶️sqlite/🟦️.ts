/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
/** 🎞️ Individually authored GIF87a logical screen, palette and image relations. */
import type { GifSnapshot } from "../🟦️.ts";
import { ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteBoolean,artifactSqliteOrderedRows,artifactSqliteText,type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { gifInteger,gifFlag,gifUnsigned,gifEntities,gifSingletons,gifGroups,gifWritePalette,gifReadPalette,gifReadPixels } from "../../../../../../🪶️sqlite/🟦️.ts";
export const GIF87_SQLITE_SCHEMA:string=sql;

/** 📤️ Projects exact owned GIF87a state without native image encoding. */
export async function gifSnapshotToSqliteDatabase(snapshot:GifSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(GIF87_SQLITE_SCHEMA,options);
 await out.insert("gif87_document",[snapshot.schema,gifInteger(snapshot.width),gifInteger(snapshot.height),gifInteger(snapshot.backgroundColorIndex,255),gifInteger(snapshot.pixelAspectRatio,255)]);
 await gifWritePalette(out,"gif87_global_palette","gif87_global_color",1n,snapshot.gct);
 for(let ordinal=0;ordinal<snapshot.images.length;ordinal++){const image=snapshot.images[ordinal]!;const width=gifInteger(image.width),height=gifInteger(image.height);
  const id=await out.insert("gif87_image",[1n,BigInt(ordinal),gifInteger(image.left),gifInteger(image.top),width,height,gifFlag(image.interlace)]);
  await gifWritePalette(out,"gif87_local_palette","gif87_local_color",id,image.lct);
  for(let position=0;position<image.indices.length;position++)await out.insert("gif87_pixel",[id,BigInt(position),gifInteger(image.indices[position]!,255)]);
 }return out.finish();
}

/** 📥️ Reconstructs indexed images with explicit palette presence and unique grids. */
export async function gifSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<GifSnapshot>{
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
 const [documents,globalRows,globalColors,imageRows,localRows,localColors,pixelRows]=await artifactSqliteTables(database,GIF87_SQLITE_SCHEMA,options);
 const document=artifactSqliteDocument(documents!);const imageIds=await gifEntities(imageRows!,options);const global=await gifSingletons(globalRows!,new Set([1n]),options);const globalEntries=await gifGroups(globalColors!,new Set(global.keys()),true,options);const local=await gifSingletons(localRows!,imageIds,options);const localEntries=await gifGroups(localColors!,new Set(local.keys()),true,options);const pixels=await gifGroups(pixelRows!,imageIds,true,options);
 const images:GifSnapshot["images"]=[];
 for(const row of artifactSqliteOrderedRows(imageRows!,2)){artifactSqliteDocumentReference(row,1);const width=gifUnsigned(row,5),height=gifUnsigned(row,6);images.push({left:gifUnsigned(row,3),top:gifUnsigned(row,4),width,height,interlace:artifactSqliteBoolean(row,7),lct:await gifReadPalette(local.get(row.rowid),localEntries.get(row.rowid),options),indices:await gifReadPixels(pixels.get(row.rowid),width,height,options)});if(images.length%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",images.length,imageRows!.length);}
 const result:GifSnapshot={schema:artifactSqliteText(document,1),width:gifUnsigned(document,2),height:gifUnsigned(document,3),gct:await gifReadPalette(global.get(1n),globalEntries.get(1n),options),backgroundColorIndex:gifUnsigned(document,4,255),pixelAspectRatio:gifUnsigned(document,5,255),images};await artifactSqliteCheckpoint(options,"reconstructSnapshot",imageRows!.length,imageRows!.length);return result;
}
