/** 🪘️ Contract: a farm child resolves the closure references of a stored owner record and files the resolved project and the plan of its rows in the verdict directory of the read; the parent adopts the file and lists the rows from the closure store, with the same project and the same rows as its own resolution, and it opens no closure entry and resolves nothing itself. A file that is gone, torn or made from another record is never adopted. With four children and 200 owners the parent spends under 50 ms of CPU per owner and its resident set does not grow with the owners. @see ../../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{brotliCompressSync}from"node:zlib";
const B=fileURLToPath(new URL("../../../../../",import.meta.url)).replace(/\/$/u,""),CHILD=join(import.meta.dir,"👷️child","🟦️.ts");process.env.R121_B=B;
const inference=await import(B+"/🕸️graph/📥️inference/🟨️.mjs") as any;await inference.libraryBootstrap;
const SI=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs") as any,OB=await import(B+"/👁️observation/🟨️.mjs") as any,PN=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs") as any,AS=await import(B+"/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs") as any;
const entries=await import(B+"/🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs") as any,rowLogs=await import(B+"/🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs") as any;
const roots:string[]=[];afterAll(()=>{for(const r of roots)rmSync(r,{recursive:true,force:true});});
const ENTRIES=8,DIRECTORIES=30,LAYERS=5,sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
const prepare=async(owners:number)=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r121-"))),store=join(root,"store"),dir=join(root,"verdicts");roots.push(root);mkdirSync(dir,{recursive:true});mkdirSync(join(root,"np"),{recursive:true});const rows:any[]=[];
 for(let at=0;at<DIRECTORIES;at++){const d=join(root,"d"+at);mkdirSync(d,{recursive:true});writeFileSync(join(d,"f"),String(at));rows.push((await OB.observeDetachedV1(()=>OB.observedFs.readdirSync(d))).rows[0]);}
 const writer=entries.createClosureStoreV1({directory:store}),recipes:any[]=[];
 for(let e=0;e<ENTRIES;e++){const recipe={fn:"closure-of",entries:["e"+e],extras:[],role:"test-discovery"};recipes.push(recipe);
  await writer.save(root+"\0"+SI.closureRecipeKeyV1(recipe),{recipe,value:["{workspaceRoot}/f"+e,"{workspaceRoot}/g"+e],rows:[rows[e]],layers:Array.from({length:LAYERS},(_,l)=>Array.from({length:DIRECTORIES-2*l},(_,a)=>rows[(a*7+e*l)%DIRECTORIES]))});}
 const jobs=Array.from({length:owners},(_,i)=>{
  const project={root:"o"+i,name:"o"+i,closureRecipes:{a:recipes[i%ENTRIES],b:recipes[(i+3)%ENTRIES]},namedInputs:{src:[{closureRef:"a"},"x"+i]},targets:{build:{inputs:[{closureRef:"b"}]}}};
  const header=JSON.stringify({digest:"owner-"+i}),cachePath=join(root,"np","o"+i+".json");
  writeFileSync(cachePath,brotliCompressSync(Buffer.from(header+"\n"+JSON.stringify(project))));
  return{job:{cachePath},project,headerDigest:sha(header)};
 });
 return{root,store,dir,jobs};
};
const parentScripts=(root:string,store:string)=>{SI.resetClosureEntriesV1();SI.bindClosureStoreV1(root,store,false);return{...SI.createScriptInputCache(),lateClosures:true};};
const childFile=(root:string,store:string,dir:string,job:any,headerDigest:string)=>{
 const text=JSON.stringify({root,store,dir,job,headerDigest}),run=Bun.spawnSync([process.execPath,...(process.env.R121_PRELOAD?["--preload",process.env.R121_PRELOAD]:[]),CHILD],{env:{...process.env,R121_B:B},stdin:new TextEncoder().encode(text+"\n")});
 expect(run.exitCode,run.stderr.toString()).toBe(0);return JSON.parse(run.stdout.toString().trim().split("\n")[0]!);
};
const keys=(list:any)=>[...list.keys()].sort();

test("the file of a child gives the parent the same project and the same rows as its own resolution, without opening an entry",async()=>{
 const{root,store,dir,jobs}=await prepare(6),log=rowLogs.createRowLogV1({path:join(root,"rows.log")});
 for(const{job,project,headerDigest}of jobs){
  const said=childFile(root,store,dir,job,headerDigest);expect(said.refs).not.toBeNull();expect(said.refs.headerDigest).toBe(headerDigest);expect(said.refs.resolver).toBe(said.pid);expect(said.refs.size).toBeGreaterThan(0);
  const scripts=parentScripts(root,store),own=await AS.resolveClosureRefsV1(project,root,scripts,"hash",log),before=SI.closureEntryCountsV1();
  const adopted=PN.adoptOwnerRefsV1(said.refs,headerDigest,root,scripts,log),after=SI.closureEntryCountsV1();
  expect(adopted).toBeDefined();expect(adopted.project).toEqual(own.project);expect(keys(adopted.rows)).toEqual(keys(own.rows));expect(adopted.derived.length).toBe(own.derived.length);expect(after).toEqual(before);expect(adopted.resolver).toBe(said.pid);
  expect(own.project.namedInputs.src.length).toBeGreaterThan(1);
 }
});

test("a file that is gone, torn, made from another record or of another digest is never adopted",async()=>{
 const{root,store,dir,jobs}=await prepare(3),log=rowLogs.createRowLogV1({path:join(root,"rows.log")}),scripts=parentScripts(root,store);
 const refsOf=(at:number)=>childFile(root,store,dir,jobs[at]!.job,jobs[at]!.headerDigest).refs;
 const gone=refsOf(0);rmSync(gone.path);expect(PN.adoptOwnerRefsV1(gone,jobs[0]!.headerDigest,root,scripts,log)).toBeUndefined();
 const torn=refsOf(1);writeFileSync(torn.path,readFileSync(torn.path).subarray(0,20));expect(PN.adoptOwnerRefsV1(torn,jobs[1]!.headerDigest,root,scripts,log)).toBeUndefined();
 const other=refsOf(2);expect(PN.adoptOwnerRefsV1(other,"0".repeat(64),root,scripts,log)).toBeUndefined();
 const again=refsOf(2);expect(PN.adoptOwnerRefsV1({...again,digest:"0".repeat(64)},jobs[2]!.headerDigest,root,scripts,log)).toBeUndefined();
 expect(PN.adoptOwnerRefsV1(undefined,jobs[2]!.headerDigest,root,scripts,log)).toBeUndefined();
 expect(PN.adoptOwnerRefsV1(refsOf(2),jobs[2]!.headerDigest,root,scripts,undefined)).toBeUndefined();
 const changed=JSON.stringify({digest:"changed"});writeFileSync(jobs[2]!.job.cachePath,brotliCompressSync(Buffer.from(changed+"\n{}")));
 expect(childFile(root,store,dir,jobs[2]!.job,jobs[2]!.headerDigest).refs).toBeNull();
});

type Child={ask:(request:unknown)=>Promise<any>;stop:()=>void};
const spawnChild=():Child=>{
 const process_=Bun.spawn([process.execPath,...(process.env.R121_PRELOAD?["--preload",process.env.R121_PRELOAD]:[]),CHILD],{env:{...process.env,R121_B:B},stdin:"pipe",stdout:"pipe",stderr:"inherit"}),waiting:((line:string)=>void)[]=[],reader=process_.stdout.getReader(),decoder=new TextDecoder();let buffer="";
 (async()=>{for(;;){const{value,done}=await reader.read();if(done)return;buffer+=decoder.decode(value);for(let cut=buffer.indexOf("\n");cut>=0;cut=buffer.indexOf("\n")){const line=buffer.slice(0,cut);buffer=buffer.slice(cut+1);waiting.shift()?.(line);}}})();
 let tail:Promise<unknown>=Promise.resolve();
 const ask=(request:unknown)=>{const run=tail.then(()=>new Promise<any>(resolve=>{waiting.push(line=>resolve(JSON.parse(line)));process_.stdin.write(JSON.stringify(request)+"\n");process_.stdin.flush();}));tail=run.catch(()=>undefined);return run;};
 return{ask,stop:()=>{try{process_.stdin.end();}catch{}process_.kill();}};
};

test("four children resolve 200 owners: the parent spends under 50 ms of CPU per owner, resolves nothing and its resident set stays flat",async()=>{
 const OWNERS=200,{root,store,dir,jobs}=await prepare(OWNERS),log=rowLogs.createRowLogV1({path:join(root,"rows.log")}),children=Array.from({length:4},spawnChild),pids=new Set<number>();
 try{
  const scripts=parentScripts(root,store);let next=0;const answers:any[]=[];
  await Promise.all(children.map(async child=>{while(next<OWNERS){const at=next++,{job,headerDigest}=jobs[at]!,said=await child.ask({root,store,dir,job,headerDigest});pids.add(said.pid);answers[at]=said.refs;}}));
  expect(pids.size).toBeGreaterThan(1);
  Bun.gc(true);const rssBefore=process.memoryUsage().rss,entriesBefore=SI.closureEntryCountsV1(),cpu=process.cpuUsage();
  for(let at=0;at<OWNERS;at++){
   const adopted=PN.adoptOwnerRefsV1(answers[at],jobs[at]!.headerDigest,root,scripts,log);expect(adopted).toBeDefined();expect(adopted.project.root).toBe("o"+at);expect(adopted.rows.length).toBeGreaterThan(0);
  }
  const used=process.cpuUsage(cpu),perOwner=(used.user+used.system)/1000/OWNERS;Bun.gc(true);const growth=(process.memoryUsage().rss-rssBefore)/1048576;
  expect(perOwner).toBeLessThan(50);expect(growth).toBeLessThan(200);expect(SI.closureEntryCountsV1()).toEqual(entriesBefore);
  console.log("[R121] parent adopt CPU per owner",perOwner.toFixed(2),"ms, rss growth",growth.toFixed(1),"MB over",OWNERS,"owners, children",pids.size);
 }finally{for(const child of children)child.stop();}
},180000);

test("the parent asks for the file only from a child that verified the record, adopts it only for the header it read, and counts what it resolves itself",async()=>{
 const text=await Bun.file(join(import.meta.dir,"..","..","🟨️.mjs")).text(),start=text.indexOf("let offered,seenHeader"),asked=text.slice(start,text.indexOf("\n",start));
 expect(asked).toContain("refs:late&&options.rowLog!==undefined");expect(asked).toContain("reply.headerDigest===headerDigest");
 expect(text).toContain('said.verdict === "holds"');expect(text).toContain("ownerRefsCountsV1.resolved++");expect(text).toContain("ownerRefsCountsV1.adopted++");
});
