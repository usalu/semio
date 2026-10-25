/** 👁️ Contract: the Bazel evidence directory, flag, events path and closure document equal the authored cases in `./🔣️.json` and an independent Python derivation, refuse relative or over-long directories with the authored message and write the closure file where the parity verifier looks for it. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {isAbsolute,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bazelEvidenceDirectoryV1,bazelEvidenceEventsPathV1,bazelEvidenceFlagsV1,writeBazelClosureEvidenceV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"bazel-evidence-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});
const environmentOf=(spec:{kind:string;name?:string;padTo?:number}):Record<string,string|undefined>=>{
 if(spec.kind==="unset")return{};if(spec.kind==="empty")return{SEMIO_BUILD_EVIDENCE_DIR:""};
 if(spec.kind==="relative")return{SEMIO_BUILD_EVIDENCE_DIR:spec.name!};
 const base=join(workspace,spec.name!),pad=spec.padTo?Math.max(0,spec.padTo-base.length):0;return{SEMIO_BUILD_EVIDENCE_DIR:spec.padTo?base.slice(0,Math.min(base.length,spec.padTo))+"x".repeat(pad):base};
};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,directories:[{...corpus.directories[0],expected:"maybe"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,closures:[{...corpus.closures[0],plan:{tasks:{},selected:[]}}]})).toBe(false);
});

test("the evidence directory is selected, refused at the character limit and drives the flag and the events path",()=>{
 for(const row of corpus.directories){
  const environment=environmentOf(row.env);let directory:string|undefined;
  const message=refusal(()=>{directory=bazelEvidenceDirectoryV1(environment);});
  expect(message,row.name).toBe(row.error);
  if(row.error===""){
   if(row.expected==="none"){expect(directory,row.name).toBeUndefined();expect(bazelEvidenceFlagsV1(directory),row.name).toEqual([]);}
   else{expect(directory,row.name).toBe(environment.SEMIO_BUILD_EVIDENCE_DIR);expect(isAbsolute(directory!),row.name).toBe(true);expect(bazelEvidenceFlagsV1(directory)).toEqual(["--execution_log_json_file="+join(directory!,"bazel.execlog.jsonl")]);expect(bazelEvidenceEventsPathV1(directory!)).toBe(join(directory!,"bazel.bep.jsonl"));}
  }
 }
 const exact=environmentOf({kind:"absolute",name:"e",padTo:200}).SEMIO_BUILD_EVIDENCE_DIR!;expect(exact.length).toBe(200);
 expect(refusal(()=>bazelEvidenceDirectoryV1({SEMIO_BUILD_EVIDENCE_DIR:exact+"x"}))).not.toBe("");
});

test("the closure document equals the Python derivation, asks the one closure query and is written under the evidence directory",async()=>{
 const answers=ask(corpus.closures);
 for(const[index,row]of corpus.closures.entries()){
  const plan={selected:row.plan.selected,tasks:Object.fromEntries(Object.entries(row.plan.tasks).map(([id,task])=>[id,{label:task.label,definition:{execution:{env:task.env===null?undefined:Object.fromEntries(task.env.map((name:string)=>[name,"1"]))}}}]))},asked:string[]=[],directory=join(workspace,"closure",String(index),"deep");
  await writeBazelClosureEvidenceV1(directory,{plan:plan as never,query:async expression=>{asked.push(expression);return row.answer;}});
  expect(asked,row.name).toEqual([row.expression]);
  const written=JSON.parse(readFileSync(join(directory,"bazel.closure.json"),"utf8"));
  expect(written,row.name).toEqual(answers[index]);expect(Object.keys(written),row.name).toEqual(["version","labelToTask","queried","declaredEnvironment"]);
 }
});
