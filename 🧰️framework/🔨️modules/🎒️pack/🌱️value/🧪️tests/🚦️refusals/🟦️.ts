/** 🚦️ Validates the handwritten refusal corpus independently of native execution. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../../🧫️fixtures/🚦️refusals/🔣️.json";
import schema from "../../🧬️schema/🚦️refusals/🔣️.json";
import "../🎞️intrinsic-media/🟦️.ts";
import "../🔃️ordering/🟦️.ts";

test("neutral strict wire corpus separates grammar from observed native backing",async()=>{
    const root=resolve(import.meta.dir,"../../../../../..");
    const {proveWireValueMaterializationFixture}=await import(resolve(root,"🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts"));
    await proveWireValueMaterializationFixture(root);
});

test("neutral controlled Pack refusals retain explicit category and literal Unicode order",()=>{
    const validator=new Ajv({strict:true});expect(validator.validate(schema,fixture)).toBe(true);
    const database=new Database(":memory:");try{
        database.exec("CREATE TABLE refusal(ordinal INTEGER PRIMARY KEY,id TEXT NOT NULL,kind TEXT NOT NULL);CREATE TABLE symbol(value TEXT NOT NULL)");
        fixture.cases.forEach((entry,index)=>database.query("INSERT INTO refusal VALUES(?,?,?)").run(index,entry.id,entry.kind));
        fixture.symbols.forEach(text=>database.query("INSERT INTO symbol VALUES(?)").run(text));
        expect(database.query("SELECT kind FROM refusal ORDER BY ordinal").all()).toEqual(fixture.cases.map(entry=>({kind:entry.kind})));
        expect(database.query("SELECT DISTINCT hex(CAST(value AS BLOB)) AS word FROM symbol ORDER BY CAST(value AS BLOB)").all()).toEqual(fixture.orderedSymbols.map(text=>({word:Buffer.from(text).toString("hex").toUpperCase()})));
    }finally{database.close();}
});

test("reference refusal corpus keeps unrestricted four-field identity independent of a URI",()=>{
    const root=resolve(import.meta.dir,"../../../../../.."),owner=resolve(root,"🧰️framework/🔨️modules/🚪️io/🧬️schema/🔗️reference");
    const fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🚦️refusals/🔣️.json"),"utf8")) as {identity:Record<string,string>;cases:{id:string;kind:string}[]};
    const schema=JSON.parse(readFileSync(resolve(owner,"🧬️schema/🚦️refusals/🔣️.json"),"utf8"));expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
    const database=new Database(":memory:");try{database.exec("CREATE TABLE identity(field TEXT PRIMARY KEY,value BLOB NOT NULL)");for(const[field,value]of Object.entries(fixture.identity))database.query("INSERT INTO identity VALUES(?,?)").run(field,Buffer.from(value));
        for(const[field,value]of Object.entries(fixture.identity))expect(database.query("SELECT hex(value) AS word FROM identity WHERE field=?").get(field)).toEqual({word:Buffer.from(value).toString("hex").toUpperCase()});
    }finally{database.close();}
});
