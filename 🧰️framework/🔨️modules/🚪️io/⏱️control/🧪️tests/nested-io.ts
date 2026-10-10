import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import Ajv from "ajv";
import fixture from "./nested-io.json";
import contract from "../../../🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json";

test("Nested IO original physical grant and native custody witness match canonical Ajv and independent JSON",()=>{
 expect(existsSync(new URL("../🧬️schema/💰️wallet.json",import.meta.url))).toBe(false);
 const validate=new Ajv({strict:true}).addSchema(contract).compile({$ref:contract.$id+"#/$defs/Grant"});expect(validate(fixture.grant)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.original))).toBe(fixture.original);expect(fixture.expected.nestedFrames).toBe(2);expect(Buffer.byteLength(fixture.original,"utf8")).toBe(28);expect(fixture.grant.maximumDepth).toBeGreaterThan(fixture.expected.nestedFrames);
 console.error("[DEBUG] Nested IO neutral original grant and two-frame witness match canonical Grant Ajv and independent JSON UTF8; native physical results are separate");
});
