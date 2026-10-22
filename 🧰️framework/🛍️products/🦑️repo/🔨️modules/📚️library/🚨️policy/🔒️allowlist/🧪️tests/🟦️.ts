/** 🔒️ Contract: a policy rule's allowlist is the union of the entries every area contributes through its own allowlist file, discovered by convention while build output, caches, tickets and generated trees are skipped and unreadable or mistyped files fail closed; the corpus is language-agnostic and `🐍️oracle.py` walks the same tree. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {POLICY_ALLOWLIST_FILENAME,policyDiscoveredAllowlist} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const build=(files:Record<string,string>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"allowlist-")));roots.push(root);for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
test("the contribution filename is the documented one",()=>{expect(POLICY_ALLOWLIST_FILENAME).toBe(corpus.filename);});
test("every area contributes to a rule, skipped trees, unreadable files, wrong types and other names contribute nothing",()=>{
 const root=build(corpus.files);for(const[key,expected]of Object.entries(corpus.expected))expect([...policyDiscoveredAllowlist(root,key)].sort(),key).toEqual(expected);
});
test("a missing repository root contributes nothing",()=>{expect(policyDiscoveredAllowlist(join(tmpdir(),"definitely-missing-"+process.pid),"rule").size).toBe(0);});
for(const row of corpus.defects)test.failing("allowlist defect: "+row.name,()=>{expect([...policyDiscoveredAllowlist(build(row.files),row.key)]).toEqual(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
