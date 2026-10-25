/** 🖥️ Contract: the repo CLI client resolves its binaries from the environment or the cache by build mode, passes the query and variables as one closed argument vector and returns the last JSON line's `data`, failing with a fixed message on a non-zero exit, GraphQL errors or no JSON; the corpus is language-agnostic and `🐍️.py` re-derives the last-line and error rules. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {chmodSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {resolve} from "node:path";
import {fixtureItemsOf,getLibRoot,resolveCliBin,resolveMcpBin,runCliGraphql} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"cli-")));roots.push(root);return root;};
const withEnv=<T,>(values:Record<string,string|undefined>,action:()=>T):T=>{const saved:Record<string,string|undefined>={};for(const key of Object.keys(values)){saved[key]=process.env[key];if(values[key]===undefined)delete process.env[key];else process.env[key]=values[key];}try{return action();}finally{for(const[key,value]of Object.entries(saved))if(value===undefined)delete process.env[key];else process.env[key]=value;}};
const executable=(root:string,spec:{lines:string[];exit:number;stderr?:string}):string=>{const path=join(root,"fake-repo");writeFileSync(path,"#!/usr/bin/env node\n"+"const lines="+JSON.stringify(spec.lines)+";for(const line of lines)console.log(line);if("+spec.exit+")process.stderr.write("+JSON.stringify(spec.stderr??"")+");process.exit("+spec.exit+");\n");chmodSync(path,0o755);return path;};
for(const[index,row]of corpus.items.entries())test("fixture items of node "+index,()=>{expect<unknown>(fixtureItemsOf(row.node)).toEqual(row.expected);});
test("the binaries resolve from the environment, else from the cache by build mode",()=>{
 const root=scratch(),directory=join(root,...corpus.defaultSuffix);
 withEnv({REPO_CLI_BIN:undefined,REPO_MCP_BIN:undefined,SEMIO_BUILD_MODE:undefined},()=>{
  for(const mode of corpus.modes as ("prototype"|"release")[]){const expected=join(directory,mode,"repo","repo"+(process.platform==="win32"?".exe":""));expect(resolveCliBin(root,mode)).toBe(expected);expect(resolveMcpBin(root,mode)).toBe(expected);}
  expect(resolveCliBin(root)).toBe(join(directory,"prototype","repo","repo"+(process.platform==="win32"?".exe":"")));expect(()=>resolveCliBin(root,corpus.badMode as never)).toThrow("Build mode must be prototype or release");
 });
 withEnv({REPO_CLI_BIN:"  "+join(root,"custom-cli")+"  ",REPO_MCP_BIN:"relative/mcp"},()=>{expect(resolveCliBin(root,"release")).toBe(join(root,"custom-cli"));expect(resolveMcpBin(root,"release")).toBe(resolve("relative/mcp"));});
 withEnv({REPO_CLI_BIN:"   ",REPO_MCP_BIN:""},()=>{expect(resolveCliBin(root,"release")).toBe(join(directory,"release","repo","repo"+(process.platform==="win32"?".exe":"")));});
});
test("the library root is the neighbouring packages directory",()=>{expect(getLibRoot().endsWith("\u{1F4E6}\ufe0fpackages")).toBe(true);});
for(const[name,spec]of Object.entries(corpus.fake))test("graphql through the CLI: "+name,()=>{
 if(process.platform==="win32")return;
 const root=scratch(),bin=executable(root,spec as never);
 withEnv({REPO_CLI_BIN:bin},()=>{const run=()=>runCliGraphql("query { x }",{v:1},{repoRoot:root,cwd:root});const row=spec as {expected?:unknown;message?:string};if(row.message!==undefined)expect(run).toThrow(row.message);else expect(run()).toEqual(row.expected);});
});
test("the query, variables and repository are passed as arguments",()=>{
 if(process.platform==="win32")return;
 const root=scratch(),path=join(root,"echo-repo");writeFileSync(path,"#!/usr/bin/env node\nconsole.log(JSON.stringify({data:{argv:process.argv.slice(2),cwd:process.cwd()}}));\n");chmodSync(path,0o755);
 withEnv({REPO_CLI_BIN:path},()=>{const data=runCliGraphql("query Q { y }",{a:1},{repoRoot:root,cwd:root}) as {argv:string[];cwd:string};expect(data.argv).toEqual(["--repo",root,"--json","graphql","--query","query Q { y }","-v",'{"a":1}']);expect(realpathSync(data.cwd)).toBe(root);});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
