import {spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";
import {mkdirSync,renameSync,rmSync,statSync} from "node:fs";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

type Environment=Readonly<Record<string,string|undefined>>;

/** 🔤️ The variables a job-token client reads, in the order the `jobserver` crate (and so Cargo, rustc and `cc`) reads them: the first one that is SET decides, even when it names no pool. */
export const JOBSERVER_VARIABLES_V1:readonly string[]=Object.freeze(["CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"]);

/** 🪪️ The variable the POSIX stand-ins export: the descriptor pair they found OPEN before the runtime started (see {@link jobserverProbeShellV1}). */
export const JOBSERVER_VERIFIED_VARIABLE_V1="SEMIO_UNIT_CACHE_JOBSERVER_FDS";

/** 🧱️ The highest descriptor number a child can be handed: a stdio vector needs a placeholder for every number below it. */
export const JOBSERVER_MAXIMUM_DESCRIPTOR_V1=63;

/** 🎰️ The pool a process is told about: nothing; an anonymous pipe (POSIX, inherited descriptors); a named fifo (POSIX, GNU make 4.4 and the repository launcher); a named semaphore (Windows, Cargo and the launcher); or a pool this process cannot reach. */
export type JobserverEndpointV1=Readonly<{kind:"none"}|{kind:"pipe";read:number;write:number}|{kind:"fifo";path:string}|{kind:"semaphore";name:string}|{kind:"closed";variable:string;reason:JobserverClosedReasonV1}>;

/** 🚫️ Why a named pool is unreachable: `malformed` (neither `fifo:PATH` nor `R,W`), `disabled` (GNU make's negative descriptors), `not-inherited` (the descriptors are not open here), `out-of-range` (above {@link JOBSERVER_MAXIMUM_DESCRIPTOR_V1}), `missing-fifo` (no fifo at that path). */
export type JobserverClosedReasonV1="malformed"|"disabled"|"not-inherited"|"out-of-range"|"missing-fifo";

/** 🔎️ The `--jobserver-auth` (else `--jobserver-fds`) value of the first variable that is set: the LAST declaration wins and the value ends at the first space, exactly as the `jobserver` crate finds it. */
export function jobserverAuthV1(environment:Environment):Readonly<{variable:string;value:string|undefined}>|undefined{
 const variable=JOBSERVER_VARIABLES_V1.find(name=>environment[name]!==undefined);
 if(variable===undefined)return undefined;
 const text=environment[variable]!;
 for(const flag of["--jobserver-auth=","--jobserver-fds="]){const at=text.lastIndexOf(flag);if(at>=0)return{variable,value:text.slice(at+flag.length).split(" ")[0]!};}
 return{variable,value:undefined};
}

/** 🧭️ Classifies the pool the environment names for `platform`, with the same grammar as the `jobserver` crate: `fifo:PATH` or `R,W` (decimal integers) on POSIX, any non-empty name on Windows. Descriptors are only trusted when `verified` (the stand-in's census, never the runtime's own view: it opens descriptors at the lowest free numbers) lists them. */
export function jobserverEndpointV1(environment:Environment,platform:NodeJS.Platform=process.platform,isFifo:(path:string)=>boolean=isFifoV1):JobserverEndpointV1{
 const found=jobserverAuthV1(environment);
 if(found===undefined||found.value===undefined)return{kind:"none"};
 const {variable,value}=found,closed=(reason:JobserverClosedReasonV1):JobserverEndpointV1=>({kind:"closed",variable,reason});
 if(platform==="win32")return value===""?closed("malformed"):{kind:"semaphore",name:value};
 if(value.split(":")[0]==="fifo"){
  const path=value.includes(":")?value.slice(value.indexOf(":")+1):"";
  return path===""?closed("malformed"):isFifo(path)?{kind:"fifo",path}:closed("missing-fifo");
 }
 const comma=value.indexOf(",");
 if(comma<0)return closed("malformed");
 const [first,second]=[value.slice(0,comma),value.slice(comma+1)];
 if(!/^[+-]?\d+$/u.test(first)||!/^[+-]?\d+$/u.test(second))return closed("malformed");
 const read=Number(first),write=Number(second);
 if(read<0||write<0)return closed("disabled");
 if(read>JOBSERVER_MAXIMUM_DESCRIPTOR_V1||write>JOBSERVER_MAXIMUM_DESCRIPTOR_V1)return closed("out-of-range");
 const census=/^(\d+),(\d+)$/u.exec(environment[JOBSERVER_VERIFIED_VARIABLE_V1]??"");
 return census&&Number(census[1])===read&&Number(census[2])===write?{kind:"pipe",read,write}:closed("not-inherited");
}

/** 🗂️ Whether a path is a named pipe. */
export function isFifoV1(path:string):boolean{try{return statSync(path).isFIFO();}catch{return false;}}

/** 🕳️ The shared pool without tokens: a fifo in the cache's `bin` directory that nobody ever fills, so a client that is pointed at it finds a working pool and runs on its own implicit slot. `undefined` where no fifo can be made (Windows, a read-only cache): the caller then names no pool at all. */
export function emptyPoolV1(root:string|undefined,platform:NodeJS.Platform=process.platform):string|undefined{
 if(!root||platform==="win32")return undefined;
 const path=join(root,"bin","jobserver-empty");
 if(isFifoV1(path))return path;
 const temporary=path+"."+randomBytes(4).toString("hex");
 try{
  mkdirSync(dirname(path),{recursive:true});
  if(spawnSync("mkfifo",["-m","600",temporary],{stdio:"ignore"}).status!==0)return undefined;
  try{renameSync(temporary,path);}catch{rmSync(temporary,{force:true});}
  return isFifoV1(path)?path:undefined;
 }catch{rmSync(temporary,{force:true});return undefined;}
}

/** 🔒️ The environment of a command whose pool is unreachable: every client variable that is set loses its jobserver words and names the empty pool instead (or just `-j1`), and `NUM_JOBS` is 1. A client that cannot reach its pool would otherwise open a private one (`cc` as large as `NUM_JOBS`, rustc 32 slots) and run more jobs than were admitted. */
export function closedPoolEnvironmentV1(environment:Environment,empty:string|undefined):Record<string,string>{
 const patch:Record<string,string>={NUM_JOBS:"1"};
 for(const name of JOBSERVER_VARIABLES_V1){
  const value=environment[name];if(value===undefined)continue;
  const kept=value.split(" ").filter(word=>word!==""&&!/^(--jobserver-(auth|fds)=|-j\d*$)/u.test(word));
  patch[name]=[...kept,"-j1",...empty===undefined?[]:[`--jobserver-fds=fifo:${empty}`,`--jobserver-auth=fifo:${empty}`]].join(" ");
 }
 return patch;
}

/** 🧾️ What one child process gets: the descriptors to inherit under their own numbers and the environment changes. */
export type JobserverPlanV1=Readonly<{endpoint:JobserverEndpointV1;descriptors:readonly number[];environment:Readonly<Record<string,string>>}>;

/** 🗺️ Decides how a child shares the pool of its parent. A pipe pair that is open is handed on under the same numbers; a fifo, a semaphore or no pool needs nothing (the variables are inherited); a pool that cannot be reached is replaced by an empty one, so the child runs serially instead of inventing parallelism. Never consumes a token: the child works on the implicit slot of the process that started it, exactly like a recipe of GNU make. */
export function jobserverPlanV1(environment:Environment,options:Readonly<{platform?:NodeJS.Platform;isFifo?:(path:string)=>boolean;empty?:(root:string|undefined)=>string|undefined}>={}):JobserverPlanV1{
 const platform=options.platform??process.platform,endpoint=jobserverEndpointV1(environment,platform,options.isFifo);
 if(endpoint.kind==="pipe")return{endpoint,descriptors:Object.freeze([...new Set([endpoint.read,endpoint.write])]),environment:{}};
 if(endpoint.kind==="closed")return{endpoint,descriptors:[],environment:closedPoolEnvironmentV1(environment,(options.empty??(root=>emptyPoolV1(root,platform)))(environment.SEMIO_UNIT_CACHE_ROOT))};
 return{endpoint,descriptors:[],environment:{}};
}

/** 🧪️ The environment a child gets: the parent's without the stand-in's census, plus the plan's changes. */
export function jobserverChildEnvironmentV1(environment:Environment,plan:JobserverPlanV1):Record<string,string>{
 const child:Record<string,string>={};
 for(const [name,value] of Object.entries(environment))if(value!==undefined&&name!==JOBSERVER_VERIFIED_VARIABLE_V1)child[name]=value;
 return{...child,...plan.environment};
}

/** 🔌️ The stdio vector that hands the plan's descriptors to a child under the numbers they have here; the numbers between are closed placeholders. */
export function jobserverStdioV1(descriptors:readonly number[],capture:boolean):Array<"inherit"|"pipe"|"ignore"|number>{
 const stdio:Array<"inherit"|"pipe"|"ignore"|number>=["inherit",capture?"pipe":"inherit",capture?"pipe":"inherit"];
 for(let index=3;index<=Math.max(2,...descriptors);index++)stdio[index]=descriptors.includes(index)?index:"ignore";
 return stdio;
}

/** 💬️ The bilingual line that says a pool was unreachable and the command runs on one slot; empty when the pool is fine. */
export function jobserverClosedMessageV1(plan:JobserverPlanV1):string{
 if(plan.endpoint.kind!=="closed")return"";
 const {variable,reason}=plan.endpoint;
 return(["en","de"] as const).map(language=>schema.definitions.Messages.const.jobserverClosed[language].replace(/\{(\w+)\}/gu,(_,name:string)=>String({variable,reason}[name as "variable"|"reason"]??""))).join("\n");
}

/** 📜️ The lines every POSIX stand-in runs BEFORE its runtime starts (the runtime opens descriptors of its own at the lowest free numbers, so afterwards no descriptor can be told from Cargo's): they pick the variable like {@link jobserverAuthV1}, take the `R,W` pair and export {@link JOBSERVER_VERIFIED_VARIABLE_V1} only when both descriptors are open. A value inherited from a parent stand-in is dropped first. Dash-safe: only `${##}`, `${%%}`, `${+}`, `case` and subshell redirects. */
export function jobserverProbeShellV1():readonly string[]{
 return[
  `unset ${JOBSERVER_VERIFIED_VARIABLE_V1}`,
  'if [ "${CARGO_MAKEFLAGS+x}" ]; then jobs=$CARGO_MAKEFLAGS; elif [ "${MAKEFLAGS+x}" ]; then jobs=$MAKEFLAGS; else jobs=${MFLAGS-}; fi',
  'case "$jobs" in *--jobserver-auth=*) jobs="${jobs##*--jobserver-auth=}";; *--jobserver-fds=*) jobs="${jobs##*--jobserver-fds=}";; *) jobs=;; esac',
  'jobs="${jobs%% *}"',
  `case "$jobs" in ""|*[!0-9,]*|*,*,*|,*|*,) ;; *,*) if ( : <&"\${jobs%,*}" ) 2>/dev/null && ( : >&"\${jobs#*,}" ) 2>/dev/null; then ${JOBSERVER_VERIFIED_VARIABLE_V1}="$jobs"; export ${JOBSERVER_VERIFIED_VARIABLE_V1}; fi;; esac`,
 ];
}

/** 🐚️ One word, quoted for POSIX `sh`. */
export function shellQuoteV1(text:string):string{return"'"+text.replaceAll("'","'\\''")+"'";}
