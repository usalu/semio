import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{buf as oracle}from"crc-32";
import{crc32ChunkV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import cases from"./🔣️.json"with{type:"json"};
test("bounded incremental CRC32 matches neutral corpus independent crc-32 and SQLite",()=>{const validate=new Ajv({strict:true}).compile(schema),database=new Database(":memory:");try{database.exec("create table checksums(value integer)");for(const row of cases.valid){const{expected,...request}=row;expect(validate(request)).toBe(true);const bytes=Uint8Array.from(row.bytes),actual=crc32ChunkV1(row.previous,bytes);expect(actual).toBe(expected);expect(actual).toBe(oracle(Buffer.from(bytes),row.previous)>>>0);database.query("insert into checksums values(?)").run(actual);for(let split=0;split<=bytes.length;split++)expect(crc32ChunkV1(crc32ChunkV1(row.previous,bytes.subarray(0,split)),bytes.subarray(split))).toBe(actual);}expect(database.query<{count:number},[]>("select count(*) count from checksums").get()?.count).toBe(cases.valid.length);for(const row of cases.invalid)expect(validate(row)).toBe(false);for(const seed of[-1,4294967296,NaN,1.5])expect(()=>crc32ChunkV1(seed,new Uint8Array())).toThrow();expect(()=>crc32ChunkV1(0,new Uint8Array(65537))).toThrow();const bytes=new Uint8Array(65536);expect(crc32ChunkV1(0,bytes)).toBe(oracle(Buffer.from(bytes),0)>>>0);}finally{database.close();}});
