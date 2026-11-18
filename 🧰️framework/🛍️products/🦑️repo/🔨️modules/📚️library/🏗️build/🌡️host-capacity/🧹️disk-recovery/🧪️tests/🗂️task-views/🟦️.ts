import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync,mkdirSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {digestTaskViewInputsV1,openTaskViewV1,taskViewsDirectoryV1} from "../../../../🏞️task-view/🟦️.ts";
import {DISK_WRITERS_V1} from "../../🟦️.ts";
import {collectMachineBoundsV1,realDiskWritersV1} from "../../🔌️real/🟦️.ts";

/** 🧪️ Contract: the pinned task roots are a registered writer of the disk recovery with the collector of the task view module, so that the gate and `repo cache gc` return them like every other derivable store. */
const roots:string[]=[],fresh=(tag:string)=>{const root=mkdtempSync(join(tmpdir(),tag+"-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

test("the task views are a registered host-relief writer with a unique priority, the repo collector command and both languages",()=>{
 const row=DISK_WRITERS_V1.find(item=>item.id==="task-views")!;
 expect(row).toBeDefined();
 expect([row.class,row.relief,row.collector,row.command]).toEqual(["derivable","host","age-prune","repo cache gc"]);
 expect(row.root).toBe("{workspace}/.cache/f/tv");
 expect(DISK_WRITERS_V1.filter(item=>item.priority===row.priority).length).toBe(1);
 expect(row.bound.en.length).toBeGreaterThan(10);expect(row.bound.de.length).toBeGreaterThan(10);
});

test("the real collector measures the views, a dry run removes nothing, the collection removes an expired view and keeps a held one, and `repo cache gc` names it",async()=>{
 const workspace=fresh("tv-ws"),tmp=fresh("tv-tmp"),live=fresh("tv-src"),now=Date.now();
 for(let index=0;index<6;index++)writeFileSync(join(live,"f"+index+".rs"),"fn f"+index+"(){}\n");
 const inputs=await digestTaskViewInputsV1(live,[0,1,2,3,4,5].map(index=>"f"+index+".rs")),directory=taskViewsDirectoryV1(workspace);
 expect(directory).toBe(join(workspace,".cache","f","tv"));
 for(const hash of["expired01","held0001"]){
  const view=await openTaskViewV1({source:live,directory,hash,inputs,pid:hash==="held0001"?process.pid:999999});
  if(view.kind!=="ready")throw Error("open");
  const old=new Date(now-48*3600000);utimesSync(join(directory,hash,"manifest.json"),old,old);
  if(hash==="expired01")await view.view.release();
 }
 const context={workspace,home:workspace,tmp,uid:501,environment:{},now:()=>now,alive:(pid:number)=>pid===process.pid},writer=realDiskWritersV1(context).find(item=>item.row.id==="task-views")!;
 expect(await writer.measure()).toBeGreaterThan(0);
 const dry=await collectMachineBoundsV1({...context,dryRun:true},"en");
 expect(dry.lines.some(line=>line.includes("task-views"))).toBe(true);
 expect(existsSync(join(directory,"expired01"))).toBe(true);
 const done=await collectMachineBoundsV1(context,"en");
 expect(done.freed["task-views"]).toBeGreaterThan(0);
 expect(readdirSync(directory).filter(name=>/^[0-9a-z]{8}$/u.test(name)).sort()).toEqual(["held0001"]);
 expect((await collectMachineBoundsV1(context,"de")).lines.some(line=>line.includes("task-views"))).toBe(true);
 mkdirSync(join(directory,"unrelated"),{recursive:true});
});
