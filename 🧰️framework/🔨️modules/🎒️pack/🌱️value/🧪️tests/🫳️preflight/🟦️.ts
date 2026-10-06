/** 🫳️ Independent semantic, symbol and compression authority for borrowed exact Record forecasts. */
import {expect,test} from "bun:test";
import {blake3} from "@noble/hashes/blake3.js";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {deflateRawSync,inflateRawSync,constants} from "node:zlib";

type Csv={schema:string;hasHeader:boolean;records:{value:string;quoted:boolean}[][]};
type Tsv={schema:string;records:string[][];trailingNewline:boolean;lineEnding:"lf"|"crlf"};
type Metadata={prefix:string;repeat:number;suffixes:string[];pageBytes:number;order:string[];comparisonCases:string[];schemaHash:string};
type Corpus={longSchemaMetadata:Metadata;version:1;contract:Record<string,string|number>;csv:Csv[];tsv:Tsv[];longText:{text:string;repeat:number};distinctSymbols:{prefix:string;count:number;repeat:number};repeatedInlineSymbols:{text:string;repeat:number}};
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
const fixture=read("./🧫️fixtures/🔣️.json") as Corpus;

test("closed borrowed Record forecast distinguishes exact output, inline scratch and caller-paid spill",()=>{
 expect(fixture.contract.measurement).toBe("exactCanonicalOrdinaryBytes");
 expect(fixture.contract.inlineSymbols).toBe(64);
 expect(fixture.repeatedInlineSymbols).toEqual({text:"cell",repeat:1000});
});

test("independent SQLite preserves every declared CSV and TSV owner field and ordered empty containers",()=>{
 const db=new Database(":memory:");
 try{
  db.run("CREATE TABLE owner(id INTEGER PRIMARY KEY, family TEXT, schema TEXT, header INTEGER, trailing INTEGER, ending TEXT); CREATE TABLE record(id INTEGER PRIMARY KEY,owner INTEGER REFERENCES owner(id),ordinal INTEGER); CREATE TABLE field(id INTEGER PRIMARY KEY,record INTEGER REFERENCES record(id),ordinal INTEGER,value TEXT,quoted INTEGER)");
  let id=0,record=0,field=0;
  for(const [family,rows] of [["csv",fixture.csv],["tsv",fixture.tsv]] as const){
   for(const row of rows){
    const owner=++id;
    db.run("INSERT INTO owner VALUES(?,?,?,?,?,?)",[owner,family,row.schema,"hasHeader"in row?Number(row.hasHeader):null,"trailingNewline"in row?Number(row.trailingNewline):null,"lineEnding"in row?row.lineEnding:null]);
    for(const [ordinal,items] of row.records.entries()){
     const parent=++record;db.run("INSERT INTO record VALUES(?,?,?)",[parent,owner,ordinal]);
     for(const [ordinal,item] of items.entries())db.run("INSERT INTO field VALUES(?,?,?,?,?)",[++field,parent,ordinal,typeof item==="string"?item:item.value,typeof item==="string"?null:Number(item.quoted)]);
    }
    const records=db.query<{id:number},[number]>("SELECT id FROM record WHERE owner=? ORDER BY ordinal").all(owner).map(parent=>db.query<{value:string;quoted:number|null},[number]>("SELECT value,quoted FROM field WHERE record=? ORDER BY ordinal").all(parent.id).map(item=>family==="csv"?{value:item.value,quoted:Boolean(item.quoted)}:item.value));
    const values=db.query<{schema:string;header:number|null;trailing:number|null;ending:string|null},[number]>("SELECT schema,header,trailing,ending FROM owner WHERE id=?").get(owner)!;
    expect(family==="csv"?{schema:values.schema,hasHeader:Boolean(values.header),records}:{schema:values.schema,records,trailingNewline:Boolean(values.trailing),lineEnding:values.ending}).toEqual(row);
   }
  }
  expect(db.query("SELECT count(*) AS total FROM owner").get()).toEqual({total:6});
  expect(db.query("SELECT count(*) AS total FROM record WHERE NOT EXISTS(SELECT 1 FROM field WHERE field.record=record.id)").get()).toEqual({total:5});
 }finally{db.close();}
});

test("third-party DEFLATE and SQLite independently validate long text and full distinct symbol spill recipes",()=>{
 const long=fixture.longText.text.repeat(fixture.longText.repeat);
 const symbols=Array.from({length:fixture.distinctSymbols.count},(_,index)=>fixture.distinctSymbols.prefix+index);
 for(const text of [long,...symbols]){const raw=Buffer.from(text);const compressed=deflateRawSync(raw,{strategy:constants.Z_FIXED});expect(inflateRawSync(compressed)).toEqual(raw);expect(raw.length).toBe(Buffer.byteLength(text));}
 expect(Buffer.byteLength(long)).toBeGreaterThan(65536);
 const db=new Database(":memory:");try{
  db.run("CREATE TABLE symbol(value TEXT PRIMARY KEY, occurrences INTEGER NOT NULL)");
  for(let repeat=0;repeat<fixture.distinctSymbols.repeat;repeat++)for(const symbol of symbols)db.run("INSERT INTO symbol VALUES(?,1) ON CONFLICT(value) DO UPDATE SET occurrences=occurrences+1",[symbol]);
  expect(db.query("SELECT count(*) AS count,min(occurrences) AS minimum,max(occurrences) AS maximum FROM symbol").get()).toEqual({count:257,minimum:2,maximum:2});
  expect(db.query<{value:string},[]>("SELECT value FROM symbol ORDER BY value COLLATE BINARY").all().map(row=>row.value)).toEqual([...symbols].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b))));
  expect(symbols.length).toBeGreaterThan(64);
  db.run("DELETE FROM symbol");
  for(let index=0;index<fixture.repeatedInlineSymbols.repeat;index++)db.run("INSERT INTO symbol VALUES(?,1) ON CONFLICT(value) DO UPDATE SET occurrences=occurrences+1",[fixture.repeatedInlineSymbols.text]);
  expect(db.query("SELECT count(*) AS count,sum(occurrences) AS occurrences FROM symbol").get()).toEqual({count:1,occurrences:1000});
 }finally{db.close();}
});

