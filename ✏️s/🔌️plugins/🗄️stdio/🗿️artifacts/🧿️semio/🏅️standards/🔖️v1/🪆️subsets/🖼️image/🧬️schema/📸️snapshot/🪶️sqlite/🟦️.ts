/** 🖼️ Queryable native image dimensions, ordered frames and metadata. */
import type {SemioImageSnapshot,SemioColorspace,SemioImageFrame,SemioImageMetadataEntry} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,artifactSqliteValueBudget,artifactSqliteTextBytes,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_IMAGE_SQLITE_SCHEMA=`CREATE TABLE semio_image_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295), colorspace TEXT NOT NULL CHECK (colorspace IN ('rgb','rgba','grayscale','grayscale_alpha','indexed')), bit_depth INTEGER NOT NULL CHECK (bit_depth BETWEEN 0 AND 255), icc BLOB);
CREATE TABLE semio_image_frame (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_image_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), delay_ms INTEGER NOT NULL CHECK (delay_ms BETWEEN 0 AND 4294967295), rgba8 BLOB NOT NULL);
CREATE TABLE semio_image_metadata (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_image_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), metadata_key TEXT NOT NULL, metadata_value TEXT NOT NULL);
`;
function bounded(value:number,max:number):bigint{if(!Number.isInteger(value)||value<0||value>max)throw Error("Semio image scalar exceeds its native width");return BigInt(value);}
function scalar(row:SqliteRow,index:number,max:number):number{const value=artifactSqliteInteger(row,index);if(value<0n||value>BigInt(max))throw Error("Semio image scalar exceeds its native width");return Number(value);}
function space(value:string):SemioColorspace{switch(value){case "rgb":case "rgba":case "grayscale":case "indexed":return value;case "grayscale_alpha":return "grayscaleAlpha";default:throw Error("unknown Semio image colorspace");}}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio image identity or column shape");}
async function bytes(input:readonly number[],options:ArtifactSqliteOptions):Promise<Uint8Array>{artifactSqliteValueBudget(input.length,options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,input.length);const result=new Uint8Array(input.length);for(let i=0;i<input.length;i++){result[i]=Number(bounded(input[i]!,255));if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,input.length);}return result;}
async function pixels(row:SqliteRow,index:number,options:ArtifactSqliteOptions):Promise<number[]>{const value=row.values[index];if(!(value instanceof Uint8Array))throw Error("Semio image pixels require BLOB storage");await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,value.length);const out:number[]=[];for(let i=0;i<value.length;i++){out.push(value[i]!);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,value.length);}return out;}

/** 📤️ Preserve all bytes and nullable profile presence without native wire encoding. */
export async function semioImageSnapshotToSqliteDatabase(snapshot:SemioImageSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  const count=1+snapshot.frames.length+snapshot.metadata.length;if(count>(options.maxRows??1_000_000))throw Error("Artifact SQLite row limit");
  const colorspace=snapshot.colorspace==="grayscaleAlpha"?"grayscale_alpha":snapshot.colorspace;space(colorspace);
  const size=BigInt(bounded(snapshot.width,4294967295))*BigInt(bounded(snapshot.height,4294967295))*4n;
  let storage=32+artifactSqliteTextBytes(snapshot.schema)+artifactSqliteTextBytes(colorspace)+(snapshot.icc?.length??0);
  for(let i=0;i<snapshot.frames.length;i++){const frame=snapshot.frames[i]!;if(BigInt(frame.rgba8.length)!==size)throw Error("Semio image RGBA8 size disagrees with dimensions");storage+=32+frame.rgba8.length;artifactSqliteValueBudget(storage,options);if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,count);}
  for(let i=0;i<snapshot.metadata.length;i++){const entry=snapshot.metadata[i]!;storage+=24+artifactSqliteTextBytes(entry.key)+artifactSqliteTextBytes(entry.value);artifactSqliteValueBudget(storage,options);if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,count);}
  artifactSqliteValueBudget(storage,options);
  const p=await ArtifactSqliteProjection.create(SEMIO_IMAGE_SQLITE_SCHEMA,options);
  await p.insert("semio_image_document",[snapshot.schema,bounded(snapshot.width,4294967295),bounded(snapshot.height,4294967295),colorspace,bounded(snapshot.bitDepth,255),snapshot.icc===null?null:await bytes(snapshot.icc,options)],1n);
  for(let i=0;i<snapshot.frames.length;i++){const frame=snapshot.frames[i]!;await p.insert("semio_image_frame",[1n,BigInt(i),bounded(frame.delayMs,4294967295),await bytes(frame.rgba8,options)]);}
  for(let i=0;i<snapshot.metadata.length;i++){const entry=snapshot.metadata[i]!;await p.insert("semio_image_metadata",[1n,BigInt(i),entry.key,entry.value]);}
  return p.finish();
}

/** 📥️ Validate native widths, dimensions and relationship order before owned reconstruction. */
export async function semioImageSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioImageSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const[documents,frames,metadata]=await artifactSqliteTables(database,SEMIO_IMAGE_SQLITE_SCHEMA,options),document=artifactSqliteDocument(documents!);identity(document,7);
  const width=scalar(document,2,4294967295),height=scalar(document,3,4294967295),size=BigInt(width)*BigInt(height)*4n,colorspace=space(artifactSqliteText(document,4)),bitDepth=scalar(document,5,255);
  const icc=document.values[6]===null?null:await pixels(document,6,options),output:SemioImageFrame[]=[],entries:SemioImageMetadataEntry[]=[],seen=new Set<bigint>();
  for(const row of await artifactSqliteOrderedRowsControlled(frames!,2,options)){identity(row,5);artifactSqliteDocumentReference(row,1);if(seen.has(row.rowid))throw Error("duplicate Semio image frame");seen.add(row.rowid);const value=row.values[4];if(!(value instanceof Uint8Array)||BigInt(value.length)!==size)throw Error("Semio image RGBA8 size disagrees with dimensions");output.push({delayMs:scalar(row,3,4294967295),rgba8:await pixels(row,4,options)});}
  seen.clear();let units=0;for(const row of await artifactSqliteOrderedRowsControlled(metadata!,2,options)){identity(row,5);artifactSqliteDocumentReference(row,1);if(seen.has(row.rowid))throw Error("duplicate Semio image metadata");seen.add(row.rowid);entries.push({key:artifactSqliteText(row,3),value:artifactSqliteText(row,4)});if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,metadata!.length);}
  return{schema:artifactSqliteText(document,1),width,height,colorspace,bitDepth,icc,frames:output,metadata:entries};
}
