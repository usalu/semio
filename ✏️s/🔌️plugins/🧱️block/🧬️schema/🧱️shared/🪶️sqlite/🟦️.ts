/** 🧱️ Literal Block scalar capture and direct semantic row ownership. */
import {ValueError,sqliteOperation,sqliteValueByteLength,type SqliteOperation,type SqliteDatabase,type SqliteDatabaseOptions,type SqliteValue,type SqliteRow,type SqliteTable} from "../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteRowIndex,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteValueByteLengthControlled} from "../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {parseBinary64,binary64Value,type Binary64} from "../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export const invalid=(message:string):never=>{throw new ValueError("invalidValue","Block "+message)};
export function strict(value:unknown,keys:readonly string[]):Record<string,unknown>{if(value===null||typeof value!=="object"||Array.isArray(value))return invalid("object required");const own=Reflect.ownKeys(value);if(own.length!==keys.length||own.some(key=>typeof key!=="string"||!keys.includes(key)))return invalid("exact owned fields required");return value as Record<string,unknown>}
export function text(value:unknown):string{if(typeof value!=="string")return invalid("well-formed TEXT required");sqliteValueByteLength(value);return value}
export const nullable=<T>(value:unknown,parse:(value:unknown)=>T):T|null=>value===null?null:parse(value);
export function list<T>(value:unknown,parse:(value:unknown)=>T):T[]{if(!Array.isArray(value))return invalid("ordered list required");const keys=Reflect.ownKeys(value);if(keys.length!==value.length+1||keys.some(key=>key!=="length"&&(typeof key!=="string"||!Number.isSafeInteger(Number(key))||Number(key)<0||Number(key)>=value.length||String(Number(key))!==key)))return invalid("exact contiguous array fields required");return value.map(parse)}
export function word(value:unknown):Binary64{strict(value,["bits"]);return parseBinary64(value)}
export function vector(value:unknown):[Binary64,Binary64,Binary64]{const words=list(value,word);if(words.length!==3)return invalid("three exact words required");return[words[0]!,words[1]!,words[2]!]}
export function kind(value:unknown){const r=strict(value,["id","name","label","variant","description","icon","unit"]);return{id:text(r.id),name:text(r.name),label:text(r.label),variant:nullable(r.variant,text),description:text(r.description),icon:nullable(r.icon,text),unit:nullable(r.unit,text)}}
export function attribute(value:unknown){const r=strict(value,["key","value","definition"]);return{key:text(r.key),value:text(r.value),definition:nullable(r.definition,text)}}
export function author(value:unknown){const r=strict(value,["id","name","email"]);return{id:text(r.id),name:text(r.name),email:nullable(r.email,text)}}
export function compatibility(value:unknown){const r=strict(value,["id","source","target","bidirectional"]);if(typeof r.bidirectional!=="boolean")return invalid("Boolean required");return{id:text(r.id),source:text(r.source),target:text(r.target),bidirectional:r.bidirectional}}
export function meta(value:unknown){const r=strict(value,["description"]);return{description:text(r.description)}}
export function camera2d(value:unknown){const r=strict(value,["x","y","zoom"]);return{x:word(r.x),y:word(r.y),zoom:word(r.zoom)}}
export function camera3d(value:unknown){const r=strict(value,["position","target","zoom"]);return{position:vector(r.position),target:vector(r.target),zoom:word(r.zoom)}}
export function wordCells(value:Binary64|null):SqliteValue[]{if(value===null)return[null,null,null];const bits=word(value).bits,query=binary64Value({bits}),kind=Number.isNaN(query)?"nan":query===Infinity?"positiveInfinity":query===-Infinity?"negativeInfinity":"finite";return[kind==="nan"?null:query,BigInt.asIntN(64,bits),kind]}
export function checkedRows(total:number,maximum:number):number{if(!Number.isSafeInteger(total)||total<0||total>maximum)throw new ValueError("workLimit","Block complete row census exceeds caller limit");return total}
export async function ownedText(value:string,operation:SqliteOperation):Promise<string>{const size=await artifactSqliteValueByteLengthControlled(value,operation,"reconstructSnapshot"),backing=operation.allocateBytes(size),encoder=new TextEncoder();let written=0;for(let start=0;start<value.length;){let end=Math.min(start+16384,value.length);if(end<value.length&&value.charCodeAt(end-1)>=0xd800&&value.charCodeAt(end-1)<=0xdbff)end--;const result=encoder.encodeInto(value.slice(start,end),backing.subarray(written));if(result.read!==end-start)throw new ValueError("invariantViolated","Block TEXT copy has incorrect extent");written+=result.written;start=end;if(size>=65536)await artifactSqliteCheckpoint(operation,"reconstructSnapshot",written,size)}if(written!==size)throw new ValueError("invariantViolated","Block TEXT copy has incorrect width");return new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(backing)}

