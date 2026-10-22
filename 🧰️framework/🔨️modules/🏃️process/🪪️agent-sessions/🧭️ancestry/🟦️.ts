import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";

/** 🧭️ One ancestor process as the session resolver sees it: id, start identity (distinguishes a recycled id) and command line. */
export type AncestorV1=Readonly<{pid:number;started:string;command:string}>;
/** 🔌️ Reads the parent chain of the current process; replaceable so resolution is testable on any platform. */
export type AncestryPortV1=()=>readonly AncestorV1[];

const depth=16;

/** 🧬️ Parses a Linux `/proc/<pid>/stat` line into the parent and the start tick; the command name sits in parentheses and may itself contain spaces and parentheses, so the fields are read after the last closing parenthesis. */
export function parseProcStatV1(text:string):Readonly<{parent:number;started:string}>|undefined{
 const close=text.lastIndexOf(")");if(close<0)return undefined;
 const fields=text.slice(close+2).trim().split(/\s+/u),parent=Number(fields[1]),started=fields[19];
 return Number.isSafeInteger(parent)&&started!==undefined&&/^[0-9]+$/u.test(started)?Object.freeze({parent,started:"tick:"+started}):undefined;
}

/** 🐧️ Walks the parent chain through `/proc` with an injected reader: no external tool, no spawn, the start tick distinguishes a recycled pid. */
export function linuxAncestryV1(read:(path:string)=>string|null,first:number=process.ppid):readonly AncestorV1[]{
 const rows:AncestorV1[]=[];let pid=first;
 for(let step=0;step<depth&&pid>1;step++){
  const stat=read("/proc/"+pid+"/stat"),parsed=stat===null?undefined:parseProcStatV1(stat);if(!parsed)break;
  const line=(read("/proc/"+pid+"/cmdline")??"").replace(/\0+$/u,"").split("\0").join(" ");
  rows.push({pid,started:parsed.started,command:line||(read("/proc/"+pid+"/comm")??"").trim()});pid=parsed.parent;
 }
 return rows;
}

/** 🍎️ The `ps` invocation that lists every process once: id, parent id, start time, command. */
export const POSIX_PROCESS_LISTING_V1=Object.freeze(["-axo","pid=,ppid=,lstart=,command="]);

/** 🍎️ Walks the parent chain of one POSIX `ps` listing; the start time column is fixed at 24 characters. */
export function psAncestryV1(listing:string,first:number=process.ppid):readonly AncestorV1[]{
 const table=new Map<number,Readonly<{ppid:number;started:string;command:string}>>();
 for(const line of listing.split("\n")){const match=/^\s*(\d+)\s+(\d+)\s+(.{24})\s+(.*)$/u.exec(line);if(match)table.set(Number(match[1]),{ppid:Number(match[2]),started:match[3]!,command:match[4]!.trim()});}
 const rows:AncestorV1[]=[],seen=new Set<number>();let pid=first;
 for(let step=0;step<depth&&pid>1&&!seen.has(pid);step++){const row=table.get(pid);if(!row)break;seen.add(pid);rows.push({pid,started:row.started,command:row.command});pid=row.ppid;}
 return rows;
}

/** 🪟️ The PowerShell program that lists every process once: id, parent, UTC start as ISO text, and the command line followed by the image name, so a client matches by either. */
export const WINDOWS_PROCESS_LISTING_V1="Get-CimInstance Win32_Process | ForEach-Object { [pscustomobject]@{ pid=[int]$_.ProcessId; ppid=[int]$_.ParentProcessId; started=$_.CreationDate.ToUniversalTime().ToString('o'); command=([string]$_.CommandLine + ' ' + [string]$_.Name).Trim() } } | ConvertTo-Json -Compress";

/** 🪟️ Walks the parent chain of a Windows process listing; a parent that started after its child is a recycled id and ends the chain. */
export function windowsAncestryV1(listing:string,first:number=process.ppid):readonly AncestorV1[]{
 let table:unknown;try{table=JSON.parse(listing);}catch{return[];}
 const rows=(Array.isArray(table)?table:[table]).filter((row):row is {pid:number;ppid:number;started:string;command:string}=>!!row&&typeof row==="object"&&Number.isSafeInteger((row as {pid?:unknown}).pid)&&Number.isSafeInteger((row as {ppid?:unknown}).ppid)&&typeof (row as {started?:unknown}).started==="string"&&typeof (row as {command?:unknown}).command==="string");
 const byPid=new Map(rows.map(row=>[row.pid,row] as const)),found:AncestorV1[]=[];let pid=first,child:string|undefined;
 for(let step=0;step<depth&&pid>4;step++){
  const row=byPid.get(pid);if(!row||child!==undefined&&row.started>child)break;
  found.push({pid,started:row.started,command:row.command});child=row.started;pid=row.ppid;
 }
 return found;
}

/** 🪪️ Reads the parent chain of the current process: `/proc` on Linux, one `ps` listing on macOS and other POSIX hosts, one PowerShell CIM listing on Windows; a platform or a read that fails reports no ancestry. */
export function systemAncestryV1(platform:NodeJS.Platform=process.platform):readonly AncestorV1[]{
 if(platform==="linux")return linuxAncestryV1(path=>{try{return readFileSync(path,"utf8");}catch{return null;}});
 if(platform==="win32"){
  const root=process.env.SystemRoot,program=root?join(root,"System32","WindowsPowerShell","v1.0","powershell.exe"):"powershell.exe";
  const result=spawnSync(program,["-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_LISTING_V1],{encoding:"utf8",windowsHide:true,timeout:20000,maxBuffer:33554432});
  return result.status===0?windowsAncestryV1(result.stdout):[];
 }
 const result=spawnSync("ps",[...POSIX_PROCESS_LISTING_V1],{encoding:"utf8",timeout:20000,maxBuffer:33554432});
 return result.status===0?psAncestryV1(result.stdout):[];
}
