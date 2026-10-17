import {expect,test} from "bun:test";
import Ajv from "ajv";
import {applyPatch} from "fast-json-patch";
import fixture from "./🧫️fixtures/🔣️.json";
import requestSchema from "../📨️request/🧬️schema/🔣️.json";
import intakeSchema from "../🏠️intake/🧬️schema/🔣️.json";
import completeSchema from "../../../../🧩️extensions/🕸️wasm/🧬️schema/📐️tessellation/📤️complete/🔣️.json";
import completeFixture from "../📨️response/🧫️fixtures/🔣️.json";
import { meshEngineCanonicalSchema as meshSchema, engineCanonicalDecodeFixture as decoderFixture } from "@semio-tech/framework-mesh-engine-ts/testing/fixtures";
import { loadNeuralEngineProjectionSchema, loadNeuralEngineProjectionFixture } from "../../../../../🧠️neural/🧪️tests/🧫️fixtures/🟦️.ts";
import { loadSceneMathResourceSchema, loadSceneMathResourceFixture, loadPresentResourcesTransportSchema, loadPresentResourcesTransportFixture } from "../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";

test("original completed tessellation preserves every canonical mesh field and exact original business source",()=>{
 const validate=new Ajv({strict:true}).addSchema(meshSchema).compile(completeSchema);expect(validate(completeFixture)).toBe(true);expect(Object.keys(completeFixture.mesh)).toEqual(Object.keys(meshSchema.properties));for(const field of ["retained","budget","wallMicros","operation","generation","meshPack","chunk","chunks"]){expect(validate({...completeFixture,[field]:1})).toBe(false);}expect(validate({...completeFixture,done:false})).toBe(false);expect(validate({...completeFixture,cancellable:true})).toBe(false);expect(validate({...completeFixture,tolerance:0})).toBe(false);for(const field of completeSchema.required){const missing={...completeFixture};delete missing[field as keyof typeof missing];expect(validate(missing)).toBe(false);}const bytes=Buffer.from(JSON.stringify(completeFixture));for(const body of [1,3,64]){const pages=[];for(let offset=0;offset<bytes.length;offset+=body)pages.push(bytes.subarray(offset,offset+body));const independent=JSON.parse(Buffer.concat(pages).toString("utf8"));expect(independent.mesh).toEqual(completeFixture.mesh);expect(independent.mesh.componentReferences.face).toEqual(["face:7"]);expect(independent.mesh.attributes.customSource.values[0]).toBe("源");}const raw=bytes.toString("utf8").replace('"nodeHash":4294967297','"nodeHash":18446744073709551615');const token=raw.match(/"nodeHash":([0-9]+)/)![1]!;expect(BigInt(token)).toBe(18446744073709551615n);
});

test("original preview request forbids reusable authority and preserves independent UTF8 business source",()=>{
 const validate=new Ajv({strict:true}).compile(requestSchema),wire={handle:fixture.query.output.solid.handle,tolerance:fixture.tessellationWire.tolerance,nodeHash:fixture.tessellationWire.nodeHash,windowId:fixture.tessellationWire.windowId,windowKindId:fixture.tessellationWire.windowKindId};expect(validate(wire)).toBe(true);expect(Object.keys(wire)).toEqual(fixture.tessellationWire.keys);for(const field of ["retained","budget","wallMicros","deadlineUs","operation","generation"]){expect(validate({...wire,[field]:1})).toBe(false);}for(const field of fixture.tessellationWire.keys){const missing={...wire};delete missing[field as keyof typeof wire];expect(validate(missing)).toBe(false);}expect(validate({...wire,tolerance:0})).toBe(false);expect(validate({...wire,chunk:0})).toBe(false);for(const copy of fixture.tickIntake.cloneBodies){const original=Buffer.from(JSON.stringify(wire)),segments=[];for(let offset=0;offset<original.length;offset+=copy)segments.push(original.subarray(offset,offset+copy));expect(JSON.parse(Buffer.concat(segments).toString("utf8"))).toEqual(wire);}
});

test("original Flow tick intake keeps the complete source and rejects authority substitutions",()=>{
 const validate=new Ajv({strict:true}).compile(intakeSchema);expect(validate(fixture.tickIntake)).toBe(true);for(const field of ["originalSourceRequired","revisionRequired","originRequired","sourceHeldOnDenial","sourceHeldThroughCancelClose","receiptDebitedOnce"]){expect(validate({...fixture.tickIntake,[field]:false})).toBe(false);}expect(validate({...fixture.tickIntake,terminalDrop:1})).toBe(false);expect(validate({...fixture.tickIntake,browserGeometryAuthority:"native-port"})).toBe(false);const original={snapshot:fixture.sourceDescriptor,revision:fixture.revision,operation:fixture.previewSourceTransfer.operation,generation:fixture.previewSourceTransfer.generation};const independent=applyPatch<{original:typeof original;working:typeof original|null}>({original,working:null},[{op:"copy",from:"/original",path:"/working"}],true,false).newDocument;expect(independent.working).toEqual(original);for(const field of ["revision","operation","generation"]){const foreign=applyPatch(independent,[{op:"replace",path:`/working/${field}`,value:field==="revision"?fixture.foreignRevision:0}],true,false).newDocument;expect(foreign.working).not.toEqual(foreign.original);}expect(fixture.previewSourceTransfer.sameHandleMemory).toBe(true);expect(fixture.previewSourceTransfer.returnsWholeReadOnDenial).toBe(true);
});


