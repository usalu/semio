/** 📊️ Native named columns, ordered rows and genuinely shared typed SemioValue cells. */
import type {SemioTableSnapshot,SemioTableColumn,SemioTableCellKind,SemioTableRow} from "../🟦️.ts";
import {projectSemioValueTree,reconstructSemioValueForest,type ValueSqliteTables} from "../../../../🔢️value/🧬️schema/📸️snapshot/🪶️sqlite/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteDocument,artifactSqliteDocumentReference,artifactSqliteInteger,artifactSqliteText,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const SEMIO_TABLE_SQLITE_SCHEMA=`CREATE TABLE semio_table_document (id INTEGER PRIMARY KEY, schema TEXT NOT NULL);
CREATE TABLE semio_table_column (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_table_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), name TEXT NOT NULL, kind TEXT NOT NULL CHECK (kind IN ('null','bool','int','float','str','bytes')));
CREATE TABLE semio_table_row (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES semio_table_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0));
CREATE TABLE semio_table_cell (id INTEGER PRIMARY KEY, row_id INTEGER NOT NULL REFERENCES semio_table_row(id), column_id INTEGER NOT NULL REFERENCES semio_table_column(id), value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
CREATE TABLE semio_table_value (id INTEGER PRIMARY KEY, kind TEXT NOT NULL CHECK (kind IN ('null','bool','int','float','str','bytes','list','map','ref')), boolean_value INTEGER CHECK (boolean_value IN (0,1)), integer_lexeme TEXT, float_lexeme TEXT, string_value TEXT, bytes_value BLOB, reference_native_id TEXT);
CREATE TABLE semio_table_list_element (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_table_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
CREATE TABLE semio_table_map_entry (id INTEGER PRIMARY KEY, parent_value_id INTEGER NOT NULL REFERENCES semio_table_value(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0), member_key TEXT NOT NULL, value_id INTEGER NOT NULL REFERENCES semio_table_value(id));
`;
const VALUES:ValueSqliteTables={value:"semio_table_value",listElement:"semio_table_list_element",mapEntry:"semio_table_map_entry"};
function kind(value:string):SemioTableCellKind{switch(value){case "null":case "bool":case "int":case "float":case "str":case "bytes":return value;default:throw Error("unknown Semio table column kind");}}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||row.values.length!==columns||artifactSqliteInteger(row,0)!==row.rowid)throw Error("invalid Semio table entity identity or columns");}

/** 📤️ Preserve full native cell variants independently of column declaration hints. */
export async function semioTableSnapshotToSqliteDatabase(snapshot:SemioTableSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);if(1+snapshot.columns.length+snapshot.rows.length>(options.maxRows??1_000_000))throw Error("Artifact SQLite row limit");const p=await ArtifactSqliteProjection.create(SEMIO_TABLE_SQLITE_SCHEMA,options);await p.insert("semio_table_document",[snapshot.schema],1n);const names=new Set<string>();
  for(let i=0;i<snapshot.columns.length;i++){const column=snapshot.columns[i]!;if(names.has(column.name))throw Error("duplicate Semio table column name");names.add(column.name);await p.insert("semio_table_column",[1n,BigInt(i),column.name,kind(column.kind)],BigInt(i+1));}
  for(let i=0;i<snapshot.rows.length;i++){const row=snapshot.rows[i]!;if(row.cells.length!==snapshot.columns.length)throw Error("Semio table row width differs from columns");const id=await p.insert("semio_table_row",[1n,BigInt(i)]);for(let column=0;column<row.cells.length;column++){const value=await projectSemioValueTree(row.cells[column]!,VALUES,null,p,options);await p.insert("semio_table_cell",[id,BigInt(column+1),value]);}}
  return p.finish();
}

/** 📥️ Require every row/column pair exactly once and complete typed value ownership. */
export async function semioTableSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<SemioTableSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const[documents,columns,rows,cells]=await artifactSqliteTables(database,SEMIO_TABLE_SQLITE_SCHEMA,options),document=artifactSqliteDocument(documents!);identity(document,2);
  const ordered=await artifactSqliteOrderedRowsControlled(columns!,2,options),columnIndex=new Map<bigint,number>(),names=new Set<string>(),out:SemioTableColumn[]=[];let units=0;
  for(let i=0;i<ordered.length;i++){const row=ordered[i]!;identity(row,5);artifactSqliteDocumentReference(row,1);const name=artifactSqliteText(row,3);if(columnIndex.has(row.rowid)||names.has(name))throw Error("duplicate Semio table column identity");columnIndex.set(row.rowid,i);names.add(name);out.push({name,kind:kind(artifactSqliteText(row,4))});if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  const orderedRows=await artifactSqliteOrderedRowsControlled(rows!,2,options),rowCells=new Map<bigint,Map<number,bigint>>();for(const row of orderedRows){identity(row,3);artifactSqliteDocumentReference(row,1);if(rowCells.has(row.rowid))throw Error("duplicate Semio table row identity");rowCells.set(row.rowid,new Map());if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  const ids=new Set<bigint>();for(const row of cells!){identity(row,4);const owner=rowCells.get(artifactSqliteInteger(row,1)),column=columnIndex.get(artifactSqliteInteger(row,2));if(!owner||column===undefined||owner.has(column)||ids.has(row.rowid))throw Error("invalid Semio table cell owner or column");ids.add(row.rowid);owner.set(column,artifactSqliteInteger(row,3));if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  const roots:bigint[]=[];for(const row of orderedRows){const items=rowCells.get(row.rowid)!;if(items.size!==out.length)throw Error("missing Semio table row cell");for(let column=0;column<out.length;column++){const id=items.get(column);if(id===undefined)throw Error("missing Semio table column cell");roots.push(id);if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}}
  const restored=await reconstructSemioValueForest(database,VALUES,roots,null,options),result:SemioTableRow[]=[];let position=0;for(const row of orderedRows){const values:SemioTableRow["cells"]=[];for(let column=0;column<out.length;column++)values.push(restored[position++]!);result.push({cells:values});if(++units%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",units,0);}
  return{schema:artifactSqliteText(document,1),columns:out,rows:result};
}
