/** 🧪️ Revision: 1 test covering neutral bounded revision contract and independent SQLite admission agree. */
import{afterAll,test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{mkdtemp,rm}from"node:fs/promises";
import{copyFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{mirrorRevisionReaderDirectoryV1}from"./🪞️mirror/🟦️.ts";
import{churnProtocolV1,staleImplementationV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{scriptInvocationEnvironment,withIssuedResourceControlV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{registerBuildTestLawV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
test("neutral bounded revision contract and independent SQLite admission agree",()=>{const db=new Database(":memory:"),ajv=new Ajv({strict:true});try{for(const row of corpus.cases){expect(ajv.validate(schema.definitions.Case,row)).toBe(true);expect(db.query<{accepted:number},[number,number,number]>("select (?2=cast(?2 as integer) and ?2>=0 and ?2<=8388608 and ?1<=?2 and (?3=0 or ?2<=65536)) accepted").get(row.bytes,row.maximumBytes,row.retainText?1:0)?.accepted===1).toBe(row.accepted);}expect(ajv.validate(schema,{sha256:"a".repeat(64),byteLength:0,text:null})).toBe(true);expect(ajv.validate(schema,{sha256:"a".repeat(64),byteLength:0,text:null,authority:true})).toBe(false);}finally{db.close();}});
registerBuildTestLawV1("bounded revision actual Source windows agree with independent Bun SHA and retain only requested finite text","sdk-fence",original=>withIssuedResourceControlV1(original,"source:bounded-revision",0,async control=>{
 const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual revision sink required");const root=await mkdtemp(join(sink,"rv-")),routing=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)),reader=fileURLToPath(new URL("../🟨️.mjs",import.meta.url));try{for(const row of corpus.cases){const program=`const{receiveScriptProcessInvocation}=await import(${JSON.stringify(routing)}),{captureBoundedSourceRevisionV1}=await import(${JSON.stringify(reader)}),{openSync,ftruncateSync,closeSync}=await import('node:fs');const row=${JSON.stringify(row)},path=${JSON.stringify(root)}+'/input',fd=openSync(path,'w');ftruncateSync(fd,row.bytes);closeSync(fd);let failed=false,message='',value;try{await receiveScriptProcessInvocation(process.env,async()=>{value=captureBoundedSourceRevisionV1(path,row.maximumBytes,row.retainText);});}catch(error){failed=true;message=error instanceof Error?error.message:String(error);}console.log(JSON.stringify({failed,message,value:value&&{...value,text:value.text===null?null:Buffer.from(value.text).toString('base64')}}));process.exitCode=failed?1:0;`,result=await captureOwnedCommandResultV1(process.execPath,["-e",program],process.cwd(),{control,env:{...scriptInvocationEnvironment(original,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"},output:{maximumBytes:98304,maximumLines:16}}),data:unknown=JSON.parse(result.stdout.trim().split("\n").at(-1)??"");expect(result.status,row.name+":"+result.stderr).toBe(row.accepted?0:1);if(!data||typeof data!=="object"||!("failed"in data))throw Error("Terminal bounded revision DATA required");expect(data.failed).toBe(!row.accepted);if(row.accepted){if(!("value"in data)||!data.value||typeof data.value!=="object")throw Error("Actual bounded revision record required");const observed="text"in data.value&&typeof data.value.text==="string"?{...data.value,text:Buffer.from(data.value.text,"base64").toString("utf8")}:data.value;expect(new Ajv({strict:true}).validate(schema,observed)).toBe(true);const hasher=new Bun.CryptoHasher("sha256"),window=Buffer.alloc(4096);for(let used=0;used<row.bytes;used+=window.length)hasher.update(window.subarray(0,Math.min(window.length,row.bytes-used)));expect(observed).toEqual({sha256:hasher.digest("hex"),byteLength:row.bytes,text:row.retainText?"\0".repeat(row.bytes):null});}}expect(original.control.signal.aborted).toBe(false);}finally{await rm(root,{recursive:true,force:true});}
}));

