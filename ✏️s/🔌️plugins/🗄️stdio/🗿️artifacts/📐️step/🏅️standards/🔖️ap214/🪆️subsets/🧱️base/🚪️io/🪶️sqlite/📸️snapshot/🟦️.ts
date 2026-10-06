/** 📐️ STEP-specific exchange entities and typed argument ownership. */
import type {StepSnapshot,StepValue,StepEntity,StepComplexType} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754IsNull,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import type {SqliteRow,SqliteDatabase} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const STEP_SQLITE_SCHEMA=`CREATE TABLE step_document (
  id INTEGER PRIMARY KEY,
  schema TEXT NOT NULL,
  header_id INTEGER NOT NULL REFERENCES step_header(id)
);
CREATE TABLE step_header (
  id INTEGER PRIMARY KEY,
  implementation_level TEXT NOT NULL,
  name TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  preprocessor_version TEXT NOT NULL,
  originating_system TEXT NOT NULL,
  authorization TEXT NOT NULL
);
CREATE TABLE step_description (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  content TEXT NOT NULL
);
CREATE TABLE step_author (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_organization (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_schema_identifier (
  id INTEGER PRIMARY KEY,
  header_id INTEGER NOT NULL REFERENCES step_header(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_entity (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES step_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  instance_id TEXT NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE step_complex_type (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER NOT NULL REFERENCES step_entity(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  name TEXT NOT NULL
);
CREATE TABLE step_argument (
  id INTEGER PRIMARY KEY,
  entity_id INTEGER REFERENCES step_entity(id),
  complex_type_id INTEGER REFERENCES step_complex_type(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES step_value(id),
  CHECK ((entity_id IS NULL) != (complex_type_id IS NULL))
);
CREATE TABLE step_value (
  id INTEGER PRIMARY KEY,
  kind TEXT NOT NULL CHECK (kind IN ('unset','derived','integer','real','string','enum','reference','aggregate','typedValue')),
  integer_value INTEGER,
  real_value REAL,
  string_value TEXT,
  enum_value TEXT,
  reference_instance_id TEXT,
  reference_entity_id INTEGER REFERENCES step_entity(id),
  real_bits INTEGER,
  real_class TEXT CHECK (real_class IN ('finite','positiveInfinity','negativeInfinity','nan')),
  CHECK ((kind = 'integer') = (integer_value IS NOT NULL)),
  CHECK ((kind = 'string') = (string_value IS NOT NULL)),
  CHECK ((kind = 'enum') = (enum_value IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_instance_id IS NOT NULL)),
  CHECK ((kind = 'reference') = (reference_entity_id IS NOT NULL)),
  CHECK ((kind = 'real') = (real_bits IS NOT NULL)),
  CHECK ((kind = 'real') = (real_class IS NOT NULL)),
  CHECK ((real_value IS NOT NULL) = (kind = 'real' AND real_class != 'nan'))
);
CREATE TABLE step_aggregate_element (
  id INTEGER PRIMARY KEY,
  aggregate_value_id INTEGER NOT NULL REFERENCES step_value(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  value_id INTEGER NOT NULL REFERENCES step_value(id)
);
CREATE TABLE step_typed_value (
  id INTEGER PRIMARY KEY,
  wrapper_value_id INTEGER NOT NULL REFERENCES step_value(id),
  type_name TEXT NOT NULL,
  value_id INTEGER NOT NULL REFERENCES step_value(id)
);
`;
const REAL=[{index:3,width:64 as const}] as const;
function signed(value:bigint):bigint{if(typeof value!=="bigint"||value<-(1n<<63n)||value>=(1n<<63n))throw Error("STEP integer exceeds signed64");return value;}
function unsigned(value:bigint):string{if(typeof value!=="bigint"||value<0n||value>=(1n<<64n))throw Error("STEP instance exceeds unsigned64");return value.toString();}
function readUnsigned(row:SqliteRow,index:number):bigint{const text=artifactSqliteText(row,index);if(text.length>20||!/^(0|[1-9][0-9]*)$/.test(text))throw Error("STEP unsigned field must be canonical decimal text");const value=BigInt(text);unsigned(value);return value;}
function identity(row:SqliteRow,columns:number):void{if(row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||row.values.length!==columns)throw Error("invalid STEP entity identity or column count");}
function absent(row:SqliteRow,index:number):void{if(row.values[index]!==null)throw Error("unexpected STEP value variant field");}
type Owner={kind:"argument";entity:bigint|null;complex:bigint|null;ordinal:number}|{kind:"aggregate";parent:bigint;ordinal:number}|{kind:"typed";parent:bigint;name:string};
async function projectValue(value:StepValue,owner:Owner,entities:ReadonlyMap<bigint,bigint>,projection:ArtifactSqliteProjection,options:ArtifactSqliteOptions):Promise<void>{
  const pending:({value:StepValue;owner:Owner}|{exit:object})[]=[{value,owner}],active=new Set<object>();
  while(pending.length){const entry=pending.pop()!;if("exit" in entry){active.delete(entry.exit);continue;}const v=entry.value,cells:Ieee754Cell[]=[0n,"unset",null,null,null,null,null,null];
    if(typeof v==="object"){if(v===null||Object.keys(v).length!==1)throw Error("STEP value must carry exactly one typed variant");if(active.has(v))throw Error("cyclic STEP owned value");active.add(v);pending.push({exit:v});}
    if(v==="unset"||v==="derived")cells[1]=v;
    else if("integer" in v){cells[1]="integer";cells[2]=signed(v.integer);}
    else if("real" in v){cells[1]="real";cells[3]=parseBinary64(v.real);}
    else if("string" in v){cells[1]="string";cells[4]=v.string;}
    else if("enum" in v){cells[1]="enum";cells[5]=v.enum;}
    else if("reference" in v){cells[1]="reference";cells[6]=unsigned(v.reference);const target=entities.get(v.reference);if(target===undefined)throw Error("dangling STEP entity reference");cells[7]=target;}
    else if("aggregate" in v)cells[1]="aggregate";
    else if("typedValue" in v)cells[1]="typedValue";
    else throw Error("unknown STEP value variant");
    const id=await projection.insert("step_value",encodeIeee754Cells(cells,REAL,options.maxColumns).slice(1));
    const o=entry.owner;switch(o.kind){case "argument":await projection.insert("step_argument",[o.entity,o.complex,BigInt(o.ordinal),id]);break;case "aggregate":await projection.insert("step_aggregate_element",[o.parent,BigInt(o.ordinal),id]);break;case "typed":await projection.insert("step_typed_value",[o.parent,o.name,id]);break;}
    if(typeof v==="object"&&"aggregate" in v){projection.checkRowsAdditional(pending.length+v.aggregate.length);for(let i=v.aggregate.length-1;i>=0;i--){pending.push({value:v.aggregate[i]!,owner:{kind:"aggregate",parent:id,ordinal:i}});if(v.aggregate.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,v.aggregate.length);}}
    else if(typeof v==="object"&&"typedValue" in v)pending.push({value:v.typedValue.value,owner:{kind:"typed",parent:id,name:v.typedValue.typeName}});
  }
}
/** 🏗️ Projects the actual STEP exchange fields into their authored relations. */
export async function stepSnapshotToSqliteDatabase(snapshot:StepSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  const p=await ArtifactSqliteProjection.create(STEP_SQLITE_SCHEMA,options),h=snapshot.header,n=h.fileName;
  p.checkRowsAdditional(2+snapshot.entities.length+h.fileDescription.description.length+n.author.length+n.organization.length+h.fileSchema.schemas.length);
  const entities=new Map<bigint,bigint>();for(let i=0;i<snapshot.entities.length;i++){const e=snapshot.entities[i]!;unsigned(e.id);if(entities.has(e.id))throw Error("duplicate STEP instance identifier");entities.set(e.id,BigInt(i+1));if(i%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",i,snapshot.entities.length);}
  await p.insert("step_document",[snapshot.schema,1n]);await p.insert("step_header",[h.fileDescription.implementationLevel,n.name,n.timestamp,n.preprocessorVersion,n.originatingSystem,n.authorization]);
  for(const [table,items] of [["step_description",h.fileDescription.description],["step_author",n.author],["step_organization",n.organization],["step_schema_identifier",h.fileSchema.schemas]] as const)for(let i=0;i<items.length;i++)await p.insert(table,[1n,BigInt(i),items[i]!]);
  for(let i=0;i<snapshot.entities.length;i++){const entity=snapshot.entities[i]!,id=entities.get(entity.id)!;await p.insert("step_entity",[1n,BigInt(i),unsigned(entity.id),entity.name],id);p.checkRowsAdditional(entity.args.length+entity.complex.length);for(let a=0;a<entity.args.length;a++)await projectValue(entity.args[a]!,{kind:"argument",entity:id,complex:null,ordinal:a},entities,p,options);for(let c=0;c<entity.complex.length;c++){const type=entity.complex[c]!,complex=await p.insert("step_complex_type",[id,BigInt(c),type.name]);p.checkRowsAdditional(type.args.length);for(let a=0;a<type.args.length;a++)await projectValue(type.args[a]!,{kind:"argument",entity:null,complex,ordinal:a},entities,p,options);}}
  return p.finish();
}
async function groups(rows:readonly SqliteRow[],owner:number,ordinal:number,options:ArtifactSqliteOptions):Promise<Map<bigint,SqliteRow[]>>{const result=new Map<bigint,SqliteRow[]>();for(let i=0;i<rows.length;i++){const row=rows[i]!,key=artifactSqliteInteger(row,owner);const group=result.get(key)??[];if(!result.has(key))result.set(key,group);group.push(row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}for(const[key,group]of result)result.set(key,await artifactSqliteOrderedRowsControlled(group,ordinal,options));return result;}
async function list(rows:readonly SqliteRow[],options:ArtifactSqliteOptions):Promise<string[]>{const sorted=await artifactSqliteOrderedRowsControlled(rows,2,options),result:string[]=[];for(let i=0;i<sorted.length;i++){const row=sorted[i]!;if(artifactSqliteInteger(row,1)!==1n)throw Error("STEP list belongs to another header");result.push(artifactSqliteText(row,3));if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,sorted.length);}return result;}
async function restoreValues(rows:readonly SqliteRow[],arguments_:readonly SqliteRow[],aggregateRows:readonly SqliteRow[],typedRows:readonly SqliteRow[],entities:ReadonlyMap<bigint,bigint>,options:ArtifactSqliteOptions):Promise<Map<bigint,StepValue>>{
  const values=new Map<bigint,SqliteRow>();for(let i=0;i<rows.length;i++){values.set(rows[i]!.rowid,rows[i]!);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}
  const aggregates=await groups(aggregateRows,1,2,options),wrappers=new Map<bigint,{name:string;child:bigint}>();for(let i=0;i<typedRows.length;i++){const row=typedRows[i]!,parent=artifactSqliteInteger(row,1),value=values.get(parent);if(!value||artifactSqliteText(value,1)!=="typedValue"||wrappers.has(parent))throw Error("invalid STEP typed wrapper owner");wrappers.set(parent,{name:artifactSqliteText(row,2),child:artifactSqliteInteger(row,3)});if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,typedRows.length);}
  for(const parent of aggregates.keys()){const row=values.get(parent);if(!row||artifactSqliteText(row,1)!=="aggregate")throw Error("STEP aggregate relationship has wrong parent kind");}
  const seen=new Set<bigint>(),complete=new Map<bigint,StepValue>();let count=0;function take(id:bigint):StepValue{const value=complete.get(id);if(value===undefined)throw Error("invalid STEP structural ownership");complete.delete(id);return value;}
  for(const argument of arguments_){const pending:{id:bigint;exit:boolean}[]=[{id:artifactSqliteInteger(argument,4),exit:false}];while(pending.length){if(++count%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",count,0);const {id,exit}=pending.pop()!,row=values.get(id);if(!row)throw Error("dangling STEP value ownership edge");if(!exit){if(seen.has(id))throw Error("cyclic or multiply owned STEP value");seen.add(id);pending.push({id,exit:true});const children=aggregates.get(id);if(children)for(let i=children.length-1;i>=0;i--){pending.push({id:artifactSqliteInteger(children[i]!,3),exit:false});if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}const wrapper=wrappers.get(id);if(wrapper)pending.push({id:wrapper.child,exit:false});continue;}
      const kind=artifactSqliteText(row,1),active=kind==="integer"?2:kind==="real"?3:kind==="string"?4:kind==="enum"?5:kind==="reference"?6:undefined;for(let i=2;i<8;i++)if(i!==active&&!(kind==="reference"&&i===7))absent(row,i);if(kind!=="real"&&!ieee754IsNull(row,3,REAL))throw Error("inactive STEP real companions");let value:StepValue;
      switch(kind){case "unset":case "derived":value=kind;break;case "integer":value={integer:artifactSqliteInteger(row,2)};break;case "real":value={real:readBinary64(row,3,REAL)};break;case "string":value={string:artifactSqliteText(row,4)};break;case "enum":value={enum:artifactSqliteText(row,5)};break;case "reference":{const reference=readUnsigned(row,6);if(entities.get(artifactSqliteInteger(row,7))!==reference)throw Error("STEP reference word differs from related entity identity");value={reference};break;}case "aggregate":{const children=aggregates.get(id)??[];aggregates.delete(id);const aggregate:StepValue[]=[];for(let i=0;i<children.length;i++){aggregate.push(take(artifactSqliteInteger(children[i]!,3)));if(children.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,children.length);}value={aggregate};break;}case "typedValue":{const wrapper=wrappers.get(id);if(!wrapper)throw Error("missing STEP typed wrapper");wrappers.delete(id);value={typedValue:{typeName:wrapper.name,value:take(wrapper.child)}};break;}default:throw Error("unknown STEP value kind");}complete.set(id,value);
    }}if(seen.size!==values.size||aggregates.size||wrappers.size)throw Error("unowned STEP value entity");return complete;
}
/** 🌳️ Restores only the owned STEP model after validating every relationship. */
export async function stepSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<StepSnapshot>{
  await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);const tables=await artifactSqliteTables(database,STEP_SQLITE_SCHEMA,options),columns=[3,7,4,4,4,4,5,4,5,10,4,4];
  for(let t=0;t<tables.length;t++){const seen=new Set<bigint>();for(let i=0;i<tables[t]!.length;i++){const row=tables[t]![i]!;identity(row,columns[t]!);if(seen.has(row.rowid))throw Error("duplicate STEP relational row identity");seen.add(row.rowid);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[t]!.length);}}
  if(tables[0]!.length!==1||tables[1]!.length!==1)throw Error("STEP requires one document and header");const document=tables[0]![0]!,header=tables[1]![0]!;if(document.rowid!==1n||artifactSqliteInteger(document,2)!==1n||header.rowid!==1n)throw Error("invalid STEP document/header ownership");
  const entityRows=await artifactSqliteOrderedRowsControlled(tables[6]!,2,options),identities=new Map<bigint,bigint>(),words=new Set<bigint>();for(let i=0;i<entityRows.length;i++){const row=entityRows[i]!,word=readUnsigned(row,3);if(artifactSqliteInteger(row,1)!==1n||words.has(word))throw Error("invalid STEP instance owner or duplicate identity");words.add(word);identities.set(row.rowid,word);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,entityRows.length);}
  const complex=await groups(tables[7]!,1,2,options),entityArgs=new Map<bigint,SqliteRow[]>(),complexArgs=new Map<bigint,SqliteRow[]>();for(let i=0;i<tables[8]!.length;i++){const row=tables[8]![i]!;let map:Map<bigint,SqliteRow[]>,owner:bigint;if(typeof row.values[1]==="bigint"&&row.values[2]===null){map=entityArgs;owner=row.values[1];}else if(row.values[1]===null&&typeof row.values[2]==="bigint"){map=complexArgs;owner=row.values[2];}else throw Error("STEP argument must have exactly one owner");const group=map.get(owner)??[];if(!map.has(owner))map.set(owner,group);group.push(row);if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,tables[8]!.length);}for(const map of [entityArgs,complexArgs])for(const[owner,rows]of map)map.set(owner,await artifactSqliteOrderedRowsControlled(rows,3,options));
  const values=await restoreValues(tables[9]!,tables[8]!,tables[10]!,tables[11]!,identities,options);async function args(map:Map<bigint,SqliteRow[]>,owner:bigint):Promise<StepValue[]>{const rows=map.get(owner)??[],result:StepValue[]=[];map.delete(owner);for(let i=0;i<rows.length;i++){const id=artifactSqliteInteger(rows[i]!,4),value=values.get(id);if(value===undefined)throw Error("missing STEP argument value");values.delete(id);result.push(value);if(rows.length>=256&&i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,rows.length);}return result;}
  const entities:StepEntity[]=[];for(let i=0;i<entityRows.length;i++){const row=entityRows[i]!,types:StepComplexType[]=[];const parts=complex.get(row.rowid)??[];for(let j=0;j<parts.length;j++){const type=parts[j]!;types.push({name:artifactSqliteText(type,3),args:await args(complexArgs,type.rowid)});if(j%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",j,parts.length);}complex.delete(row.rowid);entities.push({id:identities.get(row.rowid)!,name:artifactSqliteText(row,4),args:await args(entityArgs,row.rowid),complex:types});if(i%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",i,entityRows.length);}
  if(complex.size||entityArgs.size||complexArgs.size||values.size)throw Error("unowned STEP entity relationship");const result:StepSnapshot={schema:artifactSqliteText(document,1),header:{fileDescription:{description:await list(tables[2]!,options),implementationLevel:artifactSqliteText(header,1)},fileName:{name:artifactSqliteText(header,2),timestamp:artifactSqliteText(header,3),author:await list(tables[3]!,options),organization:await list(tables[4]!,options),preprocessorVersion:artifactSqliteText(header,4),originatingSystem:artifactSqliteText(header,5),authorization:artifactSqliteText(header,6)},fileSchema:{schemas:await list(tables[5]!,options)}},entities};await artifactSqliteCheckpoint(options,"reconstructSnapshot",entities.length,entities.length);return result;
}
