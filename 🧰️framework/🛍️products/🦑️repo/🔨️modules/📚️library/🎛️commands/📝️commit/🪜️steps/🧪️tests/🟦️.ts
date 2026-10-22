/** 🪜️ Contract: commit steps come from explicit `ct`/`cs`/`cp` words or, failing those, from the contributor's configured level, and prepare-only means no step at all or an explicit prepare word; the corpus is language-agnostic and `🐍️oracle.py` applies the same word-boundary grammar. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {isCommitPrepareOnly,loadCommitSteps,parseCommitSteps} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"steps-")));roots.push(root);return root;};
const configure=(root:string,level:string|null):void=>{const folder=join(root,".\u{1F9EC}semio","\u{1F991}\ufe0frepo",corpus.devsDir,corpus.contributor.alias);mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"commit.json"),JSON.stringify(level===null?{}:{level}));};
for(const row of corpus.parse)test("steps of "+JSON.stringify(row.segments),()=>{expect(parseCommitSteps(row.segments)).toEqual(row.expected);});
for(const row of corpus.parseDefects)test.failing("steps defect "+JSON.stringify(row.segments),()=>{expect(parseCommitSteps(row.segments)).toEqual(row.expected);});
for(const row of corpus.prepareOnly)test("prepare-only of "+JSON.stringify(row.segments),()=>{expect(isCommitPrepareOnly(row.segments)).toBe(row.expected);});
for(const row of corpus.prepareOnlyDefects)test.failing("prepare-only defect "+JSON.stringify(row.segments),()=>{expect(isCommitPrepareOnly(row.segments)).toBe(row.expected);});
for(const row of corpus.levels)test("configured level "+row.level,()=>{const root=scratch();configure(root,row.level);expect(loadCommitSteps(root,corpus.contributor,[])).toEqual(row.expected);});
test("explicit steps override the configured level and a missing file means no steps",()=>{
 const root=scratch(),bare=scratch();configure(root,"prepare-and-tag-and-squash-and-push");
 expect(loadCommitSteps(root,corpus.contributor,["ct"])).toEqual({tag:true,squash:false,push:false});expect(loadCommitSteps(bare,corpus.contributor,[])).toEqual({tag:false,squash:false,push:false});expect(loadCommitSteps(bare,corpus.contributor,["cs","cp"])).toEqual({tag:false,squash:true,push:true});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
