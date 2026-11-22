/** 🔌️ Three handwritten Jack tables preserve the manifest id and exact parent state. */
import type {JackArtifact} from "../../../🧬️schema/🟦️.ts";
import type { ArtifactDialect } from "@semio-tech/framework/schema/artifact-reference";
import type {SqliteDatabase,SqliteRow} from "@semio-tech/framework/io/sqlite-snapshot";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger as integer,artifactSqliteText as text,type ArtifactSqliteOptions} from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import {encodeIeee754Cells,readBinary64} from "@semio-tech/framework/io/sqlite-snapshot/ieee754";
export const JACK_SQLITE_SCHEMA=String.raw`CREATE TABLE jack_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL,
 name TEXT NOT NULL,
 manifest_id TEXT NOT NULL,
 root_node_id TEXT,
 query TEXT NOT NULL
);
CREATE TABLE jack_camera (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 x REAL,
 y REAL,
 zoom REAL,
 x_ieee754_bits INTEGER NOT NULL,
 x_numeric_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL,
 y_numeric_class TEXT NOT NULL,
 zoom_ieee754_bits INTEGER NOT NULL,
 zoom_numeric_class TEXT NOT NULL
);
CREATE TABLE jack_content_child (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 child_id TEXT NOT NULL,
 artifact_id TEXT NOT NULL,
 artifact_kind TEXT NOT NULL,
 standard TEXT NOT NULL,
 subset TEXT NOT NULL
);
`;
const WIDTHS=[6,11,7];
const FLOATS=[{index:2,width:64},{index:3,width:64},{index:4,width:64}]as const;
async function count(p:ArtifactSqliteProjection,options:ArtifactSqliteOptions):Promise<void>{p.checkRowsAdditional(3);await artifactSqliteCheckpoint(options,"projectSnapshot",3,3);}
/** 📤️ Forecast every literal domain entity before creating its owned relational cells. */
export async function jackSnapshotToSqliteDatabase(snapshot:JackArtifact,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(JACK_SQLITE_SCHEMA,options);await count(p,options);
 const document=await p.insert("jack_document",[snapshot.schema,snapshot.name,snapshot.manifestId,snapshot.rootNodeId??null,snapshot.query]);
 await p.insert("jack_camera",encodeIeee754Cells([1n,document,snapshot.camera.x,snapshot.camera.y,snapshot.camera.zoom],FLOATS,options.maxColumns).slice(1));
 const c=snapshot.content;await p.insert("jack_content_child",[document,c.childId,c.target.artifactId,c.target.dialect.artifactKind,c.target.dialect.standard,c.target.dialect.subset]);
 return p.finish();
}
/** 📥️ Consume each entity once through genuine owner links. */
export async function jackSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<JackArtifact>{
 const tables=await artifactSqliteTables(database,JACK_SQLITE_SCHEMA,options);let work=0;
 for(let table=0;table<tables.length;table++){const seen=new Set<bigint>();for(const row of tables[table]!){if(row.values.length!==WIDTHS[table]||integer(row,0)!==row.rowid||seen.has(row.rowid))throw Error("Jack row identity differs");seen.add(row.rowid);if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,0);}}
 const fixed=(table:number)=>{if(tables[table]!.length!==1)throw Error("Jack singleton differs");return tables[table]![0]!;};
 const document=fixed(0),camera=fixed(1),child=fixed(2);if(integer(camera,1)!==document.rowid||integer(child,1)!==document.rowid)throw Error("Jack document parent differs");
 const nullable=(row:SqliteRow,column:number)=>row.values[column]===null?undefined:text(row,column);const rootNodeId=nullable(document,4);
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,work);
 return{schema:text(document,1),name:text(document,2),manifestId:text(document,3),query:text(document,5),...(rootNodeId===undefined?{}:{rootNodeId}),camera:{x:readBinary64(camera,2,FLOATS),y:readBinary64(camera,3,FLOATS),zoom:readBinary64(camera,4,FLOATS)},content:{childId:text(child,2),target:{artifactId:text(child,3),dialect:{artifactKind:text(child,4),standard:text(child,5),subset:text(child,6)}}}};
}
/** 🧭️ Admit only the declared typed parent and complete logical state across structural aliases. */
export async function validateJackSnapshotSqliteDialect(snapshot:JackArtifact,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);if(dialect.artifactKind!=="s.trinity.jack"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Jack does not own this semantic subset");
 const restored=await jackSnapshotFromSqliteDatabase(database,options),left=await jackSnapshotToSqliteDatabase(snapshot,options),right=await jackSnapshotToSqliteDatabase(restored,options);for(let table=0;table<left.tables.length;table++){const a=left.tables[table]!.rows,b=right.tables[table]!.rows;if(a.length!==b.length)throw Error("Jack logical ownership differs");for(let row=0;row<a.length;row++){if(row%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",row,a.length);const x=a[row]!.values,y=b[row]!.values;if(x.length!==y.length||x.some((value,column)=>value!==y[column]))throw Error("Jack owned identity differs");}}return[];
}
