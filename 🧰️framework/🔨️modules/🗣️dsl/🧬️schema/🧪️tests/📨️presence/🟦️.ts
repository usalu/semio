import {readFileSync} from "node:fs";
import {describe,expect,it} from "bun:test";
import Ajv from "ajv";

const corpus=JSON.parse(readFileSync(new URL("../../🧪️fixtures/📨️presence/🔣️.json",import.meta.url),"utf8"));
const validate=new Ajv({strict:true}).compile(corpus.schema);

describe("required nullable and optional supplied fields",()=>{
  for(const row of [...corpus.cases,...corpus.tableCases]) it(row.id,()=>{
    expect(validate(row.json)).toBe(row.accepted);
    if(row.accepted){const retained=structuredClone(row.json);expect(Object.hasOwn(retained,"factor")).toBe(true);expect(Object.hasOwn(retained,"index")).toBe(Object.hasOwn(row.json,"index"));expect(retained).toEqual(row.json);}
  });
});
