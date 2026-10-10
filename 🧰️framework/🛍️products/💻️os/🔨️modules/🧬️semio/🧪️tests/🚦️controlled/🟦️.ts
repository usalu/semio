
import {expect,test} from "bun:test";
import {NativeDecodeControl} from "../../../../../../🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {unwrapBinaryControlled,admitTextPreambleControlled} from "../../🟦️.ts";
import envelopeFixture from "../../🧫️fixtures/🚦️controlled/🔣️.json";

test("neutral retained text ownership keeps payload work separate from complete backing release",()=>{
  const f=envelopeFixture;
  expect(f.retained.ownership).toEqual({copy:"bounded-original-payload",capacity:"exact-natural-birth",release:"exact-physical-allocation",depth:"exact-original-frontier",refusal:"original-owner-kept"});
  const body=f.textBody.repeat(f.retained.textRepeats),wire=`semio ${f.id}.dsl v${f.version}\n${body}`;
  const bytes=new TextEncoder().encode(wire);
  expect(Buffer.from(wire).equals(Buffer.from(bytes))).toBe(true);
  expect(Buffer.byteLength(body)).toBeGreaterThan(f.retained.retirementBytes);
  expect(f.retained.maximumRetirementDepth).toBeGreaterThan(1);
  expect(f.retained.sourceReleases).toBe(1);
  expect(f.retained.nativeMaximum).toBe(1_000_000);
  expect(f.retained.callerGrant).toEqual({maximumItems:65536,maximumCopyBytes:65536,maximumCapacityBytes:16777216,maximumReleaseBytes:16777216,maximumDepth:64});
  expect(bytes.length-Buffer.byteLength(`semio ${f.id}.dsl v${f.version}\n`)).toBe(f.retained.payloadBytes);
  expect(Math.ceil(f.retained.payloadBytes/f.retained.retirementBytes)).toBe(f.retained.minimumPayloadRetirementTurns);
  expect(f.retained.undergrantItems).toBeLessThan(f.retained.minimumPayloadRetirementTurns);
  for(const header of f.retained.structuralCopy){expect(header.optionalTextHeaderBytes).toBe(3*header.nativeWordBytes);expect(header.optionalTextHeaderBytes).toBeGreaterThan(f.retained.retirementBytes);expect(header.optionalTextHeaderBytes).toBeLessThan(f.retained.callerGrant.maximumCopyBytes);}
});

test("native envelopes admit exact identity without owning their payload",async()=>{const f=envelopeFixture;
  const token=Buffer.from(`${f.id}.pack v${f.version}`),header=Buffer.alloc(12);Buffer.from([0x89,0x53,0x45,0x4d,13,10,26,10]).copy(header);header.writeUInt32LE(token.length,8);const bytes=Buffer.concat([header,token,Buffer.alloc(f.bodyBytes,127)]);
  const control=new NativeDecodeControl(0,()=>true),body=await unwrapBinaryControlled(bytes,f.id,"pack",f.version,control);expect(body.buffer).toBe(bytes.buffer);expect(body.byteOffset).toBe(bytes.byteOffset+12+bytes.readUInt32LE(8));expect(body.length).toBe(f.bodyBytes);expect(control.ownedBytes).toBe(0);
  for(const [id,component,version] of [["foreign.owner","pack",f.version],[f.id,"spr",f.version],[f.id,"pack",f.version+1]] as const)await expect(unwrapBinaryControlled(bytes,id,component,version,control)).rejects.toThrow("identity");
  const malformed=Buffer.from(bytes);malformed.writeUInt32LE(0xffffffff,8);await expect(unwrapBinaryControlled(malformed,f.id,"pack",f.version,control)).rejects.toThrow("truncated");
  for(const newline of ["\n","\r\n"]){const text=`semio ${f.id}.dsl v${f.version}${newline}${f.textBody}`,offset=await admitTextPreambleControlled(text,f.id,"dsl",f.version,control);expect(text.slice(offset)).toBe(f.textBody);}
  await expect(admitTextPreambleControlled(`semio ${f.id}.dsl v${f.version}tail`,f.id,"dsl",f.version,control)).rejects.toThrow("line boundary");
  const canceled=new NativeDecodeControl(0,()=>false);await expect(unwrapBinaryControlled(bytes,f.id,"pack",f.version,canceled)).rejects.toThrow("canceled");expect(canceled.ownedBytes).toBe(0);
});

import Ajv2020 from "ajv/dist/2020";
import refusalSchema from "../../../../../../🔨️modules/🌱️value/⚠️refusal/🔁️codec/🧬️schema/🔣️.json";

test("original controlled binary envelope refuses through canonical literal ValueError without allocation",async()=>{
 const validate=new Ajv2020({strict:true}).compile(refusalSchema);
 const token=Buffer.from(`${envelopeFixture.id}.pack v${envelopeFixture.version}`),header=Buffer.alloc(12);Buffer.from([0x89,0x53,0x45,0x4d,13,10,26,10]).copy(header);header.writeUInt32LE(token.length,8);
 const valid=Buffer.concat([header,token,Buffer.from(envelopeFixture.textBody)]);
 for(const row of envelopeFixture.binaryRefusals){
  expect(validate({kind:row.kind,message:row.message,retainedProgress:{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0}})).toBe(true);
  const bytes=Buffer.from(valid);let id=envelopeFixture.id;if(row.case==="prefix")bytes[0]=0;if(row.case==="truncated")bytes.writeUInt32LE(0xffffffff,8);if(row.case==="identity")id="foreign.owner";
  await expect(unwrapBinaryControlled(bytes,id,"pack",envelopeFixture.version,new NativeDecodeControl(0,()=>true))).rejects.toThrow(row.message.split(": ")[1]);
 }
 const source=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text();const binary=source.slice(source.indexOf("pub fn unwrap_binary_controlled"),source.indexOf("fn parse_binary_token"));
 expect(binary).toContain("Result<&'input[u8],ValueError>");expect(binary).not.toContain("SemioError");expect(binary).not.toContain(".into()");expect(binary).not.toContain("format!");for(const row of envelopeFixture.binaryRefusals)expect(binary).toContain(`ValueError::literal(ValueRefusalKind::InvalidValue,"${row.message}")`);
 console.error("[DEBUG] Original controlled envelope per-refusal canonical Ajv and independent Buffer bytes agree; zero-allocation native System law remains separately required");
});
