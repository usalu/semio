import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";
import{brotliDecompressSync}from"node:zlib";
import{buildSelectedGenerationV1}from"../../🟦️.ts";
import{makeMachine,makeRecords,ownerOfV1}from"../../🧪️tests/🌍️world/🟦️.ts";
import{SELECTED_STREAM_POLICY_V1 as policy,streamSelectedDocumentsV1}from"../🟦️.ts";

const sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
const unpack=(blobs:ReadonlyMap<string,Uint8Array>,name:string)=>{const parts=[...blobs].filter(([key])=>key===name+".br"||key.startsWith(name+".br.")).sort(([a],[b])=>Number(a.split(".").pop())-Number(b.split(".").pop())).map(([,bytes])=>bytes);return brotliDecompressSync(Buffer.concat(parts));};

/** 🟰️ The streamed documents are the exchange's documents: decompressed graph, owners and admission bytes equal `buildSelectedGenerationV1` byte for byte, the owner digests equal its head, every blob is named and hashed as the head lists it, in head order. */
test("the streamed documents equal the exchange's documents byte for byte and its owner digests",async()=>{
 for(const count of[3,9]){
  const records=makeRecords(),machine=makeMachine({name:"a",count,records}),sealed=(await machine.read(undefined))!;
  const classic=buildSelectedGenerationV1({scope:"s",platform:"p",host:machine.host,sealed,ownerOf:ownerOfV1,now:1}),stored=new Map<string,Uint8Array>();
  const streamed=await streamSelectedDocumentsV1({host:machine.host,sealed,ownerOf:ownerOfV1,put:async blob=>{stored.set(blob.name,blob.bytes);return true;}});
  if(!("owners" in streamed))throw Error(streamed.text);
  expect(streamed.owners).toEqual(classic.head.owners);
  for(const document of classic.documents)expect(Buffer.from(unpack(stored,document.name)).equals(Buffer.from(document.bytes))).toBe(true);
  expect(streamed.blobs.map(blob=>blob.name)).toEqual(classic.head.documents.map(blob=>blob.name));
  for(const blob of streamed.blobs)expect(sha(stored.get(blob.name)!)).toBe(blob.hash);
 }
});

/** ✋️ A held head with the same owners stops the publication before one blob is written. */
test("current owners stop the publication before any blob is put",async()=>{
 const records=makeRecords(),machine=makeMachine({name:"a",count:4,records}),sealed=(await machine.read(undefined))!;let puts=0,asked:Readonly<Record<string,string>>|undefined;
 const streamed=await streamSelectedDocumentsV1({host:machine.host,sealed,ownerOf:ownerOfV1,current:async owners=>{asked=owners;return true;},put:async()=>{puts++;return true;}});
 expect(puts).toBe(0);expect("current" in streamed&&streamed.current).toBe(true);expect(Object.keys(asked!).length).toBeGreaterThan(0);
});

/** 📏️ The publication of a worker-size sealed read (270,000 distinct rows, 2,800 admission entries, about a million row references, a 28 MB graph: the size of the g1 workers' selection) adds at most `peakMegabytes` to the resident size of its process, sampled every 20 ms through the process memory seam from just before the publication (after a full collection) to its end; the same build of the exchange's three documents at once adds 2,806 MB on this fixture and added 2.6 GB on the real rows of the workers' selection (W15, `wave-1122-dist/W15-slice-worker-memory.md`). */
test("a worker-size publication stays within its declared peak",()=>{
 const child=spawnSync(process.execPath,[fileURLToPath(new URL("./📏️peak.ts",import.meta.url))],{encoding:"utf8",timeout:900_000,env:{...process.env,MIMALLOC_PURGE_DELAY:"0"}});
 const line=child.stdout.trim().split("\n").at(-1)??"";
 if(child.status!==0)throw Error("peak child failed: "+child.stderr.slice(-2000));
 const result=JSON.parse(line) as {rows:number;entries:number;references:number;graphMegabytes:number;addedMegabytes:number;blobs:number};
 expect(result.rows).toBeGreaterThanOrEqual(270_000);expect(result.entries).toBe(2_800);expect(result.references).toBeGreaterThanOrEqual(950_000);expect(result.graphMegabytes).toBeGreaterThanOrEqual(28);expect(result.blobs).toBeGreaterThanOrEqual(3);
 expect(result.addedMegabytes).toBeLessThanOrEqual(policy.peakMegabytes);
},900_000);
