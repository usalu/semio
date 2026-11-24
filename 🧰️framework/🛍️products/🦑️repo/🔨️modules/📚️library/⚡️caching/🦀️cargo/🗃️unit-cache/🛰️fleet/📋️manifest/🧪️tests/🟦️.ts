import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {parseRemoteCacheActionResultV1} from "../../../../../../🏗️build/🌐️remote-cache/🔬️action-result/🟦️.ts";
import {judgeStalenessV1,openTreeDigestsV1} from "../../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {provenanceClassV1,PROVENANCE_SUPERSEDING_CLASSES_V1} from "../../../../../../🏗️build/🌐️remote-cache/🧾️provenance/🟦️.ts";
import {readRemoteCacheMetaV1} from "../../../../../../🏗️build/🌐️remote-cache/🏷️retention/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {manifestCoversV1,manifestDeclarationV1,manifestIdV1,manifestOfV1,manifestUnionV1,manifestWireV1,manifestWithinBoundsV1,observedManifestV1,obtainManifestV1,parseManifestV1,readManifestV1,shareManifestV1,writeManifestV1} from "../🟦️.ts";
import vectors from "./🔣️.json" with {type:"json"};

/** 📋️ Contract of the input manifest of a unit slot: its address and wire form are fixed by the vectors beside this file (and recomputed by Python), a manifest only grows, the retention judge keeps it while its crate exists in the tree, and a machine that never compiled the unit finds the manifest on its first lookup. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[],sha=(text:string|Uint8Array)=>createHash("sha256").update(text).digest("hex");
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),`manifest-${label}-`)));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const manifest=manifestOfV1(vectors.manifest.files,vectors.manifest.env),declaration=(workspace:string|undefined,entry:string,digest:string|null)=>manifestDeclarationV1({toolchain:"rustc test",entry,entrySha256:digest,cwd:workspace??"/nowhere",workspace});
const python=testPythonV1();

test("the address of a manifest is the SHA-256 of the namespace, a NUL byte and the slot, as an independent Python computation gives it",()=>{
 for(const slot of vectors.slots)expect(manifestIdV1(slot)).toBe(sha(vectors.namespace+"\0"+slot));
 if(python){
  const run=Bun.spawnSync([python,"-I","-c","import hashlib,json,sys\nv=json.load(sys.stdin)\nprint(json.dumps([hashlib.sha256((v['namespace']+'\\0'+s).encode()).hexdigest() for s in v['slots']]))"],{stdin:new TextEncoder().encode(JSON.stringify(vectors)),stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()).toBe(0);expect(JSON.parse(run.stdout.toString())).toEqual(vectors.slots.map(slot=>manifestIdV1(slot)));
 }
 expect(manifestIdV1(vectors.slots[0]!)).not.toBe(manifestIdV1(vectors.slots[1]!));
});

test("what the fleet cache stores for a manifest names one blob as stage.json and inputs.json, carries the manifest inline and is read back; the declaration is of kind input-manifest",()=>{
 const tree=directory("tree");mkdirSync(join(tree,"crate","src"),{recursive:true});writeFileSync(join(tree,"crate","src","lib.rs"),"x");
 const wire=manifestWireV1(manifest,declaration(tree,join(tree,"crate","src","lib.rs"),sha("x")));
 const parsed=parseRemoteCacheActionResultV1(wire.result);expect(parsed.references.map(row=>row.hash)).toEqual([wire.hash,wire.hash].slice(0,parsed.references.length));
 expect(parseManifestV1(wire.result)).toEqual(manifest);
 const blob=JSON.parse(Buffer.from(wire.blob).toString("utf8")) as {version:number;retention:{kind:string;sources:{path:string}[];externs:string[]};files:string[];env:string[]};
 expect(blob).toMatchObject({version:1,files:[...manifest.files],env:[...manifest.env]});expect(blob.retention).toMatchObject({kind:vectors.declaration.kind,externs:vectors.declaration.externs,sources:[{path:"crate/src/lib.rs"}]});
 expect(provenanceClassV1(vectors.outputs,false)).toEqual({sort:"input-manifest",advisory:false});expect(PROVENANCE_SUPERSEDING_CLASSES_V1).toContain("input-manifest");
 expect(parseManifestV1(wire.result.subarray(0,wire.result.length-5))).toBeUndefined();
 const registry=declaration(tree,"/h/.cargo/registry/src/i/serde_core-1.0.0/src/lib.rs",sha("y"));expect(registry.sources).toEqual([]);
});

test("a manifest only grows: the union covers both, the observed files of a dependency file are portable names, and a manifest beyond its bounds is refused",()=>{
 const base=directory("observed"),roots=[{path:base,token:"@@semio-cwd@@",preferred:true}] as const;
 const found=observedManifestV1(["files/data.json","env:SEMIO_X","dep-info"],{cwd:base,roots});
 expect(found).toEqual({files:["@@semio-cwd@@/files/data.json"],env:["SEMIO_X"]});
 const union=manifestUnionV1(manifest,found);
 expect(manifestCoversV1(union,manifest)).toBe(true);expect(manifestCoversV1(union,found)).toBe(true);expect(manifestCoversV1(manifest,found)).toBe(false);
 expect(manifestCoversV1(undefined,manifestOfV1([],[]))).toBe(true);expect(manifestCoversV1(undefined,found)).toBe(false);
 expect(manifestWithinBoundsV1(manifestOfV1(Array.from({length:512},(_,index)=>"f"+index),[]))).toBe(true);
 expect(manifestWithinBoundsV1(manifestOfV1(Array.from({length:513},(_,index)=>"f"+index),[]))).toBe(false);
});

test("a machine that never compiled the unit finds the manifest with its first lookup; a fresh miss costs one request per half minute; a copy kept locally costs none; a stale server copy is replaced by the union",async()=>{
 const tree=directory("fleet-tree"),cas=directory("cas");mkdirSync(join(tree,"crate","src"),{recursive:true});writeFileSync(join(tree,"crate","src","lib.rs"),"x");
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"},graceMilliseconds:50,staleness:{mode:"enforce",workspace:tree,recheckSeconds:0,manifestNames:["unit.json","stage.json"],examples:3}});servers.push(server);
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas};
 const machineA=directory("a"),machineB=directory("b"),id=manifestIdV1("c".repeat(64)),clientA=createFleetClientV1(readFleetSettingsV1(machineA,environment)!),clientB=createFleetClientV1(readFleetSettingsV1(machineB,environment)!);
 const first=await obtainManifestV1(machineB,id,clientB);expect(first).toEqual({state:undefined,from:"absent"});
 const requests=()=>server.store.usage().entries;
 expect(await obtainManifestV1(machineB,id,clientB),"a fresh miss is remembered").toEqual({state:undefined,from:"absent"});
 const entry=join(tree,"crate","src","lib.rs");
 expect(await shareManifestV1(machineA,id,manifest,declaration(tree,entry,sha("x")),clientA)).toBe("stored");
 expect(readManifestV1(machineA,id)).toMatchObject({shared:true});
 const meta=await readRemoteCacheMetaV1(server.store.root,`main/ac/${id.slice(0,2)}/${id}`);
 expect(meta?.declaration).toMatchObject({kind:"input-manifest",sources:[{path:"crate/src/lib.rs"}]});
 const other=directory("c"),third=await obtainManifestV1(other,id,createFleetClientV1(readFleetSettingsV1(other,environment)!));
 expect(third.from,"a machine with an empty store gets it with its first lookup").toBe("fleet");expect(third.state?.manifest).toEqual(manifest);
 const again=await obtainManifestV1(other,id,undefined);expect(again.from,"and keeps a local copy").toBe("local");
 const grown=manifestUnionV1(manifest,manifestOfV1(["@@semio-ws@@/extra.txt"],[]));
 expect(await shareManifestV1(machineA,id,grown,declaration(tree,entry,sha("x")),clientA),"the newer manifest replaces the older").toBe("stored");
 const fresh=directory("d");expect((await obtainManifestV1(fresh,id,createFleetClientV1(readFleetSettingsV1(fresh,environment)!))).state?.manifest).toEqual(grown);
 expect(requests()).toBeGreaterThan(0);
 const reader=createFleetClientV1(readFleetSettingsV1(machineB,{...environment,SEMIO_REMOTE_CACHE_ACCESS:"read-only"})!);
 expect(await shareManifestV1(machineB,id,grown,declaration(tree,entry,sha("x")),reader),"a read-only machine publishes nothing").toBe("unshared");
 writeManifestV1(machineB,id,grown,false);expect(readManifestV1(machineB,id)).toMatchObject({shared:false});
},120000);

test("the retention judge keeps a manifest while its crate exists in the tree whatever its sources say, retires it when the crate is gone, and never retires one of a crate outside the workspace",async()=>{
 const tree=directory("judge");mkdirSync(join(tree,"crate","src"),{recursive:true});writeFileSync(join(tree,"crate","src","lib.rs"),"one");
 const digests=await openTreeDigestsV1(tree),entry=join(tree,"crate","src","lib.rs");
 const rows=[{key:"a",storedAt:0,refs:[],declaration:declaration(tree,entry,sha("one"))},{key:"b",storedAt:0,refs:[],declaration:declaration(tree,join(tree,"gone","src","lib.rs"),sha("two"))},{key:"c",storedAt:0,refs:[],declaration:declaration(tree,"/h/.cargo/registry/src/i/serde_core-1.0.0/src/lib.rs",sha("three"))}];
 const judge=()=>judgeStalenessV1(rows,{now:10**9,graceMilliseconds:1000,digest:path=>digests.digest(path)});
 writeFileSync(entry,"two");const verdicts=await judge();
 expect(verdicts.get("a"),"an edit of the crate does not retire its manifest").toEqual({state:"live"});
 expect(verdicts.get("b")).toMatchObject({state:"stale",reason:"deleted"});
 expect(verdicts.get("c"),"a crate outside the tree is never stale or unreached").toEqual({state:"live"});
});
