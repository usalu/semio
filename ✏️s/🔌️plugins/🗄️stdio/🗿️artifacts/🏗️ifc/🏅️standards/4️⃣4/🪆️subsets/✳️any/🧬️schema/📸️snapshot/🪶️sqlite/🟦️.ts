/** 🏗️ IFC4 authored header/entity/value relations. */
import type {IfcSnapshot,IfcValue,IfcEntity,IfcComplexType} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754IsNull,parseBinary64,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {SqliteRow,SqliteDatabase} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const IFC_SQLITE_SCHEMA=`CREATE TABLE ifc_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id)
);
CREATE TABLE ifc_header (id INTEGER PRIMARY KEY);
CREATE TABLE ifc_file_description_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_file_name_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_file_schema_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_entity (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ifc_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE ifc_complex_type (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER NOT NULL REFERENCES ifc_entity(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE ifc_argument (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER REFERENCES ifc_entity(id),
  complex_type_id INTEGER REFERENCES ifc_complex_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  CHECK ((entity_id IS NULL) != (complex_type_id IS NULL))
);
CREATE TABLE ifc_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','integer','real','string','enum','reference','aggregate','typedValue')),
  integer_value INTEGER,
  real_value REAL,
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_entity_id INTEGER REFERENCES ifc_entity(id),
  typed_name TEXT,
  real_bits INTEGER,
  real_class TEXT CHECK (real_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK ((kind = 'integer') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'string') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_entity_id IS NOT NULL)),
  CHECK ((kind = 'typedValue') = (typed_name IS NOT NULL)),
  CHECK ((kind = 'real') = (real_bits IS NOT NULL)),
  CHECK ((kind = 'real') = (real_class IS NOT NULL)),
  CHECK ((real_value IS NOT NULL) = (kind = 'real' AND real_class != 'nan'))
);
CREATE TABLE ifc_aggregate_element (
  id INTEGER PRIMARY KEY,
  aggregate_value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
CREATE TABLE ifc_typed_argument (
  id INTEGER PRIMARY KEY,
  typed_value_id INTEGER NOT NULL REFERENCES ifc_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc_value(id)
);
`;
const REAL=[{index:3,width:64 as const}] as const;
function signed(value:bigint):bigint{if(typeof value!=="bigint"||value<-(1n<<63n)||value>=(1n<<63n))throw Error("IFC integer exceeds signed64");return value;}
function unsigned(value:bigint):string{if(typeof value!=="bigint"||value<0n||value>=(1n<<64n))throw Error("IFC identity exceeds unsigned64");return value.toString();}
function word(row:SqliteRow,index:number):bigint{const value=artifactSqliteText(row,index);if(value.length>20||!/^(0|[1-9][0-9]*)$/.test(value))throw Error("IFC identity is not canonical decimal text");const result=BigInt(value);unsigned(result);return result;}
function absent(row:SqliteRow,index:number):void{if(row.values[index]!==null)throw Error("unexpected IFC value variant field");}
type Owner={kind:"header";table:string;ordinal:number}|{kind:"argument";entity:bigint|null;complex:bigint|null;ordinal:number}|{kind:"aggregate"|"typed";parent:bigint;ordinal:number};
async function projectValue(root:IfcValue,owner:Owner,identities:ReadonlyMap<bigint,bigint>,p:ArtifactSqliteProjection,options:ArtifactSqliteOptions):Promise<void>{const pending:({value:IfcValue;owner:Owner}|{exit:IfcValue})[]=[{value:root,owner}],active=new Set<IfcValue>();while(pending.length){const entry=pending.pop()!;if("exit"in entry){active.delete(entry.exit);continue;}const value=entry.value;if(active.has(value))throw Error("cyclic IFC owned value");active.add(value);pending.push({exit:value});const cells:Ieee754Cell[]=[0n,value.kind,null,null,null,null,null,null,null];switch(value.kind){case"unset":case"derived":break;case"integer":cells[2]=signed(value.value);break;case"real":cells[3]=parseBinary64(value.value);break;case"string":cells[4]=value.value;break;case"enum":cells[5]=value.value;break;case"reference":cells[6]=unsigned(value.value);cells[7]=identities.get(value.value)??null;if(cells[7]===null)throw Error("dangling IFC entity reference");break;case"aggregate":break;case"typedValue":cells[8]=value.value.name;break;default:throw Error("unknown IFC value kind");}const id=await p.insert("ifc_value",encodeIeee754Cells(cells,REAL,options.maxColumns).slice(1)),o=entry.owner;switch(o.kind){case"header":await p.insert(o.table,[1n,BigInt(o.ordinal),id]);break;case"argument":await p.insert("ifc_argument",[o.entity,o.complex,BigInt(o.ordinal),id]);break;case"aggregate":await p.insert("ifc_aggregate_element",[o.parent,BigInt(o.ordinal),id]);break;case"typed":await p.insert("ifc_typed_argument",[o.parent,BigInt(o.ordinal),id]);break;}if(value.kind==="aggregate"||value.kind==="typedValue"){const items=value.kind==="aggregate"?value.value:value.value.items;p.checkRowsAdditional(pending.length+items.length);for(let i=items.length-1;i>=0;i--){pending.push({value:items[i]!,owner:{kind:value.kind==="aggregate"?"aggregate":"typed",parent:id,ordinal:i}});if(items.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,items.length);}}}}
/** 🏗️ Projects every owned IFC4 field without native encoding. */
export async function ifcSnapshotToSqliteDatabase(snapshot:IfcSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{const p=await ArtifactSqliteProjection.create(IFC_SQLITE_SCHEMA,options),h=snapshot.header;p.checkRowsAdditional(2+snapshot.entities.length+h.fileDescription.length+h.fileName.length+h.fileSchema.length);const identities=new Map<bigint,bigint>();for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!;unsigned(entity.id);if(identities.has(entity.id))throw Error("duplicate IFC instance identifier");identities.set(entity.id,BigInt(i+1));if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,snapshot.entities.length);}await p.insert("ifc_document",[snapshot.schema,1n]);await p.insert("ifc_header",[]);for(const[table,values]of[["ifc_file_description_argument",h.fileDescription],["ifc_file_name_argument",h.fileName],["ifc_file_schema_argument",h.fileSchema]]as const)for(let i=0;i<values.length;i++)await projectValue(values[i]!,{kind:"header",table,ordinal:i},identities,p,options);
  for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!,id=identities.get(entity.id)!;await p.insert("ifc_entity",[1n,BigInt(i),unsigned(entity.id),entity.name],id);p.checkRowsAdditional(entity.args.length+entity.complex.length);for(let j=0;j<entity.args.length;j++)await projectValue(entity.args[j]!,{kind:"argument",entity:id,complex:null,ordinal:j},identities,p,options);for(let j=0;j<entity.complex.length;j++){const part=entity.complex[j]!,complex=await p.insert("ifc_complex_type",[id,BigInt(j),part.name]);p.checkRowsAdditional(part.args.length);for(let k=0;k<part.args.length;k++)await projectValue(part.args[k]!,{kind:"argument",entity:null,complex,ordinal:k},identities,p,options);}}return p.finish();}
