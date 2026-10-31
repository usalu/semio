/** 🧬️ The genuine shared neutral reference is closed by const; strictTypes lint is redundant with that exact object const. */
import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {createHash} from "node:crypto";
import schema from "./📐️.json";
import cases from "./🔣️.json";
test("closed flatten neutral schema pins its exact existing reference and preserves the original counterexample",()=>{const {reference,...previousConst}=schema.const,previous={...schema,const:previousConst};expect(createHash("sha256").update(JSON.stringify(previous,null,2)+"\n").digest("hex")).toBe("5c80ce222ed49774fe2bbb3089a1fa28bd2861329cbfebb54c4dba6a1c20246d");const ajv=new Ajv2020({strict:true,strictTypes:false}),admit=ajv.compile(schema),old=ajv.compile(previous);expect(old(cases)).toBe(false);expect(admit(cases)).toBe(true);expect(reference).toEqual(cases.reference);expect(admit({...cases,foreign:true})).toBe(false);const hostile=structuredClone(cases);hostile.reference.rgba[0]=0;expect(admit(hostile)).toBe(false);});
