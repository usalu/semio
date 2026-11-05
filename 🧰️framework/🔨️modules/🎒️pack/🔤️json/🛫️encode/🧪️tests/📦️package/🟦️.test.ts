import {test,expect} from "bun:test";
import Ajv from "ajv";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
import {readRustSourceClosureV1} from "../../../../../🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./📋️schema.json";
test("canonical encoding has separate nonoverlapping bounded source owners",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const owners: Set<string>[]=[];
 for(const owner of fixture.owners){
  const rows=await readRustSourceClosureV1([owner.entry],process.cwd(),[],async path=>new Uint8Array(await readFile(path)));
  const independent=await Promise.all(rows.map(async row=>(await readFile(resolve(row.path))).byteLength));
  expect(rows.reduce((n,row)=>n+row.bytes,0)).toBe(independent.reduce((n,size)=>n+size,0));
  expect(independent.reduce((n,size)=>n+size,0)).toBeLessThan(owner.maximumBytes);
  expect(rows.every(row=>resolve(row.path).length<=256)).toBe(true);
  owners.push(new Set(rows.map(row=>row.path)));
 }
 expect([...owners[0]!].filter(path=>owners[1]!.has(path))).toEqual([]);
});
