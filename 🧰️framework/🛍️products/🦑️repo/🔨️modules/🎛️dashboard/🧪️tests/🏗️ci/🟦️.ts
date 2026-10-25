import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import {existsSync,readFileSync} from "node:fs";
import {dirname,join,relative,sep} from "node:path";
import {parse} from "yaml";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import cases from "./🔣️.json" with {type:"json"};
import coverageSchema from "../../🧬️schema/🪟️windows/🎟️coverage/🔣️.json" with {type:"json"};
import coverage from "../../🧫️fixtures/🪟️windows/🎟️coverage/🔣️.json" with {type:"json"};

const root=()=>{let path=import.meta.dir;for(let depth=0;depth<32;depth++){if(existsSync(join(path,".github/workflows/build-system.yml")))return path;const parent=dirname(path);if(parent===path)break;path=parent;}throw Error("Repository CI root required");};
const document=(path:string)=>parse(readFileSync(join(root(),path),"utf8"));

const assertGateOrder=(actual:string[])=>{
 expect(actual).toEqual(cases.gates.map(row=>row.id));
 const db=new Database(":memory:");try{
  expect(db.query("SELECT (SELECT key FROM json_each(?1) WHERE value='canonical-cold-lazy-launch') < (SELECT key FROM json_each(?1) WHERE value=?2) AS coldFirst").get(JSON.stringify(actual),cases.windowGate.id)).toEqual({coldFirst:1});
  expect(db.query("SELECT group_concat(value,'>') AS ordered FROM json_each(?)").get(JSON.stringify(actual))).toEqual({ordered:cases.gates.map(row=>row.id).join(">")});
 }finally{db.close();}
};

test("dashboard CI host and gate vectors are closed and independently ordered",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);
 expect(validate({...cases,unknown:true})).toBe(false);
 expect(validate({...cases,windowGate:{...cases.windowGate,command:"repo cargo test"}})).toBe(false);
 expect(new Ajv({strict:true}).compile(coverageSchema)(coverage)).toBe(true);
 expect(cases.gates.find(row=>row.id===cases.windowGate.id)).toEqual(cases.windowGate);
 const db=new Database(":memory:");try{
  const rows=db.query("SELECT json_extract(value,'$.name') AS name,json_extract(value,'$.runner') AS runner FROM json_each(?) ORDER BY key").all(JSON.stringify(cases.hosts));
  expect(rows).toEqual(cases.hosts);
  expect(db.query("SELECT count(DISTINCT json_extract(value,'$.name')) AS count FROM json_each(?)").get(JSON.stringify(cases.hosts))).toEqual({count:3});
  expect(db.query("SELECT count(DISTINCT json_extract(value,'$.id')) AS count FROM json_each(?)").get(JSON.stringify(cases.gates))).toEqual({count:7});
  const native=coverage.native.flatMap(owner=>owner.required.map(name=>owner.package+"::"+name));
  expect(db.query("SELECT count(DISTINCT value) AS count FROM json_each(?)").get(JSON.stringify(native))).toEqual({count:17});
  expect(db.query("SELECT count(DISTINCT value) AS count FROM json_each(?)").get(JSON.stringify(coverage.settlementCases))).toEqual({count:8});
 }finally{db.close();}
});

test("every native host runs actual dashboard editing language cold install and journey gates",()=>{
 const workflow=document(".github/workflows/dashboard.yml");
 expect(workflow.permissions).toEqual({contents:"read"});
 expect(workflow.on).toHaveProperty("pull_request");expect(workflow.on).toHaveProperty("push");expect(workflow.on).toHaveProperty("workflow_dispatch");
 expect(workflow.env.SEMIO_LANGUAGE).toBe("en");expect(workflow.env.SEMIO_BUILD_MODE).toBe("prototype");
 const job=workflow.jobs.native;expect(job.strategy['fail-fast']).toBe(false);expect(job.strategy.matrix.include).toEqual(cases.hosts);
 expect(job['runs-on']).toBe('$'+'{{ matrix.runner }}');expect(job['timeout-minutes']).toBeGreaterThan(0);
 const steps=job.steps as {id?:string;run?:string;uses?:string;env?:Record<string,string>}[];
 const bootstrap=steps.findIndex(step=>step.uses==="./.github/actions/native-repo");expect(bootstrap).toBeGreaterThan(0);
 for(const row of cases.gates){const position=steps.findIndex(step=>step.id===row.id);expect(position,row.id).toBeGreaterThan(bootstrap);expect(steps[position]!.run?.trim(),row.id).toBe(row.command);expect(steps[position]!.env?.SEMIO_TEST_ARTIFACT_DIR).toBe("${{ runner.temp }}/dashboard-artifacts");expect(steps[position]!.env?.SEMIO_BUILD_TEST_RESULTS_DIR).toBe("${{ runner.temp }}/dashboard-results");}
 assertGateOrder(steps.filter(step=>cases.gates.some(row=>row.id===step.id)).map(step=>step.id!));
 expect(steps.findIndex(step=>step.id==="canonical-cold-lazy-launch")).toBeLessThan(steps.findIndex(step=>step.id==="installed-journeys"));
 const evidence=steps.find(step=>step.uses?.startsWith("actions/upload-artifact@")) as any;expect(evidence.if).toBe("${{ !cancelled() }}");expect(evidence.with["retention-days"]).toBe(14);expect(evidence.with.path).toContain("dashboard-artifacts");expect(evidence.with.path).toContain("dashboard-results");
 expect(steps.filter(step=>step.env?.SEMIO_LANGUAGE==="de").map(step=>step.run)).toEqual(["repo --help"]);
 for(const step of steps){if(step.run)for(const line of step.run.trim().split("\n"))expect(/^repo (?!dashboard(?: |$))/u.test(line)).toBe(true);if(step.uses)expect(step.uses.startsWith("./.github/actions/")||/^[\w.-]+\/[\w./-]+@[0-9a-f]{40}$/u.test(step.uses)).toBe(true);}
});