test("canonical native decoder fixture agrees with strict Mesh16 source schema and independent JSON ownership",()=>{
 const validate=new Ajv({strict:true}).compile(meshSchema);expect(validate(decoderFixture.mesh)).toBe(true);expect(decoderFixture.mesh).toEqual(completeFixture.mesh);expect(Object.keys(decoderFixture.mesh)).toHaveLength(16);expect(decoderFixture.cancelCopyBytes).toEqual([1,3,64]);expect(decoderFixture.normalCopyBytes).toBe(64);for(const field of ["attributes","materials","textures"]){expect(validate({...decoderFixture.mesh,[field]:{"":decoderFixture.mesh[field as "attributes"]?.customSource}})).toBe(false);}const independent=JSON.parse(JSON.stringify(decoderFixture.mesh));expect(independent).toEqual(decoderFixture.mesh);expect(independent.attributes.customSource.values).toEqual(["源",true,17]);
});


test("original tick cancellation completes only after snapshot and initiating identity custody are empty",()=>{
 const validate=new Ajv({strict:true}).compile(intakeSchema);expect(validate(fixture.tickIntake)).toBe(true);expect(validate({...fixture.tickIntake,completeRequiresEmpty:false})).toBe(false);expect(validate({...fixture.tickIntake,closeStates:["snapshot","empty"]})).toBe(false);let original={snapshot:fixture.sourceDescriptor,closing:null,origin:{operation:fixture.previewSourceTransfer.operation,generation:fixture.previewSourceTransfer.generation}};const owners=[];original=applyPatch(original,[{op:"move",from:"/snapshot",path:"/closing"},{op:"add",path:"/snapshot",value:null}],true,false).newDocument;owners.push(original);original=applyPatch(original,[{op:"replace",path:"/closing",value:null}],true,false).newDocument;owners.push(original);original=applyPatch(original,[{op:"replace",path:"/origin",value:null}],true,false).newDocument;owners.push(original);expect(owners.map(owner=>owner.snapshot===null&&owner.closing===null&&owner.origin===null)).toEqual([false,false,true]);
});


test("original incomplete preview carries only current business progress and no mesh custody",async()=>{
 const schema=(await import("../../../../🧩️extensions/🕸️wasm/🧬️schema/📐️tessellation/📈️progress/🔣️.json")).default;
 const fixture=(await import("../../../📥️preview/🧫️fixtures/📈️progress.json")).default;
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(Object.keys(fixture)).toHaveLength(10);expect(fixture.unitsDone<=fixture.unitsTotal).toBe(true);for(const field of ["mesh","meshPack","chunk","chunks","budget","retained","operation","generation"]){expect(validate({...fixture,[field]:1})).toBe(false);}for(const phase of schema.properties.phase.enum){expect(validate({...fixture,phase})).toBe(true);}const independent=JSON.parse(JSON.stringify(fixture));expect(independent).toEqual(fixture);expect(independent.unitsDone/independent.unitsTotal).toBe(3/17);
});


test("original preview occupied handle replaces one current canonical mesh and retains its original displaced source",async()=>{
 const law=(await import("../../../📥️preview/🧫️fixtures/🔁️replacement.json")).default;const schema=(await import("../../../📥️preview/🧬️schema/🔁️replacement/🔣️.json")).default;const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,borrowedSourceOnly:false})).toBe(false);expect(validate({...law,sourceEpochs:[0,1,1]})).toBe(false);
 const first=JSON.parse(JSON.stringify(completeFixture.mesh)),second=JSON.parse(JSON.stringify(completeFixture.mesh));const handle=completeFixture.handle;
 let cache:{[key:string]:typeof first}={};const displaced=[];let epoch=0;const epochs=[epoch];
 for(const mesh of[first,second]){if(cache[handle])displaced.push(cache[handle]);cache=applyPatch(cache,[{op:cache[handle]?"replace":"add",path:`/${handle.replaceAll("~","~0").replaceAll("/","~1")}`,value:mesh}],true,false).newDocument;epochs.push(++epoch);}
 expect(epochs).toEqual(law.sourceEpochs);expect(law.borrowedSourceOnly).toBe(true);expect(Object.values(law.foreignEpochEffects)).toEqual([0,0,0,0]);expect(Object.keys(cache)).toHaveLength(law.expectedEntries);expect(cache[handle]).toBe(second);expect(displaced[0]).toBe(first);expect(displaced).toHaveLength(law.rounds-1);expect(law.copyBody).toBe(64);expect(law.terminalDropBytes).toBe(0);
});


