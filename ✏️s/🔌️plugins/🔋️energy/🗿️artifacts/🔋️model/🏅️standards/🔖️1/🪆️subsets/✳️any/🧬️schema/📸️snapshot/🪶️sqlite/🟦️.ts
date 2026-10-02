/// <reference path="./🗄️.d.ts" />
import sql from "./🗄️.sql" with {type:"text"};
import type {EnergyModelSnapshot} from "../🟦️.ts";
import type {EnergyModel,EnergyVertex,EnergyFloat} from "../⚡️model/🟦️.ts";
import type {ArtifactLink} from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import type {ArtifactRef} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";
import {parseArtifactLink} from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754IsNull,type Ieee754Column,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import type {SqliteDatabase,SqliteRow,SqliteTable,SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {projectEnvelope,reconstructEnvelope} from "./🏘️envelope/🟦️.ts";
import {projectSystems,reconstructSystems} from "./⚙️systems/🟦️.ts";
import {projectSchedules,reconstructSchedules} from "./🗓️schedules/🟦️.ts";
export const ENERGY_MODEL_SQLITE_SCHEMA:string=sql;
export type Options=ArtifactSqliteOptions;
export type Columns=readonly Ieee754Column[];
export function columns(indices:readonly number[]):Columns{return indices.map(index=>({index,width:64}))}
const DOCUMENT=columns([4,5,6,7,8,9]),MONTH=columns([3,4]),VERTEX=columns([3,4,5]);
export function uint(value:number,maximum=4294967295):bigint{if(!Number.isInteger(value)||value<0||value>maximum)throw Error("Energy unsigned native width differs");return BigInt(value)}
export function bool(value:boolean):bigint{if(typeof value!=="boolean")throw Error("Energy boolean differs");return value?1n:0n}
export function optionalId(value:number|null):bigint|null{return value===null?null:uint(value)}
export function literal<T extends string>(value:string,choices:readonly T[]):T{if(!choices.includes(value as T))throw Error("Energy enum label differs");return value as T}
export async function write(out:ArtifactSqliteProjection,table:string,cells:readonly Ieee754Cell[],floats:Columns,options:Options,key?:bigint):Promise<bigint>{return out.insert(table,encodeIeee754Cells([key??0n,...cells],floats,options.maxColumns).slice(1),key)}
export async function emit(out:ArtifactSqliteProjection,table:string,ordinal:number,cells:readonly Ieee754Cell[],floats:Columns,options:Options,parent=1n):Promise<bigint>{return write(out,table,[parent,BigInt(ordinal),...cells],floats,options)}
export async function vertices(out:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly EnergyVertex[],options:Options):Promise<void>{for(let n=0;n<values.length;n++){const v=values[n]!;if(v.length!==3)throw Error("Energy vertex has exactly3coordinates");await emit(out,table,n,[v[0],v[1],v[2]],VERTEX,options,parent)}}
export async function ids(out:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly number[],options:Options):Promise<void>{for(let n=0;n<values.length;n++)await emit(out,table,n,[uint(values[n]!)],[],options,parent)}
async function child(out:ArtifactSqliteProjection,table:string,value:EnergyModelSnapshot["structure"]):Promise<void>{const t=value.target;await out.insert(table,[value.childId,t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset],1n)}
async function link(out:ArtifactSqliteProjection,table:string,value:ArtifactLink):Promise<void>{const v=parseArtifactLink(value),t=v.target,p=v.pin;const payload:SqliteValue[]=p.kind==="head"?[null,null,null,null,null]:p.kind==="checkpoint"?[p.id,null,null,null,null]:[null,p.blob.hash,p.blob.size>>32n,p.blob.size&0xffffffffn,p.blob.mediaType];await out.insert(table,[t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset,v.role,p.kind,...payload],1n)}
/** 📤️ Projects the literal Energy model into authored domain entities. */
export async function energyModelToSqliteDatabase(s:EnergyModelSnapshot,options:Options={}):Promise<SqliteDatabase>{
 await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);const m=s.model,out=await ArtifactSqliteProjection.create(sql,options);await write(out,"energy_document",[s.schema,m.name,m.version,m.site.latitude_deg,m.site.longitude_deg,m.site.elevation_m,m.site.time_zone_hours,m.site.north_axis_deg,m.ground_temperature.deep_c,uint(m.run_period.start_month,255),uint(m.run_period.start_day,255),uint(m.run_period.end_month,255),uint(m.run_period.end_day,255),uint(m.run_period.year,65535)],DOCUMENT,options,1n);await child(out,"energy_structure_child",s.structure);await child(out,"energy_zones_child",s.zones);if(s.referencedModel!==null)await link(out,"energy_referenced_model_link",s.referencedModel);if(s.weatherLink!==null)await link(out,"energy_weather_link",s.weatherLink);
 if(m.ground_temperature.building_surface_c.length!==12||m.ground_temperature.shallow_c.length!==12)throw Error("Energy ground temperatures have exactly12months");for(let n=0;n<12;n++)await emit(out,"energy_ground_month",n,[m.ground_temperature.building_surface_c[n]!,m.ground_temperature.shallow_c[n]!],MONTH,options);
 await projectEnvelope(m,out,options);await projectSystems(m,out,options);await projectSchedules(m.schedules,out,options);return out.finish();
}
export class Row{
 constructor(readonly row:SqliteRow,readonly floats:Columns,base:number){if(row.values.length!==base+2*floats.length||artifactSqliteInteger(row,0)!==row.rowid||row.rowid<=0n)throw Error("Energy row identity or owned column count differs")}
 get id():bigint{return this.row.rowid}
 integer(n:number):bigint{return artifactSqliteInteger(this.row,n)}
 u(n:number,maximum=4294967295n):number{const v=this.integer(n);if(v<0n||v>maximum)throw Error("Energy unsigned native width differs");return Number(v)}
 text(n:number):string{return artifactSqliteText(this.row,n)}
 float(n:number):EnergyFloat{return readBinary64(this.row,n,this.floats)}
 optionalFloat(n:number):EnergyFloat|null{return ieee754IsNull(this.row,n,this.floats)?null:this.float(n)}
 optionalId(n:number):number|null{return this.row.values[n]===null?null:this.u(n)}
 boolean(n:number):boolean{const v=this.integer(n);if(v!==0n&&v!==1n)throw Error("Energy boolean differs");return v===1n}
}
export class Reader{
 private readonly groups=new Map<string,Map<bigint,Row[]>>();private used=0;private visited=0;private constructor(private readonly tables:ReadonlyMap<string,SqliteTable>,readonly options:Options){}
 static async create(database:SqliteDatabase,options:Options):Promise<Reader>{await artifactSqliteTables(database,sql,options);return new Reader(new Map(database.tables.map(table=>[table.name.toLowerCase(),table])),options)}
 async tick():Promise<void>{this.visited++;if(this.visited%256===0)await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.visited,0,true)}
 async rows(table:string,parent:bigint,base:number,floats:Columns):Promise<Row[]>{if(!this.groups.has(table)){const groups=new Map<bigint,SqliteRow[]>(),ids=new Set<bigint>();for(const row of this.tables.get(table)!.rows){await this.tick();new Row(row,floats,base);if(ids.has(row.rowid))throw Error("Energy duplicate surrogate identity");ids.add(row.rowid);const owner=artifactSqliteInteger(row,1);const group=groups.get(owner)??[];group.push(row);groups.set(owner,group)}const ordered=new Map<bigint,Row[]>();for(const[parent,group]of groups)ordered.set(parent,(await artifactSqliteOrderedRowsControlled(group,2,this.options)).map(row=>new Row(row,floats,base)));this.groups.set(table,ordered)}const groups=this.groups.get(table)!,values=groups.get(parent)??[];groups.delete(parent);this.used+=values.length;return values}
 async one(table:string,base:number,floats:Columns,required:boolean):Promise<Row|null>{const rows=this.tables.get(table)!.rows;if(rows.length===0&&!required)return null;if(rows.length!==1||rows[0]!.rowid!==1n)throw Error("Energy singleton presence differs");await this.tick();this.used++;return new Row(rows[0]!,floats,base)}
 async optional(table:string,parent:bigint,base:number,floats:Columns):Promise<Row|null>{const matches=this.tables.get(table)!.rows.filter(row=>row.rowid===parent);if(matches.length>1)throw Error("Energy duplicate optional ownership");if(matches.length===0)return null;await this.tick();this.used++;return new Row(matches[0]!,floats,base)}
 async finish():Promise<void>{if([...this.groups.values()].some(groups=>groups.size!==0)||[...this.tables.values()].reduce((n,table)=>n+table.rows.length,0)!==this.used)throw Error("Energy unowned or multiply owned entities");await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.used,this.used)}
}
export async function readVertices(r:Reader,table:string,parent:bigint):Promise<EnergyVertex[]>{return(await r.rows(table,parent,6,VERTEX)).map(v=>[v.float(3),v.float(4),v.float(5)]as const)}
export async function readIds(r:Reader,table:string,parent:bigint):Promise<number[]>{return(await r.rows(table,parent,4,[])).map(v=>v.u(3))}
function reference(row:Row,start:number):ArtifactRef{return{artifactId:row.text(start),dialect:{artifactKind:row.text(start+1),standard:row.text(start+2),subset:row.text(start+3)}}}
async function readChild(r:Reader,table:string):Promise<EnergyModelSnapshot["structure"]>{const row=(await r.one(table,6,[],true))!;return{childId:row.text(1),target:reference(row,2)}}
async function readLink(r:Reader,table:string):Promise<ArtifactLink|null>{const row=await r.one(table,12,[],false);if(row===null)return null;const kind=row.text(6);let pin:ArtifactLink["pin"];if(kind==="head"){if(row.row.values.slice(7).some(v=>v!==null))throw Error("Energy head pin has extra payload");pin={kind}}else if(kind==="checkpoint"){if(row.row.values.slice(8).some(v=>v!==null))throw Error("Energy checkpoint pin has extra payload");pin={kind,id:row.text(7)}}else if(kind==="snapshot"){if(row.row.values[7]!==null)throw Error("Energy snapshot pin has checkpoint payload");pin={kind,blob:{hash:row.text(8),size:(BigInt(row.u(9))<<32n)|BigInt(row.u(10)),mediaType:row.text(11)}}}else throw Error("Energy pin tag differs");return{target:reference(row,1),role:row.text(5),pin}}
/** 📥️ Reconstructs typed Energy state without engineering inference or native-wire carriers. */
export async function energyModelFromSqliteDatabase(database:SqliteDatabase,options:Options={}):Promise<EnergyModelSnapshot>{
 const r=await Reader.create(database,options),root=(await r.one("energy_document",15,DOCUMENT,true))!,months=await r.rows("energy_ground_month",1n,5,MONTH);if(months.length!==12)throw Error("Energy ground temperatures have exactly12months");const envelope=await reconstructEnvelope(r),systems=await reconstructSystems(r),schedules=await reconstructSchedules(r);
 const model:EnergyModel={name:root.text(2),version:root.text(3),site:{latitude_deg:root.float(4),longitude_deg:root.float(5),elevation_m:root.float(6),time_zone_hours:root.float(7),north_axis_deg:root.float(8)},ground_temperature:{building_surface_c:months.map(v=>v.float(3)),shallow_c:months.map(v=>v.float(4)),deep_c:root.float(9)},run_period:{start_month:root.u(10,255n),start_day:root.u(11,255n),end_month:root.u(12,255n),end_day:root.u(13,255n),year:root.u(14,65535n)},...envelope,...systems,schedules};const result:EnergyModelSnapshot={schema:root.text(1),model,structure:await readChild(r,"energy_structure_child"),zones:await readChild(r,"energy_zones_child"),referencedModel:await readLink(r,"energy_referenced_model_link"),weatherLink:await readLink(r,"energy_weather_link")};await r.finish();return result;
}
