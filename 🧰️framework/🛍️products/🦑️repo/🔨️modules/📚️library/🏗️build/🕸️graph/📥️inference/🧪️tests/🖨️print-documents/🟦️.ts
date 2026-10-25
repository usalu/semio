/** 🖨️ Contract: the import closure of a Print compiler reaches every document target through ONE interned command-source group, never as a literal list repeated per document (98 documents x 2,470 paths made a 66 MB project that the inference reference capacity refused); the group expands to exactly the literal inputs, and esbuild finds the same closure. */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{createRequire}from"node:module";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{readInferenceSourceV1}from"../📖️source/🟦️.ts";

const inference=await import("../../🟨️.mjs"),{createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const targets=await import("../../🧩️component-targets/🟨️.mjs") as any,observation=await import("../../../../👁️observation/📥️reuse/🟨️.mjs") as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const documents=24,modules=60;
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const workspace=()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"print-documents-"));roots.push(root);
 const files:Record<string,string>={"package.json":"{}","prod/lib/a.sty":"x","prod/bundle/fonts.json":"{}","prod/mid/comp/cat/catalog.json":JSON.stringify({version:1,librarySources:["lib"],documents:Array.from({length:documents},(_,index)=>({id:`doc-${index}`,collection:index%2?"visualizations":"templates",texPath:`docs/${index}.tex`,sources:[`docs/${index}.bib`]}))})};
 for(let index=0;index<documents;index++){files[`prod/docs/${index}.tex`]="t";files[`prod/docs/${index}.bib`]="b";}
 files["prod/mid/comp/📜️script.ts"]=Array.from({length:modules},(_,index)=>`import "./m${index}.ts";`).join("\n")+"\n";
 for(let index=0;index<modules;index++)files[`prod/mid/comp/m${index}.ts`]=`export const m${index}=${index};\n`;
 write(root,files);return root;
};
const project={name:"print",metadata:{printCatalog:"prod/mid/comp/cat/catalog.json"},targets:{fonts:{inputs:["{workspaceRoot}/prod/bundle/fonts.json"]},build:{executor:"nx:run-commands"},"build-viz":{executor:"nx:run-commands"}}};
const documentTargets=(targetsOf:Record<string,any>)=>Object.entries(targetsOf).filter(([name])=>name.startsWith("build-")&&name!=="build-viz");
const expand=(inputs:any[],groups:Map<string,any[]>)=>new Set(inputs.flatMap(input=>typeof input==="string"&&groups.has(input)?groups.get(input)!:[input]).map(input=>JSON.stringify(input)));

test("every document target names the compiler closure through one shared group and expands to the literal inputs",async()=>{
 const root=workspace(),scripts=createScriptInputCache({workspaceRoot:root});
 const literal=await targets.printDocumentTargets(project,"prod/mid/comp",root,scripts);
 const groups=new Map<string,any[]>(),interned=await targets.printDocumentTargets(project,"prod/mid/comp",root,scripts,(inputs:any[])=>{const name=`commandSources${groups.size}`;groups.set(name,inputs);return[name];});
 expect(groups.size).toBe(1);
 const rows=documentTargets(interned);expect(rows.length).toBe(documents);
 for(const[name,target]of rows){expect(target.inputs.filter((input:unknown)=>typeof input==="string"&&input.startsWith("commandSources")).length,name).toBe(1);expect(target.inputs.length,name).toBeLessThan(10);expect(expand(target.inputs,groups),name).toEqual(expand(literal[name].inputs,new Map()));}
 expect(JSON.stringify(interned).length*4).toBeLessThan(JSON.stringify(literal).length);
});

test("the shared group holds exactly the closure esbuild finds for the compiler script",async()=>{
 const root=workspace(),scripts=createScriptInputCache({workspaceRoot:root}),groups=new Map<string,any[]>();
 await targets.printDocumentTargets(project,"prod/mid/comp",root,scripts,(inputs:any[])=>{groups.set("g",inputs);return["g"];});
 const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,"prod/mid/comp/📜️script.ts")],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});
 const found=Object.keys(result.metafile.inputs).map((path:string)=>`{workspaceRoot}/${path}`),group=new Set(groups.get("g")!.filter((input:unknown)=>typeof input==="string"));
 expect(found.length).toBe(modules+1);for(const path of found)expect(group.has(path),path).toBe(true);
});

test("an interned Print project is captured by the inference reference check",async()=>{
 const root=workspace(),scripts=createScriptInputCache({workspaceRoot:root}),groups=new Map<string,any[]>();
 const interned=await targets.printDocumentTargets(project,"prod/mid/comp",root,scripts,(inputs:any[])=>{groups.set("g",inputs);return["g"];});
 expect(()=>observation.captureInferenceInventoryReferencesV1({targets:interned,namedInputs:Object.fromEntries(groups)})).not.toThrow();
});

test("the project assembly hands its interner to the Print targets",()=>{
 expect(readInferenceSourceV1("project-assembly")).toContain("printDocumentTargets(json, root, workspaceRoot, scripts, internCommandSources)");
});
