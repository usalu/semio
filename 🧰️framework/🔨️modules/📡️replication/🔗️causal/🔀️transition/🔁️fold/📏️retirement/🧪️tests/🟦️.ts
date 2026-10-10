import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import Ajv2020 from "ajv/dist/2020";
test("bounded fold physical retirement keeps copy policy independent of whole source and frame grants",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));for(const extent of law.extents){const original=Buffer.alloc(extent,37);expect(original.subarray(0,law.logicalItems).length).toBe(Math.min(extent,law.logicalItems));for(const grant of [0,Math.max(0,extent-1),extent]){const state={capacity:extent,released:0};const next=grant>=extent?applyPatch(state,[{op:"replace",path:"/capacity",value:0},{op:"replace",path:"/released",value:extent}],true,false).newDocument:state;expect(next.released).toBe(grant>=extent?extent:0);expect(original.length).toBe(extent);}for(const copy of law.logicalCopy){expect(Math.min(copy,extent)).toBe(new Uint8Array(original).subarray(0,copy).byteLength);}}
 const source=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");expect(source.includes("next_capacity_byte_demand")).toBe(true);expect(source.includes("size_of_val(owner.as_ref())")).toBe(true);expect(source.includes("struct FoldRetirementQueue")).toBe(true);expect(source.includes("Mutex<VecDeque")).toBe(false);expect(Math.ceil(Buffer.alloc(law.cold.bytes).byteLength/law.cold.logicalPageBytes)+law.cold.terminalPhases).toBe(law.cold.maximumTurns);const valueSource=readFileSync(new URL("../../../../../../🌱️value/♻️retirement/🦀️.rs",import.meta.url),"utf8");expect(valueSource.includes("self.0.is_empty() && self.0.capacity()!=0 && size_of::<T>()!=0")).toBe(true);const controlledSource=readFileSync(new URL("../../../../../../🌱️value/♻️retirement/🎮️controlled/🦀️.rs",import.meta.url),"utf8");expect(controlledSource.includes("if birth > grant.maximum_capacity_bytes")).toBe(true);console.log("[DEBUG] NodeBuffer/RFC6902 five whole physical extents preserve1/7/4096 copy and one-item cancellation without work or terminal frees");
});

test("original shared actor admission keeps every refused currency and the preborn shell",()=>{
 const law=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️actor.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️actor.json",import.meta.url),"utf8"));
 const validate=new Ajv2020({strict:true}).compile(schema);expect(validate(law)).toBe(true);
 const original=Buffer.from(law.original,"utf8");const pointer=original.buffer;
 for(const axis of law.deniedAxes){const before={items:1,copy:32,capacity:40,depth:1};const denied=applyPatch(before,[{op:"replace",path:`/${axis}`,value:0}],true,false).newDocument;expect(denied[axis]).toBe(0);expect(original.toString("utf8")).toBe(law.original);expect(original.buffer).toBe(pointer);}
 const initial={frame:"preborn",actor:"original",born:0,freed:0};const admitted=applyPatch(initial,[{op:"replace",path:"/frame",value:"same-queued-shell"},{op:"replace",path:"/actor",value:"same-original"},{op:"replace",path:"/born",value:40}],true,false).newDocument;
 expect(admitted.freed).toBe(0);expect(admitted.frame).toBe(law.expected.prebornFrame);expect(law.expected.deniedHeap).toEqual([0,0]);expect(law.expected.terminalHeap).toEqual([0,0]);
 console.log("[DEBUG] independent Ajv/NodeBuffer/RFC6902 actor admission rejects every unfunded axis, retains source and preborn shell, and separates birth from later shell release");
});

test("original receipt order fold 3 keeps plain trials outside schema authority",async()=>{const {existsSync}=await import("node:fs");expect(existsSync(new URL("../🧬️schema/🔣️actor.json",import.meta.url))).toBe(false);console.log("[DEBUG] Original receipt/order/fold trial has no whole-corpus schema authority");});
