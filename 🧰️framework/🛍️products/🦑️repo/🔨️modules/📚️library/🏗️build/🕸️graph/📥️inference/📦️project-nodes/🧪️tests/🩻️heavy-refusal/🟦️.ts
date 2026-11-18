/** 🩻️ Contract: a refusal of a graph read that stopped an owner too heavy for the machine always names the owner, its need, the cap and the number of machines that delivered records, in both languages, and survives the process boundary; a refusal that never could be built (a value without a placeholder) is no heavy refusal and is never rendered as one with an empty name and "0 MB" (g1-m07, 2026-11-17: `producers` was passed to a text without that placeholder, so every heavy refusal of the take loop failed to build and the host rendered the failure as an empty one).
 * @see ../../../../../📖️texts/🟦️.ts
 * @see ../../../../../🏡️resident/🏛️service/🧯️failure/🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync,readdirSync,statSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{refuseBuildV1}from"../../../../../📖️texts/🟦️.ts";
import{refuseHeavyOwnerV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import{renderRepoErrorV1}from"../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import{errorMessageV1,errorOfMessageV1}from"../../../../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs";
import{failureRefusalV1,heavyRefusalOfV1}from"../../../../../🏡️resident/🏛️service/🧯️failure/🟨️.mjs";
const schemaText=readFileSync(new URL("../../../../../🏡️resident/🧬️schema/🔣️.json",import.meta.url),"utf8");

const values={named:"@semio-tech/s-dev-composition (about 1716 MB)",limit:4200,producers:0};
const refuse=(error:unknown)=>failureRefusalV1(error,{log:()=>undefined,logPath:"/log",home:"/Users/worker",platform:"darwin",root:"/Users/worker/semio"});

test("the refusal builds with the owner, the cap and the machines that delivered, and renders them in English and German",()=>{
 const error=refuseHeavyOwnerV1(values);
 expect(error.key).toBe("build.graph.owner-too-heavy");expect(error.code).toBe("unserved");
 for(const language of["en","de"]){const text=renderRepoErrorV1(error,language);expect(text).toContain("s-dev-composition");expect(text).toContain("4200");expect(text).toContain("1716");expect(text).toMatch(/\b0\b/u);}
});

test("the old shape (a value the text has no placeholder for) cannot be built at all, so it can never reach the host as a nameless refusal",()=>{
 expect(()=>refuseBuildV1("unserved","graph.owner-too-heavy",{named:"x",limit:1,extra:2} as never)).toThrow();
});

test("across the worker wire and the host failure core the names, the cap and the producers arrive in both languages of the refusal",()=>{
 const back=errorOfMessageV1(errorMessageV1(refuseHeavyOwnerV1(values)));
 expect(back.key).toBe("build.graph.owner-too-heavy");expect(Object.fromEntries(Object.entries(back.values))).toEqual(values);
 expect(heavyRefusalOfV1(back)).toEqual({named:values.named,limit:"4200",producers:"0"});
 const shown=refuse(back);expect(shown).toEqual({code:"unserved",key:"host-graph-owner-too-heavy",fields:{named:values.named,limit:"4200",producers:"0"},retry:"later"});
 const templates=schemaText.match(/"host-graph-owner-too-heavy": "[^"]*"/gu)??[];expect(templates).toHaveLength(2);for(const template of templates){expect(template).toContain("{named}");expect(template).toContain("{limit}");expect(template).toContain("{producers}");}
});

test("a failure that only mentions the key is no heavy refusal: no values, no names, no limit means no refusal named empty",()=>{
 for(const error of[new Error("unknown-placeholder:graph.owner-too-heavy:producers"),Object.assign(new Error("graph.owner-too-heavy"),{name:"RepoError",key:"build.graph.owner-too-heavy",values:{}}),Object.assign(new Error("graph.owner-too-heavy"),{name:"RepoError",key:"build.graph.owner-too-heavy"}),new Error("outer",{cause:new Error("unserved build.graph.owner-too-heavy")})])expect(heavyRefusalOfV1(error)).toBeUndefined();
 expect(refuse(new Error("unknown-placeholder:graph.owner-too-heavy:producers")).key).not.toBe("host-graph-owner-too-heavy");
});

test("every throw site of the library builds the refusal through the one builder, with no other literal of the key left",()=>{
 const library=fileURLToPath(new URL("../../../../../",import.meta.url)),found:string[]=[];
 const walk=(directory:string)=>{for(const name of readdirSync(directory)){if(name==="node_modules"||name==="🧪️tests")continue;const path=join(directory,name),stat=statSync(path);if(stat.isDirectory())walk(path);else if(/\.(mjs|ts)$/u.test(name)&&!name.endsWith(".d.mts")&&readFileSync(path,"utf8").includes('"graph.owner-too-heavy"'))found.push(path.slice(library.length));}};
 walk(library);expect(found.filter(path=>!path.startsWith("🪪️graph-state/🙋️wanted/💰️cost/")),found.join("\n")).toEqual([]);
});
