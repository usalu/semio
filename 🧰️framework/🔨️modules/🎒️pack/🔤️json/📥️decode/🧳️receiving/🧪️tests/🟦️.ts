import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
const read=(name:string)=>JSON.parse(readFileSync(new URL(name,import.meta.url),"utf8"));
test("original JSON receiving matches the independent JSON parser and canonical byte corpus",()=>{
 const law=read("../🔣️.json") as {cases:{name:string;header:string;wire:string;value:unknown;canonical:boolean}[];terminalDropBytes:number};
 expect(new Ajv({strict:true}).compile(read("../🧬️schema/🔣️.json"))(law)).toBe(true);
 for(const row of law.cases){const body=row.wire.slice(row.header.length);expect(JSON.parse(body)).toEqual(row.value);expect(row.header+JSON.stringify(JSON.parse(body))===row.wire).toBe(row.canonical);}
 expect(law.terminalDropBytes).toBe(0);
});
