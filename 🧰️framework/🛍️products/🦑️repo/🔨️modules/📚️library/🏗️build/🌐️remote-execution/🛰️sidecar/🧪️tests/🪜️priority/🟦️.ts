/** 🪜️ Contract: every sidecar role and provisioned service has a declared scheduling class (latency-critical ones are never throttled, batch ones stay in the background), the declaration agrees with an independent JSON-schema implementation, the renderers produce the keys each platform documents, and a role applies its class to its own process once, with injected platform facts and against a real child process. */
import {afterAll,expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {setPriority} from "node:os";
import {fileURLToPath} from "node:url";
import {applyServicePriorityV1,applyServiceUnitPriorityV1,launchdPriorityKeysV1,servicePriorityOfRoleV1,servicePriorityOfServiceV1,servicePriorityTextV1,servicePriorityV1,shellPriorityLinesV1,taskPriorityXmlV1,realServicePriorityPortsV1,type ServicePriorityPortsV1} from "../../🪜️priority/🟦️.ts";

const schemaPath=fileURLToPath(new URL("../../🪜️priority/🧬️schema/🔣️.json",import.meta.url)),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const ROLES=["agent","scheduler","cache","graph","pool","ready","spool"],SERVICES=["replica","cache","relay","compact"];
const children:{kill():void}[]=[];
afterAll(()=>{for(const child of children)child.kill();});

const ports=(platform:string,current:number,facts:{set?:(value:number)=>void;ionice?:(args:readonly string[])=>number|null}={}):ServicePriorityPortsV1&{calls:string[]}=>{
 const calls:string[]=[];
 return {calls,platform,pid:4242,current:()=>current,set:value=>{calls.push(`set ${value}`);facts.set?.(value);},ionice:args=>{calls.push(`ionice ${args.join(" ")}`);return facts.ionice===undefined?0:facts.ionice(args);}};
};

test("every sidecar role and every service declares a class, and the latency-critical ones are interactive while the batch ones are background",()=>{
 expect(Object.keys(servicePriorityV1.roles).sort()).toEqual([...ROLES].sort());
 expect(Object.keys(servicePriorityV1.services).sort()).toEqual([...SERVICES].sort());
 for(const role of ["cache","scheduler","graph"])expect(servicePriorityOfRoleV1(role)).toBe("interactive");
 for(const role of ["pool","ready"])expect(servicePriorityOfRoleV1(role)).toBe("background");
 expect(servicePriorityOfServiceV1("replica")).toBe("interactive");
 expect(servicePriorityOfServiceV1("compact")).toBe("background");
 expect(()=>servicePriorityOfRoleV1("unknown")).toThrow("Undeclared");
 expect(()=>servicePriorityOfServiceV1("unknown")).toThrow("Undeclared");
});

test("the host of an interactive role is itself interactive, because a process never raises itself above its parent",()=>{
 const rank={interactive:0,standard:1,background:2};
 const best=Math.min(...Object.values(servicePriorityV1.roles).map(name=>rank[name]));
 expect(rank[servicePriorityOfServiceV1("replica")]).toBeLessThanOrEqual(best);
});

test("the classes order the platform facts the same way: nice, launchd process type, systemd policy, Windows task priority",()=>{
 const {interactive,standard,background}=servicePriorityV1.classes;
 expect(interactive.nice).toBeLessThan(standard.nice);expect(standard.nice).toBeLessThan(background.nice);
 expect(interactive.windows.taskPriority).toBeLessThan(standard.windows.taskPriority);expect(standard.windows.taskPriority).toBeLessThan(background.windows.taskPriority);
 expect(interactive.darwin.processType).toBe("Interactive");expect(background.darwin.processType).toBe("Background");expect(background.darwin.lowPriorityIo).toBe(true);
 expect(background.linux.cpuSchedulingPolicy).toBe("idle");expect(background.linux.ioSchedulingClass).toBe("idle");
 expect(interactive.windows.taskPriority).toBeLessThan(7);
});

test("the declaration agrees with an independent JSON-schema implementation (python jsonschema)",()=>{
 const run=spawnSync("python3",[oracle,schemaPath],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const result=JSON.parse(run.stdout) as {problems:string[];roles:Record<string,string>;services:Record<string,string>};
 expect(result.problems).toEqual([]);
 expect(result.roles).toEqual(servicePriorityV1.roles);expect(result.services).toEqual(servicePriorityV1.services);
});

test("the renderers produce the documented keys of each platform",()=>{
 expect(launchdPriorityKeysV1("interactive")).toEqual(["<key>ProcessType</key><string>Interactive</string>"]);
 expect(launchdPriorityKeysV1("standard")).toEqual(["<key>ProcessType</key><string>Standard</string>"]);
 expect(launchdPriorityKeysV1("background")).toEqual(["<key>ProcessType</key><string>Background</string>","<key>LowPriorityIO</key><true/>","<key>Nice</key><integer>10</integer>"]);
 expect(shellPriorityLinesV1("interactive")).toEqual(["renice -n -5 -p $$ >/dev/null 2>&1 || :","ionice -c 2 -n 2 -p $$ >/dev/null 2>&1 || :"]);
 expect(shellPriorityLinesV1("background")).toEqual(["renice -n 10 -p $$ >/dev/null 2>&1 || :","chrt --idle -p 0 $$ >/dev/null 2>&1 || :","ionice -c 3 -p $$ >/dev/null 2>&1 || :"]);
 for(const name of Object.keys(servicePriorityV1.classes) as (keyof typeof servicePriorityV1.classes)[])expect(spawnSync("sh",["-n"],{input:shellPriorityLinesV1(name).join("\n")}).status).toBe(0);
 expect(taskPriorityXmlV1("interactive")).toBe("<Priority>3</Priority>");
 expect(taskPriorityXmlV1("background")).toBe("<Priority>8</Priority>");
});

test("linux: a background role lowers its nice value and its input/output class, an interactive role under a unit at its class changes nothing",()=>{
 const ready=ports("linux",0),result=applyServicePriorityV1("ready",ready);
 expect(ready.calls).toEqual(["set 10","ionice -c 3 -p 4242"]);
 expect(result).toMatchObject({name:"background",nice:10,outcome:"applied",io:"idle"});
 const cache=ports("linux",-5),kept=applyServicePriorityV1("cache",cache);
 expect(cache.calls).toEqual(["ionice -c 2 -n 2 -p 4242"]);
 expect(kept).toMatchObject({name:"interactive",nice:-5,outcome:"applied",io:"best-effort"});
 const agent=ports("linux",-5);
 expect(applyServicePriorityV1("agent",agent)).toMatchObject({name:"standard",nice:0,outcome:"applied"});
 expect(agent.calls[0]).toBe("set 0");
});

test("linux: an interactive role that may not raise itself reports the refusal and the process runs on at the priority it has; a missing ionice is no error",()=>{
 const denied=ports("linux",0,{set:()=>{throw Error("EACCES: permission denied");},ionice:()=>null}),result=applyServicePriorityV1("cache",denied);
 expect(result).toMatchObject({name:"interactive",nice:0,outcome:"refused",io:undefined});
 expect(result.reason).toContain("EACCES");
 expect(servicePriorityTextV1(result)).toContain("not applied (EACCES");
});

test("macos: the nice value only lowers; raising is left to the ProcessType of the LaunchAgent and reported as inherited; no ionice",()=>{
 const cache=ports("darwin",0),inherited=applyServicePriorityV1("cache",cache);
 expect(cache.calls).toEqual([]);expect(inherited).toMatchObject({name:"interactive",nice:0,outcome:"inherited",io:undefined});
 const pool=ports("darwin",0);
 expect(applyServicePriorityV1("pool",pool)).toMatchObject({nice:10,outcome:"applied"});expect(pool.calls).toEqual(["set 10"]);
});

test("windows: the nice value goes through the same call (libuv maps it to the priority class); a scheduler tries to rise above normal",()=>{
 const scheduler=ports("win32",0);
 expect(applyServicePriorityV1("scheduler",scheduler)).toMatchObject({name:"interactive",nice:-5,outcome:"applied",io:undefined});
 expect(scheduler.calls).toEqual(["set -5"]);
});

test("a provisioned service applies its own class: the replica started below normal by a stale Windows task rises to interactive, macOS leaves it to launchd, an undeclared service is refused",()=>{
 const windows=ports("win32",10);
 expect(applyServiceUnitPriorityV1("replica",windows)).toMatchObject({role:"replica",name:"interactive",nice:-5,outcome:"applied",io:undefined});
 expect(windows.calls).toEqual(["set -5"]);
 const darwin=ports("darwin",0);
 expect(applyServiceUnitPriorityV1("replica",darwin)).toMatchObject({name:"interactive",nice:0,outcome:"inherited"});expect(darwin.calls).toEqual([]);
 const compact=ports("linux",0);
 expect(applyServiceUnitPriorityV1("compact",compact)).toMatchObject({name:"background",nice:10,outcome:"applied",io:"idle"});
 expect(()=>applyServiceUnitPriorityV1("unknown",ports("win32",0))).toThrow("Undeclared");
});

test.skipIf(process.platform!=="win32")("windows: a real child process started below normal ends up in the priority class Get-Process reports for the replica service",async()=>{
 const child=spawn(process.execPath,["-e","setTimeout(()=>{},30000)"],{stdio:"ignore",windowsHide:true});children.push(child);
 await new Promise(done=>setTimeout(done,300));
 setPriority(child.pid!,10);
 const result=applyServiceUnitPriorityV1("replica",realServicePriorityPortsV1(child.pid!));
 expect(result).toMatchObject({name:"interactive",outcome:"applied"});
 const ps=spawnSync("powershell.exe",["-NoProfile","-Command",`(Get-Process -Id ${child.pid}).PriorityClass`],{encoding:"utf8"});
 expect(ps.stdout.trim()).toBe("AboveNormal");
});

test("the log line names the role, the class, the nice value and the outcome in English and German",()=>{
 const text=servicePriorityTextV1(applyServicePriorityV1("ready",ports("linux",0)));
 expect(text).toBe("ready scheduling: class background, nice 10, applied, input/output idle / ready Scheduling: Klasse background, Nice 10, angewendet, Ein-/Ausgabe idle");
 expect(servicePriorityTextV1(applyServicePriorityV1("cache",ports("darwin",0)))).toContain("inherited from the service that hosts the role / cache Scheduling");
});

test.skipIf(process.platform==="win32")("a real child process ends up with the nice value ps reports for its role",async()=>{
 const child=spawn("sleep",["30"],{stdio:"ignore"});children.push(child);
 await new Promise(done=>setTimeout(done,100));
 const result=applyServicePriorityV1("pool",realServicePriorityPortsV1(child.pid!));
 expect(result).toMatchObject({name:"background",nice:10,outcome:"applied"});
 const ps=spawnSync("ps",["-o","nice=","-p",String(child.pid)],{encoding:"utf8"});
 expect(Number(ps.stdout.trim())).toBe(10);
});
