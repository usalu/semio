import {expect,test} from "bun:test";
import {existsSync} from "node:fs";
import Ajv from "ajv";
import fixture from "../nested-io.json";
import { orderedRetirementSchema as contract } from "@semio-tech/value-ts/ordered/retirement/schema";

test("Nested IO original physical grant and native custody witness match canonical Ajv and independent JSON",()=>{
 expect(existsSync(new URL("../🧬️schema/💰️wallet.json",import.meta.url))).toBe(false);
 const validate=new Ajv({strict:true}).addSchema(contract).compile({$ref:contract.$id+"#/$defs/Grant"});expect(validate(fixture.grant)).toBe(true);expect(JSON.parse(JSON.stringify(fixture.original))).toBe(fixture.original);expect(fixture.expected.nestedFrames).toBe(2);expect(Buffer.byteLength(fixture.original,"utf8")).toBe(28);expect(fixture.grant.maximumDepth).toBeGreaterThan(fixture.expected.nestedFrames);
});
