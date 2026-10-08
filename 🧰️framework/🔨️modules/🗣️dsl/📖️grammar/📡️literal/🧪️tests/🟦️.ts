/** 🧫️ Independent Buffer and SQLite oracle for literal native record ownership. */
import{test,expect}from"bun:test";
import{Buffer}from"node:buffer";
import{Database}from"bun:sqlite";

import fixture from"../🧫️fixtures/🔣️.json";
import bits from"../🧫️fixtures/🧮️bits/🔣️.json";

test("native bit-field fixtures have independent Buffer words and arithmetic mask witnesses",()=>{
 for(const item of bits.cases){const buffer=Buffer.alloc(8);buffer.writeBigUInt64LE(BigInt(item.word));const word=buffer.readBigUInt64LE();let mask=BigInt(item.mask),remaining=word,present=true;while(mask!==0n){if(mask%2n===1n&&remaining%2n!==1n)present=false;mask/=2n;remaining/=2n;}expect(present).toBe(item.present);expect(buffer.readBigUInt64LE()).toBe(BigInt(item.word));}
 console.log(`[DEBUG] protocol bit-field independent Buffer witnesses=${bits.cases.length}`);
});

test("literal octets and unsigned64 domains have independent lexical and Buffer proof",()=>{
 for(const item of fixture.scalars.cases){const signed=/^(?:0|-?[1-9][0-9]*)$/.test(item.signed)&&BigInt(item.signed)>=-(1n<<63n)&&BigInt(item.signed)<(1n<<63n);const unsigned=/^(?:0|[1-9][0-9]*)$/.test(item.unsigned)&&BigInt(item.unsigned)<(1n<<32n);expect(signed&&unsigned).toBe(item.valid);if(item.valid){const bytes=Buffer.alloc(12);bytes.writeBigInt64LE(BigInt(item.signed));bytes.writeUInt32LE(Number(item.unsigned),8);expect(bytes.readBigInt64LE()).toBe(BigInt(item.signed));expect(bytes.readUInt32LE(8)).toBe(Number(item.unsigned));}}
 for(const item of fixture.lexical.cases){const octets=/^(?:[0-9a-f]{2})+$/.test(item.hex);const integer=/^(?:0|[1-9][0-9]*)$/.test(item.integer)&&BigInt(item.integer)<=18446744073709551615n;expect(octets&&integer).toBe(item.valid);if(item.valid){expect(Buffer.from(item.hex,"hex").toString("hex")).toBe(item.hex);const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(BigInt(item.integer));expect(bytes.readBigUInt64LE()).toBe(BigInt(item.integer));}}
});
test("literal protocol fixture independently preserves recursive record boundaries",()=>{const bytes:number[]=[fixture.revision,fixture.entries.length];const sql=new Database(":memory:");sql.run("CREATE TABLE entry(id INTEGER PRIMARY KEY,parent INTEGER,label TEXT,flag INTEGER)");let id=0;const entry=(value:{flag:number;label:string;child:unknown},parent:number|null)=>{const key=++id,text=Buffer.from(value.label);bytes.push(value.flag,text.length,...text);sql.query("INSERT INTO entry VALUES(?,?,?,?)").run(key,parent,value.label,value.flag);if(value.child)entry(value.child as typeof value,key);};try{for(const value of fixture.entries)entry(value,null);bytes.push(fixture.tail);expect(bytes).toEqual(fixture.binary);expect(sql.query("SELECT parent,label,flag FROM entry ORDER BY id").all()).toEqual([{parent:null,label:"a",flag:0},{parent:null,label:"b",flag:1},{parent:2,label:"c",flag:0}]);}finally{sql.close();}});
