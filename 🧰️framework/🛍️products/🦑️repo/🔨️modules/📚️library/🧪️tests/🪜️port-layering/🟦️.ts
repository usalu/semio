import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv/dist/2020";
import ts from "typescript";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "./📋️schema.json";
import fixture from "./🧫️cases.json";

const library=fileURLToPath(new URL("../../",import.meta.url));
/** 🔎️ Our own scanner of literal import, dynamic import, require and re-export specifiers. */
const ownSpecifiers=(text:string):string[]=>[...new Set([...text.matchAll(/(?:\bfrom|\bimport|\bimport\s*\(|\brequire\s*\()\s*(["'`])([^"'`\n]+)\1/gu)].map(match=>match[2]!))].sort();
/** 🔮️ The TypeScript compiler's own scan of the same specifiers, the independent oracle. */
const oracleSpecifiers=(text:string):string[]=>[...new Set(ts.preProcessFile(text,true,true).importedFiles.map(file=>file.fileName))].sort();
const targets=(owner:string,specifiers:readonly string[]):string[]=>specifiers.filter(specifier=>specifier.startsWith(".")).map(specifier=>resolve(dirname(join(library,owner)),specifier.split("?")[0]!)).sort();

test("the port layering table is closed data",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);
});

test("both scanners see every kind of import specifier",()=>{
 const text="import a from './a.ts';import type {B} from \"./b.ts\";export {c} from './c.ts';const d=await import('./d.ts');const e=require('./e.ts');import 'node:fs';";
 expect(ownSpecifiers(text)).toEqual(oracleSpecifiers(text));
 expect(ownSpecifiers(text)).toEqual(["./a.ts","./b.ts","./c.ts","./d.ts","./e.ts","node:fs"]);
});

test("no former knot owner imports a layer above it, statically or dynamically",()=>{
 for(const row of fixture.owners){
  const text=readFileSync(join(library,row.owner),"utf8"),own=ownSpecifiers(text),oracle=oracleSpecifiers(text);
  expect(own,row.id).toEqual(oracle);
  const reached=targets(row.owner,own);
  for(const forbidden of row.forbidden)expect(reached,row.id+" -> "+forbidden).not.toContain(resolve(library,forbidden));
 }
});
