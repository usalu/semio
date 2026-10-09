/** 🧪️ Raw Unicode Socket snapshot in its authored single text entity. */
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteText,artifactSqliteValueByteLengthControlled,artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {ValueError,sqliteOperation,type SqliteDatabase} from "../../../../../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SOCKET_PROBE_SQLITE_SCHEMA="CREATE TABLE socket_probe (id INTEGER PRIMARY KEY CHECK (id = 1), text TEXT NOT NULL);\n";
export const SOCKET_PROBE_NATIVE_DIALECT="native.socket-grant.probe@1/*";
/** 📤️ Admits raw UTF-8 and the actual identity cell before projecting its owned row. */
export async function socketProbeToSqliteDatabase(snapshot:string,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const operation=sqliteOperation(options);options=operation;
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,1);
 if(typeof snapshot!=="string")throw new ValueError("invalidValue","Socket probe requires raw Unicode text");
 if((options.maxRows??1_000_000)<1)throw new ValueError("workLimit","Socket probe requires one row");
 await artifactSqliteValueByteLengthControlled(snapshot,options,"projectSnapshot",8);
 const projection=await ArtifactSqliteProjection.create(SOCKET_PROBE_SQLITE_SCHEMA,options);
 await projection.insert("socket_probe",[snapshot],1n);
 return projection.finish();
}
/** 📥️ Restores the exact raw text after bounded schema, row and storage-class admission. */
export async function socketProbeFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<string>{
 const operation=sqliteOperation(options);options=operation;
 const tables=await artifactSqliteTables(database,SOCKET_PROBE_SQLITE_SCHEMA,options);
 const row=artifactSqliteDocument(tables[0]!);
 if(row.values.length!==2)throw new ValueError("invalidValue","Socket probe requires two declared cells");
 const text=artifactSqliteText(row,1);
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,1);
 return text;
}
