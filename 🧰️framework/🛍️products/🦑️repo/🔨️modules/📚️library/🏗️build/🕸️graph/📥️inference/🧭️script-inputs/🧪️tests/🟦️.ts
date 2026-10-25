/** 🧭 Contract: a declared target input becomes a normalised workspace-relative pattern exactly as the authored rows and an independent Python derivation say, and the module globs of a target expand to the existing module files the authored rows list: never inside node_modules, .git or target, minus the declared exclusions, sorted and without duplicates, and fast-glob lists the same files for every glob that the declaration admits. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import fastGlob from "fast-glob";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {declaredInputPatternV1,declaredModuleGlobsV1} from "../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
let workspace="";
beforeAll(()=>{
 workspace=realpathSync(mkdtempSync(join(tmpdir(),"declared-inputs-")));
 for(const[path,text]of Object.entries(corpus.tree)){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);}
});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});
const environment={
 readdir:(directory:string)=>{try{return readdirSync(directory,{withFileTypes:true}).map(entry=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}},
 Glob:Bun.Glob as unknown as new(pattern:string)=>{match(path:string):boolean},
 join,
};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,patterns:[{input:"x",projectRoot:"p"}]})).toBe(false);
});

test("a declared input resolves against the project or workspace root, stays inside the workspace and normalises its segments",()=>{
 const answers=ask(corpus.patterns.map(row=>({input:row.input,projectRoot:row.projectRoot})));
 for(const[index,row]of corpus.patterns.entries()){
  const expected=row.expected===null?undefined:row.expected;
  expect(declaredInputPatternV1(row.input,row.projectRoot),JSON.stringify(row.input)+" in "+row.projectRoot).toBe(expected);
  expect(answers[index]===null?undefined:answers[index],JSON.stringify(row.input)+" in "+row.projectRoot).toBe(expected);
 }
});

test("module globs expand to the existing files the authored rows list",()=>{
 for(const row of corpus.globs)expect(declaredModuleGlobsV1(row.inputs,workspace,row.projectRoot,environment),row.name).toEqual(row.expected);
});

test("fast-glob lists the same files for every admitted glob over the same tree",async()=>{
 const suffix=/\.[cm]?[jt]sx?$/u,hasGlob=/[*?{}[\]]/u;
 for(const row of corpus.globs){
  const patterns=row.inputs.map(input=>declaredInputPatternV1(input as never,row.projectRoot)).filter((pattern):pattern is string=>pattern!==undefined&&pattern!==""&&hasGlob.test(pattern)&&suffix.test(pattern));
  const excluded=row.inputs.filter((input):input is string=>typeof input==="string"&&input.startsWith("!")).map(input=>declaredInputPatternV1(input.slice(1),row.projectRoot)).filter((pattern):pattern is string=>pattern!==undefined);
  const listed=patterns.length?await fastGlob(patterns,{cwd:workspace,dot:true,onlyFiles:true,ignore:["**/node_modules/**","**/.git/**","**/target/**",...excluded]}):[];
  expect([...new Set(listed)].sort(),row.name).toEqual(row.expected);
 }
});
