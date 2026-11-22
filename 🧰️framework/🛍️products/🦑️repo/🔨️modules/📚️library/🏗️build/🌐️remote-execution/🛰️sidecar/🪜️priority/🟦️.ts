import {spawnSync} from "node:child_process";
import {getPriority,setPriority} from "node:os";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🏷️ The class names of the scheduling classes, as declared in `./🧬️schema/🔣️.json`. */
export type ServicePriorityNameV1=keyof typeof schema.definitions.Classes.const;
/** 🎚️ The operating system facts of one scheduling class: nice value, macOS process type, systemd CPU and input/output scheduling, Windows task priority. */
export type ServicePriorityClassV1=Readonly<{nice:number;darwin:Readonly<{processType:string;lowPriorityIo:boolean}>;linux:Readonly<{cpuSchedulingPolicy:string;ioSchedulingClass:string;ioSchedulingPriority?:number}>;windows:Readonly<{taskPriority:number}>}>;

/** 📜️ The declared classes, the class of each sidecar role and the class of each provisioned service. See `./🧬️schema/🔣️.json`. */
export const servicePriorityV1=Object.freeze({classes:schema.definitions.Classes.const as Readonly<Record<ServicePriorityNameV1,ServicePriorityClassV1>>,roles:schema.definitions.Roles.const as Readonly<Record<string,ServicePriorityNameV1>>,services:schema.definitions.Services.const as Readonly<Record<string,ServicePriorityNameV1>>});

/** 🧭️ The class of one sidecar role; a role that is not declared is refused instead of guessed. */
export function servicePriorityOfRoleV1(role:string):ServicePriorityNameV1{
 const name=servicePriorityV1.roles[role];
 if(name===undefined)throw Error(`Undeclared sidecar role: ${role}`);
 return name;
}

/** 🗺️ The class of one provisioned service (`replica`, `cache`, `relay`, `compact`); a service that is not declared is refused. */
export function servicePriorityOfServiceV1(service:string):ServicePriorityNameV1{
 const name=servicePriorityV1.services[service];
 if(name===undefined)throw Error(`Undeclared service: ${service}`);
 return name;
}

/** 🍎️ The LaunchAgent plist keys of a class: `ProcessType` always (without it launchd runs an agent at the scheduler priority 20, not at 31), `LowPriorityIO` for a background class, `Nice` for a class that lowers it (an agent of a user cannot raise it). See https://keith.github.io/xcode-man-pages/launchd.plist.5.html */
export function launchdPriorityKeysV1(name:ServicePriorityNameV1):readonly string[]{
 const {nice,darwin}=servicePriorityV1.classes[name];
 return [`<key>ProcessType</key><string>${darwin.processType}</string>`,...darwin.lowPriorityIo?["<key>LowPriorityIO</key><true/>"]:[],...nice>0?[`<key>Nice</key><integer>${nice}</integer>`]:[]];
}

/** 🐧️ The POSIX shell lines of a class for a launch script of the service (run by a system unit that grants `CAP_SYS_NICE` to the account, so the account owns the class and a change needs no privilege): `renice` of the script's own shell (every child inherits it), `chrt --idle` for the idle policy and `ionice`; each one is allowed to fail without stopping the service. See https://man7.org/linux/man-pages/man1/renice.1.html */
export function shellPriorityLinesV1(name:ServicePriorityNameV1):readonly string[]{
 const {nice,linux}=servicePriorityV1.classes[name];
 return [`renice -n ${nice} -p $$ >/dev/null 2>&1 || :`,...linux.cpuSchedulingPolicy==="idle"?["chrt --idle -p 0 $$ >/dev/null 2>&1 || :"]:[],`ionice -c ${linux.ioSchedulingClass==="idle"?3:2}${linux.ioSchedulingPriority===undefined?"":` -n ${linux.ioSchedulingPriority}`} -p $$ >/dev/null 2>&1 || :`];
}

/** 🪟️ The `Priority` element of a Windows Scheduled Task of a class (a task without it runs below normal, at 7). See https://learn.microsoft.com/windows/win32/taskschd/tasksettings-priority */
export function taskPriorityXmlV1(name:ServicePriorityNameV1):string{
 return `<Priority>${servicePriorityV1.classes[name].windows.taskPriority}</Priority>`;
}

/** 🔌️ The operating system calls of `applyServicePriorityV1`, replaceable by tests: the platform, the pid, reading and setting the nice value, and running `ionice` (its exit status, `null` when it is missing). */
export type ServicePriorityPortsV1=Readonly<{platform:string;pid:number;current():number;set(value:number):void;ionice(args:readonly string[]):number|null}>;

/** 🔩️ The real ports of this process. */
export function realServicePriorityPortsV1(pid:number=process.pid):ServicePriorityPortsV1{
 return {platform:process.platform,pid,current:()=>getPriority(pid),set:value=>{setPriority(pid,value);},ionice:args=>{try{return spawnSync("ionice",[...args],{stdio:"ignore",windowsHide:true}).status;}catch{return null;}}};
}

/** 📋️ What `applyServicePriorityV1` did: the class, the nice value now in force and the outcome (`applied`, `inherited` when the service that hosts the role decides, `refused` with the reason). */
export type ServicePriorityResultV1=Readonly<{role:string;name:ServicePriorityNameV1;nice:number;outcome:"applied"|"inherited"|"refused";io:string|undefined;reason?:string}>;

/** 🔔️ The log line of a result, English and German in one line. */
export function servicePriorityTextV1(result:ServicePriorityResultV1):string{
 const fill=(language:"en"|"de"):string=>{
  const texts=schema.definitions.Texts.const[language],outcome=texts[result.outcome].replace("{reason}",result.reason??""),io=result.io===undefined?"":texts.io.replace("{ioClass}",result.io);
  return texts.line.replace("{role}",result.role).replace("{class}",result.name).replace("{nice}",String(result.nice)).replace("{outcome}",outcome).replace("{io}",io);
 };
 return `${fill("en")} / ${fill("de")}`;
}

/** 🪜️ Puts this process into the class declared for its role, once, when the role starts: the nice value (Linux, macOS; on Windows libuv turns it into the priority class) and on Linux the input/output class through `ionice`. A process can only lower itself below its parent, so a class above the current value is tried on Linux and Windows (it succeeds where the service was provisioned at that class or the account may raise) and left to the service definition on macOS, where only the `ProcessType` of the LaunchAgent decides. Never throws: a refusal is a result, the role runs on at the priority it has. */
export function applyServicePriorityV1(role:string,ports:ServicePriorityPortsV1=realServicePriorityPortsV1()):ServicePriorityResultV1{
 const name=servicePriorityOfRoleV1(role),declared=servicePriorityV1.classes[name],before=(()=>{try{return ports.current();}catch{return declared.nice;}})();
 let outcome:ServicePriorityResultV1["outcome"]="applied",reason:string|undefined,nice=before;
 if(before!==declared.nice){
  if(declared.nice<before&&ports.platform==="darwin")outcome="inherited";
  else try{ports.set(declared.nice);nice=declared.nice;}catch(error){outcome="refused";reason=(error as Error).message;}
 }
 let io:string|undefined;
 if(ports.platform==="linux"){
  const {ioSchedulingClass,ioSchedulingPriority}=declared.linux,status=ports.ionice(ioSchedulingClass==="idle"?["-c","3","-p",String(ports.pid)]:["-c","2","-n",String(ioSchedulingPriority??4),"-p",String(ports.pid)]);
  if(status===0)io=ioSchedulingClass;
 }
 return Object.freeze({role,name,nice,outcome,io,...reason===undefined?{}:{reason}});
}
