/** 🪽️ Contract (on an entry whose layer is a TypeScript program, compared by its values, its derived facts and its graph and analysis rows (the reads of the provider designation are memoized per process and recorded by the first derivation only); a command walk is a `closure-graph` row, no layer): a query over command closures that finds a layer of a held entry gone from the store (the collector removed it, a peer superseded the entry) reads the entry anew and answers from one state, at most the declared attempts, saying why in English and German; layers and entries that are only added never touch it; a real second process that adds layers and entries in a loop and removes the held entry once does not make the query exit. */
import{afterAll,expect,spyOn,test}from"bun:test";
import{existsSync,mkdirSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{brotliDecompressSync}from"node:zlib";
import{readFileSync as readSource}from"node:fs";
import{dirname,join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{INFERENCE_LIMITS_V1}from"../../../🧬️schema/🟨️.mjs";
import schema from"../../../🧬️schema/🔣️.json";
import walks from"../../../🧪️tests/✂️derived-walks/🔣️.json";
const program=walks.program as {projectRoot:string;files:Record<string,string>};
import{markReadParentV1}from"../../../../../👁️observation/🚧️read-parent/🟨️.mjs";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-script-inputs"});
process.env.SEMIO_CLOSURE_FLEET="off";

const inference=await import("../../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const scriptInputs=await import("../../🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const lost=(digest?:string)=>Object.assign(Error("closure layer lost"),{closureLayerLost:true,storeChanged:true,layerDigest:digest});
const answer=(resolved:any)=>JSON.stringify([[...resolved.values],resolved.rows.filter((row:any)=>row.kind==="global"||row.kind==="analysis"),resolved.derived]);
const named=(entries:string)=>readdirSync(entries).flatMap(name=>(JSON.parse(brotliDecompressSync(readFileSync(join(entries,name))).toString()).layers as string[]));
const setup=async()=>{
 const root=await repository.scratch("layer-lost-");roots.push(root);
 write(root,program.files);
 scriptInputs.resetClosureEntriesV1();scriptInputs.bindClosureStoreV1(root,join(root,".ce"),true);
 const scripts=createScriptInputCache({workspaceRoot:root,lateClosures:true}),api=scriptInputs.closureApiV1(scripts,root);
 const handle=scriptInputs.closureHandleOfV1(await api.genericTargetCommandInputs({options:{cwd:program.projectRoot,command:"repo ./📜️script.ts check "+program.projectRoot},metadata:{semio:{closure:{typescript:["{projectRoot}/tsconfig.json"]}}}},program.projectRoot));
 return{root,api,recipes:[[handle.key,handle.recipe]],layers:join(root,".ce","l"),entries:join(root,".ce","e")};
};

test("the attempts are declared with an English and a German text",()=>{
 expect(INFERENCE_LIMITS_V1.closureLayerAttempts).toBe(3);
 for(const language of["en","de"]as const)expect(schema.definitions.LimitsDescriptions.const[language].closureLayerAttempts.length).toBeGreaterThan(40);
 expect(schema.definitions.LimitsDescriptions.const.en.closureLayerAttempts).not.toBe(schema.definitions.LimitsDescriptions.const.de.closureLayerAttempts);
});

test("a query that loses a layer is asked again and answers once the entry is read anew; a query that keeps losing ends with the owner's change; everything else ends at once",async()=>{
 const lines:string[]=[];let calls=0;
 expect(await scriptInputs.answeredAgainstLiveLayersV1("/w",async()=>{if(++calls<3)throw lost("a".repeat(64));return"fresh";},{say:(line:string)=>lines.push(line)})).toBe("fresh");
 expect(calls).toBe(3);expect(lines).toHaveLength(2);
 for(const line of lines){expect(line).toContain("aaaaaaaaaaaa");expect(line).toContain("Closure-Ebene");expect(line).toContain("closure layer");}
 calls=0;lines.length=0;
 await expect(scriptInputs.answeredAgainstLiveLayersV1("/w",async()=>{calls++;throw lost();},{say:(line:string)=>lines.push(line)})).rejects.toMatchObject({closureLayerLost:true,storeChanged:true});
 expect(calls).toBe(INFERENCE_LIMITS_V1.closureLayerAttempts+1);expect(lines).toHaveLength(INFERENCE_LIMITS_V1.closureLayerAttempts);
 calls=0;
 await expect(scriptInputs.answeredAgainstLiveLayersV1("/w",async()=>{calls++;throw Error("disk on fire");})).rejects.toThrow("disk on fire");
 expect(calls).toBe(1);
});

test("the parent of a read with a farm never derives: it forgets what was lost, says so, and ends the owner's look at once so that the farm derives the entries when the owner is asked again",async()=>{
 const lines:string[]=[];let calls=0;const unmark=markReadParentV1(()=>true);
 try{await expect(scriptInputs.answeredAgainstLiveLayersV1("/w",async()=>{calls++;throw lost("b".repeat(64));},{say:(line:string)=>lines.push(line)})).rejects.toMatchObject({closureLayerLost:true,storeChanged:true});}finally{unmark();}
 expect(calls).toBe(1);expect(lines).toHaveLength(1);expect(lines[0]).toContain("farm");expect(lines[0]).toContain("Farm");
});

test("a query whose provider entries keep changing past the declared attempts ends with the owner's change, and every refusal of a lost layer carries it",()=>{
 const source=readSource(new URL("../../🟨️.mjs",import.meta.url),"utf8");
 expect(source).toMatch(/closure\.provider-entry-stale", \{ port: sharedReferencesOfV1\(entry\.value\)[^\n]*\}\), \{ storeChanged: true \}\);/u);
 expect(source).toContain('{ closureLayerLost: true, storeChanged: true, layerDigest: digest }');
 expect(source.match(/layerLostRefusalV1\(/gu)!.length).toBe(3);
 expect(source.match(/port: "layers"/gu)!.length).toBe(1);
});

test("a held entry whose layers the collector removed is derived again and the query answers what it answered before",async()=>repository.invoke(async()=>{
 const{api,recipes,layers,entries}=await setup(),said:string[]=[],spy=spyOn(process.stderr,"write").mockImplementation(((chunk:unknown)=>{said.push(String(chunk));return true;})as never);
 try{
  await scriptInputs.withClosurePassV1(async()=>{
   await api.resolve(recipes);
   const first=answer(await api.resolve(recipes));
   const stored=readdirSync(layers);expect(stored.length).toBeGreaterThan(0);
   for(const name of stored)rmSync(join(layers,name));
   expect(answer(await api.resolve(recipes))).toBe(first);
   expect(readdirSync(entries).length).toBe(1);
   for(const digest of named(entries))expect(existsSync(join(layers,digest+".json.br"))).toBe(true);
   expect(answer(await api.resolve(recipes))).toBe(first);
  });
 }finally{spy.mockRestore();}
 const lines=said.filter(line=>line.startsWith("[closure] "));
 expect(lines.length).toBeGreaterThanOrEqual(1);
 for(const line of lines){expect(line).toContain("closure layer");expect(line).toContain("Closure-Ebene");}
}));

test("layers and entries that are only added never touch a query, even while a real second process writes them in a loop and removes the held entry once",async()=>repository.invoke(async()=>{
 const{api,recipes,layers,entries,root}=await setup(),store=join(root,".ce");
 const code=`
  const{readdirSync,rmSync}=await import("node:fs"),{join}=await import("node:path");
  const{createClosureStoreV1}=await import(${JSON.stringify(new URL("../../💾️entries/🟨️.mjs",import.meta.url).href)});
  const{collectClosureStoreV1}=await import(${JSON.stringify(new URL("../../🧹️retention/🟨️.mjs",import.meta.url).href)});
  const directory=${JSON.stringify(store)},held=readdirSync(join(directory,"e")),started=Date.now(),store=createClosureStoreV1({directory});
  let count=0,removed=false;
  while(Date.now()-started<1500){
   const layer=Array.from({length:40},(_,row)=>({kind:"analysis",path:"/w/c"+count+"/f"+row,fact:{digest:String(count*1000+row).padEnd(64,"e"),size:row}}));
   await store.save("child-"+count,{recipe:{fn:"child"+count},value:[],rows:[],layers:[layer]});count++;
   if(!removed&&Date.now()-started>400){removed=true;for(const name of held)rmSync(join(directory,"e",name),{force:true});await collectClosureStoreV1({directory,graceMilliseconds:0});}
  }
  console.log(JSON.stringify({count,removed}));`;
 await scriptInputs.withClosurePassV1(async()=>{
  await api.resolve(recipes);
  const first=answer(await api.resolve(recipes)),before=readdirSync(layers).length;expect(before).toBeGreaterThan(0);
  const child=Bun.spawn([process.execPath,"-e",code],{stdout:"pipe",stderr:"pipe"});
  let rounds=0,exited=false;void child.exited.then(()=>{exited=true;});
  while(!exited||rounds<5){expect(answer(await api.resolve(recipes))).toBe(first);rounds++;await Bun.sleep(5);}
  const report=JSON.parse((await new Response(child.stdout).text()).trim().split("\n").at(-1)!);
  expect(await child.exited,await new Response(child.stderr).text()).toBe(0);
  expect(report.removed).toBe(true);expect(report.count).toBeGreaterThan(10);expect(rounds).toBeGreaterThan(5);
  for(const digest of named(entries))expect(existsSync(join(layers,digest+".json.br"))).toBe(true);
  expect(answer(await api.resolve(recipes))).toBe(first);
 });
}),120000);
