/** 🏭️ IFC2x3 exact decimal/EDM relations. */
import type {Ifc2x3Snapshot,Part21Value,Part21Decimal,Part21Instance,Part21Entity,Ifc2x3EdmPreamble} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteRow,SqliteDatabase,SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const IFC2X3_SQLITE_SCHEMA=`CREATE TABLE ifc2x3_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  edm_preamble_id INTEGER REFERENCES ifc2x3_edm_preamble(id)
);
CREATE TABLE ifc2x3_header (id INTEGER PRIMARY KEY);
CREATE TABLE ifc2x3_file_description_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_file_name_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_file_schema_argument (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES ifc2x3_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_instance (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES ifc2x3_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL
);
CREATE TABLE ifc2x3_entity_type (
  id INTEGER PRIMARY KEY,
  instance_id INTEGER NOT NULL REFERENCES ifc2x3_instance(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE ifc2x3_entity_argument (
  id INTEGER PRIMARY KEY,
  entity_type_id INTEGER NOT NULL REFERENCES ifc2x3_entity_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','int','real','str','enum','ref','list','typed')),
  integer_value INTEGER,
  decimal_negative INTEGER CHECK (decimal_negative IN (0,1)),
  decimal_coefficient TEXT,
  decimal_scale INTEGER CHECK (decimal_scale >= 0 AND decimal_scale <= 4294967295),
  decimal_exponent INTEGER CHECK (decimal_exponent >= -2147483648 AND decimal_exponent <= 2147483647),
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_instance_row_id INTEGER REFERENCES ifc2x3_instance(id),
  typed_name TEXT,
  CHECK ((kind = 'int') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_negative IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_coefficient IS NOT NULL)),
  CHECK ((kind = 'real') = (decimal_scale IS NOT NULL)),
  CHECK (kind = 'real' OR decimal_exponent IS NULL),
  CHECK ((kind = 'str') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'ref') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'ref') = (reference_instance_row_id IS NOT NULL)),
  CHECK ((kind = 'typed') = (typed_name IS NOT NULL))
);
CREATE TABLE ifc2x3_list_element (
  id INTEGER PRIMARY KEY,
  list_value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_typed_argument (
  id INTEGER PRIMARY KEY,
  typed_value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES ifc2x3_value(id)
);
CREATE TABLE ifc2x3_edm_preamble (
  id INTEGER PRIMARY KEY,
  producer TEXT NOT NULL,
  module TEXT NOT NULL,
  creation_date TEXT NOT NULL,
  host TEXT NOT NULL,
  database TEXT NOT NULL,
  database_version TEXT NOT NULL,
  database_creation_date TEXT NOT NULL,
  schema TEXT NOT NULL,
  model TEXT NOT NULL,
  model_creation_date TEXT NOT NULL,
  header_model TEXT NOT NULL,
  header_model_creation_date TEXT NOT NULL,
  user TEXT NOT NULL,
  group_name TEXT NOT NULL,
  license TEXT NOT NULL,
  options TEXT NOT NULL
);
`;
function signed(value:bigint):bigint{if(typeof value!=="bigint"||value<-(1n<<63n)||value>=(1n<<63n))throw Error("IFC2x3 integer exceeds signed64");return value;}
function unsigned(value:bigint):string{if(typeof value!=="bigint"||value<0n||value>=(1n<<64n))throw Error("IFC2x3 identity exceeds unsigned64");return value.toString();}
function word(row:SqliteRow,index:number):bigint{const text=artifactSqliteText(row,index);if(text.length>20||!/^(0|[1-9][0-9]*)$/.test(text))throw Error("IFC2x3 identity is not canonical unsigned64 text");const value=BigInt(text);unsigned(value);return value;}
function width(value:number,minimum:number,maximum:number):number{if(!Number.isSafeInteger(value)||value<minimum||value>maximum)throw Error("IFC2x3 decimal scalar exceeds native width");return value;}
async function coefficient(value:string,options:ArtifactSqliteOptions,phase:"projectSnapshot"|"reconstructSnapshot"):Promise<void>{if(typeof value!=="string"||!value.length)throw Error("IFC2x3 decimal coefficient is empty");if(value.length>(options.maxValueBytes??268435456))throw Error("IFC2x3 decimal coefficient exceeds value limit");for(let start=0;start<value.length;start+=4096){await artifactSqliteCheckpoint(options,phase,start,value.length,value.length>=65536);for(let end=Math.min(start+4096,value.length),i=start;i<end;i++){const code=value.charCodeAt(i);if(code<48||code>57)throw Error("IFC2x3 decimal coefficient contains non-digits");}}}
type Owner={kind:"header";table:string;ordinal:number}|{kind:"argument"|"list"|"typed";parent:bigint;ordinal:number};
async function projectValue(root:Part21Value,owner:Owner,identities:ReadonlyMap<bigint,bigint>,p:ArtifactSqliteProjection,options:ArtifactSqliteOptions):Promise<void>{const pending:({value:Part21Value;owner:Owner}|{exit:Part21Value})[]=[{value:root,owner}],active=new Set<Part21Value>();while(pending.length){const entry=pending.pop()!;if("exit"in entry){active.delete(entry.exit);continue;}const value=entry.value;if(active.has(value))throw Error("cyclic IFC2x3 owned value");active.add(value);pending.push({exit:value});const cells:SqliteValue[]=[value.kind,null,null,null,null,null,null,null,null,null,null];switch(value.kind){case"unset":case"derived":break;case"int":cells[1]=signed(value.value);break;case"real":{const decimal=value.value;p.checkValueBytesAdditional(decimal.coefficient.length);await coefficient(decimal.coefficient,options,"projectSnapshot");if(typeof decimal.negative!=="boolean")throw Error("IFC2x3 decimal sign must be boolean");cells[2]=decimal.negative?1n:0n;cells[3]=decimal.coefficient;cells[4]=BigInt(width(decimal.scale,0,4294967295));cells[5]=decimal.exponent===undefined?null:BigInt(width(decimal.exponent,-2147483648,2147483647));break;}case"str":cells[6]=value.value;break;case"enum":cells[7]=value.value;break;case"ref":cells[8]=unsigned(value.value);cells[9]=identities.get(value.value)??null;if(cells[9]===null)throw Error("dangling IFC2x3 instance reference");break;case"list":break;case"typed":cells[10]=value.typeName;break;default:throw Error("unknown IFC2x3 value kind");}const id=await p.insert("ifc2x3_value",cells),o=entry.owner;switch(o.kind){case"header":await p.insert(o.table,[1n,BigInt(o.ordinal),id]);break;case"argument":await p.insert("ifc2x3_entity_argument",[o.parent,BigInt(o.ordinal),id]);break;case"list":await p.insert("ifc2x3_list_element",[o.parent,BigInt(o.ordinal),id]);break;case"typed":await p.insert("ifc2x3_typed_argument",[o.parent,BigInt(o.ordinal),id]);break;}if(value.kind==="list"||value.kind==="typed"){p.checkRowsAdditional(pending.length+value.values.length);for(let i=value.values.length-1;i>=0;i--){pending.push({value:value.values[i]!,owner:{kind:value.kind==="list"?"list":"typed",parent:id,ordinal:i}});if(value.values.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,value.values.length);}}}}
/** 🏭️ Projects exact decimal components and the sixteen owned EDM fields. */
export async function ifc2x3SnapshotToSqliteDatabase(snapshot:Ifc2x3Snapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{const p=await ArtifactSqliteProjection.create(IFC2X3_SQLITE_SCHEMA,options),h=snapshot.document.header;p.checkRowsAdditional(2+(snapshot.edmPreamble?1:0)+snapshot.document.instances.length+h.fileDescription.length+h.fileName.length+h.fileSchema.length);const identities=new Map<bigint,bigint>();for(let i=0;i<snapshot.document.instances.length;i++){const instance=snapshot.document.instances[i]!;unsigned(instance.id);if(identities.has(instance.id))throw Error("duplicate IFC2x3 instance identifier");identities.set(instance.id,BigInt(i+1));if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,snapshot.document.instances.length);}await p.insert("ifc2x3_document",[snapshot.schema,1n,snapshot.edmPreamble?1n:null]);await p.insert("ifc2x3_header",[]);if(snapshot.edmPreamble){const e=snapshot.edmPreamble;await p.insert("ifc2x3_edm_preamble",[e.producer,e.module,e.creationDate,e.host,e.database,e.databaseVersion,e.databaseCreationDate,e.schema,e.model,e.modelCreationDate,e.headerModel,e.headerModelCreationDate,e.user,e.group,e.license,e.options]);}for(const[table,values]of[["ifc2x3_file_description_argument",h.fileDescription],["ifc2x3_file_name_argument",h.fileName],["ifc2x3_file_schema_argument",h.fileSchema]]as const)for(let i=0;i<values.length;i++)await projectValue(values[i]!,{kind:"header",table,ordinal:i},identities,p,options);
  for(let i=0;i<snapshot.document.instances.length;i++){const instance=snapshot.document.instances[i]!,id=identities.get(instance.id)!;await p.insert("ifc2x3_instance",[1n,BigInt(i),unsigned(instance.id)],id);p.checkRowsAdditional(instance.entities.length);for(let j=0;j<instance.entities.length;j++){const type=instance.entities[j]!,entity=await p.insert("ifc2x3_entity_type",[id,BigInt(j),type.typeName]);p.checkRowsAdditional(type.arguments.length);for(let k=0;k<type.arguments.length;k++)await projectValue(type.arguments[k]!,{kind:"argument",parent:entity,ordinal:k},identities,p,options);}}return p.finish();}
