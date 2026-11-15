/** 🍎️ The launchd route of the resident service: when it is taken, the property list (judged by Python's plistlib, an independent reader) with an allow-listed environment only, the demand start after the bootstrap and its verification, the fallback to a direct spawn when launchd does not run the job, the clean-up of stale jobs and property lists, a stable property list location. */
import {expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync,statSync,readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {allowedEnvironmentV1,cleanServiceJobsV1,ensureLauncherConsentV1,launchAgentLabelV1,launchAgentPlistV1,launchdRouteV1,listServiceJobsV1,startViaLaunchdV1,stopServiceJobsV1} from "../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";

const home="/Users/worker",identity=`${home}/.local/share/semio/launcher/bin/repo`,bun="/Users/worker/Library/Caches/semio/runtimes/bun/aa/bun",command=[bun,`${home}/Documents/semio/door/script.ts`,"repo","service","host","run"];
const base={tag:"abc123def456",command,environment:{} as Record<string,string>,cwd:`${home}/Documents/semio`,runtimeDirectory:"/var/folders/x/semio-repo/501/host-aaa",platform:"darwin",home,exists:(path:string)=>path===identity,executable:bun,uid:501};
const plistOracle=(text:string)=>JSON.parse(spawnSync("python3",["-I","-c","import plistlib,sys,json;print(json.dumps(plistlib.loads(sys.stdin.buffer.read())))"],{input:text,encoding:"utf8"}).stdout);

type Job={label:string;pid?:number;runs:number};
const launchd=(options:{bootstrap?:number;kickstart?:number;runsOnKick?:boolean;jobs?:Job[]}={})=>{
 const jobs=new Map<string,Job>((options.jobs??[]).map(job=>[job.label,{...job}])),calls:string[]=[];
 const run=(program:string,args:readonly string[])=>{
  calls.push([program,...args].join(" "));expect(program).toBe("launchctl");
  const label=(args[1]??"").split("/").at(-1)!;
  if(args[0]==="list")return{status:0,stdout:"PID\tStatus\tLabel\n"+[...jobs.values()].map(job=>`${job.pid??"-"}\t0\t${job.label}`).join("\n")+"\n123\t0\tcom.apple.other\n"};
  if(args[0]==="bootout"){const had=jobs.delete(label);return had?0:113;}
  if(args[0]==="bootstrap"){if(options.bootstrap)return options.bootstrap;jobs.set(plistLabel(args[2]!),{label:plistLabel(args[2]!),runs:0});return 0;}
  if(args[0]==="kickstart"){if(options.kickstart)return options.kickstart;const job=jobs.get(label);if(!job)return 113;if(options.runsOnKick!==false){job.runs++;job.pid=4000;}return 0;}
  if(args[0]==="print"){const job=jobs.get(label);return job?{status:0,stdout:`\tstate = ${job.pid?"running":"not running"}\n\truns = ${job.runs}\n${job.pid?`\tpid = ${job.pid}\n`:""}`}:{status:113,stdout:""};}
  return 1;
 };
 return{run,calls,jobs};
};
const plistLabel=(path:string)=>path.split("/").at(-1)!.replace(/\.plist$/u,"");
const quick={verifyMilliseconds:100,pollMilliseconds:10,wait:()=>undefined};

test("the route is taken only on macOS, below a protected folder, for the runtime running the service script, with the launcher present, and its property list lives in a stable folder of the user",()=>{
  const route=launchdRouteV1(base)!;
  expect(route).toMatchObject({program:identity,arguments:command.slice(1),workingDirectory:base.cwd,domain:"gui/501",jobsDirectory:`${home}/Library/Application Support/semio/service-jobs`});
  expect(route.plistPath).toBe(`${route.jobsDirectory}/${route.label}.plist`);expect(route.plistPath).not.toContain("/var/folders");
  expect(route.target).toBe(`gui/501/${route.label}`);expect(route.label).toBe(launchAgentLabelV1(base.runtimeDirectory,base.tag));
  expect(launchdRouteV1({...base,tag:"another00000"})!.label,"a new implementation gets its own job").not.toBe(route.label);
  expect(route.label).toMatch(/^tech\.semio\.repo\.service\.[0-9a-f]{12}$/u);
  for(const [name,over] of Object.entries({linux:{platform:"linux"},"outside protected folders":{cwd:"/Users/Shared/semio"},"foreign command":{executable:"/usr/bin/other"},"no launcher":{exists:()=>false},"switched off":{environment:{SEMIO_SERVICE_LAUNCHD:"off"}}}))expect(launchdRouteV1({...base,...over as object}),name).toBeUndefined();
  expect(launchdRouteV1({...base,environment:{XDG_DATA_HOME:"/data"},exists:path=>path==="/data/semio/launcher/bin/repo"})?.program).toBe("/data/semio/launcher/bin/repo");
});

test("the environment of a job is an allow-list: a variable named like a token, an e-mail address, a session or a uuid of the caller is absent, so are unknown variables",()=>{
  const caller={HOME:home,PATH:"/usr/bin:/bin",LANG:"de_DE.UTF-8",TMPDIR:"/var/folders/x/T/",SEMIO_LANGUAGE:"de",SEMIO_WORKSPACE_ROOT:`${home}/Documents/semio`,BUN_JSC_forceRAMSize:"1024",X_TOKEN:"t0ken",SEMIO_REMOTE_CACHE_TOKEN:"secret",AGENT_EMAIL:"a@b.c",CLAUDE_SESSION_UUID:"u-u-i-d",SEMIO_AGENT_SESSION:"s",SEMIO_SCRIPT_PROCESS_INVOCATION:"{}",SSH_AUTH_SOCK:"/x",ANTHROPIC_API_KEY:"k",RANDOM_OTHER:"x",EMPTY:"",SEMIO_EMPTY:""};
  const allowed=allowedEnvironmentV1(caller);
  expect(Object.keys(allowed)).toEqual(["BUN_JSC_forceRAMSize","HOME","LANG","PATH","SEMIO_LANGUAGE","SEMIO_WORKSPACE_ROOT","TMPDIR"]);
  const text=launchAgentPlistV1({label:"tech.semio.repo.service.abc",program:identity,arguments:command.slice(1),workingDirectory:base.cwd,environment:caller,log:"/l"});
  for(const secret of ["X_TOKEN","t0ken","secret","a@b.c","u-u-i-d","SSH_AUTH_SOCK","ANTHROPIC_API_KEY","RANDOM_OTHER"])expect(text,secret).not.toContain(secret);
  expect(plistOracle(text).EnvironmentVariables).toEqual(allowed);
});

test("the property list parses with plistlib to exactly the job: no RunAtLoad (the demand start follows), never restarted by launchd, children kept",()=>{
  const text=launchAgentPlistV1({label:"tech.semio.repo.service.abc",program:identity,arguments:command.slice(1),workingDirectory:base.cwd,environment:{HOME:home,SPECIAL:"x",SEMIO_NOTE:`a<b>&"c' ü`,"BAD=NAME":"x"},log:"/var/log/s.log"});
  expect(plistOracle(text)).toEqual({Label:"tech.semio.repo.service.abc",ProgramArguments:[identity,...command.slice(1)],WorkingDirectory:base.cwd,EnvironmentVariables:{HOME:home,SEMIO_NOTE:`a<b>&"c' ü`},StandardOutPath:"/var/log/s.log",StandardErrorPath:"/var/log/s.log",KeepAlive:false,AbandonProcessGroup:true});
  expect(text).not.toContain("RunAtLoad");
});

test("the job is loaded and started by demand, in order: stale out, old out, file in, bootstrap, kickstart, print until launchd reports a run; the file stays for the life of the job",()=>{
  const route=launchdRouteV1(base)!,fake=launchd(),files=new Map<string,string>();
  const outcome=startViaLaunchdV1(route,{environment:{HOME:home,X_TOKEN:"t"},log:"/l",run:fake.run,write:(path,text)=>{files.set(path,text);},remove:path=>{files.delete(path);},files:()=>[],...quick});
  expect(outcome).toEqual({started:true,reason:"running"});
  expect(fake.calls).toEqual(["launchctl list",`launchctl bootout ${route.target}`,`launchctl bootstrap gui/501 ${route.plistPath}`,`launchctl kickstart ${route.target}`,`launchctl print ${route.target}`]);
  expect(files.get(route.plistPath)).toBeDefined();expect(files.get(route.plistPath)).not.toContain("X_TOKEN");
});

test("a job that launchd does not run within the bound (runs stays 0, as with pended nondemand spawn) is removed with its file and answers not started, so the caller spawns the service directly",()=>{
  const route=launchdRouteV1(base)!,fake=launchd({runsOnKick:false}),files=new Map<string,string>();
  const outcome=startViaLaunchdV1(route,{environment:{},log:"/l",run:fake.run,write:(path,text)=>{files.set(path,text);},remove:path=>{files.delete(path);},files:()=>[],...quick});
  expect(outcome.started).toBe(false);expect(outcome.reason).toContain("not started within");
  expect(fake.jobs.size).toBe(0);expect(files.size).toBe(0);
  for(const [name,options] of Object.entries({"bootstrap refused":{bootstrap:5},"kickstart refused":{kickstart:5}})){
   const again=launchd(options),kept=new Map<string,string>(),result=startViaLaunchdV1(route,{environment:{},log:"/l",run:again.run,write:(path,text)=>{kept.set(path,text);},remove:path=>{kept.delete(path);},files:()=>[],...quick});
   expect(result.started,name).toBe(false);expect(kept.size,name).toBe(0);expect(again.jobs.size,name).toBe(0);
  }
  const broken=startViaLaunchdV1(route,{environment:{},log:"/l",run:()=>{throw Object.assign(new Error("spawn launchctl ENOENT"),{code:"ENOENT"});},write:()=>undefined,remove:()=>undefined,files:()=>[],...quick});
  expect(broken).toMatchObject({started:false});
});

test("starting a service removes the jobs of earlier implementations that are not running and the property lists of no loaded job, and leaves a running service alone",()=>{
  const route=launchdRouteV1(base)!,label=(n:string)=>`tech.semio.repo.service.${n.repeat(12)}`;
  const fake=launchd({jobs:[{label:label("a"),runs:1},{label:label("b"),runs:1,pid:77},{label:route.label,runs:1}]}),removed:string[]=[];
  const gone=cleanServiceJobsV1(route,{run:fake.run,files:()=>[label("a")+".plist",label("b")+".plist",label("c")+".plist",route.label+".plist","notes.txt"],remove:path=>removed.push(path.split("/").at(-1)!)});
  expect(gone.sort()).toEqual([label("a"),label("c")]);
  expect([...fake.jobs.keys()].sort()).toEqual([label("b"),route.label].sort());
  expect(removed.sort()).toEqual([label("a")+".plist",label("c")+".plist"]);
  expect(listServiceJobsV1(fake.run).map(job=>job.pid).sort()).toEqual([77,undefined].sort());
});

test("`service host stop` unloads every service job, running or not, and removes every property list; other jobs of the user are not touched",()=>{
  const label=(n:string)=>`tech.semio.repo.service.${n.repeat(12)}`,fake=launchd({jobs:[{label:label("a"),runs:1},{label:label("b"),runs:1,pid:77}]}),removed:string[]=[];
  expect(stopServiceJobsV1({platform:"darwin",uid:501,home,run:fake.run,files:()=>[label("a")+".plist",label("b")+".plist","x.txt"],remove:path=>removed.push(path.split("/").at(-1)!)})).toBe(2);
  expect(fake.jobs.size).toBe(0);expect(removed.sort()).toEqual([label("a")+".plist",label("b")+".plist"]);
  expect(fake.calls.some(call=>call.includes("com.apple.other"))).toBe(false);
  expect(stopServiceJobsV1({platform:"linux",uid:501,home,run:()=>{throw Error("never");}})).toBe(0);
});

test("the default writer creates the stable folder owner-only and writes the file owner-only",()=>{
  const directory=mkdtempSync(join(tmpdir(),"launchd-")),route={...launchdRouteV1(base)!,jobsDirectory:join(directory,"jobs"),plistPath:join(directory,"jobs",launchdRouteV1(base)!.label+".plist")},fake=launchd();let mode=-1,folder=-1,text="";
  try{
   const outcome=startViaLaunchdV1(route,{environment:{HOME:"/h",X_TOKEN:"t"},log:"/l",run:(program,args)=>{if(args[0]==="bootstrap"){mode=statSync(route.plistPath).mode&0o777;folder=statSync(route.jobsDirectory).mode&0o777;text=readFileSync(route.plistPath,"utf8");}return fake.run(program,args);},files:()=>[],...quick});
   expect(outcome.started).toBe(true);expect(mode).toBe(0o600);expect(folder).toBe(0o700);expect(plistOracle(text).EnvironmentVariables).toEqual({HOME:"/h"});
   expect(readdirSync(join(directory,"jobs"))).toEqual([route.label+".plist"]);
  }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the consent of the launcher is made sure by a probe job under the launcher identity: ready at once when the marker is there, ready when the click comes within the bound, waiting (job left running) when it does not, failed when launchd does not run it, unneeded outside protected folders",()=>{
  const root=`${home}/Documents/semio`,common={root,probeScript:"/probe.mjs",platform:"darwin",home,environment:{HOME:home,X_TOKEN:"t"} as Record<string,string>,uid:501,exists:(path:string)=>path===identity,wait:()=>undefined,waitMilliseconds:50,pollMilliseconds:10,write:()=>undefined,remove:()=>undefined};
  const never=launchd();
  expect(ensureLauncherConsentV1({...common,run:never.run,consented:()=>true})).toEqual({state:"ready"});expect(never.calls).toEqual([]);
  expect(ensureLauncherConsentV1({...common,run:never.run,platform:"linux",consented:()=>false})).toEqual({state:"unneeded"});
  expect(ensureLauncherConsentV1({...common,root:"/Users/Shared/semio",run:never.run,consented:()=>false})).toEqual({state:"unneeded"});
  expect(ensureLauncherConsentV1({...common,exists:()=>false,run:never.run,consented:()=>false})).toEqual({state:"unneeded"});
  const clicks=launchd(),written:string[]=[];let polls=0;
  expect(ensureLauncherConsentV1({...common,run:clicks.run,write:(_path,text)=>{written.push(text);},consented:()=>++polls>3})).toEqual({state:"ready"});
  expect(clicks.calls.map(call=>call.split(" ").slice(0,2).join(" "))).toEqual(["launchctl print","launchctl bootout","launchctl bootstrap","launchctl kickstart","launchctl bootout"]);
  expect(plistOracle(written[0]!)).toMatchObject({ProgramArguments:[identity,"/probe.mjs",root],EnvironmentVariables:{HOME:home,SEMIO_WORKSPACE_ROOT:root}});
  expect(written[0]).not.toContain("X_TOKEN");expect(written[0]).not.toContain("WorkingDirectory");
  const rebuilt=launchd();
  expect(ensureLauncherConsentV1({...common,run:rebuilt.run,consented:()=>false,stateOf:()=>"changed"}),"a launcher replaced by other bytes after it was allowed").toEqual({state:"waiting",changed:true});
  const waiting=launchd();
  expect(ensureLauncherConsentV1({...common,run:waiting.run,consented:()=>false})).toEqual({state:"waiting"});
  expect(waiting.calls.at(-1)).toContain("kickstart");expect(waiting.jobs.size,"the probe job is left running for the click").toBe(1);
  const again=launchd({jobs:[{label:[...waiting.jobs.keys()][0]!,runs:1,pid:55}]});
  expect(ensureLauncherConsentV1({...common,run:again.run,consented:()=>false})).toEqual({state:"waiting"});
  expect(again.calls.some(call=>call.includes("bootstrap")||call.includes("kickstart")),"a probe that waits is not started twice").toBe(false);
  expect(ensureLauncherConsentV1({...common,run:launchd({bootstrap:5}).run,consented:()=>false})).toMatchObject({state:"failed"});
  expect(ensureLauncherConsentV1({...common,run:launchd({kickstart:5}).run,consented:()=>false})).toMatchObject({state:"failed"});
});
