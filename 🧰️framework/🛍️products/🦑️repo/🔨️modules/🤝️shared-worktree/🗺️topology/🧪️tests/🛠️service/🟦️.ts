/** 🛠️ Contract: the daemon installs the service definition the current code renders when the installed one differs, only after the quiet period and while no sidecar restarts, keeps the previous definition, restores it when the service manager cannot load the new one, never touches a definition it does not recognise, and on macOS swaps the agent through a detached helper job that keeps a running job alive or brings the previous one back (proved with real throwaway LaunchAgents). */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {readTopologyV1,workerPartsV1} from "../../🟦️.ts";
import {maintainServiceDefinitionV1,reloadHelperPlistV1,reloadScriptV1,serviceDefinitionTextV1,takeReloadOutcomeV1,SERVICE_DEFINITION_LIMITS_V1,type ServiceDefinitionPortsV1} from "../../🛠️service/🟦️.ts";

const topology=readTopologyV1(fixture.topology),group=fixture.topology.groups[0]!.id,HOME="/Users/tester",QUIET=SERVICE_DEFINITION_LIMITS_V1.quietMilliseconds;
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

type Fake=ServiceDefinitionPortsV1&{files:Map<string,string>;calls:string[];clock:{now:number};status:Record<string,number>;writes:string[];blocked:Set<string>;sidecarBusy:{value:boolean}};
const fake=(platform:"linux"|"macos"|"windows",files:Record<string,string>={}):Fake=>{
 const map=new Map(Object.entries(files)),calls:string[]=[],clock={now:1_000_000},status:Record<string,number>={},writes:string[]=[],blocked=new Set<string>(),sidecarBusy={value:false};
 return {files:map,calls,clock,status,writes,blocked,sidecarBusy,platform,home:HOME,user:"tester",uid:501,entryDirectory:"/entry",now:()=>clock.now,busy:()=>sidecarBusy.value,
  read:path=>map.get(path),
  write:(path,data)=>{if(blocked.has(path))throw Error("EACCES");writes.push(path);map.set(path,typeof data==="string"?data:Buffer.from(data).toString("latin1"));},
  remove:path=>{map.delete(path);},
  writable:path=>!blocked.has(path),
  run:(command,args)=>{const line=`${command} ${args.join(" ")}`;calls.push(line);const key=Object.keys(status).find(prefix=>line.startsWith(prefix));return {status:key===undefined?0:status[key]!,stdout:command==="schtasks"&&args[0]==="/Query"?"<Task>old</Task>":""};}};
};
const fillHome=(text:string):string=>text.replaceAll("__WORKER_USER__","tester").replaceAll("__WORKER_HOME__",HOME);
const oldPlist=(definition:string,environment:string):string=>fillHome(definition).replace(/<key>(ProcessType|LowPriorityIO|Nice)<\/key>(<string>[^<]*<\/string>|<true\/>|<integer>\d+<\/integer>)/gu,"").replace("<key>EnvironmentVariables</key><dict>",`<key>EnvironmentVariables</key><dict>${environment}`)+"\n";
const ENVIRONMENT="<key>SEMIO_WORKSPACE_ROOT</key><string>/Users/tester/Documents/semio</string><key>SEMIO_FLEET_GROUP</key><string>g1</string>";
const MAC_REPLICA=`${HOME}/Library/LaunchAgents/tech.semio.fleet.replica.plist`,MAC_COMPACT=`${HOME}/Library/LaunchAgents/tech.semio.fleet.compact.plist`;
const macInstalled=():Fake=>{
 const parts=workerPartsV1(topology,group,"macos"),ports=fake("macos",{[MAC_REPLICA]:oldPlist(parts["definition"]!,ENVIRONMENT),[MAC_COMPACT]:oldPlist(parts["compact-definition"]!,ENVIRONMENT)});
 return ports;
};
const pass=(ports:Fake,times:number=1):ReturnType<typeof maintainServiceDefinitionV1>=>{let result=maintainServiceDefinitionV1({topology,groupId:group,ports});for(let index=1;index<times;index++){ports.clock.now+=QUIET+1;result=maintainServiceDefinitionV1({topology,groupId:group,ports});}return result;};

