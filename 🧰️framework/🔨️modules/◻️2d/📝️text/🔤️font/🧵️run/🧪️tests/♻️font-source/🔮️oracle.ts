import type {FontWorkLedger} from "../../../🎟️work/🟦️.ts";
import {fontTestWork} from "../../../🧪️testing/🟦️.ts";
/** 🧪️ Borrowed immutable font sources survive completed and cancelled outline child retirement. */
import assert from "node:assert/strict";
import opentype from "opentype.js";
import {FontFaceAdmissionJob} from "../../../🟦️.ts";
import {UnicodeDataAdmissionJob,loadBuiltinUnicodeSource} from "../../../🧵️shape/🌐️unicode/🟦️.ts";
import {FontOutlineRunJob} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
export function checkBorrowedFaces(source:Uint8Array,unicodeSource:Uint8Array):number {const limits={maxFontBytes:67108864,maxTables:128,maxGlyphs:65536,maxPoints:1048576,maxContours:65536,maxComponents:262144,maxDepth:32,maxSegments:1048576,maxWork:1e9},settle=(job:{advance(n:number,work:FontWorkLedger):{done:boolean}})=>{while(!job.advance(4096,fontTestWork(4096)).done){}};const admission=new FontFaceAdmissionJob(source,limits),data=new UnicodeDataAdmissionJob(unicodeSource);settle(admission);settle(data);const face=admission.result(),unicode=data.result(),faces=Object.freeze([face]),oracle=opentype.parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.byteLength)as ArrayBuffer),expected=oracle.stringToGlyphs(fixture.text).map(glyph=>glyph.index);let runs=0;for(const stop of fixture.stops)for(const grant of fixture.grants){const job=new FontOutlineRunJob(faces,{text:fixture.text,size:11,origin:[0,0],lineHeight:1.2,kerning:false},limits,unicode);if(stop===null){settle(job);assert.deepEqual(job.result().glyphs.map(glyph=>glyph.glyph),expected);}else{if(stop)job.advance(stop,fontTestWork(stop));job.cancel();}const close=job.intoRetirement();let work=0;for(;;){const p=close.job.advance(grant,fontTestWork(grant));assert(p.work-work<=grant);work=p.work;if(p.done)break;}assert.equal(faces.length,fixture.expectedFaces);assert.equal(faces[0],face);assert.equal(face.bytes,source);runs++;}return runs;}
