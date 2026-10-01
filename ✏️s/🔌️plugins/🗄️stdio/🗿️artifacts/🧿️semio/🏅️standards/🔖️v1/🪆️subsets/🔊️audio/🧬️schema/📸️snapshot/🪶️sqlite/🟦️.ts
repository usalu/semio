/** 🔊️ Exact native binary32 samples beside independently queryable channel/tag entities. */
import type {SemioAudioSnapshot,SemioAudioFormat,SemioAudioChannel,SemioAudioTag} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {binary32Value,parseBinary32,type Binary32} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_AUDIO_SQLITE_SCHEMA=`CREATE TABLE semio_audio_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, sample_rate INTEGER NOT NULL CHECK (sample_rate BETWEEN 0 AND 4294967295), sample_format TEXT NOT NULL CHECK (sample_format IN ('pcm8','pcm16','pcm24','pcm32','f32','f64')));
CREATE TABLE semio_audio_channel (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_audio_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0));
CREATE TABLE semio_audio_sample (id INTEGER PRIMARY KEY, channel_id INTEGER NOT NULL REFERENCES semio_audio_channel(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value REAL, ieee754_bits INTEGER NOT NULL CHECK (ieee754_bits BETWEEN 0 AND 4294967295));
CREATE TABLE semio_audio_tag (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_audio_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), tag_key TEXT NOT NULL, tag_value TEXT NOT NULL);
`;
function format(value:string):SemioAudioFormat{switch(value){case "pcm8":case "pcm16":case "pcm24":case "pcm32":case "f32":case "f64":return value;default:throw Error("unknown Semio audio format");}}
function u32(value:number):bigint{if(!Number.isInteger(value)||value<0||value>4294967295)throw Error("Semio audio scalar exceeds unsigned32");return BigInt(value);}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio audio entity identity or columns");}
function sample(row:SqliteRow):Binary32{const bits=artifactSqliteInteger(row,4);if(bits<0n||bits>4294967295n)throw Error("invalid Semio audio sample bit width");const word={bits:Number(bits)},expected=binary32Value(word),stored=row.values[3];if(Number.isNaN(expected)){if(stored!==null)throw Error("Semio audio NaN requires NULL query scalar");}else if(typeof stored!=="number"||stored!==expected)throw Error("Semio audio query scalar disagrees with IEEE bits");return word;}

/** 📤️ Preserve all owned sample words including signaling NaN payloads. */
export async function semioAudioSnapshotToSqliteDatabase(snapshot:SemioAudioSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  const p=await ArtifactSqliteProjection.create(SEMIO_AUDIO_SQLITE_SCHEMA,options);await p.insert("semio_audio_document",[snapshot.schema,u32(snapshot.sampleRate),format(snapshot.format)],1n);
  let units=0;for(let ordinal=0;ordinal<snapshot.channels.length;ordinal++){const channel=snapshot.channels[ordinal]!,id=await p.insert("semio_audio_channel",[1n,BigInt(ordinal)]);for(let index=0;index<channel.samples.length;index++){const word=parseBinary32(channel.samples[index]),value=binary32Value(word);await p.insert("semio_audio_sample",[id,BigInt(index),Number.isNaN(value)?null:value,BigInt(word.bits)]);if(++units%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",units,0);}}
  for(let ordinal=0;ordinal<snapshot.tags.length;ordinal++){const tag=snapshot.tags[ordinal]!;await p.insert("semio_audio_tag",[1n,BigInt(ordinal),tag.key,tag.value]);}return p.finish();
}

/** 📥️ Require consistent sample bit/query pairs and complete ordered ownership. */
export async function semioAudioSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioAudioSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const[documents,channels,samples,tags]=await artifactSqliteTables(database,SEMIO_AUDIO_SQLITE_SCHEMA,options),document=artifactSqliteDocument(documents!);identity(document,4);const rate=artifactSqliteInteger(document,2);if(rate<0n||rate>4294967295n)throw Error("invalid Semio audio sample rate");
  const ordered=await artifactSqliteOrderedRowsControlled(channels!,2,options),groups=new Map<bigint,SqliteRow[]>();let units=0;
  for(const channel of ordered){identity(channel,3);artifactSqliteDocumentReference(channel,1);if(groups.has(channel.rowid))throw Error("duplicate Semio audio channel");groups.set(channel.rowid,[]);if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  const seen=new Set<bigint>();for(const row of samples!){identity(row,5);const group=groups.get(artifactSqliteInteger(row,1));if(!group||seen.has(row.rowid))throw Error("invalid Semio audio sample ownership");seen.add(row.rowid);group.push(row);if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  const out:SemioAudioChannel[]=[];for(const channel of ordered){const values:Binary32[]=[];for(const row of await artifactSqliteOrderedRowsControlled(groups.get(channel.rowid)!,2,options)){values.push(sample(row));if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}out.push({samples:values});}
  seen.clear();const entries:SemioAudioTag[]=[];for(const row of await artifactSqliteOrderedRowsControlled(tags!,2,options)){identity(row,5);artifactSqliteDocumentReference(row,1);if(seen.has(row.rowid))throw Error("duplicate Semio audio tag");seen.add(row.rowid);entries.push({key:artifactSqliteText(row,3),value:artifactSqliteText(row,4)});if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  return{schema:artifactSqliteText(document,1),sampleRate:Number(rate),format:format(artifactSqliteText(document,3)),channels:out,tags:entries};
}
