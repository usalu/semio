import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),body=(name:string)=>{const start=source.indexOf("async function "+name+"(");if(start<0)throw Error("Actual function required: "+name);const end=source.indexOf("\n/**",start);return source.slice(start,end<0?undefined:end);};

test("a second synchronization with nothing changed reuses the recorded ownership index in the preservation pass as in every other ownership walk",()=>{
 for(const name of["retireJavascriptDependencyPreservationsInStoreV1"]){
  const text=body(name),walk=text.indexOf("withCurrentBunWorkspaceOwnershipV1(workspace,{kind:\"all\"}"),seed=text.indexOf("seedBunWorkspaceOwnershipIndexV1(workspace,index)");
  expect(walk,name).toBeGreaterThan(0);expect(seed,name).toBeGreaterThan(0);expect(seed,name).toBeLessThan(walk);
 }
});

test("a synchronization whose recorded proof is current ends before the exclusive bindings gate, the preservation pass and the collection",()=>{
 const producer=body("runJavascriptDependencyStoreProducerV1"),driver=body("runJavascriptDependencyStoreGenerationsV1");
 const current=producer.indexOf('if(gate.status==="current")'),discovery=producer.indexOf("withPublishedBunWorkspaceOwnershipV1(workspace"),gate=producer.indexOf("await acquireJavascriptDependencyBindingsGateV1(frame)");
 expect(current).toBeGreaterThan(0);expect(discovery).toBeGreaterThan(current);expect(gate).toBeGreaterThan(discovery);
 expect(driver).toContain('if(result.state==="published")await retireJavascriptDependencyPreservationsInStoreV1(');
 expect(driver).toContain('if(produced.state==="published")await collectJavascriptDependencyGenerationsV1(');
});


