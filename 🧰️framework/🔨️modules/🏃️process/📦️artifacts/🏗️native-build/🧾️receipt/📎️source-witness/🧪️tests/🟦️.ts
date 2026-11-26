import {expect,test} from "bun:test";
import {blake3} from "@noble/hashes/blake3.js";
import {createHash} from "node:crypto";
import {mkdir,mkdtemp,readFile,rm,writeFile} from "node:fs/promises";
import {join,resolve} from "node:path";
import {churnProtocolV1,concurrentEditOfV1,retryOnConcurrentEditV1} from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {admitWarmCargoRetryGraphV1,runCargoPassAttemptsV1} from "../../../🧩️component-pass/🔁️failure-churn/🟦️.ts";
import {announcedCargoSourceChecksumsV1,CargoSourceAttemptV1,CargoSourceWitnessV1} from "../🟦️.ts";
import {withLawStorageV1,lawStoragePolicyV1} from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import retryCases from "./🔁️retry.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"cargo-source-witness",maximumElapsedMilliseconds:120_000,capabilities:{artifacts:true}});

test("announced dep-info proves conflicts without reading compiler outputs or source bodies",async()=>{
  const directory=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), "sw-"));
  const build=join(directory,"build"),source=join(directory,"shared.rs"),outside=join(directory,"liboutside.rlib");
  const first=join(build,"libfirst.rlib"),second=join(build,"libsecond.rmeta"),missing=join(build,"missing.wasm");
  const physical={maxBytes:4096,maxWork:32,chunkBytes:1024,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}};
  const revisions=fixture.slice(0,2).map(row=>{
    const bytes=Buffer.from(row.source);
    return {blake3:Buffer.from(blake3(bytes)).toString("hex"),length:bytes.byteLength};
  });
  const dep=(artifact:string,index:number)=>{
    const {blake3,length}=revisions[index]!;
    return `${artifact}: ${source}\n# checksum:blake3=${blake3} file_len:${length} ${source}\n`;
  };
  try{
    await mkdir(build);
    await Promise.all([
      writeFile(join(build,"first.d"),dep(first,0)),
      writeFile(join(build,"libsecond.d"),dep(second,1)),
      writeFile(join(directory,"outside.d"),dep(outside,1)),
    ]);
    await withBuildTestInvocationV1(async invocation=>{
      const control=scriptCompilerAdmissionControlV1(invocation,"test:announced-source-conflict",10000);
      const attempt=new CargoSourceAttemptV1(new Map([[source,"admitted"]]));
      for(const [unit,artifact] of [["first",first],["outside",outside],["missing",missing],["second",second]] as const){
        expect(await attempt.recordArtifact(unit,[artifact],[build],physical,control)).toBe(attempt.conflict);
        if(unit!=="second")expect(attempt.conflict).toBeUndefined();
      }
      const [expected,observed]=revisions.map(row=>row.blake3+":"+row.length).sort();
      expect(attempt.conflict).toEqual({path:source,expected,observed});
    });
  }finally{await rm(directory,{recursive:true,force:true});}
});

test("a fresh Cargo unit anchors its relative consumed paths to rustc's target record",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), "sw-"));
  const build=join(root,"build"),artifact=join(build,"libunit.rlib"),dep=join(build,"unit.d"),source=join(root,"lib.rs");
  const physical={maxBytes:4096,maxWork:32,chunkBytes:1024,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}};
  const claim=(path:string,body:string)=>`# checksum:blake3=${Buffer.from(blake3(new TextEncoder().encode(body))).toString("hex")} file_len:${Buffer.byteLength(body)} ${path}\n`;
  try{
    await mkdir(build);
    await writeFile(dep,`${dep}: lib.rs package/../module.rs\n`+claim("lib.rs",fixture[0]!.source)+claim("package/../module.rs",fixture[1]!.source));
    await withBuildTestInvocationV1(async invocation=>{
      const control=scriptCompilerAdmissionControlV1(invocation,"test:anchored-source",10000);
      const claims=async(cargoWorkspace:string,targetSource:string,sinceMs:number)=>{
        const rows=[];
        for await(const row of announcedCargoSourceChecksumsV1([artifact],[build],physical,control,undefined,{cargoWorkspace,targetSource,sinceMs}))rows.push(row.path);
        return rows;
      };
      expect(await claims(root,source,0)).toEqual([source,join(root,"module.rs")]);
      expect(await claims(root,join(root,"unrelated.rs"),0)).toEqual([]);
      expect(await claims(root,source,Date.now()+10000)).toEqual([]);
      expect(await claims(join(root,"wrong"),source,0)).toEqual([]);
      await writeFile(dep,`${artifact}: lib.rs package/../module.rs\n`+claim("lib.rs",fixture[0]!.source)+claim("package/../module.rs",fixture[1]!.source));
      expect(await claims(root,source,0)).toEqual([]);
    });
  }finally{await rm(root,{recursive:true,force:true});}
});

