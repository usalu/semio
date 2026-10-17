import{createIssuedResourceControlV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import policy from"../📋️policy.json"with{type:"json"};
import schema from"../🧬️schema.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
test("SDK clone publication charges actual copied bytes, every namespace entry and bounded authorizer writes",async()=>{
 const api=await import("../🟦️.ts"),ajv=new Ajv({strict:true}),oracle=new Database(":memory:");try{expect(ajv.validate(schema.definitions.Policy,policy)).toBe(true);for(const row of corpus.valid){expect(ajv.validate(schema.definitions.Material,row.material)).toBe(true);const expected=oracle.query<{bytes:number},[number,number]>("SELECT ?1+(?2+64)*65536+134217728 AS bytes").get(row.material.copyAllocatedBytes,row.material.entries)!.bytes;expect(expected).toBe(row.minimumAvailableBytes);expect(api.nxSdkCloneHeadroomBytesV1(row.material)).toBe(expected);}for(const row of corpus.invalid){expect(ajv.validate(schema.definitions.Material,row)).toBe(false);expect(()=>api.nxSdkCloneHeadroomBytesV1(row)).toThrow();}expect(()=>api.nxSdkCloneHeadroomBytesV1({version:1,entries:200000,copyAllocatedBytes:0})).toThrow();}finally{oracle.close();}
});

async function withActualSdkCapacityStore<T>(body:(workspace:string,store:import("../../../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts").RepositoryToolingStoreV1,source:string,inputs:ReadonlyMap<string,Uint8Array>)=>Promise<T>):Promise<T>{
 const fs=await import("node:fs/promises"),path=await import("node:path"),{receiveScriptProcessInvocation}=await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{readNativeCallerAuthorityV1}=await import("../../../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts"),{withRepositoryToolingStoreV1}=await import("../../../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts"),{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts"),retention=(await import("../../../🧹️retention/📥️physical/🧬️schema/🔣️.json")).default;
 return receiveScriptProcessInvocation(process.env,async original=>{const workspace=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;return withRepositoryToolingStoreV1(workspace,original,createIssuedResourceControlV1(original,"cache:entry",0),async store=>{
  const generation=await fs.realpath(path.join(workspace,".nx/installation")),material=JSON.parse(await fs.readFile(path.join(generation,".semio-sdk-material.json"),"utf8")),source=path.join(store.root,"sdk",material.snapshot),inputs=new Map<string,Uint8Array>();for(const name of["package.json","bun.lock"])inputs.set(name,await fs.readFile(path.join(source,name)));const manifest=JSON.parse(Buffer.from(inputs.get("package.json")!).toString("utf8"));for(const name of Object.values(manifest.patchedDependencies)as string[])inputs.set(name,await fs.readFile(path.join(source,name)));
  return body(workspace,store,source,inputs);
 });});
}
test("only a live authenticated snapshot loan can issue clone publication capacity",async()=>{
 const api=await import("../🟦️.ts"),path=await import("node:path"),{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");
 await withActualSdkCapacityStore(async(workspace,store,source,inputs)=>{let escaped:unknown;await withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>api.withNxSdkClonePublicationV1(workspace,store,source,inputs,control,async quote=>{escaped=quote;const minimum=await api.requireNxSdkClonePublicationV1(store,quote);expect(minimum).toBeGreaterThan(policy.maximumPublicationBytes);await expect(api.requireNxSdkClonePublicationV1(store,{...quote})).rejects.toThrow("Original");await expect(api.requireNxSdkClonePublicationV1(store,{kind:"sdk-clone"})).rejects.toThrow("Original");await expect(quote.clone(path.join(store.root,"tmp/quote-denied"),control)).rejects.toThrow("admission");const result=await publish(async()=>{await quote.recheck();const{captureOwnedCommandV1}=await import("../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"),python=Bun.which("python3")??Bun.which("python");if(!python)throw Error("Independent system Python capacity oracle required");const program="import os,sys,json,stat\nroot=sys.argv[1];unit=os.statvfs(root).f_frsize;known={r['path'] for r in json.load(open(os.path.join(root,'.semio-nx-sdk-binaries.json')))['objects']};entries=0;allocated=0\ndef visit(path):\n global entries,allocated\n f=os.path.join(root,path);s=os.lstat(f);entries+=1\n if stat.S_ISDIR(s.st_mode):\n  for name in os.listdir(f):visit(path+'/'+name)\n elif stat.S_ISREG(s.st_mode) and path not in known:allocated+=((s.st_size+unit-1)//unit)*unit\nvisit('node_modules')\nprint(json.dumps({'minimum':allocated+(entries+64)*65536+134217728,'entries':entries,'copyAllocatedBytes':allocated,'unit':unit}))";const observed=JSON.parse(await captureOwnedCommandV1(python,["-c",program,source],workspace,{control:control,env:store.environment,output:{maximumBytes:65536,maximumLines:1024}}));expect(observed.minimum).toBe(minimum);return 7;},quote);expect(result).toBe(7);await expect(quote.clone(path.join(store.root,"tmp/quote-closed-operation"),control)).rejects.toThrow("admission");}));await expect(api.requireNxSdkClonePublicationV1(store,escaped)).rejects.toThrow("released");});
},120000);

test("concurrent clone claims spend one issued capacity quote at most once",async()=>{
 const api=await import("../🟦️.ts"),path=await import("node:path"),{randomUUID}=await import("node:crypto"),{lstat}=await import("node:fs/promises"),{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");
 await expect(withActualSdkCapacityStore((workspace,store,source,inputs)=>withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>api.withNxSdkClonePublicationV1(workspace,store,source,inputs,control,quote=>publish(async()=>{
  const targets=["a","b"].map(name=>path.join(store.root,"tmp/quote-absent-"+randomUUID()+name));for(const target of targets)await expect(lstat(target)).rejects.toThrow("ENOENT");const results=await Promise.allSettled(targets.map(target=>quote.clone(target,control)));expect(results.every(row=>row.status==="rejected")).toBe(true);expect(results.filter(row=>row.status==="rejected"&&String(row.reason).includes("already consumed"))).toHaveLength(1);
 },quote))))).rejects.toThrow("ENOENT");
},120000);

test("the actual publisher binds its prepared clone to the issued quote and retains cold admission",async()=>{
 const{readFileSync}=await import("node:fs"),parser=(await import("typescript")).default,body=readFileSync(new URL("../../../📜️script.ts",import.meta.url),"utf8"),tree=parser.createSourceFile("publisher.ts",body,parser.ScriptTarget.Latest,true),calls:import("typescript").CallExpression[]=[];function visit(node:import("typescript").Node){if(parser.isCallExpression(node))calls.push(node);parser.forEachChild(node,visit);}visit(tree);
 expect(calls.filter(node=>node.expression.getText(tree)==="withNxSdkClonePublicationV1")).toHaveLength(1);expect(calls.filter(node=>node.expression.getText(tree)==="quote.clone")).toHaveLength(1);expect(body).toContain("ManagedToolingPublisherV1");
 const output=(await import("../../../🔏️authorizers/📦️snapshot/🔣️policy.json")).default;expect(policy.maximumPublicationBytes).toBeGreaterThanOrEqual(output.maximumSnapshotBytes*2+output.maximumOutputBytes*2+policy.maximumInputBytes);
});

test("unquoted publication cannot lower the cold installer floor",async()=>{
 const{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");await withActualSdkCapacityStore(async(workspace,store)=>{await expect(withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>publish(async()=>{throw Error("Cold floor bypass");}))).rejects.toThrow("capacity");});
},120000);

test("registry scope settles detached publication before returning",async()=>{
 const{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");await withActualSdkCapacityStore(async(workspace,store)=>{let settled=false,work:Promise<void>|undefined;try{await expect(withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},async (publish,control)=>{work=publish(async()=>{}).then(()=>{settled=true;},()=>{settled=true;});})).rejects.toThrow("capacity");expect(settled).toBe(true);}finally{await work;}});
},120000);

test("two issued quote publications serialize and keep exact ALS identity",async()=>{
 const api=await import("../🟦️.ts"),path=await import("node:path"),{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");let active=0,maximum=0,completed=0;await withActualSdkCapacityStore((workspace,store,source,inputs)=>withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>api.withNxSdkClonePublicationV1(workspace,store,source,inputs,control,first=>api.withNxSdkClonePublicationV1(workspace,store,source,inputs,control,async second=>{
  await Promise.all([first,second].map(quote=>publish(async()=>{active++;maximum=Math.max(maximum,active);try{await expect((quote===first?second:first).clone(path.join(store.root,"tmp/quote-wrong-context"),control)).rejects.toThrow("admission");await store.recheck();completed++;}finally{active--;}},quote)));expect(maximum).toBe(1);expect(completed).toBe(2);
 }))));
},120000);

test("delayed descendants cannot reuse a completed publication context",async()=>{
 const api=await import("../🟦️.ts"),path=await import("node:path"),{withManagedToolingProvisionV1}=await import("../../../🧹️retention/📥️physical/🟦️.ts");await withActualSdkCapacityStore((workspace,store,source,inputs)=>withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>api.withNxSdkClonePublicationV1(workspace,store,source,inputs,control,async quote=>{
  let release!:()=>void,late:Promise<unknown>|undefined;const resumed=new Promise<void>(accept=>{release=accept;});await publish(async()=>{late=resumed.then(()=>quote.clone(path.join(store.root,"tmp/quote-delayed"),control));void late.catch(()=>{});},quote);release();await expect(late!).rejects.toThrow("admission");
 })));
},120000);
