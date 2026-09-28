/** 🧪️ Asset replacement plans preserve capacity and exact undo with an RFC 6902 reference. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import patch from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {replacementSteps} from "../🟦️.ts";
for(const row of fixture.cases)test(row.name,()=>{
  expect(new Ajv().compile(schema)(row.input)).toBe(true);
  if(!row.steps){expect(()=>replacementSteps(row.input)).toThrow();return;}
  const steps=replacementSteps(row.input);expect(steps).toEqual(row.steps);
  let document={target:"previous" as string|null,assets:Object.fromEntries(Array.from({length:row.input.count},(_,i)=>[i===0?"previous":i===1&&row.input.nextExists?"next":"other-"+i,i]))};
  const before=structuredClone(document),inverses:patch.Operation[][]=[];
  for(const step of steps){
    const prior=structuredClone(document);
    const operation:patch.Operation=step==="detach"?{op:"replace",path:"/target",value:null}:step==="remove"?{op:"remove",path:"/assets/previous"}:step==="add"?{op:"add",path:"/assets/next",value:99}:{op:"replace",path:"/target",value:"next"};
    document=patch.applyPatch(document,[operation],true,false).newDocument;
    expect(Object.keys(document.assets).length).toBeLessThanOrEqual(row.input.capacity);
    if(document.target)expect(document.assets).toHaveProperty(document.target);
    inverses.push(patch.compare(document,prior));
  }
  expect(document.target).toBe("next");
  expect(Object.hasOwn(document.assets,"previous")).toBe(!row.input.removable);
  for(const inverse of inverses.reverse()){
    document=patch.applyPatch(document,inverse,true,false).newDocument;
    expect(Object.keys(document.assets).length).toBeLessThanOrEqual(row.input.capacity);
    if(document.target)expect(document.assets).toHaveProperty(document.target);
  }
  expect(document).toEqual(before);
});
