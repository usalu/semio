/** 🔒️ Law: with the real Bazel and the production Starlark, runner and flags, service actions run in one persistent worker (the spawn log says `worker`), a second action of the same Bazel server reaches the first one's service state, an edited input is seen by the next request without staging the rest again, and the same actions run as ordinary sandboxed processes through a flagfile when workers are off.
 * @see ../🟦️.ts — the runner/service contract without Bazel */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import buildPolicy from "../../../../../../🔣️policy.json";
import {planWorkersV1,readWorkerPolicyV1} from "../../../../../../👷️workers/🟦️.ts";
import * as api from "../../../🟦️.ts";
import {bazelWorkerSettingsV1} from "../../../../👷️workers/🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,host=api.bazelExecutionPlatformV1(process.platform,process.arch);
type Spawn={mnemonic:string;runner:string;targetLabel:string};
const readLog=(path:string):Spawn[]=>{const text=readFileSync(path,"utf8"),rows:Spawn[]=[];let depth=0,start=-1,quoted=false,escaped=false;for(let index=0;index<text.length;index++){const char=text[index]!;if(quoted){if(escaped)escaped=false;else if(char==="\\")escaped=true;else if(char==='"')quoted=false;continue;}if(char==='"')quoted=true;else if(char==="{"){if(depth===0)start=index;depth++;}else if(char==="}"&&--depth===0)rows.push(JSON.parse(text.slice(start,index+1)));}return rows;};

const alias="a/0123456789",service=`let served = 0;
export function createHermeticServiceV1() {
  return { async handle() { served++; return { served }; }, release() {} };
}
`;
const client=`import { connect } from "node:net";
import { readFileSync } from "node:fs";
const [host, port, token] = (process.env.SEMIO_TEST_SERVICE ?? "").split(":");
const root = process.env.SEMIO_WORKSPACE_ROOT;
const content = readFileSync(root + "/lib/a.txt", "utf8").trim();
const expected = JSON.parse(process.env.EXPECT);
if (expected.served === 0) { if (port) { console.error("a service address in an ordinary process"); process.exit(4); } process.exit(0); }
if (!port) { console.error("no service address: the action did not run in a worker"); process.exit(4); }
const reply = JSON.parse(await new Promise(resolve => {
  const socket = connect(Number(port), host);
  let text = "";
  socket.setEncoding("utf8");
  socket.on("data", chunk => text += chunk);
  socket.on("end", () => resolve(text.trim()));
  socket.write(JSON.stringify({ token, request: { cwd: root + "/lib", project: "p" } }) + "\\n");
}));
if (!reply.ok || reply.result.served !== expected.served || content !== expected.content) { console.error("unexpected " + JSON.stringify({ reply, content, expected })); process.exit(3); }
`;

