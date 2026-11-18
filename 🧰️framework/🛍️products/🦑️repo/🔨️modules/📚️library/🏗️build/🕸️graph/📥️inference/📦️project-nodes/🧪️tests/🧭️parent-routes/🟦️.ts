/** 🧭️ Contract: every road from the parent of a read to reference resolution, record inference or closure derivation ends in a farm job or in an owner set aside with its reason, never in the guard. The references of an owner are asked of a child again (up to five rounds, a record that is not stored yet is waited for 2.5 s), the compute step of an owner refuses with `graph.farm-job-failed` while a farm exists, the verdict wait and the reference wait name themselves in the parked line.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{farmJobRefusedV1}from"../../🟨️.mjs";

const source=readFileSync(join(import.meta.dir,"..","..","🟨️.mjs"),"utf8");

test("an owner a child cannot serve is refused with the owner, the reason and both languages, carrying the root the set-aside looks for",()=>{
 const error=farmJobRefusedV1("@semio-tech/stdio-artifact-contract-rs","stdio/artifact","the closure references (the reference file did not hold)") as Error&{code:string;semioProject:string};
 expect(error.code).toBe("graph.farm-job-failed");expect(error.semioProject).toBe("stdio/artifact");
 expect(error.message).toContain("the owner is set aside");expect(error.message).toContain("zurückgestellt");expect(error.message).toContain("reference file did not hold");
});

test("the reference road: no fall-through to the parent resolution while a farm exists, five rounds of asking, a missing record waited for",()=>{
 const start=source.indexOf("try{let adopted,last="),end=source.indexOf("return await resolveClosureRefsV1(result.projects[name]"),block=source.slice(start,end);
 expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);
 expect(block).toContain("round<5");expect(block).toContain("viaFarm()");
 expect(block).toContain('if(viaFarm())throw farmJobRefusedV1(name,root,"the closure references (');
 expect(block.indexOf("throw farmJobRefusedV1")).toBeLessThan(block.length);
 expect(block).toContain("no stored record");expect(block).toContain("setTimeout(done,2500)");
});

test("the compute road, the verdict wait and the reference wait",()=>{
 expect(source).toContain('if(farm?.exists?.())throw farmJobRefusedV1(name,root,"the owner (the farm gave no record)");return inferProjectBodyV1(projectEnvironment,');
 expect(source).toContain("stageV1(name,STAGE_VERDICT_V1)");expect(source).toContain("stageV1(name,STAGE_REFS_FARM_V1)");
 expect(source).toContain("the parent verifies nothing");expect(source).toContain("the parent resolves nothing");
});
