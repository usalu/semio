import { parseBinary64, parseBinary32, encodeIeee754Cells, readBinary64, readBinary32, ieee754IsNull, type Binary64, type Binary32, type Ieee754Cell, type Ieee754Column } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧱️ PLY typed element declarations, row cells and ordered list items. */
import type { PlySnapshot, PlyProperty, PlyScalarType, PlyValue } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { sqliteValueByteLength, type SqliteDatabase, type SqliteRow, type SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted schema byte-equal to the adjacent SQL asset. */
export const PLY_SQLITE_SCHEMA = "CREATE TABLE ply_document (\n  id INTEGER PRIMARY KEY,\n  schema TEXT NOT NULL,\n  format TEXT NOT NULL CHECK (format IN ('ascii', 'binary_little_endian', 'binary_big_endian'))\n);\nCREATE TABLE ply_comment (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES ply_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  content TEXT NOT NULL\n);\nCREATE TABLE ply_element (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES ply_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL,\n  declared_count_high INTEGER NOT NULL CHECK (declared_count_high BETWEEN 0 AND 4294967295),\n  declared_count_low INTEGER NOT NULL CHECK (declared_count_low BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE ply_property (\n  id INTEGER PRIMARY KEY,\n  element_id INTEGER NOT NULL REFERENCES ply_element(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  name TEXT NOT NULL,\n  form TEXT NOT NULL CHECK (form IN ('scalar', 'list')),\n  scalar_kind TEXT CHECK (scalar_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  count_kind TEXT CHECK (count_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  value_kind TEXT CHECK (value_kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double')),\n  CHECK ((form = 'scalar' AND scalar_kind IS NOT NULL AND count_kind IS NULL AND value_kind IS NULL) OR (form = 'list' AND scalar_kind IS NULL AND count_kind IS NOT NULL AND value_kind IS NOT NULL))\n);\nCREATE TABLE ply_row (\n  id INTEGER PRIMARY KEY,\n  element_id INTEGER NOT NULL REFERENCES ply_element(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0)\n);\nCREATE TABLE ply_cell (\n  id INTEGER PRIMARY KEY,\n  row_id INTEGER NOT NULL REFERENCES ply_row(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value_id INTEGER NOT NULL REFERENCES ply_value(id)\n);\nCREATE TABLE ply_value (\n  id INTEGER PRIMARY KEY,\n  kind TEXT NOT NULL CHECK (kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint', 'float', 'double', 'list')),\n  integer_value INTEGER,\n  real_value REAL,\n  real_value_ieee754_bits INTEGER,\n  real_value_numeric_class TEXT CHECK(real_value_numeric_class IN ('finite','positiveInfinity','negativeInfinity','nan')),\n  CHECK ((kind IN ('char', 'uchar', 'short', 'ushort', 'int', 'uint') AND integer_value IS NOT NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL) OR (kind IN ('float', 'double') AND integer_value IS NULL AND real_value_ieee754_bits IS NOT NULL AND real_value_numeric_class IS NOT NULL AND ((real_value_numeric_class='nan' AND real_value IS NULL) OR (real_value_numeric_class!='nan' AND real_value IS NOT NULL))) OR (kind = 'list' AND integer_value IS NULL AND real_value IS NULL AND real_value_ieee754_bits IS NULL AND real_value_numeric_class IS NULL)),\n  CHECK(kind!='float' OR real_value_ieee754_bits BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE ply_list_item (\n  id INTEGER PRIMARY KEY,\n  list_id INTEGER NOT NULL REFERENCES ply_value(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  value_id INTEGER NOT NULL REFERENCES ply_value(id)\n);\n";
const TABLES = ["ply_document", "ply_comment", "ply_element", "ply_property", "ply_row", "ply_cell", "ply_value", "ply_list_item"] as const;
type Table = typeof TABLES[number];
const KINDS = ["char", "uChar", "short", "uShort", "int", "uInt", "float", "double"] as const;
const RANGES: Partial<Record<PlyScalarType, readonly [number, number]>> = { char: [-128,127], uChar: [0,255], short: [-32768,32767], uShort: [0,65535], int: [-2147483648,2147483647], uInt: [0,4294967295] };
const FORMATS = { ascii: "ascii", binaryLittleEndian: "binary_little_endian", binaryBigEndian: "binary_big_endian" } as const;
function text(value: string): string { artifactSqliteTextBytes(value); return value; }
function kind(value: string): PlyScalarType {
  const found = KINDS.find(kind => kind.toLowerCase() === value);
  if (!found) throw new Error("PLY scalar kind is unknown");
  return found;
}

const BINARY32=[{index:3,width:32}] as const;
const BINARY64=[{index:3,width:64}] as const;
function floatColumns(table:Table,cells:readonly Ieee754Cell[]):readonly Ieee754Column[]{return table==="ply_value"?(cells[0]==="float"?BINARY32:BINARY64):[];}
function scalar(type: PlyScalarType, value: number | Binary64 | Binary32): readonly [bigint | null, Binary64 | Binary32 | null] {
  if (!KINDS.includes(type)) throw new Error("PLY scalar kind must be typed");
  const range = RANGES[type];
  if (range) {
    if (typeof value !== "number" || !Number.isInteger(value) || value < range[0] || value > range[1]) throw new Error("PLY integer scalar is outside its declared width");
    return [BigInt(value), null];
  }
  return [null,type==="float"?parseBinary32(value):parseBinary64(value)];
}
function declaration(property: PlyProperty): SqliteValue[] {
  text(property.name);
  if (property.form === "scalar" && KINDS.includes(property.kind)) return [property.name, "scalar", property.kind.toLowerCase(), null, null];
  if (property.form !== "list" || !KINDS.includes(property.valueKind)) throw new Error("PLY property declaration is invalid");
  if (!KINDS.includes(property.countKind)) throw new Error("PLY list count kind is invalid");
  return [property.name, "list", null, property.countKind.toLowerCase(), property.valueKind.toLowerCase()];
}
type Emit = (table: Table, cells: Ieee754Cell[]) => Promise<bigint>;
async function visit(snapshot:PlySnapshot,emit:Emit,options:ArtifactSqliteOptions):Promise<void>{
 const format=FORMATS[snapshot.format];if(!format)throw new Error("PLY format is invalid");
 await emit("ply_document",[text(snapshot.schema),format]);
 for(let ordinal=0;ordinal<snapshot.comments.length;ordinal++)await emit("ply_comment",[1n,BigInt(ordinal),text(snapshot.comments[ordinal]!)]);
 const pending:PlyValue[]=[];const maximum=options.maxRows??1_000_000;
 const append=(value:PlyValue):bigint=>{if(pending.length>=maximum)throw new Error("PLY SQLite row limit");pending.push(value);return BigInt(pending.length);};
 for(let ordinal=0;ordinal<snapshot.elements.length;ordinal++){
  const element=snapshot.elements[ordinal]!;if(typeof element.count!=="bigint"||element.count<0n||element.count>0xffffffffffffffffn)throw new Error("PLY declared count must be an unsigned64 bigint");
  const id=await emit("ply_element",[1n,BigInt(ordinal),text(element.name),element.count>>32n,element.count&0xffffffffn]);
  for(let ordinal=0;ordinal<element.properties.length;ordinal++)await emit("ply_property",[id,BigInt(ordinal),...declaration(element.properties[ordinal]!)]);
  for(let ordinal=0;ordinal<element.rows.length;ordinal++){
   const row=element.rows[ordinal]!;const rowId=await emit("ply_row",[id,BigInt(ordinal)]);
   for(let ordinal=0;ordinal<row.values.length;ordinal++)await emit("ply_cell",[rowId,BigInt(ordinal),append(row.values[ordinal]!)]);
  }
 }
 for(let index=0;index<pending.length;index++){
  const value=pending[index]!;const id=BigInt(index+1);
  if(value.kind==="list"){
   if(!Array.isArray(value.value))throw new Error("PLY list payload differs");
   await emit("ply_value",["list",null,null]);
   for(let ordinal=0;ordinal<value.value.length;ordinal++)await emit("ply_list_item",[id,BigInt(ordinal),append(value.value[ordinal]!)]);
  }else await emit("ply_value",[value.kind.toLowerCase(),...scalar(value.kind,value.value)]);
 }
}

/** 📤️ Project PLY declarations and scalar cells after cancellable aggregate preflight. */
export async function plySnapshotToSqliteDatabase(snapshot: PlySnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, 0);
  const counts = new Map<Table, number>(TABLES.map(name => [name, 0]));
  let total = 0;
  let bytes = 0;
  await visit(snapshot, async (name, cells) => {
    if (++total > (options.maxRows ?? 1_000_000)) throw new Error("PLY SQLite row limit");
    bytes += 8;
    for (const value of encodeIeee754Cells([0n,...cells],floatColumns(name,cells),options.maxColumns).slice(1)) { bytes += sqliteValueByteLength(value); artifactSqliteValueBudget(bytes, options); }
    const count = counts.get(name)! + 1;
    counts.set(name, count);
    if (total % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
    return BigInt(count);
  },options);
  const rows = new Map<Table, SqliteRow[]>(TABLES.map(name => [name, []]));
  let completed = 0;
  await visit(snapshot, async (name, cells) => {
    const table = rows.get(name)!;
    const id = BigInt(table.length + 1);
    table.push({ rowid: id, values: encodeIeee754Cells([id,...cells],floatColumns(name,cells),options.maxColumns) });
    if (++completed % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", completed, total);
    return id;
  },options);
  const database = await artifactSqliteDatabase(PLY_SQLITE_SCHEMA, TABLES.map(name => rows.get(name)!), options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}
function primitive(row:SqliteRow):Exclude<PlyValue,{kind:"list"}>{
 const type=kind(artifactSqliteText(row,1));const integer=RANGES[type]!==undefined;
 if(row.values[integer?3:2]!==null)throw new Error("PLY scalar has conflicting payloads");
 if(type==="float")return{kind:type,value:readBinary32(row,3,BINARY32)};
 if(type==="double")return{kind:type,value:readBinary64(row,3,BINARY64)};
 if(!ieee754IsNull(row,3,BINARY64))throw new Error("PLY integer scalar has conflicting IEEE payload");
 const value=Number(artifactSqliteInteger(row,2));scalar(type,value);return{kind:type,value};
}

/** 📥️ Reconstruct independent declaration, row-cell and recursive value ownership. */
export async function plySnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<PlySnapshot>{
 const total=database.tables.reduce((sum,table)=>sum+table.rows.length,0);await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,total);
 const [documents,comments,elements,properties,rows,cells,values,items]=await artifactSqliteTables(database,PLY_SQLITE_SCHEMA,options);
 if(documents!.length!==1)throw new Error("PLY document must be a singleton");const document=documents![0]!;
 const formatText=artifactSqliteText(document,2),format=(Object.keys(FORMATS) as PlySnapshot["format"][]).find(key=>FORMATS[key]===formatText);if(!format)throw new Error("PLY format is invalid");
 let completed=0;const tick=async()=>{if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",Math.min(completed,total),total);};
 for(const table of[comments!,elements!])for(const row of table){if(artifactSqliteInteger(row,1)!==document.rowid)throw new Error("PLY document relationship differs");await tick();}
 const own=async(children:readonly SqliteRow[],parents:readonly SqliteRow[])=>{
  const identities=new Set(parents.map(row=>row.rowid)),groups=new Map<bigint,SqliteRow[]>();
  for(const row of children){const owner=artifactSqliteInteger(row,1);if(!identities.has(owner))throw new Error("PLY relationship has an unknown owner");const group=groups.get(owner)??[];group.push(row);groups.set(owner,group);await tick();}
  for(const[owner,group]of groups){groups.set(owner,artifactSqliteOrderedRows(group,2));await tick();}return groups;
 };
 const propertyGroups=await own(properties!,elements!),rowGroups=await own(rows!,elements!),cellGroups=await own(cells!,rows!),itemGroups=await own(items!,values!);
 const byId=new Map(values!.map(row=>[row.rowid,row])),owners=new Set<bigint>();
 const claim=(id:bigint)=>{if(!byId.has(id)||owners.has(id))throw new Error("PLY value must have exactly one known owner");owners.add(id);};
 for(const cell of cells!){claim(artifactSqliteInteger(cell,3));await tick();}
 for(const item of items!){claim(artifactSqliteInteger(item,3));await tick();}
 if(owners.size!==values!.length)throw new Error("PLY unowned value entity");
 const restored=new Map<bigint,PlyValue>(),active=new Set<bigint>(),visited=new Set<bigint>();
 for(const cell of cells!){
  const pending:{id:bigint;exit:boolean}[]=[{id:artifactSqliteInteger(cell,3),exit:false}];
  while(pending.length){const{id,exit}=pending.pop()!;const row=byId.get(id)!;const children=itemGroups.get(id)??[];
   if(exit){active.delete(id);const value:PlyValue={kind:"list",value:[]};for(const child of children){const childId=artifactSqliteInteger(child,3),owned=restored.get(childId);if(!owned)throw new Error("PLY value topology differs");restored.delete(childId);value.value.push(owned);await tick();}restored.set(id,value);}
   else{if(active.has(id)||visited.has(id))throw new Error("PLY value topology cycles or repeats");visited.add(id);const type=artifactSqliteText(row,1);
    if(type==="list"){if(row.values[2]!==null||!ieee754IsNull(row,3,BINARY64))throw new Error("PLY list has scalar payload");active.add(id);if(pending.length+children.length+1>values!.length+1)throw new Error("PLY value frontier limit");pending.push({id,exit:true});for(let index=children.length-1;index>=0;index--)pending.push({id:artifactSqliteInteger(children[index]!,3),exit:false});}
    else{if(children.length)throw new Error("PLY scalar cannot own list items");restored.set(id,primitive(row));}
   }await tick();
  }
 }
 if(visited.size!==values!.length)throw new Error("PLY unreachable value entities");
 const snapshot:PlySnapshot={schema:artifactSqliteText(document,1),format,comments:[],elements:[]};
 for(const row of artifactSqliteOrderedRows(comments!,2)){snapshot.comments.push(artifactSqliteText(row,3));await tick();}
 for(const element of artifactSqliteOrderedRows(elements!,2)){
  const high=artifactSqliteInteger(element,4),low=artifactSqliteInteger(element,5);if(high<0n||high>0xffffffffn||low<0n||low>0xffffffffn)throw new Error("PLY declared count word exceeds unsigned32");
  const result:PlySnapshot["elements"][number]={name:artifactSqliteText(element,3),count:(high<<32n)|low,properties:[],rows:[]};
  for(const row of propertyGroups.get(element.rowid)??[]){const name=artifactSqliteText(row,3),form=artifactSqliteText(row,4);
   if(form==="scalar"){if(row.values[6]!==null||row.values[7]!==null)throw new Error("PLY scalar property has list kinds");result.properties.push({form,name,kind:kind(artifactSqliteText(row,5))});}
   else if(form==="list"){if(row.values[5]!==null)throw new Error("PLY list property has scalar kind");result.properties.push({form,name,countKind:kind(artifactSqliteText(row,6)),valueKind:kind(artifactSqliteText(row,7))});}
   else throw new Error("PLY property form is invalid");await tick();
  }
  for(const row of rowGroups.get(element.rowid)??[]){const parsed:{values:PlyValue[]}={values:[]};for(const cell of cellGroups.get(row.rowid)??[]){const id=artifactSqliteInteger(cell,3),value=restored.get(id);if(!value)throw new Error("PLY row value differs");restored.delete(id);parsed.values.push(value);await tick();}result.rows.push(parsed);await tick();}
  snapshot.elements.push(result);await tick();
 }
 if(restored.size)throw new Error("PLY unconsumed values");await artifactSqliteCheckpoint(options,"reconstructSnapshot",total,total);return snapshot;
}

/** 🛂️ Validate exact dialect and full state while accepting consistent structural identities. */
export async function plySnapshotValidateSqliteSubset(snapshot:PlySnapshot,dialect:import("../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts").ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
 if(dialect.artifactKind!=="s.stdio.ply"||dialect.standard!=="1.0"||dialect.subset!=="*")throw new Error("geometry owned SQLite dialect differs");
 const expected=await plySnapshotToSqliteDatabase(snapshot,options),candidate=await plySnapshotFromSqliteDatabase(database,options),actual=await plySnapshotToSqliteDatabase(candidate,options);
 for(let table=0;table<expected.tables.length;table++){const left=expected.tables[table]!.rows,right=actual.tables[table]!.rows;if(left.length!==right.length)throw new Error("PLY document identity differs");for(let row=0;row<left.length;row++){await artifactSqliteCheckpoint(options,"projectSnapshot",row,left.length);const a=left[row]!.values,b=right[row]!.values;if(a.length!==b.length||a.some((value,index)=>typeof value==="number"? !Object.is(value,b[index]) : value!==b[index]))throw new Error("PLY document identity differs");}}
}
