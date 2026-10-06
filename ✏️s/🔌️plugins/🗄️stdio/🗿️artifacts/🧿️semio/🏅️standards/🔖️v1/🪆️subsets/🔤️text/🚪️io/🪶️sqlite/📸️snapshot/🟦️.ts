/** 🔤️ Semio text's authored run and inline mark relationships. */
import type {SemioTextSnapshot,SemioTextMark as Mark,SemioTextMarkKind as MarkKind,SemioTextRun as Run} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

export const SEMIO_TEXT_SQLITE_SCHEMA=`CREATE TABLE semio_text_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE semio_text_run (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_text_document(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), language TEXT NOT NULL, content TEXT NOT NULL);
CREATE TABLE semio_text_mark (id INTEGER PRIMARY KEY, run_id INTEGER NOT NULL REFERENCES semio_text_run(id), ordinal INTEGER NOT NULL CHECK(ordinal >= 0), kind TEXT NOT NULL CHECK(kind IN ('bold','italic','code','link')), href TEXT NOT NULL);
`;
function markKind(value:string):MarkKind{switch(value){case "bold":case "italic":case "code":case "link":return value;default:throw Error("unknown Semio text mark kind");}}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio text entity identity or column count");}

/** 📤️ Preserve text, language and every ordered inline mark as queryable entities. */
export async function semioTextSnapshotToSqliteDatabase(snapshot:SemioTextSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  const projection=await ArtifactSqliteProjection.create(SEMIO_TEXT_SQLITE_SCHEMA,options);
  await projection.insert("semio_text_document",[snapshot.schema],1n);
  let units=0;
  for(let ordinal=0;ordinal<snapshot.runs.length;ordinal++){
    const run=snapshot.runs[ordinal]!;
    const id=await projection.insert("semio_text_run",[1n,BigInt(ordinal),run.language,run.content]);
    for(let index=0;index<run.marks.length;index++){const mark=run.marks[index]!;await projection.insert("semio_text_mark",[id,BigInt(index),markKind(mark.kind),mark.href]);if(++units%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",units,0);}
  }
  return projection.finish();
}

/** 📥️ Require complete ordered ownership before reconstructing the native text model. */
export async function semioTextSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioTextSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
  const [documents,runs,marks]=await artifactSqliteTables(database,SEMIO_TEXT_SQLITE_SCHEMA,options);
  const document=artifactSqliteDocument(documents!);identity(document,2);
  const ordered=await artifactSqliteOrderedRowsControlled(runs!,2,options),groups=new Map<bigint,SqliteRow[]>();
  for(let index=0;index<ordered.length;index++){const run=ordered[index]!;identity(run,5);artifactSqliteDocumentReference(run,1);if(groups.has(run.rowid))throw Error("duplicate Semio text run identity");groups.set(run.rowid,[]);if(index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index,ordered.length);}
  const seen=new Set<bigint>();
  for(let index=0;index<marks!.length;index++){const mark=marks![index]!;identity(mark,5);if(seen.has(mark.rowid))throw Error("duplicate Semio text mark identity");seen.add(mark.rowid);const group=groups.get(artifactSqliteInteger(mark,1));if(!group)throw Error("unknown Semio text mark owner");group.push(mark);if(index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index,marks!.length);}
  const result:Run[]=[];
  for(let index=0;index<ordered.length;index++){
    const run=ordered[index]!,inline:Mark[]=[];
    for(const mark of await artifactSqliteOrderedRowsControlled(groups.get(run.rowid)!,2,options))inline.push({kind:markKind(artifactSqliteText(mark,3)),href:artifactSqliteText(mark,4)});
    result.push({language:artifactSqliteText(run,3),content:artifactSqliteText(run,4),marks:inline});
    if(index%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index,ordered.length);
  }
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",result.length,result.length);
  return{schema:artifactSqliteText(document,1),runs:result};
}