registerBuildTestLawV1("bounded revision physical ancestry and exact failures retain original primary through descriptor closure","sdk-fence",original=>withIssuedResourceControlV1(original,"source:revision-physical",0,async control=>{
 const sink=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!sink)throw Error("Actual physical revision sink required");const root=await mkdtemp(join(sink,"rv-")),routing=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)),reader=fileURLToPath(new URL("../🟨️.mjs",import.meta.url));try{for(const[rowIndex,row]of corpus.physical.entries()){expect(new Ajv({strict:true}).validate(schema.definitions.PhysicalCase,row)).toBe(true);const program=`const{receiveScriptProcessInvocation}=await import(${JSON.stringify(routing)}),{default:fs}=await import('node:fs'),{mock}=await import('bun:test');const root=${JSON.stringify(root)}+'/'+${rowIndex},row=${JSON.stringify(row)},parent=root+'/parent',leaf=parent+'/leaf';fs.mkdirSync(parent,{recursive:true});fs.writeFileSync(leaf,'abc');let path=leaf;if(row.name==='parent-link'){fs.symlinkSync(parent,root+'/alias',process.platform==='win32'?'junction':'dir');path=root+'/alias/leaf';}if(row.name==='leaf-link'){fs.symlinkSync(leaf,root+'/alias','file');path=root+'/alias';}if(row.name==='hard-link')fs.linkSync(leaf,root+'/alias');const bodyError=Error('Exact revision body'),closeError=Error('Exact revision close'),originalOpen=fs.openSync,originalRead=fs.readSync,originalClose=fs.closeSync;let active,closed=0,observed=0;const port={...fs,openSync(candidate,...args){const fd=originalOpen.call(fs,candidate,...args);if(String(candidate)===path)active=fd;return fd;},readSync(fd,buffer,offset,length,position){if(fd===active){observed++;if(row.primary==='body')throw bodyError;if(row.primary==='undefined')throw undefined;if(row.primary==='short')return 0;if(row.primary==='eof'&&position===3)return 1;}const bytes=originalRead.call(fs,fd,buffer,offset,length,position);if(fd===active&&position===0){if(row.name==='file-change')fs.writeFileSync(leaf,'changed');if(row.name==='parent-change'){fs.renameSync(parent,root+'/moved'+observed);fs.mkdirSync(parent);fs.writeFileSync(leaf,'abc');}}return bytes;},closeSync(fd){const value=originalClose.call(fs,fd);if(fd===active){closed++;if(['body','undefined','close'].includes(row.primary))throw closeError;}return value;}};mock.module('node:fs',()=>({...port,default:port}));const{captureBoundedSourceRevisionV1}=await import(${JSON.stringify(reader)});let failed=false,message='',exact=false;try{await receiveScriptProcessInvocation(process.env,async()=>{captureBoundedSourceRevisionV1(path,16);});}catch(error){failed=true;message=error instanceof Error?error.message:String(error);exact=row.primary==='body'?error===bodyError:row.primary==='undefined'?error===undefined:row.primary==='close'?error===closeError:true;}console.log(JSON.stringify({failed,message,exact,closed,observed}));process.exitCode=failed?1:0;`,result=await captureOwnedCommandResultV1(process.execPath,["-e",program],process.cwd(),{control,env:{...scriptInvocationEnvironment(original,process.env),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"},output:{maximumBytes:8192,maximumLines:32}}),data:unknown=JSON.parse(result.stdout.trim().split("\n").at(-1)??"");if(result.status!==1)throw Error(row.name+": exit "+result.status+": "+result.stderr.slice(-2000));if(!data||typeof data!=="object"||!("failed"in data)||!("message"in data)||!("exact"in data)||!("closed"in data))throw Error("Actual physical revision DATA required");expect(data.failed).toBe(true);expect(data.exact).toBe(true);expect(data.closed).toBe(["ancestry","leaf"].includes(row.primary)?0:["changed","short","eof"].includes(row.primary)?churnProtocolV1().profiles.read.maximumAttempts:1);if(["changed","short","eof"].includes(row.primary))expect(data.message).toMatch(new RegExp(`kept changing in all ${churnProtocolV1().profiles.read.maximumAttempts} attempts|änderte sich in allen ${churnProtocolV1().profiles.read.maximumAttempts} Versuchen`,"u"));if(row.primary==="ancestry")expect(data.message).toBe("Physical Source revision ancestry required");if(row.primary==="leaf")expect(data.message).toBe("Bounded physical Source revision required");}expect(original.control.signal.aborted).toBe(false);}finally{await rm(root,{recursive:true,force:true});}
}));

test("captured physical Source refusal diagnostics stay closed, relative and bounded",async()=>{
 const fs=await import("node:fs/promises"),paths=await import("node:path"),{default:ts}=await import("typescript"),source=await fs.readFile(new URL("../🟨️.mjs",import.meta.url),"utf8"),file=ts.createSourceFile("revision.mjs",source,ts.ScriptTarget.Latest,true),node=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="capturedReadRefusalV1");if(!node)throw Error("Actual captured Source refusal owner required");const actual=new Function("relative","isAbsolute","sep",node.getText()+";return capturedReadRefusalV1;")(paths.posix.relative,paths.posix.isAbsolute,"/"),ajv=new Ajv({strict:true}),db=new Database(":memory:");
 try{for(const row of corpus.refusals){expect(ajv.validate({...schema.definitions.CapturedReadRefusalCase,definitions:schema.definitions},row)).toBe(true);const inside=db.query("select ?1=?2 or substr(?2,1,length(?1)+1)=?1||'/' accepted").get(row.root,row.path)as{accepted:number},oracle={version:1,scope:inside.accepted?"workspace":"outside",path:inside.accepted?row.path.slice(row.root.length+(row.path===row.root?0:1)):null,sourceCount:row.sources};expect(oracle).toEqual(row.expected);expect(actual({root:row.root,sources:{size:row.sources}},row.path)).toEqual(oracle);expect(ajv.validate(schema.definitions.CapturedReadRefusal,oracle)).toBe(true);}}finally{db.close();}
});

/** 🦜️ Loads distinct real reader modules and the actual self-check against a controlled changed capture: the second reader is a physical copy laid out like the repository (Bun evaluates one module per path, so a query suffix cannot make a second instance). */
const copies:string[]=[],pinCount=((globalThis as unknown as Record<symbol,{open:number;processWide:boolean}>)[Symbol.for("semio.implementationPin.count.v1")]??={open:0,processWide:false});
/** 🛎️ Runs a law that models a long-lived service (invocations side by side, no process-wide pin): a process the repository itself births already opted in to the process-wide pin, so the flag is cleared for the body and restored afterwards. */
async function asService<T>(body:()=>Promise<T>):Promise<T>{const was=pinCount.processWide;pinCount.processWide=false;try{return await body();}finally{pinCount.processWide=was;}}
afterAll(()=>{for(const root of copies)rmSync(root,{recursive:true,force:true});});
async function implementationPinReadersV1(){
 const fs=await import("node:fs/promises"),{default:ts}=await import("typescript"),source=await fs.readFile(new URL("../🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("revision.mjs",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="current"),same=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="unchanged");
 if(!node||!same)throw Error("Actual reader current guard and its comparison required");
 const root=mkdtempSync(join(realpathSync(tmpdir()),"pin-copy-")),directory=mirrorRevisionReaderDirectoryV1(root);copies.push(root);mkdirSync(join(directory,"🧬️schema"),{recursive:true});
 const owned=(name:string)=>fileURLToPath(new URL(name,import.meta.url));copyFileSync(owned("../🟨️.mjs"),join(directory,"🟨️.mjs"));copyFileSync(owned("../🧬️schema/🔣️.json"),join(directory,"🧬️schema","🔣️.json"));
 const main=await import("../🟨️.mjs"),copy=await import(join(directory,"🟨️.mjs"));
 expect(copy.withImplementationPinV1).not.toBe(main.withImplementationPinV1);
 const guard=(reader:typeof main)=>new Function("implementationPinnedV1","capture","ownPath","loaded","loadedSchema","ownSchemaPath","ownUnchanged","staleImplementationV1",same.getText(tree)+";"+node.getText(tree)+";return current;")(reader.implementationPinnedV1,()=>({sha256:"changed"}),"/fixture/reader",{byteLength:1,sha256:"loaded"},undefined,"/fixture/schema",()=>false,staleImplementationV1)as()=>void;
 return{main,copy,guard};
}

test("a pinned invocation completes with the Source it loaded: an edited reader refuses outside the pin and lets the invocation finish inside it; pins nest and a failing body releases its pin",()=>asService(async()=>{
 const{main:reader,copy,guard}=await implementationPinReadersV1(),current=guard(reader),primary=Object.freeze({pinBodyFailure:true});
 expect(reader.implementationPinnedV1()).toBe(false);expect(current).toThrow("Stale loaded Source revision reader; restart required");
 await reader.withImplementationPinV1(async()=>{
  expect(reader.implementationPinnedV1()).toBe(true);expect(copy.implementationPinnedV1()).toBe(true);
  await copy.withImplementationPinV1(async()=>{expect(current).not.toThrow();});
  expect(reader.implementationPinnedV1()).toBe(true);expect(current).not.toThrow();
 });
 expect(reader.implementationPinnedV1()).toBe(false);expect(current).toThrow("Stale loaded Source revision reader");
 await expect(reader.withImplementationPinV1(async()=>{throw primary;})).rejects.toBe(primary);expect(reader.implementationPinnedV1()).toBe(false);
}));

test("implementation pins isolate concurrent invocations while sharing module-copy context",()=>asService(async()=>{
 const readers=await implementationPinReadersV1(),db=new Database(":memory:"),ajv=new Ajv({strict:true});
 try{
  for(const row of corpus.implementationPins.scopeCases){
   expect(ajv.validate(schema.definitions.ImplementationPinCase,row),row.name).toBe(true);
   const oracle=db.query("WITH RECURSIVE scopes AS(SELECT json_extract(value,'$.id')id,json_extract(value,'$.parent')parent,json_extract(value,'$.pin')pin FROM json_each(?1)),lineage(id,pinned,depth)AS(SELECT id,pin,0 FROM scopes WHERE parent IS NULL UNION ALL SELECT s.id,CASE WHEN s.pin OR l.pinned THEN 1 ELSE 0 END,l.depth+1 FROM scopes s JOIN lineage l ON s.parent=l.id WHERE l.depth<16)SELECT id,pinned FROM lineage ORDER BY id").all(JSON.stringify(row.scopes))as{id:string;pinned:number}[];
   expect(oracle.length,row.name+" complete neutral forest").toBe(row.scopes.length);expect(new Set(oracle.map(value=>value.id)).size).toBe(row.scopes.length);
   const expected=Object.fromEntries(row.scopes.map(value=>[value.id,value.expected])),actual:Record<string,boolean>={};expect(Object.fromEntries(oracle.map(value=>[value.id,Boolean(value.pinned)])),row.name+" SQLite ancestry").toEqual(expected);
   let entered=0,release!:()=>void;const allEntered=new Promise<void>(resolve=>{release=resolve;});
   const visit=async(scope:typeof row.scopes[number]):Promise<void>=>{
    const reader=readers[scope.module as"main"|"copy"],body=async()=>{
     const children=Promise.all(row.scopes.filter(child=>child.parent===scope.id).map(visit));
     if(++entered===row.scopes.length)release();await allEntered;
     actual[scope.id]=reader.implementationPinnedV1();expect(actual[scope.id],row.name+":"+scope.id).toBe(expected[scope.id]);
     const current=readers.guard(reader);if(expected[scope.id])expect(current).not.toThrow();else expect(current).toThrow("Stale loaded Source revision reader");
     await children;
    };
    if(scope.pin)await reader.withImplementationPinV1(body);else await body();
   };
   await Promise.all(row.scopes.filter(scope=>scope.parent===null).map(visit));expect(actual,row.name).toEqual(expected);expect(readers.main.implementationPinnedV1()).toBe(false);expect(readers.copy.implementationPinnedV1()).toBe(false);
  }
  for(const row of corpus.implementationPins.lifetimeCases){
   expect(ajv.validate(schema.definitions.ImplementationPinLifetimeCase,row),row.name).toBe(true);
   const innerPrimary=Object.freeze({inner:row.name}),outerPrimary=Object.freeze({outer:row.name}),actual:Record<string,boolean>={};let releaseInner!:()=>void,releaseOuter!:()=>void,innerDescendant!:Promise<boolean>,outerDescendant!:Promise<boolean>,outerRejected=false;
   const innerGate=new Promise<void>(resolve=>{releaseInner=resolve;}),outerGate=new Promise<void>(resolve=>{releaseOuter=resolve;});
   try{await readers.main.withImplementationPinV1(async()=>{
    actual.outer=readers.copy.implementationPinnedV1();outerDescendant=outerGate.then(()=>readers.copy.implementationPinnedV1());let innerRejected=false;
    try{await readers.copy.withImplementationPinV1(async()=>{actual.inner=readers.main.implementationPinnedV1();innerDescendant=innerGate.then(()=>readers.main.implementationPinnedV1());if(row.innerFailure)throw innerPrimary;});}catch(error){innerRejected=true;expect(error).toBe(innerPrimary);}
    expect(innerRejected,row.name).toBe(row.innerFailure);actual.afterInner=readers.copy.implementationPinnedV1();releaseInner();actual.innerDescendant=await innerDescendant;if(row.outerFailure)throw outerPrimary;
   });}catch(error){outerRejected=true;expect(error).toBe(outerPrimary);}
   expect(outerRejected,row.name).toBe(row.outerFailure);actual.afterOuter=readers.main.implementationPinnedV1();releaseOuter();actual.outerDescendant=await outerDescendant;expect(actual,row.name).toEqual(row.expected);expect(readers.copy.implementationPinnedV1()).toBe(false);
  }
 }finally{db.close();}
}));

test("authority data is read once per pinned invocation and again after the snapshot is cleared; outside a pin every read is fresh",async()=>{
 const fs=await import("node:fs"),os=await import("node:os"),home=fileURLToPath(new URL("..",import.meta.url)),listing=fs.readdirSync(home),moduleName=listing.find(name=>name.endsWith(".mjs"))!,schemaName=listing.find(name=>name.endsWith("schema"))!;
 const scratch=fs.mkdtempSync(join(fs.realpathSync(os.tmpdir()),"revision-snapshot-")),mirror=(await import("./🪞️mirror/🟦️.ts")).mirrorRevisionReaderDirectoryV1(scratch);
 try{
  fs.mkdirSync(join(mirror,schemaName));const schemaFile=fs.readdirSync(join(home,schemaName))[0]!;fs.copyFileSync(join(home,schemaName,schemaFile),join(mirror,schemaName,schemaFile));fs.copyFileSync(join(home,moduleName),join(mirror,moduleName));
  const reader=await import(join(mirror,moduleName));let version=0;const read=()=>++version;
  expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(1);expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(2);
  await reader.withImplementationPinV1(async()=>{
   expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);expect(reader.pinnedAuthorityReadV1("policy",read)).toBe(4);
   await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);});
   expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(3);reader.clearPinnedAuthorityV1();expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(5);
  });
  expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(6);
  await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(7);});
  await reader.withImplementationPinV1(async()=>{expect(reader.pinnedAuthorityReadV1("registry",read)).toBe(8);});
 }finally{fs.rmSync(scratch,{recursive:true,force:true});}
});

test("a process the repository itself births is pinned as a whole: every scope of it counts as pinned while one pin is open, and none once it closed",async()=>{
 const{main}=await implementationPinReadersV1(),was=pinCount.processWide;pinCount.processWide=true;
 const ambient=pinCount.open>0;
 try{expect(main.implementationPinnedV1()).toBe(ambient);await main.withImplementationPinV1(async()=>{await new Promise<void>(resolve=>setImmediate(resolve));expect(main.implementationPinnedV1()).toBe(true);});expect(main.implementationPinnedV1()).toBe(ambient);}finally{pinCount.processWide=was;}
});
