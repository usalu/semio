import {execFile} from "node:child_process";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The numbers of the stack sample of a silent read.
 * @see ./🧬️schema/🔣️.json */
export const READY_SAMPLE_POLICY_V1=schema.definitions.Policy.const;
const P=READY_SAMPLE_POLICY_V1,TEXTS=schema.definitions.Texts.const;

/** 🧮️ One frame of a summary and the samples it holds. */
export type ReadySampleFrameV1=Readonly<{frame:string;count:number}>;
/** 🧾️ What a stack sample says about the main thread of a process: its samples, its own time by frame (largest first) and the end of its heaviest path (outermost first). */
export type ReadySampleSummaryV1=Readonly<{total:number;self:readonly ReadySampleFrameV1[];path:readonly ReadySampleFrameV1[]}>;

/** ⏰️ Whether the supervisor samples the slice worker of a reading stage now: its own report of this attempt (written at `atMs`, not before `startedMs`, by process `pid`) is not the last one of a finished read and is at least `silentMilliseconds` old. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readySliceSampleDueV1(report:unknown,nowMs:number,startedMs:number,policy:typeof P=P):boolean{
 const row=report as {atMs?:unknown;reason?:unknown;pid?:unknown}|null|undefined;
 return typeof row?.atMs==="number"&&row.atMs>=startedMs&&row.reason!=="end"&&Number.isSafeInteger(row.pid)&&(row.pid as number)>0&&nowMs-row.atMs>=policy.silentMilliseconds;
}

/** 🏷️ The name of one frame of a sample line: the address is dropped, JIT code is `JIT`, an unsymbolized frame of a binary is `<binary>+<offset>`, a named one keeps its name and image without the offset. A pure function. */
export function readySampleFrameV1(text:string,policy:typeof P=P):string{
 const bare=text.replace(/\s+\[0x[0-9a-f]+\]$/u,"");
 if(/^\?\?\?\s+\(in <unknown binary>\)/u.test(bare))return "JIT";
 const unnamed=/^\?\?\?\s+\(in (.+?)\)\s+load address 0x[0-9a-f]+ \+ (0x[0-9a-f]+)$/u.exec(bare);
 return (unnamed!==null?unnamed[1]+"+"+unnamed[2]:bare.replace(/\s+\+\s+\d+$/u,"")).replace(/\s+/gu," ").slice(0,policy.frameCharacters);
}

/** 🔬️ Summarizes the call graph of a `sample` report for the main thread (the thread named `com.apple.main-thread`, else the first): its samples, the own time of every frame (a node's count minus its children's) summed by frame name, largest first and then by name, the first `frames` of them, and the last `pathFrames` frames of the path that always follows the heaviest child (the first on a tie). Undefined when the text holds no call graph. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function readySampleSummaryV1(text:string,policy:typeof P=P):ReadySampleSummaryV1|undefined{
 const lines=text.split("\n"),start=lines.findIndex(line=>line.trim()==="Call graph:");if(start<0)return undefined;
 type Node={depth:number;count:number;frame:string;children:Node[];thread:string|undefined};
 const threads:Node[]=[],stack:Node[]=[];
 for(const line of lines.slice(start+1)){
  if(line.trim()==="")break;
  const found=/^([ +!:|]*)(\d+) +(.+)$/u.exec(line);if(found===null)continue;
  const depth=found[1].length,rest=found[3].trim(),node:Node={depth,count:Number(found[2]),frame:rest,children:[],thread:undefined};
  while(stack.length>0&&stack[stack.length-1].depth>=depth)stack.pop();
  if(stack.length===0){if(!rest.startsWith("Thread_"))continue;node.thread=rest;threads.push(node);}else{node.frame=readySampleFrameV1(rest,policy);stack[stack.length-1].children.push(node);}
  stack.push(node);
 }
 const main=threads.find(thread=>thread.thread!.includes("com.apple.main-thread"))??threads[0];if(main===undefined)return undefined;
 const own=new Map<string,number>(),visit=(node:Node):void=>{const self=node.count-node.children.reduce((sum,child)=>sum+child.count,0);if(self>0)own.set(node.frame,(own.get(node.frame)??0)+self);for(const child of node.children)visit(child);};
 for(const child of main.children)visit(child);
 const path:ReadySampleFrameV1[]=[];for(let node=main;node.children.length>0;){node=node.children.reduce((best,child)=>child.count>best.count?child:best);path.push({frame:node.frame,count:node.count});}
 const self=[...own].map(([frame,count])=>({frame,count})).sort((a,b)=>b.count-a.count||(a.frame<b.frame?-1:a.frame>b.frame?1:0)).slice(0,policy.frames);
 return Object.freeze({total:main.count,self:Object.freeze(self),path:Object.freeze(path.slice(-policy.pathFrames))});
}

/** 🗣️ The lines of a summary in both languages, English first: the head, the own time by frame, the end of the heaviest path. A pure function. */
export function readySampleLinesV1(summary:ReadySampleSummaryV1,fields:Readonly<{pid:number;silentSeconds:number;seconds:number}>):string[]{
 const fill=(text:string):string=>text.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(({...fields,total:summary.total} as Record<string,number>)[name]??""));
 return (["en","de"] as const).flatMap(language=>{const texts=TEXTS[language];return [fill(texts.head),texts.self,...summary.self.map(row=>"  "+row.count+" "+row.frame),texts.path,...summary.path.map(row=>"  "+row.count+" "+row.frame)];});
}

/** 🩺️ Samples process `pid` for `seconds` with the sampler of the operating system and answers the summary lines, or none where there is no sampler (not macOS), the sampler failed or its report holds no call graph. Never throws; bounded by `timeoutMilliseconds` and `outputBytes`.
 * @see ./🧬️schema/🔣️.json */
export async function readySliceSampleV1(input:Readonly<{pid:number;silentSeconds:number;platform?:string;run?:(program:string,args:readonly string[],options:Readonly<{timeout:number;maxBuffer:number}>)=>Promise<string>;policy?:typeof P}>):Promise<string[]>{
 const policy=input.policy??P;if((input.platform??process.platform)!=="darwin"||!Number.isSafeInteger(input.pid)||input.pid<=0)return [];
 const run=input.run??((program,args,options)=>new Promise<string>((resolve,reject)=>{execFile(program,[...args],{windowsHide:true,...options,encoding:"utf8"},(error,stdout)=>error?reject(error):resolve(stdout));}));
 try{const summary=readySampleSummaryV1(await run(policy.program,[String(input.pid),String(policy.seconds)],{timeout:policy.timeoutMilliseconds,maxBuffer:policy.outputBytes}),policy);return summary===undefined?[]:readySampleLinesV1(summary,{pid:input.pid,silentSeconds:input.silentSeconds,seconds:policy.seconds});}catch{return [];}
}
