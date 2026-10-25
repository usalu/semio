/** 📅️ Contract: bundle day lines are read from the micro-commit body timestamp (else the subject), stripped of hand-written metrics and re-rendered with the computed metric suffixes, and the bundle subject and date patterns accept exactly their spelling; the corpus is language-agnostic and `🐍️.py` recomputes each line with exact-decimal metric formatting. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {BUNDLE_DATE_SECTION_RE,BUNDLE_WIP_SUBJECT_RE} from "../../🧾️bundle-model/🟦️.ts";
import {extractBundleDateLineFromCommit,extractBundleDateLineFromCommitBody,extractBundleDateLineFromSubject,formatBundleDateLine,normalizeBundleDateLine} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.subject)test("subject day of "+JSON.stringify(row.subject),()=>{expect(extractBundleDateLineFromSubject(row.subject)).toBe(row.expected);});
for(const row of corpus.body)test("body day of "+JSON.stringify(row.body),()=>{expect(extractBundleDateLineFromCommitBody(row.body)).toBe(row.expected);});
for(const row of corpus.commit)test("commit day of "+JSON.stringify(row.subject)+" + "+JSON.stringify(row.body),()=>{expect(extractBundleDateLineFromCommit(row.subject,row.body)).toBe(row.expected);});
for(const row of corpus.normalize)test("date line normalisation of "+JSON.stringify(row.line),()=>{expect(normalizeBundleDateLine(row.line)).toBe(row.expected);});
for(const row of corpus.format)test("formatted date line "+JSON.stringify(row.line),()=>{expect(formatBundleDateLine(row.line,row.ulocDelta,row.sizeDelta,row.ulocBloc,row.sizeBloc)).toBe(row.expected);});
for(const row of corpus.bundleModel.wip)test("bundle subject pattern on "+JSON.stringify(row.subject),()=>{const match=BUNDLE_WIP_SUBJECT_RE.exec(row.subject);expect(match===null?null:match[1]).toBe(row.expected);});
for(const row of corpus.bundleModel.date)test("bundle date section pattern on "+JSON.stringify(row.line),()=>{expect(BUNDLE_DATE_SECTION_RE.test(row.line)).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
