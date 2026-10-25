/** 🔒️ Contract: `repo leases status` prints text or one JSON document for the repository lease directory, refuses unknown options and operations with the authored messages, and the first text line equals the one an independent Python summary rebuilds from the JSON document; the admission section keeps at least the authored floor and is ajv-valid. */
import {afterAll,beforeAll,expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
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
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"leases-cli-")));mkdirSync(join(root,...REPOSITORY_LEASE_DIRECTORY_V1.split("/")),{recursive:true});});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});
const run=async(args:string[])=>{const lines:string[]=[];try{return{code:await runLeasesCliV1(root,args,{},line=>lines.push(line)),lines};}catch(error){return{error:(error as Error).message,lines};}};

test("the corpus is closed by its schema and the lease directory is the one every repo lease uses",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,commands:[{args:["x"],outcome:"throw"}]})).toBe(false);
 expect(REPOSITORY_LEASE_DIRECTORY_V1).toBe(corpus.leaseDirectory);
});

test("every command yields text, one JSON document or the authored refusal, and the text summary equals the Python summary of the JSON document",async()=>{
 const document=(await run(["status","--json"])).lines.join("\n"),parsed=JSON.parse(document),expected=ask(parsed);
 expect(expected.problems).toEqual([]);expect(parsed.directory).toBe(join(root,...corpus.leaseDirectory.split("/")));
 for(const row of corpus.commands){
  const result=await run(row.args),label=row.args.join(" ");
  if(row.outcome==="throw"){expect(result.error,label).toBe((row as {message:string}).message);continue;}
  expect(result.error,label).toBeUndefined();expect(result.code,label).toBe(0);
  if(row.outcome==="json"){const value=JSON.parse(result.lines.join("\n"));expect(ask(value).problems,label).toEqual([]);expect(value.holders,label).toEqual([]);}
  else{expect(result.lines[0],label).toBe(expected.summary);expect(result.lines.length,label).toBeGreaterThan(1);}
 }
});

test("the admission section holds the authored minimum floor, is closed and is omitted instead of failing when it cannot be read",async()=>{
 const admission=await readLeaseAdmissionV1({},"en",Date.now());
 if(admission!==undefined){
  const ajv=new Ajv({strict:false});
  expect(ajv.validate({type:"object",additionalProperties:false,required:["floorMegabytes","headroomMegabytes","grants","waiters"],properties:{floorMegabytes:{type:"integer",minimum:corpus.floorMegabytes.minimum},headroomMegabytes:{type:"integer"},grants:{type:"array",items:{type:"object",required:["id","operation","pid","jobs","reservedMegabytes","heldSeconds"],properties:{heldSeconds:{minimum:0}}}},waiters:{type:"array",items:{type:"object",required:["id","operation","pid","needMegabytes","waitedSeconds","reason","reasonText"],properties:{waitedSeconds:{minimum:0}}}}}},JSON.parse(JSON.stringify(admission))),JSON.stringify(ajv.errors)).toBe(true);
  expect(Object.isFrozen(admission)).toBe(true);
 }
 const unreadable=await readLeaseAdmissionV1({PATH:""} as never,"de",0);expect(unreadable===undefined||typeof unreadable.floorMegabytes==="number").toBe(true);
});
