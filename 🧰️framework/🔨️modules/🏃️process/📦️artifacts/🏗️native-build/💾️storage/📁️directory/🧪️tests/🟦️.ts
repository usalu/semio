import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,writeFileSync,existsSync,readdirSync,symlinkSync,linkSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {activateNativeDirectoryV1,admitNativeDirectoryCapacityV1,allocateNativeDirectoryV1,createNativeDirectoryMeterV1,nativeDirectorySwapPendingV1,probeNativeDirectoryCaseCapabilityV1,recoverNativeDirectoryV1,deactivateNativeDirectoryV1,exchangeNativeDirectoriesV1,listNativeDirectoryChildrenV1,measureNativeDirectoryUsageV1,nativeDirectoryAnchorBytesV1,nativeDirectoryMappingV1,nativeDirectoryProtocolV1,nativeDirectorySystemPortV1,observeNativeDirectoryIdentityV1,observeNativeDirectoryStateV1,probeNativeDirectoryCaseSensitivityV1,probeNativeDirectoryExchangeV1,readNativeDirectoryAnchorV1,runNativeDirectoryActionV1,type NativeDirectoryPlatformPortV1} from "../🟦️.ts";
import {createNativeDirectoryExchangePortV1} from "../🔀️exchange/🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),script=resolve(import.meta.dir,"../📜️script.ts"),oracle=new Ajv({strict:true});
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"dir-"));};
const sensitive=()=>true;
const partition=(base:string,name:string)=>{const directory=join(base,name);mkdirSync(directory,{recursive:true});mkdirSync(join(directory,"m"));allocateNativeDirectoryV1({directory,filesystemBytes:33554432,maximumPhysicalBytes:34603008},sensitive);return directory;};
const sh=(...args:string[])=>Bun.spawnSync(args,{stdout:"pipe",stderr:"pipe"});

test("admission matches an independent SQLite inequality and the closed Ajv schema",()=>{
 const db=new Database(":memory:"),admitted=oracle.compile(schema.definitions.Admission);
 try{for(const row of fixture.admission){expect(admitted(row.value),row.id).toBe(true);const expected=db.query("SELECT ?2<=?3 AND ?1+?2<=?5 AND ?4+?2<=?5 AS ok").get(row.value.usedBytes,row.value.requestBytes,row.value.filesystemBytes,row.value.reservedBytes,row.value.maximumPhysicalBytes) as {ok:number};expect(Boolean(expected.ok),row.id).toBe(row.accepted);
  if(row.accepted)expect(admitNativeDirectoryCapacityV1(row.value).remainingBytes,row.id).toBe(row.remainingBytes as number);else expect(()=>admitNativeDirectoryCapacityV1(row.value),row.id).toThrow(protocol.refusals.capacity);}
  for(const value of fixture.malformed){expect(admitted(value),JSON.stringify(value)).toBe(false);expect(()=>admitNativeDirectoryCapacityV1(value)).toThrow("Closed directory capacity request");}}finally{db.close();}
});

test("anchor bytes are canonical, closed and agree with Ajv",()=>{
 oracle.addSchema(schema,"directory");const anchor=oracle.getSchema("directory#/definitions/Anchor")!;
 for(const row of fixture.anchors){const value={version:1,kind:"directory-partition",filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,underlying:row.underlying};expect(anchor(value),row.id).toBe(row.valid);
  if(row.valid){const bytes=nativeDirectoryAnchorBytesV1(value as never);expect(JSON.parse(bytes.toString())).toEqual(value);expect(readNativeDirectoryAnchorV1(bytes)).toEqual({filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,underlying:row.underlying as never});
   expect(()=>readNativeDirectoryAnchorV1(Buffer.from(JSON.stringify(value,null,1)))).toThrow("Canonical directory anchor");expect(()=>readNativeDirectoryAnchorV1(Buffer.from(bytes.toString().replace("{","{\"x\":1,"))) ).toThrow();expect(()=>readNativeDirectoryAnchorV1(Buffer.alloc(protocol.maximumAnchorBytes+1))).toThrow("Bounded directory anchor");}
  else expect(()=>nativeDirectoryAnchorBytesV1(value as never)).toThrow("Closed directory anchor");}
});

