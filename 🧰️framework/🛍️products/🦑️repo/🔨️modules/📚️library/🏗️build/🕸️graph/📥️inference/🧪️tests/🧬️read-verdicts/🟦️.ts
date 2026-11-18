/** 🧬️ Contract: a shared closure entry and each layer of it is verified once per read, however many owners ask for it and however many processes of the read do: the first verifier writes a verdict file named by the digest into the directory of the validation round, every other look of that round reads it. A new round has a new directory, so an edit between rounds is seen at once. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
const B=fileURLToPath(new URL("../../../../",import.meta.url)).replace(/\/$/u,""),CHILD=join(import.meta.dir,"👷️child","🟦️.ts");process.env.R119_B=B;
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import(B+"/👁️observation/🟨️.mjs") as any,SI=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any,entries=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs") as any;
const roots:string[]=[];afterAll(()=>{for(const r of roots)rmSync(r,{recursive:true,force:true});});
const ENTRIES=8,DIRECTORIES=30,LAYERS=5;
const prepare=async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r119-"))),store=join(root,"store");roots.push(root);const rows:any[]=[];
 for(let at=0;at<DIRECTORIES;at++){const d=join(root,"d"+at);mkdirSync(d,{recursive:true});writeFileSync(join(d,"f"),String(at));rows.push((await observation.observeDetachedV1(()=>observation.observedFs.readdirSync(d))).rows[0]);}
 const writer=entries.createClosureStoreV1({directory:store}),recipes:any[]=[];
 for(let e=0;e<ENTRIES;e++){const recipe={fn:"closure-of",entries:["e"+e],extras:[],role:"test-discovery"};recipes.push(recipe);
  await writer.save(root+"\0"+SI.closureRecipeKeyV1(recipe),{recipe,value:[],rows:[],layers:Array.from({length:LAYERS},(_,l)=>Array.from({length:DIRECTORIES-2*l},(_,a)=>rows[(a*7+e*l)%DIRECTORIES]))});}
 return{root,store,recipes};
};
const child=(job:any)=>{const run=Bun.spawnSync([process.execPath,...(process.env.R119_PRELOAD?["--preload",process.env.R119_PRELOAD]:[]),CHILD],{env:{...process.env,R119_JOB:JSON.stringify(job)}});expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString().split("\n").find(l=>l.startsWith("CHILD "))!.slice(6));};
const layerWork=(c:any)=>c.layers.checked+c.layers.indexed;
const parent=async(root:string,store:string,recipes:any[])=>{
 SI.resetClosureEntriesV1();SI.bindClosureStoreV1(root,store,false);
 return observation.withObservationRoundV1(async()=>{await SI.primeSharedClosuresV1(root,{},{recipes});return SI.closureVerdictScopeV1?.();});
};
test("two processes of one round verify each shared entry once: the child reads the verdicts of the parent",async()=>{
 const{root,store,recipes}=await prepare();
 const before=SI.layerVerdictCountsV1(),dir=await parent(root,store,recipes),after=SI.layerVerdictCountsV1();
 expect(dir).toBeDefined();expect(SI.closureEntryCountsV1().checked).toBe(ENTRIES);
 expect(after.checked+after.indexed-before.checked-before.indexed).toBeGreaterThan(0);
 const c=child({root,store,dir,recipes});
 expect(c.entries.checked).toBe(0);expect(layerWork(c)).toBe(0);expect(c.probes).toBe(0);
 const lone=child({root,store,dir:"",recipes});
 expect(lone.entries.checked).toBe(ENTRIES);expect(layerWork(lone)).toBeGreaterThan(0);
});
test("a new round has its own directory and verifies again",async()=>{
 const{root,store,recipes}=await prepare();
 const dir=await parent(root,store,recipes);
 const next=await parent(root,store,recipes);
 expect(next).toBeDefined();expect(next).not.toBe(dir);
 expect(SI.closureEntryCountsV1().checked).toBe(ENTRIES);
});
test("without a round nothing is shared and no directory exists",()=>{
 expect(SI.closureVerdictScopeV1?.()).toBeUndefined();
});
