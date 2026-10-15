/** 📏️ The actual fixed page maximum refuses its next binding before new backing is allocated. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import law from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("original native page maximum and its exact physical stock agree with SQLite",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,pageCount:257})).toBe(false);const database=new Database(":memory:");try{let pages=0,capacity=0;for(let index=0;index<=law.pageCount;index++){const admitted=index<law.pageCount;const row=database.query("select (? < ?) as admitted, ?*? as capacity").get(index,law.pageCount,pages,law.pageBytes) as {admitted:number;capacity:number};expect(Boolean(row.admitted)).toBe(admitted);expect(row.capacity).toBe(capacity);if(admitted){pages++;capacity+=law.pageBytes;}}expect(capacity).toBe(law.sourceBytes-1);expect(capacity).toBeLessThan(law.stockBytes);expect(law.pageCount*(law.pageBytes/law.copyPerTurn+1)+16).toBeLessThan(law.maximumTurns);}finally{database.close();}console.log("[DEBUG] Ajv/SQLite native256x16384 maximum plus1 refused, real4MiB stock has separate65536root margin; actual Original runtime separate");});
