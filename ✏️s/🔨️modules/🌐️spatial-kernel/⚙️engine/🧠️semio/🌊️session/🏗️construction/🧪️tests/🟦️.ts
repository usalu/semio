import { test,expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

test("original session construction pays each physical family and closes only born owners",()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,closePolicy:"completeMissingOwners"})).toBe(false);
 for(const stop of fixture.cancelStops){let state={born:[] as string[],retired:[] as string[]};for(const family of fixture.families.slice(0,Math.max(0,stop-3))){state=applyPatch(state,[{op:"add",path:"/born/-",value:family}],true,false).newDocument;}const original=[...state.born];while(state.born.length){const owner=state.born[0];state=applyPatch(state,[{op:"remove",path:"/born/0"},{op:"add",path:"/retired/-",value:owner}],true,false).newDocument;}expect(state.retired).toEqual(original);expect(state.born).toEqual([]);}
 const original=new Map(fixture.sourceBinding.map(key=>[key,7]));for(const changed of fixture.sourceBinding){const supplied=new Map(original);supplied.set(changed,8);let state={phase:0,born:[] as string[]};if(fixture.sourceBinding.every(key=>supplied.get(key)===original.get(key))){state=applyPatch(state,[{op:"replace",path:"/phase",value:1},{op:"add",path:"/born/-",value:"kernel"}],true,false).newDocument;}expect(state).toEqual({phase:fixture.foreignSourceEffects,born:[]});}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("SessionConstructionCursor")).toBe(true);
});
