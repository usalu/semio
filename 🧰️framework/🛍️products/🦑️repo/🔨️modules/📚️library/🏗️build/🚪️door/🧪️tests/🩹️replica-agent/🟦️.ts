/** 🩹️ Every `repo` command heals a replica agent that launchd loaded but does not run: macOS only, only where the agent is installed, at most once a minute, a demand start with one bilingual line; an agent that runs, an unloaded one and a refused start are handled; a fake `launchctl` proves the calls, a real throwaway job proves that kickstart starts a loaded job that is not running. */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync,statSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {healReplicaAgentV1,REPLICA_AGENT_LABEL_V1} from "../../🩹️replica-agent/🟦️.ts";

const target=`gui/501/${REPLICA_AGENT_LABEL_V1}`;
const setup=()=>{const home=mkdtempSync(join(tmpdir(),"heal-agent-")),calls:string[]=[],lines:string[]=[];return{home,calls,lines,stamp:join(home,"stamp")};};
const options=(box:ReturnType<typeof setup>,print:{status:number;stdout:string},kick=0,now=()=>Date.now())=>({platform:"darwin",home:box.home,uid:501,exists:()=>true,stamp:box.stamp,now,notice:(line:string)=>box.lines.push(line),run:(program:string,args:readonly string[])=>{box.calls.push([program,...args].join(" "));return args[0]==="print"?print:{status:kick,stdout:""};}});

test("a loaded agent that does not run gets a demand start and one bilingual line, never a restart of a running one",()=>{
 const box=setup();
 try{
  expect(healReplicaAgentV1(options(box,{status:0,stdout:"\tstate = not running\n\truns = 40\n\tpended nondemand spawn = inefficient\n"}))).toBe("kickstarted");
  expect(box.calls).toEqual([`launchctl print ${target}`,`launchctl kickstart ${target}`]);
  expect(box.lines.length).toBe(1);expect(box.lines[0]).toContain("started it now");expect(box.lines[0]).toContain("wurde jetzt gestartet");
  rmSync(box.stamp);box.calls.length=0;
  expect(healReplicaAgentV1(options(box,{status:0,stdout:"\tstate = running\n\tpid = 7\n"}))).toBe("running");
  expect(box.calls).toEqual([`launchctl print ${target}`]);
 }finally{rmSync(box.home,{recursive:true,force:true});}
});

test("it looks at most once a minute, again after a minute, and says what to run when launchd refuses",()=>{
 const box=setup();
 try{
  const sick={status:0,stdout:"\tstate = not running\n"};let clock=1_000_000;
  expect(healReplicaAgentV1(options(box,sick,0,()=>clock))).toBe("kickstarted");
  utimesSync(box.stamp,new Date(clock),new Date(clock));
  clock+=30_000;box.calls.length=0;
  expect(healReplicaAgentV1(options(box,sick,0,()=>clock))).toBe("skipped");expect(box.calls).toEqual([]);
  clock+=40_000;
  expect(healReplicaAgentV1(options(box,sick,5,()=>clock))).toBe("failed");
  expect(box.lines.at(-1)).toContain(`launchctl kickstart -k ${target}`);expect(box.lines.at(-1)).toContain("ausführen");
 }finally{rmSync(box.home,{recursive:true,force:true});}
});

test("other systems, a machine without the agent and an agent that is not loaded are left alone",()=>{
 const box=setup();
 try{
  const sick={status:0,stdout:"\tstate = not running\n"};
  expect(healReplicaAgentV1({...options(box,sick),platform:"linux"})).toBe("skipped");
  expect(healReplicaAgentV1({...options(box,sick),exists:()=>false})).toBe("skipped");
  expect(box.calls).toEqual([]);
  expect(healReplicaAgentV1(options(box,{status:113,stdout:""}))).toBe("unloaded");
  expect(box.calls).toEqual([`launchctl print ${target}`]);expect(box.lines).toEqual([]);
  expect(statSync(box.stamp).isFile()).toBe(true);
 }finally{rmSync(box.home,{recursive:true,force:true});}
});
