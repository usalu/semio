/** 🧫️ Independent schema and carrier descriptors witness exact logical TIFF words. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parse,buildASTSchema} from "graphql";
import schema from "../../🔣️.json";
import fixture from "../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";
import {parseTiffSnapshot} from "../../🟦️.ts";
test("Ajv and the TypeScript owner admit the neutral exact words",()=>{const validate=new Ajv({strict:false}).compile(schema);for(const item of fixture.cases){expect(validate(item.snapshot),JSON.stringify(validate.errors)).toBe(true);expect(JSON.stringify(parseTiffSnapshot(item.snapshot))).toBe(JSON.stringify(item.snapshot));}for(const item of fixture.reject){expect(validate(item)).toBe(false);expect(()=>parseTiffSnapshot(item)).toThrow();}});
test("GraphQL publishes word inputs and owned blocks without native policy",async()=>{const source=await Bun.file(new URL("../../🔗️.graphql",import.meta.url)).text(),document=buildASTSchema(parse(source.replace(/ @state\(class: ARTIFACT\)/g,"")));const fields=(document.getType("TiffSnapshot") as any).getFields();expect(Object.keys(fields)).toEqual(["schema","ifds"]);expect((document.getType("TiffWord64Input") as any).getFields().lo.type.toString()).toBe("UInt32!");expect(document.getType("TiffByteOrder")).toBeUndefined();});
