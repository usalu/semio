/** 🫳️ Explicit DWG row ownership and bounded ordered relationship reconstruction. */
import { artifactSqliteTables,artifactSqliteCheckpoint,type ArtifactSqliteOptions } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { type SqliteDatabase,type SqliteRow } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type { Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { DwgNumberRow } from "../🔢️number/🟦️.ts";

export class DwgReader{
  private readonly used=new Map<string,Set<bigint>>();
  private readonly groups=new Map<string,Map<bigint,SqliteRow[]>>();
  private readonly indices=new Map<string,Map<bigint,SqliteRow>>();
  private completed=0;
  private scanned=0;
  private sealed=false;
  private constructor(private readonly tables:ReadonlyMap<string,readonly SqliteRow[]>,readonly options:ArtifactSqliteOptions,private readonly total:number){}
  static async create(database:SqliteDatabase,sql:string,options:ArtifactSqliteOptions={}):Promise<DwgReader>{
    await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0);
    await artifactSqliteTables(database,sql,options);
    return new DwgReader(new Map(database.tables.map(table=>[table.name.toLowerCase(),table.rows])),options,database.tables.reduce((sum,table)=>sum+table.rows.length,0));
  }
  private source(table:string):readonly SqliteRow[]{const rows=this.tables.get(table);if(rows===undefined)throw new Error("DWG typed table is missing");return rows;}
  private async consume(table:string,row:SqliteRow):Promise<DwgNumberRow>{
    if(this.sealed||row.values[0]!==row.rowid)throw new Error("DWG row identity is mismatched or reader completed");
    let identities=this.used.get(table);if(identities===undefined){identities=new Set();this.used.set(table,identities);}
    if(identities.has(row.rowid))throw new Error("DWG row is multiply owned");
    identities.add(row.rowid);this.completed++;
    if(this.completed%256===0)await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.completed,this.total);
    return new DwgNumberRow(table,row);
  }
  async one(table:string):Promise<DwgNumberRow>{const rows=this.source(table);if(rows.length!==1||rows[0]!.rowid!==1n)throw new Error("DWG component requires one identity-1 row");return this.consume(table,rows[0]!);}
  async component(table:string,identity:bigint):Promise<DwgNumberRow>{
    let index=this.indices.get(table);
    if(index===undefined){
      index=new Map();
      for(const row of this.source(table)){
        if(index.has(row.rowid))throw new Error("DWG component has duplicate row identities");index.set(row.rowid,row);
        if(++this.scanned%256===0)await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.completed,this.total);
      }
      this.indices.set(table,index);
    }
    const row=index.get(identity);if(row===undefined)throw new Error("DWG typed component is missing");return this.consume(table,row);
  }
  async list(table:string,ownerColumn:number,owner:bigint,ordinalColumn:number):Promise<DwgNumberRow[]>{
    const key=table+":"+ownerColumn+":"+ordinalColumn;let groups=this.groups.get(key);
    if(groups===undefined){
      groups=new Map();
      for(const raw of this.source(table)){
        const row=new DwgNumberRow(table,raw);const ownerValue=row.value(ownerColumn);
        if(ownerValue!==null){const identity=row.integer(ownerColumn);let rows=groups.get(identity);if(rows===undefined){rows=[];groups.set(identity,rows);}rows.push(raw);}
        if(++this.scanned%256===0)await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.completed,this.total);
      }
      this.groups.set(key,groups);
    }
    const source=groups.get(owner)??[];groups.delete(owner);
    if(source.length>(this.options.maxRows??1_000_000))throw new Error("DWG ordered row limit");
    const ordered=new Array<DwgNumberRow>(source.length);
    for(const raw of source){
      const ordinal=new DwgNumberRow(table,raw).integer(ordinalColumn);
      if(ordinal<0n||ordinal>=BigInt(ordered.length)||ordered[Number(ordinal)]!==undefined)throw new Error("DWG ordinals must be contiguous");
      ordered[Number(ordinal)]=await this.consume(table,raw);
    }
    return ordered;
  }
  async finish():Promise<void>{
    if(this.sealed)throw new Error("DWG reader is completed");
    for(const[table,rows]of this.tables)if((this.used.get(table)?.size??0)!==rows.length)throw new Error("DWG "+table+" has unowned rows");
    await artifactSqliteCheckpoint(this.options,"reconstructSnapshot",this.total,this.total);this.sealed=true;
  }
}

export function dwgDocument(row:DwgNumberRow):void{if(row.integer(1)!==1n)throw new Error("DWG component has an unknown document");}
export function dwgRange(row:DwgNumberRow,column:number,minimum:bigint,maximum:bigint):number{const value=row.integer(column);if(value<minimum||value>maximum)throw new Error("DWG integer exceeds its native width");return Number(value);}
export function dwgByte(row:DwgNumberRow,column:number):number{return dwgRange(row,column,0n,255n);}
export function dwgWord(row:DwgNumberRow,column:number):number{return dwgRange(row,column,0n,65535n);}
export function dwgSignedByte(row:DwgNumberRow,column:number):number{return dwgRange(row,column,-128n,127n);}
export function dwgSignedWord(row:DwgNumberRow,column:number):number{return dwgRange(row,column,-32768n,32767n);}
export function dwgSignedInteger(row:DwgNumberRow,column:number):number{return dwgRange(row,column,-2147483648n,2147483647n);}
export function dwgUnsigned(row:DwgNumberRow,column:number):number{return dwgRange(row,column,0n,4294967295n);}
export function dwgFullUnsigned(row:DwgNumberRow,high:number,low:number):bigint{return(BigInt(dwgUnsigned(row,high))<<32n)|BigInt(dwgUnsigned(row,low));}
export function dwgOptionalUnsigned(row:DwgNumberRow,high:number,low:number):bigint|undefined{
  const upper=row.value(high),lower=row.value(low);if(upper===null&&lower===null)return undefined;
  if(upper===null||lower===null)throw new Error("DWG optional unsigned identity requires two NULLs or two words");return dwgFullUnsigned(row,high,low);
}
export function dwgOptionalInteger(row:DwgNumberRow,column:number):number|undefined{return row.value(column)===null?undefined:dwgUnsigned(row,column);}
export function dwgOptionalText(row:DwgNumberRow,column:number):string|undefined{return row.value(column)===null?undefined:row.text(column);}
export function dwgOptionalReal(row:DwgNumberRow,column:number):Binary64|undefined{return row.value(column)===null?undefined:row.real(column);}
export function dwgBoolean(row:DwgNumberRow,column:number):boolean{const value=row.integer(column);if(value!==0n&&value!==1n)throw new Error("DWG boolean requires 0 or 1");return value===1n;}
export function dwgEnum<T extends string>(row:DwgNumberRow,column:number,members:readonly T[]):T{const value=row.text(column);if(!members.includes(value as T))throw new Error("DWG enum is unknown");return value as T;}
