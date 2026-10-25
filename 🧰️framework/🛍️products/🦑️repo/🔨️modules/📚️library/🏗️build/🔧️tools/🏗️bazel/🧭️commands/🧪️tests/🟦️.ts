/** 🧪️ Bazel command mapping: the fixture is the contract, a Python implementation and Bazel's own binary are the oracles. */
import {test,expect} from "bun:test";
import {makeTestTreeWritableV1,testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {bazelCommandPlanV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const ajv=new Ajv({strict:true,allowUnionTypes:true}),compile=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`}),input=compile("Input"),plan=compile("Plan");

test("the fixture and the schema agree with Ajv, and every refused input is refused by both validators",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 for(const row of fixture.cases){expect(input(row.input),row.name).toBe(true);expect(plan(row.expected),row.name).toBe(true);}
 for(const row of fixture.refused){expect(input(row.input),row.name).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.Input,row.input,schema).length>0,row.name).toBe(true);expect(()=>bazelCommandPlanV1(row.input as never),row.name).toThrow("Closed Bazel command input required");}
 expect(schema.definitions.Operations.const.map(row=>row.operation)).toEqual(["run","run-many","affected","exec","watch","graph","show","cache-status","cache-clear"]);
 expect(Object.keys(schema.definitions.Policy.const)).toEqual(["version","testOutput"]);
});

test("every fixture case maps to exactly its authored Bazel invocation",()=>{
 for(const row of fixture.cases)expect(bazelCommandPlanV1(row.input as never),row.name).toEqual(row.expected as never);
});

const python=`
import json,sys
def plan(i):
    batch=[] if i["mode"]=="plan" else [t for t in i["tasks"] if t["kind"]!="launch"]
    if not batch: return {"version":1,"steps":[]}
    tests=sum(1 for t in batch if t["kind"]=="test")
    o=["--jobs=%d"%i["jobs"]]
    if not i["bail"]: o.append("--keep_going")
    if i["verbose"]: o.append("--subcommands")
    if i.get("events"): o.append("--build_event_json_file="+i["events"])
    if tests:
        if tests==len(batch): o.append("--build_tests_only")
        o.append("--test_output="+("all" if i["verbose"] else "errors"))
        o.append("--nocache_test_results" if i["skipCache"] else "--cache_test_results=auto")
        if i["bail"]: o.append("--notest_keep_going")
    return {"version":1,"steps":[{"verb":"test" if tests else "build","targets":[t["label"] for t in batch],"options":i["base"]+o}]}
print(json.dumps([plan(c["input"]) for c in json.load(sys.stdin)]))
`;

test("an independent Python implementation of the mapping agrees on every case",()=>{
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",python],{input:JSON.stringify(fixture.cases),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout) as unknown[];
 expect(answers.length).toBe(fixture.cases.length);
 for(const[index,row]of fixture.cases.entries())expect(answers[index],row.name).toEqual(bazelCommandPlanV1(row.input as never) as never);
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary?test:test.skip;
real("Bazel 8 accepts every mapped invocation, caches test results for tests and rebuilds them under skipCache",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-cmd-"))),output=join(directory,"ob"),startup=[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=15"];
 try{
  writeFileSync(join(directory,"MODULE.bazel"),'module(name = "commands", version = "0.0.0")\n');
  writeFileSync(join(directory,"rules.bzl"),`def _task(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".out")
    ctx.actions.write(out, "ok")
    return [DefaultInfo(files = depset([out]))]
repo_task = rule(implementation = _task)
def _test(ctx):
    launcher = ctx.actions.declare_file(ctx.label.name + ".sh")
    ctx.actions.write(launcher, "#!/bin/sh\\necho ran-" + ctx.label.name + "\\n", is_executable = True)
    return [DefaultInfo(executable = launcher)]
repo_test = rule(implementation = _test, test = True)
`);
  const labels=new Map<string,string>();
  for(const row of fixture.cases)for(const task of row.input.tasks)labels.set(task.label,task.kind);
  writeFileSync(join(directory,"BUILD.bazel"),'load(":rules.bzl", "repo_task", "repo_test")\n'+[...labels].map(([label,kind])=>`${kind==="test"?"repo_test":"repo_task"}(name = "${label.slice(3)}")\n`).join(""));
  const bazel=(verb:string,options:string[],targets:string[])=>spawnSync(binary!,[...startup,verb,"--lockfile_mode=off","--incompatible_autoload_externally=",...options,...targets],{cwd:directory,encoding:"utf8",env:{...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}});
  const disk=join(directory,"disk");mkdirSync(disk);
  for(const row of fixture.cases){
   const{events,...rest}=row.input as {events?:string},mapped=bazelCommandPlanV1({...rest,mode:"execute",base:[`--disk_cache=${disk}`],...events?{events:join(directory,"events.json")}:{}} as never);
   for(const step of mapped.steps){
    const options=step.options.filter(flag=>flag!=="--subcommands"&&!flag.startsWith("--lockfile"));
    const first=bazel(step.verb,options,[...step.targets]);
    expect(first.status,row.name+"\n"+first.stderr).toBe(0);
    if(step.verb==="test"){
     const second=bazel(step.verb,options,[...step.targets]),skipped=step.options.includes("--nocache_test_results");
     expect(second.status,row.name+"\n"+second.stderr).toBe(0);
     expect(/\(cached\) PASSED/u.test(second.stdout+second.stderr),row.name).toBe(!skipped);
     expect(/PASSED/u.test(second.stdout+second.stderr),row.name).toBe(true);
    }
   }
  }
 }finally{
  spawnSync(binary!,[...startup,"shutdown"],{cwd:directory});
  makeTestTreeWritableV1(directory);
  rmSync(directory,{recursive:true,force:true});
 }
},300000);
