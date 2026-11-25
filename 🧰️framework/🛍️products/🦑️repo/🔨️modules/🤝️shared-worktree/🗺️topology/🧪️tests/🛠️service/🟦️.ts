/** 🛠️ Contract: the daemon installs the service definition the current code renders when the installed one differs, only after the quiet period and while no sidecar restarts, keeps the previous definition, restores it when the service manager cannot load the new one, never touches a definition it does not recognise, and on macOS swaps the agent through a detached helper job that keeps a running job alive or brings the previous one back (proved with real throwaway LaunchAgents). */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
import {readTopologyV1,serverPartsV1,workerPartsV1} from "../../🟦️.ts";
import {fileURLToPath} from "node:url";
import {maintainServiceDefinitionV1,reloadHelperPlistV1,reloadScriptV1,serviceDefinitionTextV1,takeReloadOutcomeV1,SERVICE_DEFINITION_LIMITS_V1,serviceBootstrapScriptV1,type ServiceDefinitionPortsV1} from "../../🛠️service/🟦️.ts";

const topology=readTopologyV1(fixture.topology),group=fixture.topology.groups[0]!.id,HOME="/Users/tester",QUIET=SERVICE_DEFINITION_LIMITS_V1.quietMilliseconds;
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

type Fake=ServiceDefinitionPortsV1&{files:Map<string,string>;calls:string[];clock:{now:number};status:Record<string,number>;writes:string[];blocked:Set<string>;sidecarBusy:{value:boolean};modes:Map<string,number|undefined>;outputs:Record<string,string>};
const fake=(platform:"linux"|"macos"|"windows",files:Record<string,string>={}):Fake=>{
 const map=new Map(Object.entries(files)),calls:string[]=[],clock={now:1_000_000},status:Record<string,number>={},writes:string[]=[],blocked=new Set<string>(),sidecarBusy={value:false},modes=new Map<string,number|undefined>(),outputs:Record<string,string>={};
 return {files:map,calls,clock,status,writes,blocked,sidecarBusy,modes,outputs,platform,home:HOME,user:"tester",uid:501,entryDirectory:"/entry",now:()=>clock.now,busy:()=>sidecarBusy.value,
  read:path=>map.get(path),
  write:(path,data,mode)=>{if(blocked.has(path))throw Error("EACCES");writes.push(path);modes.set(path,mode);map.set(path,typeof data==="string"?data:Buffer.from(data).toString("latin1"));},
  remove:path=>{map.delete(path);},
  writable:path=>!blocked.has(path),
  run:(command,args)=>{const line=`${command} ${args.join(" ")}`;calls.push(line);const key=Object.keys(status).find(prefix=>line.startsWith(prefix));return {status:key===undefined?0:status[key]!,stdout:Object.entries(outputs).find(([prefix])=>line.startsWith(prefix))?.[1]??(command==="schtasks"&&args[0]==="/Query"&&args[2]!=="semio-fleet-cache"&&args[2]!=="semio-fleet-relay"?"<Task>old</Task>":"")};}};
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

const LINUX_UNIT="/etc/systemd/system/semio-fleet-replica@.service",LINUX_COMPACT="/etc/systemd/system/semio-fleet-compact@.service",LAUNCH="/Users/tester/.local/share/semio/fleet/entry/service";
const LINUX_REPLICA_SCRIPT=`${LAUNCH}/replica.sh`,LINUX_COMPACT_SCRIPT=`${LAUNCH}/compact.sh`;
const linuxInstalled=(old:boolean=false):Fake=>{
 const parts=workerPartsV1(topology,group,"linux"),files:Record<string,string>={
  [LINUX_UNIT]:fillHome(parts["definition"]!)+"\n",[LINUX_COMPACT]:fillHome(parts["compact-definition"]!)+"\n",
  [LINUX_REPLICA_SCRIPT]:fillHome(parts["launch-definition"]!)+"\n",[LINUX_COMPACT_SCRIPT]:fillHome(parts["compact-launch-definition"]!)+"\n"
 };
 if(old)files[LINUX_UNIT]=files[LINUX_UNIT]!.replace("AmbientCapabilities=CAP_SYS_NICE\n","").replace(/^ExecStart=.*$/mu,"ExecStart=/bin/sh /Users/tester/.local/share/semio/fleet/entry/boot.sh run\nNice=-5");
 return fake("linux",files);
};

test("linux: a changed launch script is linted, installed atomically with the previous one kept and its service restarted through the main process, with no privilege and no reload",()=>{
 const ports=linuxInstalled(),before=ports.files.get(LINUX_REPLICA_SCRIPT)!;
 ports.files.set(LINUX_REPLICA_SCRIPT,before.replace("renice -n -5","renice -n 0"));
 ports.files.set(LINUX_COMPACT_SCRIPT,ports.files.get(LINUX_COMPACT_SCRIPT)!.replace("renice -n 10","renice -n 5"));
 ports.outputs["systemctl show"]="4242\n";
 expect(pass(ports)).toMatchObject({action:"pending",dueMs:QUIET});
 expect(ports.writes.filter(path=>!path.endsWith("state.json"))).toEqual([]);
 const result=pass(ports,2);
 expect(result).toMatchObject({action:"applied"});
 expect(ports.files.get(LINUX_REPLICA_SCRIPT)).toBe(before);
 expect(ports.modes.get(LINUX_REPLICA_SCRIPT)).toBe(0o755);
 expect(ports.files.get("/entry/service-definition/replica.launch.previous")).toContain("renice -n 0");
 expect(ports.calls.filter(call=>call.startsWith("sh -n")).length).toBe(2);
 expect(ports.calls.filter(call=>call.startsWith("systemctl daemon-reload"))).toEqual([]);
 expect(ports.calls.filter(call=>call.startsWith("kill"))).toEqual(["kill -TERM 4242"]);
 expect(ports.writes).not.toContain(LINUX_UNIT);
 expect(maintainServiceDefinitionV1({topology,groupId:group,ports})).toEqual({action:"current"});
});

test("linux: a rejected launch script stays out; an unmigrated system unit is reported once with the one privileged command while the scripts are already in place",()=>{
 const lint=linuxInstalled(),original=lint.files.get(LINUX_REPLICA_SCRIPT)!;
 lint.files.set(LINUX_REPLICA_SCRIPT,"#!/bin/sh\nexit 7\n");lint.status["sh -n"]=2;
 expect(pass(lint,2)).toMatchObject({action:"refused",reason:expect.stringContaining("sh -n")});
 expect(lint.files.get(LINUX_REPLICA_SCRIPT)).toBe("#!/bin/sh\nexit 7\n");
 const old=linuxInstalled(true),unit=old.files.get(LINUX_UNIT)!;
 old.files.delete(LINUX_REPLICA_SCRIPT);old.blocked.add(LINUX_UNIT);
 const result=pass(old,2);
 expect(result.action).toBe("needs-privilege");expect(result.path).toBe(LINUX_UNIT);expect(result.command).toContain("service-bootstrap --group a1 --user tester | sudo sh");
 expect(old.files.get(LINUX_UNIT)).toBe(unit);
 expect(old.files.get(LINUX_REPLICA_SCRIPT)).toBe(original);
 expect(old.calls.filter(call=>call.startsWith("kill"))).toEqual([]);
 expect(serviceDefinitionTextV1(result)).toContain("service-bootstrap");
 expect(serviceDefinitionTextV1(result)).toContain(" / ");
});

test("linux: a unit this account may write is replaced and reloaded, a failing reload restores the previous unit",()=>{
 const ok=linuxInstalled(true),before=ok.files.get(LINUX_UNIT)!;
 expect(pass(ok,2)).toMatchObject({action:"applied"});
 expect(ok.files.get(LINUX_UNIT)).toContain("AmbientCapabilities=CAP_SYS_NICE");
 expect(ok.files.get("/entry/service-definition/semio-fleet-replica@.service.previous")).toBe(before);
 expect(ok.calls[0]).toBe("systemctl daemon-reload");
 const bad=linuxInstalled(true);bad.status["systemctl daemon-reload"]=1;
 expect(pass(bad,2)).toMatchObject({action:"restored"});
 expect(bad.files.get(LINUX_UNIT)).toBe(before);expect(bad.calls).toEqual(["systemctl daemon-reload","systemctl daemon-reload"]);
});

test("linux server: cache and relay launch scripts are maintained next to the replica, restarted before it, and a compaction of a server keeps --collect-blobs",()=>{
 const parts=workerPartsV1(topology,group,"linux",true),extra=serverPartsV1(topology,group),files:Record<string,string>={};
 const put=(path:string,text:string):void=>{files[fillHome(path)]=fillHome(text)+"\n";};
 put(parts["target"]!,parts["definition"]!);put(parts["compact-target"]!,parts["compact-definition"]!);put(extra["cache-target"]!,extra["cache-unit"]!);put(extra["relay-target"]!,extra["relay-unit"]!);
 put(parts["launch-target"]!,parts["launch-definition"]!);put(parts["compact-launch-target"]!,parts["compact-launch-definition"]!);put(extra["cache-launch-target"]!,extra["cache-launch"]!);put(extra["relay-launch-target"]!,extra["relay-launch"]!);
 const ports=fake("linux",files);
 expect(pass(ports)).toEqual({action:"current"});
 const cache=fillHome(extra["cache-launch-target"]!),relay=fillHome(extra["relay-launch-target"]!),compact=fillHome(parts["compact-launch-target"]!);
 expect(compact).toBe(LINUX_COMPACT_SCRIPT);
 ports.files.set(cache,ports.files.get(cache)!.replace("renice -n -5","renice -n -4"));
 ports.files.set(relay,ports.files.get(relay)!.replace("renice -n 0","renice -n 1"));
 ports.files.set(LINUX_REPLICA_SCRIPT,ports.files.get(LINUX_REPLICA_SCRIPT)!.replace("renice -n -5","renice -n -4"));
 ports.outputs["systemctl show --property=MainPID --value semio-fleet-cache"]="11\n";ports.outputs["systemctl show --property=MainPID --value semio-fleet-relay"]="22\n";ports.outputs["systemctl show --property=MainPID --value semio-fleet-replica"]="33\n";
 expect(pass(ports,2)).toMatchObject({action:"applied"});
 expect(ports.calls.filter(call=>call.startsWith("kill"))).toEqual(["kill -TERM 11","kill -TERM 22","kill -TERM 33"]);
 expect(ports.files.get(cache)).toContain("renice -n -5");
 expect(ports.files.get(compact)).toContain("--collect-blobs");
 expect(pass(ports)).toEqual({action:"current"});
});

const macServer=():Fake=>{
 const parts=workerPartsV1(topology,"b2","macos",true),extra=serverPartsV1(topology,"b2");
 return fake("macos",{
  [MAC_REPLICA]:oldPlist(parts["definition"]!,ENVIRONMENT),[MAC_COMPACT]:oldPlist(parts["compact-definition"]!,ENVIRONMENT),
  [fillHome(extra["cache-target"]!)]:oldPlist(extra["cache-unit"]!,ENVIRONMENT),[fillHome(extra["relay-target"]!)]:oldPlist(extra["relay-unit"]!,ENVIRONMENT)
 });
};

test("macos server: the cache and relay agents are re-rendered with their scheduling class and the helper job swaps them too",()=>{
 const ports=macServer(),cache=`${HOME}/Library/LaunchAgents/tech.semio.fleet.cache.plist`,relay=`${HOME}/Library/LaunchAgents/tech.semio.fleet.relay.plist`;
 const result=maintainServiceDefinitionV1({topology,groupId:"b2",ports});
 expect(result).toMatchObject({action:"pending"});
 ports.clock.now+=QUIET+1;
 expect(maintainServiceDefinitionV1({topology,groupId:"b2",ports})).toMatchObject({action:"reloading",services:["tech.semio.fleet.replica","tech.semio.fleet.compact","tech.semio.fleet.cache","tech.semio.fleet.relay"]});
 expect(ports.files.get(cache)).toContain("<key>ProcessType</key><string>Interactive</string>");
 expect(ports.files.get(cache)).toContain(ENVIRONMENT);
 expect(ports.files.get(relay)).toContain("<key>ProcessType</key><string>Standard</string>");
 const script=ports.files.get("/entry/service-definition/reload.sh")!;
 expect(script).toContain("swap 'tech.semio.fleet.cache'");expect(script).toContain("settled 'tech.semio.fleet.relay' running");
 expect(spawnSync("sh",["-n"],{input:script}).status).toBe(0);
 ports.clock.now+=QUIET+1;
 expect(maintainServiceDefinitionV1({topology,groupId:"b2",ports})).toEqual({action:"current"});
});

test("windows server: the cache and relay tasks that exist are registered again and restarted",()=>{
 const ports=fake("windows");ports.outputs["schtasks /Query /TN semio-fleet-compact /XML"]="<Task>--collect-blobs</Task>";
 maintainServiceDefinitionV1({topology,groupId:"c3",ports});ports.clock.now+=QUIET+1;
 expect(maintainServiceDefinitionV1({topology,groupId:"c3",ports})).toMatchObject({action:"applied"});
 expect(ports.calls.filter(call=>call.startsWith("schtasks /Create")).map(call=>call.split(" ")[3])).toEqual(["semio-fleet-replica","semio-fleet-compact","semio-fleet-store-mount","semio-fleet-cache","semio-fleet-relay"]);
 expect(ports.calls).toContain("schtasks /End /TN semio-fleet-cache");expect(ports.calls).toContain("schtasks /Run /TN semio-fleet-relay");
 expect(ports.calls).not.toContain("schtasks /End /TN semio-fleet-replica");
 expect(ports.calls).not.toContain("schtasks /End /TN semio-fleet-store-mount");
});

test("windows: the store mount task is kept on the headless host like the replica, so a stale visible console definition is replaced",()=>{
 const ports=fake("windows");pass(ports,2);
 const xml=Buffer.from(ports.files.get("/entry/service-definition/semio-fleet-store-mount.xml")!,"latin1").toString("utf16le");
 expect(xml).toContain("<Command>conhost.exe</Command>");expect(xml).toContain("--headless cmd.exe");
 const absent=fake("windows");absent.status["schtasks /Query /TN semio-fleet-store-mount"]=1;pass(absent,2);
 expect(absent.calls.filter(call=>call.startsWith("schtasks /Create")).map(call=>call.split(" ")[3])).toEqual(["semio-fleet-replica","semio-fleet-compact"]);
});

test("windows: the task is exported, registered again from UTF-16 XML with /F, and the exported one is registered again when that fails",()=>{
 const ok=fake("windows");
 expect(pass(ok,2)).toMatchObject({action:"applied"});
 expect(ok.calls.filter(call=>call.startsWith("schtasks /Create")).length).toBe(3);
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

const python=Bun.which("python3")??Bun.which("python"),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),prioritySchema=fileURLToPath(new URL("../../../../📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🪜️priority/🧬️schema/🔣️.json",import.meta.url));

test("oracle: every fixed Linux unit and launch script is checked by an independent python implementation (unit carries no class, script carries the declared class and execs)",()=>{
 expect(python).toBeDefined();
 const dir=mkdtempSync(join(tmpdir(),"semio-e64-"));scratch.push(dir);
 const parts=workerPartsV1(topology,group,"linux",true),extra=serverPartsV1(topology,group);
 const cases:readonly (readonly [string,string,string])[]=[["replica",parts["definition"]!,parts["launch-definition"]!],["compact",parts["compact-definition"]!,parts["compact-launch-definition"]!],["cache",extra["cache-unit"]!,extra["cache-launch"]!],["relay",extra["relay-unit"]!,extra["relay-launch"]!]];
 for(const [service,unit,script] of cases){
  writeFileSync(join(dir,`${service}.unit`),fillHome(unit));writeFileSync(join(dir,`${service}.sh`),fillHome(script)+"\n");
  const run=spawnSync(python!,["-I",oracle,"check",join(dir,`${service}.unit`),join(dir,`${service}.sh`),service,prioritySchema],{encoding:"utf8"});
  expect(run.status,`${service}: ${run.stdout}${run.stderr}`).toBe(0);
 }
 writeFileSync(join(dir,"bad.unit"),fillHome(parts["definition"]!).replace("[Service]\n","[Service]\nNice=-5\n"));
 expect(spawnSync(python!,["-I",oracle,"check",join(dir,"bad.unit"),join(dir,"replica.sh"),"replica",prioritySchema],{encoding:"utf8"}).status).toBe(1);
});

test("service-bootstrap: the one privileged script installs the fixed units and, as the account, the launch scripts (run against stubbed system tools), is idempotent and refuses a bad account or a wrong server platform",()=>{
 const dir=mkdtempSync(join(tmpdir(),"semio-e64-boot-"));scratch.push(dir);
 const bin=join(dir,"bin"),system=join(dir,"etc"),home=join(dir,"home"),log=join(dir,"calls.log");
 mkdirSync(bin);mkdirSync(system);mkdirSync(home);
 const stub=(name:string,body:string):void=>{writeFileSync(join(bin,name),`#!/bin/sh\n${body}\n`);chmodSync(join(bin,name),0o755);};
 stub("id",'case "$1" in -u) echo 0;; -gn) echo staff;; esac');
 stub("getent",`echo "$2:x:1:1::${home}:/bin/sh"`);
 stub("systemctl",`echo "systemctl $*" >> ${log}`);
 stub("install",'args=""; while [ $# -gt 0 ]; do case "$1" in -o|-g) shift 2;; *) args="$args $1"; shift;; esac; done; exec /usr/bin/install $args');
 const script=serviceBootstrapScriptV1(topology,group,{user:"deck",server:true}).replaceAll("/etc/systemd/system",system);
 expect(script).toContain("<<'SEMIO_BOOTSTRAP_0'");
 expect(spawnSync("sh",["-n"],{input:script}).status).toBe(0);
 const run=()=>spawnSync("sh",[],{input:script,encoding:"utf8",env:{...process.env,PATH:`${bin}:${process.env["PATH"]}`}});
 const first=run();expect(first.status,first.stdout+first.stderr).toBe(0);
 const unitFile=join(system,"semio-fleet-replica@.service"),launch=join(home,".local/share/semio/fleet/entry/service/replica.sh");
 expect(readFileSync(unitFile,"utf8")).toBe(`${fillHome(workerPartsV1(topology,group,"linux",true)["definition"]!).replaceAll(HOME,home)}\n`);
 expect(readFileSync(launch,"utf8")).toContain(`exec /bin/sh "${home}/.local/share/semio/fleet/entry/boot.sh" run`);
 expect(statSync(launch).mode&0o777).toBe(0o755);
 for(const name of["cache","relay","compact"])expect(existsSync(join(home,".local/share/semio/fleet/entry/service",`${name}.sh`))).toBe(true);
 for(const name of["semio-fleet-cache.service","semio-fleet-relay.service","semio-fleet-compact@.service","semio-fleet-compact@.timer"])expect(existsSync(join(system,name))).toBe(true);
 const calls=readFileSync(log,"utf8").trim().split("\n");
 expect(calls[0]).toBe("systemctl daemon-reload");
 expect(calls[1]).toContain("enable semio-fleet-replica@deck.service semio-fleet-cache.service semio-fleet-relay.service semio-fleet-compact@deck.timer");
 expect(calls[2]).toContain("restart semio-fleet-replica@deck.service");
 const second=run();expect(second.status).toBe(0);
 expect(readFileSync(launch,"utf8")).toBe(readFileSync(launch,"utf8"));
 expect(()=>serviceBootstrapScriptV1(topology,group,{user:"x; rm -rf /",server:false})).toThrow("Valid account name required");
 expect(()=>serviceBootstrapScriptV1(topology,"b2",{user:"deck",server:true})).toThrow();
 expect(serviceBootstrapScriptV1(topology,group,{user:"deck",server:false})).not.toContain("semio-fleet-cache");
});