test("macos: a differing plist waits for the quiet period and for the sidecars, then is written atomically with the previous one kept and swapped by a detached helper job",()=>{
 const ports=macInstalled(),before=ports.files.get(MAC_REPLICA)!;
 expect(pass(ports)).toMatchObject({action:"pending",dueMs:QUIET});
 ports.clock.now+=1000;expect(maintainServiceDefinitionV1({topology,groupId:group,ports})).toMatchObject({action:"pending"});
 ports.clock.now+=QUIET;ports.sidecarBusy.value=true;
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports})).toMatchObject({action:"pending",reason:"a sidecar is restarting"});
 expect(ports.writes).not.toContain(MAC_REPLICA);
 ports.sidecarBusy.value=false;
 const result=maintainServiceDefinitionV1({topology,groupId:group,ports});
 expect(result).toMatchObject({action:"reloading",services:["tech.semio.fleet.replica","tech.semio.fleet.compact"]});
 const after=ports.files.get(MAC_REPLICA)!;
 expect(after).toContain("<key>ProcessType</key><string>Interactive</string>");
 expect(after).toContain(ENVIRONMENT);
 expect(ports.files.get("/entry/service-definition/tech.semio.fleet.replica.previous")).toBe(before);
 expect(ports.files.get(MAC_COMPACT)).toContain("<key>ProcessType</key><string>Background</string>");
 expect(ports.calls.filter(call=>call.startsWith("plutil -lint")).length).toBe(2);
 expect(ports.calls.at(-2)).toBe("launchctl bootout gui/501/tech.semio.fleet.replica.reload");
 expect(ports.calls.at(-1)).toBe("launchctl bootstrap gui/501 /entry/service-definition/tech.semio.fleet.replica.reload.plist");
 const script=ports.files.get("/entry/service-definition/reload.sh")!;
 expect(script).toContain("swap 'tech.semio.fleet.replica'");
 expect(spawnSync("sh",["-n"],{input:script}).status).toBe(0);
 expect(ports.writes.indexOf(MAC_REPLICA)).toBeLessThan(ports.writes.indexOf("/entry/service-definition/reload.sh"));
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports})).toEqual({action:"current"});
});

test("macos: a rejected plist, a failing helper start or an unrecognised plist leave the installed definitions untouched",()=>{
 const linted=macInstalled(),original=linted.files.get(MAC_REPLICA)!;
 linted.status["plutil"]=1;
 expect(pass(linted,2)).toMatchObject({action:"refused",reason:expect.stringContaining("plutil")});
 expect(linted.files.get(MAC_REPLICA)).toBe(original);
 const helper=macInstalled();helper.status["launchctl bootstrap"]=5;
 expect(pass(helper,2)).toMatchObject({action:"refused",reason:expect.stringContaining("helper")});
 expect(helper.files.get(MAC_REPLICA)).toBe(original);expect(helper.files.get(MAC_COMPACT)).toBe(macInstalled().files.get(MAC_COMPACT));
 const foreign=fake("macos",{[MAC_REPLICA]:"<plist><dict><key>Label</key><string>tech.semio.fleet.replica</string></dict></plist>\n"});
 expect(pass(foreign,2)).toEqual({action:"current"});expect(foreign.writes).toEqual([]);
 expect(pass(fake("macos"),2)).toEqual({action:"absent"});
});

test("macos: an up to date plist is current and clears a pending difference; the helper result is reported once",()=>{
 const ports=macInstalled();
 pass(ports,3);
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports})).toEqual({action:"current"});
 expect(JSON.parse(ports.files.get("/entry/service-definition/state.json")!)).toMatchObject({pendingDigest:""});
 ports.files.set("/entry/service-definition/result.json",`{"version":1,"atMs":${ports.clock.now+5},"outcome":"restored"}\n`);
 expect(takeReloadOutcomeV1(ports)).toMatchObject({action:"restored"});
 expect(takeReloadOutcomeV1(ports)).toBeUndefined();
 expect(serviceDefinitionTextV1({action:"restored",reason:"x"})).toContain(" / Der Dienstmanager konnte die neue Dienstdefinition nicht laden");
});

