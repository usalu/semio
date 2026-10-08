import {test,expect} from "bun:test";
import {readFileSync,existsSync} from "node:fs";
import {resolve} from "node:path";
import {Database} from "bun:sqlite";
import {Reader,Writer} from "protobufjs/minimal.js";
import {applyPatch} from "fast-json-patch";

test("retained inline symbols preserve original wire indices and independent canonical UTF8",()=>{
  const owner=resolve(import.meta.dir,"..");
  const law=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8"));
  const database=new Database(":memory:");
  try{
    database.exec("CREATE TABLE symbols (ordinal INTEGER PRIMARY KEY, value BLOB NOT NULL)");
    for(const row of law.valid){
      const bytes=Uint8Array.from(Buffer.from(row.hex,"hex"));
      const reader=Reader.create(bytes);
      reader.pos=row.sourceOffset;
      const count=Number(reader.uint64().toString());
      const writer=Writer.create().uint64(count);
      const symbols=[];
      database.exec("DELETE FROM symbols");
      for(let index=0;index<count;index++){
        const raw=reader.bytes();
        writer.bytes(raw);
        symbols.push(new TextDecoder("utf-8",{fatal:true}).decode(raw));
        database.query("INSERT INTO symbols VALUES (?,?)").run(index,raw);
      }
      expect(symbols).toEqual(row.symbols);
      expect(reader.pos).toBe(row.prefixBytes);
      expect(Buffer.from(writer.finish())).toEqual(Buffer.from(bytes.subarray(row.sourceOffset,reader.pos)));
      expect(database.query("SELECT ordinal,value FROM symbols ORDER BY ordinal").all().map((value:any)=>({ordinal:value.ordinal,text:new TextDecoder().decode(value.value)}))).toEqual(row.symbols.map((text:string,ordinal:number)=>({ordinal,text})));
      expect(applyPatch({symbols},[{op:"move",from:"/symbols",path:"/retained"}],true,false).newDocument).toEqual({retained:row.symbols});
    }
    for(const row of law.invalid){
      const bytes=Uint8Array.from(Buffer.from(row.hex,"hex"));
      const reader=Reader.create(bytes);
      if(row.refusal==="symbol-count")expect(Number(reader.uint64().toString())).toBeGreaterThan(law.maximumSymbols);
      else if(row.refusal==="symbol-length"){reader.uint64();expect(Number(reader.uint64().toString())).toBeGreaterThan(law.maximumSymbolBytes);}
      else if(row.refusal==="noncanonical"){
        const count=reader.uint64();const writer=Writer.create().uint64(count);
        if(Number(count.toString())!==0)writer.uint64(reader.uint64());
        expect(Buffer.from(writer.finish())).not.toEqual(Buffer.from(bytes.subarray(0,reader.pos)));
      }else expect(()=>{const count=Number(reader.uint64().toString());for(let index=0;index<count;index++)new TextDecoder("utf-8",{fatal:true}).decode(reader.bytes());}).toThrow();
    }
    const large=law.large.word.repeat(law.large.repetitions);
    const largeBytes=new TextEncoder().encode(large);
    expect(largeBytes.length).toBe(law.large.bytes);
    const largeReader=Reader.create(Writer.create().uint64(1).bytes(largeBytes).finish());
    expect(largeReader.uint64().toString()).toBe("1");
    expect(new TextDecoder("utf-8",{fatal:true}).decode(largeReader.bytes())).toBe(large);
    const emptyWriter=Writer.create().uint64(law.large.emptySymbols);
    for(let index=0;index<law.large.emptySymbols;index++)emptyWriter.bytes(new Uint8Array());
    const emptyReader=Reader.create(emptyWriter.finish());
    expect(emptyReader.uint64().toString()).toBe(String(law.large.emptySymbols));
    for(let index=0;index<law.large.emptySymbols;index++)expect(emptyReader.bytes().length).toBe(0);
    expect(emptyReader.pos).toBe(emptyReader.len);
  }finally{database.close();}
  expect(existsSync(resolve(owner,"🦀️.rs"))).toBe(true);
  const source=readFileSync(resolve(owner,"🦀️.rs"),"utf8");
  expect(source.includes("PagedList<SymbolSpan")).toBe(true);
  expect(source.includes("RetainedCloneBinding::close_one")).toBe(true);
  expect(source.includes("decode_inline_symbols(")).toBe(false);
  expect(source.includes("Utf8Admission")).toBe(false);
  console.log("[DEBUG] Retained inline symbol indices/canonical unsigned wire/UTF8/NUL/empty/duplicate/prefix tail agree with Protobuf.js, fatal TextDecoder, SQLite ordinal BLOBs and RFC6902; native bounded parsing/heap/alias proof remains required");
});
