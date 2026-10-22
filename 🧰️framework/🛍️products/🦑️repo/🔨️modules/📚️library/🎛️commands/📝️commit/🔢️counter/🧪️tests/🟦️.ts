/** 🔢️ Contract: the micro-commit counter reads `…🚩️NNN` subjects and bundle tags, continues the highest counter of the contributor on the right epoch and refuses to reset on an unparsable prior; the corpus is language-agnostic and `🐍️oracle.py` recomputes every verdict on selector-stripped text. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {bumpCounterFromHistory,bumpCounterFromSubject,canonicalWipLine1Base,contributorWipLine1Base,extractCounterFromSubject,extractNumericCounterFromSubject,formatSecond,line1BaseFromBundleTag,nextCounter,pad2} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const history=(commits:string[],tags:string[]):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"counter-")));git(root,"init","-q");for(const subject of commits)git(root,"commit","-q","--allow-empty","-m",subject);for(const tag of tags)git(root,"tag",tag);return root;};
const when=(parts:number[]):Date=>new Date(parts[0]!,parts[1]!-1,parts[2]!,parts[3]??0,parts[4]??0,parts[5]??0);
for(const row of corpus.extract)test("extract counter of "+JSON.stringify(row.subject),()=>{expect(extractCounterFromSubject(row.subject)).toEqual(row.expected);});
for(const row of corpus.numeric)test("numeric counter of "+JSON.stringify(row.subject),()=>{expect(extractNumericCounterFromSubject(row.subject)).toBe(row.expected);});
for(const row of corpus.tag)test("bundle tag epoch of "+JSON.stringify(row.tag),()=>{expect(line1BaseFromBundleTag(row.tag)).toBe(row.expected);});
for(const row of corpus.bump)test("bump: "+row.name,()=>{expect(bumpCounterFromHistory(row.subjects,row.contributor,when(row.now),row.wip)).toEqual(row.expected);});
for(const row of corpus.bumpErrors)test("bump fails: "+row.name,()=>{expect(()=>bumpCounterFromHistory(row.subjects,row.contributor,when([2026,10,21]))).toThrow(row.message);});
for(const row of corpus.bumpDefects)test.failing("bump defect: "+row.name,()=>{expect(bumpCounterFromHistory(row.subjects,row.contributor,when(row.now),row.wip)).toEqual(row.expected);});
for(const row of corpus.canonical)test("canonical epoch of "+JSON.stringify(row.base),()=>{expect(canonicalWipLine1Base(row.base,row.contributor)).toBe(row.expected);});
for(const row of corpus.canonicalDefects)test.failing("canonical epoch defect "+JSON.stringify(row.base),()=>{expect(canonicalWipLine1Base(row.base,row.contributor)).toBe(row.expected);});
for(const row of corpus.second)test("second stamp "+row.expected,()=>{expect(formatSecond(when(row.now))).toBe(row.expected);});
for(const row of corpus.pad)test("pad "+row.n,()=>{expect(pad2(row.n)).toBe(row.expected);});
test("a single-subject bump equals the history bump",()=>{const row=corpus.bump[1]!;expect(bumpCounterFromSubject(row.subjects[0]!,row.contributor,when(row.now))).toEqual(bumpCounterFromHistory([row.subjects[0]!],row.contributor,when(row.now)));});
test("the repository log and tags drive the persisted epoch and the next counter",()=>{
 const found=history(corpus.repo.commits,corpus.repo.tags),tagged=history(corpus.repo.tagOnly.commits,corpus.repo.tagOnly.tags),empty=history(corpus.repo.none.commits,corpus.repo.none.tags);
 try{
  expect(contributorWipLine1Base(found,corpus.contributor)).toBe(corpus.repo.expectedWip);expect(nextCounter(found,corpus.contributor)).toEqual(corpus.repo.expectedNext);
  expect(contributorWipLine1Base(tagged,corpus.contributor)).toBe(corpus.repo.tagOnly.expectedWip);expect(contributorWipLine1Base(empty,corpus.contributor)).toBeNull();
 }finally{for(const root of[found,tagged,empty])rmSync(root,{recursive:true,force:true});}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
