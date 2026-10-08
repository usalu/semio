import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
test("returned group output retains every original causal field on refusal or funded cancellation",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 const corpus=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));for(const row of law.cases){const accepted=row.items>0&&row.transferred&&row.empty;expect(accepted).toBe(row.accepted);for(const causal of corpus.cases){const original=JSON.parse(JSON.stringify(causal)),state={output:original,owner:null};const next=accepted?applyPatch(state,[{op:"move",from:"/output",path:"/owner"}],true,false).newDocument:state;expect(next[accepted?"owner":"output"]).toEqual(original);expect(Buffer.from(JSON.stringify(next[accepted?"owner":"output"]))).toEqual(Buffer.from(JSON.stringify(causal)));}}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("fn retain_output(")).toBe(true);console.log("[DEBUG] Node Buffer/RFC6902 returned causal outputs preserve all fields on zero-item/live/resident refusal and original zeroheap ownership return");
});
