import{createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import policy from "../🔣️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {effectiveCapacityV1,elasticBelowFloorDueV1,elasticCapacityV1,elasticMinimumV1,hostShareCapacityV1,hostSharePercentV1,ImageBelowFloorErrorV1,ImageDoesNotFitHostErrorV1,partitionNeedBytesV1,pinHostFreeBytesForSizingV1,pinnedCapacityV1,readRepositoryCacheBudgetV1,repositoryCacheBudgetV1,repositoryPartitionNeedBytesV1,requireImageAtFloorV1,requireImageFitsHostV1,requireRepositoryPartitionFitsHostV1} from "../🟦️.ts";
import {mkdtempSync,mkdirSync,writeFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {repositoryStrictStorageV1} from "../../🟦️.ts";
import {cleanProtectedPrefixes,cleanRemovalProtection} from "../../../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts";
import {cleanRemovePath} from "../../../../../🧼️workspace-cleanup/🗑️removal/🟦️.ts";
import {resolve,join} from "node:path";
import {withRepositoryCacheStoreV1} from "../📥️opening/🏪️store/🟦️.ts";
import {runProbe} from "../../../../../🏃️process/🔍️probe/🟦️.ts";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {lstatSync,readdirSync,readFileSync} from "node:fs";
import{privateRepositoryHostFreeBytesV1,privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{nativePlatformImageBackendV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import{nativeStorageBackendsV1,nativeStorageBackingFilenameV1,nativeStorageMountedAtV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-budget"});
const repository=await privateRepositoryWorkspaceV1();

test("cache backing and mount expectations agree with the neutral backend corpus",()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema.$defs.imageBackendCase);
 try{expect(fixture.imageBackends.map(row=>row.id).sort()).toEqual(nativeStorageBackendsV1.map(row=>row.id).sort());for(const row of fixture.imageBackends){expect(validate(row)).toBe(true);const declared=nativeStorageBackendsV1.find(backend=>backend.id===row.id)!,oracle=db.query("SELECT json_extract(?1,'$.backingFilename') backing,EXISTS(SELECT 1 FROM json_each(?1,'$.capabilities') WHERE value='mounted-volume') mounted").get(JSON.stringify(declared))as{backing:string;mounted:number};expect(nativeStorageBackingFilenameV1(row.id)).toBe(row.backing);expect(oracle.backing).toBe(row.backing);expect(Boolean(oracle.mounted)).toBe(row.mounted);}}
 finally{db.close();}
});

/** 🎚️ Keeps authored logical observation limits independent of every physical capacity charge. */
test("closed repository publication logical observation policy leaves the physical ledger unchanged",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema),cases=new Ajv({strict:true}).compile(schema.$defs.logicalCase);try{expect((policy.partitions.tools as any).maximumLogicalBytes).toBe(policy.partitions.tools.capacityBytes-1048576);const physical=Object.fromEntries(Object.entries(policy.partitions).map(([id,row])=>[id,row.capacityBytes]));for(const row of fixture.logicalCases){expect(cases(row),row.id).toBe(true);const value:any=structuredClone(policy);if(row.missing)delete value.partitions.tools.maximumLogicalBytes;else value.partitions.tools.maximumLogicalBytes=row.maximumLogicalBytes;const encoded=JSON.stringify(value),oracle=database.query("SELECT json_type(?1,'$.partitions.tools.maximumLogicalBytes') IN('integer','real') AND json_extract(?1,'$.partitions.tools.maximumLogicalBytes')=CAST(json_extract(?1,'$.partitions.tools.maximumLogicalBytes') AS INTEGER) AND json_extract(?1,'$.partitions.tools.maximumLogicalBytes') BETWEEN 1 AND 103080263680 accepted,(SELECT sum(json_extract(value,'$.capacityBytes')) FROM json_each(?1,'$.partitions')) bytes").get(encoded)as{accepted:number|null;bytes:number};expect(validate(value),row.id).toBe(row.accepted);expect(Boolean(oracle.accepted),row.id).toBe(row.accepted);expect(oracle.bytes).toBe(fixture.expectedPartitionBytes);if(row.accepted){const actual:any=readRepositoryCacheBudgetV1(value);expect(actual.partitionBytes).toBe(fixture.expectedPartitionBytes);expect(actual.headroomBytes).toBe(fixture.expectedHeadroomBytes);expect(actual.partitions.tools.maximumLogicalBytes).toBe(row.maximumLogicalBytes);expect(Object.fromEntries(Object.entries(actual.partitions).map(([id,part]:[string,any])=>[id,part.capacityBytes]))).toEqual(physical);}else expect(()=>readRepositoryCacheBudgetV1(value)).toThrow();}expect(physical.tools).toBe(8589934592);expect((policy.partitions.tools as any).maximumLogicalBytes).toBeLessThanOrEqual(physical.tools!-1048576);}finally{database.close();}
});

test("closed repository cache partitions agree with independent SQLite byte and path admission",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);
 try{for(const row of fixture.cases){
  const value=structuredClone(policy);if(row.mutation==="oversubscribed")value.partitions.products.capacityBytes=value.maximumBytes;if(row.mutation==="overlapping")value.partitions.tools.directory=value.partitions.rust.directory+"/nested";if(row.mutation==="escaping")value.partitions.tools.directory="../foreign";if(row.mutation==="unaligned")value.partitions.tools.capacityBytes++;if(row.mutation==="missing")delete (value.partitions as any).tools;if(row.mutation==="control-overlap")value.partitions.tools.directory=".cache/b/c/t";
  const encoded=JSON.stringify(value),sum=(database.query("SELECT sum(json_extract(value,'$.capacityBytes')) AS bytes FROM json_each(?,'$.partitions')").get(encoded) as {bytes:number}).bytes;
  const unsafe=(database.query("SELECT count(*) AS count FROM json_each(?,'$.partitions') WHERE json_extract(value,'$.directory') LIKE '%..%' OR json_extract(value,'$.capacityBytes') % 1048576 != 0 OR instr(json_extract(value,'$.directory') || '/', '.cache/b/c/') = 1 OR instr('.cache/b/c/',json_extract(value,'$.directory') || '/') = 1").get(encoded) as {count:number}).count;
  const overlaps=(database.query("SELECT count(*) AS count FROM json_each(?,'$.partitions') a JOIN json_each(?,'$.partitions') b ON a.key < b.key WHERE json_extract(a.value,'$.directory') = json_extract(b.value,'$.directory') OR instr(json_extract(a.value,'$.directory') || '/',json_extract(b.value,'$.directory') || '/') = 1 OR instr(json_extract(b.value,'$.directory') || '/',json_extract(a.value,'$.directory') || '/') = 1").get(encoded,encoded) as {count:number}).count;
  const oracle=validate(value)&&sum+value.maximumControlBytes<=value.maximumBytes&&unsafe===0&&overlaps===0;
  expect(oracle,row.id).toBe(row.accepted);if(row.accepted){const actual:any=readRepositoryCacheBudgetV1(value);expect(actual.partitionBytes).toBe(sum);expect(sum).toBe(fixture.expectedPartitionBytes);expect(actual.headroomBytes).toBe(fixture.expectedHeadroomBytes);}else expect(()=>readRepositoryCacheBudgetV1(value),row.id).toThrow();
 }}finally{database.close();}
});

