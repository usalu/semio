/** 🧪️ Neutral PNG samples match independent pngjs with bounded publication and cancellation. */
import Ajv from "ajv";
import domainSchema from "../../../🧬️schema/🔣️.json";
import ioSchema from "../🧬️schema/🔣️.json";
import assert from "node:assert/strict";
import {PNG} from "pngjs";
import refusals from "../🧫️fixtures/⚠️invalid/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {DrawingImageAdmissionJob,DrawingImageEmissionJob,drawingImageDataUri} from "../🟦️.ts";
import {RasterSceneJob} from "../../../🧬️schema/🎬️scene/📷️raster/🟦️.ts";
export async function testDrawingImageAdmission():Promise<void>{
 const validator=new Ajv({strict:false,validateFormats:false}).addSchema(domainSchema).addSchema(ioSchema),validProgress=validator.compile({$ref:ioSchema.$id+"#/$defs/EmissionProgress"}),validInput=validator.compile({$ref:ioSchema.$id+"#/$defs/EmissionInput"}),validGrant=validator.compile({$ref:ioSchema.$id+"#/$defs/WorkGrant"});
 for(const grant of fixture.grants)assert(validGrant(grant));for(const grant of fixture.invalidGrants)assert(!validGrant(grant));
 let laws=0;
 for(const row of fixture.cases)for(const data of [Buffer.from(row.png).toString("base64"),"data:image/png;base64,"+Buffer.from(row.png).toString("base64")]){
  const input={mime:"image/png",data,maxSourceBytes:4096,maxBytes:4096,maxPixels:4096,maxChunks:64};
  for(const grant of fixture.grants){const job=new DrawingImageAdmissionJob(input);assert.throws(()=>job.result());let prior=0;for(let i=0;i<100000;i++){const progress=job.advance(grant);assert(progress.work-prior<=grant);prior=progress.work;if(progress.done)break;}assert.deepEqual(job.result(),row.expected);laws++;}
  const foreign=PNG.sync.read(Buffer.from(row.png));assert.deepEqual(Array.from(foreign.data),row.expected.samples.flat());
  const retainedUriOwner=structuredClone(row.expected);let uriWork=0;const uri=drawingImageDataUri(row.expected,state=>{assert(validProgress(state));assert(state.work>uriWork);uriWork=state.work;return true;}),uriPixels=PNG.sync.read(Buffer.from(uri.slice(uri.indexOf(",")+1),"base64"));assert.deepEqual(Array.from(uriPixels.data),row.expected.samples.flat());assert.deepEqual(row.expected,retainedUriOwner);laws++;
  for(const stop of ["encoding","base64","done"]){assert.throws(()=>drawingImageDataUri(row.expected,state=>stop==="done"?!state.done:state.phase!==stop),/cancel/i);assert.deepEqual(row.expected,retainedUriOwner);laws++;}

  for(const grant of fixture.grants){const original=structuredClone(row.expected);assert(validInput({asset:original,maximumPixels:4096,maximumEncodedBytes:4096}));const job=new DrawingImageEmissionJob(original,4096,4096);assert.throws(()=>job.result());let previous=0;for(let at=0;at<100000;at++){const progress=job.advance(grant);assert(validProgress(progress));assert(progress.work-previous<=grant);previous=progress.work;if(progress.done)break;}const encoded=PNG.sync.read(Buffer.from(job.result()));assert.equal(encoded.width,row.expected.width);assert.equal(encoded.height,row.expected.height);assert.deepEqual(Array.from(encoded.data),row.expected.samples.flat());assert.deepEqual(original,row.expected);laws++;}
  for(const stop of [0,1,row.expected.samples.length+1]){const original=structuredClone(row.expected);assert(validInput({asset:original,maximumPixels:4096,maximumEncodedBytes:4096}));const job=new DrawingImageEmissionJob(original,4096,4096);for(let at=0;at<stop;at++){if(job.advance(1).done)break;}job.cancel();assert.throws(()=>job.advance(1));assert.throws(()=>job.result());assert.deepEqual(original,row.expected);laws++;}
  const retained=structuredClone(row.expected),refused=new DrawingImageEmissionJob(row.expected,4096,8);assert.throws(()=>{while(!refused.advance(1).done){}});assert.throws(()=>refused.result());assert.deepEqual(row.expected,retained);
  const pixels=Uint8Array.from(row.expected.samples.flat()),scene=new RasterSceneJob({width:row.expected.width,height:row.expected.height,origin:[0,0],tolerance:.01,maxPixels:4096,maxSourceBytes:4096,assets:[{id:"admitted",image:{width:row.expected.width,height:row.expected.height,pixels}}],nodes:[{id:"image",groups:[],transform:[1,0,0,1,0,0],opacity:1,blendMode:"normal",visible:true,content:{kind:"image",asset:"admitted",width:row.expected.width,height:row.expected.height}}]});
  let rasterWork=0;for(let i=0;i<100000;i++){const progress=scene.advance(1);assert(progress.work-rasterWork<=1);rasterWork=progress.work;if(progress.done)break;}assert.deepEqual(Array.from(scene.result().pixels),Array.from(foreign.data));assert.deepEqual(Array.from(pixels),row.expected.samples.flat());laws++;

  for(const stop of [0,1,10,100]){const job=new DrawingImageAdmissionJob(input);for(let i=0;i<stop;i++){if(job.advance(1).done)break;}job.cancel();assert.throws(()=>job.advance(1));assert.throws(()=>job.result());laws++;}
  for(const grant of fixture.invalidGrants){const job=new DrawingImageAdmissionJob(input);assert.throws(()=>job.advance(grant));assert.throws(()=>job.result());}
 }
 for(const data of ["not-base64!!",Buffer.from("not a PNG").toString("base64")]){const job=new DrawingImageAdmissionJob({mime:"image/png",data,maxSourceBytes:4096,maxBytes:4096,maxPixels:4096,maxChunks:64});assert.throws(()=>{while(!job.advance(7).done){}});assert.throws(()=>job.result());assert.throws(()=>job.advance(1));}
 let refusalLaws=0;
 for(const row of refusals)for(const grant of fixture.grants){const before=structuredClone(row.input);assert.throws(()=>{
  const input=row.input,assets=[];let sourceBytes=0;
  for(const source of input.assets){sourceBytes+=new TextEncoder().encode(source.data).length;assert(sourceBytes<=input.maxSourceBytes);const job=new DrawingImageAdmissionJob({mime:source.mime,data:source.data,maxSourceBytes:input.maxSourceBytes,maxBytes:input.maxBytes,maxPixels:input.maxPixels,maxChunks:input.maxChunks});for(let at=0;at<1000000;at++){if(job.advance(grant).done)break;}const asset=job.result();assets.push({id:source.id,image:{width:asset.width,height:asset.height,pixels:Uint8Array.from(asset.samples.flat())}});}
  const job=new RasterSceneJob({...input,assets} as any);for(let at=0;at<1000000;at++){if(job.advance(grant).done)break;}job.result();
 },row.name);assert.deepEqual(row.input,before);refusalLaws++;}
 assert.equal(refusalLaws,129);console.log(`[DEBUG] Draw original physical scene refusal vectors=${refusals.length}; independent grants=${refusalLaws}; original raw inputs retained`);
 console.log(`[DEBUG] Draw physical image admission completed neutral/grant/cancellation witnesses=${laws}; independent PNG samples=${fixture.cases.length}`);
}
