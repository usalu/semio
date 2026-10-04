/** 🔄️ `transform-image` payload contract: every committed quintet's mutation is accepted alike by the leaf schema (Ajv) and
 * the TypeScript twin, and what the schema refuses the twin refuses too. The pixel law of every grid change is the shared
 * pixel corpus (`🔲️pixels/✍️editing`), checked by the Rust engine and the TypeScript twin. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {semioSchemaAjvV1} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import {parseTransformImage} from "../../🟦️.ts";

const validate=semioSchemaAjvV1({allErrors:true}).compile(schema);
const root=join(import.meta.dir,"../../../../🧫️fixtures/🧬️mutations/🔄️transform-image");
const cases=readdirSync(root);

test("every committed transform-image mutation is accepted by the schema and the twin alike",()=>{
  expect(cases.length).toBe(8);
  for(const name of cases){
    const mutation=JSON.parse(readFileSync(join(root,name,"🦠️mutation/🔣️.json"),"utf8"));
    expect(validate(mutation),name).toBe(true);
    const {mutation:tag,...payload}=mutation;
    expect(tag).toBe("transformImage");
    expect(parseTransformImage(mutation),name).toEqual(payload);
  }
});

test("what the transform-image schema refuses the twin refuses",()=>{
  const crop={mutation:"transformImage",layerId:"paint",operation:"crop",x:1,y:1,width:3,height:2,bilinear:false};
  const turn={mutation:"transformImage",layerId:"paint",operation:"rotateClockwise",x:0,y:0,width:0,height:0,bilinear:false};
  const resize={mutation:"transformImage",layerId:"paint",operation:"resize",x:0,y:0,width:12,height:8,bilinear:true};
  for(const valid of [crop,turn,resize]){expect(validate(valid)).toBe(true);const {mutation:_,...payload}=valid;expect(parseTransformImage(valid)).toEqual(payload);}
  for(const [what,broken] of [
    ["unknown operation",{...turn,operation:"skew"}],
    ["a turn with an extent",{...turn,width:2}],
    ["a turn with an origin",{...turn,y:1}],
    ["a turn that samples",{...turn,bilinear:true}],
    ["a resize with an origin",{...resize,x:1}],
    ["an empty resize",{...resize,height:0}],
    ["a crop that samples",{...crop,bilinear:true}],
    ["an empty crop",{...crop,width:0}],
    ["a side past 16384",{...resize,width:16385}],
    ["fractional extent",{...resize,width:2.5}],
    ["empty layer",{...crop,layerId:""}],
    ["unknown field",{...crop,target:"pixels"}],
  ] as const){
    expect(()=>parseTransformImage(broken),what).toThrow();
    expect(validate(broken),what).toBe(false);
  }
});
