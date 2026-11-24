/** 🎯️ Contract: a `run-many` project selector with wildcards, negation, comma lists or `name:` and `directory:` prefixes resolves against the declared owner registry, so it admits exactly the owners of the equivalent exact list and never the whole workspace; selectors that need inferred tags keep the complete read and say so in both languages. */
import {expect,test} from "bun:test";
import picomatch from "picomatch";
import {buildInvocationGraphSelectionV1,buildWholeReadReasonV1,closeBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,readBuildGraphSelectionV1,buildGraphScopeKeyV1} from "../../🟦️.ts";
import {selectBuildNamesV1} from "../../../../🧭️selection/📥️options/🟦️.ts";

const names=["semio-hub-flow-component","semio-s-plugin-flow-math-component","semio-s-plugin-flow-brep-component","semio-s-plugin-flow-brep-mesh","semio-s-plugin-flow-bim-component","semio-s-plugin-other-component","@semio-tech/repo-mcp","app"],
 registry=readBuildOwnerRegistryV1({version:1,owners:names.map((name,index)=>({name,root:"owner"+index,manifest:"owner"+index+"/📋️project.json",packages:[]})),catalogProviders:[]}),
 entries=registry.owners.map(({name,root})=>({name,root}));
const oracle=(include:string[],exclude:string[]=[])=>names.filter(name=>include.some(pattern=>picomatch(pattern)(name))&&!exclude.some(pattern=>picomatch(pattern)(name))).sort();

test("pattern expansion over declared names equals an independent picomatch oracle",()=>{
 const cases:{projects:string[];exclude?:string[]}[]=[
  {projects:["semio-s-plugin-flow-*-component"]},
  {projects:["semio-s-plugin-flow-*"],exclude:["*-mesh"]},
  {projects:["semio-hub-flow-component","semio-s-plugin-flow-*-component"]},
  {projects:["semio-*-component","app"],exclude:["semio-hub-*","*-other-*"]},
  {projects:["@semio-tech/*"]},
  {projects:["nothing-*"]}
 ];
 for(const row of cases)expect(selectBuildNamesV1(entries,row.projects,row.exclude??[]),row.projects.join(",")).toEqual(oracle(row.projects,row.exclude));
 expect(selectBuildNamesV1(entries,["semio-s-plugin-flow-*","!semio-s-plugin-flow-brep-*"])).toEqual(["semio-s-plugin-flow-bim-component","semio-s-plugin-flow-math-component"]);
 expect(selectBuildNamesV1(entries,["!app"])).toEqual(names.filter(name=>name!=="app").sort());
 expect(selectBuildNamesV1(entries,["name:app","directory:owner1"])).toEqual(["app","semio-s-plugin-flow-math-component"]);
 expect(selectBuildNamesV1([{name:"a",root:"a",tags:["scope:x"]},{name:"b",root:"b",tags:[]}],["tag:scope:*"])).toEqual(["a"]);
 expect(()=>selectBuildNamesV1(entries,["a[b]"])).toThrow("Unsupported build selector pattern");
});

test("a glob selection requests the same owner set and closure as the equivalent exact list",async()=>{
 const glob=buildInvocationGraphSelectionV1(["run-many","-t","component","-p","semio-s-plugin-flow-*-component"]),
  exact=buildInvocationGraphSelectionV1(["run-many","-t","component","-p",selectBuildNamesV1(entries,["semio-s-plugin-flow-*-component"]).join(",")]);
 expect(glob.kind).toBe("owners");
 const admit=async(selection:ReturnType<typeof buildInvocationGraphSelectionV1>)=>{const seen:string[]=[],closed=await closeBuildOwnerAdmissionV1(selection,registry,async owner=>{seen.push(owner.name);return{project:{name:owner.name,targets:{component:{}}},files:[owner.manifest],dependencies:[]};});return{owners:closed.owners,seen:seen.sort()};};
 const expected=oracle(["semio-s-plugin-flow-*-component"]),left=await admit(glob),right=await admit(exact);
 expect(left.owners).toEqual(expected);expect(left).toEqual(right);expect(left.seen.length).toBeLessThan(names.length);
 expect(readBuildGraphSelectionV1(glob)).toEqual(glob);expect(buildGraphScopeKeyV1(glob,registry)).toMatch(/^[0-9a-f]{64}$/u);
});

test("comma lists, repeated options, negation and excludes scope the read; order of a negation is kept",()=>{
 expect(buildInvocationGraphSelectionV1(["run-many","-t","component","-p","semio-hub-*,semio-s-plugin-flow-*-component","--exclude=*-bim-*"])).toEqual({kind:"owners",projects:["semio-hub-*","semio-s-plugin-flow-*-component"],exclude:["*-bim-*"],targets:["component"]});
 expect(buildInvocationGraphSelectionV1(["run-many","-t","component","-p","semio-s-plugin-*","-p","!semio-s-plugin-other-*"])).toEqual({kind:"owners",projects:["semio-s-plugin-*","!semio-s-plugin-other-*"],exclude:[],targets:["component"]});
});

test("selectors needing inferred tags or selecting everything keep the whole read and announce it in English and German",()=>{
 for(const projects of ["tag:scope:flow","*","!app","semio-*,!tag:x","a[b]"]){
  const args=["run-many","-t","component","-p",projects];expect(buildInvocationGraphSelectionV1(args),projects).toEqual({kind:"all"});
  const reason=buildWholeReadReasonV1(args);expect(reason,projects).toMatch(/whole workspace\nProjektauswahl|Project selector[^\n]*\n[^\n]*Arbeitsbereich/u);
 }
 expect(buildWholeReadReasonV1(["run-many","-t","component","-p","semio-*-component"])).toBeUndefined();
 expect(buildWholeReadReasonV1(["run","app:test"])).toBeUndefined();
});
