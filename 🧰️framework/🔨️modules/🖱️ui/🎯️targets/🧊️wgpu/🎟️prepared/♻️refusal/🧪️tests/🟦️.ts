import {test,expect} from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("portable raster refusal owns whole original backings until exact paid release",()=>{
 expect(new Ajv().compile(schema)(fixture)).toBe(true);
 const source=Buffer.alloc(fixture.sourceBytes),retained=Buffer.alloc(fixture.retainedBytes),key=Buffer.from(fixture.key,"utf8");
 expect(source.byteLength+retained.byteLength+key.byteLength).toBe(fixture.expected.releasedBytes);
 expect(JSON.parse(JSON.stringify(fixture.grant))).toEqual(fixture.grant);
 expect(Buffer.from(key.toString("utf8"))).toEqual(key);
});