const LINUX_UNIT="/etc/systemd/system/semio-fleet-replica@.service",LINUX_COMPACT="/etc/systemd/system/semio-fleet-compact@.service";
const linuxInstalled=():Fake=>{
 const parts=workerPartsV1(topology,group,"linux"),strip=(text:string):string=>fillHome(text).replace(/^(Nice|CPUSchedulingPolicy|IOSchedulingClass|IOSchedulingPriority)=.*\n?/gmu,"")+"\n";
 return fake("linux",{[LINUX_UNIT]:strip(parts["definition"]!),[LINUX_COMPACT]:strip(parts["compact-definition"]!)});
};

test("linux: a unit this account may write is replaced, reloaded and restarted without blocking; a failing reload restores the previous unit; a system unit is reported",()=>{
 const ok=linuxInstalled(),before=ok.files.get(LINUX_UNIT)!;
 expect(pass(ok,2)).toMatchObject({action:"applied"});
 expect(ok.files.get(LINUX_UNIT)).toContain("Nice=-5");
 expect(ok.files.get("/entry/service-definition/semio-fleet-replica@.service.previous")).toBe(before);
 expect(ok.calls).toEqual(["systemctl daemon-reload","systemctl --no-block restart semio-fleet-replica@tester.service"]);
 const bad=linuxInstalled();bad.status["systemctl daemon-reload"]=1;
 expect(pass(bad,2)).toMatchObject({action:"restored"});
 expect(bad.files.get(LINUX_UNIT)).toBe(before);expect(bad.calls).toEqual(["systemctl daemon-reload","systemctl daemon-reload"]);
 const root=linuxInstalled();root.blocked.add(LINUX_UNIT);
 expect(pass(root,2)).toMatchObject({action:"needs-privilege",path:LINUX_UNIT});
 expect(root.files.get(LINUX_UNIT)).toBe(before);
 expect(serviceDefinitionTextV1({action:"needs-privilege",path:LINUX_UNIT})).toContain(LINUX_UNIT);
});

test("windows: the task is exported, registered again from UTF-16 XML with /F, and the exported one is registered again when that fails",()=>{
 const ok=fake("windows");
 expect(pass(ok,2)).toMatchObject({action:"applied"});
 expect(ok.calls.filter(call=>call.startsWith("schtasks /Create")).length).toBe(2);
 expect(ok.calls[0]).toContain("/Query");
 const xml=ok.files.get("/entry/service-definition/semio-fleet-replica.xml")!;
 expect(xml.startsWith("\xff\xfe")).toBe(true);
 expect(Buffer.from(xml,"latin1").toString("utf16le")).toContain("<Priority>3</Priority>");
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports:ok})).toEqual({action:"current"});
 const bad=fake("windows");bad.status["schtasks /Create /TN semio-fleet-replica"]=1;
 const result=pass(bad,2);
 expect(result).toMatchObject({action:"restored"});
 expect(bad.calls.filter(call=>call.includes("previous.xml")).length).toBe(1);
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports:bad})).toMatchObject({action:"pending"});
});

