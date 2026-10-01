/** 🧩️ Literal-table bookkeeping for explicitly authored GLTF typed projections. */
import type {SqliteDatabase,SqliteRow,SqliteValue} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type {ArtifactSqliteOptions} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {sqliteValueByteLength} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteValueBudget} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,ieee754CellByteLength,readBinary64,ieee754IsNull,type Ieee754Cell,type Ieee754Column} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {GLTF_SQLITE_SCHEMA} from "../🗄️.ts";
import type {GltfJson} from "../../🟦️.ts";
import {projectJson,reconstructJson} from "../🧩️extras/🟦️.ts";
export function columns(...indices:number[]):readonly Ieee754Column[]{return indices.map(index=>({index,width:64}))}
export const optionalText=(value:string|undefined):SqliteValue=>value??null;
export function word(value:bigint):[bigint,bigint]{if(typeof value!=="bigint"||value<0n||value>18446744073709551615n)throw Error("GLTF unsigned word differs");return[value>>32n,value&4294967295n]}
export const optionalWord=(value:bigint|undefined):[SqliteValue,SqliteValue]=>value===undefined?[null,null]:word(value);
export function ordinal(value:number):bigint{if(!Number.isSafeInteger(value)||value<0)throw Error("GLTF occurrence ordinal differs");return BigInt(value)}
export class Write{
 private jsonNext=1n;
 private constructor(readonly projection:ArtifactSqliteProjection,readonly options:ArtifactSqliteOptions){}
 static async create(options:ArtifactSqliteOptions):Promise<Write>{return new Write(await ArtifactSqliteProjection.create(GLTF_SQLITE_SCHEMA,options),options)}
 async check(count:number):Promise<void>{this.projection.checkRowsAdditional(count);await artifactSqliteCheckpoint(this.options,"projectSnapshot",0,0,false)}
 async insert(table:string,cells:readonly SqliteValue[],key?:bigint):Promise<bigint>{return this.projection.insert(table,cells,key)}
 async floats(table:string,cells:readonly Ieee754Cell[],positions:readonly Ieee754Column[],key?:bigint):Promise<bigint>{let bytes=8;const selected=new Map(positions.map(value=>[value.index,value.width]));for(const[index,value]of cells.entries()){if(selected.has(index+1))bytes+=value===null?0:ieee754CellByteLength(value as Binary64,64);else bytes+=sqliteValueByteLength(value as SqliteValue)}this.projection.checkValueBytesAdditional(bytes);const encoded=encodeIeee754Cells([0n,...cells],positions,this.options.maxColumns);return this.insert(table,encoded.slice(1),key)}
 async extras(extensions:GltfJson|undefined,extras:GltfJson|undefined):Promise<[SqliteValue,SqliteValue]>{return[await projectJson(this,extensions)??null,await projectJson(this,extras)??null]}
 jsonId():bigint{const key=this.jsonNext++;if(key>9223372036854775807n)throw Error("GLTF extras identity overflow");return key}
 finish():Promise<SqliteDatabase>{return this.projection.finish()}
}
const WIDTHS:readonly(readonly[string,number])[]=[
 ["gltf_document",7],["gltf_asset",8],["gltf_scene",6],["gltf_scene_node",5],["gltf_extension_used",4],["gltf_extension_required",4],["gltf_resolved_buffer",3],["gltf_resolved_byte",4],
 ["gltf_json_value",7],["gltf_json_array_element",4],["gltf_json_object_member",5],
 ["gltf_node",12],["gltf_node_child",5],["gltf_node_matrix",1],["gltf_node_matrix_component",6],["gltf_node_translation",1],["gltf_node_translation_component",6],["gltf_node_rotation",1],["gltf_node_rotation_component",6],["gltf_node_scale",1],["gltf_node_scale_component",6],["gltf_node_weight",6],
 ["gltf_mesh",6],["gltf_mesh_weight",6],["gltf_primitive",11],["gltf_primitive_attribute",6],["gltf_morph_target",3],["gltf_morph_attribute",6],
 ["gltf_buffer",9],["gltf_buffer_view",16],["gltf_accessor",15],["gltf_accessor_max",1],["gltf_accessor_max_component",6],["gltf_accessor_min",1],["gltf_accessor_min_component",6],["gltf_sparse_accessor",3],["gltf_sparse_indices",6],["gltf_sparse_values",5],
 ["gltf_material",20],["gltf_pbr_metallic_roughness",21],["gltf_base_color_texture",7],["gltf_metallic_roughness_texture",7],["gltf_emissive_texture",7],["gltf_normal_texture",10],["gltf_occlusion_texture",10],
 ["gltf_texture",10],["gltf_image",10],["gltf_sampler",14],["gltf_skin",10],["gltf_skin_joint",5],
 ["gltf_animation",6],["gltf_animation_sampler",10],["gltf_animation_channel",7],["gltf_animation_channel_target",6],["gltf_camera",7],["gltf_camera_orthographic",15],["gltf_camera_perspective",15]
];
export class Read{
 private readonly tables=new Map<string,readonly SqliteRow[]>();
 private readonly keys=new Map<string,Map<bigint,SqliteRow>>();
 private readonly groups=new Map<string,Map<bigint,Map<number,SqliteRow>>>();
 private readonly used=new Map<string,Set<bigint>>();
 private visits=0;
 private copied=0;
 private constructor(readonly database:SqliteDatabase,readonly options:ArtifactSqliteOptions){}
 static async create(database:SqliteDatabase,options:ArtifactSqliteOptions):Promise<Read>{await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);await artifactSqliteTables(database,GLTF_SQLITE_SCHEMA,options);const read=new Read(database,options);for(const table of database.tables){const name=table.name.toLowerCase();const width=WIDTHS.find(([key])=>key===name)?.[1];if(width===undefined)throw Error("GLTF undeclared table");read.tables.set(name,table.rows);for(const row of table.rows){if(row.values.length!==width)throw Error("GLTF authored row width differs");await read.checkpoint()}}return read}
 async checkpoint():Promise<void>{if(++this.visits%256===0)await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.visits,0)}
 private reserve(bytes:number):void{this.copied+=bytes;artifactSqliteValueBudget(this.copied,this.options)}
 async consume(table:string,row:SqliteRow):Promise<SqliteRow>{let used=this.used.get(table);if(!used)this.used.set(table,used=new Set());if(row.rowid<1n||artifactSqliteInteger(row,0)!==row.rowid||used.has(row.rowid))throw Error("GLTF exclusive entity ownership differs");used.add(row.rowid);await this.checkpoint();return row}
 private async ensureKeys(table:string):Promise<Map<bigint,SqliteRow>>{let keys=this.keys.get(table);if(!keys){keys=new Map();for(const row of this.tables.get(table)??[]){if(row.rowid<1n||artifactSqliteInteger(row,0)!==row.rowid||keys.has(row.rowid))throw Error("GLTF identities differ");keys.set(row.rowid,row);await this.checkpoint()}this.keys.set(table,keys)}return keys}
 async key(table:string,key:bigint):Promise<SqliteRow>{const row=(await this.ensureKeys(table)).get(key);if(!row)throw Error("GLTF required entity is missing");return this.consume(table,row)}
 async optionalKey(table:string,key:bigint):Promise<SqliteRow|undefined>{return(await this.ensureKeys(table)).has(key)?this.key(table,key):undefined}
 async rows(table:string,ownerColumn:number,owner:bigint,ordinalColumn:number):Promise<SqliteRow[]>{let groups=this.groups.get(table);if(!groups){groups=new Map();for(const row of this.tables.get(table)??[]){const parent=artifactSqliteInteger(row,ownerColumn);const position=artifactSqliteInteger(row,ordinalColumn);if(parent<1n||position<0n||position>BigInt(this.options.maxRows??1000000))throw Error("GLTF relationship owner/ordinal differs");let group=groups.get(parent);if(!group)groups.set(parent,group=new Map());if(group.has(Number(position)))throw Error("GLTF duplicate ordinal");group.set(Number(position),row);await this.checkpoint()}this.groups.set(table,groups)}const group=groups.get(owner)??new Map<number,SqliteRow>();groups.delete(owner);if(group.size>(this.options.maxRows??1000000))throw Error("GLTF row limit");const rows=new Array<SqliteRow>(group.size);for(let index=0;index<rows.length;index++){const row=group.get(index);if(!row)throw Error("GLTF relationship ordinals must be contiguous");rows[index]=await this.consume(table,row)}return rows}
 text(row:SqliteRow,column:number):string{const value=artifactSqliteText(row,column);this.reserve(sqliteValueByteLength(value));return value}
 optionalText(row:SqliteRow,column:number):string|undefined{return row.values[column]===null?undefined:this.text(row,column)}
 word(row:SqliteRow,column:number):bigint{this.reserve(8);const high=artifactSqliteInteger(row,column),low=artifactSqliteInteger(row,column+1);if(high<0n||high>4294967295n||low<0n||low>4294967295n)throw Error("GLTF unsigned words differ");return(high<<32n)|low}
 optionalWord(row:SqliteRow,column:number):bigint|undefined{if(row.values[column]===null&&row.values[column+1]===null)return undefined;return this.word(row,column)}
 integer(row:SqliteRow,column:number):bigint{this.reserve(8);return artifactSqliteInteger(row,column)}
 boolean(row:SqliteRow,column:number):boolean{const value=this.integer(row,column);if(value!==0n&&value!==1n)throw Error("GLTF boolean differs");return value===1n}
 float(row:SqliteRow,column:number,positions:readonly Ieee754Column[]):Binary64{this.reserve(8);return readBinary64(row,column,positions)}
 optionalFloat(row:SqliteRow,column:number,positions:readonly Ieee754Column[]):Binary64|undefined{return ieee754IsNull(row,column,positions)?undefined:this.float(row,column,positions)}
 async json(row:SqliteRow,column:number):Promise<GltfJson|undefined>{if(row.values[column]===null)return undefined;return reconstructJson(this,this.integer(row,column))}
 async finish():Promise<void>{for(const[name,rows]of this.tables)for(const row of rows){if(!this.used.get(name)?.has(row.rowid))throw Error("GLTF unowned entity remains in "+name);await this.checkpoint()}await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.visits,this.visits)}
}
