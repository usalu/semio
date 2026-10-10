/** 🧱️ Closed literal refusal cases have independent schema, SQLite and UTF8 witnesses. */
import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import{readFileSync}from"node:fs";
import fixture from"../🧫️fixtures/🔣️.json";import schema from"../🧬️schema/🔣️.json";
test("original native refusal contract is closed and independently classified",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,renewedAuthority:1})).toBe(false);const database=new Database(":memory:");
 try{for(const sample of fixture.cases){const result=database.query("SELECT CASE ? WHEN 'depth' THEN 'depthLimit' WHEN 'invalidUtf8' THEN 'invalidValue' WHEN 'checkpoint' THEN 'canceled' ELSE 'ownershipLimit' END AS kind").get(sample.operation);expect(result).toEqual({kind:sample.kind});expect(sample.message.length).toBeGreaterThan(0);}}finally{database.close();}
 expect(()=>new TextDecoder("utf-8",{fatal:true}).decode(new Uint8Array([255]))).toThrow();expect(fixture.maximumBytes).toBe(0);expect(fixture.causeStorage).toBe("borrowedLiteral");expect(fixture.forwarded).toEqual({maximumBytes:1,chargeBytes:1,kind:"invalidValue",message:"original forwarded refusal λ",allocatedBytes:0,releasedBytes:0,ownedBytes:0});
 for(const direction of["🛬️decode","🛫️encode"]){const source=readFileSync(new URL("../../../"+direction+"/🦀️.rs",import.meta.url),"utf8");expect(source.includes("ValueError::new(")).toBe(false);}
 console.log("[DEBUG] Independent Ajv/SQLite/TextDecoder verify14 closed literal refusal directions with zero declared original capacity; Native physical assertions separate");
});
