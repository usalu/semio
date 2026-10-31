import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{compileJsonSchemaSubset}from"../../🟦️.ts";
import{semioSchemaAjvV1}from"../../../📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
const corpus=JSON.parse(readFileSync(new URL("../../../🧪️tests/🧩️module-compile/📥️inputs.json",import.meta.url),"utf8")) as{referenceCases:{id:string;root:string;documents:string[];compiles:boolean}[]};
for(const row of corpus.referenceCases)test("portable URI reference "+row.id,()=>{
 const root=JSON.parse(row.root),documents=row.documents.map(body=>JSON.parse(body)),ajv=semioSchemaAjvV1({strict:false});let oracle=true,owned=true;
 try{for(const document of documents)ajv.addSchema(document);ajv.compile(root);}catch{oracle=false;}
 try{compileJsonSchemaSubset(root,root,documents);}catch{owned=false;}
 expect(oracle).toBe(row.compiles);expect(owned).toBe(row.compiles);
});

import {pathToFileURL} from "node:url";
import {resolve} from "node:path";
import * as validatorApi from "../../🟦️.ts";
import sourceCorpus from "../../../📄️source/🔗️closure/🧪️tests/🧫️fixtures/🔣️.json";
const sourceControl=()=>({limits:{sourceBytes:2000000,documentBytes:1000000,coordinateBytes:1000000,depth:256,schemas:10000,resources:10000,references:10000,chunk:128},cancelled:()=>false,progress:()=>{},yield:async()=>{}});
for(const row of sourceCorpus.cases.filter(row=>row.request.dialect==="draft-07"))test("captured Source validation retains retrieval authority: "+row.id,async()=>{
 const compile=(validatorApi as any).compileJsonSchemaSourceSubset;expect(typeof compile).toBe("function");
 if("error" in row.expected){await expect(compile(row.request,sourceControl())).rejects.toMatchObject({code:row.expected.error});return;}
 const owned=await compile(row.request,sourceControl());if(!row.oracle?.compile)return;const oracle=semioSchemaAjvV1({strict:false});for(const input of row.request.resources)oracle.addSchema(JSON.parse(input.source),input.baseUri);oracle.addSchema(JSON.parse(row.request.root.source),row.request.root.baseUri);const independent=oracle.getSchema(row.request.root.baseUri)!;
 for(const instance of row.oracle.instances){expect(owned.validate(instance.value).length===0,row.id).toBe(instance.valid);expect(Boolean(independent(instance.value)),row.id).toBe(instance.valid);}
});
test("the actual Project and Producer documents compile at their physical retrieval URIs",async()=>{
 const root=resolve(import.meta.dir,"../../../../../.."),project=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json"),binding=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🧬️schema/🔣️.json"),capture=(path:string)=>({id:path,path,baseUri:pathToFileURL(path).href,source:readFileSync(path,"utf8")}),request={dialect:"draft-07",root:capture(project),resources:[capture(binding)],annotations:[]},compile=(validatorApi as any).compileJsonSchemaSourceSubset;
 expect(typeof compile).toBe("function");const before=JSON.stringify(request),owned=await compile(request,sourceControl()),oracle=semioSchemaAjvV1();for(const input of[...request.resources,request.root])oracle.addSchema(JSON.parse(input.source),input.baseUri);const independent=oracle.getSchema(request.root.baseUri)!,manifest=JSON.parse(readFileSync(resolve(root,"✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/📋️project.json"),"utf8"));
 for(const accepted of[true,false]){const value=structuredClone(manifest);if(!accepted)value.metadata.semio.artifactPackage.kind="unowned";expect(owned.validate(value).length===0).toBe(accepted);expect(Boolean(independent(value))).toBe(accepted);}expect(JSON.stringify(request)).toBe(before);
 await expect(compile({...request,resources:[]},sourceControl())).rejects.toMatchObject({code:"unknown-resource"});await expect(compile(request,{...sourceControl(),cancelled:()=>true})).rejects.toMatchObject({code:"cancelled"});const bounded=sourceControl();bounded.limits.sourceBytes=1;await expect(compile(request,bounded)).rejects.toMatchObject({code:"source-byte-budget"});
});

test("Source compilation snapshots authority while cooperative scheduling proceeds",async()=>{
 const row=sourceCorpus.cases.find(row=>row.id==="recursive-object")!,request=structuredClone(row.request),control=sourceControl();control.limits.chunk=1;let changed=false;control.yield=async()=>{if(!changed){changed=true;request.dialect="2020-12";request.root.source="false";}};
 const owned=await (validatorApi as any).compileJsonSchemaSourceSubset(request,control);expect(changed).toBe(true);expect(owned.validate({next:{}})).toEqual([]);expect(owned.validate({next:1}).length).toBeGreaterThan(0);
 const unsupported={...structuredClone(row.request),dialect:"2020-12"};await expect((validatorApi as any).compileJsonSchemaSourceSubset(unsupported,sourceControl())).rejects.toThrow("requires draft-07");
});
