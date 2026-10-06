import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { Database } from "bun:sqlite";
import { createToken, EmbeddedActionsParser, Lexer } from "chevrotain";
const owner=resolve(import.meta.dir,"../.."), read=(path:string)=>JSON.parse(readFileSync(resolve(owner,path),"utf8"));
test("controlled decoder intrinsic corpus has closed independent literal and arithmetic identities",()=>{
 const mirror=resolve(owner,"../../../..","🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema");
 for(const path of ["🧫️fixtures/🛬️decoding/🔣️.json","🧫️fixtures/🛬️decoding/🔑️keys.json","🧪️tests/🧾️record-list/🧫️fixtures/🔣️.json"])expect(JSON.parse(readFileSync(resolve(mirror,path),"utf8"))).toEqual(read(path));
 const fixture = read("🧫️fixtures/🛬️decoding/🔣️.json");
 const word=Buffer.alloc(8);word.writeBigUInt64LE(BigInt("0x"+fixture.floatWord));expect(new DataView(word.buffer,word.byteOffset,8).getBigUint64(0,true).toString(16)).toBe(fixture.floatWord);
 const database=new Database(":memory:");try{
 database.run("CREATE TABLE literal(text TEXT,octets BLOB,word BLOB,signed TEXT,unsigned TEXT)");database.run("INSERT INTO literal VALUES(?,?,?,?,?)",fixture.text,new Uint8Array(fixture.octets),word,fixture.signed,fixture.unsigned);
 expect(database.query("SELECT text,hex(octets) AS octets,hex(word) AS word,signed,unsigned FROM literal").get()).toEqual({text:fixture.text,octets:"00FF80",word:"010000000000F07F",signed:"-9223372036854775808",unsigned:"18446744073709551615"});
 expect(database.query("SELECT 10-2*3 AS value").get()).toEqual({value:fixture.formulaValue});
 const wires=database.query("SELECT json_extract(value,'$.from') AS source,json_extract(value,'$.to') AS target,json_extract(value,'$.directed') AS directed,json_extract(value,'$.labelId') AS labelId,json_extract(value,'$.labelKind') AS labelKind FROM json_each(?1) ORDER BY key").all(JSON.stringify(fixture.wires));
 expect(wires).toEqual(fixture.wires.map((row:{from:string;to:string|null;directed:boolean|null;labelId:string|null;labelKind:string|null})=>({source:row.from,target:row.to,directed:row.directed===null?null:Number(row.directed),labelId:row.labelId,labelKind:row.labelKind})));expect(wires).toHaveLength(8);
 }finally{database.close();}
});
test("controlled decoder literal keys have exact closed identities and independent SQLite ordering",()=>{
 const keys=read("🧫️fixtures/🛬️decoding/🔑️keys.json") as string[];
 const database=new Database(":memory:");try{database.run("CREATE TABLE identity(id INTEGER PRIMARY KEY,key TEXT)");keys.forEach((key,index)=>database.query("INSERT INTO identity VALUES(?,?)").run(index+1,key));expect(database.query("SELECT key FROM identity ORDER BY id").all().map((row:any)=>row.key)).toEqual(keys);for(const key of keys)expect(JSON.parse(JSON.stringify(key))).toBe(key);}finally{database.close();}
});
const Space=createToken({name:"Space",pattern:/\s+/,group:Lexer.SKIPPED}),Item=createToken({name:"Item",pattern:/items\b/}),Key=createToken({name:"Key",pattern:/[a-zA-Z_][a-zA-Z_0-9]*/}),Text=createToken({name:"Text",pattern:/"(?:[^"\\]|\\.)*"/}),NumberToken=createToken({name:"NumberToken",pattern:/-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?/}),Equals=createToken({name:"Equals",pattern:/=/}),LeftList=createToken({name:"LeftList",pattern:/\[/}),RightList=createToken({name:"RightList",pattern:/\]/}),LeftRecord=createToken({name:"LeftRecord",pattern:/\{/}),RightRecord=createToken({name:"RightRecord",pattern:/\}/});
const tokens=[Space,Item,Key,Text,NumberToken,Equals,LeftList,RightList,LeftRecord,RightRecord],lexer=new Lexer(tokens);
class BoundaryOracle extends EmbeddedActionsParser{
 document=this.RULE("document",()=>{this.CONSUME(Item);this.CONSUME(Equals);return this.SUBRULE(this.list);});
 list=this.RULE("list",()=>{const rows:unknown[]=[];this.CONSUME(LeftList);this.MANY(()=>{const row=this.SUBRULE(this.record);this.ACTION(()=>rows.push(row));});this.CONSUME(RightList);return rows;});
 record=this.RULE("record",()=>{const fields:Record<string,unknown>={};this.CONSUME(LeftRecord);this.MANY(()=>{const key=this.CONSUME(Key);this.CONSUME(Equals);const value=this.OR([{ALT:()=>this.SUBRULE(this.list)},{ALT:()=>{const token=this.CONSUME(Text);return this.ACTION(()=>JSON.parse(token.image));}},{ALT:()=>{const token=this.CONSUME(NumberToken);return this.ACTION(()=>Number(token.image));}}]);this.ACTION(()=>{const name=key.image;if(Object.hasOwn(fields,name)||!["min","max","label","children"].includes(name))throw Error("undeclared or duplicate field");if(name==="children"?!Array.isArray(value):name==="label"?typeof value!=="string":typeof value!=="number")throw Error("field shape");fields[name]=value;});});this.CONSUME(RightRecord);return fields;});
 constructor(){super(tokens);this.performSelfAnalysis();}
 parse(source:string){const result=lexer.tokenize(source);if(result.errors.length)throw Error("lexical boundary");this.input=result.tokens;const rows=this.document();if(this.errors.length||this.LA(1).image)throw Error("record boundary");return rows;}
}
test("controlled record list fixtures retain braced empty and optional owners through an independent parser",()=>{
 const fixture=read("🧪️tests/🧾️record-list/🧫️fixtures/🔣️.json"), oracle=new BoundaryOracle();
 for(const row of fixture.cases)expect(oracle.parse(row.source)).toEqual(row.rows);for(const source of fixture.invalid)expect(()=>oracle.parse(source)).toThrow();
});
