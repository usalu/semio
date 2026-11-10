import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
const law=JSON.parse(readFileSync(join(import.meta.dir,"cases.json"),"utf8")) as {cases:{name:string;input:number[];index:number;item:number|null;expected:number[]}[];denialAxes:string[]};
test("paged edits match independent SQLite ordering",()=>{
 const validate=new Ajv().compile(JSON.parse(readFileSync(join(import.meta.dir,"cases.schema.json"),"utf8")));expect(validate(law),JSON.stringify(validate.errors)).toBe(true);
 const db=new Database(":memory:");try{db.run("CREATE TABLE entry(position INTEGER, value INTEGER)");for(const row of law.cases){db.run("DELETE FROM entry");row.input.forEach((value,position)=>db.run("INSERT INTO entry VALUES (?,?)",[position,value]));const result=row.name==="insert"?db.query("SELECT value FROM (SELECT position*2 AS ordering,value FROM entry UNION ALL SELECT ?1*2-1,?2) ORDER BY ordering").all(row.index,row.item):db.query("SELECT value FROM entry WHERE position != ?1 ORDER BY position").all(row.index);expect(result.map((x:any)=>x.value)).toEqual(row.expected)}}finally{db.close()}
});
