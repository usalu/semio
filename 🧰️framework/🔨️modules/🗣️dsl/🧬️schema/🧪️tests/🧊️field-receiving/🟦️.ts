import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import law from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
test("field receiving corpus satisfies its schema",()=>{expect(new Ajv({strict:false}).compile(schema)(law)).toBe(true);});
test("set members equal the independent SQLite distinct ordered members",()=>{const db=new Database(":memory:");for(const row of law.cases.filter(row=>row.kind==="set")){const members=(db.query("SELECT DISTINCT value FROM json_each(?) ORDER BY value").all(JSON.stringify(row.source))as{value:string}[]).map(member=>member.value);expect(members).toEqual(row.expected);}db.close();});
test("map entries equal the independent SQLite key ordered entries",()=>{const db=new Database(":memory:");for(const row of law.cases.filter(row=>row.kind==="map")){const entries=(db.query("SELECT key,value FROM json_each(?) ORDER BY key").all(JSON.stringify(Object.fromEntries(row.source as string[][])))as{key:string,value:string}[]).map(entry=>[entry.key,entry.value]);expect(entries).toEqual(row.expected);}db.close();});
test("value and block literals equal their canonical JSON form",()=>{const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([key,member])=>[key,canonical(member)])):value;for(const row of law.cases.filter(row=>row.kind==="value"||row.kind==="block"))expect(canonical(row.source)).toEqual(row.expected);});
