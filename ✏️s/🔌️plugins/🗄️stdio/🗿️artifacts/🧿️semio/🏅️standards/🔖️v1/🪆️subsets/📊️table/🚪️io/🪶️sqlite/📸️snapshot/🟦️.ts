/** 📊️ Native named columns, ordered rows and genuinely shared typed SemioValue cells. */
import type {SemioTableSnapshot,SemioTableColumn,SemioTableCellKind,SemioTableRow} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {projectSemioValueTree,reconstructSemioValueForest,type ValueSqliteTables} from "../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,ArtifactSqliteRowIndex,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {sqliteOperation,type SqliteDatabase,type SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_TABLE_SQLITE_SCHEMA=`CREATE TABLE semio_table_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE semio_table_column (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_table_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), name TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('null','bool','int','float','str','bytes')));
CREATE TABLE semio_table_row (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_table_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0));
CREATE TABLE semio_table_cell (id INTEGER PRIMARY KEY, row_id INTEGER NOT NULL REFERENCES semio_table_row(id), column_ordinal INTEGER NOT NULL CHECK (column_ordinal >= 0), value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
CREATE TABLE semio_table_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('null','bool','int','float','str','bytes','list','map','ref')), boolean_value INTEGER CHECK (boolean_value IN (0,1)), integer_lexeme TEXT, float_lexeme TEXT, string_value TEXT, bytes_value BLOB, reference_native_id TEXT);
CREATE TABLE semio_table_list_element (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_table_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
CREATE TABLE semio_table_map_entry (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_table_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), member_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
`;
const VALUES:ValueSqliteTables={value:"semio_table_value",listElement:"semio_table_list_element",mapEntry:"semio_table_map_entry"};
function kind(value:string):SemioTableCellKind{switch(value){case "null":case "bool":case "int":case "float":case "str":case "bytes":return value;default:throw Error("unknown Semio table column kind");}}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio table entity identity or columns");}

/** 📤️ Preserve full native cell variants independently of column declaration hints. */
export async function semioTableSnapshotToSqliteDatabase(snapshot:SemioTableSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  options=sqliteOperation(options);await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);if(1+snapshot.columns.length+snapshot.rows.length>(options.maxRows??1_000_000))throw Error("Artifact SQLite row limit");const p=await ArtifactSqliteProjection.create(SEMIO_TABLE_SQLITE_SCHEMA,options);await p.insert("semio_table_document",[snapshot.schema],1n);
  for(let i=0;i<snapshot.columns.length;i++){const column=snapshot.columns[i]!;await p.insert("semio_table_column",[1n,BigInt(i),column.name,kind(column.kind)],BigInt(i+1));}
  for(let i=0;i<snapshot.rows.length;i++){const row=snapshot.rows[i]!;const id=await p.insert("semio_table_row",[1n,BigInt(i)]);for(let column=0;column<row.cells.length;column++){const value=await projectSemioValueTree(row.cells[column]!,VALUES,null,p,options);await p.insert("semio_table_cell",[id,BigInt(column),value]);}}
  return p.finish();
}

/** 📥️ Restore contiguous cells per row independently of named declaration hints. */
export async function semioTableSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioTableSnapshot>{
  options=sqliteOperation(options);await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
  const tables=await artifactSqliteTables(database,SEMIO_TABLE_SQLITE_SCHEMA,options),document=artifactSqliteDocument(tables[0]!);identity(document,2);
  const index=await ArtifactSqliteRowIndex.create(database.tables.filter(table=>["semio_table_document","semio_table_column","semio_table_row","semio_table_cell"].includes(table.name.toLowerCase())),options);await index.take(document);
  const columns:SemioTableColumn[]=[],orderedRows:SqliteRow[]=[],widths:number[]=[],roots:bigint[]=[];
  for(const row of await index.grouped("semio_table_column",1n)){identity(row,5);artifactSqliteDocumentReference(row,1);columns.push({name:artifactSqliteText(row,3),kind:kind(artifactSqliteText(row,4))});await index.take(row);}
  for(const row of await index.grouped("semio_table_row",1n)){
    identity(row,3);artifactSqliteDocumentReference(row,1);orderedRows.push(row);await index.take(row);
    const cells=await index.grouped("semio_table_cell",row.rowid);widths.push(cells.length);
    for(const cell of cells){identity(cell,4);if(artifactSqliteInteger(cell,1)!==row.rowid)throw Error("invalid Semio table cell parent");roots.push(artifactSqliteInteger(cell,3));await index.take(cell);}
  }
  await index.finish();
  const restored=await reconstructSemioValueForest(database,VALUES,roots,null,options),rows:SemioTableRow[]=[];let position=0;
  for(let ordinal=0;ordinal<orderedRows.length;ordinal++){const cells:SemioTableRow["cells"]=[];for(let column=0;column<widths[ordinal]!;column++){cells.push(restored[position++]!);if(position%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",position,restored.length);}rows.push({cells});}
  if(position!==restored.length)throw Error("unconsumed Semio table value");await artifactSqliteCheckpoint(options,"reconstructSnapshot",position,position);return{schema:artifactSqliteText(document,1),columns,rows};
}
