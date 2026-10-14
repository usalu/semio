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
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {lstatSync,readdirSync} from "node:fs";

test("closed repository cache partitions agree with independent SQLite byte and path admission",()=>{
 const database=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);
 try{for(const row of fixture.cases){
  const value=structuredClone(policy);if(row.mutation==="oversubscribed")value.partitions.products.capacityBytes=value.maximumBytes;if(row.mutation==="overlapping")value.partitions.tools.directory=value.partitions.rust.directory+"/nested";if(row.mutation==="escaping")value.partitions.tools.directory="../foreign";if(row.mutation==="unaligned")value.partitions.tools.capacityBytes++;if(row.mutation==="missing")delete (value.partitions as any).tools;if(row.mutation==="control-overlap")value.partitions.tools.directory=".cache/b/c/t";
  const encoded=JSON.stringify(value),sum=(database.query("SELECT sum(json_extract(value,'$.capacityBytes')) AS bytes FROM json_each(?,'$.partitions')").get(encoded) as {bytes:number}).bytes;
  const unsafe=(database.query("SELECT count(*) AS count FROM json_each(?,'$.partitions') WHERE json_extract(value,'$.directory') LIKE '%..%' OR json_extract(value,'$.capacityBytes') % 1048576 != 0 OR instr(json_extract(value,'$.directory') || '/', '.cache/b/c/') = 1 OR instr('.cache/b/c/',json_extract(value,'$.directory') || '/') = 1").get(encoded) as {count:number}).count;
  const overlaps=(database.query("SELECT count(*) AS count FROM json_each(?,'$.partitions') a JOIN json_each(?,'$.partitions') b ON a.key < b.key WHERE json_extract(a.value,'$.directory') = json_extract(b.value,'$.directory') OR instr(json_extract(a.value,'$.directory') || '/',json_extract(b.value,'$.directory') || '/') = 1 OR instr(json_extract(b.value,'$.directory') || '/',json_extract(a.value,'$.directory') || '/') = 1").get(encoded,encoded) as {count:number}).count;
  const oracle=validate(value)&&sum+value.maximumControlBytes<=value.maximumBytes&&unsafe===0&&overlaps===0;
  expect(oracle,row.id).toBe(row.accepted);if(row.accepted){const actual:any=readRepositoryCacheBudgetV1(value);expect(actual.partitionBytes).toBe(sum);expect(sum).toBe(fixture.expectedPartitionBytes);expect(actual.headroomBytes).toBe(fixture.expectedHeadroomBytes);}else expect(()=>readRepositoryCacheBudgetV1(value),row.id).toThrow();
 }console.log("[DEBUG] Closed seven-partition cache ceiling and independent SQLite byte/path oracle agree");}finally{database.close();}
});

test("owned process probe captures exact bytes inside actual fixed diagnostics storage",async()=>{
 await receiveScriptProcessInvocation(process.env,async original=>withRepositoryCacheStoreV1(process.cwd(),"diagnostics",original,async store=>{
  const names=["SEMIO_DIAGNOSTIC_DIRECTORY"] as const,previous=Object.fromEntries(names.map(name=>[name,process.env[name]])),directory=store.environment.SEMIO_DIAGNOSTIC_DIRECTORY!;for(const name of names)process.env[name]=directory;
  const database=new Database(":memory:");try{const before=readdirSync(directory).sort(),script='process.stdout.write(Buffer.from(process.argv[1],"base64"));process.stderr.write(Buffer.from(process.argv[2],"base64"));',result=runProbe("node",["-e",script,Buffer.from(fixture.probe.stdout).toString("base64"),Buffer.from(fixture.probe.stderr).toString("base64")],{cwd:process.cwd(),env:store.environment,budgetMs:5000}),oracle=database.query("SELECT length(cast(? AS blob)) AS bytes").get(fixture.probe.stdout) as {bytes:number};expect(result.stdout).toBe(fixture.probe.stdout);expect(result.stderr).toBe(fixture.probe.stderr);expect(result.status).toBe(fixture.probe.status);expect(Buffer.byteLength(result.stdout)).toBe(oracle.bytes);expect(readdirSync(directory).sort()).toEqual(before);expect(lstatSync(store.image.image).blocks*512).toBeLessThanOrEqual(store.image.maximumPhysicalBytes);await store.recheck();console.log("[DEBUG] Actual fixed2GiB diagnostic backing contains exact Node probe bytes and retires its owned temporary capture");}finally{database.close();for(const name of names)if(previous[name]===undefined)delete process.env[name];else process.env[name]=previous[name];}
 }));
},120000);

test("ordinary cleanup cannot retire the authored fixed cache backings",()=>{
 const database=new Database(":memory:"),root=process.cwd(),protectedPaths=cleanProtectedPrefixes(root),view={kind:()=>"missing" as const,children:()=>[],read:()=>undefined};
 try{for(const row of Object.values(policy.partitions)){const path=resolve(root,row.directory),oracle=database.query("SELECT ? || '/' || ? AS path").get(root,row.directory) as {path:string};expect(path).toBe(oracle.path);expect(protectedPaths).toContain(path);for(const candidate of fixture.cleanupSuffixes.map(suffix=>suffix?join(path,suffix):path)){expect(cleanRemovalProtection(root,candidate,view)).toContain(candidate);for(const incremental of [false,true])expect(cleanRemovePath(root,candidate,true,[],false,false,incremental)).toBe(false);}}for(const relative of [...fixture.protectedAncestors,fixture.bootstrapControlDirectory]){const path=resolve(root,relative);expect(cleanRemovalProtection(root,path,view)).toContain(path);expect(cleanRemovePath(root,path,true,[],true,true,true)).toBe(false);}console.log("[DEBUG] Ordinary cleanup refuses every authored fixed backing and its mounted data without physical retirement custody");}finally{database.close();}
});
