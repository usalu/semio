/** 🔁️ Contract: the graph retry diagnostic projects only bounded identities and digests: it equals the unit's own corpus in `./🔣️.json`, the extra corpus `./📋️extra.json` and an independent Python projection, redacts paths outside the root, never reads what is not a file inside the root, truncates to the policy bounds and is valid for ajv's reading of the unit's own diagnostic schema. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join,relative,resolve,sep} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildGraphRetryDiagnosticV1} from "../../🟨️.mjs";
import schema from "../../🧬️schema/🔣️.json";
import own from "../🔣️.json";
import corpus from "./📋️extra.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),posix=process.platform!=="win32",root=resolve(corpus.root);
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({root:corpus.root,rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const build=(row:{reason:string;before:string|null;after:string|null;stale:any[];current:Record<string,string>},seen:string[]=[])=>buildGraphRetryDiagnosticV1(root,row.reason,{before:row.before,after:row.after,stale:row.stale},(path:string)=>{const key=relative(root,path).split(sep).join("/");seen.push(key);return row.current[key]??null;});
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};

test("the corpora are closed by their schemas and every authored diagnostic is valid",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({$defs:schema.$defs,$ref:"#/$defs/Fixture"},own),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],current:{a:1}}]})).toBe(false);
 expect(schema.$defs.Policy.const).toEqual({maximumFacts:8,maximumPathUnits:256,maximumFactBytes:65536});
});

test("the unit's own cases produce their expected diagnostics",()=>{
 for(const row of own.cases)expect<unknown>(build({reason:row.reason,before:row.before,after:row.after,stale:row.stale,current:{[row.stale[0]?.path??""]:row.current as string}}),row.name).toEqual(row.expected);
});

test.skipIf(!posix)("every extra case is projected like the Python projection with the authored refusals, and every projection is a valid diagnostic",()=>{
 const answers=ask(corpus.cases),ajv=new Ajv({strict:false}),validate=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Diagnostic"});let redacted=0,truncated=0;
 for(const[index,row]of corpus.cases.entries()){
  let value:any=null;const message=refusal(()=>{value=build(row as never);});
  expect(message,row.name).toBe(row.error);expect(answers[index].error??"",row.name).toBe(row.error);
  if(row.error===""){expect(value,row.name).toEqual(answers[index].value);expect(validate(value),row.name+JSON.stringify(validate.errors)).toBe(true);if(value.facts.some((fact:any)=>fact.path===null))redacted++;if(row.stale.length>8&&value.facts.length===8)truncated++;}
 }
 expect(redacted).toBeGreaterThan(3);expect(truncated).toBe(1);
});

test.skipIf(!posix)("the current digest is asked only for files inside the root and never for directories or redacted paths",()=>{
 const seen:string[]=[];
 build({reason:"observed-facts",before:null,after:null,current:{},stale:[{kind:"file",path:"a.ts",fact:{sha256:"a".repeat(64)}},{kind:"directory",path:"d",fact:{}},{kind:"file",path:"../x.ts",fact:{}},{kind:"file",path:"/elsewhere/y.ts",fact:{}},{kind:"file",path:"p".repeat(300),fact:{}}]},seen);
 expect(seen).toEqual(["a.ts"]);
 expect(refusal(()=>buildGraphRetryDiagnosticV1(root,"observed-facts",{stale:[{kind:"file",path:"a.ts",fact:{}}]},()=>{throw Error("unreadable");}))).toBe("");
 expect(buildGraphRetryDiagnosticV1(root,"observed-facts",{stale:[{kind:"file",path:"a.ts",fact:{}}]},()=>{throw Error("unreadable");}).facts[0]!.after).toBeNull();
 expect(buildGraphRetryDiagnosticV1(root,"membership").facts).toEqual([]);
});
