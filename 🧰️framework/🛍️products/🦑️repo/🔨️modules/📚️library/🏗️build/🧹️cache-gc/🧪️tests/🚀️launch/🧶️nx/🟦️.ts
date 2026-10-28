import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheLedgerV1,cacheTextV1} from "../../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {nxToolingStoreBoundPolicyV1} from "../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts";
import {graphCollectorDecisionV1,launchNxToolingCollectorIfPressedV1,launchNxToolingCollectorV1,type GraphCollectorPortsV1} from "../../../🚀️launch/🟦️.ts";

const GIB=1024**3,ledger=cacheLedgerV1(),reclaim=ledger.areas.find(area=>area.id==="nx-tooling")!.reclaim!,policy=nxToolingStoreBoundPolicyV1();
let workspace="";
beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"cgl-nx-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

type Started=Array<{command:string;args:readonly string[];cwd:string;env:Readonly<Record<string,string>>;log:string}>;
function ports(state:{now:number;stamp:number|null;started:Started;fail?:boolean;reports?:string[];statuses?:string[]}):Partial<GraphCollectorPortsV1>{
 return{now:()=>state.now,readStamp:async()=>state.stamp,writeStamp:async()=>{if(state.fail)throw Error("disk full");state.stamp=state.now;},start:(command,args,options)=>{state.started.push({command,args:[...args],...options});},writeStatus:async(_path,text)=>{state.statuses?.push(text);},report:line=>{state.reports?.push(line);},environment:{PATH:"/bin",HOME:"/h",SEMIO_SCRIPT_PROCESS_INVOCATION:"{}"},execPath:"/bin/bun",script:"/repo/script.ts"};
}

test("the tools image declares a soft threshold between the reactive headroom and its target, and the collector interval is the bound policy's",()=>{
 expect(reclaim.softAvailableBytes).toBeGreaterThan(1610612736);expect(reclaim.targetAvailableBytes).toBeGreaterThan(reclaim.softAvailableBytes);
 expect(graphCollectorDecisionV1({availableBytes:reclaim.softAvailableBytes,softBytes:reclaim.softAvailableBytes,nowMs:1e9,lastLaunchMs:null,minimumIntervalMs:policy.intervalMs})).toBe("enough");
 expect(policy.intervalMs).toBeGreaterThanOrEqual(60000);
});

test("a run below the soft threshold starts one detached collector per interval for both Nx areas, above it nothing, and a failing start never fails the run",async()=>{
 const state={now:50_000_000,stamp:null as number|null,started:[] as Started,reports:[] as string[],statuses:[] as string[]};
 expect(await launchNxToolingCollectorV1(workspace,reclaim.softAvailableBytes+GIB,ports(state),ledger)).toBe("enough");expect(state.started).toEqual([]);
 expect(await launchNxToolingCollectorV1(workspace,2*GIB,ports(state),ledger)).toBe("launch");
 expect(state.started.length).toBe(1);expect(state.started[0]!.args).toEqual(["/repo/script.ts","repo","cache","gc","--area=nx-tooling,nx-tmp"]);expect(state.started[0]!.cwd).toBe(workspace);
 expect(state.started[0]!.env).toEqual({SEMIO_CACHE_BACKGROUND:"1",PATH:"/bin",HOME:"/h"});expect(state.started[0]!.log).toContain("nx-tooling-collector");
 state.now+=policy.intervalMs-1;expect(await launchNxToolingCollectorV1(workspace,0,ports(state),ledger)).toBe("throttled");expect(state.started.length).toBe(1);
 state.now+=1;expect(await launchNxToolingCollectorV1(workspace,0,ports(state),ledger)).toBe("launch");expect(state.started.length).toBe(2);
 const failing={now:90_000_000,stamp:null as number|null,started:[] as Started,fail:true,reports:[] as string[],statuses:[] as string[]};
 expect(await launchNxToolingCollectorV1(workspace,GIB,ports(failing),ledger)).toBe("failed");
 expect(failing.reports.join("\n")).toContain(cacheTextV1("en","nxTooling.launch.failed",{message:"disk full"}));expect(failing.reports.join("\n")).toContain(cacheTextV1("de","nxTooling.launch.failed",{message:"disk full"}));expect(failing.statuses.join("")).toContain("disk full");
});

test("the pressed-image entry reads the mounted free space, treats an unmounted image as free and never throws",async()=>{
 const state={now:70_000_000,stamp:null as number|null,started:[] as Started};
 expect(await launchNxToolingCollectorIfPressedV1(workspace,ports(state),()=>undefined)).toBe("unmounted");
 expect(await launchNxToolingCollectorIfPressedV1(workspace,ports(state),()=>{throw Error("statfs");})).toBe("unmounted");
 expect(await launchNxToolingCollectorIfPressedV1(workspace,ports(state),()=>10*GIB)).toBe("enough");
 expect(await launchNxToolingCollectorIfPressedV1(workspace,ports(state),()=>GIB)).toBe("launch");expect(state.started.length).toBe(1);
});
