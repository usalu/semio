/** 🥞️ Contract: an unexpected failure of a request reaches the client with its code, message and the first stack frames in the client's own output (both languages), not only in a host-local log.
 * @see ../../🏛️service/🧯️failure/🟨️.mjs */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {failureFramesV1,failureRefusalV1} from "../../🏛️service/🧯️failure/🟨️.mjs";

const table=JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const find=(node:any):any=>node&&typeof node==="object"?(node.en&&node.de&&node.en["internal"]?node:Object.values(node).map(find).find(Boolean)):undefined;

test("the refusal of an unexpected failure carries the first stack frames, bounded, and both texts print them",()=>{
 const error=Object.assign(new TypeError("x is not iterable"),{code:"E_BOOM"});
 const refusal=failureRefusalV1(error,{log:()=>undefined,logPath:"/log",root:"/Users/Shared/semio"});
 expect(refusal.key).toBe("internal");
 const frames=(refusal.fields as any).frames as string;
 expect(frames.split("\n").filter(Boolean).length).toBeGreaterThan(0);expect(frames.split("\n").filter(Boolean).length).toBeLessThanOrEqual(6);
 for(const line of frames.split("\n").filter(Boolean))expect(line.trim().startsWith("at ")).toBe(true);
 expect(frames.length).toBeLessThanOrEqual(1500);
 const texts=find(table);
 for(const language of["en","de"])expect(texts[language].internal).toContain("{frames}");
});

test("no stack means no frames, and a huge stack is cut",()=>{
 expect(failureFramesV1("plain string")).toBe("");
 const long=Object.assign(new Error("m"),{stack:"Error: m\n"+Array.from({length:50},(_,index)=>"    at f"+index+" ("+"p".repeat(400)+")").join("\n")});
 const frames=failureFramesV1(long);
 expect(frames.split("\n").filter(Boolean).length).toBe(6);expect(frames.length).toBeLessThanOrEqual(1500);
});
