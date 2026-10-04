/** 🫳️ Independent semantic, symbol and compression authority for borrowed exact Record forecasts. */
import {expect,test} from "bun:test";
import {blake3} from "@noble/hashes/blake3.js";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import {deflateRawSync,inflateRawSync,constants} from "node:zlib";

type Csv={schema:string;hasHeader:boolean;records:{value:string;quoted:boolean}[][]};
type Tsv={schema:string;records:string[][];trailingNewline:boolean;lineEnding:"lf"|"crlf"};
type Metadata={prefix:string;repeat:number;suffixes:string[];pageBytes:number;order:string[];comparisonCases:string[];schemaHash:string};
type Corpus={longSchemaMetadata:Metadata;version:1;contract:Record<string,string|number>;csv:Csv[];tsv:Tsv[];longText:{text:string;repeat:number};distinctSymbols:{prefix:string;count:number;repeat:number};repeatedInlineSymbols:{text:string;repeat:number}};
const read=(path:string)=>JSON.parse(readFileSync(new URL(path,import.meta.url),"utf8"));
const fixture=read("./🧫️fixtures/🔣️.json") as Corpus;

test("closed borrowed Record forecast distinguishes exact output, inline scratch and caller-paid spill",()=>{
 const validate=new Ajv({strict:true}).compile(read("./🧬️schema/🔣️.json"));
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(fixture.contract.measurement).toBe("exactCanonicalOrdinaryBytes");
 expect(fixture.contract.inlineSymbols).toBe(64);
 expect(fixture.repeatedInlineSymbols).toEqual({text:"cell",repeat:1000});
 expect(validate({...fixture,contract:{...fixture.contract,measurement:"conservativeUpperBound"}})).toBe(false);
 expect(validate({...fixture,contract:{...fixture.contract,spill:"privateLedger"}})).toBe(false);
 expect(validate({...fixture,unknown:true})).toBe(false);
 expect(validate({...fixture,tsv:fixture.tsv.map(row=>({...row,lineEnding:"normalized"}))})).toBe(false);
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