test("native CI recovery publishes the owned frontend before ordinary repository commands",()=>{
 const action=document(".github/actions/native-repo/action.yml");expect(action.runs.using).toBe("composite");
 const steps=action.runs.steps as {uses?:string;run?:string;shell?:string;if?:string}[];
 expect(steps[0]!.uses).toBe("./.github/actions/repo-launcher");
 const recoveries=steps.filter(step=>step.run?.includes("repo-mcp:build"));expect(recoveries).toHaveLength(2);
 for(const step of recoveries){expect(step.run).toContain("repo run repo-mcp:build --skip-cache");expect(step.run).toContain("--help");expect(step.run).toContain("GITHUB_PATH");expect(step.run).not.toMatch(/repo dashboard|cargo |rustc |git (?:reset|checkout|clean)/u);expect(step.if).toContain("runner.os");}
 expect(recoveries.map(step=>step.shell)).toEqual(["bash","pwsh"]);
});

test("devcontainer CI builds the actual configuration and refuses an unexecuted adapter result",()=>{
 const workflow=document(".github/workflows/dashboard.yml"),job=workflow.jobs.devcontainer;
 expect(job['runs-on']).toBe(cases.devcontainer.runner);expect(job['timeout-minutes']).toBeGreaterThan(0);
 expect(job.steps.some((step:any)=>step.uses==="./.github/actions/devcontainer-repo")).toBe(true);
 expect(job.steps.some((step:any)=>step.uses?.startsWith("actions/upload-artifact@")&&step.with.path.includes("dashboard-devcontainer.log"))).toBe(true);
 const action=document(".github/actions/devcontainer-repo/action.yml");expect(action.runs.using).toBe("composite");
 const execution=action.runs.steps.find((step:any)=>step.uses===cases.devcontainer.adapter);expect(execution.with.configFile).toBe(cases.devcontainer.configuration);expect(execution.with.push).toBe("never");expect(execution.with.inheritEnv).toBe(false);
 const commands=execution.with.runCmd.trim().split("\n");expect(commands[0]).toBe("set -euo pipefail");expect(commands.at(-1)).toBe("tail -c 16384 /tmp/dashboard-help.log");expect(commands).toContain("run_gate help repo --help");
 for(const id of cases.devcontainer.gates)expect(commands).toContain("run_gate "+id+" "+cases.gates.find(row=>row.id===id)!.command);
 assertGateOrder(commands.filter((line:string)=>line.startsWith("run_gate ")).map((line:string)=>line.split(" ")[1]!).filter((id:string)=>cases.gates.some(row=>row.id===id)));
 expect(execution.with.runCmd).toContain('tail -c 16384 "/tmp/dashboard-$gate.log"');expect(execution.with.runCmd).toContain('exit "$status"');
 expect(execution.with.env).toContain("SEMIO_LANGUAGE=en");expect(execution.with.env).toContain("SEMIO_TEST_ARTIFACT_DIR=/tmp/dashboard-artifacts");
 const observed=action.runs.steps.find((step:any)=>step.id==="observed");expect(observed.if).toContain("!cancelled()");expect(observed.env.ACTUAL_OUTPUT).toContain("runCmdOutput");expect(observed.run).toContain(cases.devcontainer.completedEntry);expect(observed.run).toContain("exit 1");expect(observed.run).toContain("dashboard-devcontainer.log");
});


test("dashboard CI Source gate is closed and reaches the owned early test branch",()=>{
 const gate=cases.sourceGate,project=JSON.parse(readFileSync(join(root(),gate.owner,"📋️project.json"),"utf8")),target=project.targets[gate.target],script=readFileSync(join(root(),gate.owner,"../../🛤️task-routes/🟦️.ts"),"utf8").split("class TestScript")[1]!;
 expect(project.name).toBe(gate.project);expect(target.executor).toBe("repo:script");expect(target.cache).toBe(true);expect(target.dependsOn).toEqual([]);expect(target.outputs).toEqual([]);expect(target.inputs).toEqual(gate.inputs);expect(target.options).toEqual({cwd:gate.owner,command:gate.command,forwardAllArgs:false});expect(target.metadata.semio.execution).toEqual({version:1,kind:"source"});expect(target.metadata.semio.sourceInputObservation).toBe("freshness");expect(target.metadata.semio.runtimeSourceEntries).toEqual([gate.entry]);
 const entry=join(root(),gate.entry.slice("{workspaceRoot}/".length)),literal=relative(join(root(),gate.owner),entry).split(sep).join("/");expect(existsSync(entry)).toBe(true);expect(script).toContain(JSON.stringify(literal));expect(script).toContain('if(segments.length!==1)throw Error("Expected test ci-source");');
 const db=new Database(":memory:");try{expect(db.query("SELECT instr(?1,?2)>0 AND instr(?1,?2)<instr(?1,'await dashboardExecutable(') AS early,(SELECT count(DISTINCT value) FROM json_each(?3)) AS inputs").get(script,'if(segments[0]==="'+gate.selector+'")',JSON.stringify(target.inputs))).toEqual({early:1,inputs:9});}finally{db.close();}
});
