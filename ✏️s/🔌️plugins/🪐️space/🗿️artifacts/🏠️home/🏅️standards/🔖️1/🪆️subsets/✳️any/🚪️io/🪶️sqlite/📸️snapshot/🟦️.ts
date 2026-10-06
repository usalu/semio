/** 🏠️ Literal Home schema and exact unsigned64 generation SQLite fields. */
import {parseSHomeSnapshot,type SHomeSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {SqliteDatabase} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactDialect} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteValueBudget,artifactSqliteInteger,artifactSqliteText,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
export const HOME_SQLITE_SCHEMA=String.raw`CREATE TABLE home_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL, catalog_generation_high INTEGER NOT NULL CHECK(catalog_generation_high BETWEEN 0 AND 4294967295), catalog_generation_low INTEGER NOT NULL CHECK(catalog_generation_low BETWEEN 0 AND 4294967295));
`;
/** 🔤️ Schema scanning reports UTF-16 code units before inserting entity rows. */
async function schemaBytes(value:string,options:ArtifactSqliteOptions,phase:"projectSnapshot"|"reconstructSnapshot"):Promise<number>{
 let bytes=0,nextCheckpoint=65536;await artifactSqliteCheckpoint(options,phase,0,value.length);for(let index=0;index<value.length;index++){
  const code=value.charCodeAt(index);if(code>=0xd800&&code<=0xdbff){const next=value.charCodeAt(++index);if(!(next>=0xdc00&&next<=0xdfff))throw Error("Home schema is not Unicode scalar text");bytes+=4;}else if(code>=0xdc00&&code<=0xdfff)throw Error("Home schema is not Unicode scalar text");else bytes+=code<0x80?1:code<0x800?2:3;
  if(index+1>=nextCheckpoint){artifactSqliteValueBudget(bytes+24,options);await artifactSqliteCheckpoint(options,phase,index+1,value.length);await new Promise<void>(resolve=>setTimeout(resolve,0));nextCheckpoint=index+1+65536;}
 }artifactSqliteValueBudget(bytes+24,options);await artifactSqliteCheckpoint(options,phase,value.length,value.length);return bytes;
}
/** 📤️ Admit actual fields before allocating the one literal entity. */
export async function homeSnapshotToSqliteDatabase(snapshot:SHomeSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(HOME_SQLITE_SCHEMA,options);const value=parseSHomeSnapshot(snapshot);p.checkRowsAdditional(1);p.checkValueBytesAdditional(24+await schemaBytes(value.schema,options,"projectSnapshot"));await p.insert("home_document",[value.schema,value.catalogGeneration>>32n,value.catalogGeneration&0xffffffffn],1n);return p.finish();
}
/** 📥️ Restore the actual schema and unsigned words after exact entity admission. */
export async function homeSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SHomeSnapshot>{
 const tables=await artifactSqliteTables(database,HOME_SQLITE_SCHEMA,options);if(tables.length!==1||tables[0]!.length!==1)throw Error("Home requires its single document");const row=tables[0]![0]!;if(row.values.length!==4||artifactSqliteInteger(row,0)!==row.rowid)throw Error("Home document identity differs");const high=artifactSqliteInteger(row,2),low=artifactSqliteInteger(row,3);if(high<0n||high>0xffffffffn||low<0n||low>0xffffffffn)throw Error("Home generation word exceeds unsigned32");const schema=artifactSqliteText(row,1);await schemaBytes(schema,options,"reconstructSnapshot");return{schema,catalogGeneration:(high<<32n)|low};
}
/** 🧭️ Require the exact declaration and complete semantic state, independent of surrogate IDs. */
export async function validateHomeSnapshotSqliteDialect(snapshot:SHomeSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0,false);if(dialect.artifactKind!=="s.space.home"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Home does not own this semantic subset");const expected=parseSHomeSnapshot(snapshot),actual=await homeSnapshotFromSqliteDatabase(database,options);if(expected.schema!==actual.schema||expected.catalogGeneration!==actual.catalogGeneration)throw Error("Home owned state differs");
}
