/** ⌨️ Contract: the native recovery selection picks the fixed frontend, maintenance or reset command exactly where the authored protocol says, for every argument vector of `./🔣️.json`, in agreement with an independent Python reading, and closes the install child dependency only for the frontend install. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join,dirname} from "node:path";
import {mkdirSync,mkdtempSync,realpathSync,writeFileSync,readFileSync,rmSync,lstatSync,linkSync,unlinkSync} from "node:fs";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {nativeRepoRecoveryBuildDescendantsV1,readNativeRepoRecoveryPlanV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),protocol=schema.$defs.Protocol.const;
let root="";
beforeAll(()=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Caller-owned recovery test artifacts required");mkdirSync(artifacts,{recursive:true});root=realpathSync(mkdtempSync(join(artifacts,"r")));for(const [owner,manifest]of [[protocol.owner,corpus.owners.frontend],[protocol.maintenance.owner,corpus.owners.maintenance],[protocol.reset.owner,corpus.owners.reset]]as const){const file=join(root,owner,"📋️project.json");mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(manifest),{flag:"wx"});}});
afterAll(()=>{if(root)rmSync(root,{recursive:true,force:true});});
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({protocol,rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema and the authored protocol is closed too",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],kind:"frontend",target:null}]})).toBe(false);
 for(const [name,owner]of [["Owner",corpus.owners.frontend],["MaintenanceOwner",corpus.owners.maintenance],["ResetOwner",corpus.owners.reset]]as const)expect(ajv.validate({$defs:schema.$defs,$ref:"#/$defs/"+name},owner),JSON.stringify(ajv.errors)).toBe(true);
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


test("private recovery owners retain ordinary single-link bounded physical Source custody",()=>{for(const group of [protocol.maintenance,protocol.reset]){const request=group.project+":"+Object.keys(group.targets)[0],file=join(root,group.owner,"📋️project.json"),bytes=readFileSync(file),alias=join(dirname(file),"l");expect(bytes.length).toBeLessThanOrEqual(protocol.maximumOwnerBytes);expect(lstatSync(file).nlink).toBe(1);for(const refusal of corpus.refusals){try{if(refusal==="linked")linkSync(file,alias);else writeFileSync(file,Buffer.concat([bytes,Buffer.alloc(protocol.maximumOwnerBytes+1-bytes.length,32)]));expect(()=>readNativeRepoRecoveryPlanV1(root,["run",request]),refusal).toThrow("Bounded physical Source revision required");}finally{if(refusal==="linked")unlinkSync(alias);else writeFileSync(file,bytes);}}expect(readNativeRepoRecoveryPlanV1(root,["run",request])?.kind).toBe(group===protocol.reset?"cache-reset":"maintenance");}});

test("actual recovery commands use bounded domain owners with every authored profile",()=>{
 const ajv=new Ajv({strict:false});
 for(const [group,name,definition]of [[protocol.maintenance,"repo-cache-retirement","MaintenanceOwner"],[protocol.reset,"repo-cache-reset","ResetOwner"]]as const){
  expect(group.project).toBe(name);const file=join(process.cwd(),group.owner,"📋️project.json"),bytes=readFileSync(file),owner=JSON.parse(bytes.toString("utf8"));
  expect(bytes.length).toBeLessThanOrEqual(protocol.maximumOwnerBytes);expect(lstatSync(file).nlink).toBe(1);expect(ajv.validate({$defs:schema.$defs,$ref:"#/$defs/"+definition},owner),JSON.stringify(ajv.errors)).toBe(true);
  for(const [target,profile]of Object.entries(group.targets)){expect(owner.targets[target]).toBeDefined();expect(readNativeRepoRecoveryPlanV1(process.cwd(),["run",name+":"+target])?.target).toBe(profile);}
 }
});
