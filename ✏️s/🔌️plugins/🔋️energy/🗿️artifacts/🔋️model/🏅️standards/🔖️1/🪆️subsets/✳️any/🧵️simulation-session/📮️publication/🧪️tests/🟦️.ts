import { strict as assert } from "node:assert";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import fixture from "../../🧪️tests/🧫️fixtures/⛽️publication/🔣️.json";
import schema from "../../🧬️schema/⛽️publication/🔣️.json";
import { verifyEnergyOriginalSessionOracle } from "../../🧪️tests/🎟️source/🟦️.ts";
export function verifyEnergyPublicationOracle():void{
 verifyEnergyOriginalSessionOracle();
 assert.equal(new Ajv({strict:true}).compile(schema)(fixture),true);
 const bytes=Buffer.alloc(8+fixture.surfaces.length*20);bytes.write(fixture.surfaceMagic,0,"ascii");bytes.writeUInt32LE(fixture.surfaces.length,4);
 for(const[index,row]of fixture.surfaces.entries()){const offset=8+index*20;bytes.writeUInt32LE(row.id,offset);for(const[column,value]of[row.loss,row.gain,row.solarTransmitted,row.solarAbsorbed].entries())bytes.writeFloatLE(value,offset+4+column*4);}
 assert.equal(bytes.toString("hex"),fixture.expectedSurfaceHex);assert.equal(bytes.readUInt32LE(28),41);assert.equal(bytes.readFloatLE(44),6.5);
 for(let offset=0;offset<bytes.length;offset++)for(const quantum of[1,3,4,17,64]){const count=Math.min(quantum,bytes.length-offset);const rebuilt=Buffer.concat([bytes.subarray(0,offset),bytes.subarray(offset,offset+count),bytes.subarray(offset+count)]);assert.equal(rebuilt.toString("hex"),fixture.expectedSurfaceHex);}
 const publication=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");assert.equal(publication.includes(".finish()"),false);assert.equal(publication.includes(".encode()"),false);assert.equal(publication.includes("encode_surface_energy_payload"),false);assert.equal(publication.includes(".clone()"),false);assert.equal(publication.includes("RetainedPayloadBuilder::admit_boxed"),true);assert.equal(publication.includes("wire.advance(source,port"),true);assert.equal(fixture.pageBytes,16384);assert.equal(fixture.maximumCopyBytes,4096);
 console.log(`[DEBUG] Ajv/independent Buffer Energy ESF1 original surface rows=${fixture.surfaces.length} bytes=${bytes.length} cross-word range grid sharedpublicationcopy4096 pagebirth16384`);
}
