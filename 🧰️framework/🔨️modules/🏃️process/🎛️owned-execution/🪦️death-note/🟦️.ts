import {readFile,readdir,stat} from "node:fs/promises";
import {homedir} from "node:os";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,texts=schema.definitions.Texts.const;

/** 📄️ The crash report the operating system wrote for a pid. */
export type DeathReportV1=Readonly<{path:string;exception:string}>;
/** 🪦️ What is known about a child that died from a signal. */
export type DeathFactsV1=Readonly<{label:string;pid:number|undefined;signal:string|null;status:number|null;stderrTail:string;lines?:number;report?:DeathReportV1}>;

const fill=(text:string,values:Readonly<Record<string,string|number>>):string=>text.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(values[name]??""));

/** 🗣️ The note of a dead child in English, then German (one line each): label, pid, signal, the last stderr lines or the statement that there were none, and the crash report when one was found. */
export function ownedDeathNoteV1(facts:DeathFactsV1):string{
 const lines=(facts.stderrTail.split("\n").map(line=>line.trim()).filter(Boolean)).slice(-(facts.lines??policy.tailLines));
 return (["en","de"] as const).map(language=>{
  const text=texts[language];
  return fill(text.died,{label:facts.label,pid:facts.pid??"?",signal:facts.signal??"none",status:facts.status??"none",tail:lines.length?lines.join(" | "):text.silent,report:facts.report===undefined?text.noReport:fill(text.report,facts.report)});
 }).join("\n");
}

/** 💀️ The cause of a death the platform reports without a signal: Windows has no signals, a process killed by the system (access violation, heap corruption, memory exhaustion) ends with an NTSTATUS exit code, 32 bit signed or unsigned; undefined for a plain exit. */
export function windowsDeathStatusV1(status:number|null):string|undefined{
 if(status===null)return undefined;
 const signed=status>2147483647?status-4294967296:status;
 return (policy.windowsStatuses as Readonly<Record<string,string>>)[String(signed)]??(signed>=-1073741824&&signed<=-805306369?"NTSTATUS 0x"+(signed+4294967296).toString(16).toUpperCase():undefined);
}

/** ⚰️ Whether a finished child died instead of ending: a signal on POSIX, a fatal NTSTATUS exit code on Windows; the name of the cause or undefined. */
export function deathCauseV1(signal:string|null,status:number|null,platform:NodeJS.Platform=process.platform):string|undefined{
 if(signal!==null)return signal;
 return platform==="win32"?windowsDeathStatusV1(status):undefined;
}

/** 🔎️ The crash report of exactly `pid` (macOS `~/Library/Logs/DiagnosticReports` `*.ips` by their pid field, Windows `%LOCALAPPDATA%\\CrashDumps` `<program>.<pid>.dmp`, files newer than `reportMaximumAgeMilliseconds`): the system writes it a few seconds after the death, so the search repeats every `pollMilliseconds` for up to `waitMilliseconds`; undefined where the system writes none. */
export async function findCrashReportV1(pid:number,options:Readonly<{directory?:string;waitMilliseconds?:number;pollMilliseconds?:number;now?:()=>number;platform?:NodeJS.Platform}>={}):Promise<DeathReportV1|undefined>{
 const platform=options.platform??process.platform,suffix=platform==="win32"?".dmp":".ips",directory=options.directory??(platform==="darwin"?join(homedir(),"Library","Logs","DiagnosticReports"):platform==="win32"?join(process.env.LOCALAPPDATA??join(homedir(),"AppData","Local"),policy.windowsDumpDirectory):undefined);
 if(directory===undefined)return undefined;
 const now=options.now??Date.now,until=now()+(options.waitMilliseconds??policy.reportWaitMilliseconds),poll=options.pollMilliseconds??policy.reportPollMilliseconds;
 for(;;){
  try{
   for(const name of await readdir(directory)){
    if(!name.endsWith(suffix))continue;
    const path=join(directory,name),info=await stat(path);
    if(!info.isFile()||now()-info.mtimeMs>policy.reportMaximumAgeMilliseconds||info.size>policy.reportMaximumBytes)continue;
    if(suffix===".dmp"){if(name.split(".").at(-2)===String(pid))return Object.freeze({path,exception:"minidump"});continue;}
    const body=(await readFile(path,"utf8")).split("\n").slice(1).join("\n");
    try{
     const row=JSON.parse(body) as {pid?:number;exception?:{type?:string;signal?:string};termination?:{namespace?:string;code?:number}};
     if(row.pid===pid)return Object.freeze({path,exception:[row.exception?.type,row.exception?.signal,row.termination?.namespace].filter(Boolean).join(" ")});
    }catch{}
   }
  }catch{}
  if(now()>=until)return undefined;
  await new Promise(resolve=>setTimeout(resolve,poll));
 }
}
