import {execFile} from "node:child_process";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const RULES=schema.definitions.Unmanaged.const;

/** 🧾️ One process of the machine: id, parent, resident kilobytes and executable name. */
export type HostProcessRowV1=Readonly<{pid:number;ppid:number;kilobytes:number;name:string}>;
/** 📊️ The processes of one class that run beside the gate and the memory they hold. */
export type HostUnmanagedClassV1=Readonly<{name:string;processes:number;megabytes:number}>;

/** 🔎️ Parses `pid ppid rss name` lines. */
export function parseHostProcessRowsV1(text:string):readonly HostProcessRowV1[]{
 return text.split("\n").flatMap(line=>{const match=/^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.+?)\s*$/u.exec(line);return match?[{pid:Number(match[1]),ppid:Number(match[2]),kilobytes:Number(match[3]),name:match[4]!.split("/").pop()!}]:[];});
}

/** 🧺️ Counts every process that is not part of a granted process tree (the roots are the pids of the live grants) into the authored classes, first match wins; the largest class comes first. A pure function over the process table. */
export function censusUnmanagedHostV1(rows:readonly HostProcessRowV1[],grantRoots:readonly number[]):readonly HostUnmanagedClassV1[]{
 const children=new Map<number,number[]>();
 for(const row of rows){const list=children.get(row.ppid);if(list)list.push(row.pid);else children.set(row.ppid,[row.pid]);}
 const managed=new Set<number>(),stack=[...grantRoots];
 while(stack.length>0){const pid=stack.pop()!;if(managed.has(pid))continue;managed.add(pid);stack.push(...children.get(pid)??[]);}
 const patterns=RULES.classes.map(row=>({name:row.name,pattern:new RegExp(row.pattern,"u")})),totals=new Map<string,{processes:number;kilobytes:number}>();
 for(const row of rows){
  if(managed.has(row.pid))continue;
  const name=patterns.find(candidate=>candidate.pattern.test(row.name))!.name,total=totals.get(name)??{processes:0,kilobytes:0};
  total.processes++;total.kilobytes+=row.kilobytes;totals.set(name,total);
 }
 return Object.freeze([...totals].map(([name,total])=>Object.freeze({name,processes:total.processes,megabytes:Math.ceil(total.kilobytes/1024)})).sort((left,right)=>right.megabytes-left.megabytes||(left.name<right.name?-1:1)));
}

/** 🖥️ Reads the process table of this machine with the command of the schema for its platform. */
export function readHostProcessRowsV1(platform:NodeJS.Platform=process.platform):Promise<readonly HostProcessRowV1[]>{
 const row=platform==="win32"?RULES.table.win32:RULES.table.unix;
 return new Promise(done=>execFile(row.command,[...row.arguments],{maxBuffer:64*1048576,windowsHide:true},(error,stdout)=>done(error?[]:parseHostProcessRowsV1(stdout))));
}