test("early capture retains only exact admitted Rust source bodies",async()=>{
  const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!);
  const workspace=await mkdtemp(join(artifacts,"sw-"));
  const physical={maxBytes:65536,maxWork:100,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}};
  try{
    await withBuildTestInvocationV1(invocation=>{
      const compiler=scriptCompilerAdmissionControlV1(invocation,"test:early-source-witness",120000);
      return withLawStorageV1(compiler,lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
        const build=join(held.root,"build");
        await mkdir(build);
        const witness=new CargoSourceWitnessV1(workspace,join(held.root,"witness"),held.storage,control);
        try{
          for(const [index,row] of fixture.entries()){
            const source=join(workspace,index+".rs"),artifact=join(build,"libunit"+index+".rlib"),dep=join(build,"unit"+index+".d");
            const consumed=new TextEncoder().encode(row.consumed);
            const admittedSha256=createHash("sha256").update(consumed).digest("hex");
            const expected={path:source,blake3:Buffer.from(blake3(consumed)).toString("hex"),length:consumed.byteLength};
            if(row.admitted)witness.expect(source);
            await writeFile(source,row.source);
            if(row.existed)await writeFile(dep,`${artifact}: ${source}\n# checksum:blake3=${expected.blake3} file_len:${expected.length} ${source}\n`);
            await Promise.all([witness.captureAnnouncedSources([artifact],[build],physical,control),witness.captureAnnouncedSources([artifact],[build],physical,control)]);
            expect(witness.canTransfer(source,expected),row.id).toBe(row.retained);
            expect(witness.matchesAdmitted(expected,admittedSha256),row.id).toBe(row.retained);
            expect(witness.admittedDrift(expected,admittedSha256),row.id).toBeUndefined();
            if(row.retained){
              const changed={...expected,blake3:"2".repeat(64)};
              expect(witness.admittedDrift(changed,admittedSha256),row.id).toEqual({path:source,expected:expected.blake3+":"+expected.length,observed:changed.blake3+":"+changed.length});
              expect(witness.admittedDrift(changed,"0".repeat(64)),row.id).toBeUndefined();
            }
            expect(witness.matchesAdmitted(expected,"0".repeat(64)),row.id).toBe(false);
            expect(witness.selectedPath(source,admittedSha256),row.id).toBeUndefined();
            const live=new TextEncoder().encode(row.source);
            const drift=row.drift?{path:source,expected:expected.blake3+":"+expected.length,observed:Buffer.from(blake3(live)).toString("hex")+":"+live.byteLength}:undefined;
            if(row.existed)expect(await witness.captureClaim(expected,physical,control),row.id).toEqual(drift);
            await writeFile(source,"later");
            if(row.retained){
              const result=await witness.input(source,expected,physical);
              expect(result.retained).toBe(true);
              expect(await witness.captureClaim(expected,physical,control)).toBeUndefined();
              expect(result.row.sha256).toBe(admittedSha256);
              const selected=witness.selectedPath(source,admittedSha256);
              expect(selected).toBeDefined();
              expect(await readFile(selected!,"utf8")).toBe(row.consumed);
              expect(witness.selectedPath(source,"0".repeat(64))).toBeUndefined();
              expect(witness.selectedPath(join(workspace,"other.rs"),admittedSha256)).toBeUndefined();
              witness.noteUncheckedInput(source,admittedSha256);
              expect(witness.selectedPath(source,admittedSha256)).toBeUndefined();
            }else if(row.admitted){
              const profiles=churnProtocolV1().profiles;
              let snapshotAttempts=0,buildAttempts=0;
              await retryOnConcurrentEditV1("build",async build=>{
                buildAttempts++;
                if(build===2){
                  const compiled=Buffer.from("later"),sha256=createHash("sha256").update(compiled).digest("hex");
                  const current={path:source,blake3:Buffer.from(blake3(compiled)).toString("hex"),length:compiled.byteLength};
                  expect((await witness.input(source,current,physical)).row.sha256).toBe(sha256);
                  return;
                }
                try{
                  await retryOnConcurrentEditV1("snapshot",async()=>{snapshotAttempts++;return witness.input(source,expected,physical);},{modifiedAgeMs:()=>undefined,sleep:async()=>{},onRetry:()=>{}});
                  throw Error("Unproven compiler bytes accepted");
                }catch(error){
                  expect(concurrentEditOfV1(error,profiles.snapshot.rank),row.id).toBeUndefined();
                  expect(concurrentEditOfV1(error,profiles.build.rank),row.id).toMatchObject({path:source,reason:"bytes"});
                  throw error;
                }
              },{modifiedAgeMs:()=>undefined,sleep:async()=>{},onRetry:()=>{}});
              expect({snapshotAttempts,buildAttempts},row.id).toEqual({snapshotAttempts:1,buildAttempts:2});
            }
          }
          const missing=join(workspace,"missing.rs");
          witness.expect(missing);
          expect(await witness.captureClaim({path:missing,blake3:Buffer.from(blake3(Buffer.from("missing"))).toString("hex"),length:7},physical,control)).toBeUndefined();
        }finally{await witness.close();}
      },invocation);
    });
  }finally{await rm(workspace,{recursive:true,force:true});}
},120000);

