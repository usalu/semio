/** 🎨️ Ordered RGBA8 channel occurrences with exact intermediate owner cardinality. */
import type {SemioImageSnapshot,SemioColorspace,SemioImageFrame,SemioImageMetadataEntry} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,ArtifactSqliteRowIndex,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteValueBudget,artifactSqliteTextBytes,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {sqliteOperation,type SqliteDatabase,type SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_IMAGE_SQLITE_SCHEMA=`CREATE TABLE semio_image_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295), colorspace TEXT NOT NULL CHECK (colorspace IN ('rgb','rgba','grayscale','grayscale_alpha','indexed')), bit_depth INTEGER NOT NULL CHECK (bit_depth BETWEEN 0 AND 255), icc BLOB);
CREATE TABLE semio_image_frame (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_image_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), delay_ms INTEGER NOT NULL CHECK (delay_ms BETWEEN 0 AND 4294967295));
CREATE TABLE semio_image_sample (id INTEGER PRIMARY KEY, frame_id INTEGER NOT NULL REFERENCES semio_image_frame(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), channel INTEGER NOT NULL CHECK (channel BETWEEN 0 AND 3), sample INTEGER NOT NULL CHECK (sample BETWEEN 0 AND 255));
CREATE TABLE semio_image_metadata (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_image_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), metadata_key TEXT NOT NULL, metadata_value TEXT NOT NULL);
`;
function bounded(value:number,max:number):bigint{if(!Number.isInteger(value)||value<0||value>max)throw Error("Semio image scalar exceeds its native width");return BigInt(value);}
function scalar(row:SqliteRow,index:number,max:number):number{const value=artifactSqliteInteger(row,index);if(value<0n||value>BigInt(max))throw Error("Semio image scalar exceeds its native width");return Number(value);}
function space(value:string):SemioColorspace{switch(value){case "rgb":case "rgba":case "grayscale":case "indexed":return value;case "grayscale_alpha":return "grayscaleAlpha";default:throw Error("unknown Semio image colorspace");}}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio image identity or column shape");}
async function bytes(input:readonly number[],options:ArtifactSqliteOptions):Promise<Uint8Array>{artifactSqliteValueBudget(input.length,options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,input.length);const result=new Uint8Array(input.length);for(let i=0;i<input.length;i++){result[i]=Number(bounded(input[i]!,255));if((i+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i+1,input.length);}return result;}
async function profile(row:SqliteRow,options:ArtifactSqliteOptions):Promise<number[]>{const value=row.values[6];if(!(value instanceof Uint8Array))throw Error("Semio image ICC requires BLOB storage");await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,value.length);const out:number[]=[];for(let i=0;i<value.length;i++){out.push(value[i]!);if((i+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i+1,value.length);}return out;}

/** 📤️ Materialize each retained channel octet without imposing renderer dimensions. */
export async function semioImageSnapshotToSqliteDatabase(snapshot:SemioImageSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 options=sqliteOperation(options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
 let count=1+snapshot.frames.length+snapshot.metadata.length;
 for(let i=0;i<snapshot.frames.length;i++){count+=snapshot.frames[i]!.rgba8.length;if(!Number.isSafeInteger(count)||count>(options.maxRows??1_000_000))throw Error("Artifact SQLite row limit");if((i+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i+1,snapshot.frames.length);}
 if(!Number.isSafeInteger(count)||count>(options.maxRows??1_000_000))throw Error("Artifact SQLite row limit");
 const colorspace=snapshot.colorspace==="grayscaleAlpha"?"grayscale_alpha":snapshot.colorspace;space(colorspace);
 const p=await ArtifactSqliteProjection.create(SEMIO_IMAGE_SQLITE_SCHEMA,options);
 await p.insert("semio_image_document",[snapshot.schema,bounded(snapshot.width,4294967295),bounded(snapshot.height,4294967295),colorspace,bounded(snapshot.bitDepth,255),snapshot.icc===null?null:await bytes(snapshot.icc,options)],1n);
 for(let i=0;i<snapshot.frames.length;i++){
  const frame=snapshot.frames[i]!,id=await p.insert("semio_image_frame",[1n,BigInt(i),bounded(frame.delayMs,4294967295)]);
  for(let ordinal=0;ordinal<frame.rgba8.length;ordinal++)await p.insert("semio_image_sample",[id,BigInt(ordinal),BigInt(ordinal%4),bounded(frame.rgba8[ordinal]!,255)]);
 }
 for(let i=0;i<snapshot.metadata.length;i++){const entry=snapshot.metadata[i]!;await p.insert("semio_image_metadata",[1n,BigInt(i),entry.key,entry.value]);}
 return p.finish();
}

/** 📥️ Consume every frame, ordered sample and metadata relationship exactly once. */
export async function semioImageSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioImageSnapshot>{
 options=sqliteOperation(options);await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
 const[documents]=await artifactSqliteTables(database,SEMIO_IMAGE_SQLITE_SCHEMA,options),document=artifactSqliteDocument(documents!);identity(document,7);
 const index=await ArtifactSqliteRowIndex.create(database.tables,options);await index.take(document);
 const width=scalar(document,2,4294967295),height=scalar(document,3,4294967295),colorspace=space(artifactSqliteText(document,4)),bitDepth=scalar(document,5,255),icc=document.values[6]===null?null:await profile(document,options);
 const frames:SemioImageFrame[]=[],metadata:SemioImageMetadataEntry[]=[];
 for(const row of await index.grouped("semio_image_frame",1n)){
  identity(row,4);artifactSqliteDocumentReference(row,1);await index.take(row);
  const rgba8:number[]=[];
  for(const sample of await index.grouped("semio_image_sample",row.rowid)){
   identity(sample,5);if(artifactSqliteInteger(sample,1)!==row.rowid||scalar(sample,3,3)!==rgba8.length%4)throw Error("invalid Semio image sample parent or channel");
   rgba8.push(scalar(sample,4,255));await index.take(sample);
  }
  frames.push({delayMs:scalar(row,3,4294967295),rgba8});
 }
 for(const row of await index.grouped("semio_image_metadata",1n)){identity(row,5);artifactSqliteDocumentReference(row,1);await index.take(row);metadata.push({key:artifactSqliteText(row,3),value:artifactSqliteText(row,4)});}
 await index.finish();return{schema:artifactSqliteText(document,1),width,height,colorspace,bitDepth,icc,frames,metadata};
}