test("owned process probe captures exact bytes inside actual fixed diagnostics storage",async()=>{
 await withBuildTestInvocationV1(async original=>withRepositoryCacheStoreV1(repository.root,"diagnostics",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async store=>{
  const names=["SEMIO_DIAGNOSTIC_DIRECTORY"] as const,previous=Object.fromEntries(names.map(name=>[name,process.env[name]])),directory=store.environment.SEMIO_DIAGNOSTIC_DIRECTORY!;for(const name of names)process.env[name]=directory;
  const database=new Database(":memory:");try{const before=readdirSync(directory).sort(),script='process.stdout.write(Buffer.from(process.argv[1],"base64"));process.stderr.write(Buffer.from(process.argv[2],"base64"));',result=runProbe("node",["-e",script,Buffer.from(fixture.probe.stdout).toString("base64"),Buffer.from(fixture.probe.stderr).toString("base64")],{cwd:process.cwd(),env:store.environment,budgetMs:5000}),oracle=database.query("SELECT length(cast(? AS blob)) AS bytes").get(fixture.probe.stdout) as {bytes:number};expect(result.stdout).toBe(fixture.probe.stdout);expect(result.stderr).toBe(fixture.probe.stderr);expect(result.status).toBe(fixture.probe.status);expect(Buffer.byteLength(result.stdout)).toBe(oracle.bytes);expect(readdirSync(directory).sort()).toEqual(before);expect(lstatSync(store.image.image).blocks*512).toBeLessThanOrEqual(store.image.maximumPhysicalBytes);await store.recheck();}finally{database.close();for(const name of names)if(previous[name]===undefined)delete process.env[name];else process.env[name]=previous[name];}
 }));
},120000);

/** 🧊️ Keeps the Nx graph pool a top-level image whose charge leaves room for funded namespace children. */
test("the nx cache is a declared top-level partition with a funded namespace envelope",()=>{
 const database=new Database(":memory:");
 try{const row=fixture.nxPartition,declared=policy.partitions.nx as Record<string,unknown>,actual=readRepositoryCacheBudgetV1(policy).partitions;expect(declared).toEqual(row.declaration);
  const oracle=database.query("SELECT json_extract(?1,'$.nx.directory') directory,(SELECT count(*) FROM json_each(?1) p WHERE p.key<>'nx' AND (instr(json_extract(p.value,'$.directory')||'/',json_extract(?1,'$.nx.directory')||'/')=1 OR instr(json_extract(?1,'$.nx.directory')||'/',json_extract(p.value,'$.directory')||'/')=1)) overlaps,(json_extract(?1,'$.nx.capacityBytes')-1048576-json_extract(?1,'$.nx.maximumArtifactBytes'))/(?2+?3) children").get(JSON.stringify(policy.partitions),row.childPhysicalBytes,row.childMetadataBytes)as{directory:string;overlaps:number;children:number};
  expect(oracle.directory).toBe(".cache/b/n");expect(oracle.overlaps).toBe(0);expect(oracle.children).toBeGreaterThanOrEqual(row.minimumChildren);expect(actual.nx.maximumArtifactBytes).toBe(row.declaration.maximumArtifactBytes);
  for(const[id,partition]of Object.entries(actual))if(id!=="nx")expect(partition.maximumArtifactBytes,id).toBe(partition.filesystemBytes);
 }finally{database.close();}
});

/** 🏗️ Provisions the real nx partition and proves its declared backing, mount capability and artifact charge. */
test("the nx partition provisions its own backing and charges the artifact envelope",async()=>{
 await withBuildTestInvocationV1(async original=>withRepositoryCacheStoreV1(repository.root,"nx",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async store=>{
  const declared=policy.partitions.nx,directory=resolve(repository.root,declared.directory),backend=nativePlatformImageBackendV1(store.image),expected=fixture.imageBackends.find(row=>row.id===backend)!;
  expect(expected).toBeDefined();expect(store.image.image).toBe(join(directory,expected.backing));expect(store.root).toBe(join(directory,"m"));
  const backing=lstatSync(store.image.image);expect(backing.blocks*512).toBeGreaterThanOrEqual(backing.size);expect(backing.size).toBeLessThanOrEqual(declared.capacityBytes);expect(nativeStorageMountedAtV1(directory)).toBe(expected.mounted);expect(lstatSync(store.root).dev===lstatSync(directory).dev).toBe(!expected.mounted);
  const receipt=JSON.parse(readFileSync(join(store.root,"nx.json"),"utf8"));expect(receipt.owner).toBe("repository:nx-store");expect(receipt.bytes).toBe(declared.maximumArtifactBytes);expect(store.storage.allocation.bytes).toBe(declared.maximumArtifactBytes);
  await store.recheck();
 }));
},120000);

