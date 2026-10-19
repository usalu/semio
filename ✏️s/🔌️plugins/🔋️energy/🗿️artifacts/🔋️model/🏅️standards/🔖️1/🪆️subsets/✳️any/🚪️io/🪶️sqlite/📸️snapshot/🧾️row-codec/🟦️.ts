/// <reference path="../🗄️.d.ts" />
/** 🧾️ The Energy sqlite row codec shared by the snapshot document and its envelope, systems and schedules tables: IEEE-754 column layout, row writers and the validating reader. */

import sql from "../🗄️.sql" with {type:"text"};
import type { EnergyVertex, EnergyFloat } from "../../../../🧬️schema/📸️snapshot/⚡️model/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteTables, artifactSqliteCheckpoint, artifactSqliteInteger, artifactSqliteText, artifactSqliteOrderedRowsControlled } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import type { ArtifactSqliteOptions } from "@semio-tech/framework/io/sqlite-snapshot/artifact";
import { encodeIeee754Cells, readBinary64, ieee754IsNull } from "@semio-tech/framework/io/sqlite-snapshot/ieee754";
import type { Ieee754Column, Ieee754Cell } from "@semio-tech/framework/io/sqlite-snapshot/ieee754";
import type { SqliteDatabase, SqliteRow, SqliteTable } from "@semio-tech/framework/io/sqlite-snapshot";

export type Options=ArtifactSqliteOptions;

export type Columns=readonly Ieee754Column[];

export function columns(indices:readonly number[]):Columns{return indices.map(index=>({index,width:64}))}

export const DOCUMENT=columns([4,5,6,7,8,9]),MONTH=columns([3,4]),VERTEX=columns([3,4,5]);

export function uint(value:number,maximum=4294967295):bigint{if(!Number.isInteger(value)||value<0||value>maximum)throw Error("Energy unsigned native width differs");return BigInt(value)}

export function bool(value:boolean):bigint{if(typeof value!=="boolean")throw Error("Energy boolean differs");return value?1n:0n}

export function optionalId(value:number|null):bigint|null{return value===null?null:uint(value)}

export function literal<T extends string>(value:string,choices:readonly T[]):T{if(!choices.includes(value as T))throw Error("Energy enum label differs");return value as T}

export async function write(out:ArtifactSqliteProjection,table:string,cells:readonly Ieee754Cell[],floats:Columns,options:Options,key?:bigint):Promise<bigint>{return out.insert(table,encodeIeee754Cells([key??0n,...cells],floats,options.maxColumns).slice(1),key)}

export async function emit(out:ArtifactSqliteProjection,table:string,ordinal:number,cells:readonly Ieee754Cell[],floats:Columns,options:Options,parent=1n):Promise<bigint>{return write(out,table,[parent,BigInt(ordinal),...cells],floats,options)}

export async function vertices(out:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly EnergyVertex[],options:Options):Promise<void>{for(let n=0;n<values.length;n++){const v=values[n]!;if(v.length!==3)throw Error("Energy vertex has exactly3coordinates");await emit(out,table,n,[v[0],v[1],v[2]],VERTEX,options,parent)}}

export async function ids(out:ArtifactSqliteProjection,table:string,parent:bigint,values:readonly number[],options:Options):Promise<void>{for(let n=0;n<values.length;n++)await emit(out,table,n,[uint(values[n]!)],[],options,parent)}

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
