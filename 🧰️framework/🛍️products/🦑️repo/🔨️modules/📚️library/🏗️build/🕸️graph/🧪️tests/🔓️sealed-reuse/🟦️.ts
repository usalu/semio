/** 🔓️ Contract: a current sealed all-workspace graph is served by a shared cache-store reader and never by the graph writer; inference, publication and input expansion keep their order. The model is the fixture's ordered text laws, checked here against the graph source and in Python by `🐍️.py`. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
const body=(name:string):string=>{
 const row=fixture.functions[name as keyof typeof fixture.functions],start=source.indexOf(row.start),end=source.indexOf(row.end,start+row.start.length);
 expect(start,name+" start").toBeGreaterThan(-1);expect(source.indexOf(row.start,start+1),name+" start unique").toBe(-1);expect(end,name+" end").toBeGreaterThan(start);
 return source.slice(start,end);
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.order)test("order: "+row.name,()=>{const text=body(row.function),before=text.indexOf(row.before),after=text.indexOf(row.after);expect(before,"before text").toBeGreaterThan(-1);expect(after,"after text").toBeGreaterThan(-1);expect(before).toBeLessThan(after);});
for(const row of fixture.present)test("present: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(true);});
for(const row of fixture.absent)test("absent: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(false);});