test("pressure reclamation keeps sealed, live and pinned generations before exact physical readmission",async()=>{
 const ts=await import("typescript"),fs=await import("node:fs"),path=await import("node:path"),{Database}=await import("bun:sqlite"),{concurrentAdmissionV1,concurrentCollectionV1,concurrentEventV1,concurrentProtocolV1}=await import("../../../🚦️concurrent/🟦️.ts"),{concurrentGenerationReferencedV1}=await import("../../../🚦️concurrent/💾️storage/🟦️.ts"),{selectObsoleteJavascriptDependencyGenerationsV1}=await import("../../📥️read-context/🟦️.ts"),{javascriptInstallSchemaV1:installSchema}=await import("../../🧱️schemas/🟦️.ts"),{default:cases}=await import("./🔣️.json");
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned pressure test artifact directory required");
 const files=["reclaimJavascriptDependencyPreservationsUnderPressureV1","collectJavascriptDependencyGenerationsV1"];
 const tree=ts.createSourceFile("actual-dependencies.ts",source,ts.ScriptTarget.Latest,true);
 const selected=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&files.includes(node.name?.text??""));
 expect(selected).toHaveLength(files.length);
 const compiled=ts.transpileModule(selected.map(node=>node.getText(tree)).join("\n").replace(/import\("([.][^"]+)"\)/gu,(_all,specifier:string)=>"import("+JSON.stringify(new URL(specifier,new URL("../../🟦️.ts",import.meta.url)).href)+")")+";return [reclaimJavascriptDependencyPreservationsUnderPressureV1,collectJavascriptDependencyGenerationsV1];",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const database=new Database(":memory:");
 try{for(const row of cases){
  const root=fs.mkdtempSync(path.join(artifacts,"reclaim-")),workspace=path.join(root,"s"),store={root:path.join(root,"d")},container=path.join(store.root,"j"),endpoint=path.join(workspace,"member","node_modules"),frame={gate:false},stop=new AbortController(),control={signal:stop.signal,publish:async()=>{},yieldContinuation:async()=>{}},now=Date.now()-604800000,old=new Date(now),byRole=Object.fromEntries(row.generations.map(generation=>[generation.role,generation.name])),events:string[]=[],entry=(generation:(typeof row.generations)[number])=>path.join(container,(generation.tombstone?".x":"")+generation.name);
  try{
   for(const generation of row.generations){const directory=entry(generation);fs.mkdirSync(path.join(directory,"node_modules"),{recursive:true});fs.writeFileSync(path.join(directory,"bindings.json"),JSON.stringify({bindings:generation.role==="current"||row.normalMaintenance&&generation.role==="live"?[{sourceDirectory:endpoint}]:[]}));fs.utimesSync(directory,old,old);}
   fs.mkdirSync(path.dirname(endpoint),{recursive:true});
   const target=path.join(container,byRole.live!,"node_modules"),alias=path.join(root,"alias");
   fs.symlinkSync(target,alias,process.platform==="win32"?"junction":"dir");
   fs.symlinkSync(row.aliased?alias:target,endpoint,process.platform==="win32"?"junction":"dir");
   const firstTombstone=fs.readdirSync(container).find(name=>name.startsWith(".x"));
   const expectedRemoved=firstTombstone?row.generations.filter(value=>path.basename(entry(value))===firstTombstone).map(value=>value.role):row.removed;
   const pointers=[{scope:".",generation:byRole.current!.padEnd(64,"0"),bytes:row.generationBytes,retired:[]}];
   let available=row.startFree,pins=0;
   const free=()=>available+row.generations.filter(generation=>!fs.existsSync(entry(generation))).reduce((sum,generation)=>sum+generation.bytes,0);
   const strictFailure=Error("Exact current JavaScript dependency binding required"),cancelled=Error("Cancelled while observing pressure bindings");
   const check=()=>control.signal.throwIfAborted();
   const ports={join:path.join,relative:path.relative,resolve:path.resolve,dirname:path.dirname,sep:path.sep,process,lstatSync:fs.lstatSync,realpathSync:fs.realpathSync,readProducerBindingLinkV1:fs.readlinkSync,readdirSync:fs.readdirSync,renameSync:fs.renameSync,performance,installSchema,concurrentProtocolV1,selectObsoleteJavascriptDependencyGenerationsV1,concurrentAdmissionV1,concurrentCollectionV1,concurrentEventV1,concurrentGenerationReferencedV1,readSingletonManifestV1:fs.readFileSync,
    concurrentUnreadyCapsulePatternV1:/^\.p[a-f0-9]{8}$/u,concurrentUnreadyCapsuleRetirableV1:()=>false,readConcurrentPointersV1:()=>pointers,concurrentFreeBytesV1:async()=>{expect(pins).toBe(0);events.push("measure");return free();},concurrentFreeFractionV1:async()=>0.01,sweepConcurrentTemporariesV1:()=>{events.push("sweep");return 0;},writeOwnedCommandOutputV1:async()=>{},repositoryLeaseDirectoryV1:()=>root,requireRepositoryCacheStoreV1:check,requireRepositoryCacheStoreReadCustodyV1:check,checkResourceLeaseControl:check,requireJavascriptDependencyStoreFrameV1:()=>{check();return frame;},javascriptDependencyConcurrencyReportV1:()=>async(event:{event:string})=>{events.push(event.event);},
    withJavascriptDependencyBindingsGateReleaseV1:async(_frame:unknown,body:()=>Promise<void>)=>{try{await body();}finally{frame.gate=false;}},acquireJavascriptDependencyBindingsGateV1:async()=>{frame.gate=true;available=row.freeAfterGate;events.push("gate");},
    seedBunWorkspaceOwnershipIndexV1:()=>{},withCurrentBunWorkspaceOwnershipV1:async(actual:string,selection:unknown,_progress:unknown,body:(custody:unknown)=>Promise<void>)=>{expect(frame.gate).toBe(true);expect(actual).toBe(workspace);expect(selection).toEqual({kind:"all"});await body({scopes:[{manifest:"package.json",packages:["member"]}],recheck:async()=>{expect(frame.gate).toBe(true);check();events.push("custody");}});},
    javascriptDependencyReadContextV1:async()=>({root:store.root}),readJavascriptDependencyPreservationSourceV1:async(actual:string,_context:unknown,_control:unknown,ownership:unknown)=>{expect(frame.gate).toBe(true);expect(actual).toBe(workspace);expect(ownership).toEqual({version:1,scopes:[{manifest:"package.json",packages:["member"]}]});events.push("source");if(row.cancelled)stop.abort(cancelled);return {rows:[{kind:"link",sourceDirectory:endpoint,target:fs.readlinkSync(endpoint)}]};},
    tryConcurrentPinExclusiveV1:async(_directory:string,_workspace:string,name:string)=>{
     events.push("pin:"+name);
     if(name===byRole.pinned)return null;
     pins++;
     return {release:()=>{pins--;}};
    },removeJavascriptDependencyTreeV1:async(name:string)=>{expect(frame.gate).toBe(!row.normalMaintenance);events.push("delete");fs.rmSync(name,{recursive:true});},
    retireJavascriptDependencyPreservationsInStoreV1:async()=>{expect(frame.gate).toBe(true);events.push("strict");throw strictFailure;}
   };
   const [run,collect]=new Function(...Object.keys(ports),compiled)(...Object.values(ports));
   const operation=row.normalMaintenance?collect(workspace,store,control,{}):run(workspace,store,control);
   const failure=await operation.then(()=>null,(error:unknown)=>error);
   expect(failure,row.id).toBe(row.cancelled?cancelled:row.strict?strictFailure:null);
   expect(frame.gate,row.id).toBe(false);
   expect(events.filter(event=>event==="source").length,row.id).toBe(row.freeAfterGate>=row.requiredBytes?0:1);
   if(events.includes("source"))expect(events.indexOf("custody"),row.id).toBeGreaterThan(events.indexOf("gate"));
   if(events.includes("source"))expect(events.indexOf("source"),row.id).toBeGreaterThan(events.indexOf("custody"));
   const removed=row.generations.filter(generation=>!fs.existsSync(entry(generation))).map(generation=>generation.role);
   expect(removed,row.id).toEqual(expectedRemoved);
   if("skipSweep" in row&&row.skipSweep)expect(events).not.toContain("sweep");
   if(!row.normalMaintenance&&row.removed.length&&byRole.untouched)for(const generation of row.generations.filter(value=>value.eligible&&!expectedRemoved.includes(value.role)))expect(events).not.toContain("pin:"+generation.name);
   expect(pins,row.id).toBe(0);
   expect(fs.realpathSync(endpoint),row.id).toBe(fs.realpathSync(target));
   expect(free(),row.id).toBe(row.finalFree);
   const oracle=database.query(`with candidates as (
    select json_extract(value,'$.bytes') bytes,
           coalesce(sum(json_extract(value,'$.bytes')) over (
             order by json_extract(value,'$.tombstone') desc, json_extract(value,'$.name')
             rows between unbounded preceding and 1 preceding),0) before
      from json_each(?1)
     where json_extract(value,'$.eligible')
   ), free as (
    select ?2+coalesce(sum(bytes),0) n
      from candidates where not ?4 and (?5 or ?2+before<?3)
   ) select n, n<?3 and not ?4 and not ?5 strict from free`).get(JSON.stringify(row.generations),row.freeAfterGate,row.requiredBytes,Number(row.cancelled),Number(row.normalMaintenance)) as {n:number;strict:number};
   expect(oracle,row.id).toEqual({n:row.finalFree,strict:Number(row.strict)});
  }finally{fs.rmSync(root,{recursive:true,force:true});}
 }}finally{database.close();}
});


