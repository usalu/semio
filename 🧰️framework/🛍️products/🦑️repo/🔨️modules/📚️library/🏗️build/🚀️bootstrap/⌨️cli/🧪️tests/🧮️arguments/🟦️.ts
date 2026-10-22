/** ⌨️ Contract: the native recovery selection picks the fixed frontend, maintenance or reset command exactly where the authored protocol says, for every argument vector of `./🔣️.json`, in agreement with an independent Python reading, and closes the install child dependency only for the frontend install. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeRepoRecoveryBuildDescendantsV1,readNativeRepoRecoveryPlanV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),protocol=schema.$defs.Protocol.const,root=findWorkspaceRoot();
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({protocol,rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema and the authored protocol is closed too",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],kind:"frontend",target:null}]})).toBe(false);
 expect(protocol.targets).toEqual(["build","install"]);expect(Object.keys(protocol.maintenance.targets).length).toBe(2);expect(protocol.buildChild).toEqual(["run","repo-mcp:build"]);
});

test("every argument vector selects what the corpus and the Python reading say, with the authored owner directories",()=>{
 const answers=ask(corpus.cases.map(row=>row.args));
 for(const[index,row]of corpus.cases.entries()){
  const plan=readNativeRepoRecoveryPlanV1(root,row.args),expected=answers[index];
  expect<unknown>(plan?.kind??null,row.name).toBe(row.kind);expect<unknown>(plan?.target??null,row.name).toBe(row.target);
  expect(expected?.kind??null,row.name).toBe(row.kind);expect(expected?.target??null,row.name).toBe(row.target);
  if(plan){expect(plan.owner,row.name).toBe(join(root,...expected.owner.split("/")));expect(Object.isFrozen(plan),row.name).toBe(true);}
  expect(nativeRepoRecoveryBuildDescendantsV1(root,row.args),row.name).toEqual(row.kind==="frontend"&&row.target==="install"?[protocol.buildChild]:[]);
 }
 expect(new Set(corpus.cases.map(row=>row.kind)).size).toBe(4);
});
