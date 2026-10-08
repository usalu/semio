/** 🧪️ Artifact-owned physical transport corpus and independent oracles. */
import {test,expect,afterAll} from "bun:test";
import Ajv from "ajv";
import {Buffer} from "node:buffer";
import fixture from "./🧫️fixtures/🔣️.json";
import {parseSemioGraphJsonValue,semioGraphJsonValue} from "./../../🟦️.ts";
const oracle=new Ajv({strict:false});
test("Graph JSON preserves nonfinite words and ordered literal fields",()=>{
 const owned=parseSemioGraphJsonValue(fixture.graph);
 expect(owned.nodes[0]!.position.x.bits).toBe(0x8000000000000000n);
 expect(owned.nodes[0]!.width.bits).toBe(0x7ff8000000000042n);
 expect(oracle.compile({const:fixture.graph})(semioGraphJsonValue(owned))).toBe(true);
});
afterAll(()=>console.log("[DEBUG] Owned transport corpus completed: @semio-tech/stdio-semio"));