async function groups(rows:readonly SqliteRow[],index:number,ordinal:number,options:ArtifactSqliteOptions):Promise<Map<bigint,SqliteRow[]>>{const result=new Map<bigint,SqliteRow[]>();for(let i=0;i<rows.length;i++){const row=rows[i]!,owner=artifactSqliteInteger(row,index),group=result.get(owner)??[];if(!result.has(owner))result.set(owner,group);group.push(row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}for(const[key,rows]of result)result.set(key,await artifactSqliteOrderedRowsControlled(rows,ordinal,options));return result;}
async function restoreValues(tables:readonly(readonly SqliteRow[])[],identities:ReadonlyMap<bigint,bigint>,options:ArtifactSqliteOptions):Promise<Map<bigint,IfcValue>>{const records=new Map<bigint,SqliteRow>();for(let i=0;i<tables[8]!.length;i++){const row=tables[8]![i]!;records.set(row.rowid,row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[8]!.length);}const aggregates=await groups(tables[9]!,1,2,options),typed=await groups(tables[10]!,1,2,options);for(const[map,kind]of[[aggregates,"aggregate"],[typed,"typedValue"]]as const){let i=0;for(const parent of map.keys()){const row=records.get(parent);if(!row||artifactSqliteText(row,1)!==kind)throw Error("IFC value relationship has wrong parent kind");if(i++%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,map.size);}}
  const seen=new Set<bigint>(),complete=new Map<bigint,IfcValue>();let count=0;for(const[index,field]of[[2,3],[3,3],[4,3],[7,4]]as const)for(const root of tables[index]!){const pending:{id:bigint;exit:boolean}[]=[{id:artifactSqliteInteger(root,field),exit:false}];while(pending.length){if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);const{id,exit}=pending.pop()!,row=records.get(id);if(!row)throw Error("dangling IFC value ownership edge");if(!exit){if(seen.has(id))throw Error("cyclic or multiply owned IFC value");seen.add(id);pending.push({id,exit:true});for(const map of[aggregates,typed]){const children=map.get(id)??[];for(let i=children.length-1;i>=0;i--){pending.push({id:artifactSqliteInteger(children[i]!,3),exit:false});if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}}continue;}
    const kind=artifactSqliteText(row,1);for(let i=2;i<9;i++){const active=kind==="integer"&&i===2||kind==="real"&&i===3||kind==="string"&&i===4||kind==="enum"&&i===5||kind==="reference"&&(i===6||i===7)||kind==="typedValue"&&i===8;if(!active)absent(row,i);}if(kind!=="real"&&!ieee754IsNull(row,3,REAL))throw Error("inactive IFC IEEE companions");let value:IfcValue;switch(kind){case"unset":case"derived":value={kind};break;case"integer":value={kind,value:artifactSqliteInteger(row,2)};break;case"real":value={kind,value:readBinary64(row,3,REAL)};break;case"string":value={kind,value:artifactSqliteText(row,4)};break;case"enum":value={kind,value:artifactSqliteText(row,5)};break;case"reference":{const reference=word(row,6);if(identities.get(artifactSqliteInteger(row,7))!==reference)throw Error("IFC reference word differs from related entity");value={kind,value:reference};break;}case"aggregate":case"typedValue":{const map=kind==="aggregate"?aggregates:typed,children=map.get(id)??[];map.delete(id);const items:IfcValue[]=[];for(let i=0;i<children.length;i++){const key=artifactSqliteInteger(children[i]!,3),item=complete.get(key);if(!item)throw Error("invalid IFC structural ownership");complete.delete(key);items.push(item);if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}value=kind==="aggregate"?{kind,value:items}:{kind,value:{name:artifactSqliteText(row,8),items}};break;}default:throw Error("unknown IFC value kind");}complete.set(id,value);
  }}if(seen.size!==records.size||aggregates.size||typed.size)throw Error("unowned IFC value entity");return complete;}
