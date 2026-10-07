import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { projectNativeCodecReceiptPublicationV1 } from "../../♻️native-receipt/🟦️.ts";
const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/♻️native-receipt/🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/♻️native-receipt/🔣️.json",import.meta.url),"utf8"));
describe("native codec receipt publication",()=>{
 it("validates the neutral receipt against Ajv and independent Node SHA256",()=>{
  expect(new Ajv({strict:false}).validate(schema,fixture.receipt)).toBe(true);
  expect(createHash("sha256").update(fixture.protocol).digest("hex")).toBe(fixture.receipt.protocolSourceSha256);
 });
 for(const row of fixture.cases)it(row.id,()=>{
  const receipt={...fixture.receipt,...row.patch};
  const native={factory_id:fixture.receipt.factoryId,artifact_kind:fixture.receipt.artifactKind,artifact_schema:fixture.receipt.artifactSchema,extension:fixture.receipt.extension,pack_schema_hash:"3".repeat(64),protocol_source_sha256:"4".repeat(64)};
  const peer={factory_id:"peer",pack_schema_hash:"5".repeat(64)};
  const input={receipt,protocol:fixture.protocol,factories:{receipts:[native,peer]},definition:{id:fixture.receipt.artifactKind,codecs:[{native_factory:{...native}}]},catalog:{nativeCodecs:[{...fixture.receipt,packSchemaHash:native.pack_schema_hash,protocolSourceSha256:native.protocol_source_sha256}],openTargets:[]}};
  const before=JSON.stringify(input);
  if(!row.accepted){expect(()=>projectNativeCodecReceiptPublicationV1(input)).toThrow();expect(JSON.stringify(input)).toBe(before);return;}
  const projected=projectNativeCodecReceiptPublicationV1(input) as any;
  expect(projected.factories.receipts[0].pack_schema_hash).toBe(receipt.packSchemaHash);
  expect(projected.factories.receipts[0].protocol_source_sha256).toBe(receipt.protocolSourceSha256);
  expect(projected.factories.receipts[1]).toEqual(peer);
  expect(projected.definition.codecs[0].native_factory.pack_schema_hash).toBe(receipt.packSchemaHash);
  expect(projected.catalog.nativeCodecs[0].packSchemaHash).toBe(receipt.packSchemaHash);
  expect(projected.catalog.nativeCodecs[0].protocolSourceSha256).toBe(receipt.protocolSourceSha256);
  expect(JSON.stringify(input)).toBe(before);
 });
});
