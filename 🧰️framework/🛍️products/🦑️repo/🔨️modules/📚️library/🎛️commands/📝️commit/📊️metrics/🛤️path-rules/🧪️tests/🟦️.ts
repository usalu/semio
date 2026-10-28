/** 🛤️ Contract: path rules of the commit metrics keep the same skip list, language buckets and emoji for every path; the corpus is language-agnostic and `🐍️.py` classifies the same paths independently. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {MAX_METRICS_FILE_BYTES,classifyPathForMetrics,langMetricsEmoji,normalizeRepoPath,shouldSkipPathForUloc} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.normalize)test("normalize: "+JSON.stringify(row.path),()=>{expect(normalizeRepoPath(row.path)).toBe(row.expected);});
for(const row of corpus.skip)test("skip: "+JSON.stringify(row.path),()=>{expect(shouldSkipPathForUloc("/repo",row.path)).toBe(row.expected);});
for(const row of corpus.skipDefects)test.failing("skip defect: "+row.path,()=>{expect(shouldSkipPathForUloc("/repo",row.path)).toBe(row.expected);});
for(const row of corpus.classify)test("classify: "+JSON.stringify(row.path),()=>{expect(classifyPathForMetrics(row.path)).toBe(row.expected);});
for(const row of corpus.emoji)test("emoji: "+JSON.stringify(row.lang),()=>{expect(langMetricsEmoji(row.lang)).toBe(row.expected);});
test("the metrics byte ceiling is eight mebibytes",()=>{expect(MAX_METRICS_FILE_BYTES).toBe(8*1024*1024);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
