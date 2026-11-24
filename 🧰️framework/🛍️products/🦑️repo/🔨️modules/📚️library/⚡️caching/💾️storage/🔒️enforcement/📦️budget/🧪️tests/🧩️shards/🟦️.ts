/** 🧩️ Tests for the image shard declaration of the cache partitions: the policy is closed, mirrors the shard schema, keeps the live layout (disabled) and sizes its images within the platform cap. */
import{test,expect}from"bun:test";
import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import shardSchema from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🧬️schema/🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_SHARD_V1,nativeImageShardCapBytesV1,nativeImageShardCapacityV1,nativeImageShardKeyV1,nativeImageShardPathBudgetV1,requireNativeImageShardRuleV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import budgetSchema from"../../🧬️schema/🔣️.json"with{type:"json"};
import policy from"../../🔣️policy.json"with{type:"json"};
import{readRepositoryCacheBudgetV1}from"../../🟦️.ts";
const GiB=1073741824;
const declared=Object.entries(policy.partitions).flatMap(([id,row])=>"imageShards"in row?[[id,(row as{imageShards:unknown}).imageShards]as const]:[]);
test("the oversized partitions declare their shards, closed and equal to the shard schema, and keep the live single-image layout until the switchover",()=>{
 expect(declared.map(([id])=>id)).toEqual(["rust","dependencies"]);
 for(const[,rule]of declared){expect(validateJsonSchemaSubset(shardSchema.definitions.Rule,rule,shardSchema)).toEqual([]);expect(requireNativeImageShardRuleV1(rule)).toEqual(rule);expect((rule as{enabled:boolean}).enabled).toBe(false);}
 const budget=readRepositoryCacheBudgetV1(policy);expect(budget.partitions.rust.imageShards).toEqual(declared[0]![1]);expect(budget.partitions.tools.imageShards).toBeUndefined();
 const bad={...(declared[0]![1] as object),extra:true};expect(validateJsonSchemaSubset(budgetSchema,{...policy,partitions:{...policy.partitions,rust:{...policy.partitions.rust,imageShards:bad}}}).length).toBeGreaterThan(0);
 expect(validateJsonSchemaSubset(shardSchema.definitions.Rule,bad,shardSchema).length).toBeGreaterThan(0);
});
test("every key image of the partitions fits the cap of every platform that can shard, and the partition capacity needs the shards the note names",()=>{
 const rust=declared[0]![1] as Parameters<typeof requireNativeImageShardRuleV1>[0]&{roots:{needBytes:number}[];headroomPercent:number;hostFreeReservePercent:number};
 const born=nativeImageShardCapacityV1("darwin",rust as never,{needBytes:rust.roots[0]!.needBytes,hostFreeBytes:200*GiB});
 expect(born).toEqual({capacityBytes:32*GiB});expect(nativeImageShardCapBytesV1("darwin")).toBe(2**35);
 expect(Math.ceil(policy.partitions.rust.capacityBytes/nativeImageShardCapBytesV1("darwin"))).toBe(4);expect(Math.ceil(policy.partitions.dependencies.capacityBytes/nativeImageShardCapBytesV1("darwin"))).toBe(2);
 const deps=declared[1]![1] as typeof rust;expect(nativeImageShardCapacityV1("darwin",deps as never,{needBytes:deps.roots[0]!.needBytes,hostFreeBytes:200*GiB})).toEqual({capacityBytes:32*GiB});
});
test("the declared roots claim exactly the lanes of the Rust unit cache and the job workspaces of the dependencies, nothing else",()=>{
 const[rust,deps]=declared.map(([,rule])=>rule as Parameters<typeof nativeImageShardKeyV1>[0]);
 expect(nativeImageShardKeyV1(rust!,"u/l/pcab777/t/x")).toEqual({root:0,key:"pcab777",id:"pcab777"});
 for(const path of["u/e/abc","u/k/lane-pcab777","u/sealed/d148cc4e846068aa","cargo/home","components/x","u/l","u/lanes/x","b/x"])expect(nativeImageShardKeyV1(rust!,path)).toBeNull();
 expect(nativeImageShardKeyV1(deps!,"j/02c779b7d284b46ef3fd/node_modules/x")?.id).toMatch(/^x[0-9a-f]{11}$/u);
 for(const path of["bun-cache/x","generation/x","j","python/x"])expect(nativeImageShardKeyV1(deps!,path)).toBeNull();
});
test("the shard directories and mount points of the partitions stay within the 256 character rule of the workspace",()=>{
 for(const[id]of declared){
  const row=(policy.partitions as Record<string,{directory:string}>)[id]!,directory="/"+"w".repeat(100)+"/"+row.directory;
  expect(nativeImageShardPathBudgetV1(directory,"x1bd8fd47297","posix").fits).toBe(true);expect(row.directory.length).toBeLessThanOrEqual(NATIVE_IMAGE_SHARD_V1.pathBudgetCharacters-40);
 }
});
