import { retentionReferenceSchema as cargoReferenceSchema } from "@semio-tech/process/artifacts/native-build/receipt/retention/reference/schema";
import {readCompletedCargoPublicationReferenceV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/reference";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync, rmSync, renameSync, symlinkSync } from "node:fs";
import { join } from "node:path";
import { trustedCatalogByteClaimV1, trustedCatalogProvenancePathV1, writeTrustedCatalogPublicationProvenanceV1, readTrustedCatalogPublicationProvenanceV1 } from "../🟦️.ts";
import {observePhysicalFileV1,type FileObservationControlV1} from "@semio-tech/framework/filesystem/observation";

export function createTrustedCatalogProvenanceTests() {
  async function testTrustedCatalogProvenanceV1(repoRoot: string) {
    const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8"));
    const started=Date.now(),control:FileObservationControlV1={maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>false,remainingMs:()=>60_000-(Date.now()-started),onProgress:()=>{}};
    const evidence=process.env.SEMIO_TEST_ARTIFACT_DIR!; assert(evidence);
    const root=mkdtempSync(join(evidence,"p-"));
    const catalog=join(root,"trusted-catalog"), generation=join(catalog,"generations",fixture.generationId); mkdirSync(generation,{recursive:true});
    const names=["component","descriptor","actor","cargo","compiler"];
    for(const name of names)writeFileSync(join(generation,name),fixture.files[name]);
    const claim=(name:string)=>trustedCatalogByteClaimV1(generation,join(generation,name),control);
    const bundle={profiles:[{id:fixture.profileId,generationId:fixture.generationId}],packages:[]};
    writeFileSync(join(generation,"trusted-catalog.json"),JSON.stringify(bundle));
    const observation=join(catalog,"provenance","generations",fixture.generationId,"n");mkdirSync(observation,{recursive:true});for(const name of ["cargo","actor"])writeFileSync(join(observation,name),fixture.files[name]);
    const producer={schema:"semio.hub.trusted-catalog-producer/v1",profileId:fixture.profileId,generationId:fixture.generationId,bundle:await claim("trusted-catalog.json"),packages:[{pluginId:"neutral",packageId:"semio:neutral",cargoPackage:"neutral",component:await claim("component"),descriptor:await claim("descriptor"),cargoInvocations:[readCompletedCargoPublicationReferenceV1({version:1,mode:"release",kind:"component",pass:"component-release",receiptSha256:"1".repeat(64),indexSha256:"2".repeat(64),originalArtifact:{path:join(generation,"component"),sha256:(await claim("component")).sha256,byteLength:fixture.files.component.length}}),readCompletedCargoPublicationReferenceV1({version:1,mode:"release",kind:"emitter",pass:"descriptor-emitter",receiptSha256:"3".repeat(64),indexSha256:"4".repeat(64),originalArtifact:{path:join(generation,"compiler"),sha256:(await claim("compiler")).sha256,byteLength:fixture.files.compiler.length}})],browserActor:await observePhysicalFileV1(join(observation,"actor"),control)}]};
    const producerPath=join(observation,"p.json");writeFileSync(producerPath,JSON.stringify(producer));
    const pointer={profileId:fixture.profileId,generationId:fixture.generationId,bundleSha256:producer.bundle.sha256,publicationRevision:fixture.revision};
    const pointerPath=join(catalog,"current.json");writeFileSync(pointerPath,JSON.stringify(pointer));
    await assert.rejects(()=>readTrustedCatalogPublicationProvenanceV1(root,control));
    const published=await writeTrustedCatalogPublicationProvenanceV1(root,producerPath,control);
    assert.equal(published, trustedCatalogProvenancePathV1(root,createHash("sha256").update(readFileSync(pointerPath)).digest("hex")));
    const read=await readTrustedCatalogPublicationProvenanceV1(root,control); assert.deepEqual(read.generation,producer);
    const {default:Ajv}=await import("ajv");const ajv=new Ajv({strict:false}); const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
    ajv.addSchema(cargoReferenceSchema);ajv.addSchema(schema);for(const [name,value]of [["GenerationProducerV1",producer],["PublicationProducerV1",read.record]] as const){const validate=ajv.getSchema(schema.$id+"#/$defs/"+name);assert(validate);assert(validate(value));}
    const hash=Buffer.from(await crypto.subtle.digest("SHA-256",readFileSync(producerPath))).toString("hex");assert.equal(hash,read.record.producer.sha256);
    for(const [path,text]of [[pointerPath,JSON.stringify({...pointer,publicationRevision:"2"})],[join(generation,"trusted-catalog.json"),"{}"],[producerPath,"{}"],[join(generation,"component"),"changed"]] as const){const before=readFileSync(path);writeFileSync(path,text);await assert.rejects(()=>readTrustedCatalogPublicationProvenanceV1(root,control));writeFileSync(path,before);}
    await assert.rejects(()=>writeTrustedCatalogPublicationProvenanceV1(root,join(repoRoot,"package.json"),control));
    const escaped=join(root,"escaped-observation");renameSync(observation,escaped);symlinkSync(escaped,observation,process.platform==="win32"?"junction":"dir");await assert.rejects(()=>readTrustedCatalogPublicationProvenanceV1(root,control));await assert.rejects(()=>writeTrustedCatalogPublicationProvenanceV1(root,producerPath,control));rmSync(observation);renameSync(escaped,observation);
    const durable=readFileSync(published);rmSync(join(root,"diagnostics"),{recursive:true,force:true});assert.deepEqual(readFileSync(published),durable);await readTrustedCatalogPublicationProvenanceV1(root,control);
    assert.throws(()=>readCompletedCargoPublicationReferenceV1({...producer.packages[0]!.cargoInvocations[0],origin:{workspace:"/foreign"}}));
    rmSync(root,{recursive:true,force:true});
  }
  return {testTrustedCatalogProvenanceV1};
}
