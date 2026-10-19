import{createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import policy from "../🔣️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {readRepositoryCacheBudgetV1} from "../🟦️.ts";
import {cleanProtectedPrefixes,cleanRemovalProtection} from "../../../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts";
import {cleanRemovePath} from "../../../../../🧼️workspace-cleanup/🗑️removal/🟦️.ts";
import {resolve,join} from "node:path";
import {withRepositoryCacheStoreV1} from "../📥️opening/🏪️store/🟦️.ts";
import {runProbe} from "../../../../../🏃️process/🔍️probe/🟦️.ts";
import {withBuildTestInvocationV1} from "@semio-tech/process/testing/invocation";
import {lstatSync,readdirSync,readFileSync} from "node:fs";

/** 🎚️ Keeps authored logical observation limits independent of every physical capacity charge. */
test("closed repository publication logical observation policy leaves the physical ledger unchanged",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema),cases=new Ajv({strict:true}).compile(schema.$defs.logicalCase);try{expect((policy.partitions.tools as any).maximumLogicalBytes).toBe(68719476736);const physical=Object.fromEntries(Object.entries(policy.partitions).map(([id,row])=>[id,row.capacityBytes]));for(const row of fixture.logicalCases){expect(cases(row),row.id).toBe(true);const value:any=structuredClone(policy);if(row.missing)delete value.partitions.tools.maximumLogicalBytes;else value.partitions.tools.maximumLogicalBytes=row.maximumLogicalBytes;const encoded=JSON.stringify(value),oracle=database.query("SELECT json_type(?1,'$.partitions.tools.maximumLogicalBytes') IN('integer','real') AND json_extract(?1,'$.partitions.tools.maximumLogicalBytes')=CAST(json_extract(?1,'$.partitions.tools.maximumLogicalBytes') AS INTEGER) AND json_extract(?1,'$.partitions.tools.maximumLogicalBytes') BETWEEN 1 AND 68719476736 accepted,(SELECT sum(json_extract(value,'$.capacityBytes')) FROM json_each(?1,'$.partitions')) bytes").get(encoded)as{accepted:number|null;bytes:number};expect(validate(value),row.id).toBe(row.accepted);expect(Boolean(oracle.accepted),row.id).toBe(row.accepted);expect(oracle.bytes).toBe(fixture.expectedPartitionBytes);if(row.accepted){const actual:any=readRepositoryCacheBudgetV1(value);expect(actual.partitionBytes).toBe(fixture.expectedPartitionBytes);expect(actual.headroomBytes).toBe(fixture.expectedHeadroomBytes);expect(actual.partitions.tools.maximumLogicalBytes).toBe(row.maximumLogicalBytes);expect(Object.fromEntries(Object.entries(actual.partitions).map(([id,part]:[string,any])=>[id,part.capacityBytes]))).toEqual(physical);}else expect(()=>readRepositoryCacheBudgetV1(value)).toThrow();}expect(physical.tools).toBe(8589934592);expect((policy.partitions.tools as any).maximumLogicalBytes).toBeGreaterThan(physical.tools!);}finally{database.close();}
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
 await withBuildTestInvocationV1(async original=>withRepositoryCacheStoreV1(process.cwd(),"diagnostics",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async store=>{
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

/** 🏗️ Provisions the real nx image end to end and proves its separate mount and exact artifact charge. */
test("the nx partition provisions its own mounted image and charges the artifact envelope",async()=>{
 await withBuildTestInvocationV1(async original=>withRepositoryCacheStoreV1(process.cwd(),"nx",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async store=>{
  const declared=policy.partitions.nx,directory=resolve(process.cwd(),declared.directory);
  expect(store.image.image).toBe(join(directory,"fixed.dmg"));expect(store.root).toBe(join(directory,"m"));
  const backing=lstatSync(store.image.image);expect(backing.blocks*512).toBeGreaterThanOrEqual(backing.size);expect(backing.size).toBeLessThanOrEqual(declared.capacityBytes);expect(lstatSync(store.root).dev).not.toBe(lstatSync(directory).dev);
  const receipt=JSON.parse(readFileSync(join(store.root,"nx.json"),"utf8"));expect(receipt.owner).toBe("repository:nx-store");expect(receipt.bytes).toBe(declared.maximumArtifactBytes);expect(store.storage.allocation.bytes).toBe(declared.maximumArtifactBytes);
  await store.recheck();
 }));
},120000);

test("ordinary cleanup cannot retire the authored fixed cache backings",()=>{
 const database=new Database(":memory:"),root=process.cwd(),protectedPaths=cleanProtectedPrefixes(root),view={kind:()=>"missing" as const,children:()=>[],read:()=>undefined};
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
