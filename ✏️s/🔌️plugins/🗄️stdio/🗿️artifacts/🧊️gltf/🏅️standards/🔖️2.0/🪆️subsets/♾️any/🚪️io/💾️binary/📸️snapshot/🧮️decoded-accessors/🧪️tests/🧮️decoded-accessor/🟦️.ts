/** 🧮️ Independent Node Buffer oracle for the neutral decoded accessor fixture. */
import {test,expect} from "bun:test";
import {Buffer} from "node:buffer";
import fixture from "../../../../../../🧬️schema/📸️snapshot/🧫️fixtures/🧮️decoded-accessor/🔣️.json";
test("decoded accessor neutral bytes agree with independent binary32 reads and writes",()=>{
 const bytes=Buffer.from(fixture.bytes);
 expect(fixture.components.map((_,index)=>bytes.readFloatLE(index*4))).toEqual(fixture.components);
 const encoded=Buffer.alloc(fixture.components.length*4);
 fixture.components.forEach((value,index)=>encoded.writeFloatLE(value,index*4));
 expect([...encoded]).toEqual(fixture.bytes);
 console.log(`[DEBUG] Node Buffer independently verified ${fixture.components.length} decoded glTF values`);
});