test("original evaluation reply owns a typed complete Dictionary without text output",async()=>{
 const schema=(await import("../📨️response/🧬️schema/🔣️.json")).default,reply=(await import("../📨️response/🧫️fixtures/🧠️dictionary.json")).default;const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(reply)).toBe(true);for(const name of["outputJson","budget","retained","operation","generation"]){expect(validate({...reply,[name]:1})).toBe(false)}expect(validate({...reply,output:JSON.stringify(reply.output)})).toBe(false);const independent=JSON.parse(JSON.stringify(reply));expect(independent.output).toEqual(reply.output);expect(independent.output.solid.handle).toBe("brep:original-s源");expect(validate({...reply,done:false,cancellable:true,phase:"solverIteration",unitsDone:3,output:null})).toBe(true);expect(validate({...reply,done:true,cancellable:false,phase:"cancelled",output:null})).toBe(true);
});


test("original ordered Dictionary projection preserves typed logical source and one finite Pack reply caller",async()=>{
 const schema=(await loadNeuralEngineProjectionSchema()),law=(await loadNeuralEngineProjectionFixture());
 const responseSchema=(await import("../../../../🧩️extensions/🕸️wasm/🧬️schema/📤️response/🔣️.json")).default,response=(await import("../../../../🧩️extensions/🕸️wasm/🧪️tests/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).default.originalEnvelopeSource;
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,replyWire:"JSON"})).toBe(false);expect(validate({...law,projectionHeapBytes:1})).toBe(false);expect(new Ajv({strict:true}).compile(responseSchema)(response)).toBe(true);expect(response.replyWire).toBe("Pack");expect(response.caller.stock.maximumItems).toBeGreaterThan(0);expect(response.caller.clock).toBeLessThan(response.caller.deadline);
 const source=law.source;expect(Object.keys(source).sort()).toEqual(law.rootKeys);expect(source.nested.text).toBe(law.source.nested.text);expect(law.textPath.reduce((value:unknown,index:number)=>Object.values(value as Record<string,unknown>)[index],source)).toBe(source.nested.text);const moved=applyPatch<{original?:typeof source;receiver:typeof source|null}>({original:source,receiver:null},[{op:"move",from:"/original",path:"/receiver"}],true,true).newDocument;expect(moved.receiver).toBe(source);const logical=JSON.parse(JSON.stringify(moved.receiver));expect(logical).toEqual(source);expect(logical.negative).toBe(-2);expect(logical.null).toBeNull();
});


test("native mesh resource schema preserves original arena owner and paid shared loans",async()=>{
 const schema=(await loadSceneMathResourceSchema()),law=(await loadSceneMathResourceFixture());
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,wire:"JSON"})).toBe(false);expect(validate({...law,loans:{...law.loans,tokenOnlyClose:true}})).toBe(false);
 const original={pages:[{positions:[0,0,0],indices:[0]}],identity:{operation:941,generation:17,revision:23,cacheEpoch:1,sourceSlot:0,nativeSlot:0,nativeEpoch:1}};
 const state=applyPatch<{componentTree:typeof original|null;preparedFrame:typeof original|null}>({componentTree:original,preparedFrame:null},[{op:"move",from:"/componentTree",path:"/preparedFrame"}],true,true).newDocument;expect(state.preparedFrame).toBe(original);if(state.preparedFrame===null)throw new Error("The move operation did not fill the prepared frame");expect(state.preparedFrame.pages).toBe(original.pages);
 let loans=0;const loan=()=>{loans++;return original};const first=loan(),second=loan();expect(first).toBe(second);expect(loans===0).toBe(false);loans--;expect(loans===0).toBe(false);loans--;expect(loans===0).toBe(true);expect(law.admission.loanItems).toBe(1);expect(law.admission.loanCopyBytes).toBe(0);expect(law.admission.newArena).toBe(false);expect(law.close.dropEffects).toBe(0);
});

test("native geometry transport preserves independent Three geometry bytes and closed pinned source contract",async()=>{
 const schema=(await loadPresentResourcesTransportSchema()),law=(await loadPresentResourcesTransportFixture());
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);expect(validate({...law,tokenOnWire:true})).toBe(false);expect(validate({...law,maximumChunkBytes:65})).toBe(false);
 const {BufferGeometry,Float32BufferAttribute,Uint32BufferAttribute}=await import("three");
 const geometry=new BufferGeometry();geometry.setAttribute("position",new Float32BufferAttribute([0,0,0,1,0,0,0,1,0],3));geometry.setAttribute("normal",new Float32BufferAttribute([0,0,1,0,0,1,0,0,1],3));geometry.setIndex(new Uint32BufferAttribute([0,1,2],1));
 const independent=new Uint8Array(84);independent.set(new Uint8Array(geometry.getAttribute("position").array.buffer),0);independent.set(new Uint8Array(geometry.getAttribute("normal").array.buffer),36);independent.set(new Uint8Array(geometry.getIndex()!.array.buffer),72);
 expect(Array.from(independent)).toEqual(law.geometry);expect(JSON.parse(JSON.stringify(law.geometry))).toEqual(Array.from(independent));geometry.dispose();
});
