/** 🪣️ `fill-region` payload contract: every committed quintet's mutation is accepted alike by the leaf schema (Ajv) and the
 * TypeScript twin, and what the schema refuses the twin refuses too. The flood + fill pixel law itself is the shared pixel
 * corpus (`🔲️pixels/✍️editing`, `floodSelections`), checked by the Rust engine, the TypeScript twin and a Python oracle. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {parseFillRegion} from "../../🟦️.ts";

const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);
const root=join(import.meta.dir,"../../../../🧫️fixtures/🧬️mutations/🪣️fill-region");
const cases=readdirSync(root);

test("every committed fill-region mutation is accepted by the schema and the twin alike",()=>{
  expect(cases.length).toBe(8);
  for(const name of cases){
    const mutation=JSON.parse(readFileSync(join(root,name,"🦠️mutation/🔣️.json"),"utf8"));
    expect(validate(mutation),name).toBe(true);
    const {mutation:tag,...payload}=mutation;
    expect(tag).toBe("fillRegion");
    expect(parseFillRegion(mutation),name).toEqual(payload);
  }
});

test("what the fill-region schema refuses the twin refuses",()=>{
  const valid={mutation:"fillRegion",layerId:"paint",target:"pixels",seed:{x:1,y:2},tolerance:24,color:[0,0.5,1,1],selection:null};
  expect(validate(valid)).toBe(true);
  for(const [what,broken] of [
    ["target",{...valid,target:"canvas"}],
    ["seed off the grid",{...valid,seed:{x:-1,y:2}}],
    ["fractional seed",{...valid,seed:{x:1.5,y:2}}],
    ["tolerance past 255",{...valid,tolerance:256}],
    ["three channels",{...valid,color:[0,0,0]}],
    ["channel past 1",{...valid,color:[0,0,2,1]}],
    ["empty layer",{...valid,layerId:""}],
    ["unknown field",{...valid,opacity:1}],
    ["overlapping runs",{...valid,selection:[{start:4,length:2,coverage:255},{start:5,length:1,coverage:255}]}],
  ] as const){
    expect(()=>parseFillRegion(broken),what).toThrow();
    if(what!=="overlapping runs")expect(validate(broken),what).toBe(false);
  }
});
