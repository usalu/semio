/** 📦️ Native Bun ESM and require expose canonical values without erased interfaces. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {createRequire} from "node:module";
import * as actual from "../🟦️.ts";
import * as canonical from "../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import law from "./module.json";
import schema from "./module.schema.json";
test("actual native Bun Font work module exports only real canonical runtime values",()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const native=createRequire(import.meta.url)("../🟦️.ts");for(const name of law.runtime){expect(Reflect.get(actual,name)).toBe(Reflect.get(canonical,name));expect(Reflect.get(native,name)).toBe(Reflect.get(canonical,name));}for(const name of [...law.declarations,...law.forbidden]){expect(Reflect.has(actual,name)).toBe(false);expect(Reflect.has(native,name)).toBe(false);}});