test("warm retry requires stable source evidence before publication",async()=>{
  const workspace=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), "sw-"));
  const physical={maxBytes:65536,maxWork:100,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}};
  try{
    await withBuildTestInvocationV1(invocation=>{
      const compiler=scriptCompilerAdmissionControlV1(invocation,"test:witness-warm-retry",120000);
      return withLawStorageV1(compiler,lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
        const build=join(held.root,"build");
        await mkdir(build);
        const witness=new CargoSourceWitnessV1(workspace,join(held.root,"witness"),held.storage,control);
        try{
          for(const [index,row] of retryCases.entries()){
            const source=join(workspace,index+".rs"),file=join(build,"libunit"+index+".rmeta"),dep=join(build,"unit"+index+".d");
            const digest=(body:string)=>Buffer.from(blake3(Buffer.from(body))).toString("hex");
            const checksum={path:source,blake3:digest(row.consumed),length:Buffer.byteLength(row.consumed)};
            const events:string[]=[];
            const graph={units:[source]};
            let turn=0;
            witness.expect(source);
            const output=await runCargoPassAttemptsV1(async suffix=>{
              events.push("compile"+suffix);
              const live=row.live[turn++];
              expect(live,row.id).toBeDefined();
              await writeFile(source,live!);
              await writeFile(dep,`${file}: ${source}\n# checksum:blake3=${checksum.blake3} file_len:${checksum.length} ${source}\n`);
              for await(const claim of announcedCargoSourceChecksumsV1([file],[build],physical,control)){
                return {claim,drift:await witness.captureClaim(claim,physical,control)};
              }
              throw Error("Expected fixture compiler checksum");
            },async outcome=>outcome.drift,async (drift,suffix)=>{
              expect(drift).toEqual({path:source,expected:checksum.blake3+":"+checksum.length,observed:digest(row.live[0]!)+":"+Buffer.byteLength(row.live[0]!)});
              await admitWarmCargoRetryGraphV1({
                manifest:join(workspace,"Cargo.toml"),previous:graph,suffix,
                observe:async suffix=>{events.push("graph"+suffix);return graph;},
                admit:async()=>{events.push("admit");await held.storage.recheck();},
                replacementFits:()=>false,
              });
            });
            if(!output.drift){
              expect((await witness.input(source,output.claim,physical)).retained).toBe(true);
              events.push("stage");
            }
            expect(events,row.id).toEqual(row.events);
            expect(witness.canTransfer(source,checksum),row.id).toBe(row.staged);
          }
        }finally{await witness.close();}
      },invocation);
    });
  }finally{await rm(workspace,{recursive:true,force:true});}
},120000);
