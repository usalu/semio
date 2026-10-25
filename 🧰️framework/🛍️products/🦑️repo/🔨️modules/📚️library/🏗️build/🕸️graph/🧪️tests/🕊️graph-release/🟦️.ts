/** 🕊️ Contract: once a backend has planned from the build graph it calls `release`, the graph content is dropped and collected (the process keeps only the pinned snapshot), every later read of the released graph throws, and the order laws of the fixture hold in the sources: the graph is released before the first Bazel process starts, the generation builder custody ended before the graph was returned, and no copy of the graph outlives its planning. The laws are plain text laws verified again by `🐍️.py`. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const library=fileURLToPath(new URL("../../../../",import.meta.url)),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),fixturePath=fileURLToPath(new URL("./🔣️.json",import.meta.url));
const body=(name:string):string=>{
 const row=fixture.functions[name as keyof typeof fixture.functions],source=readFileSync(library+row.file,"utf8"),start=source.indexOf(row.start),end=source.indexOf(row.end,start+row.start.length);
 expect(start,name+" start").toBeGreaterThan(-1);expect(source.indexOf(row.start,start+1),name+" start unique").toBe(-1);expect(end,name+" end").toBeGreaterThan(start);
 return source.slice(start,end);
};

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.order)test("order: "+row.name,()=>{const text=body(row.function),before=text.indexOf(row.before),after=text.indexOf(row.after,before);expect(before,"before text").toBeGreaterThan(-1);expect(after,"after text").toBeGreaterThan(before);});
for(const row of fixture.present)test("present: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(true);});
for(const row of fixture.absent)test("absent: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(false);});
for(const row of fixture.absentAfter)test("absent after: "+row.name,()=>{const text=body(row.function),at=text.indexOf(row.marker);expect(at).toBeGreaterThan(-1);expect(text.slice(at+row.marker.length).includes(row.text)).toBe(false);});
test("the independent oracle agrees with the laws",()=>{const result=Bun.spawnSync(["python3",oracle,fixturePath,library],{stdout:"pipe",stderr:"pipe"});expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);});

const child=fileURLToPath(new URL("./🟨️.mjs",import.meta.url));

test("releasing a graph collects its content and refuses every later read",()=>{
 const result=Bun.spawnSync([process.execPath,child,library,"60"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const row=JSON.parse(result.stdout.toString());
 expect(row.before-row.after).toBeGreaterThan(row.before*0.8);
 expect(row.refused).toEqual([true,true,true]);
 expect(row.keys).toEqual(["version"]);
 expect(row.survivor).toBe(4000);
});
