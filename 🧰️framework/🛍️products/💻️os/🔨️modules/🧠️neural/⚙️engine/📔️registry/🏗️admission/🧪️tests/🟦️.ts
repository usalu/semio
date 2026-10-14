import {test,expect} from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "../🧫️fixtures/🔣️.json" with {type:"json"};
test("original registry header admission returns the same source and closes only born shells",()=>{
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,sourceReturn:"clonedRegistry"})).toBe(false);
 for(const stop of fixture.cancelStops){let state={source:"original",born:[] as string[]};if(stop>=2){state=applyPatch(state,[{op:"add",path:"/born/-",value:"finalRoot"}],true,false).newDocument;}if(stop>=3){state=applyPatch(state,[{op:"add",path:"/born/-",value:"registry"}],true,false).newDocument;}while(state.born.length){state=applyPatch(state,[{op:"remove",path:"/born/0"}],true,false).newDocument;}expect(state).toEqual({source:"original",born:[]});}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("SharedRegistryIssuer")).toBe(true);
 console.log("[DEBUG] Ajv and independent JSONPatch return the original owned registry while only actual born root shells close; native allocator and original caller laws remain separate");
});
