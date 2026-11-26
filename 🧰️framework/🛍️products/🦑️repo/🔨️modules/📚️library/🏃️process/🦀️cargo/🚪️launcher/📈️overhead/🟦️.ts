import {treePidsV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts";
import {readHostProcessRowsV1,type HostProcessRowV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧺️unmanaged/🟦️.ts";

/** 🧮️ Prices the launcher and its children except the rustc processes the broker already samples. The scheduler and any separate linker still count here. */
export function createCargoOverheadSamplerV1(root=process.pid,read:()=>Promise<readonly HostProcessRowV1[]>=readHostProcessRowsV1,launcherRss=process.memoryUsage.rss){
 let launcherBytes=0,childBytes=0;
 return async (schedulerPids:readonly number[]):Promise<number>=>{
  const rows=await Promise.resolve().then(read).catch(()=>[]);
  const byPid=new Map(rows.map(row=>[row.pid,row])),launcher=byPid.get(root);
  if(launcher){
   launcherBytes=launcher.kilobytes*1024;
   const schedulers=new Set(schedulerPids),members=treePidsV1(rows.map(row=>({pid:row.pid,parent:row.ppid})),root);
   childBytes=0;
   for(const pid of members){
    const row=byPid.get(pid)!;
    if(pid!==root&&!schedulers.has(row.ppid))childBytes+=row.kilobytes*1024;
   }
  }
  return Math.max(launcherRss(),launcherBytes)+childBytes;
 };
}
