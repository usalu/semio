/** 🌈️ `apply-filter` payload contract: every committed quintet's mutation is accepted alike by the leaf schema (Ajv) and the
 * TypeScript twin, and what the schema refuses the twin refuses too. The pixel law of every filter is the shared pixel
 * corpus (`🔲️pixels/✍️editing`), checked by the Rust engine and the TypeScript twin. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {parseApplyFilter} from "../../🟦️.ts";

const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);
const root=join(import.meta.dir,"../../../../🧫️fixtures/🧬️mutations/🌈️apply-filter");
const cases=readdirSync(root);

test("every committed apply-filter mutation is accepted by the schema and the twin alike",()=>{
  expect(cases.length).toBe(8);
  for(const name of cases){
    const mutation=JSON.parse(readFileSync(join(root,name,"🦠️mutation/🔣️.json"),"utf8"));
    expect(validate(mutation),name).toBe(true);
    const {mutation:tag,...payload}=mutation;
    expect(tag).toBe("applyFilter");
    expect(parseApplyFilter(mutation),name).toEqual(payload);
  }
});

test("what the apply-filter schema refuses the twin refuses",()=>{
  const valid={mutation:"applyFilter",layerId:"paint",filter:"brightness",amount:0.5,selection:null};
  expect(validate(valid)).toBe(true);
  expect(parseApplyFilter(valid)).toEqual({layerId:"paint",filter:"brightness",amount:0.5,selection:null});
  for(const [what,broken] of [
    ["a target",{...valid,target:"mask"}],
    ["unknown filter",{...valid,filter:"emboss"}],
    ["brightness past 1",{...valid,amount:1.5}],
    ["fractional blur",{...valid,filter:"blur",amount:1.5}],
    ["posterize below 2",{...valid,filter:"posterize",amount:1}],
    ["an amount for invert",{...valid,filter:"invert",amount:1}],
    ["a flip with a selection",{...valid,filter:"flipHorizontal",amount:0,selection:[{start:0,length:2,coverage:255}]}],
    ["empty layer",{...valid,layerId:""}],
    ["unknown field",{...valid,opacity:1}],
    ["overlapping runs",{...valid,selection:[{start:4,length:2,coverage:255},{start:5,length:1,coverage:255}]}],
  ] as const){
    expect(()=>parseApplyFilter(broken),what).toThrow();
    if(what!=="overlapping runs")expect(validate(broken),what).toBe(false);
  }
});
