/** 🏷️ Contract: bundle scope labels are emoji clusters plus a lowercase area name; the label module counts clusters, strips reserved emojis, extracts ASCII path tokens and refuses paths, reserved emojis and malformed names with a fixed message; the corpus is language-agnostic and `🐍️oracle.py` classifies emoji by Unicode range and recomputes each verdict. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {bundleScopeLabelError,emojiClusterCountInLine,isBundleScopeLine,labelPathTokens,leadingEmojiClusterCount,normalizeBundleScopeLabel} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.lead)test("leading emoji clusters of "+JSON.stringify(row.line),()=>{expect(leadingEmojiClusterCount(row.line)).toBe(row.expected);});
for(const row of corpus.anywhere)test("emoji clusters anywhere in "+JSON.stringify(row.line),()=>{expect(emojiClusterCountInLine(row.line)).toBe(row.expected);});
for(const row of corpus.normalize)test("scope label normalisation of "+JSON.stringify(row.line),()=>{expect(normalizeBundleScopeLabel(row.line)).toBe(row.expected);});
for(const row of corpus.tokens)test("path tokens of "+JSON.stringify(row.label),()=>{expect(labelPathTokens(row.label)).toEqual(row.expected);});
for(const row of corpus.scopeErrors)test("scope label error: "+row.name,()=>{const error=bundleScopeLabelError(row.line);if(row.message===null)expect(error).toBeNull();else expect(error).toContain(row.message);});
for(const row of corpus.scopeLine)test("scope line "+JSON.stringify(row.line),()=>{expect(isBundleScopeLine(row.line)).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
