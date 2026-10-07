/** 🔌️ Thirteen handwritten Jack tables preserve the inline manifest and exact parent state. */
import type {JackArtifact,PropertyDef,ValueType,Manifest} from "../../../🧬️schema/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger as integer,artifactSqliteText as text,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
export const JACK_SQLITE_SCHEMA=String.raw`CREATE TABLE jack_document (
 id INTEGER PRIMARY KEY,
 schema TEXT NOT NULL,
 name TEXT NOT NULL,
 manifest_id TEXT,
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
CREATE TABLE jack_node_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL
);
CREATE TABLE jack_edge_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL
);
CREATE TABLE jack_port_kind (
 id INTEGER PRIMARY KEY,
 document_id INTEGER NOT NULL REFERENCES jack_document(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 direction TEXT NOT NULL
);
CREATE TABLE jack_node_kind_port (
 id INTEGER PRIMARY KEY,
 node_kind_id INTEGER NOT NULL REFERENCES jack_node_kind(id),
 ordinal INTEGER NOT NULL,
 port_kind TEXT NOT NULL
);
CREATE TABLE jack_value_type (
 id INTEGER PRIMARY KEY,
 variant TEXT NOT NULL
);
CREATE TABLE jack_value_type_list (
 id INTEGER PRIMARY KEY,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id),
 child_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_value_type_schema (
 id INTEGER PRIMARY KEY,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id),
 schema TEXT NOT NULL
);
CREATE TABLE jack_node_property (
 id INTEGER PRIMARY KEY,
 node_kind_id INTEGER NOT NULL REFERENCES jack_node_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_edge_property (
 id INTEGER PRIMARY KEY,
 edge_kind_id INTEGER NOT NULL REFERENCES jack_edge_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
CREATE TABLE jack_port_property (
 id INTEGER PRIMARY KEY,
 port_kind_id INTEGER NOT NULL REFERENCES jack_port_kind(id),
 ordinal INTEGER NOT NULL,
 name TEXT NOT NULL,
 property_kind TEXT NOT NULL,
 expression TEXT,
 value_type_id INTEGER NOT NULL REFERENCES jack_value_type(id)
);
`;
const TABLES=["jack_document","jack_camera","jack_content_child","jack_node_kind","jack_edge_kind","jack_port_kind","jack_node_kind_port","jack_value_type","jack_value_type_list","jack_value_type_schema","jack_node_property","jack_edge_property","jack_port_property"]as const;
const WIDTHS=[6,11,7,4,4,5,4,2,3,3,7,7,7];
const FLOATS=[{index:2,width:64},{index:3,width:64},{index:4,width:64}]as const;
async function count(snapshot:JackArtifact,p:ArtifactSqliteProjection,options:ArtifactSqliteOptions):Promise<void>{let rows=3+snapshot.manifest.nodeKinds.reduce((count,kind)=>count+kind.portKinds.length,0),work=0;p.checkRowsAdditional(rows);for(const group of[snapshot.manifest.nodeKinds,snapshot.manifest.edgeKinds,snapshot.manifest.portKinds])for(const kind of group){rows++;p.checkRowsAdditional(rows);for(const property of kind.properties){rows++;const seen=new Set<object>();let type=property.valueType;for(;;){if(!type||typeof type!=="object"||!["boolean","integer","decimal","text","any","schema","list"].includes(type.kind))throw Error("Jack ValueType variant differs");if(seen.has(type))throw Error("Jack ValueType ownership cycles");seen.add(type);rows++;if(type.kind==="list"||type.kind==="schema")rows++;p.checkRowsAdditional(rows);if(++work%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",work,0);if(type.kind!=="list")break;type=type.of;}}}await artifactSqliteCheckpoint(options,"projectSnapshot",work,work);}
/** 📤️ Forecast every literal domain entity before creating its owned relational cells. */
export async function jackSnapshotToSqliteDatabase(snapshot:JackArtifact,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(JACK_SQLITE_SCHEMA,options);await count(snapshot,p,options);
 const document=await p.insert("jack_document",[snapshot.schema,snapshot.name,snapshot.manifestId??null,snapshot.rootNodeId??null,snapshot.query]);
 await p.insert("jack_camera",encodeIeee754Cells([1n,document,snapshot.camera.x,snapshot.camera.y,snapshot.camera.zoom],FLOATS,options.maxColumns).slice(1));
 const c=snapshot.content;await p.insert("jack_content_child",[document,c.childId,c.target.artifactId,c.target.dialect.artifactKind,c.target.dialect.standard,c.target.dialect.subset]);
 async function valueType(value:ValueType):Promise<bigint>{let type=value;let root:bigint|undefined,previous:bigint|undefined;for(;;){const id=await p.insert("jack_value_type",[type.kind]);root??=id;if(previous!==undefined)await p.insert("jack_value_type_list",[previous,id]);if(type.kind==="schema")await p.insert("jack_value_type_schema",[id,type.of]);if(type.kind!=="list")return root;previous=id;type=type.of;}}
 async function properties(table:string,parent:bigint,values:readonly PropertyDef[]):Promise<void>{for(let ordinal=0;ordinal<values.length;ordinal++){const value=values[ordinal]!;if(value.kind!=="data"&&value.kind!=="derived")throw Error("Jack property variant differs");await p.insert(table,[parent,BigInt(ordinal),value.name,value.kind,value.expr??null,await valueType(value.valueType)]);}}
 for(let ordinal=0;ordinal<snapshot.manifest.nodeKinds.length;ordinal++){const value=snapshot.manifest.nodeKinds[ordinal]!,id=await p.insert("jack_node_kind",[document,BigInt(ordinal),value.name]);for(let port=0;port<value.portKinds.length;port++)await p.insert("jack_node_kind_port",[id,BigInt(port),value.portKinds[port]!]);await properties("jack_node_property",id,value.properties);}
 for(let ordinal=0;ordinal<snapshot.manifest.edgeKinds.length;ordinal++){const value=snapshot.manifest.edgeKinds[ordinal]!,id=await p.insert("jack_edge_kind",[document,BigInt(ordinal),value.name]);await properties("jack_edge_property",id,value.properties);}
 for(let ordinal=0;ordinal<snapshot.manifest.portKinds.length;ordinal++){const value=snapshot.manifest.portKinds[ordinal]!;if(value.direction!=="in"&&value.direction!=="out")throw Error("Jack direction differs");const id=await p.insert("jack_port_kind",[document,BigInt(ordinal),value.name,value.direction]);await properties("jack_port_property",id,value.properties);}
 return p.finish();
}
/** 📥️ Consume each entity once through genuine owner links and contiguous literal ordinals. */
export async function jackSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<JackArtifact>{
 const tables=await artifactSqliteTables(database,JACK_SQLITE_SCHEMA,options),maps:Map<bigint,SqliteRow>[]=[],used=new Set<string>();let work=0;
 for(let table=0;table<tables.length;table++){const map=new Map<bigint,SqliteRow>();for(const row of tables[table]!){if(row.values.length!==WIDTHS[table]||integer(row,0)!==row.rowid||map.has(row.rowid))throw Error("Jack row identity differs");map.set(row.rowid,row);if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,0);}maps.push(map);}
 const take=(table:number,id:bigint):SqliteRow=>{const key=table+":"+id,row=maps[table]!.get(id);if(!row||used.has(key))throw Error("Jack entity ownership differs");used.add(key);return row;};
 const fixed=(table:number)=>{if(tables[table]!.length!==1)throw Error("Jack singleton differs");return take(table,tables[table]![0]!.rowid);};
 const document=fixed(0),camera=fixed(1),child=fixed(2);if(integer(camera,1)!==document.rowid||integer(child,1)!==document.rowid)throw Error("Jack document parent differs");
 const nullable=(row:SqliteRow,column:number)=>row.values[column]===null?undefined:text(row,column);
 async function ordered(table:number,parent:bigint):Promise<SqliteRow[]>{const selected:SqliteRow[]=[];for(const row of tables[table]!){if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,0);if(integer(row,1)===parent)selected.push(row);}return artifactSqliteOrderedRowsControlled(selected,2,options);}
 async function type(id:bigint):Promise<ValueType>{let lists=0,result:ValueType;for(;;){const row=take(7,id),variant=text(row,1);if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,0);if(variant==="list"){const rows=tables[8]!.filter(row=>integer(row,1)===id);if(rows.length!==1)throw Error("Jack List child differs");const link=take(8,rows[0]!.rowid);id=integer(link,2);lists++;continue;}if(variant==="schema"){const rows=tables[9]!.filter(row=>integer(row,1)===id);if(rows.length!==1)throw Error("Jack Schema body differs");result={kind:"schema",of:text(take(9,rows[0]!.rowid),2)};}else{if(!["boolean","integer","decimal","text","any"].includes(variant))throw Error("Jack ValueType variant differs");result={kind:variant as "boolean"|"integer"|"decimal"|"text"|"any"};}break;}while(lists--){result={kind:"list",of:result};if(++work%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,0);}return result;}
 async function properties(table:number,parent:bigint):Promise<PropertyDef[]>{const result:PropertyDef[]=[];for(const row of await ordered(table,parent)){take(table,row.rowid);const kind=text(row,4);if(kind!=="data"&&kind!=="derived")throw Error("Jack property variant differs");const expr=nullable(row,5);result.push({name:text(row,3),kind,valueType:await type(integer(row,6)),...(expr===undefined?{}:{expr})});}return result;}
 const manifest:Manifest={nodeKinds:[],edgeKinds:[],portKinds:[]};
 for(const row of await ordered(3,document.rowid)){take(3,row.rowid);const ports:string[]=[];for(const port of await ordered(6,row.rowid)){take(6,port.rowid);ports.push(text(port,3));}manifest.nodeKinds.push({name:text(row,3),portKinds:ports,properties:await properties(10,row.rowid)});}
 for(const row of await ordered(4,document.rowid)){take(4,row.rowid);manifest.edgeKinds.push({name:text(row,3),properties:await properties(11,row.rowid)});}
 for(const row of await ordered(5,document.rowid)){take(5,row.rowid);const direction=text(row,4);if(direction!=="in"&&direction!=="out")throw Error("Jack direction differs");manifest.portKinds.push({name:text(row,3),direction,properties:await properties(12,row.rowid)});}
 if(used.size!==workRows(tables))throw Error("Jack has unowned entity rows");const manifestId=nullable(document,3),rootNodeId=nullable(document,4);
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",work,work);
 return{schema:text(document,1),name:text(document,2),query:text(document,5),...(manifestId===undefined?{}:{manifestId}),...(rootNodeId===undefined?{}:{rootNodeId}),manifest,camera:{x:readBinary64(camera,2,FLOATS),y:readBinary64(camera,3,FLOATS),zoom:readBinary64(camera,4,FLOATS)},content:{childId:text(child,2),target:{artifactId:text(child,3),dialect:{artifactKind:text(child,4),standard:text(child,5),subset:text(child,6)}}}};
}
function workRows(tables:readonly(readonly SqliteRow[])[]):number{return tables.reduce((count,rows)=>count+rows.length,0);}
/** 🧭️ Admit only the declared typed parent and complete logical state across structural aliases. */
export async function validateJackSnapshotSqliteDialect(snapshot:JackArtifact,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<readonly never[]>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);if(dialect.artifactKind!=="s.trinity.jack"||dialect.standard!=="1"||dialect.subset!=="*")throw Error("Jack does not own this semantic subset");
 const restored=await jackSnapshotFromSqliteDatabase(database,options),left=await jackSnapshotToSqliteDatabase(snapshot,options),right=await jackSnapshotToSqliteDatabase(restored,options);for(let table=0;table<left.tables.length;table++){const a=left.tables[table]!.rows,b=right.tables[table]!.rows;if(a.length!==b.length)throw Error("Jack logical ownership differs");for(let row=0;row<a.length;row++){if(row%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",row,a.length);const x=a[row]!.values,y=b[row]!.values;if(x.length!==y.length||x.some((value,column)=>value!==y[column]))throw Error("Jack owned identity differs");}}return[];
}
