import {expect,test} from "bun:test";
import {blake3} from "@noble/hashes/blake3.js";
import {createHash} from "node:crypto";
import {mkdir,mkdtemp,rm,utimes,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {isConcurrentEditV1} from "../../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../🏗️build/🎟️capacity/🟦️.ts";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {withLawStorageV1,lawStoragePolicyV1} from "../../../🧪️tests/🦀️cargo/🟦️.ts";
import {runWasmCargoCommandV1} from "../../../../🕸️wasm-build/🟦️.ts";
import {RetainedCargoSourcesV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"cargo-retained-source",maximumElapsedMilliseconds:120_000,capabilities:{}});

const sha256=(text:string)=>createHash("sha256").update(text).digest("hex");
const dependency=(artifact:string,source:string,text:string)=>`${artifact}: ${source}\n# checksum:blake3=${Buffer.from(blake3(Buffer.from(text))).toString("hex")} file_len:${Buffer.byteLength(text)} ${source}\n`;
const bodies:Record<string,string>={admitted:fixture.admitted,edited:fixture.edited};

test("retained admitted sources prove the consumed bytes against independent checksums",async()=>{
  const workspace=await mkdtemp(join(tmpdir(),"semio-retained-"));
  try{
    await withBuildTestInvocationV1(invocation=>withLawStorageV1(scriptCompilerAdmissionControlV1(invocation,"test:retained-sources",120000),lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
      for(const [index,row] of fixture.cases.entries()){
        const source=join(workspace,index+".rs"),target=join(held.root,"target"+index),old=new Date(Date.now()-3_600_000);
        await mkdir(target);
        await writeFile(source,fixture.admitted);
        const retained=await RetainedCargoSourcesV1.admit({startedMs:Date.now(),inputs:[{path:source,digest:sha256(fixture.admitted)}]},{workspace,directory:join(held.root,"retained"+index),storage:held.storage,control,maxSourceBytes:65536});
        try{
          if(row.edit){
            await writeFile(source,fixture.edited);
            expect(retained.defer({path:source,expected:sha256(fixture.admitted),observed:sha256(fixture.edited)},control),row.id).toBe(true);
          }
          expect(retained.defer({path:source,expected:sha256("unretained"),observed:sha256(fixture.edited)},control),row.id).toBe(false);
          for(const [claim,spec] of row.claims.entries()){
            const file=join(target,"unit"+claim+".d");
            await writeFile(file,dependency(join(target,"libunit"+claim+".rlib"),source,bodies[spec.body]!));
            if(!spec.recent)await utimes(file,old,old);
          }
          const outcome=await retained.prove([target],Date.now()-60_000,control).then(provenance=>provenance,error=>error);
          if(row.outcome==="refused"){
            expect(isConcurrentEditV1(outcome),row.id).toBe(true);
            continue;
          }
          expect(outcome.proven,row.id).toBe(row.proven);
          expect(outcome.files).toBe(1);
          expect(outcome.sha256).toBe(sha256(source+"\0"+sha256(fixture.admitted)+"\n"));
          expect(outcome.deferred.map((entry:{observed:string})=>entry.observed),row.id).toEqual(row.edit?[sha256(fixture.edited)]:[]);
        }finally{await retained.close();}
      }
    },invocation));
  }finally{await rm(workspace,{recursive:true,force:true});}
},120000);

test("an edit mid-compile defers instead of cancelling and the artifact proves the admitted digest",async()=>{
  const workspace=await mkdtemp(join(tmpdir(),"semio-retained-"));
  try{
    await withBuildTestInvocationV1(invocation=>withLawStorageV1(scriptCompilerAdmissionControlV1(invocation,"test:retained-churn",120000),lawStoragePolicyV1.stubImageBytes,async(held,control)=>{
      {
        const source=join(workspace,"churn.rs"),target=join(held.root,"churn");
        await mkdir(target);
        await writeFile(source,fixture.admitted);
        const roster={startedMs:Date.now(),inputs:[{path:source,digest:sha256(fixture.admitted)}]};
        const retained=await RetainedCargoSourcesV1.admit(roster,{workspace,directory:join(held.root,"churn-retained"),storage:held.storage,control,maxSourceBytes:65536});
        let deferred!:()=>void;
        const seen=new Promise<void>(accept=>{deferred=accept;}),defer=retained.defer.bind(retained);
        retained.defer=(mismatch,reporter)=>{const held=defer(mismatch,reporter);if(held)deferred();return held;};
        const run=async()=>{
          await writeFile(join(target,"unit.d"),dependency(join(target,"libunit.rlib"),source,fixture.admitted));
          await writeFile(source,fixture.edited);
          await Promise.race([seen,Bun.sleep(3000)]);
        };
        try{
          await runWasmCargoCommandV1(run,"compiler",[],{cwd:workspace,environment:{},label:"fixture-WASM",signal:control.signal,control},roster,retained);
          const provenance=await retained.prove([target],Date.now()-60_000,control);
          expect(provenance.sha256).toBe(sha256(source+"\0"+sha256(fixture.admitted)+"\n"));
          expect(provenance.deferred).toEqual([{path:source,admitted:sha256(fixture.admitted),observed:sha256(fixture.edited)}]);
          expect(provenance.proven).toBe(1);
        }finally{await retained.close();}
      }
    },invocation));
  }finally{await rm(workspace,{recursive:true,force:true});}
},120000);
