/** 🧩️ Tests for the image shard declaration of the cache partitions: the policy is closed, mirrors the shard schema, keeps the live layout (disabled) and sizes its images within the platform cap. */
import{test,expect}from"bun:test";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import shardSchema from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🧬️schema/🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_SHARD_V1,nativeImageShardCapBytesV1,nativeImageShardCapacityV1,nativeImageShardKeyV1,nativeImageShardPathBudgetV1,requireNativeImageShardRuleV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import budgetSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import policy from"../../🔣️policy.json"with{type:"json"};
import{readRepositoryCacheBudgetV1}from"../../🟦️.ts";
import{imageOversizeV1,imageShardPlanV1,readImageReceiptBytesV1,workingSetBytesV1}from"../../📐️elastic/🟦️.ts";
import{nativeImageCommitmentLayoutFitsV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧮️commitment/🟦️.ts";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"../🧫️fixtures/🔣️.json"with{type:"json"};
const GiB=1073741824;
const declared=Object.entries(policy.partitions).flatMap(([id,row])=>"imageShards"in row?[[id,(row as{imageShards:unknown}).imageShards]as const]:[]);
test("the oversized partitions declare their shards, closed and equal to the shard schema, and the split of the Rust partition is enabled",()=>{
 expect(declared.map(([id])=>id)).toEqual(["rust","dependencies"]);
 for(const[,rule]of declared){expect(validateJsonSchemaSubset(shardSchema.definitions.Rule,rule,shardSchema)).toEqual([]);expect<unknown>(requireNativeImageShardRuleV1(rule)).toEqual(rule);}
 expect(declared.map(([,rule])=>(rule as{enabled:boolean}).enabled)).toEqual([true,false]);
 const budget=readRepositoryCacheBudgetV1(policy);expect<unknown>(budget.partitions.rust.imageShards).toEqual(declared[0]![1]);expect(budget.partitions.tools.imageShards).toBeUndefined();
 const bad={...(declared[0]![1] as object),extra:true};expect(validateJsonSchemaSubset(budgetSchema,{...policy,partitions:{...policy.partitions,rust:{...policy.partitions.rust,imageShards:bad}}}).length).toBeGreaterThan(0);
 expect(validateJsonSchemaSubset(shardSchema.definitions.Rule,bad,shardSchema).length).toBeGreaterThan(0);
});
test("every key image of the partitions fits the cap of every platform that can shard, and the partition capacity needs the shards the note names",()=>{
 const rust=declared[0]![1] as Parameters<typeof requireNativeImageShardRuleV1>[0]&{roots:{needBytes:number}[];headroomPercent:number;hostFreeReservePercent:number};
 const born=nativeImageShardCapacityV1("darwin",rust as never,{needBytes:rust.roots[0]!.needBytes,hostFreeBytes:200*GiB});
 expect(born).toEqual({capacityBytes:32*GiB});expect(nativeImageShardCapBytesV1("darwin")).toBe(2**35);
 expect(Math.ceil(policy.partitions.rust.capacityBytes/nativeImageShardCapBytesV1("darwin"))).toBe(4);expect(policy.partitions.dependencies.capacityBytes).toBeLessThanOrEqual(nativeImageShardCapBytesV1("darwin"));
 const deps=declared[1]![1] as typeof rust;expect(nativeImageShardCapacityV1("darwin",deps as never,{needBytes:deps.roots[0]!.needBytes,hostFreeBytes:200*GiB})).toEqual({capacityBytes:32*GiB});
});
test("the declared roots claim exactly the lanes, sealed and worker areas of the Rust unit cache, its cargo home and its components, and the job workspaces of the dependencies, nothing else",()=>{
 const[rust,deps]=declared.map(([,rule])=>rule as Parameters<typeof nativeImageShardKeyV1>[0]);
 expect(nativeImageShardKeyV1(rust!,"u/l/pcab777/t/x")).toEqual({root:0,key:"pcab777",id:"pcab777"});
 expect(nativeImageShardKeyV1(rust!,"u/sealed/d148cc4e846068aa")).toEqual({root:1,key:"sealed",id:"sealed"});expect(nativeImageShardKeyV1(rust!,"u/w/x")).toEqual({root:1,key:"w",id:"w"});expect(nativeImageShardKeyV1(rust!,"cargo/home")).toEqual({root:2,key:"cargo",id:"cargo"});expect(nativeImageShardKeyV1(rust!,"components/stages/x")).toEqual({root:2,key:"components",id:"components"});
 for(const path of["u/e/abc","u/k/lane-pcab777","u/l","u/lanes/x","b/x","published/x","nx/x","configuration/x","u/ballast"])expect(nativeImageShardKeyV1(rust!,path)).toBeNull();
 expect(nativeImageShardKeyV1(deps!,"j/02c779b7d284b46ef3fd/node_modules/x")?.id).toMatch(/^x[0-9a-f]{11}$/u);
 for(const path of["bun-cache/x","generation/x","j","python/x"])expect(nativeImageShardKeyV1(deps!,path)).toBeNull();
});
test("the shard directories and mount points of the partitions stay within the 256 character rule of the workspace",()=>{
 for(const[id]of declared){
  const row=(policy.partitions as Record<string,{directory:string}>)[id]!,directory="/"+"w".repeat(100)+"/"+row.directory;
  expect(nativeImageShardPathBudgetV1(directory,"x1bd8fd47297","posix").fits).toBe(true);expect(row.directory.length).toBeLessThanOrEqual(NATIVE_IMAGE_SHARD_V1.pathBudgetCharacters-40);
 }
});

test("the images of a measured working set equal the independent Python model on every measured worker, keep every image below its high-water mark within the cap and refuse a host that cannot hold the working set",()=>{
 const plan=fixture.imageShardPlan,input=(row:(typeof plan.cases)[number])=>({workingSetBytes:workingSetBytesV1(row),capBytes:plan.capBytes,alignBytes:plan.alignBytes,minimumBytes:plan.minimumBytes,highWaterBasisPoints:plan.highWaterBasisPoints,hostFreeBytes:row.hostFreeBytes,hostFreeReservePercent:plan.hostFreeReservePercent});
 const model=`import json,sys
plan=json.loads(sys.argv[1])
out=[]
for row in plan["cases"]:
    ws=max(0,row["bytes"]-row["backingBytes"])
    cap=plan["capBytes"]//plan["alignBytes"]*plan["alignBytes"]
    usable=cap*plan["highWaterBasisPoints"]//10000
    room=row["hostFreeBytes"]*plan["hostFreeReservePercent"]//100
    if ws>room:
        out.append({"refused":"host-free"}); continue
    shards=max(1,-(-ws//usable))
    share=-(-ws//shards)
    want=-(-(-(-share*10000//plan["highWaterBasisPoints"]))//plan["alignBytes"])*plan["alignBytes"]
    per=min(cap,max(plan["minimumBytes"],want))
    out.append({"shards":shards,"perShardBytes":per,"totalBytes":shards*per})
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(plan)]);
 expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);
 const expected=JSON.parse(new TextDecoder().decode(run.stdout)) as unknown[];
 expect(plan.cases.map(row=>imageShardPlanV1(input(row)) as unknown)).toEqual(expected);
 for(const row of plan.cases){const made=imageShardPlanV1(input(row));if("refused"in made)continue;const share=Math.ceil(workingSetBytesV1(row)/made.shards);expect(made.perShardBytes).toBeLessThanOrEqual(Math.floor(plan.capBytes/plan.alignBytes)*plan.alignBytes);expect(made.perShardBytes%plan.alignBytes).toBe(0);expect(share).toBeLessThanOrEqual(Math.floor(made.perShardBytes*plan.highWaterBasisPoints/10000)+1);}
 const measured=plan.cases.filter(row=>row.origin.startsWith("g1-"));expect(measured.length).toBeGreaterThan(30);
 expect(plan.cases.find(row=>row.origin==="synthetic one byte above it")&&imageShardPlanV1(input(plan.cases.find(row=>row.origin==="synthetic one byte above it")!))).toMatchObject({shards:2});
 expect(imageShardPlanV1(input(plan.cases.find(row=>row.origin==="synthetic exactly the usable part of one image")!))).toMatchObject({shards:1,perShardBytes:plan.capBytes});
 expect(imageShardPlanV1(input(plan.cases.find(row=>row.origin==="synthetic empty")!))).toMatchObject({shards:1,perShardBytes:plan.minimumBytes});
});

test("layout invariant: every image the policy can birth, the home of every partition and every shard of a sharded one, stays at or below the 32 GiB unmap limit, so only a sharded set may declare more in all",()=>{
 const cap=nativeImageShardCapBytesV1("darwin");expect(cap).toBe(32*GiB);
 for(const[id,rule]of Object.entries(policy.elastic))expect(nativeImageCommitmentLayoutFitsV1("darwin",rule.maximumBytes),"elastic "+id).toBe(true);
 for(const[id,row]of Object.entries(policy.partitions)){
  const rule=(row as{imageShards?:{enabled:boolean}}).imageShards;
  if(!rule?.enabled&&!(id in policy.elastic))expect(nativeImageCommitmentLayoutFitsV1("darwin",row.capacityBytes),"home "+id).toBe(true);
 }
 for(const[id,rule]of declared){
  const sharded=rule as Parameters<typeof requireNativeImageShardRuleV1>[0]&{roots:{needBytes:number}[]};
  for(const[index,root]of sharded.roots.entries()){const born=nativeImageShardCapacityV1("darwin",sharded as never,{needBytes:root.needBytes,hostFreeBytes:1000*GiB});expect("capacityBytes"in born&&nativeImageCommitmentLayoutFitsV1("darwin",born.capacityBytes),id+" root "+index).toBe(true);}
 }
 const rust=policy.partitions.rust;expect(rust.imageShards.enabled).toBe(true);expect(nativeImageCommitmentLayoutFitsV1("darwin",policy.elastic.rust.maximumBytes)).toBe(true);expect(Math.ceil(rust.capacityBytes/cap)).toBeGreaterThan(1);
 expect(nativeImageCommitmentLayoutFitsV1("darwin",policy.partitions.dependencies.capacityBytes)).toBe(true);
});
test("layout invariant: an image born above the cap, as the 96 GiB compiler container and the 36 GiB dependencies container were, is oversize by its receipt and a 32 GiB one is not",()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-oversize-"));
 try{
  for(const[name,bytes,oversize]of[["r",103080263680,true],["d",38654705664,true],["ok",34359738368,false]]as const){
   const image=join(directory,name);mkdirSync(image);writeFileSync(join(image,"image.json"),JSON.stringify({version:1,maximumPhysicalBytes:bytes}));
   expect(imageOversizeV1(readImageReceiptBytesV1(image),"darwin"),name).toBe(oversize);expect(imageOversizeV1(readImageReceiptBytesV1(image),"linux"),name+" linux").toBe(false);
  }
  expect(imageOversizeV1(readImageReceiptBytesV1(join(directory,"nobody")),"darwin")).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
