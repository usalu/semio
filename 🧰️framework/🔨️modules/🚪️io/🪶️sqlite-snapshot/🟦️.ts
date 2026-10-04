import { hostContinuations } from "../../⏳️async/🪃️continuation/🟦️.ts";
/** 🪶️ Dependency-free relational SQLite 3 physical engine. @see https://sqlite.org/fileformat.html */
import { ValueError, type ValueRefusalKind } from "../../🌱️value/⚠️refusal/🟦️.ts";
export { ValueError, type ValueRefusalKind } from "../../🌱️value/⚠️refusal/🟦️.ts";

export type SqliteValue = null | bigint | number | string | Uint8Array;
/** 🧾️ Semantic cells in declared column order, including resolved INTEGER PRIMARY KEY aliases. */
export interface SqliteRow { readonly rowid: bigint; readonly values: readonly SqliteValue[] }
/** 🗃️ A handcrafted rowid-table schema and its semantic rows. */
export interface SqliteTable { readonly name: string; readonly sql: string; readonly rows: readonly SqliteRow[] }
/** 🗄️ A standalone relational database containing no opaque snapshot carrier. */
export interface SqliteDatabase { readonly tables: readonly SqliteTable[] }
/** 📈️ Progress at physical database page boundaries. */
export interface SqliteDatabaseProgress { readonly phase: "readPages" | "writePages" | "measureValues" | "decodeValues" | "projectSnapshot" | "reconstructSnapshot" | "parseSchema" | "validateSchema" | "validateRows" | "buildTree" | "indexTables"; readonly completed: number; readonly total: number }
/** 🛑️ Cancellation and aggregate allocation limits for relational transfer. */
export interface SqliteDatabaseOptions {
  readonly signal?: AbortSignal;
  readonly onProgress?: (progress: SqliteDatabaseProgress) => void;
  readonly maxFileBytes?: number;
  readonly maxValueBytes?: number;
  readonly maxAllocationBytes?: number;
  readonly maxSchemaBytes?: number;
  readonly maxRows?: number;
  readonly maxColumns?: number;
  readonly maxTables?: number;
  readonly maxPages?: number;
}

/** 🏗️ Operation-owned cumulative backing admission, independent of literal semantic bytes. */
export class SqliteAllocationControl {
  private admitted = 0;
  private readonly maximum: number;
  constructor(private readonly options: Pick<SqliteDatabaseOptions, "maxAllocationBytes" | "signal"> = {}) {
    this.maximum = options.maxAllocationBytes ?? 512 * 1024 * 1024;
    if (!Number.isSafeInteger(this.maximum) || this.maximum < 0) throw new ValueError("invalidValue", "Invalid SQLite allocation limit");
  }
  /** 📏️ Returns the remaining backing budget across all operation stages. */
  remainingBytes(): number { return this.maximum - this.admitted; }
  /** 🧾️ Reports cumulative concrete byte backing, including retired construction buffers. */
  get ownedBytes(): number { return this.admitted; }
  /** 🧱️ Admits the exact typed-array backing before requesting it from the runtime. */
  allocateBytes(bytes:number):Uint8Array {
    this.admit(bytes);
    try{return new Uint8Array(bytes);}catch(error){if(error instanceof RangeError)throw new ValueError("allocationFailed","SQLite byte backing allocation failed");throw error;}
  }
  /** 🛑️ Rejects before a concrete backing allocation; retirement never refunds admission. */
  admit(bytes: number): void {
    cancelled(this.options);
    if (!Number.isSafeInteger(bytes) || bytes < 0) throw new ValueError("invalidValue", "Invalid SQLite allocation count");
    this.commit(bytes);
  }
  /** 🪆️ Settles the actual child ledger on success and failure without a fresh per-stage allowance. */
  async stage<C extends {readonly ownedBytes:number},T>(create: (maximum:number)=>C, operation:(control:C)=>Promise<T>):Promise<T> {
    cancelled(this.options);
    const control = create(this.remainingBytes());
    try { return await operation(control); } finally { this.commit(control.ownedBytes); }
  }
  private commit(bytes: number): void {
    const total = this.admitted + bytes;
    if (!Number.isSafeInteger(bytes) || bytes < 0 || !Number.isSafeInteger(total) || total > this.maximum) throw new ValueError("ownershipLimit", "SQLite backing allocation limit");
    this.admitted = total;
  }

}

