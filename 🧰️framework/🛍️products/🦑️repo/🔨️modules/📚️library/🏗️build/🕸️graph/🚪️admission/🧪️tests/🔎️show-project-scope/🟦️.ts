/** 🔎️ Contract: a question whose answer is determined by one project reads that project's closure alone (a selected read with scope {project}), while a question about every project stays a whole read that is answered from the sealed generation under the policy current; the runtime command takes both from the selection function, not from a literal. */
import {expect,test} from "bun:test";
import fixture from "../🔣️.json" with {type:"json"};
const runtimeSource=await Bun.file(new URL("../../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url)).text();
import {buildInvocationGraphSelectionV1,buildGraphScopeKeyV1,readBuildOwnerRegistryV1} from "../../🟦️.ts";

test("show project <p> plans a selected read with scope {p}, the same scope as exec --projects=<p>",()=>{
 const registry=readBuildOwnerRegistryV1(fixture.registry);
 for(const name of ["app","registry","@semio-tech/repo-mcp"]){
  const selection=buildInvocationGraphSelectionV1(["show","project",name]);
  expect(selection).toEqual({kind:"owners",projects:[name],exclude:[]});
  expect(buildGraphScopeKeyV1(selection)).toBe(buildGraphScopeKeyV1(buildInvocationGraphSelectionV1(["exec","--projects="+name,"--","true"])));
 }
});

test("questions about every project and non-literal project names stay whole reads",()=>{
 expect(buildInvocationGraphSelectionV1(["show","projects"])).toEqual({kind:"all"});
 expect(buildInvocationGraphSelectionV1(["show","targets"])).toEqual({kind:"all"});
 expect(buildInvocationGraphSelectionV1(["graph"])).toEqual({kind:"all"});
 expect(buildInvocationGraphSelectionV1(["show","project","*"])).toEqual({kind:"all"});
 expect(buildInvocationGraphSelectionV1(["show","project"])).toEqual({kind:"all"});
 expect(buildInvocationGraphSelectionV1(["affected","--targets=test"])).toEqual({kind:"all"});
});

test("the runtime command reads show project over the selection and defaults a whole question to the sealed graph",()=>{
 const source=String(runtimeSource);
 expect(source).toContain("buildInvocationGraphSelectionV1(args)");expect(source).toContain('sealedProject?{kind:"all" as const}:buildInvocationGraphSelectionV1(args)');
 expect(source).toContain('policy??(whole?"current":"fresh")');
 expect(source).not.toContain('selection:{kind:"all"},reuse:process.env.SEMIO_BUILD_REUSE_GRAPH');
});
