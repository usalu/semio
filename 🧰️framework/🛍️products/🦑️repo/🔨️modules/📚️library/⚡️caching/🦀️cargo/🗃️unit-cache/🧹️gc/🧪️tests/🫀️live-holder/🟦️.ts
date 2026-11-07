import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{lanePathsV1,listLanesV1,removeLaneV1}from"../../../🧵️lane/🟦️.ts";
import{tryClaimV1}from"../../../🔒️lock/🟦️.ts";
import{UNIT_CACHE_POLICY_V1}from"../../../🔏️key/🟦️.ts";
const layout=UNIT_CACHE_POLICY_V1.layout;
const lane=(root:string,id:string,pid:number,ageMs:number)=>{const paths=lanePathsV1(root,id);mkdirSync(paths.live,{recursive:true});const marker=join(paths.live,pid+"-abc123");writeFileSync(marker,JSON.stringify({pid}));const then=new Date(Date.now()-ageMs);utimesSync(marker,then,then);return paths;};
/** 🫀️ A lane is live while its holder pid is alive, however old the heartbeat; only a dead holder frees it; the reserved lane is never listed. */
test("garbage collection keeps a lane whose holder is alive with a stale heartbeat",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-gc-live-"));
 try{
  const hour=3_600_000,alive=lane(root,"palive1",process.pid,hour);
  const finished=spawnSync(process.execPath,["-e","0"]),dead=lane(root,"pdead01",finished.pid!,hour);
  mkdirSync(lanePathsV1(root,"d").base,{recursive:true});
  const rows=listLanesV1(root,Date.now());
  expect(rows.find(row=>row.id==="palive1")?.live).toBe(true);
  expect(rows.find(row=>row.id==="pdead01")?.live).toBe(false);
  expect(rows.some(row=>row.id==="d")).toBe(false);
  expect(removeLaneV1(root,"palive1","lane",Date.now())).toBe(false);
  expect(existsSync(alive.base)).toBe(true);
  expect(removeLaneV1(root,"pdead01","lane",Date.now())).toBe(true);
  expect(existsSync(dead.base)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
/** 🔒️ A claim of a live holder is not taken over however old its file is. */
test("a claim held by a live process is never taken over by age",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-claim-live-"));
 try{
  const locks=join(root,layout.locks),held=tryClaimV1(locks,"lane-x");
  expect(held).toBeDefined();
  const then=new Date(Date.now()-3_600_000);utimesSync(held!.path,then,then);
  expect(tryClaimV1(locks,"lane-x")).toBeUndefined();
  held!.release();
  const again=tryClaimV1(locks,"lane-x");expect(again).toBeDefined();again!.release();
 }finally{rmSync(root,{recursive:true,force:true});}
});
