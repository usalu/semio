import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "./birth-chain.schema.json";
import law from "./birth-chain.json";
/** 🌱️ A birth chain turn is atomic: it is born exactly when its Work budget covers the whole fresh chain, and each stage quote is spent exactly. */
test("return birth chain cut at every turn budget",()=>{
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const sum=(stages:number[])=>stages.reduce((total,stage)=>total+stage,0),needed=sum(law.fresh);
 expect(needed).toBeGreaterThan(sum(law.refresh));
 const born=law.cuts.map(cut=>cut.fuel>=needed);
 expect(born.some(Boolean)&&!born.every(Boolean)).toBe(true);
 expect(law.cuts.map(cut=>cut.fuel)).toEqual(expect.arrayContaining([needed-1,needed]));
});
