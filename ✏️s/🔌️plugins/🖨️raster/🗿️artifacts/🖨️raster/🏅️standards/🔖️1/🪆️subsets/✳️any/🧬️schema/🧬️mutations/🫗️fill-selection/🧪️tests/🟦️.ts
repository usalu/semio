/** 🫗️ `fill-selection` payload contract: every committed quintet's mutation is accepted alike by the leaf schema (Ajv) and
 * the TypeScript twin, and what the schema refuses the twin refuses too. The fill pixel law is the shared pixel corpus. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {parseFillSelection} from "../../🟦️.ts";

const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);
const root=join(import.meta.dir,"../../../../🧫️fixtures/🧬️mutations/🫗️fill-selection");
const cases=readdirSync(root);

test("every committed fill-selection mutation is accepted by the schema and the twin alike",()=>{
  expect(cases.length).toBe(7);
  for(const name of cases){
    const mutation=JSON.parse(readFileSync(join(root,name,"🦠️mutation/🔣️.json"),"utf8"));
    expect(validate(mutation),name).toBe(true);
    const {mutation:tag,...payload}=mutation;
    expect(tag).toBe("fillSelection");
    expect(parseFillSelection(mutation),name).toEqual(payload);
  }
});

test("what the fill-selection schema refuses the twin refuses",()=>{
  const valid={mutation:"fillSelection",layerId:"paint",target:"mask",color:[0.5,0.5,0.5,1],selection:[{start:0,length:3,coverage:255}]};
  expect(validate(valid)).toBe(true);
  for(const [what,broken] of [
    ["target",{...valid,target:"canvas"}],
    ["three channels",{...valid,color:[0,0,0]}],
    ["channel past 1",{...valid,color:[0,0,2,1]}],
    ["empty layer",{...valid,layerId:""}],
    ["a seed",{...valid,seed:{x:0,y:0}}],
    ["overlapping runs",{...valid,selection:[{start:4,length:2,coverage:255},{start:5,length:1,coverage:255}]}],
  ] as const){
    expect(()=>parseFillSelection(broken),what).toThrow();
    if(what!=="overlapping runs")expect(validate(broken),what).toBe(false);
  }
});
