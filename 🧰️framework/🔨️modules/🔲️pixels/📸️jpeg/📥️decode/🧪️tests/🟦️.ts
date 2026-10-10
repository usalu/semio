import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import sharp from "sharp";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import {JpegDecodeJob,type JpegDecodeInput,type JpegGrant} from "../🟦️.ts";
const validate=new Ajv2020({strict:true}).compile(schema);
const grant=(job:JpegDecodeJob):JpegGrant=>({maximumItems:1,maximumCopyBytes:job.nextCopyBytes(),maximumCapacityBytes:job.nextCapacityBytes(),maximumReleaseBytes:0,maximumDepth:1});
const input=(row:typeof fixture.cases[number]):JpegDecodeInput=>({data:Uint8Array.from(Buffer.from(row.base64,"base64")),maxPixels:16777216,maxBytes:67108864,maxSegments:65536,maxWorkingBytes:536870912});
test("JPEG neutral samples match independent libvips decoder and funded yielded reconstruction",async()=>{
 for(const row of fixture.cases){const request=input(row);expect(validate({...request,data:Array.from(request.data)})).toBe(true);const oracle=await sharp(Buffer.from(request.data)).ensureAlpha().raw().toBuffer({resolveWithObject:true});expect(Array.from(oracle.data)).toEqual(row.pixels);const job=new JpegDecodeJob(request);let previous=0;for(let step=0;step<1000000;step++){const state=job.advance(grant(job));expect(state.receipt.copiedItems).toBeLessThanOrEqual(1);expect(state.progress.work-previous).toBeLessThanOrEqual(1);previous=state.progress.work;if(state.progress.done)break;}const image=job.result();expect([image.width,image.height]).toEqual([row.width,row.height]);let maximum=0;for(let at=0;at<image.pixels.length;at++)maximum=Math.max(maximum,Math.abs(image.pixels[at]!-oracle.data[at]!));expect(maximum).toBeLessThanOrEqual(row.oracleTolerance);}
 console.info("[DEBUG] JPEG portable baseline/progressive reconstruction matched shared neutral pixels and independent libvips");
});
test("JPEG denied axes preserve original bytes and unpublished candidate; cancellation remains flags only",()=>{
 for(const row of fixture.cases){const request=input(row),job=new JpegDecodeJob(request);expect(job.sourceIdentity()).toBe(request.data);let phases=new Set<string>();for(let step=0;step<1000000;step++){const actual=grant(job),before=job.progress();for(const axis of ["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumDepth"]as const){if(actual[axis]===0)continue;const denied={...actual,[axis]:actual[axis]-1};const state=job.advance(denied);expect(state.progress).toEqual(before);expect(state.receipt).toEqual({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});expect(job.sourceIdentity()).toBe(request.data);}phases.add(before.phase);if(job.advance(actual).progress.done)break;}expect(phases.has("entropy")).toBe(true);expect(phases.has("transform")).toBe(true);expect(phases.has("pixels")).toBe(true);}
 for(const stop of fixture.interruptions){const request=input(fixture.cases[0]!),job=new JpegDecodeJob(request);for(let at=0;at<stop;at++)job.advance(grant(job));job.cancel();expect(job.sourceIdentity()).toBe(request.data);expect(()=>job.result()).toThrow(/cancel/);expect(()=>job.advance(grant(job))).toThrow(/cancel/);}
 console.info("[DEBUG] JPEG original source identity, independent denied grants and flags-only cancellation retained private owners");
});
