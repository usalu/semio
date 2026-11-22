/** 🪜️ Contract: the partial graph discovery marks every plan stage of the declared policy exactly once and in the declared order, runs its whole capture inside the plan guard (so a plan that outgrows the machine is stopped with the typed refusal `graph.plan-too-heavy`), and builds that refusal from the guard's own numbers.
 * @see ../../🧭️discovery/🟦️.ts */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {PLAN_STAGE_POLICY_V1} from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";

const source=readFileSync(new URL("../../🧭️discovery/🟦️.ts",import.meta.url),"utf8"),part=source.slice(source.indexOf("export async function discoverBuildGraphPartV1"));

test("every declared plan stage is marked once, in the declared order, inside the part discovery only",()=>{
 const marks=[...part.matchAll(/(?<![\w.])(?:stages\.)?mark\("([a-z-]+)"\)/gu)].map(match=>match[1]);
 expect(marks).toEqual([...PLAN_STAGE_POLICY_V1.stages]);
 expect(source.slice(0,source.indexOf("export async function discoverBuildGraphPartV1")).includes("stages.mark(")).toBe(false);
});

test("the capture runs inside the guard, the guard refuses with the catalog key and the three numbers, and the closing brackets match",()=>{
 expect(part).toContain("const captured=await stages.run(()=>withClosurePassV1(()=>observationV1.captureObservedSourceInputsV1(async()=>{");
 expect(part).toContain("},undefined,false,rowLog))).catch(error=>{rowLog.close();throw error;});");
 expect(part).toContain('createPlanStageGuardV1({refuse:detail=>refuseBuildV1("unserved","graph.plan-too-heavy",detail)})');
 const catalog=JSON.parse(readFileSync(new URL("../../../📖️texts/🔣️.json",import.meta.url),"utf8")).messages["graph.plan-too-heavy"];
 for(const language of ["en","de"])for(const hole of ["stage","operation","megabytes","limit"])expect(catalog[language]).toContain("{"+hole+"}");
});

test("the environment stage reports its operations to the guard, which makes every batch of the derivation a throw point",()=>{
 expect(part).toContain("read,stat,operation:name=>stages.operation(name)});");
});