test.skipIf(!binary)("law: service actions run in one persistent worker, share its state across actions and Bazel invocations, see edited inputs, and run as ordinary processes without workers",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"service-law-"));
 try{
  const workspace=join(directory,"ws"),write=(path:string,text:string)=>{mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text);};
  const scratch=Object.fromEntries(["darwin","linux","win32"].map(platform=>[platform,{base:join(directory,"w"),workDigits:8,workspace:"r",separator:"/"}]));
  write("MODULE.bazel",'module(name = "fx", version = "0")\n');
  write("tasks.bzl",api.BAZEL_HERMETIC_STARLARK);
  write("hermetic-runner.mjs",api.BAZEL_HERMETIC_RUNNER);
  write(alias+"/svc.ts",service);write(alias+"/client.ts",client);write(alias+"/a.txt","one\n");
  const manifest=(expect:{served:number;content:string})=>({version:1,id:"fixture:check",project:"fixture",target:"check",root:"lib",args:[],commands:["bun {projectRoot}/client.ts"],outputs:[],inputs:["lib/svc.ts","lib/client.ts","lib/a.txt"],aliases:{lib:alias},scratch,env:{EXPECT:JSON.stringify(expect)},service:{module:"lib/svc.ts",environment:"SEMIO_TEST_SERVICE",idleMilliseconds:0}});
  const rewrite=(expectA:{served:number;content:string},expectB:{served:number;content:string})=>{write("t_a.json",JSON.stringify(manifest(expectA)));write("t_b.json",JSON.stringify(manifest(expectB)));};
  rewrite({served:1,content:"one"},{served:2,content:"one"});
  const task=(name:string,deps:string)=>`repo_hermetic_task(\n    name = "${name}",\n    manifest = "${name}.json",\n    srcs = [":source"],\n    deps = [${deps}],\n    runner = "hermetic-runner.mjs",\n    identity = "${name}",\n    worker = True,\n    mnemonic = "${api.bazelServiceFlagsV1({host:{platform:process.platform},workers:true,remote:false})[0]!.split("=")[1]}",\n)\n`;
  write("BUILD.bazel",`load(":tasks.bzl", "repo_bun_toolchain", "repo_hermetic_task")\n\n${api.renderBazelExecutionBuildV1({host:host.id,tools:{[host.id]:process.execPath}})}\n\nfilegroup(name = "source", srcs = glob(["a/**"]))\n\n${task("t_a","")}\n${task("t_b",'":t_a"')}`);
  await api.linkBazelToolchainsV1(workspace,{[host.id]:process.execPath});
  const plan=planWorkersV1(readWorkerPolicyV1(buildPolicy.workers),{cores:4,memoryBytes:16*1024**3}),settings=bazelWorkerSettingsV1(plan);
  const common=[`--output_base=${join(directory,"base")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=300"];
  const environment={...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};
  const flags=(workers:boolean)=>[...api.bazelExecutionFlagsV1({host:{platform:process.platform,arch:process.arch},hermetic:true,jobs:2}).command,...(workers?settings.flags:[]),...api.bazelServiceFlagsV1({host:{platform:process.platform},workers,remote:false})];
  const bazel=(args:string[])=>spawnSync(binary!,[...common,...args],{cwd:workspace,encoding:"utf8",timeout:420000,env:environment});
  const build=(workers:boolean,log:string,targets=["//:t_b"])=>bazel(["build","--lockfile_mode=off","--incompatible_strict_action_env","--incompatible_autoload_externally=",`--execution_log_json_file=${log}`,...flags(workers),...targets]);

  const logOne=join(directory,"one.json"),first=build(true,logOne);expect(first.status,first.stderr+first.stdout).toBe(0);
  const spawns=readLog(logOne).filter(row=>row.mnemonic==="RepoHermeticService");
  expect(spawns.map(row=>row.targetLabel).sort()).toEqual(["//:t_a","//:t_b"]);expect(spawns.every(row=>row.runner==="worker"),JSON.stringify(spawns)).toBe(true);
  write(alias+"/a.txt","two plus\n");rewrite({served:3,content:"two plus"},{served:4,content:"two plus"});
  const second=build(true,join(directory,"two.json"));expect(second.status,"the next invocation reaches the same worker and sees the edited input\n"+second.stderr+second.stdout).toBe(0);
  rewrite({served:3,content:"two plus"},{served:9,content:"two plus"});
  const wrong=build(true,join(directory,"three.json"));expect(wrong.status,"a mismatching expectation fails the action").not.toBe(0);
  bazel(["shutdown"]);

  rewrite({served:0,content:"two plus"},{served:0,content:"two plus"});write(alias+"/a.txt","three\n");
  const logCold=join(directory,"four.json"),cold=build(false,logCold);
  expect(cold.status,"without workers the actions run as ordinary processes: no service address\n"+cold.stderr+cold.stdout).toBe(0);
  expect(readLog(logCold).filter(row=>row.mnemonic==="RepoHermeticService").every(row=>/sandbox|local|processwrapper/u.test(row.runner)),"ordinary spawns").toBe(true);
 }finally{
  spawnSync(binary!,[`--output_base=${join(directory,"base")}`,`--install_base=${join(directory,"install")}`,"--nosystem_rc","--nohome_rc","shutdown"],{timeout:120000});
  spawnSync("chmod",["-R","u+w",directory]);rmSync(directory,{recursive:true,force:true});
 }
},1800000);