test("independent byte ordering and BLAKE3 retain the authored long metadata canonical graph",()=>{
 const m=fixture.longSchemaMetadata;const prefix=m.prefix.repeat(m.repeat);const labels=m.suffixes.map(suffix=>prefix+suffix);const sorted=[...labels].sort((a,b)=>Buffer.compare(Buffer.from(a),Buffer.from(b)));expect(sorted).toEqual(m.order.map(suffix=>prefix+suffix));
 const bytes:number[]=[];const integer=(n:number)=>{while(n>=128){bytes.push((n&127)|128);n=Math.floor(n/128);}bytes.push(n);};const text=(s:string)=>{const b=Buffer.from(s);integer(b.length);for(const byte of b)bytes.push(byte);};integer(1);integer(1);integer(0);text("label");bytes.push(0,7);integer(2);for(const label of sorted){integer(0);text(label);}expect(Buffer.from(blake3(Uint8Array.from(bytes))).toString("hex")).toBe(m.schemaHash);expect(Buffer.byteLength(labels[0])).toBeGreaterThan(m.pageBytes);expect(m.comparisonCases).toEqual(["field","enum","statement","reference"]);console.log("[DEBUG] independent byte comparator and BLAKE3 preserve long schema metadata canonical order and exact graph digest");
});

test("closed sparse table and statement recipe retains complete literal column roles under independent DEFLATE",()=>{
 const recipe=read("./🧫️fixtures/🔣️.json").tableStatements;expect(recipe.rowCount).toBe(9);expect(recipe.columns).toEqual([["text",5],["flag",1],["signed",2],["unsigned",3],["real",4],["kind",6],["nested",0]]);expect(recipe.sparseOrdinals).toEqual([1,4,8]);expect(recipe.keywords).toEqual(["append","remove","k".repeat(129)]);
 const raw=Buffer.from(recipe.longText.prefix.repeat(recipe.longText.repeat));expect(raw.length).toBeGreaterThan(128);expect(inflateRawSync(deflateRawSync(raw,{strategy:constants.Z_FIXED}))).toEqual(raw);const presence=new Uint8Array(Math.ceil(recipe.rowCount/8));for(let row=0;row<recipe.rowCount;row++)if(!recipe.sparseOrdinals.includes(row))presence[row>>3]!|=1<<(row&7);expect(Array.from(presence)).toEqual([237,0]);console.log("[DEBUG] independent sparse row bitmaps and full forced symbol text recipe admitted");
});

test("closed logical wire depths and signed reference specimen preserve independent SQLite words",()=>{
 const corpus=read("./🧫️fixtures/🔣️.json"),r=corpus.logicalDepth;
 expect([r.tableStatements,r.nestedRecord,r.packedSignedList]).toEqual([3,3,1]);expect(r.referenceTarget).toBe("document");expect(r.rootAbsent).toBe(0);
 const db=new Database(":memory:",{safeIntegers:true});try{db.run("CREATE TABLE specimen(ordinal INTEGER PRIMARY KEY,word INTEGER,label TEXT)");for(const[index,word]of r.signedWords.entries())db.query("INSERT INTO specimen VALUES(?,?,?)").run(index,BigInt(word),r.nestedText);expect((db.query("SELECT word,label FROM specimen ORDER BY ordinal").all() as {word:bigint,label:string}[]).map(row=>[String(row.word),row.label])).toEqual(r.signedWords.map((word:string)=>[word,r.nestedText]));}finally{db.close();}
 const raw=Buffer.from(r.nestedText);expect(inflateRawSync(deflateRawSync(raw))).toEqual(raw);console.log("[DEBUG] independent SQLite retains all signed words and literal UTF8 at the authored logical-depth frontier");
});

test("closed authored coordinates and directions preserve exact words through independent SQLite and DEFLATE",()=>{
 const corpus=read("./🧫️fixtures/🔣️.json"),recipe=corpus.spatial;
 const db=new Database(":memory:");try{db.run("CREATE TABLE scalar(role TEXT NOT NULL,ordinal INTEGER NOT NULL,word BLOB NOT NULL,PRIMARY KEY(role,ordinal))");for(const role of["coordinate","direction"]){expect(recipe[role].arity).toBe(3);const bytes=new Uint8Array(24);for(const[index,word]of recipe[role].words.entries()){const bit=BigInt("0x"+word);new DataView(bytes.buffer).setBigUint64(index*8,bit,true);db.run("INSERT INTO scalar VALUES(?,?,?)",[role,index,bytes.subarray(index*8,index*8+8)]);}expect(inflateRawSync(deflateRawSync(bytes,{strategy:constants.Z_FIXED}))).toEqual(Buffer.from(bytes));expect(db.query("SELECT word FROM scalar WHERE role=? ORDER BY ordinal").all(role).map((row:any)=>new DataView(row.word.buffer,row.word.byteOffset,8).getBigUint64(0,true).toString(16).padStart(16,"0"))).toEqual(recipe[role].words);}expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});}finally{db.close();}console.log("[DEBUG] independent authored spatial IEEE words retain role, ordinal and exact storage");
});
