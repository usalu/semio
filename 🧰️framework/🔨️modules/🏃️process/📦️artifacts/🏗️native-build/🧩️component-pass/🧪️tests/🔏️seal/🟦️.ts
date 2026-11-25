import { sha256 } from "@noble/hashes/sha2.js";
import { blake3 } from "@noble/hashes/blake3.js";
import { scriptCompilerAdmissionControlV1 } from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import { registerIsolatedBuildTestV1, withBuildTestInvocationV1 } from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import { expect } from "bun:test";
import { createHash } from "node:crypto";
import { decodeCargoProvenanceV1 } from "../../../🧾️receipt/🟦️.ts";
import { CargoSourceWitnessV1 } from "../../../🧾️receipt/📎️source-witness/🟦️.ts";
import { chmodSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { lawStoragePolicyV1, withLawStorageV1 } from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import { NATIVE_BUILD_DRIVER_POLICY_V1 } from "../../🟦️.ts";
import { installRestartableFailureHandlerV1 } from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import { artifacts, stubCargo, stubWorkspace, runStub } from "../🎭️stub/🟦️.ts";

const compilerSealMutationError = /Current physical bytes changed before transfer/u;

for (const scenario of fixture.provenanceSeal.beforeAnnouncement.cases) {
  const name = `the driver seals compiler provenance across a pre-announcement source edit (${scenario.retained ? "retained" : "unbacked"})`;
  const refusal = /retry stopped by the fixture after the unbacked source edit/u;
  registerIsolatedBuildTestV1(import.meta.path,name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    const root=mkdtempSync(join(artifacts,"z-")),cargo=stubCargo(root),pass=stubWorkspace(root,"app",["seal-a"],"release");
    const source=join(dirname(pass.packages[0]!.manifest),"lib.rs"),expected=fixture.provenanceSeal;
    writeFileSync(source,expected.originalSource);
    const log:string[]=[];
    const run=runStub(root,{version:1,passes:[pass]},cargo,{
      log:line=>log.push(line),
      ...(scenario.retained?{}:{retainCompilationBytes:1_048_576,maximumObservationBytes:1_048_576,consume:async()=>{throw Error("unbacked fixture must not publish");}}),
    },scenario.retained?"observe-before-announcement":"observe-before-announcement-unbacked",({outcome})=>{
      if(!outcome)return;
      const receipts=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const files=readdirSync(receipts).filter(name=>name.startsWith("cargo-unit-provenance-"));
      expect(files).toHaveLength(1);
      const units=decodeCargoProvenanceV1(readFileSync(join(receipts,files[0]!),"utf8")).units;
      expect(units.flatMap(unit=>unit.inputs).find(row=>row.path===source)?.sha256)
        .toBe(createHash("sha256").update(expected.originalSource).digest("hex"));
      expect(units.flatMap(unit=>unit.depInfo).flatMap(row=>row.checksums).find(row=>resolve(row.path)===source))
        .toMatchObject({blake3:Buffer.from(blake3(Buffer.from(expected.originalSource))).toString("hex"),length:Buffer.byteLength(expected.originalSource)});
    });
    if(scenario.retained)expect((await run).staged).toEqual(["seal-a:release"]);
    else await expect(run).rejects.toThrow(refusal);
    const calls=readFileSync(cargo.calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
    expect(calls.filter(row=>row.pid)).toHaveLength(scenario.attempts);
    expect(calls.filter(row=>row.end)).toHaveLength(scenario.retained?1:0);
    expect(log.filter(line=>line.includes("recompiling in the held lane"))).toHaveLength(scenario.attempts-1);
    expect(readFileSync(source,"utf8")).toBe(expected.editedSource);
    checkpoint();
  },scenario.retained?undefined:refusal);
}

for (const scenario of fixture.provenanceSeal.failedSpan.cases) {
  registerIsolatedBuildTestV1(import.meta.path,"the driver recaptures an authenticated failed span ("+scenario.name+")",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    const root=mkdtempSync(join(artifacts,"e-")),cargo=stubCargo(root),pass=stubWorkspace(root,"app",["seal-a"],"release");
    const source=join(dirname(pass.packages[0]!.manifest),"lib.rs"),expected=fixture.provenanceSeal;
    writeFileSync(source,expected.originalSource);
    const run=runStub(root,{version:1,passes:[pass]},cargo,{},"observe-failed-span:"+scenario.name,({outcome})=>{
      if(!outcome)return;
      const receipts=join(outcome.generation.report,NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const record=readdirSync(receipts).find(name=>name.startsWith("cargo-unit-provenance-"));
      expect(record).toBeDefined();
      const units=decodeCargoProvenanceV1(readFileSync(join(receipts,record!),"utf8")).units;
      expect(units.flatMap(unit=>unit.depInfo).flatMap(row=>row.checksums).find(row=>resolve(row.path)===source))
        .toMatchObject({blake3:Buffer.from(blake3(Buffer.from(expected.editedSource))).toString("hex"),length:Buffer.byteLength(expected.editedSource)});
    });
    if(scenario.changed)expect((await run).staged).toEqual(["seal-a:release"]);
    else await expect(run).rejects.toThrow(/^Native build failed:/u);
    const calls=readFileSync(cargo.calls,"utf8").trim().split("\n").map(line=>JSON.parse(line));
    expect(calls.filter(row=>row.pid)).toHaveLength(scenario.attempts);
    expect(calls.some(row=>row.keepGoing)).toBe(!scenario.retired);
    checkpoint();
  },scenario.changed?undefined:/^Native build failed:/u);
}

for (const scenario of fixture.provenanceSeal.cases) {
  const packages = scenario.packages ?? ["seal-a"];
  const name = "the driver seals compiler provenance while independent stages run (" + scenario.mutation + " " + scenario.timing + (packages.length > 1 ? " pair" : "") + ")";
  registerIsolatedBuildTestV1(import.meta.path,name,{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
    if (!scenario.publishesReceipt) installRestartableFailureHandlerV1(() => undefined);
    const expected=fixture.provenanceSeal, announced=scenario.timing==="after-announcement", withChecksum=announced||scenario.timing==="after-compiler-checksum";
    const root=mkdtempSync(join(artifacts,"z-")), cargo=stubCargo(root), pass=stubWorkspace(root,"app",packages,"release");
    const sources=pass.packages.map(row=>join(dirname(row.manifest),"lib.rs"));
    for(const source of sources)writeFileSync(source,announced?expected.admittedSource:expected.originalSource);
    let signalSeal!:()=>void, sealed=false, edited=false;
    const seal=new Promise<void>(resolve=>{signalSeal=resolve;});
    const run=runStub(root,{version:1,passes:[pass]},cargo,{
      verifyStageWriters:true,
      log(line) {
        if(line==="[native-build] compiler provenance sealed"){sealed=true;signalSeal();}
      },
      stages(linked) {
        return [{name:"independent",after:[],capacity:"io",async run(context){
          const provenance=await context.provenance();
          expect(provenance.units).toBe(1);
          if(scenario.timing==="after-seal") {
            let timer:ReturnType<typeof setTimeout>|undefined;
            try{await Promise.race([seal,new Promise<void>(resolve=>{timer=setTimeout(resolve,5000);})]);}
            finally{if(timer!==undefined)clearTimeout(timer);}
          }
          writeFileSync(scenario.mutation==="source"?join(dirname(linked.package.manifest),"lib.rs"):linked.staged,expected.editedSource);
          edited=true;
        }}];
      },
    },announced?"observe-announced":withChecksum?"observe-consumed":"observe-linked",({outcome,held})=>{
      const runs=join(held.root,"cache",NATIVE_BUILD_DRIVER_POLICY_V1.warmRoot);
      const receipts=join(outcome?.generation.report??join(runs,readdirSync(runs)[0]!,NATIVE_BUILD_DRIVER_POLICY_V1.stores.report),NATIVE_BUILD_DRIVER_POLICY_V1.provenanceDirectory);
      const files=readdirSync(receipts).filter(name=>name.startsWith("cargo-unit-provenance-"));
      expect(files.length>0).toBe(scenario.publishesReceipt);
      if(!files.length)return;
      const units=files.flatMap(name=>decodeCargoProvenanceV1(readFileSync(join(receipts,name),"utf8")).units);
      for(const source of sources){
        const original=units.flatMap(unit=>unit.inputs).find(row=>row.path===source);
        expect(original?.sha256).toBe(createHash("sha256").update(expected.originalSource).digest("hex"));
        if(withChecksum){
          const compiler=units.flatMap(unit=>unit.depInfo).flatMap(dep=>dep.checksums).find(row=>resolve(row.path)===source);
          expect(compiler?.blake3).toBe(Buffer.from(blake3(Buffer.from(expected.originalSource))).toString("hex"));
          expect(compiler?.length).toBe(Buffer.byteLength(expected.originalSource));
          expect(readFileSync(source,"utf8")).toBe(expected.editedSource);
        }
      }
    });
    if (scenario.publishesReceipt) expect([...(await run).staged].sort()).toEqual(packages.map(name=>name+":release").sort());
    else await expect(run).rejects.toThrow(compilerSealMutationError);
    expect(edited).toBe(true);
    expect(sealed).toBe(scenario.publishesReceipt||scenario.timing==="after-seal");
    checkpoint();
  },scenario.publishesReceipt?undefined:compilerSealMutationError);
}

registerIsolatedBuildTestV1(import.meta.path,"compiler source witnesses refuse mixed consumed versions in either order and reject altered blobs",{bodyMilliseconds:120_000,scope:{kind:"body"}},async checkpoint=>{
  const vector=fixture.provenanceSeal,root=mkdtempSync(join(artifacts,"m-")),source=join(root,"source.rs");
  const original=Buffer.from(vector.originalSource),revised=Buffer.from(vector.editedSource);
  const physical=()=>({maxBytes:65536,maxWork:100,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>10000,onProgress:()=>{}});
  const claim=(bytes:Buffer)=>({path:source,blake3:Buffer.from(blake3(bytes)).toString("hex"),length:bytes.byteLength});
  await withBuildTestInvocationV1(originalInvocation=>{
    const control=scriptCompilerAdmissionControlV1(originalInvocation,"test:source-witness",120000);
    return withLawStorageV1(control,lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
      for(const order of vector.mixedOrders){
        const directory=join(held.root,"c"),witness=new CargoSourceWitnessV1(root,directory,held.storage,control);
        writeFileSync(source,original);
        await witness.remember(source,original,control);
        writeFileSync(source,revised);
        const missing=Buffer.concat([original,revised]);
        await expect(witness.input(source,claim(missing),physical())).rejects.toThrow("Compiler consumed bytes have no matching retained source witness");
        const first=order[0]==="retained"?original:revised,second=first===original?revised:original;
        const selected=await witness.input(source,claim(first),physical());
        expect(selected.retained).toBe(first===original);
        expect(selected.row.sha256).toBe(createHash("sha256").update(first).digest("hex"));
        await expect(witness.input(source,claim(second),physical())).rejects.toThrow("Compiler consumed inconsistent source versions");
        if(selected.retained){
          const blob=join(directory,selected.row.sha256!);
          chmodSync(blob,0o600);
          writeFileSync(blob,revised);
          await expect(witness.recheck(physical)).rejects.toThrow("Retained compiler source witness changed before transfer");
        }
        await witness.close();
      }
    },originalInvocation);
  });
  checkpoint();
});