test("ordinary cleanup cannot retire the authored fixed cache backings",()=>{
 const database=new Database(":memory:"),root=repository.root,protectedPaths=cleanProtectedPrefixes(root),view={kind:()=>"missing" as const,children:()=>[],read:()=>undefined};
 try{for(const row of Object.values(policy.partitions)){const path=resolve(root,row.directory),oracle=database.query("SELECT ? || '/' || ? AS path").get(root,row.directory) as {path:string};expect(path).toBe(oracle.path);expect(protectedPaths).toContain(path);for(const candidate of fixture.cleanupSuffixes.map(suffix=>suffix?join(path,suffix):path)){expect(cleanRemovalProtection(root,candidate,view)).toContain(candidate);for(const incremental of [false,true])expect(cleanRemovePath(root,candidate,true,[],false,false,incremental)).toBe(false);}}for(const relative of [...fixture.protectedAncestors,fixture.bootstrapControlDirectory]){const path=resolve(root,relative);expect(cleanRemovalProtection(root,path,view)).toContain(path);expect(cleanRemovePath(root,path,true,[],true,true,true)).toBe(false);}}finally{database.close();}
});

/** 🪫️ Keeps the logical limit of the tiny lease image inside the bytes its filesystem can hold after APFS metadata and journals. */
test("tiny cache partitions bound their logical limit by the usable bytes of the filesystem overhead model",()=>{
 const database=new Database(":memory:");
 try{
  const actual=readRepositoryCacheBudgetV1(policy).partitions,leases=actual.leases,fs=leases.filesystemBytes-40960;
  const oracle=database.query("SELECT ?1-(1048576+(?1+255)/256)-1048576-524288 AS usable").get(fs) as{usable:number};
  expect(leases.usableBytes).toBe(oracle.usable);expect(leases.maximumLogicalBytes).toBeLessThanOrEqual(leases.usableBytes);expect(leases.maximumLogicalBytes).toBeLessThan(leases.filesystemBytes-2000000);
  const value:any=structuredClone(policy);value.partitions.leases.maximumLogicalBytes=leases.filesystemBytes;
  expect(()=>readRepositoryCacheBudgetV1(value)).toThrow("Tiny cache partition");
  value.partitions.leases.maximumLogicalBytes=leases.usableBytes;expect(()=>readRepositoryCacheBudgetV1(value)).not.toThrow();
  for(const[id,partition]of Object.entries(actual))if(partition.capacityBytes>schema.$defs.filesystemOverhead.const.tinyCapacityBytes)expect(partition.usableBytes,id).toBeLessThan(partition.filesystemBytes);
 }finally{database.close();}
});