const launchctl=(...args:string[]):{status:number|null;stdout:string}=>{const run=spawnSync("launchctl",args,{encoding:"utf8"});return {status:run.status,stdout:run.stdout??""};};
const uid=process.getuid?.()??0,domainReady=process.platform==="darwin"&&launchctl("print",`gui/${uid}`).status===0;
const pidOf=(label:string):number|undefined=>{const match=/\bpid = (\d+)/u.exec(launchctl("print",`gui/${uid}/${label}`).stdout);return match===null?undefined:Number(match[1]);};
const priority=(pid:number):number=>Number(spawnSync("ps",["-o","pri=","-p",String(pid)],{encoding:"utf8"}).stdout.trim().replace(/\D/gu,""));
const waitFor=async(check:()=>boolean,seconds:number):Promise<boolean>=>{for(let index=0;index<seconds*4;index++){if(check())return true;await new Promise(done=>setTimeout(done,250));}return check();};
const job=(label:string,program:string,extra:string):string=>`<?xml version="1.0" encoding="UTF-8"?>\n<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n<plist version="1.0"><dict><key>Label</key><string>${label}</string><key>LimitLoadToSessionType</key><string>Aqua</string>${extra}<key>ProgramArguments</key><array><string>${program}</string><string>600</string></array><key>RunAtLoad</key><true/><key>KeepAlive</key><true/><key>ThrottleInterval</key><integer>1</integer></dict></plist>\n`;

const real=async(good:boolean):Promise<{outcome:string;pidBefore:number;pidAfter:number|undefined;before:number;after:number|undefined;helperGone:boolean}>=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-e27-launchd-"));scratch.push(directory);
 const label=`tech.semio.e27probe.${process.pid}.${good?"good":"bad"}`,helper=`${label}.reload`,target=join(directory,`${label}.plist`),previous=join(directory,"previous.plist"),result=join(directory,"result.json"),script=join(directory,"reload.sh"),helperPlist=join(directory,`${helper}.plist`),domain=`gui/${uid}`;
 try{
  const old=job(label,"/bin/sleep","");
  writeFileSync(target,old);writeFileSync(previous,old);
  expect(launchctl("bootstrap",domain,target).status).toBe(0);
  expect(await waitFor(()=>pidOf(label)!==undefined,10)).toBe(true);
  const pidBefore=pidOf(label)!,before=priority(pidBefore);
  writeFileSync(target,good?job(label,"/bin/sleep","<key>ProcessType</key><string>Interactive</string>"):job(label,"/nonexistent/semio-e27","")); 
  writeFileSync(script,reloadScriptV1({domain,helper,result,items:[{label,plist:target,previous,expect:"running"}],delaySeconds:1,settleSeconds:6}),{mode:0o755});
  writeFileSync(helperPlist,reloadHelperPlistV1(helper,script,join(directory,"reload.log")));
  expect(launchctl("bootstrap",domain,helperPlist).status).toBe(0);
  expect(await waitFor(()=>existsSync(result),90)).toBe(true);
  const outcome=(JSON.parse(readFileSync(result,"utf8")) as {outcome:string}).outcome;
  await waitFor(()=>launchctl("print",`${domain}/${helper}`).status!==0,10);
  await waitFor(()=>pidOf(label)!==undefined,10);
  const pidAfter=pidOf(label);
  return {outcome,pidBefore,pidAfter,before,after:pidAfter===undefined?undefined:priority(pidAfter),helperGone:launchctl("print",`${domain}/${helper}`).status!==0};
 }finally{launchctl("bootout",`${domain}/${label}`);launchctl("bootout",`${domain}/${helper}`);}
};

test.skipIf(!domainReady)("real launchd: the helper job swaps a running LaunchAgent to its new plist (process type Interactive: priority 20 becomes 31) and boots itself out",async()=>{
 const outcome=await real(true);
 expect(outcome.outcome).toBe("applied");
 expect(outcome.before).toBe(20);expect(outcome.after).toBe(31);
 expect(outcome.pidAfter).not.toBe(outcome.pidBefore);expect(outcome.helperGone).toBe(true);
},180_000);

test.skipIf(!domainReady)("real launchd: a new plist that does not run is replaced by the previous one and the job runs again",async()=>{
 const outcome=await real(false);
 expect(outcome.outcome).toBe("restored");
 expect(outcome.pidAfter).toBeDefined();expect(outcome.after).toBe(20);expect(outcome.helperGone).toBe(true);
},180_000);
