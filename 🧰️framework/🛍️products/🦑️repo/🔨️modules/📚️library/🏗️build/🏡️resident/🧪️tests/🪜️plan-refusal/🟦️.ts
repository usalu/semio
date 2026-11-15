/** 🪜️ Contract: a graph plan that was stopped for its size keeps its words (stage, size, limit) through the values, the message text and the cause, becomes the typed refusal `unserved` that is retried later in both languages, the service maps the stop of the pool to the same refusal, and the catalogs hold the texts with the same placeholders in English and German.
 * @see ../../🏛️service/🧯️failure/🟨️.mjs */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {failureRefusalV1,heavyRefusalOfV1,planRefusalOfV1} from "../../🏛️service/🧯️failure/🟨️.mjs";
import {renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {BUILD_TEXTS_V1,refuseBuildV1} from "../../../📖️texts/🟦️.ts";
import {textV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";

const logs:string[]=[],refuse=(error:unknown)=>failureRefusalV1(error,{log:(event,detail)=>{logs.push(event+" "+detail);},logPath:"/log",root:"/Users/Shared/semio"});
const values={stage:"edges",operation:"manifests",megabytes:9400,limit:4915},fields={stage:"edges",operation:"manifests",megabytes:"9400",limit:"4915",available:"?",reserve:"?"};

test("the stop of a plan keeps its stage and numbers through values, message text and cause, and its refusal is retried later",()=>{
 const message="unserved build.graph.plan-too-heavy "+JSON.stringify(values);
 expect(planRefusalOfV1(refuseBuildV1("unserved","graph.plan-too-heavy",values))).toEqual(fields);
 expect(planRefusalOfV1(Object.assign(new Error(message),{name:"RepoError",key:"build.graph.plan-too-heavy",values}))).toEqual(fields);
 expect(planRefusalOfV1(new Error(message))).toEqual(fields);
 expect(planRefusalOfV1(new Error("outer",{cause:new Error(message)}))).toEqual(fields);
 expect(planRefusalOfV1(new Error("secret"))).toBeUndefined();expect(heavyRefusalOfV1(new Error(message))).toBeUndefined();
 expect(refuse(new Error(message))).toEqual({code:"unserved",key:"host-graph-plan-too-heavy",fields,retry:"later"});
 expect(logs.at(-1)).toBe("graph-plan-too-heavy "+JSON.stringify(fields));
});

test("the measures of the host travel with the stop of a plan",()=>{
 const measured={...values,available:1500,reserve:2048};
 expect(planRefusalOfV1(new Error("unserved build.graph.plan-too-heavy "+JSON.stringify(measured)))).toEqual({...fields,available:"1500",reserve:"2048"});
});

test("both catalogs hold the refusal with the same placeholders and render it with the stage and the numbers",()=>{
 const holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 const texts=readFileSync(new URL("../../🧬️schema/🔣️.json",import.meta.url),"utf8");
 expect(texts.match(/"host-graph-plan-too-heavy": "[^"]*\{stage\}[^"]*\{operation\}[^"]*\{megabytes\}[^"]*\{limit\}[^"]*\{available\}[^"]*\{reserve\}/gu)).toHaveLength(2);
 const rendered=renderTextV1("host-graph-plan-too-heavy",fields,["en","de"])!;
 for(const language of ["en","de"] as const){expect(rendered[language]).toContain("edges");expect(rendered[language]).toContain("9400");expect(rendered[language]).toContain("4915");expect(rendered[language]).not.toMatch(/\{[a-z]+\}/u);}
 expect(rendered.en).toContain("plan stage");expect(rendered.de).toContain("Plan-Stufe");
 const inProcess={stage:"edges",operation:"manifests",megabytes:"9400",limit:"4915"},build=BUILD_TEXTS_V1.catalog.messages["graph.plan-too-heavy" as never] as unknown as {en:string;de:string};
 expect(holes(build.en)).toEqual(holes(build.de));expect(holes(build.en)).toEqual(["limit","megabytes","operation","stage"]);
 expect(textV1(BUILD_TEXTS_V1.catalog,"graph.plan-too-heavy","en",inProcess)).toContain("plan stage edges (during manifests) grew to 9400 MB");
 expect(textV1(BUILD_TEXTS_V1.catalog,"graph.plan-too-heavy","de",inProcess)).toContain("Plan-Stufe edges (während manifests) ist auf 9400 MB gewachsen");
});

test("the service maps the stop of the pool (code plan-memory) to the same refusal key and the pool and the worker use the same stage message",()=>{
 const service=readFileSync(new URL("../../🏛️service/🌐️host/🟨️.mjs",import.meta.url),"utf8");
 expect(service).toContain('error?.code==="plan-memory"');expect(service).toContain('log("graph-plan-too-heavy",JSON.stringify({stage:error.stage,operation:error.operation,megabytes:error.currentMegabytes,limit:error.limitMegabytes,available:error.availableMegabytes,reserve:error.reserveMegabytes,reason:error.reason,worker:error.sample}))');expect(service).toContain('"unserved","host-graph-plan-too-heavy",{stage:error.stage,operation:error.operation,megabytes:error.currentMegabytes,limit:error.limitMegabytes,available:error.availableMegabytes,reserve:error.reserveMegabytes},"later"');
 const pool=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs",import.meta.url),"utf8"),worker=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts",import.meta.url),"utf8");
 expect(pool).toContain('message?.type==="stage"');expect(worker).toContain("onPlanStageV1(event=>parentPort!.postMessage(event))");
});
