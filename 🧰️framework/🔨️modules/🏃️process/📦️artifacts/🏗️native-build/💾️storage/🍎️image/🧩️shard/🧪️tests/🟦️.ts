/** 🧩️ Tests for the sharded images of a cache partition: selection against an independent oracle, aggregation, the ledger, per-image reclaim with lease isolation, the path budget and, on macOS, two real scratch shards whose freed blocks return one by one. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{closeSync,existsSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync,writeSync}from"node:fs";
import{join,posix,win32}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{acquireResourceLease}from"../../../../../../🔒️leases/🟦️.ts";
import{createNativeImageDarwinKickPortV1,type NativeImageKickPortV1}from"../../👟️kick/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{appendNativeImageShardEventV1,createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,measureNativeImageShardFreeV1,measureNativeImageShardRootV1,measureNativeImageShardV1,nativeImageShardPartitionOfV1,nativeImageShardPathV1,nativeImageShardVolumeFactsV1,nativeImageShardAdmissionV1,nativeImageShardAggregateV1,nativeImageShardBirthEventV1,nativeImageShardCapBytesV1,nativeImageShardCapacityV1,nativeImageShardFoldV1,nativeImageShardHomeLayoutV1,nativeImageShardIdV1,nativeImageShardKeyV1,nativeImageShardLayoutV1,nativeImageShardLayoutsV1,nativeImageShardMountedV1,nativeImageShardMountPointV1,nativeImageShardRowV1,nativeImageShardPathBudgetV1,nativeImageShardPartialTextV1,nativeImageShardReclaimDueV1,nativeImageShardRefusedTextV1,nativeImageShardSelectV1,nativeImageShardSupportedV1,readNativeImageShardLedgerV1,reclaimNativeImageShardsV1,repairNativeImageShardsV1,requireNativeImageShardRuleV1,retireNativeImageShardV1,NATIVE_IMAGE_SHARD_V1,type NativeImageShardBornEventV1,type NativeImageShardEventV1,type NativeImageShardLayoutV1,type NativeImageShardRuleV1,type NativeImageShardStateV1}from"../🟦️.ts";
const MiB=1048576,GiB=1073741824,env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const rule=corpus.rule as NativeImageShardRuleV1;
const same=(actual:unknown,expected:unknown)=>expect(actual).toEqual(expected);
const oracle=(...args:string[])=>{
 const python=testPythonV1();if(python===null)throw Error("Python required by the shard oracle");
 const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"});
 if(result.status!==0)throw result.error??Error(result.stderr);return result.stdout.trim();
};
const answers=JSON.parse(oracle("answers",join(import.meta.dir,"🔣️.json")));
const stateOf=(rows:readonly{id:string;key:string;root:number;path?:string;capacityBytes:number}[]):NativeImageShardStateV1=>new Map(rows.map(row=>[row.id,{...row,path:row.path??rule.roots[row.root]!.prefix+"/"+row.key}]));
test("the corpus is schema-valid and the declared rule is closed",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,corpus,schema)).toEqual([]);expect(requireNativeImageShardRuleV1(corpus.rule)).toEqual(corpus.rule);
});
test("the shard key and id of every corpus path equal the independent oracle and a few hand-checked answers",()=>{
 const ours=corpus.keys.map(row=>{const found=nativeImageShardKeyV1(rule,row.path);return{path:row.path,root:found?.root??null,key:found?.key??null,id:found?.id??null};});
 expect(ours).toEqual(answers.keys);expect(ours).toEqual(corpus.keys);
 const by=(path:string)=>ours.find(row=>row.path===path)!;
 expect(by("u/l/pcab777/t/x")).toEqual({path:"u/l/pcab777/t/x",root:0,key:"pcab777",id:"pcab777"});
 expect(by("u/l").key).toBeNull();expect(by("u/l/UPPER").key).toBeNull();expect(by("u/l/abcdefghijklm").key).toBeNull();expect(by("cargo/home").key).toBeNull();expect(by("../u/l/abc").key).toBeNull();
 expect(by("j/0123456789abcdef0123/node_modules").id).toBe(oracle("id","0123456789abcdef0123"));expect(by("j/0123456789abcdef0123/node_modules").id).toMatch(/^x[0-9a-f]{11}$/u);
 expect(nativeImageShardIdV1("0123456789abcdef0123")).toBe("x1bd8fd47297");
});
test("the capacity at birth equals the oracle: the cap and the host share bound it, the working set with headroom floors it",()=>{
 const ours=corpus.capacity.map(row=>{const made=nativeImageShardCapacityV1(row.platform,rule,{needBytes:row.need,hostFreeBytes:row.hostFreeBytes});return"capacityBytes"in made?made.capacityBytes:null;});
 expect(ours).toEqual(answers.capacity);expect(ours).toEqual(corpus.capacity.map(row=>row.capacityBytes));
 expect(nativeImageShardCapBytesV1("darwin")).toBe(2**35);expect(nativeImageShardCapBytesV1("linux")).toBe(128*GiB);expect(nativeImageShardCapBytesV1("win32")).toBe(64*GiB);expect(nativeImageShardCapBytesV1("freebsd")).toBe(0);
 expect(nativeImageShardCapBytesV1("darwin",{maximumImageBytes:8*GiB})).toBe(8*GiB);expect(nativeImageShardCapBytesV1("darwin",{maximumImageBytes:99*GiB})).toBe(2**35);
 expect(nativeImageShardCapacityV1("darwin",rule,{needBytes:8*GiB,hostFreeBytes:20*GiB})).toEqual({capacityBytes:13*GiB});
 expect(nativeImageShardCapacityV1("darwin",rule,{needBytes:40*GiB,hostFreeBytes:900*GiB})).toEqual({refused:"cap"});expect(nativeImageShardCapacityV1("darwin",rule,{needBytes:8*GiB,hostFreeBytes:10*GiB})).toEqual({refused:"host-free"});
});
test("the macOS cap is the measured limit of the trim: no image of any declared platform and rule exceeds what its platform can return",()=>{
 const limit=NATIVE_IMAGE_SHARD_V1.maximumImageBytes;expect(limit.darwin).toBe(34359738368);
 for(const row of corpus.capacity){const made=nativeImageShardCapacityV1(row.platform,rule,{needBytes:row.need,hostFreeBytes:row.hostFreeBytes});if("capacityBytes"in made)expect(made.capacityBytes).toBeLessThanOrEqual(nativeImageShardCapBytesV1(row.platform));}
 const total=96*GiB,shards=Math.ceil(total/limit.darwin);expect(shards).toBe(3);expect(Math.ceil(36*GiB/limit.darwin)).toBe(2);
});
test("shard selection equals the oracle on every case: disabled rules and unsupported platforms stay home, a live key is reused, a new key is born, collisions and host shortage are refused",()=>{
 const ours=corpus.selection.map(row=>nativeImageShardSelectV1({rule:{...rule,enabled:row.enabled},platform:row.platform,live:stateOf(row.live),path:row.path,hostFreeBytes:row.hostFreeBytes}));
 expect(ours).toEqual(answers.selection);same(ours,corpus.selection.map(row=>row.decision));
 expect(ours.map(row=>row.kind)).toEqual(["existing","birth","refused","home","home","home","refused","home","birth"]);
 expect(nativeImageShardSupportedV1("darwin")).toBe(true);expect(nativeImageShardSupportedV1("linux")).toBe(false);expect(nativeImageShardSupportedV1("win32")).toBe(false);
 const full=stateOf(Array.from({length:256},(_,index)=>({id:"a"+index.toString(36).padStart(3,"0"),key:"a"+index.toString(36).padStart(3,"0"),root:0,capacityBytes:GiB})));
 expect(nativeImageShardSelectV1({rule,platform:"darwin",live:full,path:"u/l/zzzz",hostFreeBytes:900*GiB})).toMatchObject({kind:"refused",reason:"shard-limit"});
 expect(nativeImageShardSelectV1({rule,platform:"darwin",live:full,path:"u/l/a001/x",hostFreeBytes:900*GiB})).toMatchObject({kind:"existing"});
});
test("selection is stable: the same key always selects the same image whatever its path below, and different keys select different images",()=>{
 const keys=["pcab777","pec11e9","abc","a0b1c2d3e4f5"],ids=new Set<string>();
 for(const key of keys){const found=new Set(["","/t","/t/x86_64/debug","/lane.json"].map(tail=>nativeImageShardKeyV1(rule,"u/l/"+key+tail)?.id));expect(found.size).toBe(1);ids.add([...found][0]!);}
 expect(ids.size).toBe(keys.length);
 for(let n=0;n<500;n++){const key=(n*2654435761>>>0).toString(36).slice(0,12);expect(nativeImageShardKeyV1(rule,"u/l/"+key+"/x")?.id).toBe(key);}
 const hashed=Array.from({length:500},(_,n)=>(n*40503>>>0).toString(16).padStart(20,"0"));expect(hashed.map(key=>nativeImageShardKeyV1(rule,"j/"+key)?.id)).toEqual(JSON.parse(oracle("ids",...hashed)));
});
test("a rule with overlapping roots or an unanchored pattern is refused",()=>{
 const root=(prefix:string,pattern="^[a-z]+$")=>({prefix,pattern,needBytes:GiB});
 const make=(roots:ReturnType<typeof root>[])=>({enabled:true,headroomPercent:0,hostFreeReservePercent:50,roots});
 expect(()=>requireNativeImageShardRuleV1(make([root("u/l"),root("u")]))).toThrow("Disjoint shard roots");
 expect(()=>requireNativeImageShardRuleV1(make([root(""),root("j")]))).toThrow("Disjoint shard roots");
 expect(()=>requireNativeImageShardRuleV1(make([root("u/l","[a-z]+")]))).toThrow("Anchored shard key pattern");
 expect(requireNativeImageShardRuleV1(make([root("u/l"),root("u","^(sealed|w)$"),root("","^(cargo|components)$")])).roots.length).toBe(3);
 expect(()=>requireNativeImageShardRuleV1({...make([root("u/l")]),extra:1})).toThrow("Closed image shard rule");
 expect(requireNativeImageShardRuleV1(make([root("u/l"),root("j")])).roots.length).toBe(2);
});
test("the layout mounts every shard at the logical path of its key, so no path below it changes or grows, and every image has its own lease",()=>{
 const directory="/ws/.cache/b/r",layouts=nativeImageShardLayoutsV1(directory,stateOf([{id:"pcab777",key:"pcab777",root:0,capacityBytes:GiB},{id:"x1bd8fd47297",key:"0123456789abcdef0123",root:1,capacityBytes:GiB}]),"posix");
 expect(layouts.map(row=>row.id)).toEqual(["home","pcab777","x1bd8fd47297"]);
 const logical=(path:string)=>posix.join(directory,"m",path);
 expect(layouts[1]!.mountPoint).toBe(logical("u/l/pcab777"));expect(layouts[2]!.mountPoint).toBe(logical("j/0123456789abcdef0123"));expect(layouts[0]!.mountPoint).toBe(logical(""));
 const file="u/l/pcab777/t/x86_64-unknown-linux-gnu/debug/deps/libx.rlib";expect(posix.join(layouts[1]!.mountPoint,file.split("/").slice(3).join("/")).length).toBe(logical(file).length);
 expect(new Set(layouts.map(row=>row.leaseResource)).size).toBe(3);expect(layouts[0]!.leaseResource).toBe("native-fixed-image:"+directory);
 expect(layouts[1]!.leaseResource).toBe("native-fixed-image:"+directory+"/s/pcab777");expect(layouts[1]!.image).toBe(directory+"/s/pcab777/fixed.dmg");
 expect(nativeImageShardLayoutV1("C:\\ws\\.cache\\b\\r",{id:"pcab777",key:"pcab777",path:"u/l/pcab777"},"win32").mountPoint).toBe("C:\\ws\\.cache\\b\\r\\m\\u\\l\\pcab777");
});
test("the path budget of the files a shard adds equals the oracle and every fitting directory keeps every owned path within 256 characters",()=>{
 const ours=corpus.paths.map(row=>{const budget=nativeImageShardPathBudgetV1(row.directory,row.id,row.flavor as"posix"|"win32");return{flavor:row.flavor,directory:row.directory,id:row.id,longest:budget.longest,maximumDirectoryCharacters:budget.maximumDirectoryCharacters,fits:budget.fits};});
 expect(ours).toEqual(corpus.paths);expect(ours.map(({longest,maximumDirectoryCharacters,fits})=>({longest,maximumDirectoryCharacters,fits}))).toEqual(answers.paths);
 for(const row of ours)expect(row.fits).toBe(row.directory.length<=row.maximumDirectoryCharacters);
 for(const flavor of["posix","win32"]as const){
  const path=flavor==="win32"?win32:posix,max=nativeImageShardPathBudgetV1(path.join(path.sep,"x"),"pcab777",flavor).maximumDirectoryCharacters;
  for(const id of["a","pcab777","x1bd8fd47297"]){const edge=path.join(path.sep,"a".repeat(nativeImageShardPathBudgetV1(path.sep+"x",id,flavor).maximumDirectoryCharacters-1));expect(nativeImageShardPathBudgetV1(edge,id,flavor).fits).toBe(true);expect(nativeImageShardPathBudgetV1(edge,id,flavor).longest).toBeLessThanOrEqual(256);expect(nativeImageShardPathBudgetV1(edge+"a",id,flavor).fits).toBe(false);expect(nativeImageShardPathBudgetV1(edge+"a",id,flavor).longest).toBe(257);}
  expect(max).toBeGreaterThan(200);
 }
 expect(NATIVE_IMAGE_SHARD_V1.pathBudgetCharacters).toBe(256);
});
test("aggregation adds the images up and counts the free bytes of one host volume once, equal to the oracle",()=>{
 const ours=corpus.aggregate.map(row=>nativeImageShardAggregateV1(row.rows));expect(ours).toEqual(answers.aggregate);expect(ours).toEqual(corpus.aggregate.map(row=>row.total));
 const shared=corpus.aggregate[1]!.total;expect(shared.freeBytes).toBe(30*GiB);expect(shared.capacityBytes).toBe(64*GiB);expect(shared.slackBytes).toBe(23*GiB);
 expect(corpus.aggregate[3]!.total.freeBytes).toBe(15*GiB+20*GiB);expect(corpus.aggregate[4]!.total.slackBytes).toBe(0);expect(corpus.aggregate[5]!.total.shards).toBe(0);
});
test("the upfront admission over all images equals the oracle: slack is credited, a new shard adds its birth allocation, volumes are judged separately",()=>{
 const ours=corpus.admission.map(row=>{const made=nativeImageShardAdmissionV1(row.rows,row.reserveBytes,row.slackCreditBasisPoints);return{volumes:made.volumes,shortfallBytes:made.shortfallBytes};});
 expect(ours).toEqual(answers.admission);same(ours,corpus.admission.map(row=>({volumes:row.volumes,shortfallBytes:row.shortfallBytes})));
 expect(corpus.admission[0]!.shortfallBytes).toBe(0);expect(corpus.admission[2]!.shortfallBytes).toBe(NATIVE_IMAGE_SHARD_V1.birthAllocationBytes);expect(corpus.admission[3]!.shortfallBytes).toBe(9*GiB);expect(corpus.admission[4]!.shortfallBytes).toBe(0);
});
test("the due decision of a reclaim pass equals the oracle for every image",()=>{
 const ours=corpus.reclaim.map(row=>nativeImageShardReclaimDueV1(row.rule,row.facts));expect(ours).toEqual(answers.reclaim);expect(ours).toEqual(corpus.reclaim.map(row=>row.due));
});
const born=(key:string,at=1,root=0,capacityBytes=GiB):NativeImageShardBornEventV1=>({type:"born",id:nativeImageShardIdV1(key),key,root,path:"u/l/"+key,capacityBytes,atMs:at});
test("the event log folds into the live shards: births are idempotent, a foreign key under a live id and the retirement of an unknown shard are errors",()=>{
 expect([...nativeImageShardFoldV1([born("abc"),born("def"),{type:"retired",id:"abc",atMs:3}]).keys()]).toEqual(["def"]);
 expect(nativeImageShardFoldV1([born("abc"),born("abc",2)]).size).toBe(1);
 expect(()=>nativeImageShardFoldV1([born("abc"),{...born("zzz"),id:"abc"}])).toThrow("belongs to abc");expect(()=>nativeImageShardFoldV1([{type:"retired",id:"abc",atMs:1}])).toThrow("Live image shard required");
 expect(()=>nativeImageShardFoldV1([{type:"born",id:"A",key:"k",root:0,path:"u/l/k",capacityBytes:GiB,atMs:1}]as never)).toThrow("Closed image shard event");expect(nativeImageShardFoldV1([born("abc"),{type:"retired",id:"abc",atMs:3},born("abc",4)]).size).toBe(1);
});
test("a birth event is validated against the rule, the cap and the limits, and the ledger file survives a torn last line",()=>{
 const live=nativeImageShardFoldV1([born("abc")]),input={rule,platform:"darwin",live,key:"def",root:0,capacityBytes:32*GiB,atMs:7};
 expect(nativeImageShardBirthEventV1(input)).toEqual({type:"born",id:"def",key:"def",root:0,path:"u/l/def",capacityBytes:32*GiB,atMs:7});
 expect(()=>nativeImageShardBirthEventV1({...input,key:"abc"})).toThrow("live already");expect(()=>nativeImageShardBirthEventV1({...input,capacityBytes:33*GiB})).toThrow("within the cap");expect(()=>nativeImageShardBirthEventV1({...input,capacityBytes:GiB+1})).toThrow("within the cap");
 expect(()=>nativeImageShardBirthEventV1({...input,capacityBytes:GiB/2})).toThrow("within the cap");expect(()=>nativeImageShardBirthEventV1({...input,key:"UPPER"})).toThrow("declared root");expect(()=>nativeImageShardBirthEventV1({...input,root:1})).toThrow("declared root");
 expect(nativeImageShardBirthEventV1({...input,platform:"linux",capacityBytes:100*GiB}).capacityBytes).toBe(100*GiB);
 const directory=mkdtempSync(join(testScratchRootV1("shard"),"ledger-"));
 try{
  expect(readNativeImageShardLedgerV1(directory)).toEqual([]);appendNativeImageShardEventV1(directory,born("abc"));appendNativeImageShardEventV1(directory,born("def",2));
  writeFileSync(join(directory,"shards.jsonl"),readFileSync(join(directory,"shards.jsonl"),"utf8")+"{\"type\":\"born\",\"id\":\"gh");
  expect(readNativeImageShardLedgerV1(directory).map(row=>row.type==="born"?row.key:"")).toEqual(["abc","def"]);
  expect(()=>appendNativeImageShardEventV1(directory,{...born("abc"),key:"k".repeat(2000)})).toThrow("line bound");
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("the volume the collector sees for a sharded partition is the budget, the used bytes of all images and the honest free bytes, and a path outside a sharded partition is not measured",()=>{
 expect(nativeImageShardVolumeFactsV1({capacityBytes:100*GiB,usedBytes:20*GiB,freeBytes:70*GiB},96*GiB)).toEqual({volumeBytes:96*GiB,freeBytes:70*GiB});
 expect(nativeImageShardVolumeFactsV1({capacityBytes:100*GiB,usedBytes:20*GiB,freeBytes:90*GiB},96*GiB)).toEqual({volumeBytes:96*GiB,freeBytes:76*GiB});
 expect(nativeImageShardVolumeFactsV1({capacityBytes:100*GiB,usedBytes:20*GiB,freeBytes:80*GiB},1)).toEqual({volumeBytes:100*GiB,freeBytes:80*GiB});
 expect(nativeImageShardVolumeFactsV1({capacityBytes:33*GiB,usedBytes:40*GiB,freeBytes:0},96*GiB)).toEqual({volumeBytes:33*GiB,freeBytes:0});
 const directory=mkdtempSync(join(testScratchRootV1("shard"),"root-"));
 try{
  mkdirSync(join(directory,"m/u"),{recursive:true});expect(measureNativeImageShardRootV1(join(directory,"m/u"))).toBeUndefined();
  appendNativeImageShardEventV1(directory,born("abc"));expect(nativeImageShardPartitionOfV1(join(directory,"m/u"))).toBe(directory);expect(measureNativeImageShardRootV1(join(directory,"m/u"))).toBeUndefined();
  expect(measureNativeImageShardFreeV1(nativeImageShardLayoutsV1(directory,nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)))[1]!)).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("the refusal and the partial-failure lines exist in both languages",()=>{
 const values={key:"pcab777",partition:"rust",free:"3 GiB",need:"9 GiB"};
 expect(nativeImageShardRefusedTextV1("en",values)).toBe("No image can be born for pcab777: the host volume of rust has 3 GiB free, a shard needs 9 GiB.");
 expect(nativeImageShardRefusedTextV1("de",values)).toBe("Für pcab777 kann kein Image angelegt werden: das Hostvolume von rust hat 3 GiB frei, ein Shard braucht 9 GiB.");
 expect(nativeImageShardPartialTextV1("en",{failed:"1",total:"3",partition:"rust",ids:"abc"})).toContain("1 of 3 shards of rust failed (abc)");expect(nativeImageShardPartialTextV1("de",{failed:"1",total:"3",partition:"rust",ids:"abc"})).toContain("1 von 3 Shards von rust");
});
const reclaimRule={reclaimFloorBytes:GiB,reclaimRatioPercent:150,reclaimTightHostFreeBytes:0};
const leaseTable=()=>{const exclusive=new Set<string>(),shared=new Map<string,number>();return{exclusive,shared,lease:async(layout:NativeImageShardLayoutV1)=>{if(exclusive.has(layout.leaseResource))return undefined;shared.set(layout.leaseResource,(shared.get(layout.leaseResource)??0)+1);return()=>shared.set(layout.leaseResource,shared.get(layout.leaseResource)!-1);}};};
const fakePort=(log:string[],id:string,before:number,after:number,effective=true):NativeImageKickPortV1=>{let size=before;return{async ensure(){log.push("ensure "+id);},async kick(){log.push("kick "+id);size=after;},allocatedBytes:()=>size,async dispose(){log.push("dispose "+id);},canary:()=>effective?{bytes:256*MiB,allocatedAfterWrite:before+256*MiB}:undefined};};
const dueFacts={allocatedBytes:10*GiB,usedBytes:2*GiB,hostFreeBytes:50*GiB,sinceLastKickMilliseconds:10**9};
const layouts=nativeImageShardLayoutsV1("/ws/.cache/b/r",stateOf([{id:"aaa",key:"aaa",root:0,capacityBytes:GiB},{id:"bbb",key:"bbb",root:0,capacityBytes:GiB},{id:"ccc",key:"ccc",root:0,capacityBytes:GiB}]),"posix");
test("a pass kicks each due image under its own lease: a held lease, a failure and an undue image never stop the others, and the totals add up over the kicked ones",async()=>{
 const log:string[]=[],table=leaseTable();table.exclusive.add(layouts[2]!.leaseResource);
 const facts:Record<string,typeof dueFacts|undefined>={home:{...dueFacts,allocatedBytes:GiB,usedBytes:GiB},aaa:dueFacts,bbb:dueFacts,ccc:dueFacts};
 const sizes:Record<string,[number,number]>={aaa:[10*GiB,4*GiB],bbb:[10*GiB,1*GiB],ccc:[7*GiB,7*GiB]};
 const steps:number[]=[];
 const outcome=await reclaimNativeImageShardsV1({layouts,rule:reclaimRule,hooks:{check:()=>{},sleep:async()=>{},progress:done=>steps.push(done)},ports:{measure:layout=>facts[layout.id],lease:table.lease,port:layout=>layout.id==="home"?null:fakePort(log,layout.id,...sizes[layout.id]!,layout.id==="aaa"),kicked:layout=>log.push("kicked "+layout.id)}});
 expect(outcome.rows.map(row=>[row.id,row.status])).toEqual([["home","not-due"],["aaa","kicked"],["bbb","lease-held"],["ccc","kicked"]]);
 expect(log).toEqual(["ensure aaa","kick aaa","dispose aaa","kicked aaa","ensure ccc","kick ccc","dispose ccc","kicked ccc"]);
 expect(outcome.beforeBytes).toBe(17*GiB);expect(outcome.afterBytes).toBe(11*GiB);expect(outcome.effective).toBe(true);expect(outcome.settled).toBe(true);
 expect([...table.shared.values()].every(count=>count===0)).toBe(true);expect(steps.at(-1)).toBe(100);expect(steps).toEqual([...steps].sort((a,b)=>a-b));
});
test("one image failing its kick is reported and kept apart: the others are served and cancellation still propagates at once",async()=>{
 const table=leaseTable(),broken:NativeImageKickPortV1={...fakePort([],"bbb",GiB,GiB),async kick(){throw Error("busy");}};
 const outcome=await reclaimNativeImageShardsV1({layouts:layouts.slice(1),rule:reclaimRule,hooks:{check:()=>{},sleep:async()=>{}},ports:{measure:()=>dueFacts,lease:table.lease,port:layout=>layout.id==="bbb"?broken:fakePort([],layout.id,10*GiB,2*GiB)}});
 expect(outcome.rows.map(row=>row.status)).toEqual(["kicked","failed","kicked"]);expect((outcome.rows[1]!.error as Error).message).toBe("busy");expect(outcome.effective).toBe(true);expect([...table.shared.values()].every(count=>count===0)).toBe(true);
 let seen=0;await expect(reclaimNativeImageShardsV1({layouts,rule:reclaimRule,hooks:{check:()=>{if(++seen>3)throw Error("cancelled");},sleep:async()=>{}},ports:{measure:()=>dueFacts,lease:table.lease,port:layout=>fakePort([],layout.id,10*GiB,2*GiB)}})).rejects.toThrow("cancelled");
 expect([...table.shared.values()].every(count=>count===0)).toBe(true);
 const none=await reclaimNativeImageShardsV1({layouts:layouts.slice(1),rule:reclaimRule,hooks:{check:()=>{}},ports:{measure:()=>undefined,lease:table.lease,port:()=>null}});expect(none.rows.map(row=>row.status)).toEqual(["unmeasured","unmeasured","unmeasured"]);expect(none.effective).toBe(false);
});
test("the leases of different images are independent: an exclusive cycle of one image never blocks the kick of another, equal to the real lease table",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("shard"),"lease-")),signal=new AbortController().signal;
 try{
  const hold=await acquireResourceLease({directory,resource:layouts[1]!.leaseResource,mode:"exclusive",signal,timeoutMs:0});
  try{
   const other=await acquireResourceLease({directory,resource:layouts[2]!.leaseResource,mode:"shared",signal,timeoutMs:0});other.release();
   const home=await acquireResourceLease({directory,resource:layouts[0]!.leaseResource,mode:"shared",signal,timeoutMs:0});home.release();
   await expect(acquireResourceLease({directory,resource:layouts[1]!.leaseResource,mode:"shared",signal,timeoutMs:0})).rejects.toThrow();
  }finally{hold.release();}
  const again=await acquireResourceLease({directory,resource:layouts[1]!.leaseResource,mode:"shared",signal,timeoutMs:0});again.release();
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a live shard is recognised by its logical mount point and its own backing alone, a retired, foreign or misplaced image never is, equal to an independent path replay",()=>{
 const directory=mkdtempSync(join(testScratchRootV1("shard"),"rows-"));
 try{
  const born=(id:string,path:string):NativeImageShardBornEventV1=>({type:"born",id,key:id,root:0,path,capacityBytes:GiB,atMs:1});
  appendNativeImageShardEventV1(directory,born("cargo","cargo"));appendNativeImageShardEventV1(directory,born("w","u/w"));
  const mount=(...parts:string[])=>[directory,"m",...parts].join("/"),image=(id:string)=>[directory,"s",id,"fixed.dmg"].join("/");
  const rows=[{root:mount("cargo"),image:image("cargo"),live:true},{root:mount("u","w"),image:image("w"),live:true},{root:mount("u","w"),image:image("cargo"),live:false},{root:mount("u"),image:image("w"),live:false},{root:mount("cargo","x"),image:image("cargo"),live:false},{root:mount("other"),image:[directory,"fixed.dmg"].join("/"),live:false}];
  for(const row of rows){expect(nativeImageShardRowV1(directory,row),row.root).toBe(row.live);expect(nativeImageShardMountPointV1(directory,row.root),row.root).toBe(row.live||row.root===mount("u","w")||row.root===mount("cargo"));}
  appendNativeImageShardEventV1(directory,{type:"retired",id:"w",atMs:2});
  expect(nativeImageShardRowV1(directory,rows[1]!)).toBe(false);expect(nativeImageShardMountPointV1(directory,mount("u","w"))).toBe(false);expect(nativeImageShardRowV1(directory,rows[0]!)).toBe(true);
  expect(nativeImageShardRowV1(mkdtempSync(join(directory,"none-")),rows[0]!)).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
const run=(program:string,args:readonly string[])=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr+row.stdout);return row.stdout;};
const serial=()=>{let tail=Promise.resolve();return<T>(body:()=>Promise<T>):Promise<T>=>{const next=tail.then(body);tail=next.then(()=>undefined,()=>undefined);return next;};};
const mountTable=()=>run("/sbin/mount",[]);
const backing=(layout:NativeImageShardLayoutV1)=>lstatSync(layout.image).blocks*512;
test.skipIf(process.platform!=="darwin")("two real shards are born lazily at their logical paths and each one gives its own deleted blocks back while all stay mounted; retiring a shard returns everything, and a repair mounts or retires after a restart",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("shard"),"shards-"))),home=nativeImageShardHomeLayoutV1(directory),lifecycle=createNativeImageShardDarwinLifecycleV1({run:async(program,args)=>run(program,args)}),lock=serial();
 const small:NativeImageShardRuleV1={enabled:true,maximumImageBytes:GiB,headroomPercent:0,hostFreeReservePercent:90,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:256*MiB}]};
 const ensure=(path:string)=>ensureNativeImageShardV1({directory,rule:small,platform:"darwin",path,hostFreeBytes:()=>64*GiB,now:()=>Date.now(),lifecycle,lock});
 const kickLog:string[]=[];
 try{
  mkdirSync(home.mountPoint,{recursive:true});
  const seed=join(directory,"seed");mkdirSync(seed);run("/usr/bin/hdiutil",["create","-size","1280m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,home.image]);rmSync(seed,{recursive:true});
  run("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",home.mountPoint,home.image]);
  expect(nativeImageShardMountedV1(home.mountPoint)).toBe(true);
  expect(await ensure("cargo/home/x")).toEqual({kind:"home"});expect(await ensure("u/l")).toEqual({kind:"home"});
  const a=await ensure("u/l/laneaaa/t/x"),b=await ensure("u/l/lanebbb"),again=await ensure("u/l/laneaaa/other");
  expect([a.kind,b.kind,again.kind]).toEqual(["birth","birth","existing"]);
  if(a.kind==="home"||a.kind==="refused"||b.kind==="home"||b.kind==="refused")throw Error("Shards required");
  expect(a.capacityBytes).toBe(GiB);expect(a.layout.mountPoint).toBe(join(home.mountPoint,"u/l/laneaaa"));expect(b.layout.mountPoint).toBe(join(home.mountPoint,"u/l/lanebbb"));
  expect(nativeImageShardMountedV1(a.layout.mountPoint)&&nativeImageShardMountedV1(b.layout.mountPoint)).toBe(true);
  const devices=[home.mountPoint,a.layout.mountPoint,b.layout.mountPoint].map(path=>lstatSync(path).dev);expect(new Set(devices).size).toBe(3);
  expect(readNativeImageShardLedgerV1(directory).map(row=>row.type==="born"?row.id:"-")).toEqual(["laneaaa","lanebbb"]);
  expect(nativeImageShardPathBudgetV1(directory,"laneaaa").fits).toBe(true);
  const whole=measureNativeImageShardRootV1(join(home.mountPoint,"u/l/laneaaa/t"));expect(whole?.shards).toBe(3);expect(whole!.capacityBytes).toBe(1342136320+2*1073700864);expect(nativeImageShardPartitionOfV1(join(home.mountPoint,"x/y"))).toBe(directory);expect(nativeImageShardPartitionOfV1(directory)).toBeUndefined();
  expect(nativeImageShardPathV1(small,0,"laneaaa")).toBe("u/l/laneaaa");
  writeFileSync(join(home.mountPoint,"keep"),Buffer.alloc(2*MiB,5));
  const open=openSync(join(a.layout.mountPoint,"open"),"w");
  try{
   for(const layout of[a.layout,b.layout]){writeFileSync(join(layout.mountPoint,"payload"),Buffer.alloc(128*MiB,7));writeFileSync(join(layout.mountPoint,"stay"),Buffer.alloc(MiB,9));}
   run("/bin/sync",[]);for(const layout of[a.layout,b.layout])rmSync(join(layout.mountPoint,"payload"));run("/bin/sync",[]);
   const stale=[a.layout,b.layout].map(backing);for(const value of stale)expect(value).toBeGreaterThan(100*MiB);
   const layouts=nativeImageShardLayoutsV1(directory,nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)));
   const measured=layouts.map(measureNativeImageShardV1);expect(measured.every(row=>row!==undefined)).toBe(true);
   const aggregate=nativeImageShardAggregateV1(measured as never);expect(aggregate.shards).toBe(3);expect(aggregate.allocatedBytes).toBe(layouts.map(backing).reduce((sum,value)=>sum+value,0));
   expect(layouts.map(row=>Number(oracle("allocated",row.image)))).toEqual(layouts.map(backing));expect(new Set(measured.map(row=>row!.volume)).size).toBe(1);expect(aggregate.freeBytes).toBeLessThanOrEqual(aggregate.hostFreeBytes);
   const outcome=await reclaimNativeImageShardsV1({layouts,rule:{reclaimFloorBytes:64*MiB,reclaimRatioPercent:101,reclaimTightHostFreeBytes:0},hooks:{check:()=>{}},ports:{
    measure:layout=>{const row=measureNativeImageShardV1(layout);return row&&{allocatedBytes:row.allocatedBytes,usedBytes:row.usedBytes,hostFreeBytes:row.hostFreeBytes,sinceLastKickMilliseconds:Infinity};},
    lease:async layout=>{kickLog.push("lease "+layout.id);return()=>{kickLog.push("release "+layout.id);};},
    port:layout=>createNativeImageDarwinKickPortV1({image:layout.image,mountPoint:layout.mountPoint,run:async(program,args)=>run(program,args)})
   }});
   const row=(id:string)=>outcome.rows.find(item=>item.id===id)!;
   for(const[index,layout]of[a.layout,b.layout].entries()){const after=backing(layout);expect(row(layout.id).status).toBe("kicked");expect(row(layout.id).outcome?.effective).toBe(true);expect(after).toBeLessThan(stale[index]!-100*MiB);expect(Number(oracle("allocated",layout.image))).toBe(after);expect(row(layout.id).outcome?.afterBytes).toBe(after);}
   expect(outcome.effective).toBe(true);expect(outcome.beforeBytes-outcome.afterBytes).toBeGreaterThan(200*MiB);expect(kickLog.filter(line=>line.startsWith("lease")).length).toBe(outcome.rows.filter(item=>item.status!=="unmeasured"&&item.status!=="not-due").length);
   writeSync(open,"still writable");expect(readFileSync(join(home.mountPoint,"keep")).equals(Buffer.alloc(2*MiB,5))).toBe(true);expect(readFileSync(join(a.layout.mountPoint,"stay")).equals(Buffer.alloc(MiB,9))).toBe(true);
   expect(mountTable()).not.toContain("semio-kick-");expect(mountTable()).toContain(" on "+a.layout.mountPoint+" ");
  }finally{closeSync(open);}
  const beforeRetire=backing(b.layout);expect(await retireNativeImageShardV1({directory,id:"lanebbb",now:()=>Date.now(),lifecycle,lock})).toBe(true);
  expect(existsSync(b.layout.directory)).toBe(false);expect(mountTable()).not.toContain(" on "+b.layout.mountPoint+" ");expect(existsSync(b.layout.mountPoint)).toBe(false);expect(beforeRetire).toBeGreaterThan(0);
  expect([...nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).keys()]).toEqual(["laneaaa"]);expect(await retireNativeImageShardV1({directory,id:"lanebbb",now:()=>Date.now(),lifecycle,lock})).toBe(false);
  run("/usr/bin/hdiutil",["detach",a.layout.mountPoint]);expect(nativeImageShardMountedV1(a.layout.mountPoint)).toBe(false);
  expect(await repairNativeImageShardsV1({directory,now:()=>Date.now(),lifecycle,lock})).toEqual({laneaaa:"mounted"});expect(nativeImageShardMountedV1(a.layout.mountPoint)).toBe(true);expect(readFileSync(join(a.layout.mountPoint,"stay")).length).toBe(MiB);
  run("/usr/bin/hdiutil",["detach",a.layout.mountPoint]);rmSync(a.layout.image);
  expect(await repairNativeImageShardsV1({directory,now:()=>Date.now(),lifecycle,lock})).toEqual({laneaaa:"retired"});expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).size).toBe(0);expect(readdirSync(join(directory,"s"))).toEqual([]);
  const refused=await ensureNativeImageShardV1({directory,rule:small,platform:"darwin",path:"u/l/lanecc/x",hostFreeBytes:()=>GiB,now:()=>1,lifecycle,lock});expect(refused).toEqual({kind:"refused",reason:"host-free",key:"lanecc"});
 }finally{for(const path of[join(home.mountPoint,"u/l/lanebbb"),join(home.mountPoint,"u/l/laneaaa"),home.mountPoint])spawnSync("/usr/bin/hdiutil",["detach",path,"-force"],{env});rmSync(directory,{recursive:true,force:true});}
},600000);
