/** 📇️ Contract: a pinned build graph is never one string. It is a small manifest and content-addressed gzip records (one per project, one per chunk of dependencies and of external nodes); every compressed byte is counted as it is produced, a record is verified against its name before it is parsed, identical long arrays are shared again on decode, and a graph whose serialized text would exceed the old whole-string limit round-trips with bounded memory. */
import {expect,test} from "bun:test";
import {mkdtempSync,mkdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv2020 from "ajv/dist/2020";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json";
import policy from "../../🔣️policy.json";
import schema from "../../🧬️schema/🔣️.json";
import {decodeGraphRecordsV1,encodeGraphRecordsV1,graphRecordDigestsV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),canonical=(value:unknown):string=>JSON.stringify(value,(_key,item)=>item&&typeof item==="object"&&!Array.isArray(item)?Object.fromEntries(Object.entries(item).sort(([a],[b])=>a<b?-1:a>b?1:0)):item);

test("the declared record limits are admitted by the policy schema",()=>{expect(new Ajv2020({strict:true}).validate(schema,policy)).toBe(true);});

test("a graph is encoded as one record per project, per chunk of dependencies and of external nodes, and decodes to the same graph",async()=>{
 const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes});
 const manifest=JSON.parse(encoded.manifestBytes.toString("utf8"));
 expect(manifest.projects.length).toBe(corpus.expected.projectRecords);expect(manifest.dependencies.length).toBe(corpus.expected.dependencyRecords);expect(manifest.externalNodes.length).toBe(corpus.expected.externalRecords);expect(encoded.records.size).toBeLessThanOrEqual(corpus.expected.recordsTotal);
 expect(manifest.projects.find((row:string[])=>row[0]==="delta")![1]).not.toBe(manifest.projects.find((row:string[])=>row[0]==="twin")![1]);
 expect(graphRecordDigestsV1(encoded.manifestBytes).size).toBe(encoded.records.size);
 const decoded=await decodeGraphRecordsV1(encoded.manifestBytes,async digest=>encoded.records.get(digest),{policy:corpus.policy as never});
 expect(canonical(decoded)).toBe(canonical(corpus.graph));
 expect(decoded.projects.alpha.targets.build.inputs).toBe(decoded.projects.beta.targets.build.inputs);
});

test("a record that differs from its name, a missing record and an open manifest are refused; a graph beyond the byte limit is refused while it is encoded",async()=>{
 const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes}),first=[...encoded.records.keys()][0]!;
 const read=(replace:(digest:string)=>Buffer|undefined)=>decodeGraphRecordsV1(encoded.manifestBytes,async digest=>replace(digest),{policy:corpus.policy as never});
 await expect(read(digest=>digest===first?Buffer.from("tampered"):encoded.records.get(digest))).rejects.toThrow("Graph record content differs from its name");
 await expect(read(digest=>digest===first?undefined:encoded.records.get(digest))).rejects.toThrow("Graph record content differs from its name");
 await expect(decodeGraphRecordsV1(Buffer.from(JSON.stringify({...JSON.parse(encoded.manifestBytes.toString()),extra:1})),async digest=>encoded.records.get(digest),{policy:corpus.policy as never})).rejects.toThrow("Closed graph record manifest required");
 await expect(encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:200})).rejects.toThrow("Build graph exceeds finite snapshot byte limit");
 await expect(encodeGraphRecordsV1(corpus.graph,{policy:{...corpus.policy,maximumRecordBytes:20} as never,maximumBytes:policy.maximumSnapshotBytes})).rejects.toThrow("Graph record exceeds its finite size");
});

test("the independent python reader rebuilds the same graph from the written records",async()=>{
 if(!python)return;
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-records-"));
 try{
  const encoded=await encodeGraphRecordsV1(corpus.graph,{policy:corpus.policy as never,maximumBytes:policy.maximumSnapshotBytes});
  writeFileSync(join(dir,encoded.digest+".json"),encoded.manifestBytes);for(const[digest,bytes]of encoded.records)writeFileSync(join(dir,digest+".gz"),bytes);
  const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),dir,encoded.digest]);expect(result.exitCode).toBe(0);
  expect(canonical(JSON.parse(result.stdout.toString()))).toBe(canonical(corpus.graph));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("a 5,000 project graph whose text exceeds the old whole-string limit encodes and decodes with bounded memory",async()=>{
 const result=Bun.spawnSync([process.execPath,join(here,"🧱️heap","🟨️.mjs"),"5000","4000"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const row=JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!);
 expect(row.projects).toBe(5000);expect(row.raw).toBeGreaterThan(policy.maximumSnapshotBytes);expect(row.compressed).toBeLessThan(row.raw/8);expect(row.compressed).toBeLessThan(policy.maximumSnapshotBytes);
 expect(row.equal).toBe(true);expect(row.edges).toBe(true);expect(row.sameShare).toBe(true);
 expect(row.encodePeak-row.baseline).toBeLessThan(768*1048576);expect(row.decodePeak-row.baseline).toBeLessThan(1024*1048576);
},300000);
