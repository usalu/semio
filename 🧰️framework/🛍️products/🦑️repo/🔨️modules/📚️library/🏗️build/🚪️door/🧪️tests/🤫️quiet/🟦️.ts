/** 🤫️ A `runtime` input of a task is hashed by Nx with its stderr: the commands nx.json lets Nx run (`repo tools exec --light ...`) must stay silent, or every task hash would depend on the load, the seconds waited and the language of the machine. Cross-checked with the native Nx task hasher (`nx.darwin-arm64.node` and its siblings): the same plan hashes differently when only stderr differs. */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {buildDoorStaysQuietV1} from "../../🟦️.ts";
import policy from "../../🔣️.json" with {type:"json"};

const root=resolve(import.meta.dir,"../../../../../../../../..");
const scratch=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"door-quiet-"));
afterAll(()=>rmSync(scratch,{recursive:true,force:true}));
const quiet=(...typed:string[])=>buildDoorStaysQuietV1(typed,{});

const nxRoot=join(root,".nx/installation/node_modules/nx");
const nxAvailable=existsSync(join(nxRoot,"dist/src/native/index.js"));
const nxHash=(command:string):string=>{
 const require=createRequire(import.meta.url),native=require(join(nxRoot,"dist/src/native")),{transformProjectGraphForRust}=require(join(nxRoot,"dist/src/native/transform-objects")),workspace=join(scratch,"workspace");
 mkdirSync(join(workspace,"a"),{recursive:true});writeFileSync(join(workspace,"a/x.txt"),"x\n");writeFileSync(join(workspace,"package.json"),'{"name":"w"}');writeFileSync(join(workspace,"nx.json"),"{}");
 const graph={nodes:{a:{name:"a",type:"lib",data:{root:"a",targets:{t:{executor:"nx:run-commands",options:{},inputs:[{runtime:command},"{projectRoot}/**/*"]}}}}},dependencies:{a:[]},externalNodes:{}};
 const files=new native.WorkspaceContext(workspace,join(workspace,".nxcache")).getWorkspaceFiles({a:"a"}),reference=native.transferProjectGraph(transformProjectGraphForRust(graph)),planner=new native.HashPlanner({},reference);
 const taskGraph={roots:["a:t"],tasks:{"a:t":{id:"a:t",target:{project:"a",target:"t"},outputs:[],overrides:{},parallelism:true,cache:true,continuous:false}},dependencies:{"a:t":[]},continuousDependencies:{"a:t":[]}};
 const hasher=new native.TaskHasher(workspace,reference,files.externalReferences.projectFiles,files.externalReferences.allWorkspaceFiles,Buffer.from("{}"),{},null,null);
 return hasher.hashPlans(planner.getPlansReference(["a:t"],taskGraph),{"a:t":{}},workspace,false)["a:t"].value;
};

test("only the short queries the policy names stay quiet, never a tool that runs long or a flag that belongs to the tool",()=>{
 expect(policy.quiet).toEqual([{command:"tools",subcommand:"exec",option:"--light"}]);
 expect(quiet("tools","exec","--light","node","--","-p","1")).toBe(true);
 expect(quiet("repo","tools","exec","--light","bun","--","--version")).toBe(true);
 expect(quiet("--tool=nx","tools","exec","--cwd=x","--light","node","--","-p","1")).toBe(true);
 expect(quiet("tools","exec","cargo","--","check")).toBe(false);
 expect(quiet("tools","exec","cargo","--","--light")).toBe(false);
 expect(quiet("tools","list")).toBe(false);
 expect(quiet("run","@a/b:c","--light")).toBe(false);
 expect(quiet("--tool=bazel","tools","exec","--light","node","--")).toBe(true);
 expect(quiet()).toBe(false);
});

test("every repo command that nx.json hands to Nx as a runtime input is a quiet invocation",()=>{
 const named=JSON.parse(readFileSync(join(root,"nx.json"),"utf8")).namedInputs as Record<string,unknown[]>,commands=Object.values(named).flat().filter((input):input is {runtime:string}=>typeof input==="object"&&input!==null&&typeof (input as {runtime?:unknown}).runtime==="string").map(input=>input.runtime);
 expect(commands.length).toBeGreaterThanOrEqual(2);
 for(const command of commands){
  const words=[...command.matchAll(/"[^"]*"|\S+/gu)].map(word=>word[0].replace(/^"|"$/gu,""));
  if(words[0]!=="repo")continue;
  expect(buildDoorStaysQuietV1(words,{}),command).toBe(true);
 }
});

test.skipIf(!nxAvailable||process.platform==="win32")("the native Nx hasher hashes the stderr of a runtime input, so a heartbeat line would give a task a hash of its own",()=>{
 const loud=join(scratch,"loud.sh"),silent=join(scratch,"silent.sh");
 writeFileSync(loud,'#!/bin/sh\necho darwinarm64\necho "[repo] still working after $1 s; last phase started: boot.loaded" >&2\n');writeFileSync(silent,"#!/bin/sh\necho darwinarm64\n");chmodSync(loud,0o755);chmodSync(silent,0o755);
 const first=nxHash(`${silent}`),again=nxHash(`${silent}`),five=nxHash(`${loud} 5`),ten=nxHash(`${loud} 10`);
 expect(again).toBe(first);
 expect(five).not.toBe(first);
 expect(ten).not.toBe(five);
});

test.skipIf(!nxAvailable||process.platform==="win32")("the runtime input of nx.json hashes the same in every language and at every speed",()=>{
 const command=`cd ${JSON.stringify(root)} && repo tools exec --light node -- -p "process.platform.concat(process.arch)"`,hashes=new Set<string>();
 for(const language of ["de","en",""]){
  const run=spawnSync("sh",["-c",command],{encoding:"utf8",timeout:120_000,env:{...process.env,SEMIO_LANGUAGE:language}});
  expect(run.status,run.stderr).toBe(0);
  expect(run.stderr).toBe("");
  hashes.add(nxHash(command));
 }
 expect(hashes.size).toBe(1);
},300_000);
