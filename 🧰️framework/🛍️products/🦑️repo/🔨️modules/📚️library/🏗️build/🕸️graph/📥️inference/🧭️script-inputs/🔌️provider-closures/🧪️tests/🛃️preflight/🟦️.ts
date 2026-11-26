/** 🛃️ Contract: a closure that can never be derived is found before a read derives anything. The preflight returns every refusal (not the first); a fixture import of a generated output into a tool closure is refused with the tool and the producer; and every declared tool closure and provider port of the real tree walks without importing a generated output, so a landing that introduces such an import fails here and not 90 minutes into the group read. Run by every landing that touches a tool closure, a provider or a file they import. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import corpus from "../🔣️.json" with {type:"json"};
import policy from "../../🔣️.json" with {type:"json"};

setDefaultTimeout(600000);
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🟨️.mjs") as any,projectNodes=await import("../../../../📦️project-nodes/🟨️.mjs") as any,ownerDiscovery=await import("../../../../🔎️owner-discovery/🟨️.mjs") as any;
const designationSchema=join(import.meta.dir,"../../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const workspace=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"pf-law-")));roots.push(root);for(const[path,text]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const infer=(body:()=>Promise<any>):Promise<any>=>observation.observeInferenceSnapshotV1(body);
const fixtureCache=(root:string,generated:boolean)=>{
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:root,scriptProviderDesignation:join(root,corpus.designation)});
 cache.toolGroups=new Map(Object.keys(policy.tools.closures).filter(name=>name!=="cargo-command").map(name=>[name,new Set<string>()]));
 if(generated)cache.generatedOutput=(path:string)=>path===join(root,"lib/gen.ts")?{target:"fixture:generate",output:"lib/gen.ts"}:undefined;
 return cache;
};

test("a fixture tool closure that imports a generated output is refused with the tool and the producer, and the same tree without the import is accepted",async()=>{
 const root=workspace();
 const clean=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,true)));
 expect(clean.refused).toEqual([]);expect(clean.checked).toBeGreaterThan(30);
 writeFileSync(join(root,"lib/gen.ts"),"export const generated = 1;\n");
 writeFileSync(join(root,"lib/util.ts"),'import "./gen.ts";\n'+readFileSync(join(root,"lib/util.ts"),"utf8"));
 const found=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,true)));
 const tool=found.refused.find((row:any)=>row.name==="cargo-command");
 expect(tool,JSON.stringify(found.refused)).toBeDefined();
 expect(tool.code).toBe("invalid");expect(tool.key).toBe("build.closure.provider-generated-import");expect(tool.values).toEqual({port:"cargo-command",targets:"fixture:generate"});
 for(const row of found.refused)expect(row.key).toBe("build.closure.provider-generated-import");
 const without=await infer(()=>scriptInputs.preflightToolClosuresV1(root,fixtureCache(root,false)));
 expect(without.refused).toEqual([]);
});

test("every declared tool closure and provider port of the real tree is derivable and imports no generated output",async()=>{
 const checkout=process.env.SEMIO_TEST_CHECKOUT??resolve(import.meta.dir,"../".repeat(12)),contracts=await infer(async()=>projectNodes.projectGeneratorContractsV1(checkout));
 const cache=scriptInputs.createScriptInputCache({workspaceRoot:checkout});
 cache.generatedOutput=(path:string)=>ownerDiscovery.readBuildProducedCompileOutputV1(path,checkout,contracts,undefined);
 const answer=await infer(()=>scriptInputs.preflightToolClosuresV1(checkout,cache));
 expect(answer.refused.map((row:any)=>row.name+" "+row.key+" "+JSON.stringify(row.values)),"a tool closure or provider closure imports a generated output (declare the import in the command, or remove the tool row from the provider-closure policy)").toEqual([]);
 expect(answer.checked).toBeGreaterThan(30);
});
