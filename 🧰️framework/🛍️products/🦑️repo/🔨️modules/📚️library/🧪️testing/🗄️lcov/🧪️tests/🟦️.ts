/** 🗄️ Contract: LCOV text is parsed into per-file line hits (only `SF:`, `DA:` and `end_of_record`), merged by summing hits, rendered with sorted lines, and Go cover profiles expand each statement range into per-line hits; the corpus is language-agnostic and `🐍️.py` is an independent LCOV/Go-profile model. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {goProfileToLcov,mergeLcov,parseLcov,renderLcov} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const flat=(records:{path:string;lines:Map<number,number>}[])=>records.map(record=>({path:record.path,lines:[...record.lines.entries()]}));
const asMap=(rows:[string,[number,number][]][]):Map<string,Map<number,number>>=>new Map(rows.map(([path,lines])=>[path,new Map(lines)]));
const fromRows=(rows:{path:string;lines:number[][]}[])=>rows.map(row=>({path:row.path,lines:new Map(row.lines.map(pair=>[pair[0]!,pair[1]!] as [number,number]))}));
for(const row of corpus.parse)test("parse: "+row.name,()=>{expect<unknown>(flat(parseLcov(row.text))).toEqual(row.expected);});
test("merging sums the hits of the same line across runs",()=>{const merged=mergeLcov(corpus.merge.sets.map(fromRows));expect<unknown>([...merged.entries()].map(([path,lines])=>[path,[...lines.entries()]])).toEqual(corpus.merge.expected);expect(mergeLcov([]).size).toBe(0);});
test("rendering sorts lines and keeps file order",()=>{expect(renderLcov(asMap(corpus.render.merged as [string,[number,number][]][]))).toBe(corpus.render.expected);expect(renderLcov(new Map())).toBe(corpus.render.empty);});
test("a rendered report parses back to the merged map",()=>{const merged=mergeLcov(corpus.merge.sets.map(fromRows));expect(mergeLcov([parseLcov(renderLcov(merged))])).toEqual(merged);});
for(const row of corpus.go)test("go profile: "+row.name,()=>{expect<unknown>(flat(goProfileToLcov(row.text))).toEqual(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
