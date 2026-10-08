import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, rmSync, renameSync, symlinkSync } from "node:fs";
import { join } from "node:path";
import { trustedCatalogByteClaimV1, trustedCatalogPhysicalClaimV1, trustedCatalogProvenancePathV1, writeTrustedCatalogPublicationProvenanceV1, readTrustedCatalogPublicationProvenanceV1 } from "../🟦️.ts";

export function createTrustedCatalogProvenanceTests() {
  async function testTrustedCatalogProvenanceV1(repoRoot: string) {
    const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
    const evidence=process.env.SEMIO_TEST_ARTIFACT_DIR!; assert(evidence);
    const root=mkdtempSync(join(evidence,"trusted-catalog-producer-neutral-"));
    const catalog=join(root,"trusted-catalog"), generation=join(catalog,"generations",fixture.generationId); mkdirSync(generation,{recursive:true});
    const names=["component","descriptor","actor","cargo","compiler"];
    for(const name of names)writeFileSync(join(generation,name),fixture.files[name]);
    const claim=(name:string)=>trustedCatalogByteClaimV1(generation,join(generation,name));
    const bundle={profiles:[{id:fixture.profileId,generationId:fixture.generationId}],packages:[]};
    writeFileSync(join(generation,"trusted-catalog.json"),JSON.stringify(bundle));
    const observation=join(catalog,"provenance","generations",fixture.generationId,"neutral");mkdirSync(observation,{recursive:true});for(const name of ["cargo","actor"])writeFileSync(join(observation,name),fixture.files[name]);
    const producer={schema:"semio.hub.trusted-catalog-producer/v1",profileId:fixture.profileId,generationId:fixture.generationId,bundle:claim("trusted-catalog.json"),packages:[{pluginId:"neutral",packageId:"semio:neutral",cargoPackage:"neutral",component:claim("component"),descriptor:claim("descriptor"),cargoInvocations:[trustedCatalogPhysicalClaimV1(join(observation,"cargo")),trustedCatalogPhysicalClaimV1(join(observation,"cargo"))],browserActor:trustedCatalogPhysicalClaimV1(join(observation,"actor"))}]};
    const producerPath=join(observation,"producer.json");writeFileSync(producerPath,JSON.stringify(producer));
    const pointer={profileId:fixture.profileId,generationId:fixture.generationId,bundleSha256:producer.bundle.sha256,publicationRevision:fixture.revision};
    const pointerPath=join(catalog,"current.json");writeFileSync(pointerPath,JSON.stringify(pointer));
    assert.throws(()=>readTrustedCatalogPublicationProvenanceV1(root));
    const published=writeTrustedCatalogPublicationProvenanceV1(root,producerPath);
    assert.equal(published, trustedCatalogProvenancePathV1(root,createHash("sha256").update(readFileSync(pointerPath)).digest("hex")));
    const read=readTrustedCatalogPublicationProvenanceV1(root); assert.deepEqual(read.generation,producer);
    const {default:Ajv}=await import("ajv");const ajv=new Ajv({strict:false}); const schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
    ajv.addSchema(schema);for(const [name,value]of [["GenerationProducerV1",producer],["PublicationProducerV1",read.record]] as const)assert(ajv.getSchema(schema.$id+"#/$defs/"+name)!(value));
    const hash=Buffer.from(await crypto.subtle.digest("SHA-256",readFileSync(producerPath))).toString("hex");assert.equal(hash,read.record.producer.sha256);
    for(const [path,text]of [[pointerPath,JSON.stringify({...pointer,publicationRevision:"2"})],[join(generation,"trusted-catalog.json"),"{}"],[producerPath,"{}"],[join(generation,"component"),"changed"]]){const before=readFileSync(path);writeFileSync(path,text);assert.throws(()=>readTrustedCatalogPublicationProvenanceV1(root));writeFileSync(path,before);}
    assert.throws(()=>writeTrustedCatalogPublicationProvenanceV1(root,join(repoRoot,"package.json")));
    const escaped=join(root,"escaped-observation");renameSync(observation,escaped);symlinkSync(escaped,observation,process.platform==="win32"?"junction":"dir");assert.throws(()=>readTrustedCatalogPublicationProvenanceV1(root));assert.throws(()=>writeTrustedCatalogPublicationProvenanceV1(root,producerPath));rmSync(observation);renameSync(escaped,observation);
    const durable=readFileSync(published);rmSync(join(root,"diagnostics"),{recursive:true,force:true});assert.deepEqual(readFileSync(published),durable);readTrustedCatalogPublicationProvenanceV1(root);
    console.log("trusted-catalog-producer-neutral: 6 refusal laws; Ajv=2 WebCrypto=1; evidence="+root);
  }
  return {testTrustedCatalogProvenanceV1};
}
