/** 🧱️ Contract: a fact-table block is compressed cheaply and a refused block is never recompressed within the refusal window. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),start=source.indexOf("function persistInferenceBlockV1("),end=source.indexOf("/** 🧾️ The header text of a record",start),body=source.slice(start,end);

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);expect(start).toBeGreaterThan(-1);expect(end).toBeGreaterThan(start);});
test("blocks are compressed at a bounded quality, after the existence and refusal checks",()=>{
 expect(body).toContain("zlibConstants.BROTLI_PARAM_QUALITY]:5");
 const exists=body.indexOf("fs.existsSync(path)"),refused=body.indexOf("blocksRefused.get(directory)"),pack=body.indexOf("brotliCompressSync(");
 expect(exists).toBeGreaterThan(-1);expect(refused).toBeGreaterThan(exists);expect(pack).toBeGreaterThan(refused);
 expect(body.indexOf("blocksRefused.set(directory")).toBeGreaterThan(pack);
 expect(body.match(/brotliCompressSync\(/gu)?.length).toBe(1);
});