test("the elastic products capacity sizes a new image from need and host, never recreates the existing image automatically, and agrees with an independent Python model",()=>{
 const validate=new Ajv({strict:true}).compile(schema.$defs.elasticCapacityCase),rule=policy.elastic.products,declared=policy.partitions.products.capacityBytes;
 expect(new Ajv({strict:true}).compile(schema.$defs.elasticPartition)(rule)).toBe(true);expect(rule.automatic).toBe(true);expect(rule.pressureFreeBytes).toBeLessThan(3*2**30);
 const model=`import json,sys
rule,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out={}
for row in rows:
    wanted=-(-rule["needBytes"]*(100+rule["headroomPercent"])//100)
    ceiling=min(rule["maximumBytes"],row["hostFreeBytes"]*(100-rule["hostFreeReservePercent"])//100)
    target=min(wanted,max(ceiling,row["minimumBytes"]))
    out[row["id"]]=max(row["minimumBytes"],target//rule["alignBytes"]*rule["alignBytes"])
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(fixture.elasticProductsCapacity)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const independent=JSON.parse(run.stdout.toString());
 for(const row of fixture.elasticProductsCapacity){expect(validate(row),row.id).toBe(true);const minimum=row.usesPolicyMinimum?rule.minimumBytes:row.minimumBytes;expect(elasticCapacityV1(rule,minimum,row.hostFreeBytes),row.id).toBe(row.capacityBytes);expect(independent[row.id],row.id).toBe(row.capacityBytes);expect(row.capacityBytes%1048576).toBe(0);expect(row.capacityBytes).toBeGreaterThanOrEqual(row.minimumBytes);}
 expect(elasticCapacityV1(rule,declared,0)).toBe(declared);
 const grown=structuredClone(policy);grown.partitions.products.capacityBytes=31*2**30;grown.partitions.products.maximumLogicalBytes=31*2**30-1048576;grown.maximumBytes+=31*2**30-declared;
 const budget=readRepositoryCacheBudgetV1(grown);expect(budget.partitions.products.filesystemBytes).toBe(31*2**30-1048576);expect(budget.partitionBytes).toBe(readRepositoryCacheBudgetV1(policy).partitionBytes+31*2**30-declared);expect(budget.headroomBytes).toBe(readRepositoryCacheBudgetV1(policy).headroomBytes);
});

test("the elastic rust capacity sizes a new compiler image from need and host, the strict storage envelope follows the same rule, and agrees with an independent Python model",()=>{
 const validate=new Ajv({strict:true}).compile(schema.$defs.elasticCapacityCase),rule=policy.elastic.rust,declared=policy.partitions.rust.capacityBytes;
 expect(new Ajv({strict:true}).compile(schema.$defs.elasticPartition)(rule)).toBe(true);expect(rule.automatic).toBe(true);expect(rule.pressureFreeBytes).toBeLessThanOrEqual(8*2**30);expect(rule.needBytes).toBeLessThan(declared);expect(rule.needBytes).toBeGreaterThanOrEqual(rule.minimumBytes);
 const model=`import json,sys
rule,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out={}
for row in rows:
    wanted=-(-rule["needBytes"]*(100+rule["headroomPercent"])//100)
    ceiling=min(rule["maximumBytes"],row["hostFreeBytes"]*(100-rule["hostFreeReservePercent"])//100)
    target=min(wanted,max(ceiling,row["minimumBytes"]))
    out[row["id"]]=max(row["minimumBytes"],target//rule["alignBytes"]*rule["alignBytes"])
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(fixture.elasticRustCapacity)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const independent=JSON.parse(run.stdout.toString());
 for(const row of fixture.elasticRustCapacity){expect(validate(row),row.id).toBe(true);expect(elasticCapacityV1(rule,row.minimumBytes,row.hostFreeBytes),row.id).toBe(row.capacityBytes);expect(independent[row.id],row.id).toBe(row.capacityBytes);expect(row.capacityBytes%1048576).toBe(0);expect(row.capacityBytes).toBeGreaterThanOrEqual(row.minimumBytes);expect(row.capacityBytes).toBeLessThanOrEqual(rule.maximumBytes);}
 const grown=structuredClone(policy);grown.partitions.rust.capacityBytes=60*2**30;grown.partitions.rust.maximumLogicalBytes=60*2**30-1048576;grown.maximumBytes+=60*2**30-declared;
 const budget=readRepositoryCacheBudgetV1(grown);expect(budget.partitions.rust.filesystemBytes).toBe(60*2**30-1048576);expect(budget.headroomBytes).toBe(readRepositoryCacheBudgetV1(policy).headroomBytes);
 expect(rule.minimumBytes).toBeLessThan(declared);expect(rule.minimumBytes).toBeLessThanOrEqual(rule.maximumBytes);
 for(const free of Array.from({length:600},(_,i)=>(i+34)*2**30)){const capacity=elasticCapacityV1(rule,rule.minimumBytes,free);expect(capacity,String(free)).toBeLessThanOrEqual(free-64*2**20);expect(capacity).toBeGreaterThanOrEqual(rule.minimumBytes);}
 const strict=repositoryStrictStorageV1(repository.root);expect(strict.capacityBytes).toBeGreaterThanOrEqual(elasticMinimumV1(rule,rule.minimumBytes,privateRepositoryHostFreeBytesV1));expect(strict.capacityBytes).toBeLessThanOrEqual(rule.maximumBytes);expect(strict.filesystemBytes).toBe(strict.capacityBytes-1048576);
});

test("the elastic graph capacity follows need and host free space, never below the declared capacity, and agrees with an independent Python model",()=>{
 const validate=new Ajv({strict:true}).compile(schema.$defs.elasticCapacityCase),rule=policy.elastic.graph,declared=policy.partitions.graph.capacityBytes;
 expect(new Ajv({strict:true}).compile(schema.$defs.elasticPartition)(rule)).toBe(true);
 expect(readRepositoryCacheBudgetV1(policy).partitions.graph.capacityBytes).toBe(declared);
 const model=`import json,sys
rule,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out={}
for row in rows:
    wanted=-(-rule["needBytes"]*(100+rule["headroomPercent"])//100)
    ceiling=min(rule["maximumBytes"],row["hostFreeBytes"]*(100-rule["hostFreeReservePercent"])//100)
    target=min(wanted,max(ceiling,row["minimumBytes"]))
    out[row["id"]]=max(row["minimumBytes"],target//rule["alignBytes"]*rule["alignBytes"])
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(fixture.elasticCapacity)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 const independent=JSON.parse(run.stdout.toString());
 for(const row of fixture.elasticCapacity){expect(validate(row),row.id).toBe(true);expect(elasticCapacityV1(rule,row.minimumBytes,row.hostFreeBytes),row.id).toBe(row.capacityBytes);expect(independent[row.id],row.id).toBe(row.capacityBytes);expect(row.capacityBytes%1048576).toBe(0);}
 const grown=structuredClone(policy);grown.partitions.graph.capacityBytes=12*2**30;grown.partitions.graph.maximumLogicalBytes=12*2**30-1048576;grown.maximumBytes+=6*2**30;
 const budget=readRepositoryCacheBudgetV1(grown);expect(budget.partitions.graph.filesystemBytes).toBe(12*2**30-1048576);expect(budget.partitionBytes).toBe(readRepositoryCacheBudgetV1(policy).partitionBytes+6*2**30);expect(budget.headroomBytes).toBe(readRepositoryCacheBudgetV1(policy).headroomBytes);
});


test("a policy target changed after birth leaves an existing image at its own size until the explicit reset births a new one, and agrees with an independent Python model",()=>{
 const declared=policy.partitions.dependencies,unit=1048576,rows=[
  {id:"born-smaller",receipt:{maximumPhysicalBytes:8*2**30},expected:8*2**30},
  {id:"born-equal",receipt:{maximumPhysicalBytes:declared.capacityBytes},expected:declared.capacityBytes},
  {id:"born-larger-than-declared",receipt:{maximumPhysicalBytes:declared.capacityBytes+unit},expected:declared.capacityBytes},
  {id:"unaligned",receipt:{maximumPhysicalBytes:8*2**30+1},expected:declared.capacityBytes},
  {id:"below-minimum",receipt:{maximumPhysicalBytes:unit},expected:declared.capacityBytes},
  {id:"not-a-number",receipt:{maximumPhysicalBytes:"8589934592"},expected:declared.capacityBytes},
  {id:"no-receipt",receipt:null,expected:declared.capacityBytes},
  {id:"unreadable",receipt:"{",expected:declared.capacityBytes}
 ],model=`import json,sys
d,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out={}
for r in rows:
    v=r["bytes"]
    ok=isinstance(v,int) and not isinstance(v,bool) and v%1048576==0 and v>=33554432 and v<=d
    out[r["id"]]=v if ok else d
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(declared.capacityBytes),JSON.stringify(rows.map(row=>({id:row.id,bytes:row.receipt&&typeof row.receipt==="object"?row.receipt.maximumPhysicalBytes:null})))]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);const independent=JSON.parse(run.stdout.toString());
 const pinRust=(root:string)=>{mkdirSync(join(root,".cache","b"),{recursive:true});const rustDirectory=join(root,policy.partitions.rust.directory);mkdirSync(rustDirectory,{recursive:true});writeFileSync(join(rustDirectory,"image.json"),JSON.stringify({maximumPhysicalBytes:policy.partitions.rust.capacityBytes}));},control=mkdtempSync(join(tmpdir(),"budget-pin-")),base=(()=>{try{pinRust(control);return repositoryCacheBudgetV1(control);}finally{rmSync(control,{recursive:true,force:true});}})();
 for(const row of rows){
  const root=mkdtempSync(join(tmpdir(),"budget-pin-"));
  try{
   pinRust(root);const directory=join(root,declared.directory);mkdirSync(directory,{recursive:true});
   if(row.receipt!==null)writeFileSync(join(directory,"image.json"),typeof row.receipt==="string"?row.receipt:JSON.stringify(row.receipt));
   expect(pinnedCapacityV1(directory,declared.capacityBytes),row.id).toBe(row.expected);expect(independent[row.id],row.id).toBe(row.expected);
   const budget=repositoryCacheBudgetV1(root),partition=budget.partitions.dependencies,born=row.id==="no-receipt"?hostShareCapacityV1(declared.capacityBytes,declared.hostSharePercent,privateRepositoryHostFreeBytesV1):row.expected;
   expect(partition.capacityBytes,row.id).toBe(born);expect(partition.filesystemBytes,row.id).toBe(born-unit);
   expect(budget.partitionBytes-base.partitionBytes,row.id).toBe(born-base.partitions.dependencies.capacityBytes);
   expect(budget.maximumBytes-base.maximumBytes,row.id).toBe(born-base.partitions.dependencies.capacityBytes);
   expect(budget.headroomBytes,row.id).toBe(base.headroomBytes);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

/** 📐️ An elastic image honours its receipt like a pinned one: clamped down to the rule maximum, never snapped up to the declared minimum, and compared with an independent Python model. */
test("elastic capacity reads image receipts like pinned capacity",()=>{
 const {effectiveCapacityV1}=require("../📐️elastic/🟦️.ts"),mib=1048576,minimum=64*mib,rule={needBytes:minimum,headroomPercent:0,hostFreeReservePercent:0,alignBytes:mib,maximumBytes:256*mib,automatic:true,pressureFreeBytes:0},
 rows=[{id:"smaller",bytes:32*mib},{id:"between",bytes:128*mib},{id:"above-maximum",bytes:512*mib},{id:"unaligned",bytes:128*mib+1},{id:"below-floor",bytes:mib},{id:"text",bytes:"x"},{id:"malformed",bytes:null}],
 model=`import json,sys
m,x,rows=int(sys.argv[1]),int(sys.argv[2]),json.loads(sys.argv[3])
out={}
for r in rows:
    v=r["bytes"]
    ok=isinstance(v,int) and not isinstance(v,bool) and v%1048576==0 and v>=33554432
    out[r["id"]]=min(v,x) if ok else m
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,String(minimum),String(rule.maximumBytes),JSON.stringify(rows)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);const independent=JSON.parse(run.stdout.toString());
 for(const row of rows){
  const root=mkdtempSync(join(tmpdir(),"budget-elastic-"));
  try{writeFileSync(join(root,"image.json"),row.bytes===null?"{":JSON.stringify({maximumPhysicalBytes:row.bytes}));expect(effectiveCapacityV1(root,minimum,rule),row.id).toBe(independent[row.id]);}finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("a tiny foreign receipt of one partition is ignored for that partition and never breaks the budget of the repository",()=>{
 const root=mkdtempSync(join(tmpdir(),"budget-tiny-"));
 try{
  mkdirSync(join(root,".cache","b"),{recursive:true});const rustDirectory=join(root,policy.partitions.rust.directory);mkdirSync(rustDirectory,{recursive:true});writeFileSync(join(rustDirectory,"image.json"),JSON.stringify({maximumPhysicalBytes:policy.partitions.rust.capacityBytes}));
  const declared=policy.partitions.dependencies,directory=join(root,declared.directory);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"image.json"),JSON.stringify({maximumPhysicalBytes:33554432}));
  expect(pinnedCapacityV1(directory,declared.capacityBytes)).toBe(33554432);
  const budget=repositoryCacheBudgetV1(root);expect(budget.partitions.dependencies.maximumLogicalBytes).toBeGreaterThan(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the bloat rule recognises host blocks an idle image no longer uses, and the elastic partitions carry the rule; agrees with an independent Python model",()=>{
 const {elasticBloatedV1}=require("../📐️elastic/🟦️.ts"),GiB=2**30,rows:[number,number,number][]=[];
 for(const allocated of[0,GiB,3*GiB,10*GiB,24*GiB,45*GiB])for(const used of[0,GiB/2,GiB,3*GiB,12*GiB])for(const free of[2*GiB,20*GiB,47*GiB,49*GiB,300*GiB])rows.push([allocated,used,free]);
 for(const name of["graph","products","rust"] as const){
  const rule=policy.elastic[name];expect(new Ajv({strict:true}).compile(schema.$defs.elasticPartition)(rule)).toBe(true);
  const model=`import json,sys
rule,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out=[]
for a,u,f in rows:
    tight=f<rule["reclaimTightHostFreeBytes"]
    floor=rule["reclaimFloorBytes"]//8 if tight else rule["reclaimFloorBytes"]
    out.append(a-u>=floor and (tight or a*100>=u*rule["reclaimRatioPercent"]))
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(rows)]);
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  const independent=JSON.parse(run.stdout.toString()) as boolean[];
  rows.forEach(([a,u,f],i)=>expect(elasticBloatedV1(rule,a,u,f),name+" "+[a,u,f]).toBe(independent[i]));
  expect(elasticBloatedV1(rule,24*GiB,2.7*GiB,20*GiB)).toBe(true);
  expect(elasticBloatedV1(rule,rule.reclaimFloorBytes/2,0,300*GiB)).toBe(false);
 }
});

test("a pressed elastic image grows by half within the host and the maximum, the grown size outlives the reset and the next image is born with it; agrees with an independent Python model",()=>{
 const {elasticGrowthV1,effectiveCapacityV1,writeElasticGrowthV1}=require("../📐️elastic/🟦️.ts"),GiB=2**30,rows:[number,number,number][]=[];
 for(const capacity of[6*GiB,12*GiB,24*GiB,32*GiB])for(const free of[2*GiB,10*GiB,30*GiB,92*GiB,500*GiB])for(const allocated of[0,6*GiB])rows.push([capacity,free,allocated]);
 for(const name of["graph","products","rust"] as const){
  const rule=policy.elastic[name],model=`import json,sys,math
rule,rows=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out=[]
for c,f,a in rows:
    ceiling=min(rule["maximumBytes"],(f+a)*(100-rule["hostFreeReservePercent"])//100)
    wanted=-(-(c*3//2)//rule["alignBytes"])*rule["alignBytes"]
    target=min(wanted,ceiling)//rule["alignBytes"]*rule["alignBytes"]
    out.append(max(c,target))
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(rows)]);
  expect(run.exitCode,run.stderr.toString()).toBe(0);
  const independent=JSON.parse(run.stdout.toString()) as number[];
  rows.forEach(([c,f,a],i)=>expect(elasticGrowthV1(rule,c,f,a),name+" "+[c,f,a]).toBe(independent[i]));
 }
 const graph=policy.elastic.graph;expect(elasticGrowthV1(graph,12*GiB,400*GiB)).toBe(18*GiB);expect(elasticGrowthV1(graph,6*GiB,92*GiB)).toBe(7*GiB);expect(elasticGrowthV1(graph,24*GiB,500*GiB)).toBe(24*GiB);
 const root=mkdtempSync(join(tmpdir(),"elastic-growth-"));
 try{
  const directory=join(root,"g");mkdirSync(directory);mkdirSync(join(root,"c"));
  const born=effectiveCapacityV1(directory,graph.minimumBytes,graph);
  writeElasticGrowthV1(directory,born+6*GiB);
  const allowed=Math.floor(privateRepositoryHostFreeBytesV1*(100-graph.hostFreeReservePercent)/100/graph.alignBytes)*graph.alignBytes;expect(effectiveCapacityV1(directory,graph.minimumBytes,graph)).toBe(Math.max(born,Math.min(born+6*GiB,graph.maximumBytes,allowed)));
  expect(effectiveCapacityV1(directory,graph.minimumBytes,graph)).toBeLessThanOrEqual(graph.maximumBytes);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("every partition has a bloat rule, the common one is schema-valid, and the rust image is sized from its working set instead of its cap",()=>{
 const {repositoryPartitionReclaimRuleV1}=require("../🟦️.ts"),{elasticCapacityV1}=require("../📐️elastic/🟦️.ts"),GiB=2**30;
 expect(new Ajv({strict:true}).compile(schema.$defs.reclaimRule)(policy.reclaim)).toBe(true);
 for(const name of Object.keys(policy.partitions))expect(repositoryPartitionReclaimRuleV1(name)).toEqual((policy.elastic as Record<string,unknown>)[name]??policy.reclaim);
 expect(policy.elastic.rust.needBytes).toBeLessThan(policy.elastic.rust.maximumBytes);
 expect(elasticCapacityV1(policy.elastic.rust,policy.elastic.rust.minimumBytes,7*GiB)).toBe(policy.elastic.rust.minimumBytes);
 expect(elasticCapacityV1(policy.elastic.rust,policy.elastic.rust.minimumBytes,400*GiB)).toBe(40*GiB);
});

/** 🧮️ The disk policy is declared as fractions of the host: no absolute minimum exceeds what the host can create, every logical limit fits its capacity, and the images born on a 16 GB / 60 GB, a 32 GB / 200 GB and a Windows 256 GB / 5 GB-free machine add up to less than the host's free bytes; the elastic minimum agrees with an independent Python model. */
test("the declared disk policy fits the host: logical limits within capacity, shares within the host, born images within free bytes on the three profiles",()=>{
 const GiB=2**30,unit=1048576,profiles=[{id:"16 GB RAM, 60 GB free",free:56*GiB},{id:"32 GB RAM, 200 GB free",free:186*GiB},{id:"Windows 256 GB disk, 5 GB free",free:5*10**9},{id:"tiny 1 GB free",free:GiB}];
 for(const[id,row]of Object.entries(policy.partitions)){expect(row.maximumLogicalBytes,id).toBeLessThanOrEqual(row.capacityBytes-unit);expect(readRepositoryCacheBudgetV1(policy).partitions[id as keyof typeof policy.partitions].maximumLogicalBytes,id).toBe(row.maximumLogicalBytes);}
 for(const[id,rule]of Object.entries(policy.elastic)){expect(rule.minimumBytes,id).toBeLessThanOrEqual(rule.maximumBytes);}
 const shares=Object.keys(policy.partitions).map(id=>hostSharePercentV1(policy as never,id));
 expect(shares.every(share=>share>0),String(shares)).toBe(true);expect(shares.reduce((sum,share)=>sum+share,0)).toBeLessThanOrEqual(90);
 const model=`import json,sys
pol,profiles=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out={}
for p in profiles:
    total=0;free=p["free"]
    for pid,part in pol["partitions"].items():
        rule=pol.get("elastic",{}).get(pid)
        if rule:
            share=free*(100-rule["hostFreeReservePercent"])//100
            minimum=max(rule["floorBytes"],min(rule["minimumBytes"],share//rule["alignBytes"]*rule["alignBytes"]))
            wanted=-(-rule["needBytes"]*(100+rule["headroomPercent"])//100)
            ceiling=min(rule["maximumBytes"],share)
            target=min(wanted,max(ceiling,minimum))
            total+=max(minimum,target//rule["alignBytes"]*rule["alignBytes"])
        else:
            share=free*part["hostSharePercent"]//100//1048576*1048576
            need=-(-part.get("needBytes",{}).get("darwin",0)//1048576)*1048576
            total+=max(min(part["capacityBytes"],67108864),min(part["capacityBytes"],max(share,need)))
    out[p["id"]]=total
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(policy),JSON.stringify(profiles)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);const independent=JSON.parse(run.stdout.toString());
 for(const profile of profiles){
  let total=0;
  for(const[id,row]of Object.entries(policy.partitions)){
   const rule=(policy.elastic as Record<string,typeof policy.elastic.rust>)[id],born=rule?elasticCapacityV1(rule,elasticMinimumV1(rule,rule.minimumBytes,profile.free),profile.free):hostShareCapacityV1(row.capacityBytes,(row as {hostSharePercent:number}).hostSharePercent,profile.free,partitionNeedBytesV1(policy as never,id,"darwin"));
   expect(born%unit,profile.id+" "+id).toBe(0);expect(born,profile.id+" "+id).toBeLessThanOrEqual(rule?rule.maximumBytes:row.capacityBytes);total+=born;
  }
  expect(total,profile.id).toBe(independent[profile.id]);if(profile.free>=40*GiB)expect(total,profile.id).toBeLessThanOrEqual(profile.free);
 }
 expect(elasticMinimumV1(policy.elastic.rust,policy.elastic.rust.minimumBytes,56*GiB)).toBeLessThan(policy.elastic.rust.minimumBytes);
 expect(elasticMinimumV1(policy.elastic.rust,policy.elastic.rust.minimumBytes,186*GiB)).toBe(policy.elastic.rust.minimumBytes);
});

const GiB=2**30,mib=1048576,pinnedRoot=(prepare:(root:string)=>void=()=>{})=>{const root=mkdtempSync(join(tmpdir(),"budget-fit-"));mkdirSync(join(root,".cache","b"),{recursive:true});prepare(root);return root;};
const withPinnedHost=<T>(bytes:number,body:()=>T):T=>{pinHostFreeBytesForSizingV1(bytes);try{return body();}finally{pinHostFreeBytesForSizingV1(privateRepositoryHostFreeBytesV1);}};

/** 📏️ A fixed image is born at least at its declared need: on a busy host the share of the free bytes (20 % of 18 GiB is 3.6 GiB) no longer decides below what the writers of the partition need, and the independent Python model agrees on every platform. */
test("a fixed image is sized from its declared need and never below it by the share of a busy host",()=>{
 const declared=policy.partitions.dependencies,needs=declared.needBytes as Record<string,number>,hosts=[8*GiB,12*GiB,18*GiB,22*GiB,56*GiB,186*GiB],
 model=`import json,sys
d,pct,needs,hosts=json.loads(sys.argv[1]),int(sys.argv[2]),json.loads(sys.argv[3]),json.loads(sys.argv[4])
out={}
for platform,need in needs.items():
    n=-(-need//1048576)*1048576
    out[platform]=[max(min(d,67108864),min(d,max(free*pct//100//1048576*1048576,n))) for free in hosts]
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,String(declared.capacityBytes),String(declared.hostSharePercent),JSON.stringify(needs),JSON.stringify(hosts)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);const independent=JSON.parse(run.stdout.toString()) as Record<string,number[]>;
 for(const[platform,need]of Object.entries(needs)){
  const born=hosts.map(free=>hostShareCapacityV1(declared.capacityBytes,declared.hostSharePercent,free,need));
  expect(born,platform).toEqual(independent[platform]!);for(const bytes of born){expect(bytes,platform).toBeGreaterThanOrEqual(need);expect(bytes%mib,platform).toBe(0);expect(bytes,platform).toBeLessThanOrEqual(declared.capacityBytes);}
 }
 expect(hostShareCapacityV1(declared.capacityBytes,declared.hostSharePercent,18.5*GiB)).toBe(Math.floor(18.5*GiB*20/100/mib)*mib);
 expect(hostShareCapacityV1(declared.capacityBytes,declared.hostSharePercent,18.5*GiB,needs.darwin)).toBeGreaterThanOrEqual(needs.darwin);
 const need=partitionNeedBytesV1(policy as never,"dependencies"),root=pinnedRoot();
 try{const born=withPinnedHost(18*GiB,()=>repositoryCacheBudgetV1(root).partitions.dependencies);expect(born.capacityBytes).toBe(hostShareCapacityV1(declared.capacityBytes,declared.hostSharePercent,18*GiB,need));expect(born.capacityBytes).toBeGreaterThanOrEqual(need);expect(born.usableBytes).toBeGreaterThanOrEqual(need*0.9);}
 finally{rmSync(root,{recursive:true,force:true});}
});

/** 🛑️ The ledger share is part of the need: what the dependencies store keeps after the nested areas of its partition still holds the install of every platform's backend, and the declared capacity is the smallest that does (compared with an independent Python search over the image overhead). */
test("the declared dependencies need leaves the store its install after the ledger's nested shares and is the smallest such capacity",async()=>{
 const {cacheLedgerV1}=await import("../../../🧾️ledger/🟦️.ts"),{concurrentRequiredImageBytesV1}=await import("../../../../🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts"),{nativeImageUsableBytesV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📏️capacity/🟦️.ts"),
 shares=cacheLedgerV1().areas.filter(area=>area.partition==="dependencies"&&area.kind==="nested").reduce((sum,area)=>sum+(area.budget&&"basisPoints" in area.budget?area.budget.basisPoints:0),0),needs=policy.partitions.dependencies.needBytes as Record<string,number>,backends={darwin:true,linux:false,win32:false};
 expect(shares).toBeGreaterThan(0);
 for(const[platform,sharing]of Object.entries(backends)){
  const required=concurrentRequiredImageBytesV1(sharing),need=needs[platform]!;
  expect(need%mib,platform).toBe(0);expect(need,platform).toBeLessThanOrEqual(policy.partitions.dependencies.capacityBytes);
  expect(nativeImageUsableBytesV1(need-mib)*(10000-shares)/10000,platform).toBeGreaterThanOrEqual(required);
  expect(nativeImageUsableBytesV1(need-2*mib)*(10000-shares)/10000,platform).toBeLessThan(required);
  const search=`import json,sys
required,shares,overhead=int(sys.argv[1]),int(sys.argv[2]),json.loads(sys.argv[3])
c=-(-required//1048576)*1048576+1048576
while overhead[str(c)]*(10000-shares)//10000<required:c+=1048576
print(c)`,first=Math.ceil(required/mib)*mib+mib,table=Object.fromEntries(Array.from({length:(need-first)/mib+1},(_,index)=>[String(first+index*mib),nativeImageUsableBytesV1(first+index*mib-mib)])),run=Bun.spawnSync(["python3","-I","-c",search,String(required),String(shares),JSON.stringify(table)]);
  expect(run.exitCode,run.stderr.toString()).toBe(0);expect(Number(run.stdout.toString().trim()),platform).toBeLessThanOrEqual(need);
 }
});

/** 🚫️ A host that cannot give what an image needs is refused before the image is born, in both languages and with both numbers; a born image is never refused, so the commands that free space still open it. */
test("an image the host cannot hold is refused up front, a born image or a partition without a need is not",()=>{
 const rust=policy.elastic.rust.floorBytes,fits=(receipt:Parameters<typeof requireImageFitsHostV1>[1],required:number,free:number|undefined)=>requireImageFitsHostV1("rust",receipt,required,free);
 expect(()=>fits("missing",rust,rust-1)).toThrow(ImageDoesNotFitHostErrorV1);
 try{fits("missing",rust,12*GiB);throw Error("not refused");}catch(error){const refusal=error as ImageDoesNotFitHostErrorV1;expect(refusal.code).toBe("image-does-not-fit-host");expect(refusal.en).toContain("16384 MiB");expect(refusal.en).toContain("12288 MiB");expect(refusal.de).toContain("16384 MiB");expect(refusal.de).toContain("12288 MiB");expect(refusal.message).toBe(refusal.en+" | "+refusal.de);expect(refusal.de).toContain("Speicherplatz");}
 expect(()=>fits("missing",rust,rust,undefined)).not.toThrow();expect(()=>fits("missing",rust,rust,rust)).not.toThrow();expect(()=>fits(4*GiB,rust,GiB)).not.toThrow();expect(()=>fits(null,rust,GiB)).not.toThrow();expect(()=>fits("missing",0,0)).not.toThrow();
 const rustRoot=pinnedRoot(),bornRoot=pinnedRoot(root=>{mkdirSync(join(root,policy.partitions.rust.directory),{recursive:true});writeFileSync(join(root,policy.partitions.rust.directory,"image.json"),JSON.stringify({maximumPhysicalBytes:4*GiB}));});
 try{
  expect(()=>withPinnedHost(12*GiB,()=>requireRepositoryPartitionFitsHostV1(rustRoot,"rust"))).toThrow(ImageDoesNotFitHostErrorV1);
  expect(()=>withPinnedHost(12*GiB,()=>requireRepositoryPartitionFitsHostV1(bornRoot,"rust"))).not.toThrow();
  expect(()=>withPinnedHost(40*GiB,()=>requireRepositoryPartitionFitsHostV1(rustRoot,"rust"))).not.toThrow();
  const need=repositoryPartitionNeedBytesV1("dependencies");expect(need).toBeGreaterThan(0);
  expect(()=>withPinnedHost(need-mib,()=>requireRepositoryPartitionFitsHostV1(rustRoot,"dependencies"))).toThrow(ImageDoesNotFitHostErrorV1);
  expect(()=>withPinnedHost(1,()=>requireRepositoryPartitionFitsHostV1(rustRoot,"leases"))).not.toThrow();
 }finally{rmSync(rustRoot,{recursive:true,force:true});rmSync(bornRoot,{recursive:true,force:true});}
});

/** 🧱️ The elastic image grows to its floor on every path or is refused: the minimum is cut by the host share only down to the floor (independent Python model), the budget and the strict storage of the compiler answer the same capacity on every host, a rule whose minimum, floor and maximum agree is a fixed size that the host never changes, and a born image below the floor is refused for compiles and due for regrow. */
test("the elastic image is born at least at its floor, the budget and the compiler agree on every host, and a fixed-size rule stays fixed",()=>{
 const rule=policy.elastic.rust,hosts=[6*GiB,12*GiB,18*GiB,22*GiB,56*GiB,100*GiB,186*GiB],
 model=`import json,sys
r,hosts=json.loads(sys.argv[1]),json.loads(sys.argv[2])
out=[]
for free in hosts:
    share=free*(100-r["hostFreeReservePercent"])//100
    minimum=max(r["floorBytes"],min(r["minimumBytes"],share//r["alignBytes"]*r["alignBytes"]))
    wanted=-(-r["needBytes"]*(100+r["headroomPercent"])//100)
    target=min(wanted,max(min(r["maximumBytes"],share),minimum))
    out.append(max(minimum,target//r["alignBytes"]*r["alignBytes"]))
print(json.dumps(out))`,run=Bun.spawnSync(["python3","-I","-c",model,JSON.stringify(rule),JSON.stringify(hosts)]);
 expect(run.exitCode,run.stderr.toString()).toBe(0);const independent=JSON.parse(run.stdout.toString()) as number[];
 for(const[id,row]of Object.entries(policy.elastic))expect(row.floorBytes,id).toBeLessThanOrEqual(row.minimumBytes);
 const born=hosts.map(free=>elasticCapacityV1(rule,elasticMinimumV1(rule,rule.minimumBytes,free),free));expect(born).toEqual(independent);for(const bytes of born)expect(bytes).toBeGreaterThanOrEqual(rule.floorBytes);
 for(const free of hosts){
  const root=pinnedRoot();
  try{
   const [budget,envelope]=withPinnedHost(free,()=>[repositoryCacheBudgetV1(root).partitions.rust,repositoryStrictStorageV1(root)]);
   expect(budget.capacityBytes,String(free)).toBe(envelope.capacityBytes);expect(budget.filesystemBytes,String(free)).toBe(envelope.filesystemBytes);expect(budget.capacityBytes,String(free)).toBe(independent[hosts.indexOf(free)]!);expect(budget.capacityBytes).toBeGreaterThanOrEqual(rule.floorBytes);expect(envelope.maximumWorkspaceBytes*2+envelope.maximumArtifactBytes).toBeLessThanOrEqual(envelope.filesystemBytes);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 const fixed={...rule,minimumBytes:16*GiB,floorBytes:16*GiB,maximumBytes:16*GiB,needBytes:16*GiB,headroomPercent:0},directory=join(tmpdir(),"fixed-"+Math.random(),".cache","b","r");
 for(const free of hosts)expect(withPinnedHost(free,()=>effectiveCapacityV1(directory,fixed.minimumBytes,fixed)),String(free)).toBe(16*GiB);
 expect(()=>requireImageAtFloorV1("rust",4*GiB,rule.floorBytes)).toThrow(ImageBelowFloorErrorV1);
 try{requireImageAtFloorV1("rust",4*GiB,rule.floorBytes);}catch(error){const refusal=error as ImageBelowFloorErrorV1;expect(refusal.en).toContain("4096 MiB");expect(refusal.en).toContain("16384 MiB");expect(refusal.de).toContain("4096 MiB");expect(refusal.de).toContain("Abbild");expect(refusal.message).toBe(refusal.en+" | "+refusal.de);}
 expect(()=>requireImageAtFloorV1("rust",rule.floorBytes,rule.floorBytes)).not.toThrow();
 const compiler=readFileSync(new URL("../../🏗️compiler/🟦️.ts",import.meta.url),"utf8");expect(compiler).toContain('requireImageAtFloorV1("rust",declaration.capacityBytes,repositoryPartitionNeedBytesV1("rust"))');expect(()=>requireImageAtFloorV1("tools",mib,0)).not.toThrow();
 expect([[4*GiB,12*GiB,0],[4*GiB,12*GiB,4*GiB],[4*GiB,1*GiB,0],[rule.floorBytes,1*GiB,0],["missing",40*GiB,0],[null,40*GiB,0]].map(([receipt,free,held])=>elasticBelowFloorDueV1(receipt as never,rule.floorBytes,free as number,held as number))).toEqual([false,true,false,false,false,false]);
});
