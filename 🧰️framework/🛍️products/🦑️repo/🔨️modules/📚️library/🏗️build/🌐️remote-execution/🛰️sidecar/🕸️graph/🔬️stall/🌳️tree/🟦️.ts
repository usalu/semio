import type{WantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";

/** 🌳️ One process of the table: `ps -axo pid=,ppid=,etime=,pcpu=,rss=,state=,command=`. */
export type ReadProcessRowV1=Readonly<{pid:number;parentPid:number;elapsedSeconds:number;cpu:number;rssMegabytes:number;state:string;command:string}>;

/** 🌳️ What the tree of a read does, summed: running, idle (no CPU for a minute) and paging (uninterruptible) processes, resident and physical memory, the largest process. */
export type ReadTreeSummaryV1=Readonly<{processes:number;running:number;idle:number;paging:number;rssMegabytes:number;footprintMegabytes:number|undefined;largest:Readonly<{pid:number;megabytes:number}>|undefined}>;

const ROW=/^\s*(\d+)\s+(\d+)\s+(?:(?:(\d+)-)?(?:(\d+):)?)(\d+):(\d+)\s+([\d.]+)\s+(\d+)\s+(\S+)\s+(.*)$/u;
const IDLE_SECONDS=60;

/** 🌳️ The rows of the table text; lines that are not rows are ignored. */
export function parseProcessTableV1(text:string):readonly ReadProcessRowV1[]{
 const rows:ReadProcessRowV1[]=[];
 for(const line of text.split("\n")){
  const found=ROW.exec(line);if(found===null)continue;
  rows.push({pid:Number(found[1]),parentPid:Number(found[2]),elapsedSeconds:Number(found[3]??0)*86400+Number(found[4]??0)*3600+Number(found[5])*60+Number(found[6]),cpu:Number(found[7]),rssMegabytes:Math.round(Number(found[8])/1024),state:found[9]!,command:found[10]!});
 }
 return rows;
}

/** 🌳️ The root and every descendant of it. */
export function readTreeRowsV1(rows:readonly ReadProcessRowV1[],rootPid:number):readonly ReadProcessRowV1[]{
 const children=new Map<number,ReadProcessRowV1[]>();
 for(const row of rows){const list=children.get(row.parentPid);if(list)list.push(row);else children.set(row.parentPid,[row]);}
 const root=rows.find(row=>row.pid===rootPid);if(root===undefined)return[];
 const found=[root],seen=new Set([rootPid]);
 for(let at=0;at<found.length;at++)for(const child of children.get(found[at]!.pid)??[])if(!seen.has(child.pid)){seen.add(child.pid);found.push(child);}
 return found;
}

/** 🌳️ Sums the tree; `footprints` (megabytes per pid, where the system can say) beat the resident size, which leaves out what the system compressed or swapped. */
export function summarizeReadTreeV1(rows:readonly ReadProcessRowV1[],footprints:ReadonlyMap<number,number>|undefined):ReadTreeSummaryV1{
 let running=0,idle=0,paging=0,rss=0,footprint=0,measured=false,largest:{pid:number;megabytes:number}|undefined;
 for(const row of rows){
  rss+=row.rssMegabytes;
  if(row.state.startsWith("U"))paging++;
  else if(row.state.startsWith("R")||row.cpu>=5)running++;
  else if(row.cpu<0.5&&row.elapsedSeconds>=IDLE_SECONDS)idle++;
  const own=footprints?.get(row.pid);
  if(own!==undefined){measured=true;footprint+=own;}
  const size=own??row.rssMegabytes;
  if(largest===undefined||size>largest.megabytes)largest={pid:row.pid,megabytes:size};
 }
 return{processes:rows.length,running,idle,paging,rssMegabytes:rss,footprintMegabytes:measured?footprint:undefined,largest};
}

/** 🌳️ The physical footprints of the output of the system tool `footprint`, in megabytes per pid. */
export function parseFootprintsV1(text:string):ReadonlyMap<number,number>{
 const found=new Map<number,number>();
 for(const line of text.split("\n")){
  const hit=/\[(\d+)\].*Footprint:\s*([\d.]+)\s*([KMG]B)/u.exec(line);if(hit===null)continue;
  const scale={KB:1/1024,MB:1,GB:1024}[hit[3] as"KB"|"MB"|"GB"];
  found.set(Number(hit[1]),Math.round(Number(hit[2])*scale));
 }
 return found;
}

const capture=async(command:readonly string[],milliseconds:number):Promise<string>=>{
 try{
  const child=Bun.spawn([...command],{windowsHide:true,stdin:"ignore",stdout:"pipe",stderr:"ignore"}),timer=setTimeout(()=>{try{child.kill("SIGKILL");}catch{}},milliseconds);
  try{return await new Response(child.stdout).text();}finally{clearTimeout(timer);}
 }catch{return"";}
};

/** 🌳️ The process table of the machine (nothing where the system has no `ps`). */
export async function readProcessTableV1(platform:string=process.platform):Promise<readonly ReadProcessRowV1[]>{
 if(platform==="win32")return[];
 return parseProcessTableV1(await capture(["ps","-axo","pid=,ppid=,etime=,pcpu=,rss=,state=,command="],10_000));
}

/** 🌳️ The physical footprints of `pids` where the system says them (macOS `footprint`); an empty map elsewhere. */
export async function readFootprintsV1(pids:readonly number[],platform:string=process.platform):Promise<ReadonlyMap<number,number>>{
 if(platform!=="darwin"||pids.length===0)return new Map();
 return parseFootprintsV1(await capture(["footprint",...pids.flatMap(pid=>["-p",String(pid)])],20_000));
}

/** 🌳️ The summary of the tree of `rootPid` now. */
export async function measureReadTreeV1(rootPid:number):Promise<ReadTreeSummaryV1>{
 const rows=readTreeRowsV1(await readProcessTableV1(),rootPid);
 return summarizeReadTreeV1(rows,await readFootprintsV1(rows.map(row=>row.pid)));
}

/** 🚨️ Whether a read has overrun its memory: its tree holds `memoryOverrunFactor` times its grant or more, the host is under pressure and the read has been silent for `memoryOverrunMilliseconds`. Such a read pages against the swap and waits for nothing; every minute it goes on slows the machine for all others. */
export function memoryOverrunV1(input:Readonly<{footprintMegabytes:number|undefined;grantMegabytes:number|undefined;pressure:string;silentMilliseconds:number;policy:Pick<WantedServerPolicyV1,"memoryOverrunFactor"|"memoryOverrunMilliseconds">}>):boolean{
 if(input.footprintMegabytes===undefined||input.grantMegabytes===undefined||input.grantMegabytes<=0)return false;
 return input.pressure!=="normal"&&input.silentMilliseconds>=input.policy.memoryOverrunMilliseconds&&input.footprintMegabytes>=input.grantMegabytes*input.policy.memoryOverrunFactor;
}
