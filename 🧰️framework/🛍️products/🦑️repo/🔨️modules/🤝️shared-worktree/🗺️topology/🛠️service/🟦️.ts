import {spawnSync} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {accessSync,chmodSync,constants,mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {compileJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {groupV1,workerPartsV1,type FleetPlatformV1,type FleetTopologyV1} from "../🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧮️ The numbers of the reload: the quiet period of a difference (the debounce of a code restart), the seconds a reloaded job has to settle before the previous definition comes back, and the seconds the helper waits before it starts. See `./🧬️schema/🔣️.json`. */
export const SERVICE_DEFINITION_LIMITS_V1=Object.freeze({...schema.definitions.Limits.const});

/** 🔌️ The operating system of the reload, replaceable by tests: the platform and the account, the entry directory that keeps state and previous definitions, the clock, file access (`write` is atomic and creates folders), a runner for the platform's own commands (`launchctl`, `systemctl`, `schtasks`, `plutil`) and whether a sidecar is in the middle of a restart. */
export type ServiceDefinitionPortsV1=Readonly<{platform:FleetPlatformV1;home:string;user:string;uid:number;entryDirectory:string;now():number;read(path:string):string|undefined;write(path:string,data:string|Uint8Array,mode?:number):void;remove(path:string):void;writable(path:string):boolean;run(command:string,args:readonly string[]):Readonly<{status:number|null;stdout:string}>;busy():boolean}>;

/** 📋️ What one maintenance pass did: `absent` (nothing installed here), `current`, `pending` (a difference waits for its quiet period, `dueMs` says how long), `reloading` (written, the detached helper reloads the service manager and records the outcome), `applied`, `restored`, `needs-privilege`, `refused`. */
export type ServiceDefinitionResultV1=Readonly<{action:"absent"|"current"|"pending"|"reloading"|"applied"|"restored"|"needs-privilege"|"refused";reason?:string;dueMs?:number;services?:readonly string[];path?:string}>;

type Unit=Readonly<{name:"replica"|"compact";target:string;label:string;desired:string;installed:string|undefined}>;
type State=Readonly<{version:1;pendingDigest:string;pendingMs:number;appliedDigest:string;appliedMs:number;reportedMs:number}>;

const validators=new Map<string,ReturnType<typeof compileJsonSchemaSubset>>();
function requireShape(definition:"State"|"Result",value:unknown):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${definition}: ${errors[0]}`);
}

const sha256=(text:string):string=>createHash("sha256").update(text).digest("hex");
const xml=(text:string):string=>text.replaceAll("&","&amp;").replaceAll("<","&lt;");
const quoted=(text:string):string=>`'${text.replaceAll("'","'\\''")}'`;
const normal=(text:string|undefined):string|undefined=>text?.replaceAll("\r\n","\n").replace(/\n+$/u,"");

/** 🔔️ The message of a result, English and German in one line. */
export function serviceDefinitionTextV1(result:ServiceDefinitionResultV1):string{
 const key=result.action==="needs-privilege"?"needsPrivilege":result.action==="pending"?"pending":result.action==="reloading"?"reloading":result.action==="applied"?"applied":result.action==="restored"?"restored":"refused";
 const fill=(language:"en"|"de"):string=>schema.definitions.Texts.const[language][key].replace("{seconds}",String(Math.round((result.dueMs??0)/1000))).replace("{services}",(result.services??[]).join(", ")).replace("{reason}",result.reason??"").replace("{path}",result.path??"");
 return `${fill("en")} / ${fill("de")}`;
}

/** 🍎️ The shell script of the detached helper that swaps LaunchAgents: for each item `bootout`, `bootstrap`, wait until the job runs (or is loaded), and when it does not, copy the previous plist back and bootstrap that; it records the outcome in `result` and boots itself out last. launchd reads a plist only at `bootstrap`, and `bootout` of the replica ends the daemon that would do it, so the swap runs as a job of its own. See https://keith.github.io/xcode-man-pages/launchctl.1.html */
export function reloadScriptV1(input:Readonly<{domain:string;helper:string;result:string;items:readonly Readonly<{label:string;plist:string;previous:string;expect:"running"|"loaded"}>[];delaySeconds?:number;settleSeconds?:number}>):string{
 const steps=input.items.map(item=>`swap ${quoted(item.label)} ${quoted(item.plist)}\nif ! settled ${quoted(item.label)} ${item.expect}; then\n cp ${quoted(item.previous)} ${quoted(item.plist)}.restore && mv ${quoted(item.plist)}.restore ${quoted(item.plist)}\n swap ${quoted(item.label)} ${quoted(item.plist)}\n if settled ${quoted(item.label)} ${item.expect}; then [ "$outcome" = failed ] || outcome=restored; else outcome=failed; fi\nfi`);
 return [
  "#!/bin/sh",
  `domain=${quoted(input.domain)}`,`settle=${input.settleSeconds??SERVICE_DEFINITION_LIMITS_V1.settleSeconds}`,"outcome=applied",
  'loaded() { launchctl print "$domain/$1" >/dev/null 2>&1; }',
  'running() { launchctl print "$domain/$1" 2>/dev/null | grep -q "state = running"; }',
  'settled() { i=0; while [ "$i" -lt "$settle" ]; do if [ "$2" = running ]; then running "$1" && return 0; else loaded "$1" && return 0; fi; sleep 1; i=$((i+1)); done; return 1; }',
  'swap() { launchctl bootout "$domain/$1" >/dev/null 2>&1; i=0; while loaded "$1" && [ "$i" -lt 30 ]; do sleep 1; i=$((i+1)); done; launchctl bootstrap "$domain" "$2"; }',
  `sleep ${input.delaySeconds??SERVICE_DEFINITION_LIMITS_V1.helperDelaySeconds}`,
  ...steps,
  `printf '{"version":1,"atMs":%s,"outcome":"%s"}\\n' "$(($(date +%s)*1000))" "$outcome" > ${quoted(input.result)}`,
  `launchctl bootout "$domain/"${quoted(input.helper)} >/dev/null 2>&1`,
  ""
 ].join("\n");
}

/** 🍎️ The one-shot LaunchAgent that runs the helper script; a job of its own survives the `bootout` of the replica. */
export function reloadHelperPlistV1(label:string,script:string,log:string):string{
 return ['<?xml version="1.0" encoding="UTF-8"?>','<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">','<plist version="1.0">','<dict>',`<key>Label</key><string>${xml(label)}</string>`,"<key>LimitLoadToSessionType</key><string>Aqua</string>","<key>ProcessType</key><string>Interactive</string>",`<key>ProgramArguments</key><array><string>/bin/sh</string><string>${xml(script)}</string></array>`,"<key>RunAtLoad</key><true/>",`<key>StandardOutPath</key><string>${xml(log)}</string>`,`<key>StandardErrorPath</key><string>${xml(log)}</string>`,"</dict>","</plist>",""].join("\n");
}

/** 🔌️ The real ports of this machine. */
export function realServiceDefinitionPortsV1(entryDirectory:string,busy:()=>boolean,platform:FleetPlatformV1,home:string,user:string,uid:number):ServiceDefinitionPortsV1{
 return {platform,home,user,uid,entryDirectory,now:()=>Date.now(),busy,
  read:path=>{try{return readFileSync(path,"utf8");}catch{return undefined;}},
  write:(path,data,mode)=>{mkdirSync(dirname(path),{recursive:true});const temporary=`${path}.${process.pid}.${randomBytes(4).toString("hex")}.tmp`;try{writeFileSync(temporary,data);if(mode!==undefined)chmodSync(temporary,mode);renameSync(temporary,path);}catch(error){rmSync(temporary,{force:true});throw error;}},
  remove:path=>{rmSync(path,{force:true});},
  writable:path=>{try{accessSync(path,constants.W_OK);accessSync(dirname(path),constants.W_OK);return true;}catch{return false;}},
  run:(command,args)=>{try{const outcome=spawnSync(command,[...args],{encoding:"utf8",timeout:30_000,windowsHide:true});return {status:outcome.status,stdout:outcome.stdout??""};}catch{return {status:null,stdout:""};}}};
}

function readState(ports:ServiceDefinitionPortsV1,directory:string):State{
 try{const value=JSON.parse(ports.read(join(directory,"state.json"))??"") as State;requireShape("State",value);return value;}catch{return {version:1,pendingDigest:"",pendingMs:0,appliedDigest:"",appliedMs:0,reportedMs:0};}
}

const writeState=(ports:ServiceDefinitionPortsV1,directory:string,state:State):void=>{try{ports.write(join(directory,"state.json"),`${JSON.stringify(state)}\n`);}catch{}};

function fill(text:string,ports:ServiceDefinitionPortsV1):string{
 const home=ports.home.replace(/[\\/]+$/u,"");
 return text.replaceAll("__WORKER_USER__",xml(ports.user)).replaceAll("__WORKER_HOME__",xml(home));
}

function unitsOf(topology:FleetTopologyV1,groupId:string,ports:ServiceDefinitionPortsV1):readonly Unit[]{
 const {platform}=ports,own=workerPartsV1(topology,groupId,platform);
 const replicaTarget=fill(own["target"]!,ports),compactTarget=fill(own["compact-target"]!,ports);
 if(platform==="windows"){
  const server=ports.run("schtasks",["/Query","/TN",own["compact-target"]!,"/XML"]).stdout.includes("--collect-blobs"),parts=workerPartsV1(topology,groupId,platform,server);
  return [{name:"replica",target:parts["target"]!,label:parts["target"]!,desired:fill(parts["definition"]!,ports),installed:undefined},{name:"compact",target:parts["compact-target"]!,label:parts["compact-target"]!,desired:fill(parts["compact-definition"]!,ports),installed:undefined}];
 }
 const installedCompact=ports.read(compactTarget),server=installedCompact?.includes("--collect-blobs")??false,parts=workerPartsV1(topology,groupId,platform,server);
 const wrap=(name:"replica"|"compact",target:string,definition:string):Unit|undefined=>{
  const installed=ports.read(target);
  if(installed===undefined)return undefined;
  const label=target.replace(/^.*[\\/]/u,"").replace(/\.plist$/u,"");
  let desired=fill(definition,ports);
  if(platform==="macos"){
   const marker="<key>EnvironmentVariables</key><dict>",start=installed.indexOf(marker),end=installed.indexOf("<key>HOME</key>",start);
   if(start<0||end<0)return {name,target,label,desired:installed,installed};
   desired=desired.replace(marker,marker+installed.slice(start+marker.length,end));
  }
  return {name,target,label,desired:normal(desired)!,installed:normal(installed)};
 };
 return [wrap("replica",replicaTarget,parts["definition"]!),wrap("compact",compactTarget,parts["compact-definition"]!)].filter((unit):unit is Unit=>unit!==undefined);
}

/** 🛠️ Keeps the installed service definition of the replica daemon (and of the daily compaction) equal to what the current code renders, without any hands. The rendered definition keeps the machine's own environment block (provisioning owns it); a difference is waited for through the quiet period and while a sidecar restarts, then installed: macOS writes the plist atomically, keeps the previous one and starts a detached helper job that swaps the agents and restores the previous plist when the new one does not run; Linux writes the unit where this account may (a system unit needs the privileges of provisioning and is reported instead), runs `systemctl daemon-reload` and queues the restart, restoring the previous unit when the reload fails; Windows exports the task, registers the new XML with `schtasks /Create /F` and registers the exported one again on failure. A machine is never left without its service definition. */
export function maintainServiceDefinitionV1(options:Readonly<{topology:FleetTopologyV1;groupId:string;ports:ServiceDefinitionPortsV1;quietMilliseconds?:number}>):ServiceDefinitionResultV1{
 const {ports}=options,directory=join(ports.entryDirectory,"service-definition");
 try{groupV1(options.topology,options.groupId);}catch(error){return {action:"absent",reason:(error as Error).message};}
 const units=unitsOf(options.topology,options.groupId,ports);
 if(units.length===0)return {action:"absent"};
 const state=readState(ports,directory),now=ports.now();
 const refresh=(patch:Partial<State>):State=>{const next={...state,...patch} as State;writeState(ports,directory,next);return next;};
 const changed=units.filter(unit=>ports.platform==="windows"?false:unit.desired!==unit.installed);
 const digest=sha256(units.map(unit=>unit.desired).join("\n\0\n"));
 const differs=ports.platform==="windows"?state.appliedDigest!==digest:changed.length>0;
 if(!differs){if(state.pendingDigest!=="")refresh({pendingDigest:"",pendingMs:0});return {action:"current"};}
 const quiet=options.quietMilliseconds??SERVICE_DEFINITION_LIMITS_V1.quietMilliseconds;
 if(state.pendingDigest!==digest){refresh({pendingDigest:digest,pendingMs:now});return {action:"pending",dueMs:quiet};}
 if(now-state.pendingMs<quiet)return {action:"pending",dueMs:quiet-(now-state.pendingMs)};
 if(ports.busy())return {action:"pending",dueMs:60_000,reason:"a sidecar is restarting"};
 const done=(result:ServiceDefinitionResultV1,applied:boolean):ServiceDefinitionResultV1=>{refresh({pendingDigest:"",pendingMs:0,...applied?{appliedDigest:digest,appliedMs:now}:{}});return result;};
 const keep=(unit:Unit):string=>join(directory,`${unit.label.replaceAll(/[\\/]/gu,"_")}.previous`);
 if(ports.platform==="macos"){
  const staged=changed.map(unit=>({unit,next:join(directory,`${unit.label}.next`)}));
  for(const {unit,next} of staged){
   ports.write(next,`${unit.desired}\n`);
   const linted=ports.run("plutil",["-lint",next]);ports.remove(next);
   if(linted.status!==0)return done({action:"refused",reason:`plutil -lint rejected ${unit.label}`},false);
  }
  for(const {unit} of staged)ports.write(keep(unit),`${unit.installed}\n`);
  const written:Unit[]=[];
  try{for(const {unit} of staged){ports.write(unit.target,`${unit.desired}\n`);written.push(unit);}}
  catch(error){for(const unit of written)try{ports.write(unit.target,`${unit.installed}\n`);}catch{}return done({action:"refused",reason:(error as Error).message},false);}
  const helper=`${staged[0]!.unit.label}.reload`,script=join(directory,"reload.sh"),plist=join(directory,`${helper}.plist`),result=join(directory,"result.json"),domain=`gui/${ports.uid}`;
  ports.remove(result);
  ports.write(script,reloadScriptV1({domain,helper,result,items:staged.map(({unit})=>({label:unit.label,plist:unit.target,previous:keep(unit),expect:unit.name==="replica"?"running":"loaded"}))}),0o755);
  ports.write(plist,reloadHelperPlistV1(helper,script,join(directory,"reload.log")));
  ports.run("launchctl",["bootout",`${domain}/${helper}`]);
  const started=ports.run("launchctl",["bootstrap",domain,plist]);
  if(started.status!==0){for(const {unit} of staged)try{ports.write(unit.target,`${unit.installed}\n`);}catch{}return done({action:"refused",reason:"launchctl bootstrap of the helper failed"},false);}
  return done({action:"reloading",services:staged.map(({unit})=>unit.label)},true);
 }
 if(ports.platform==="linux"){
  const blocked=changed.find(unit=>!ports.writable(unit.target));
  if(blocked!==undefined)return done({action:"needs-privilege",path:blocked.target},false);
  const written:Unit[]=[];
  for(const unit of changed){ports.write(keep(unit),`${unit.installed}\n`);ports.write(unit.target,`${unit.desired}\n`);written.push(unit);}
  const reload=ports.run("systemctl",["daemon-reload"]);
  if(reload.status!==0){for(const unit of written)ports.write(unit.target,`${unit.installed}\n`);ports.run("systemctl",["daemon-reload"]);return done({action:"restored",reason:"systemctl daemon-reload failed"},false);}
  if(changed.some(unit=>unit.name==="replica"))ports.run("systemctl",["--no-block","restart",`semio-fleet-replica@${ports.user}.service`]);
  return done({action:"applied",services:changed.map(unit=>unit.label)},true);
 }
 const registered:Unit[]=[];
 for(const unit of units){
  const exported=ports.run("schtasks",["/Query","/TN",unit.label,"/XML"]),file=join(directory,`${unit.label}.xml`),before=join(directory,`${unit.label}.previous.xml`);
  const bytes=(text:string):Uint8Array=>Buffer.concat([Buffer.from([0xff,0xfe]),Buffer.from(text.replaceAll("\r\n","\n").replaceAll("\n","\r\n"),"utf16le")]);
  const hadBefore=exported.status===0&&exported.stdout.trim()!=="";
  if(hadBefore)ports.write(before,bytes(exported.stdout));
  ports.write(file,bytes(unit.desired));
  const created=ports.run("schtasks",["/Create","/TN",unit.label,"/XML",file,"/F"]);
  if(created.status!==0){
   const undone=hadBefore?ports.run("schtasks",["/Create","/TN",unit.label,"/XML",before,"/F"]):{status:1,stdout:""};
   for(const earlier of registered){const back=join(directory,`${earlier.label}.previous.xml`);ports.run("schtasks",["/Create","/TN",earlier.label,"/XML",back,"/F"]);}
   return done({action:"restored",reason:`schtasks /Create failed for ${unit.label}${undone.status===0?"":", the previous task could not be registered again"}`},false);
  }
  registered.push(unit);
 }
 return done({action:"applied",services:units.map(unit=>unit.label)},true);
}

/** 📬️ The outcome the detached macOS helper recorded, once per file: `restored` and `failed` are reported (the helper runs after the daemon that started it ended), `applied` needs no word. */
export function takeReloadOutcomeV1(ports:ServiceDefinitionPortsV1):ServiceDefinitionResultV1|undefined{
 const directory=join(ports.entryDirectory,"service-definition"),state=readState(ports,directory);
 try{
  const value=JSON.parse(ports.read(join(directory,"result.json"))??"") as {version:1;atMs:number;outcome:"applied"|"restored"|"failed"};requireShape("Result",value);
  if(value.atMs<=state.reportedMs)return undefined;
  writeState(ports,directory,{...state,reportedMs:value.atMs});
  if(value.outcome==="applied")return {action:"applied"};
  return {action:value.outcome==="restored"?"restored":"refused",reason:value.outcome==="restored"?"the new definition did not start":"neither definition started"};
 }catch{return undefined;}
}
