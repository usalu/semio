import leaseStatusSchema from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/📋️status/🧬️schema/🔣️.json";
import {hostFloorMegabytesV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../../🌡️host-capacity/🎛️control/🟦️.ts";
/** 🔒️ Contract: `repo leases status` prints text or one JSON document for the repository lease directory, refuses unknown options and operations with the authored messages, and the first text line equals the one an independent Python summary rebuilds from the JSON document; the admission section keeps at least the authored floor and is ajv-valid. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";

import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {readLeaseAdmissionV1,REPOSITORY_LEASE_DIRECTORY_V1,runLeasesCliV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(document:unknown):{summary:string;problems:string[]}=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({document,floor:corpus.floorMegabytes}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
let root="";
beforeAll(()=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Caller-owned lease status test artifacts required");mkdirSync(artifacts,{recursive:true});root=realpathSync(mkdtempSync(join(artifacts,"leases-cli-")));mkdirSync(join(root,...REPOSITORY_LEASE_DIRECTORY_V1.split("/")),{recursive:true});});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});
const run=async(args:string[],environment:NodeJS.ProcessEnv=corpus.environment)=>{const lines:string[]=[];try{return{code:await runLeasesCliV1(root,args,environment,line=>lines.push(line)),lines};}catch(error){return{error:(error as Error).message,lines};}};

test("the corpus is closed by its schema and the lease directory is the one every repo lease uses",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,commands:[{args:["x"],outcome:"throw"}]})).toBe(false);
 expect(REPOSITORY_LEASE_DIRECTORY_V1).toBe(corpus.leaseDirectory);
});

test("every command yields text, one JSON document or the authored refusal, and the text summary equals the Python summary of the JSON document",async()=>{
 const initial=await run(["status","--json"]);expect(initial.error).toBeUndefined();const document=initial.lines.join("\n"),parsed=JSON.parse(document),expected=ask(parsed);
 expect(expected.problems).toEqual([]);expect(parsed.directory).toBe(join(root,...corpus.leaseDirectory.split("/")));
 for(const row of corpus.commands){
  const result=await run(row.args,row.environment??corpus.environment),label=row.args.join(" ");
  if(row.outcome==="throw"){if(row.messageIncludes){expect(result.error,label).toContain(row.messageIncludes);expect(result.error,label).toContain("Keine Sprache für Texte gewählt.");}else expect(result.error,label).toBe(row.message);expect(result.lines,label).toEqual([]);continue;}
  expect(result.error,label).toBeUndefined();expect(result.code,label).toBe(0);
  if(row.outcome==="json"){const value=JSON.parse(result.lines.join("\n"));expect(ask(value).problems,label).toEqual([]);expect(value.holders,label).toEqual([]);}
  else{expect(result.lines[0],label).toBe(expected.summary);expect(result.lines.length,label).toBeGreaterThan(1);}
 }
});

test("the admission section holds the authored minimum floor, is closed and is omitted instead of failing when it cannot be read",async()=>{
 const admission=await readLeaseAdmissionV1({},"en",Date.now());
 if(admission!==undefined){
  const ajv=new Ajv({strict:false});
  expect(ajv.validate({definitions:leaseStatusSchema.definitions,...leaseStatusSchema.definitions.Admission},JSON.parse(JSON.stringify(admission))),JSON.stringify(ajv.errors)).toBe(true);expect(admission.floorMegabytes).toBeGreaterThanOrEqual(corpus.floorMegabytes.minimum);expect(admission.floorMegabytes).toBeLessThanOrEqual(corpus.floorMegabytes.maximum);
  expect(Object.isFrozen(admission)).toBe(true);
 }
 const unreadable=await readLeaseAdmissionV1({PATH:""} as never,"de",0);expect(unreadable===undefined||typeof unreadable.floorMegabytes==="number").toBe(true);
});


test("status floor follows the authored host-size law and independent Python vectors",()=>{const memory=BUILD_HOST_CAPACITY_POLICY_V1.memory;expect({minimum:memory.floorMinimumMegabytes,maximum:memory.floorMaximumMegabytes,basisPoints:memory.floorBasisPoints}).toEqual(corpus.floorMegabytes);const result=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({samples:corpus.floorCases,floor:corpus.floorMegabytes}),encoding:"utf8"});expect(result.status,result.stderr).toBe(0);const independent=JSON.parse(result.stdout);for(const [index,row]of corpus.floorCases.entries()){expect(independent[index]).toBe(row.expected);expect(hostFloorMegabytesV1(BUILD_HOST_CAPACITY_POLICY_V1,{totalMegabytes:row.totalMegabytes})).toBe(row.expected);}});


test("quiet zero-job grants remain valid under the current closed status contract",()=>{const ajv=new Ajv({strict:true});for(const row of corpus.grantCases){const grant={id:"fixture",operation:"run",pid:1,jobs:row.jobs,reservedMegabytes:0,heldSeconds:0},document={version:1,directory:root,atMs:0,holders:[],waiters:[],waitFor:[],cycles:[],violations:[],admission:{floorMegabytes:corpus.floorMegabytes.minimum,headroomMegabytes:0,grants:[grant],waiters:[]}};expect(ajv.validate(leaseStatusSchema.definitions.AdmissionGrant,grant)).toBe(row.valid);expect(ask(document).problems.length===0).toBe(row.valid);}});