test("post-publication collection yields a partial tombstone and an owned complete pass resumes it",async()=>{
 const ts=await import("typescript"),fs=await import("node:fs"),path=await import("node:path"),{Database}=await import("bun:sqlite"),{concurrentCollectionV1,concurrentEventV1,concurrentProtocolV1}=await import("../../../🚦️concurrent/🟦️.ts"),{selectObsoleteJavascriptDependencyGenerationsV1}=await import("../../📥️read-context/🟦️.ts"),{javascriptInstallSchemaV1:installSchema}=await import("../../🧱️schemas/🟦️.ts"),{default:cases}=await import("./🧹️.json");
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned cleanup test artifact directory required");
 const tree=ts.createSourceFile("actual-dependencies.ts",source,ts.ScriptTarget.Latest,true);
 const collector=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="collectJavascriptDependencyGenerationsV1");
 expect(collector).toBeDefined();
 const code=ts.transpileModule(collector!.getText(tree)+";return collectJavascriptDependencyGenerationsV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:");
 try{for(const row of cases){
  const root=fs.mkdtempSync(path.join(artifacts,"cleanup-")),container=path.join(root,"j"),candidate="a".repeat(20),current="b".repeat(20),linked="c".repeat(20),original=path.join(container,candidate),tombstone=path.join(container,".x"+candidate),pins:string[]=[];
  const cancelled=Error("fixture requested cancellation"),cleanup=Error("fixture cleanup fault"),release=Error("fixture pin release fault");
  let clock=0,attempt=0,active=0,stop=new AbortController(),reports:string[]=[];
  const check=()=>stop.signal.throwIfAborted();
  try{
   for(const name of [candidate,current,linked])fs.mkdirSync(path.join(container,name),{recursive:true});
   for(let index=0;index<row.files;index++)fs.writeFileSync(path.join(original,String(index)),"owned");
   const old=new Date(Date.now()-604800000);fs.utimesSync(original,old,old);
   const protocol={...concurrentProtocolV1,retention:{...concurrentProtocolV1.retention,postPublicationCleanupEntries:row.maximumEntries,postPublicationCleanupMilliseconds:row.maximumMilliseconds}};
   const ports={resolve:path.resolve,join:path.join,performance:{now:()=>clock},installSchema,concurrentProtocolV1:protocol,process,lstatSync:fs.lstatSync,readdirSync:fs.readdirSync,renameSync:fs.renameSync,requireRepositoryCacheStoreV1:check,requireRepositoryCacheStoreReadCustodyV1:check,checkResourceLeaseControl:check,repositoryLeaseDirectoryV1:()=>root,concurrentCollectionV1,concurrentEventV1,selectObsoleteJavascriptDependencyGenerationsV1,concurrentUnreadyCapsulePatternV1:/^\.p[a-f0-9]{8}$/u,
    javascriptDependencyConcurrencyReportV1:()=>async()=>{},concurrentGenerationReferencedV1:(_container:string,name:string)=>name===linked,readConcurrentPointersV1:()=>[{scope:".",generation:current.padEnd(64,"0"),retired:[]}],sweepConcurrentTemporariesV1:()=>0,concurrentFreeFractionV1:async()=>0.1,concurrentFreeBytesV1:async()=>0,
    writeOwnedCommandOutputV1:async(_stream:unknown,line:string)=>{reports.push(line);},
    tryConcurrentPinExclusiveV1:async(_directory:string,_workspace:string,name:string)=>{
     pins.push(name);active++;
     return {release:()=>{active--;if(attempt===0&&row.fault==="release")throw release;}};
    },
    removeJavascriptDependencyTreeV1:async(directory:string,tick:(count:number)=>Promise<void>)=>{
     expect(active).toBe(1);expect(directory).toBe(tombstone);
     let count=0;
     for(const name of fs.readdirSync(directory)){
      fs.unlinkSync(path.join(directory,name));count++;clock+=row.stepMilliseconds;
      if(attempt===0&&count===row.faultAt){
       if(row.fault==="cancel")stop.abort(cancelled);
       if(row.fault==="cleanup")throw cleanup;
      }
      await tick(count);
     }
     fs.rmdirSync(directory);return count+1;
    }
   };
   const collect=new Function(...Object.keys(ports),code)(...Object.values(ports));
   const run=(options:unknown)=>collect(root,{root},{signal:stop.signal,publish:async()=>{},yieldContinuation:async()=>{}},options).then(()=>null,(error:unknown)=>error);
   const result=await run(row.mode==="slice"?{postPublication:true}:row.mode==="pressure"?{neededBytes:100,stopAtNeededBytes:true}:{});
   const remaining=fs.existsSync(tombstone)?fs.readdirSync(tombstone).length:0,removed=row.files-remaining;
   expect(removed,row.id).toBe(row.removed);expect(result,row.id).toBe(row.outcome==="cancel"?cancelled:null);
   expect(pins,row.id).toEqual([candidate]);expect(active,row.id).toBe(0);expect(fs.existsSync(original),row.id).toBe(false);expect(fs.existsSync(tombstone),row.id).toBe(remaining>0);
   const expectedReport=row.outcome==="deferred"?"current-dependency-generation-collection-deferral":row.outcome==="failed"?"current-dependency-generation-collection-failure":null;
   expect(reports.length,row.id).toBe(expectedReport?1:0);
   if(expectedReport)expect(reports[0],row.id).toContain(expectedReport);
   if(row.fault==="release")expect(reports[0],row.id).toContain(release.message);
   if(row.fault==="cleanup")expect(reports[0],row.id).toContain(cleanup.message);
   const oracle=db.query("select min(?1,case when ?2<>'slice' then ?1 when ?3 in ('cancel','cleanup') then ?4 else min(?5,(?6+?7-1)/?7) end) removed").get(row.files,row.mode,row.fault,row.faultAt,row.maximumEntries,row.maximumMilliseconds,row.stepMilliseconds);
   expect(oracle,row.id).toEqual({removed});
   attempt++;stop=new AbortController();reports=[];
   expect(await run({}),row.id).toBeNull();
   expect(pins,row.id).toEqual(remaining?[candidate,candidate]:[candidate]);expect(active,row.id).toBe(0);
   expect(fs.existsSync(tombstone),row.id).toBe(false);expect(fs.existsSync(path.join(container,current)),row.id).toBe(true);expect(fs.existsSync(path.join(container,linked)),row.id).toBe(true);
  }finally{fs.rmSync(root,{recursive:true,force:true});}
 }}finally{db.close();}
});