/** 🪆️ Owns one immutable SQLite operation configuration and its cumulative byte backing. */
export class SqliteOperation implements SqliteDatabaseOptions {
  readonly #options:Readonly<SqliteDatabaseOptions>;
  readonly #allocation:SqliteAllocationControl;
  constructor(options:SqliteDatabaseOptions={}) {
    const {signal,onProgress,maxFileBytes,maxValueBytes,maxAllocationBytes,maxSchemaBytes,maxRows,maxColumns,maxTables,maxPages}=options;
    this.#options=Object.freeze({signal,onProgress,maxFileBytes,maxValueBytes,maxAllocationBytes,maxSchemaBytes,maxRows,maxColumns,maxTables,maxPages});
    limits(this.#options);this.#allocation=new SqliteAllocationControl(this.#options);Object.freeze(this);
  }
  get signal(){return this.#options.signal;}
  get onProgress(){return this.#options.onProgress;}
  get maxFileBytes(){return this.#options.maxFileBytes;}
  get maxValueBytes(){return this.#options.maxValueBytes;}
  get maxAllocationBytes(){return this.#options.maxAllocationBytes;}
  get maxSchemaBytes(){return this.#options.maxSchemaBytes;}
  get maxRows(){return this.#options.maxRows;}
  get maxColumns(){return this.#options.maxColumns;}
  get maxTables(){return this.#options.maxTables;}
  get maxPages(){return this.#options.maxPages;}
  get ownedBytes(){return this.#allocation.ownedBytes;}
  remainingBytes():number{return this.#allocation.remainingBytes();}
  allocateBytes(bytes:number):Uint8Array{return this.#allocation.allocateBytes(bytes);}
  /** 🪆️ Settles a concrete child backing ledger against this same caller on every outcome. */
  allocationStage<C extends {readonly ownedBytes:number},T>(create:(maximum:number)=>C,operation:(control:C)=>Promise<T>):Promise<T>{return this.#allocation.stage(create,operation);}
}
/** 🏗️ Retains an explicit operation owner or creates one from its declarative configuration. */
export function sqliteOperation(options:SqliteDatabaseOptions={}):SqliteOperation{return options instanceof SqliteOperation?options:new SqliteOperation(options);}

const PAGE_SIZE = 4096;
const APPLICATION_ID = 0x534d534e;
const encoder = new TextEncoder();
const magic = encoder.encode("SQLite format 3\0");
const INT_MIN = -(1n << 63n);
const INT_MAX = (1n << 63n) - 1n;

function refuse(kind: ValueRefusalKind, reason: string): never { throw new ValueError(kind, `Invalid relational SQLite database: ${reason}`); }
function invalid(reason: string): never { return refuse("invalidValue", reason); }
function cancelled(options: SqliteDatabaseOptions): void {
  if (options.signal?.aborted) {
    throw new ValueError("canceled", "Relational SQLite transfer cancelled");
  }
}
function limits(options: SqliteDatabaseOptions) {
  const value = { file: options.maxFileBytes ?? 272 * 1024 * 1024, data: options.maxValueBytes ?? 256 * 1024 * 1024, allocation: options.maxAllocationBytes ?? 512 * 1024 * 1024, schema: options.maxSchemaBytes ?? 4 * 1024 * 1024, rows: options.maxRows ?? 1_000_000, columns: options.maxColumns ?? 1024, tables: options.maxTables ?? 4096, pages: options.maxPages ?? 1_000_000 };
  for (const limit of Object.values(value)) if (!Number.isSafeInteger(limit) || limit < 0) invalid("resource limit");
  return value;
}
async function cooperate(options: SqliteDatabaseOptions, step: number): Promise<void> {
  cancelled(options);
  if (step % 64 === 0) await hostContinuations.yieldContinuation();
  cancelled(options);
}
async function checkpoint(options: SqliteDatabaseOptions, phase: SqliteDatabaseProgress["phase"], completed: number, total: number): Promise<void> {
  cancelled(options);
  options.onProgress?.({ phase, completed, total });
  await cooperate(options, phase === "measureValues" || phase === "parseSchema" || phase === "validateSchema" || phase === "indexTables" ? 0 : completed);
}
async function text(bytes: Uint8Array,options:SqliteDatabaseOptions): Promise<string> {
  const decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});let out="";
  if(bytes.length>=65536)await checkpoint(options,"decodeValues",0,bytes.length);
  for(let offset=0;offset<bytes.length;offset+=65536){
    const end=Math.min(bytes.length,offset+65536);try{out+=decoder.decode(bytes.subarray(offset,end),{stream:true});}catch{return invalid("UTF-8");}
    if(bytes.length>=65536)await checkpoint(options,"decodeValues",end,bytes.length);
  }
  try{return out+decoder.decode();}catch{return invalid("UTF-8");}
}
function textByteLength(value: string): number {
  let bytes = 0;
  for (let index = 0; index < value.length; index++) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const next = value.charCodeAt(++index);
      if (!(next >= 0xdc00 && next <= 0xdfff)) invalid("UTF-8");
      bytes += 4;
    } else {
      if (code >= 0xdc00 && code <= 0xdfff) invalid("UTF-8");
      bytes += code < 0x80 ? 1 : code < 0x800 ? 2 : 3;
    }
  }
  return bytes;
}
async function textByteLengthControlled(value:string,options:SqliteDatabaseOptions):Promise<number>{
  if(value.length<16384){cancelled(options);return textByteLength(value);}
  await checkpoint(options,"measureValues",0,value.length);let bytes=0,index=0,frontier=16384;
  while(index<value.length){const code=value.charCodeAt(index++);if(code>=0xd800&&code<=0xdbff){const next=value.charCodeAt(index++);if(!(next>=0xdc00&&next<=0xdfff))invalid("UTF-8");bytes+=4;}else{if(code>=0xdc00&&code<=0xdfff)invalid("UTF-8");bytes+=code<0x80?1:code<0x800?2:3;}
    if(index>=frontier){await checkpoint(options,"measureValues",index,value.length);frontier=index+16384;}}
  await checkpoint(options,"measureValues",value.length,value.length);return bytes;
}
async function utf8(value: string,allocation:SqliteOperation,options:SqliteDatabaseOptions): Promise<Uint8Array> {
  const bytes=allocation.allocateBytes(await textByteLengthControlled(value,options));
  let offset=0,completed=0,step=0;
  while(offset<value.length){
    let end=Math.min(value.length,offset+16384);const last=value.charCodeAt(end-1);if(end<value.length&&last>=0xd800&&last<=0xdbff)end--;
    const part=value.slice(offset,end),result=encoder.encodeInto(part,bytes.subarray(completed));
    if(result.read!==part.length)refuse("invariantViolated","UTF-8 backing differs from measured scalar bytes");
    completed+=result.written;offset=end;if(value.length>=16384)await cooperate(options,step++);
  }
  if(completed!==bytes.byteLength)refuse("invariantViolated","UTF-8 backing differs from measured scalar bytes");
  return bytes;
}

/** 🧮️ Count validated semantic scalar bytes without allocating encoded copies. */
export function sqliteValueByteLength(value: SqliteValue): number {
  if (value === null) return 0;
  if (typeof value === "string") return textByteLength(value);
  if (typeof value === "bigint") { if (value < INT_MIN || value > INT_MAX) invalid("signed64 integer"); return 8; }
  if (typeof value === "number") { if (Number.isNaN(value)) invalid("NaN REAL"); return 8; }
  if (!(value instanceof Uint8Array)) invalid("cell type");
  return value.length;
}
function lower(value: string): string { return value.replace(/[A-Z]/g, (letter) => String.fromCharCode(letter.charCodeAt(0) + 32)); }
function varintLength(value:bigint):number{
  let unsigned=BigInt.asUintN(64,value);if(value<0n||unsigned>=1n<<56n)return 9;
  let length=1;while((unsigned>>=7n)>0n)length++;return length;
}
function varint(value: bigint,allocation:SqliteOperation): Uint8Array {
  const unsigned = BigInt.asUintN(64, value);
  if (value < 0n || unsigned >= 1n << 56n) {
    const result = allocation.allocateBytes(9);
    result[8] = Number(unsigned & 255n);
    let remaining = unsigned >> 8n;
    for (let i = 7; i >= 0; i--) { result[i] = Number(remaining & 127n) | 128; remaining >>= 7n; }
    return result;
  }
  let remaining = unsigned;
  const result=allocation.allocateBytes(varintLength(value));
  for(let index=result.length-1;index>=0;index--){result[index]=Number(remaining&127n)|(index===result.length-1?0:128);remaining>>=7n;}
  return result;
}
async function join(parts: readonly Uint8Array[],allocation:SqliteOperation,options:SqliteDatabaseOptions): Promise<Uint8Array> {
  const bytes = allocation.allocateBytes(parts.reduce((sum, part) => sum + part.length, 0));
  let offset = 0,step=0;
  for (const part of parts)for(let start=0;start<part.length;start+=65536){const chunk=part.subarray(start,Math.min(part.length,start+65536));bytes.set(chunk,offset);offset+=chunk.length;if(bytes.length>=65536)await cooperate(options,step++);}
  return bytes;
}
function localPayload(length: number, usable: number): number {
  if (length <= usable - 35) return length;
  const minimum = Math.floor(((usable - 12) * 32) / 255) - 23;
  const candidate = minimum + ((length - minimum) % (usable - 4));
  return candidate <= usable - 35 ? candidate : minimum;
}

interface Token { value: string; folded: string; quoted: boolean; literal: boolean; start: number; end: number }
interface Column { name: string; affinity: "integer" | "real" | "text" | "blob" | "numeric"; notNull: boolean; integerType: boolean }
interface TableSchema { columns: Column[]; alias?: number }
type GrammarWork<T> = Generator<SqliteDatabaseProgress,T>;
function grammarSync<T>(work:GrammarWork<T>):T {let step=work.next();while(!step.done)step=work.next();return step.value;}
async function grammarControlled<T>(work:GrammarWork<T>,options:SqliteDatabaseOptions):Promise<T>{cancelled(options);let step=work.next();while(!step.done){const {phase,completed,total}=step.value;await checkpoint(options,phase,completed,total);step=work.next();}cancelled(options);return step.value;}
function* foldedText(value:string,total=value.length,start=0,phase:SqliteDatabaseProgress["phase"]="validateSchema"):GrammarWork<string>{let out="";for(let offset=0;offset<value.length;offset+=16384){const end=Math.min(value.length,offset+16384);out+=lower(value.slice(offset,end));if(value.length>=16384)yield {phase,completed:start+end,total};}return out;}
function* equalText(actual:string,expected:string):GrammarWork<boolean>{if(actual.length!==expected.length)return false;for(let offset=0;offset<actual.length;offset+=16384){const end=Math.min(actual.length,offset+16384);if(actual.slice(offset,end)!==expected.slice(offset,end))return false;if(actual.length>=16384)yield {phase:"validateSchema",completed:end,total:actual.length};}return true;}
function* measuredText(value:string):GrammarWork<number>{let bytes=0,index=0,frontier=16384;while(index<value.length){const code=value.charCodeAt(index++);if(code>=0xd800&&code<=0xdbff){const next=value.charCodeAt(index++);if(!(next>=0xdc00&&next<=0xdfff))invalid("UTF-8");bytes+=4;}else{if(code>=0xdc00&&code<=0xdfff)invalid("UTF-8");bytes+=code<0x80?1:code<0x800?2:3;}if(index>=frontier){yield {phase:"measureValues",completed:index,total:value.length};frontier=index+16384;}}return bytes;}
function* tokenWork(sql:string):GrammarWork<Token[]>{
 const result:Token[]=[];let offset=0,frontier=16384;
 const digit=(at:number)=>at<sql.length&&sql[at]!>="0"&&sql[at]!<="9";
 function* advance(count=1):GrammarWork<void>{for(let i=0;i<count;i++){offset++;if(offset>=frontier){yield {phase:"parseSchema",completed:offset,total:sql.length};frontier=offset+16384;}}}
 while(offset<sql.length){
  const start=offset,first=sql[offset]!;
  if(/\s/.test(first)){do{yield* advance();}while(offset<sql.length&&/\s/.test(sql[offset]!));continue;}
  if(first==="-"&&sql[offset+1]==="-"){yield* advance(2);while(offset<sql.length&&sql[offset]!=="\n")yield* advance();continue;}
  if(first==="/"&&sql[offset+1]==="*"){yield* advance(2);while(offset<sql.length&&!(sql[offset]==="*"&&sql[offset+1]==="/"))yield* advance();if(offset>=sql.length)invalid("schema SQL token");yield* advance(2);continue;}
  const quoted=first==='"'||first==="`"||first==="[",literal=first==="'";let value="";
  if(quoted||literal){const close=first==="["?"]":first;yield* advance();if(literal)value=first;let closed=false;while(offset<sql.length){const c=sql[offset]!;if(c===close){if(first!=="["&&sql[offset+1]===close){value+=literal?close+close:close;yield* advance(2);continue;}if(literal)value+=close;yield* advance();closed=true;break;}value+=c;yield* advance();}if(!closed)invalid("schema SQL token");}
  else if(/[A-Za-z_]/.test(first)){do{value+=sql[offset]!;yield* advance();}while(offset<sql.length&&/[A-Za-z_0-9]/.test(sql[offset]!));}
  else if(digit(offset)||(first==="."&&digit(offset+1))){while(digit(offset)){value+=sql[offset]!;yield* advance();}if(sql[offset]==="."){value+=".";yield* advance();while(digit(offset)){value+=sql[offset]!;yield* advance();}}if(sql[offset]==="e"||sql[offset]==="E"){let next=offset+1;if(sql[next]==="+"||sql[next]==="-")next++;if(digit(next)){while(offset<next){value+=sql[offset]!;yield* advance();}while(digit(offset)){value+=sql[offset]!;yield* advance();}}}}
  else if("(),;.+-*/%=<>!|&~".includes(first)){value=first;yield* advance();}
  else invalid("schema SQL token");
  result.push({value,folded:yield* foldedText(value,sql.length,start),quoted,literal,start,end:offset});
  if(result.length%256===0)yield {phase:"parseSchema",completed:offset,total:sql.length};
 }
 if(result.at(-1)?.value===";")result.pop();return result;
}
function* tokenFrontier(completed:number,total:number):GrammarWork<void>{if(completed%256===0)yield {phase:"validateSchema",completed,total};}
function* copiedTokens(source:readonly Token[],start:number,end=source.length):GrammarWork<Token[]>{const out:Token[]=[];for(let index=start;index<end;index++){out.push(source[index]!);yield* tokenFrontier(index-start+1,end-start);}return out;}
function* affinityWork(typeName:string):GrammarWork<Column["affinity"]>{let int=false,text=false,blob=false,real=false,tail="";for(let offset=0;offset<typeName.length;offset+=16384){const chunk=tail+typeName.slice(offset,offset+16384);int||=chunk.includes("int");text||=/char|clob|text/.test(chunk);blob||=chunk.includes("blob");real||=/real|floa|doub/.test(chunk);tail=chunk.slice(-3);if(typeName.length>=16384)yield {phase:"validateSchema",completed:Math.min(typeName.length,offset+16384),total:typeName.length};}return int?"integer":text?"text":typeName===""||blob?"blob":real?"real":"numeric";}
function keyword(token: Token | undefined, word: string): boolean { return token !== undefined && !token.quoted && !token.literal && token.folded === word; }
function identifier(token: Token | undefined): string {
  if (!token || token.literal || (!token.quoted && !/^[A-Za-z_]/.test(token.value))) invalid("schema identifier");
  return token.value;
}
function* topLevelPair(part: readonly Token[], first: string, second: string): GrammarWork<boolean> {
  let depth = 0;
  for (let i = 0; i < part.length; i++) {
    yield* tokenFrontier(i+1,part.length);
    const token = part[i]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth === 0 && keyword(token, first) && keyword(part[i + 1], second)) return true;
  }
  return false;
}
function* schemaWork(name: string, sql: string, maxColumns: number, prepared?:readonly Token[]): GrammarWork<TableSchema> {
  const all = prepared ?? (yield* tokenWork(sql));
  identifier(all[2]);
  if (!keyword(all[0], "create") || !keyword(all[1], "table") || !(yield* equalText(all[2]!.folded,yield* foldedText(name))) || all[3]?.value !== "(") invalid("schema CREATE TABLE");
  for(let index=0;index<all.length;index++){yield* tokenFrontier(index+1,all.length);if(["unique","without","autoincrement","generated","desc"].some(word=>keyword(all[index],word)))invalid("unsupported indexed or generated schema");}
  const groups: Token[][] = [];
  let group: Token[] = [];
  let depth = 1;
  let offset = 4;
  for (; offset < all.length; offset++) {
    yield* tokenFrontier(offset+1,all.length);
    const token = all[offset]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    if (!token.quoted && !token.literal && token.value === ")") depth--;
    if (depth === 0) { if (group.length) groups.push(group); break; }
    if (depth === 1 && token.value === "," && !token.quoted && !token.literal) { groups.push(group); group = []; }
    else group.push(token);
  }
  if (depth !== 0 || offset !== all.length - 1 || !groups.length) invalid("schema parentheses");
  const columns: Column[] = [];
  let alias: number | undefined;
  let primaryColumn: string | undefined;
  const columnNames=new Set<string>();
  const constraints = ["primary", "not", "null", "check", "default", "collate", "references", "constraint"];
  let parsedGroups=0;
  for (const source of groups) {
    yield* tokenFrontier(++parsedGroups,groups.length);
    let part = source;
    if (keyword(part[0], "constraint")) { identifier(part[1]); part = yield* copiedTokens(part,2); }
    if (keyword(part[0], "foreign") || keyword(part[0], "check")) continue;
    if (keyword(part[0], "primary")) {
      if (!keyword(part[1], "key") || part[2]?.value !== "(" || part[4]?.value !== ")" || part.length !== 5 || primaryColumn !== undefined) invalid("unsupported primary key schema");
      primaryColumn = identifier(part[3]);
      continue;
    }
    const columnName = identifier(part[0]);
    if(columnNames.has(part[0]!.folded))invalid("schema duplicate column");columnNames.add(part[0]!.folded);
    let boundary=-1;for(let index=1;index<part.length;index++){yield* tokenFrontier(index+1,part.length);if(constraints.some(word=>keyword(part[index],word))){boundary=index;break;}}
    const typeTokens = yield* copiedTokens(part,1,boundary<0?part.length:boundary);
    let typeName="";for(let index=0;index<typeTokens.length;index++){yield* tokenFrontier(index+1,typeTokens.length);typeName+=(index?" ":"")+typeTokens[index]!.folded;}
    const affinity=yield* affinityWork(typeName);
    const notNull = yield* topLevelPair(part, "not", "null");
    const primary = yield* topLevelPair(part, "primary", "key");
    if (primary) {
      if (typeTokens.length !== 1 || typeTokens[0]!.quoted || !keyword(typeTokens[0], "integer") || alias !== undefined || primaryColumn !== undefined) invalid("unsupported non-INTEGER primary key");
      alias = columns.length;
    }
    columns.push({ name: columnName, affinity, notNull, integerType: typeTokens.length === 1 && keyword(typeTokens[0], "integer") });
    if (columns.length > maxColumns) refuse("workLimit", "column limit");
  }
  if (primaryColumn !== undefined) {
    const primaryName=yield* foldedText(primaryColumn);let index=-1;for(let at=0;at<columns.length;at++){yield* tokenFrontier(at+1,columns.length);if(yield* equalText(yield* foldedText(columns[at]!.name),primaryName)){index=at;break;}}
    if (index < 0 || alias !== undefined || !columns[index]!.integerType) invalid("unsupported non-INTEGER primary key");
    alias = index;
  }
  if (!columns.length) invalid("schema empty table");
  return { columns, alias };
}

async function schemaControlled(name:string,sql:string,maxColumns:number,options:SqliteDatabaseOptions):Promise<TableSchema>{return grammarControlled(schemaWork(name,sql,maxColumns),options);}
/** 🔤️ Canonicalize a literal SQLite identifier using bounded ASCII case folding. */
export async function sqliteIdentifierKeyControlled(value:string,options:SqliteDatabaseOptions):Promise<string>{const operation=sqliteOperation(options);return grammarControlled(foldedText(value,value.length,0,"indexTables"),operation);}
interface ParsedTableSchema { tokens:readonly Token[]; columns:TableSchema }
function* parseSchemaWork(sql:string,options:SqliteDatabaseOptions,prepared?:Map<string,ParsedTableSchema>):GrammarWork<SqliteDatabase>{
 const limit=limits(options);if(sql.length>limit.schema||(yield* measuredText(sql))>limit.schema)refuse("ownershipLimit","schema limit");
 const statements:Token[][]=[];let statement:Token[]=[],units=0;
 for(const token of yield* tokenWork(sql)){yield* tokenFrontier(++units,sql.length);if(token.value===";"&&!token.quoted&&!token.literal){if(statement.length)statements.push(statement);statement=[];}else statement.push(token);}
 if(statement.length)statements.push(statement);if(statements.length>limit.tables)refuse("workLimit","table limit");
 const names=new Set<string>(),tables:SqliteTable[]=[];let bytes=0;
 for(const statement of statements){const name=identifier(statement[2]),source=sql.slice(statement[0]!.start,statement.at(-1)!.end),folded=statement[2]!.folded;
  if(names.has(folded)||folded.startsWith("sqlite_"))invalid("schema table name");names.add(folded);bytes+=(yield* measuredText(name))+(yield* measuredText(source));if(bytes>limit.schema)refuse("ownershipLimit","schema limit");const columns=yield* schemaWork(name,source,limit.columns,statement);prepared?.set(folded,{tokens:statement,columns});tables.push({name,sql:source,rows:[]});yield* tokenFrontier(tables.length,statements.length);
 }return {tables};
}
/** 🏛️ Parse handcrafted CREATE TABLE statements synchronously without filesystem access. */
export function parseSqliteDatabaseSchema(sql:string,options:SqliteDatabaseOptions={}):SqliteDatabase{cancelled(options);return grammarSync(parseSchemaWork(sql,options));}
/** 🧵️ Parse the same handcrafted schema with bounded operation-owned grammar checkpoints. */
export async function parseSqliteDatabaseSchemaControlled(sql:string,options:SqliteDatabaseOptions={}):Promise<SqliteDatabase>{const operation=sqliteOperation(options);return grammarControlled(parseSchemaWork(sql,operation),operation);}

function* validateSchemaWork(database:SqliteDatabase,sql:string,options:SqliteDatabaseOptions):GrammarWork<void>{
 const limit=limits(options),expectedNames=new Map<string,ParsedTableSchema>(),expected=yield* parseSchemaWork(sql,options,expectedNames);if(database.tables.length!==expected.tables.length)invalid("artifact table count");
 const names=new Set<string>();let rows=0,total=0,schemaBytes=0,tables=0;for(const table of database.tables){total+=table.rows.length;if(total>limit.rows)refuse("workLimit","row limit");yield* tokenFrontier(++tables,database.tables.length);}
 for(const table of database.tables){const name=yield* foldedText(table.name);if(names.has(name))invalid("artifact duplicate table");names.add(name);yield* tokenFrontier(names.size,database.tables.length);const definition=expectedNames.get(name);if(!definition)invalid("artifact unknown table");
  schemaBytes+=(yield* measuredText(table.name))+(yield* measuredText(table.sql));if(schemaBytes>limit.schema)refuse("ownershipLimit","schema limit");const received=yield* tokenWork(table.sql);if(!(yield* equivalentSchema(received,definition.tokens)))invalid("artifact table schema mismatch");const columns=definition.columns,ids=new Set<bigint>();
  for(const row of table.rows){if(++rows>limit.rows)refuse("workLimit","row limit");if(row.values.length!==columns.columns.length)invalid("column count");if(typeof row.rowid!=="bigint"||row.rowid<INT_MIN||row.rowid>INT_MAX||ids.has(row.rowid))invalid("rowid range or duplicate");ids.add(row.rowid);if(columns.alias!==undefined&&row.values[columns.alias]!==row.rowid)invalid("primary key alias");if(rows%256===0)yield {phase:"validateRows",completed:rows,total};}
 }
}
/** 🔎️ Validate the handcrafted semantic schema synchronously, including row identities. */
export function validateSqliteDatabaseSchema(database:SqliteDatabase,sql:string,options:SqliteDatabaseOptions={}):void{cancelled(options);grammarSync(validateSchemaWork(database,sql,options));}
/** 🧵️ Validate the same semantic schema and row corpus with bounded operation checkpoints. */
export async function validateSqliteDatabaseSchemaControlled(database:SqliteDatabase,sql:string,options:SqliteDatabaseOptions={}):Promise<void>{const operation=sqliteOperation(options);await grammarControlled(validateSchemaWork(database,sql,operation),operation);}

function* equivalentSchema(received:readonly Token[], trusted:readonly Token[]): GrammarWork<boolean> {
  if (trusted.length !== received.length) return false;
  const identifiers = new Set<number>([2]);
  const columnNames = new Set<string>();
  let depth = 1;
  let groupStart = true;
  for (let index = 4; index < trusted.length; index++) {
    yield* tokenFrontier(index+1,trusted.length);
    const token = trusted[index]!;
    if (depth === 1 && groupStart) {
      if (keyword(token, "constraint")) { identifiers.add(index + 1); index++; continue; }
      groupStart = false;
      if (!["primary", "foreign", "check"].some((word) => keyword(token, word))) { identifiers.add(index); columnNames.add(token.folded); }
    }
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth === 1 && token.value === "," && !token.quoted && !token.literal) groupStart = true;
    if (keyword(token, "references")) {
      identifiers.add(index + 1);
      if (trusted[index + 2]?.value === "(" && !trusted[index + 2]?.quoted) {
        let consumed=0;const maximum=Math.floor((trusted.length-index-3)/2);for(let next=index+3;next<trusted.length&&trusted[next]!.value!==")";next+=2){identifiers.add(next);yield* tokenFrontier(++consumed,maximum);}
      }
    }
  }
  depth = 0;
  const syntax = ["not", "null", "is", "in", "between", "and", "or", "like", "glob", "match", "regexp", "escape", "case", "when", "then", "else", "end", "collate", "as"];
  for (let index = 0; index < trusted.length; index++) {
    yield* tokenFrontier(index+1,trusted.length);
    const token = trusted[index]!;
    if (!token.quoted && !token.literal && token.value === "(") depth++;
    else if (!token.quoted && !token.literal && token.value === ")") depth--;
    else if (depth > 1 && !token.literal && columnNames.has(token.folded) && (token.quoted || !syntax.some((word) => keyword(token, word)))) identifiers.add(index);
    if (keyword(token, "collate")) identifiers.add(index + 1);
  }
  for(let index=0;index<trusted.length;index++){yield* tokenFrontier(index+1,trusted.length);const token=trusted[index]!,other=received[index]!;
    if(token.literal||other.literal){if(token.literal!==other.literal||!(yield* equalText(token.value,other.value)))return false;}
    else if(!(yield* equalText(token.folded,other.folded))||(!identifiers.has(index)&&token.quoted!==other.quoted))return false;
  }return true;
}

async function encodedField(value: SqliteValue,allocation:SqliteOperation,options:SqliteDatabaseOptions): Promise<{ serial: bigint; data: Uint8Array }> {
  if (value === null) return { serial: 0n, data: allocation.allocateBytes(0) };
  if (typeof value === "bigint") {
    if (value < INT_MIN || value > INT_MAX) invalid("signed64 integer");
    if (value === 0n || value === 1n) return { serial: value + 8n, data: allocation.allocateBytes(0) };
    const widths = [1, 2, 3, 4, 6, 8];
    const width = widths.find((size) => value >= -(1n << BigInt(size * 8 - 1)) && value < 1n << BigInt(size * 8 - 1))!;
    const bytes = allocation.allocateBytes(width);
    let unsigned = BigInt.asUintN(width * 8, value);
    for (let i = width - 1; i >= 0; i--) { bytes[i] = Number(unsigned & 255n); unsigned >>= 8n; }
    return { serial: BigInt(widths.indexOf(width) + 1), data: bytes };
  }
  if (typeof value === "number") {
    if (Number.isNaN(value)) invalid("NaN REAL");
    const data = allocation.allocateBytes(8);
    new DataView(data.buffer).setFloat64(0, value);
    return { serial: 7n, data };
  }
  if (typeof value === "string") { const data = await utf8(value,allocation,options); return { serial: BigInt(data.length) * 2n + 13n, data }; }
  if (!(value instanceof Uint8Array)) invalid("cell type");
  return { serial: BigInt(value.length) * 2n + 12n, data: value };
}
async function encodedRecord(fields: readonly { serial: bigint; data: Uint8Array }[],allocation:SqliteOperation,options:SqliteDatabaseOptions): Promise<Uint8Array> {
  const serials = await join(fields.map((field) => varint(field.serial,allocation)),allocation,options);
  let length = serials.length + 1;
  while (length !== serials.length + varintLength(BigInt(length))) length = serials.length + varintLength(BigInt(length));
  return join([varint(BigInt(length),allocation), serials, ...fields.map((field) => field.data)],allocation,options);
}
interface Cell { key: bigint; bytes: Uint8Array }
interface TreeNode { page: number; maximum: bigint }

async function preflightDatabase(database: SqliteDatabase, options: SqliteDatabaseOptions, limit: ReturnType<typeof limits>): Promise<TableSchema[]> {
  const definitions: TableSchema[] = [];
  const names = new Set<string>();
  let dataBytes = 0;
  let schemaBytes = 0;
  let rows = 0;
  for (const table of database.tables) {
    const name=await grammarControlled(foldedText(table.name),options);
    if (!table.name || name.startsWith("sqlite_") || names.has(name)) invalid("schema table name");
    names.add(name);
    schemaBytes += await textByteLengthControlled(table.sql,options) + await textByteLengthControlled(table.name,options);
    if (schemaBytes > limit.schema) refuse("ownershipLimit", "schema limit");
    const definition = await schemaControlled(table.name, table.sql, limit.columns,options);
    definitions.push(definition);
    let previous: bigint | undefined;
    for (const row of table.rows) {
      if (++rows > limit.rows) refuse("workLimit", "row limit");
      if (typeof row.rowid !== "bigint" || row.rowid < INT_MIN || row.rowid > INT_MAX || (previous !== undefined && row.rowid <= previous)) invalid("rowid order or range");
      previous = row.rowid;
      if (row.values.length !== definition.columns.length) invalid("column count");
      for (let index = 0; index < row.values.length; index++) {
        const value = row.values[index]!;
        if (index === definition.alias && (typeof value !== "bigint" || value !== row.rowid)) invalid("primary key alias");
        if (value === null && definition.columns[index]!.notNull) invalid("NOT NULL cell");
        dataBytes += typeof value==="string"?await textByteLengthControlled(value,options):sqliteValueByteLength(value);
        if (dataBytes > limit.data) refuse("ownershipLimit", "value limit");
      }
      await cooperate(options, rows);
    }
    await cooperate(options, definitions.length);
  }
  return definitions;
}

/** 📤️ Export handcrafted semantic tables as an integrity-valid standalone SQLite file. */
export async function exportSqliteDatabase(value: SqliteDatabase, options: SqliteDatabaseOptions = {}): Promise<Uint8Array> {
  const allocation=sqliteOperation(options);options=allocation;
  cancelled(options);
  const limit = limits(options);
  if (value.tables.length > limit.tables) refuse("workLimit", "table limit");
  const definitions = await preflightDatabase(value, options, limit);
  const pages: Uint8Array[] = [];
  const allocate = (): number => {
    if (pages.length + 1 > limit.pages) refuse("workLimit", "page or file limit");
    if ((pages.length + 1) * PAGE_SIZE > limit.file) refuse("ownershipLimit", "page or file limit");
    pages.push(allocation.allocateBytes(PAGE_SIZE));
    return pages.length;
  };
  allocate();
  const roots:number[]=[];for(let index=0;index<value.tables.length;index++){roots.push(allocate());await cooperate(options,index+1);}
  let rowCount = 0;
  const cell = async (rowid: bigint, payload: Uint8Array): Promise<Cell> => {
    const local = localPayload(payload.length, PAGE_SIZE);
    let first = 0;
    let previous = 0;
    for (let offset = local; offset < payload.length; offset += PAGE_SIZE - 4) {
      const page = allocate();
      if (!first) first = page;
      if (previous) new DataView(pages[previous - 1]!.buffer).setUint32(0, page);
      pages[page - 1]!.set(payload.subarray(offset, offset + PAGE_SIZE - 4), 4);
      previous = page;
      await cooperate(options, page);
    }
    const pointer = allocation.allocateBytes(first ? 4 : 0);
    if (first) new DataView(pointer.buffer).setUint32(0, first);
    return { key: rowid, bytes: await join([varint(BigInt(payload.length),allocation), varint(rowid,allocation), payload.subarray(0, local), pointer],allocation,options) };
  };
  const writeLeaf = (page: number, cells: readonly Cell[]): TreeNode => {
    const bytes = pages[page - 1]!;
    const view = new DataView(bytes.buffer);
    const header = page === 1 ? 100 : 0;
    bytes[header] = 13;
    view.setUint16(header + 3, cells.length);
    let cursor = PAGE_SIZE;
    for (let i = 0; i < cells.length; i++) { cursor -= cells[i]!.bytes.length; bytes.set(cells[i]!.bytes, cursor); view.setUint16(header + 8 + i * 2, cursor); }
    view.setUint16(header + 5, cursor);
    return { page, maximum: cells.at(-1)?.key ?? 0n };
  };
  const writeInterior = (page: number, children: readonly TreeNode[]): TreeNode => {
    const bytes = pages[page - 1]!;
    const view = new DataView(bytes.buffer);
    const header = page === 1 ? 100 : 0;
    bytes[header] = 5;
    view.setUint16(header + 3, children.length - 1);
    view.setUint32(header + 8, children.at(-1)!.page);
    let cursor = PAGE_SIZE;
    for (let i = 0; i < children.length - 1; i++) {
      const key = varint(children[i]!.maximum,allocation);
      cursor -= key.length + 4;
      view.setUint32(cursor, children[i]!.page);
      bytes.set(key, cursor + 4);
      view.setUint16(header + 12 + i * 2, cursor);
    }
    view.setUint16(header + 5, cursor);
    return { page, maximum: children.at(-1)!.maximum };
  };
  const tree = async (root: number, cells: readonly Cell[]): Promise<void> => {
    let total=0;for(let index=0;index<cells.length;index++){total+=cells[index]!.bytes.length+2;if((index+1)%256===0)await checkpoint(options,"buildTree",index+1,cells.length);}
    if (total <= PAGE_SIZE - 8 - (root === 1 ? 100 : 0)) { writeLeaf(root, cells); return; }
    let nodes: TreeNode[] = [];
    let group: Cell[] = [];
    let used = 8;
    let grouped=0;
    for (const item of cells) {
      if(++grouped%256===0)await checkpoint(options,"buildTree",grouped,cells.length);
      if (used + item.bytes.length + 2 > PAGE_SIZE) { nodes.push(writeLeaf(allocate(), group)); group = []; used = 8; }
      group.push(item);
      used += item.bytes.length + 2;
    }
    if (group.length) nodes.push(writeLeaf(allocate(), group));
    const rootBranches = Math.floor((PAGE_SIZE - 12 - (root === 1 ? 100 : 0)) / 15) + 1;
    const branches = Math.floor((PAGE_SIZE - 12) / 15) + 1;
    while (nodes.length > rootBranches) {
      const next: TreeNode[] = [];
      let offset = 0;
      while (offset < nodes.length) {
        let count = Math.min(branches, nodes.length - offset);
        if (nodes.length - offset - count === 1) count--;
        next.push(writeInterior(allocate(), nodes.slice(offset, offset + count)));
        offset += count;
        await cooperate(options, pages.length);
      }
      nodes = next;
    }
    writeInterior(root, nodes);
  };
  const schemaCells: Cell[] = [];
  for (let tableIndex = 0; tableIndex < value.tables.length; tableIndex++) {
    const table = value.tables[tableIndex]!;
    const definition = definitions[tableIndex]!;
    const cells: Cell[] = [];
    for (const row of table.rows) {
      rowCount++;
      const fields:{serial:bigint;data:Uint8Array}[]=[];for(let index=0;index<row.values.length;index++)fields.push(await encodedField(index===definition.alias?null:row.values[index]!,allocation,options));
      cells.push(await cell(row.rowid, await encodedRecord(fields,allocation,options)));
      await cooperate(options, rowCount);
    }
    await tree(roots[tableIndex]!, cells);
    const fields:{serial:bigint;data:Uint8Array}[]=[];for(const value of ["table",table.name,table.name,BigInt(roots[tableIndex]!),table.sql])fields.push(await encodedField(value,allocation,options));
    schemaCells.push(await cell(BigInt(tableIndex + 1), await encodedRecord(fields,allocation,options)));
  }
  await tree(1, schemaCells);
  const first = pages[0]!;
  const header = new DataView(first.buffer);
  first.set(magic);
  header.setUint16(16, PAGE_SIZE);
  first.set([1, 1, 0, 64, 32, 32], 18);
  for (const [offset, integer] of [[24, 1], [28, pages.length], [40, 1], [44, 4], [56, 1], [60, 1], [68, APPLICATION_ID], [92, 1], [96, 3046000]]) header.setUint32(offset!, integer!);
  const bytes = allocation.allocateBytes(pages.length * PAGE_SIZE);
  for (let i = 0; i < pages.length; i++) { bytes.set(pages[i]!, i * PAGE_SIZE); await checkpoint(options, "writePages", i + 1, pages.length); }
  return bytes;
}

class Reader {
  readonly view: DataView;
  readonly pageSize: number;
  readonly usable: number;
  readonly pages: number;
  private readonly seen:Uint8Array;
  private claimed=0;
  readonly limit: ReturnType<typeof limits>;
  dataBytes = 0;
  schemaBytes = 0;
  rowCount = 0;
  constructor(readonly bytes: Uint8Array, readonly options: SqliteOperation) {
    cancelled(options);
    this.limit = limits(options);
    if (bytes.length > this.limit.file) refuse("ownershipLimit", "file limit");
    if (bytes.length < 100 || !magic.every((byte, i) => bytes[i] === byte)) invalid("file header");
    this.view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const size = this.view.getUint16(16);
    this.pageSize = size === 1 ? 65536 : size;
    if (this.pageSize < 512 || this.pageSize > 65536 || (this.pageSize & (this.pageSize - 1)) !== 0) invalid("page size");
    if (bytes.length % this.pageSize !== 0) invalid("truncated page");
    this.pages = bytes.length / this.pageSize;
    if (this.pages > this.limit.pages) refuse("workLimit", "page limit");
    this.usable = this.pageSize - bytes[20]!;
    if (this.usable < 480 || bytes[18] !== 1 || bytes[19] !== 1 || bytes[21] !== 64 || bytes[22] !== 32 || bytes[23] !== 32) invalid("page or journal header");
    if (this.view.getUint32(44) !== 4 || this.view.getUint32(56) !== 1 || this.view.getUint32(60) !== 1 || this.view.getUint32(68) !== APPLICATION_ID) invalid("identity, version, schema format or UTF-8 encoding");
    if (bytes.subarray(72, 92).some((byte) => byte !== 0)) invalid("reserved header bytes");
    const declared = this.view.getUint32(28);
    if (declared && this.view.getUint32(24) === this.view.getUint32(92) && declared !== this.pages) invalid("database page count");
    this.seen=options.allocateBytes(this.pages);
  }
  claim(page: number): number {
    if (!Number.isSafeInteger(page) || page < 1 || page > this.pages) invalid("page reference");
    if (this.seen[page-1]) invalid("cyclic or aliased page");
    this.seen[page-1]=1;this.claimed++;
    return (page - 1) * this.pageSize;
  }
  variable(bytes: Uint8Array, offset: number, end: number): { value: bigint; next: number } {
    let value = 0n;
    for (let i = 0; i < 9; i++) {
      if (offset >= end) invalid("truncated varint");
      const byte = bytes[offset++]!;
      value = (value << BigInt(i === 8 ? 8 : 7)) | BigInt(i === 8 ? byte : byte & 127);
      if (i === 8 || byte < 128) return { value, next: offset };
    }
    return invalid("varint");
  }
  integer(value: bigint): number { if (value > BigInt(Number.MAX_SAFE_INTEGER)) refuse("ownershipLimit", "integer limit"); return Number(value); }
  async fields(payload: Uint8Array, count: number): Promise<SqliteValue[]> {
    const header = this.variable(payload, 0, payload.length);
    const headerEnd = this.integer(header.value);
    if (headerEnd < header.next || headerEnd > payload.length) invalid("record header");
    const serials: number[] = [];
    let offset = header.next;
    while (offset < headerEnd) {
      if (serials.length >= count) invalid("column count");
      const serial = this.variable(payload, offset, headerEnd);
      serials.push(this.integer(serial.value));
      offset = serial.next;
    }
    if (serials.length !== count) invalid("column count");
    offset = headerEnd;
    const values: SqliteValue[] = [];
    for (const serial of serials) {
      const length = serial >= 12 ? Math.floor((serial - 12) / 2) : [0, 1, 2, 3, 4, 6, 8, 8, 0, 0][serial];
      if (length === undefined || offset + length > payload.length) invalid("serial type or cell length");
      if (serial === 0) values.push(null);
      else if (serial === 8 || serial === 9) values.push(BigInt(serial - 8));
      else if (serial >= 12) values.push(serial % 2 ? await text(payload.subarray(offset, offset + length),this.options) : payload.subarray(offset, offset + length));
      else if (serial === 7) {
        const value = new DataView(payload.buffer, payload.byteOffset + offset, 8).getFloat64(0);
        if (Number.isNaN(value)) invalid("NaN REAL");
        values.push(value);
      } else {
        let value = 0n;
        for (let i = 0; i < length; i++) value = (value << 8n) | BigInt(payload[offset + i]!);
        if ((payload[offset]! & 128) !== 0) value -= 1n << BigInt(length * 8);
        values.push(value);
      }
      offset += length;
    }
    if (offset !== payload.length) invalid("record trailing bytes");
    return values;
  }
  async *table(root: number, count: number, schemaRows: boolean): AsyncGenerator<SqliteRow> {
    const pending: { page: number; lower?: bigint; upper?: bigint }[] = [{ page: root }];
    while (pending.length) {
      const node = pending.pop()!;
      const start = this.claim(node.page);
      await checkpoint(this.options, "readPages", this.claimed, this.pages);
      const end = start + this.usable;
      const header = start + (node.page === 1 ? 100 : 0);
      const type = this.bytes[header];
      if (type !== 5 && type !== 13) invalid("unsupported index or table B-tree page");
      const headerSize = type === 5 ? 12 : 8;
      if (header + headerSize > end) invalid("page header");
      const cells = this.view.getUint16(header + 3);
      const content = this.view.getUint16(header + 5) || 65536;
      const pointerEnd = header + headerSize + cells * 2;
      if (content > this.usable || start + content < pointerEnd || this.bytes[header + 7]! > 60) invalid("cell pointer array");
      const ranges: [number, number][] = [];
      let free = this.view.getUint16(header + 1);
      while (free) {
        if (free < content || free + 4 > this.usable) invalid("freeblock pointer");
        const size = this.view.getUint16(start + free + 2);
        const next = this.view.getUint16(start + free);
        if (size < 4 || free + size > this.usable || (next && next < free + size)) invalid("freeblock chain");
        ranges.push([start + free, start + free + size]);
        free = next;
      }
      const children: typeof pending = [];
      let previousKey = node.lower;
      for (let i = 0; i < cells; i++) {
        const pointer = this.view.getUint16(header + headerSize + i * 2);
        const cell = start + pointer;
        if (pointer < content || cell >= end) invalid("cell pointer");
        let cursor = cell;
        if (type === 5) {
          if (cursor + 4 > end) invalid("interior cell");
          const separator = this.variable(this.bytes, cursor + 4, end);
          const key = BigInt.asIntN(64, separator.value);
          if ((previousKey !== undefined && key <= previousKey) || (node.upper !== undefined && key > node.upper)) invalid("interior rowid order or bounds");
          children.push({ page: this.view.getUint32(cursor), lower: previousKey, upper: key });
          previousKey = key;
          ranges.push([cell, separator.next]);
          continue;
        }
        const size = this.variable(this.bytes, cursor, end);
        if (!schemaRows && this.rowCount >= this.limit.rows) refuse("workLimit", "row limit");
        const length = this.integer(size.value);
        const rowid = this.variable(this.bytes, size.next, end);
        const key = BigInt.asIntN(64, rowid.value);
        if ((previousKey !== undefined && key <= previousKey) || (node.upper !== undefined && key > node.upper)) invalid("leaf rowid order or bounds");
        previousKey = key;
        cursor = rowid.next;
        const budget = schemaRows ? this.limit.schema + count * 9 + 64 : this.limit.data - this.dataBytes + count * 9 + 64;
        if (length > budget || length > this.bytes.length) refuse("ownershipLimit", "schema or value limit");
        const local = localPayload(length, this.usable);
        const overflow = length > local;
        if (cursor + local + (overflow ? 4 : 0) > end) invalid("truncated cell payload");
        ranges.push([cell, cursor + local + (overflow ? 4 : 0)]);
        const payload = this.options.allocateBytes(length);
        payload.set(this.bytes.subarray(cursor, cursor + local));
        let written = local;
        let next = overflow ? this.view.getUint32(cursor + local) : 0;
        while (written < length) {
          const overflowStart = this.claim(next);
          await checkpoint(this.options, "readPages", this.claimed, this.pages);
          next = this.view.getUint32(overflowStart);
          const chunk = Math.min(this.usable - 4, length - written);
          payload.set(this.bytes.subarray(overflowStart + 4, overflowStart + 4 + chunk), written);
          written += chunk;
        }
        if (next) invalid("overflow trailing page");
        yield { rowid: key, values: await this.fields(payload, count) };
      }
      ranges.sort((a, b) => a[0] - b[0]);
      let previous = start + content;
      let fragmented = 0;
      for (const [begin, finish] of ranges) {
        if (begin < previous) invalid("overlapping cells");
        const gap = begin - previous;
        if (gap > 3) invalid("untracked freeblock");
        fragmented += gap;
        previous = finish;
      }
      const tail = end - previous;
      if (tail > 3 || fragmented + tail !== this.bytes[header + 7]) invalid("fragmented cell bytes");
      if (type === 5) {
        children.push({ page: this.view.getUint32(header + 8), lower: previousKey, upper: node.upper });
        for (let i = children.length - 1; i >= 0; i--) pending.push(children[i]!);
      }
    }
  }
}

/** 📥️ Read semantic table rows without access to any native artifact codec. */
export async function importSqliteDatabase(bytes: Uint8Array, options: SqliteDatabaseOptions = {}): Promise<SqliteDatabase> {
  const operation=sqliteOperation(options);options=operation;const reader = new Reader(bytes, operation);
  const definitions: { name: string; sql: string; root: number; schema: TableSchema }[] = [];
  const names = new Set<string>();
  for await (const row of reader.table(1, 5, true)) {
    const [kind, name, tableName, root, sql] = row.values;
    if (kind !== "table") invalid("unsupported index, view or trigger schema");
    if(typeof name!=="string"||typeof tableName!=="string"||typeof root!=="bigint"||root<1n||root>BigInt(reader.pages)||typeof sql!=="string")invalid("schema row types or identifiers");
    const folded=await grammarControlled(foldedText(name),options),owner=await grammarControlled(foldedText(tableName),options);
    if(!(await grammarControlled(equalText(folded,owner),options))||folded.startsWith("sqlite_")||names.has(folded))invalid("schema row types or identifiers");names.add(folded);
    if (definitions.length + 1 > reader.limit.tables) refuse("workLimit", "table limit");
    reader.schemaBytes += await textByteLengthControlled(name,options) + await textByteLengthControlled(sql,options);
    if (reader.schemaBytes > reader.limit.schema) refuse("ownershipLimit", "schema limit");
    definitions.push({ name, sql, root: Number(root), schema: await schemaControlled(name, sql, reader.limit.columns,options) });
  }
  const tables: SqliteTable[] = [];
  for (const definition of definitions) {
    const rows: SqliteRow[] = [];
    for await (const row of reader.table(definition.root, definition.schema.columns.length, false)) {
      if (++reader.rowCount > reader.limit.rows) refuse("workLimit", "row limit");
      const values:SqliteValue[]=[];for(let index=0;index<row.values.length;index++){
        let cell=row.values[index]!;
        const column = definition.schema.columns[index]!;
        if (index === definition.schema.alias) {
          if (cell !== null) invalid("primary key alias storage");
          cell = row.rowid;
        } else if (column.affinity === "real" && typeof cell === "bigint") cell = Number(cell);
        if (cell === null && column.notNull) invalid("NOT NULL cell");
        reader.dataBytes += typeof cell==="string"?await textByteLengthControlled(cell,options):sqliteValueByteLength(cell);
        if (reader.dataBytes > reader.limit.data) refuse("ownershipLimit", "value limit");
        values.push(cell);
      }
      rows.push({ rowid: row.rowid, values });
      await cooperate(options, reader.rowCount);
    }
    tables.push({ name: definition.name, sql: definition.sql, rows });
  }
  await checkpoint(options, "readPages", reader.pages, reader.pages);
  return { tables };
}