async function groups(rows:readonly SqliteRow[],owner:number,ordinal:number,options:ArtifactSqliteOptions):Promise<Map<bigint,SqliteRow[]>>{const result=new Map<bigint,SqliteRow[]>();for(let i=0;i<rows.length;i++){const row=rows[i]!,key=artifactSqliteInteger(row,owner),group=result.get(key)??[];if(!result.has(key))result.set(key,group);group.push(row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}for(const[key,group]of result)result.set(key,await artifactSqliteOrderedRowsControlled(group,ordinal,options));return result;}
async function restoreValues(tables:readonly(readonly SqliteRow[])[],identities:ReadonlyMap<bigint,bigint>,options:ArtifactSqliteOptions):Promise<Map<bigint,Part21Value>>{const records=new Map<bigint,SqliteRow>();for(let i=0;i<tables[8]!.length;i++){const row=tables[8]![i]!;records.set(row.rowid,row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[8]!.length);}const lists=await groups(tables[9]!,1,2,options),typed=await groups(tables[10]!,1,2,options);for(const[map,kind]of[[lists,"list"],[typed,"typed"]]as const){let i=0;for(const parent of map.keys()){const row=records.get(parent);if(!row||artifactSqliteText(row,1)!==kind)throw Error("IFC2x3 value relationship has wrong parent kind");if(i++%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,map.size);}}
  const seen=new Set<bigint>(),complete=new Map<bigint,Part21Value>();let count=0;for(const index of[2,3,4,7])for(const root of tables[index]!){const pending:{id:bigint;exit:boolean}[]=[{id:artifactSqliteInteger(root,3),exit:false}];while(pending.length){if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);const{id,exit}=pending.pop()!,row=records.get(id);if(!row)throw Error("dangling IFC2x3 value ownership edge");if(!exit){if(seen.has(id))throw Error("cyclic or multiply owned IFC2x3 value");seen.add(id);pending.push({id,exit:true});for(const map of[lists,typed]){const children=map.get(id)??[];for(let i=children.length-1;i>=0;i--){pending.push({id:artifactSqliteInteger(children[i]!,3),exit:false});if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}}continue;}
    const kind=artifactSqliteText(row,1);for(let i=2;i<12;i++){const active=kind==="int"&&i===2||kind==="real"&&i>=3&&i<=6||kind==="str"&&i===7||kind==="enum"&&i===8||kind==="ref"&&(i===9||i===10)||kind==="typed"&&i===11;if(!active&&row.values[i]!==null)throw Error("unexpected IFC2x3 value variant field");}let value:Part21Value;switch(kind){case"unset":case"derived":value={kind};break;case"int":value={kind,value:artifactSqliteInteger(row,2)};break;case"real":{const sign=artifactSqliteInteger(row,3);if(sign!==0n&&sign!==1n)throw Error("invalid IFC2x3 decimal sign");const c=artifactSqliteText(row,4);await coefficient(c,options,"reconstructSnapshot");const decimal:Part21Decimal={negative:sign===1n,coefficient:c,scale:width(Number(artifactSqliteInteger(row,5)),0,4294967295)};if(row.values[6]!==null)decimal.exponent=width(Number(artifactSqliteInteger(row,6)),-2147483648,2147483647);value={kind,value:decimal};break;}case"str":value={kind,value:artifactSqliteText(row,7)};break;case"enum":value={kind,value:artifactSqliteText(row,8)};break;case"ref":{const reference=word(row,9);if(identities.get(artifactSqliteInteger(row,10))!==reference)throw Error("IFC2x3 reference word differs from related instance");value={kind,value:reference};break;}case"list":case"typed":{const map=kind==="list"?lists:typed,children=map.get(id)??[];map.delete(id);const items:Part21Value[]=[];for(let i=0;i<children.length;i++){const key=artifactSqliteInteger(children[i]!,3),item=complete.get(key);if(!item)throw Error("invalid IFC2x3 structural ownership");complete.delete(key);items.push(item);if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}value=kind==="list"?{kind,values:items}:{kind,typeName:artifactSqliteText(row,11),values:items};break;}default:throw Error("unknown IFC2x3 value kind");}complete.set(id,value);
  }}if(seen.size!==records.size||lists.size||typed.size)throw Error("unowned IFC2x3 value entity");return complete;}
