/** 🔖️ Contract: the bundle tag and subject are `<emoji><alias>🎆️YY🌙️MM☀️DD` plus the tag or squash marker, and the newest squash subject in the log is found by its own pattern; the corpus is language-agnostic and `🐍️oracle.py` rebuilds the names and replays the log with the git CLI. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {findLastBundleWipCommit,formatBundleSubject,formatBundleTagName} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const history=(subjects:string[]):{root:string;shas:string[]}=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"tags-")));git(root,"init","-q");const shas:string[]=[];for(const subject of subjects){git(root,"commit","-q","--allow-empty","-m",subject);shas.push(git(root,"rev-parse","HEAD"));}return{root,shas};};
for(const[index,row]of corpus.names.entries())test("tag and subject names "+index,()=>{const now=new Date(row.now[0]!,row.now[1]!-1,row.now[2]!);expect(formatBundleTagName(corpus.contributor,now)).toBe(row.expectedTag);expect(formatBundleSubject(corpus.contributor,now)).toBe(row.expectedSubject);});
test("the newest bundle subject is found in the log",()=>{
 const{root,shas}=history(corpus.repo.commits),empty=history(corpus.repo.none);
 try{
  const found=findLastBundleWipCommit(root);expect(found).not.toBeNull();expect(found!.sha).toBe(shas[corpus.repo.expectedIndex]);expect(found!.subject).toBe(corpus.repo.commits[corpus.repo.expectedIndex]);
  expect(findLastBundleWipCommit(empty.root)).toBeNull();expect(findLastBundleWipCommit(join(empty.root,"missing"))).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});rmSync(empty.root,{recursive:true,force:true});}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
