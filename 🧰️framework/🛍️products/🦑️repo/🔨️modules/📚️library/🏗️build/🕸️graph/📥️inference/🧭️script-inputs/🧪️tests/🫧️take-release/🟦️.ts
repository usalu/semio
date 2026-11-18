import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";

/** 🫧️ A take pass over thousands of owners keeps the module graph, the source reads and the closure walks of one owner at a time, not of all: a real process that resolves the references of 600 owners one after the other stays under a ceiling, and the same process without the release (the control) passes it. */
const module=join(import.meta.dir,"..","..","🟨️.mjs");
const script=`
import{createScriptInputCache,releaseAfterTakeLookV1}from${JSON.stringify(module)};
const release=process.env.MODE==="release",scripts=createScriptInputCache({workspaceRoot:"/tmp/none",registry:null,lateClosures:true});
scripts.walkNodes??=new Map();let peak=0;const mb=n=>Math.round(n/1048576);
for(let owner=0;owner<(release?600:150);owner++){
 for(let node=0;node<1500;node++){const key=owner+"/"+node;scripts.walkNodes.set(key,{paths:Array.from({length:12},(_all,index)=>"🧰️framework/🛍️products/a/very/long/module/path/"+owner+"/"+node+"/"+index+".ts"),reaches:new Set(["x"+owner+"-"+node])});}
 for(let file=0;file<800;file++)scripts.files.set("file-"+owner+"-"+file,"source text of the file number "+file+" of owner "+owner+" "+"x".repeat(200));
 if(release)releaseAfterTakeLookV1(scripts,{shrink:false,gate:undefined});
 if(owner%10===0){Bun.gc(true);const m=process.memoryUsage();peak=Math.max(peak,m.heapUsed+m.external);}
}
Bun.gc(true);const m=process.memoryUsage();peak=Math.max(peak,m.heapUsed+m.external);
console.log(JSON.stringify({peak:mb(peak),nodes:scripts.walkNodes.size}));`;
const run=(mode:string)=>{const r=spawnSync(process.execPath,["-e",script],{encoding:"utf8",env:{...process.env,MODE:mode}});if(r.status!==0)throw new Error(r.stderr);return JSON.parse(r.stdout.trim()) as{peak:number;nodes:number};};

test("real process: with the release after every look the memory of 600 looks stays under the declared growth bound, without it 150 looks already pass 350 MB",()=>{
 const without=run("control"),withRelease=run("release");
 expect(without.nodes).toBe(150*1500);
 
 expect(without.peak).toBeGreaterThan(350);
 const growth=INFERENCE_LIMITS_V1.lookReleaseGrowthBytes/1048576;
 expect(growth,"the declared growth that makes a look release").toBe(128);
 expect(withRelease.peak,"growth bound of a look release plus the process itself and one owner").toBeLessThan(growth+96);
 expect(withRelease.nodes).toBeLessThan(32*1500+1);
},120000);

test("the take pass releases after every look: the look of an owner ends in the release, in the source of the reader",async()=>{
 const text=await Bun.file(join(import.meta.dir,"..","..","..","📦️project-nodes","🟨️.mjs")).text();
 const look=text.slice(text.indexOf("const look=job=>{"),text.indexOf("const resolveOwners=async"));
 expect(look).toContain("inflight.delete(job.name);releaseAfterTakeLookV1(scripts,");
});

test("the take pass of a machine that does not serve the fleet does not offer the records it finds: the look passes deferOffer, the observation skips the offer for it",async()=>{
 const nodes=await Bun.file(join(import.meta.dir,"..","..","..","📦️project-nodes","🟨️.mjs")).text(),observation=await Bun.file(join(import.meta.dir,"..","..","..","..","..","👁️observation","🟨️.mjs")).text();
 expect(nodes).toContain("deferOffer:away.take===true&&planStageLimitBytesV1()!==Infinity");
 expect(observation).toMatch(/if\(reused&&port&&!deferOffer\)await [^\n]*port\.offer\(info\)/u);
 expect(observation).toMatch(/observeInferenceComputationV1\(\{[^}]*prepare,fleet,deferOffer[,}][^\n]*\},async\(\)=>\{projectComputations/u);
});