/** 🌳️ Reconstructs the complete owned IFC4 graph after checking every relationship. */
export async function ifcSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<IfcSnapshot>{await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const tables=await artifactSqliteTables(database,IFC_SQLITE_SCHEMA,options),columns=[3,1,4,4,4,5,4,5,11,4,4];for(let t=0;t<tables.length;t++){const ids=new Set<bigint>();for(let i=0;i<tables[t]!.length;i++){const row=tables[t]![i]!;if(row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||row.values.length!==columns[t]||ids.has(row.rowid))throw Error("invalid IFC relational identity or columns");ids.add(row.rowid);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[t]!.length);}}if(tables[0]!.length!==1||tables[1]!.length!==1)throw Error("IFC requires one document/header");const root=tables[0]![0]!;if(root.rowid!==1n||artifactSqliteInteger(root,2)!==1n||tables[1]![0]!.rowid!==1n)throw Error("invalid IFC document/header ownership");const rows=await artifactSqliteOrderedRowsControlled(tables[5]!,2,options),identities=new Map<bigint,bigint>(),words=new Set<bigint>();for(let i=0;i<rows.length;i++){const row=rows[i]!,native=word(row,3);if(artifactSqliteInteger(row,1)!==1n||words.has(native))throw Error("invalid IFC instance ownership or duplicate word");words.add(native);identities.set(row.rowid,native);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}
  const complex=await groups(tables[6]!,1,2,options),complexIds=new Set<bigint>();for(let i=0;i<tables[6]!.length;i++){complexIds.add(tables[6]![i]!.rowid);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[6]!.length);}let ownerCount=0;for(const owner of complex.keys()){if(!identities.has(owner))throw Error("dangling IFC complex owner");if(ownerCount++%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ownerCount,complex.size);}const args=new Map<string,SqliteRow[]>();for(let i=0;i<tables[7]!.length;i++){const row=tables[7]![i]!;let owner:string;if(typeof row.values[1]==="bigint"&&row.values[2]===null&&identities.has(row.values[1]))owner="entity:"+row.values[1];else if(row.values[1]===null&&typeof row.values[2]==="bigint"&&complexIds.has(row.values[2]))owner="complex:"+row.values[2];else throw Error("IFC argument must have one existing owner");const group=args.get(owner)??[];if(!args.has(owner))args.set(owner,group);group.push(row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[7]!.length);}for(const[key,rows]of args)args.set(key,await artifactSqliteOrderedRowsControlled(rows,3,options));const values=await restoreValues(tables,identities,options);
  async function take(rows:readonly SqliteRow[],index:number):Promise<IfcValue[]>{const result:IfcValue[]=[];for(let i=0;i<rows.length;i++){const key=artifactSqliteInteger(rows[i]!,index),value=values.get(key);if(!value)throw Error("missing IFC argument value");values.delete(key);result.push(value);if(rows.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}return result;}
  async function header(rows:readonly SqliteRow[]):Promise<IfcValue[]>{const ordered=await artifactSqliteOrderedRowsControlled(rows,2,options);for(let i=0;i<ordered.length;i++){if(artifactSqliteInteger(ordered[i]!,1)!==1n)throw Error("IFC header argument belongs to another header");if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,ordered.length);}return take(ordered,3);}
  async function arguments_(owner:string):Promise<IfcValue[]>{const rows=args.get(owner)??[];args.delete(owner);return take(rows,4);}
  const h={fileDescription:await header(tables[2]!),fileName:await header(tables[3]!),fileSchema:await header(tables[4]!)},entities:IfcEntity[]=[];for(let i=0;i<rows.length;i++){const row=rows[i]!,parts=complex.get(row.rowid)??[],types:IfcComplexType[]=[];complex.delete(row.rowid);for(let j=0;j<parts.length;j++){const part=parts[j]!;types.push({name:artifactSqliteText(part,3),args:await arguments_("complex:"+part.rowid)});if(j%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",j,parts.length);}entities.push({id:identities.get(row.rowid)!,name:artifactSqliteText(row,4),args:await arguments_("entity:"+row.rowid),complex:types});if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}if(values.size||args.size||complex.size)throw Error("unowned IFC relationship");await artifactSqliteCheckpoint(options,"reconstructSnapshot",entities.length,entities.length);return{schema:artifactSqliteText(root,1),header:h,entities};}