/** 📖️ Validated actual rows retain paid consumed bits and full parent/ordinal indexes. */
export class Reader{
 private constructor(readonly index:ArtifactSqliteRowIndex,readonly document:SqliteRow,readonly operation:SqliteOperation){}
 static async create(input:SqliteDatabase,sql:string,options:SqliteDatabaseOptions={}):Promise<Reader>{const operation=sqliteOperation(options);if(!input||!Array.isArray(input.tables))return invalid("database tables required");const database={tables:input.tables.map((table:SqliteTable)=>({name:table.name,sql:table.sql,rows:table.rows.map((row:SqliteRow)=>({rowid:row.rowid,values:[...row.values]}))}))};const tables=await artifactSqliteTables(database,sql,operation);if(tables[0]!.length!==1)return invalid("one document required");const document=tables[0]![0]!,index=await ArtifactSqliteRowIndex.create(database.tables,operation);return new Reader(index,document,operation)}
 async singleton(name:string):Promise<Cursor>{const rows=await this.index.grouped(name,this.document.rowid,null);if(rows.length!==1)return invalid("one owned singleton required");return this.cursor(rows[0]!,2)}
 async group(name:string,parent=this.document.rowid):Promise<SqliteRow[]>{return this.index.grouped(name,parent,2)}
 async cursor(row:SqliteRow,start:number):Promise<Cursor>{await this.index.take(row);return new Cursor(row,start,this.operation)}
 async finish():Promise<void>{await this.index.finish()}
}
/** 📐️ Exact column cursor rejects optional partial words and scalar disagreement. */
export class Cursor{
 constructor(readonly row:SqliteRow,private position:number,readonly operation:SqliteOperation){}
 private cell():SqliteValue{if(this.position>=this.row.values.length)return invalid("missing owned column");return this.row.values[this.position++]!}
 async text():Promise<string>{return ownedText(text(this.cell()),this.operation)}
 async optionalText():Promise<string|null>{const value=this.cell();return value===null?null:ownedText(text(value),this.operation)}
 boolean():boolean{const value=this.cell();return value===0n?false:value===1n?true:invalid("Boolean INTEGER required")}
 word(optional=false):Binary64|null{const query=this.cell(),signed=this.cell(),kind=this.cell();if(optional&&query===null&&signed===null&&kind===null)return null;if(typeof signed!=="bigint"||signed< -9223372036854775808n||signed>9223372036854775807n)return invalid("signed IEEE word required");const bits=BigInt.asUintN(64,signed),number=binary64Value({bits}),expected=Number.isNaN(number)?"nan":number===Infinity?"positiveInfinity":number===-Infinity?"negativeInfinity":"finite";if(kind!==expected)return invalid("IEEE class disagreement");if(expected==="nan"?query!==null:typeof query!=="number"&&typeof query!=="bigint"||Number(query)!==number)return invalid("IEEE query disagreement");if(typeof query==="bigint"&&(query< -9223372036854775808n||query>9223372036854775807n||!Number.isInteger(number)||BigInt(number)!==query))return invalid("IEEE INTEGER magnitude disagreement");return{bits}}
 requiredWord():Binary64{return this.word()!}
 vector():[Binary64,Binary64,Binary64]{return[this.requiredWord(),this.requiredWord(),this.requiredWord()]}
 done():void{if(this.position!==this.row.values.length)return invalid("extra owned column")}
}
export async function readKind(c:Cursor){const value={id:await c.text(),name:await c.text(),label:await c.text(),variant:await c.optionalText(),description:await c.text(),icon:await c.optionalText(),unit:await c.optionalText()};c.done();return value}
export async function readAttribute(c:Cursor){const value={key:await c.text(),value:await c.text(),definition:await c.optionalText()};c.done();return value}
export async function readAuthor(c:Cursor){const value={id:await c.text(),name:await c.text(),email:await c.optionalText()};c.done();return value}
export async function readCompatibility(c:Cursor){const value={id:await c.text(),source:await c.text(),target:await c.text(),bidirectional:c.boolean()};c.done();return value}