test("exchange outcomes agree with the declared refusal table and prove identities moved",()=>{
 const base=scratch();try{
  for(const row of fixture.ports){const a=join(base,row.id+"-a"),b=join(base,row.id+"-b");mkdirSync(a);mkdirSync(b);const before=[observeNativeDirectoryIdentityV1(a),observeNativeDirectoryIdentityV1(b)];
   const port:NativeDirectoryPlatformPortV1=createNativeDirectoryExchangePortV1(row.effect==="swap"?{exchange:(first,second)=>{const hold=first+".held";renameSync(first,hold);renameSync(second,first);renameSync(hold,second);return{status:0,errno:0};}}:row.effect==="nothing"?{exchange:()=>({status:0,errno:0})}:{exchange:()=>({status:row.status!,errno:row.errno!})});
   if(row.effect==="swap"){const after=exchangeNativeDirectoriesV1(a,b,port);expect(after.first,row.id).toEqual(before[1]);expect(after.second,row.id).toEqual(before[0]);}
   else{const unsupported=(row.unsupportedOn as string[]).includes(process.platform);expect(()=>exchangeNativeDirectoriesV1(a,b,port),row.id).toThrow(row.effect==="nothing"?"Ambiguous directory exchange":unsupported?protocol.refusals.unsupported:"Directory exchange failed");expect(observeNativeDirectoryIdentityV1(a),row.id).toEqual(before[0]);}}
  expect(()=>exchangeNativeDirectoriesV1(join(base,"x"),join(base,"x"))).toThrow("Two distinct directories");
  expect(()=>exchangeNativeDirectoriesV1("relative/a","relative/b")).toThrow("Canonical bounded exchange paths");
  mkdirSync(join(base,"f"));writeFileSync(join(base,"file"),"x");symlinkSync(join(base,"f"),join(base,"link"));
  expect(()=>exchangeNativeDirectoriesV1(join(base,"f"),join(base,"file"))).toThrow();expect(()=>exchangeNativeDirectoriesV1(join(base,"f"),join(base,"link"))).toThrow();
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the real kernel exchange swaps two names atomically and an independent python implementation agrees on the state",()=>{
 const base=scratch();try{
  const first=join(base,"first"),second=join(base,"second");mkdirSync(first);mkdirSync(second);writeFileSync(join(first,"marker"),"first");writeFileSync(join(second,"marker"),"second");
  const ino=(path:string)=>Number(sh("stat",process.platform==="darwin"?"-f":"-c",process.platform==="darwin"?"%i":"%i",path).stdout.toString().trim());
  const before=[ino(first),ino(second)],port=nativeDirectorySystemPortV1();
  exchangeNativeDirectoriesV1(first,second,port);
  expect([ino(first),ino(second)]).toEqual([before[1],before[0]]);expect(readFileSync(join(first,"marker"),"utf8")).toBe("second");
  const python=Bun.which("python3");
  if(python){const program=process.platform==="darwin"?"import ctypes,sys;l=ctypes.CDLL('/usr/lib/libSystem.B.dylib');r=l.renameatx_np(-2,sys.argv[1].encode(),-2,sys.argv[2].encode(),2)":"import ctypes,sys;l=ctypes.CDLL(None);r=l.renameat2(-100,sys.argv[1].encode(),-100,sys.argv[2].encode(),2)";
   const run=sh(python,"-c",program+";sys.exit(0 if r==0 else 1)",first,second);
   if(run.exitCode===0){expect([ino(first),ino(second)]).toEqual(before);expect(readFileSync(join(first,"marker"),"utf8")).toBe("first");}}
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("concurrent readers never observe a missing or duplicated name while a child process exchanges repeatedly",async()=>{
 const base=scratch();try{
  const first=join(base,"first"),second=join(base,"second");mkdirSync(first);mkdirSync(second);
  const wanted=new Set([String(lstatSync(first).ino),String(lstatSync(second).ino)]);
  const code=`import {exchangeNativeDirectoriesV1} from ${JSON.stringify(resolve(import.meta.dir,"../🟦️.ts"))};for(let i=0;i<400;i++)exchangeNativeDirectoriesV1(${JSON.stringify(first)},${JSON.stringify(second)});`;
  const child=Bun.spawn([process.execPath,"-e",code],{stdout:"pipe",stderr:"pipe"});let reads=0,missing=0,foreign=0;
  while(child.exitCode===null&&reads<400000){for(let round=0;round<64;round++)for(const path of[first,second]){try{if(!wanted.has(String(lstatSync(path).ino)))foreign++;}catch{missing++;}reads++;}await new Promise(done=>setImmediate(done));}
  expect(await child.exited,await new Response(child.stderr).text()).toBe(0);expect(missing).toBe(0);expect(foreign).toBe(0);expect(reads).toBeGreaterThan(0);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the lifecycle allocate, activate, inactive, deactivate keeps one volume identity and a stable mapping",()=>{
 const base=scratch();try{
  const directory=partition(base,"p"),anchor=readNativeDirectoryAnchorV1(readFileSync(join(directory,protocol.anchor)));
  expect(anchor.filesystemBytes).toBe(33554432);expect(readdirSync(directory).sort()).toEqual([protocol.anchor,protocol.volume,protocol.root].sort());
  expect(runNativeDirectoryActionV1(["inactive",directory])).toBe("");expect(observeNativeDirectoryStateV1(directory)).toBe("inactive");
  const volume=observeNativeDirectoryIdentityV1(join(directory,"v"));
  runNativeDirectoryActionV1(["activate",directory]);expect(runNativeDirectoryActionV1(["inactive",directory])).toBe("attached\n");
  expect(observeNativeDirectoryIdentityV1(join(directory,"m"))).toEqual(volume);const mapping=nativeDirectoryMappingV1(directory);
  writeFileSync(join(directory,"m","journal"),"payload");
  expect(()=>runNativeDirectoryActionV1(["activate",directory])).toThrow("Inactive partition required");
  runNativeDirectoryActionV1(["deactivate",directory]);expect(runNativeDirectoryActionV1(["inactive",directory])).toBe("");expect(readdirSync(join(directory,"m"))).toEqual([]);expect(existsSync(join(directory,"v","journal"))).toBe(true);
  expect(()=>nativeDirectoryMappingV1(directory)).toThrow("Attached directory partition required");
  runNativeDirectoryActionV1(["activate",directory]);expect(nativeDirectoryMappingV1(directory)).toBe(mapping);expect(readFileSync(join(directory,"m","journal"),"utf8")).toBe("payload");
  expect(()=>runNativeDirectoryActionV1(["allocate",directory,"33554432","34603008"],undefined,sensitive)).toThrow();
  expect(()=>runNativeDirectoryActionV1(["format",directory])).toThrow("Closed directory partition action");expect(()=>runNativeDirectoryActionV1(["activate",directory,"extra"])).toThrow("Closed directory partition arguments");
  mkdirSync(join(directory,"other"));renameSync(join(directory,"m"),join(directory,"m.held"));renameSync(join(directory,"other"),join(directory,"m"));expect(observeNativeDirectoryStateV1(directory)).toBe("attached");expect(nativeDirectoryMappingV1(directory)).not.toBe(mapping);rmSync(join(directory,"m"),{recursive:true});renameSync(join(directory,"m.held"),join(directory,"m"));
  renameSync(join(directory,"v"),join(directory,"v.held"));renameSync(join(directory,"m"),join(directory,"m.held"));mkdirSync(join(directory,"m"));mkdirSync(join(directory,"v"));expect(()=>observeNativeDirectoryStateV1(directory)).toThrow("Original underlying placeholder");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("allocation refuses a non-empty placeholder, a case-insensitive host and an existing anchor",()=>{
 const base=scratch();try{
  const dirty=join(base,"dirty");mkdirSync(join(dirty,"m"),{recursive:true});writeFileSync(join(dirty,"m","x"),"");
  expect(()=>allocateNativeDirectoryV1({directory:dirty,filesystemBytes:33554432,maximumPhysicalBytes:34603008},sensitive)).toThrow("Empty original placeholder");
  const insensitive=join(base,"insensitive");mkdirSync(join(insensitive,"m"),{recursive:true});
  expect(()=>allocateNativeDirectoryV1({directory:insensitive,filesystemBytes:33554432,maximumPhysicalBytes:34603008},()=>false)).toThrow(protocol.refusals.caseInsensitive);expect(readdirSync(insensitive)).toEqual(["m"]);
  const twice=partition(base,"twice");expect(()=>allocateNativeDirectoryV1({directory:twice,filesystemBytes:33554432,maximumPhysicalBytes:34603008},sensitive)).toThrow();
  expect(()=>allocateNativeDirectoryV1({directory:join(base,"small"),filesystemBytes:1,maximumPhysicalBytes:2},sensitive)).toThrow();
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the case probe agrees with the shell on the actual host filesystem",()=>{
 const base=scratch();try{
  const probe=probeNativeDirectoryCaseSensitivityV1(base);const directory=join(base,"shell");mkdirSync(directory);
  const run=sh("sh","-c",`cd ${JSON.stringify(directory)} && : > case-a && ( : > CASE-A ) 2>/dev/null; ls | wc -l`);
  expect(Number(run.stdout.toString().trim())===2).toBe(probe);expect(readdirSync(base).filter(name=>name.startsWith(".case-"))).toEqual([]);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("physical usage agrees with du and counts hard links once without following symbolic links",()=>{
 const base=scratch();try{
  const root=join(base,"tree");mkdirSync(root);for(const name of fixture.usage.directories)mkdirSync(join(root,name),{recursive:true});
  for(const[path,size]of fixture.usage.files)writeFileSync(join(root,String(path)),Buffer.alloc(size as number,1));
  linkSync(join(root,"dir/b"),join(root,"dir/b-link"));symlinkSync(join(base),join(root,"escape"));
  const measured=measureNativeDirectoryUsageV1(root),du=sh("du","-sk",root),kilobytes=Number(du.stdout.toString().split(/\s/u)[0]);
  expect(Math.ceil(measured.allocatedBytes/1024)).toBe(kilobytes);
  expect(measured.logicalBytes).toBe(1+5000+100000);expect(()=>measureNativeDirectoryUsageV1(root,3)).toThrow("entry bound");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("nested partitions are listed only when attached and within the declared depth",()=>{
 const base=scratch();try{
  const root=join(base,"parent");mkdirSync(root);
  const make=(path:string,attach:boolean)=>{const directory=join(root,path);mkdirSync(join(directory,"m"),{recursive:true});allocateNativeDirectoryV1({directory,filesystemBytes:33554432,maximumPhysicalBytes:34603008},sensitive);if(attach)activateNativeDirectoryV1(directory);return directory;};
  for(const path of fixture.children.attached)make(path,true);for(const path of fixture.children.inactive)make(path,false);for(const path of fixture.children.tooDeep)make(path,true);mkdirSync(join(root,fixture.children.plain[0]!),{recursive:true});
  const listed=listNativeDirectoryChildrenV1(root);expect(listed).toEqual(fixture.children.attached.map(path=>join(root,path,"m")));
  expect(()=>listNativeDirectoryChildrenV1(root,2)).toThrow("entry bound");
  deactivateNativeDirectoryV1(join(root,fixture.children.attached[0]!));expect(listNativeDirectoryChildrenV1(root)).toEqual([join(root,fixture.children.attached[1]!,"m")]);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the owned child process runs the same closed actions and refuses on a case-insensitive host with an exit code",()=>{
 const base=scratch();try{
  const directory=join(base,"cli");mkdirSync(join(directory,"m"),{recursive:true});
  const run=(...args:string[])=>Bun.spawnSync([process.execPath,script,...args],{stdout:"pipe",stderr:"pipe"});
  const allocate=run("allocate",directory,"33554432","34603008");
  if(!probeNativeDirectoryCaseSensitivityV1(base)){expect(allocate.exitCode).toBe(1);expect(allocate.stderr.toString()).toContain(protocol.refusals.caseInsensitive);return;}
  expect(allocate.exitCode).toBe(0);expect(run("inactive",directory).stdout.toString()).toBe("");expect(run("activate",directory).exitCode).toBe(0);expect(run("inactive",directory).stdout.toString()).toBe("attached\n");
  const again=run("activate",directory);expect(again.exitCode).toBe(1);expect(again.stderr.toString()).toContain("Inactive partition required");expect(run("deactivate",directory).exitCode).toBe(0);expect(run("inactive",directory).stdout.toString()).toBe("");expect(run("nonsense",directory).exitCode).toBe(1);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the exchange probe leaves no scratch directory and the schema declares both kernel symbols",()=>{
 const base=scratch();try{
  probeNativeDirectoryExchangeV1(base);expect(readdirSync(base).filter(name=>name.startsWith(".exchange-"))).toEqual([]);
  expect(()=>probeNativeDirectoryExchangeV1(base,createNativeDirectoryExchangePortV1({exchange:()=>({status:-1,errno:22})}))).toThrow(protocol.refusals.unsupported);expect(readdirSync(base).filter(name=>name.startsWith(".exchange-"))).toEqual([]);
  expect(protocol.exchange.linux.symbol).toBe("renameat2");expect(protocol.exchange.darwin.symbol).toBe("renameatx_np");expect(protocol.exchange.linux.flag).toBe(2);expect(protocol.exchange.darwin.flag).toBe(2);
  expect(createHash("sha256").update(JSON.stringify(protocol.actions)).digest("hex").length).toBe(64);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the meter charges every admission against the cap: many small writes, one large write and growth after commit are all refused, an honest writer never is",()=>{
 for(const row of fixture.meter){
  let written=0,clock=0;const counts={reserve:0,commit:0,recheck:0},refused:Record<"reserve"|"commit"|"recheck",number|null>={reserve:null,commit:null,recheck:null};
  const meter=createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,measure:()=>written,now:()=>clock});
  const attempt=(mode:"reserve"|"commit"|"recheck",requestBytes:number)=>{counts[mode]++;try{meter.admit({mode,requestBytes});return true;}catch(error){expect((error as Error).message,row.id).toBe(protocol.refusals.capacity);refused[mode]=counts[mode];return false;}};
  run:for(const step of row.steps as readonly {repeat:number;reserve:number;write:number;free?:boolean;commit?:boolean;recheck?:boolean;advanceMilliseconds?:number}[])for(let index=0;index<step.repeat;index++){
   if(step.free){written=0;continue;}
   if(step.reserve>0&&!attempt("reserve",step.reserve))break run;
   written+=step.write;
   if(step.commit&&!attempt("commit",0))break run;
   if(step.recheck){clock+=step.advanceMilliseconds??0;if(!attempt("recheck",0))break run;}
  }
  expect(refused.reserve,row.id).toBe((row as {refusedAtReserve?:number|null}).refusedAtReserve??null);expect(refused.commit,row.id).toBe((row as {refusedAtCommit?:number|null}).refusedAtCommit??null);expect(refused.recheck,row.id).toBe((row as {refusedAtRecheck?:number|null}).refusedAtRecheck??null);
 }
 for(const value of fixture.meterMalformed)expect(()=>createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:33554432,maximumPhysicalBytes:34603008,measure:()=>0}).admit(value),JSON.stringify(value)).toThrow("Closed directory meter request");
 expect(()=>createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:0,maximumPhysicalBytes:1})).toThrow("Closed directory meter limits");
});

test("the meter over a real tree refuses many small files and one large file that bypassed the old sixteen MiB gate",()=>{
 const base=scratch();try{
  const small=join(base,"small");mkdirSync(small);const meter=createNativeDirectoryMeterV1({root:small,filesystemBytes:33554432,maximumPhysicalBytes:34603008});let refusedAt=0;
  for(let index=1;index<=70&&!refusedAt;index++){try{meter.admit({mode:"reserve",requestBytes:524288});writeFileSync(join(small,"f"+index),Buffer.alloc(524288,index));}catch(error){expect((error as Error).message).toBe(protocol.refusals.capacity);refusedAt=index;}}
  expect(refusedAt).toBeGreaterThan(55);expect(refusedAt).toBeLessThanOrEqual(67);
  const large=join(base,"large");mkdirSync(large);const second=createNativeDirectoryMeterV1({root:large,filesystemBytes:33554432,maximumPhysicalBytes:34603008});
  second.admit({mode:"reserve",requestBytes:1048576});writeFileSync(join(large,"big"),Buffer.alloc(67108864,1));
  expect(()=>second.admit({mode:"commit",requestBytes:0})).toThrow(protocol.refusals.capacity);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("every platform port honours one swap contract: sibling swap, identity proof, flush, no pending state and a case-sensitive volume",()=>{
 const base=scratch();try{
  const ports:[string,NativeDirectoryPlatformPortV1][]=[["system",nativeDirectorySystemPortV1()],["exchange-simulated",createNativeDirectoryExchangePortV1({exchange:(first,second)=>{const hold=first+".held";renameSync(first,hold);renameSync(second,first);renameSync(hold,second);return{status:0,errno:0};}})]];
  for(const[name,port]of ports){
   const directory=join(base,name);mkdirSync(join(directory,"m"),{recursive:true});allocateNativeDirectoryV1({directory,filesystemBytes:33554432,maximumPhysicalBytes:34603008},sensitive,port);
   expect(nativeDirectorySwapPendingV1(directory,port),name).toBe(false);expect(recoverNativeDirectoryV1(directory,port),name).toBe("clean");
   activateNativeDirectoryV1(directory,port);writeFileSync(join(directory,"m","marker"),"payload");expect(observeNativeDirectoryStateV1(directory,port),name).toBe("attached");
   deactivateNativeDirectoryV1(directory,port);expect(readFileSync(join(directory,"v","marker"),"utf8"),name).toBe("payload");expect(readdirSync(directory).sort(),name).toEqual([protocol.anchor,protocol.volume,protocol.root].sort());
   expect(probeNativeDirectoryCaseCapabilityV1(base,port),name).toBe(probeNativeDirectoryCaseSensitivityV1(base));
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});


test("directory meter separates metadata, fresh entitlement and recovered committed content",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.meterLifecycle){let written=row.initialBytes,clock=0;const create=()=>createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:row.filesystemBytes,maximumPhysicalBytes:row.maximumPhysicalBytes,measure:()=>written,now:()=>clock});let meter=create();for(const step of row.steps as readonly {mode:"write"|"reopen"|"reserve"|"commit"|"recheck";bytes?:number;accepted?:boolean}[]){if(step.mode==="write"){written+=step.bytes!;continue;}if(step.mode==="reopen"){meter=create();clock+=2000;continue;}const request=step.bytes!;let accepted=true;try{meter.admit({mode:step.mode,requestBytes:request});}catch(error){expect((error as Error).message,row.id).toBe(protocol.refusals.capacity);accepted=false;}const physical=db.query("SELECT ?1+?2<=?3 AND ?2<=?4 accepted").get(written,request,row.maximumPhysicalBytes,row.filesystemBytes)as{accepted:number};expect(accepted,row.id).toBe(step.accepted!);expect(accepted,row.id).toBe(Boolean(physical.accepted));}}}finally{db.close();}
});


import{nativeStorageEventV1,nativeStorageStateV1}from"../../📜️journal/🟦️.ts";
test("directory payload admission leaves the actual durable journal physical ceiling unchanged",()=>{
 const row=fixture.journalCapacity,db=new Database(":memory:");try{let state=nativeStorageStateV1(),charged=0;for(const[index,bytes]of row.requests.entries()){const allocation={id:String(index+1).padStart(32,"0"),owner:"fixture:directory-charge",kind:"artifact-publication"as const,bytes,phase:"reserved"as const},expected=Boolean((db.query("SELECT ?1+?2<=?3 accepted").get(charged,bytes,row.maximumPhysicalBytes)as{accepted:number}).accepted);expect(expected).toBe(row.accepted[index]!);if(expected){state=nativeStorageEventV1(state,{type:"reserved",allocation},row.maximumPhysicalBytes);charged+=bytes;}else expect(()=>nativeStorageEventV1(state,{type:"reserved",allocation},row.maximumPhysicalBytes)).toThrow("Native storage capacity exhausted");}expect(charged).toBe(row.maximumPhysicalBytes);}finally{db.close();}
});