/** 🌳️ Reconstructs the complete IFC2x3 model with exact decimal optionality. */
export async function ifc2x3SnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<Ifc2x3Snapshot>{await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const tables=await artifactSqliteTables(database,IFC2X3_SQLITE_SCHEMA,options),columns=[4,1,4,4,4,4,4,4,12,4,4,17];for(let t=0;t<tables.length;t++){const ids=new Set<bigint>();for(let i=0;i<tables[t]!.length;i++){const row=tables[t]![i]!;if(row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||row.values.length!==columns[t]||ids.has(row.rowid))throw Error("invalid IFC2x3 relational identity or columns");ids.add(row.rowid);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[t]!.length);}}if(tables[0]!.length!==1||tables[1]!.length!==1)throw Error("IFC2x3 requires one document/header");const root=tables[0]![0]!;if(root.rowid!==1n||artifactSqliteInteger(root,2)!==1n||tables[1]![0]!.rowid!==1n)throw Error("invalid IFC2x3 document/header ownership");let edm:Ifc2x3EdmPreamble|undefined;if(root.values[3]===null){if(tables[11]!.length)throw Error("unowned IFC2x3 EDM preamble");}else{if(root.values[3]!==1n||tables[11]!.length!==1||tables[11]![0]!.rowid!==1n)throw Error("invalid IFC2x3 optional EDM relationship");const e=tables[11]![0]!;edm={producer:artifactSqliteText(e,1),module:artifactSqliteText(e,2),creationDate:artifactSqliteText(e,3),host:artifactSqliteText(e,4),database:artifactSqliteText(e,5),databaseVersion:artifactSqliteText(e,6),databaseCreationDate:artifactSqliteText(e,7),schema:artifactSqliteText(e,8),model:artifactSqliteText(e,9),modelCreationDate:artifactSqliteText(e,10),headerModel:artifactSqliteText(e,11),headerModelCreationDate:artifactSqliteText(e,12),user:artifactSqliteText(e,13),group:artifactSqliteText(e,14),license:artifactSqliteText(e,15),options:artifactSqliteText(e,16)};}
  const rows=await artifactSqliteOrderedRowsControlled(tables[5]!,2,options),identities=new Map<bigint,bigint>(),words=new Set<bigint>();for(let i=0;i<rows.length;i++){const row=rows[i]!,native=word(row,3);if(artifactSqliteInteger(row,1)!==1n||words.has(native))throw Error("invalid IFC2x3 instance ownership or duplicate word");words.add(native);identities.set(row.rowid,native);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}const types=await groups(tables[6]!,1,2,options),typeIds=new Set<bigint>();for(let i=0;i<tables[6]!.length;i++){typeIds.add(tables[6]![i]!.rowid);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[6]!.length);}let ownerCount=0;for(const owner of types.keys()){if(!identities.has(owner))throw Error("dangling IFC2x3 entity type owner");if(ownerCount++%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ownerCount,types.size);}const args=await groups(tables[7]!,1,2,options);ownerCount=0;for(const owner of args.keys()){if(!typeIds.has(owner))throw Error("dangling IFC2x3 argument owner");if(ownerCount++%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ownerCount,args.size);}const values=await restoreValues(tables,identities,options);
  async function take(rows:readonly SqliteRow[]):Promise<Part21Value[]>{const result:Part21Value[]=[];for(let i=0;i<rows.length;i++){const key=artifactSqliteInteger(rows[i]!,3),value=values.get(key);if(!value)throw Error("missing IFC2x3 argument value");values.delete(key);result.push(value);if(rows.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}return result;}
  async function header(rows:readonly SqliteRow[]):Promise<Part21Value[]>{const ordered=await artifactSqliteOrderedRowsControlled(rows,2,options);for(let i=0;i<ordered.length;i++){if(artifactSqliteInteger(ordered[i]!,1)!==1n)throw Error("IFC2x3 header argument belongs to another header");if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,ordered.length);}return take(ordered);}
  const h={fileDescription:await header(tables[2]!),fileName:await header(tables[3]!),fileSchema:await header(tables[4]!)},instances:Part21Instance[]=[];for(let i=0;i<rows.length;i++){const row=rows[i]!,parts=types.get(row.rowid)??[],entities:Part21Entity[]=[];types.delete(row.rowid);for(let j=0;j<parts.length;j++){const part=parts[j]!,arguments_=await take(args.get(part.rowid)??[]);args.delete(part.rowid);entities.push({typeName:artifactSqliteText(part,3),arguments:arguments_});if(j%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",j,parts.length);}instances.push({id:identities.get(row.rowid)!,entities});if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}if(values.size||args.size||types.size)throw Error("unowned IFC2x3 relationship");const result:Ifc2x3Snapshot={schema:artifactSqliteText(root,1),document:{header:h,instances}};if(edm)result.edmPreamble=edm;await artifactSqliteCheckpoint(options,"reconstructSnapshot",instances.length,instances.length);return result;}
