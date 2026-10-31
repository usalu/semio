import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{mkdtemp,mkdir,symlink,writeFile,rm,realpath,readFile,lstat}from"node:fs/promises";
import{join}from"node:path";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import{javascriptDependencyBindingDiagnosticV1,observeJavascriptDependencyBindingsReadV1,type JavascriptDependencyReadContextV1}from"../🟦️.ts";
import schema from"../📋️binding-diagnostic.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
declareBuildTestEnvelopeV1({owner:"test:javascript-binding-diagnostic",maximumElapsedMilliseconds:30000,capabilities:{artifacts:true}});
test("real filesystem dependency binding refusal under genuine control includes bounded exact metadata and preserves links",()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:binding-observation",0),root=await mkdtemp(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"bd-")),context:JavascriptDependencyReadContextV1=Object.freeze({root,invocation:original,control,check:actual=>checkResourceLeaseControl(actual),recheck:async actual=>{checkResourceLeaseControl(actual);}}),validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{const target=join(root,"target"),other=join(root,"other");await mkdir(target);await mkdir(other);
 for(let index=0;index<corpus.cases.length;index++){const row=corpus.cases[index]!,source=join(root,"s"+index);if(row.kind==="symlink")await symlink(row.matches?target:other,source,process.platform==="win32"?"junction":"dir");else if(row.kind==="directory")await mkdir(source);else await writeFile(source,"unchanged");
 const before=await lstat(source,{bigint:true});let error:unknown;try{observeJavascriptDependencyBindingsReadV1(context,[{sourceDirectory:source,targetDirectory:target}],control);}catch(cause){error=cause;}
 const expected=db.query<{accepted:number},[string,number]>("select (?='symlink' and ?=1) accepted").get(row.kind,Number(row.matches))!.accepted===1;expect(expected).toBe(row.accepted);expect(error===undefined).toBe(expected);
 if(!expected){expect(error).toBeInstanceOf(Error);const text=(error as Error).message,prefix=schema.$defs.Policy.const.prefix;expect(text.startsWith(prefix)).toBe(true);const actual=JSON.parse(text.slice(prefix.length));expect(validate(actual),JSON.stringify(validate.errors)).toBe(true);expect(actual).toEqual({version:1,purpose:"javascript-dependency-binding-mismatch",sourceDirectory:source,expectedTarget:target,observedTarget:row.kind==="symlink"?other:null,kind:row.kind,truncated:false});expect(Buffer.byteLength(text)).toBeLessThanOrEqual(schema.$defs.Policy.const.maximumMessageBytes);}
 const after=await lstat(source,{bigint:true});expect([after.dev,after.ino,after.mode,after.size,after.mtimeNs]).toEqual([before.dev,before.ino,before.mode,before.size,before.mtimeNs]);if(row.kind==="symlink")expect(await realpath(source)).toBe(row.matches?target:other);else if(row.kind==="file")expect(await readFile(source,"utf8")).toBe("unchanged");
 }
 expect(()=>observeJavascriptDependencyBindingsReadV1(context,[{sourceDirectory:join(root,"missing"),targetDirectory:target}],control)).toThrow("ENOENT");
 }finally{db.close();await rm(root,{recursive:true,force:true});}
}));

test("binding diagnostics preserve Unicode and satisfy independent SQLite path and encoded bounds",()=>{const db=new Database(":memory:");try{const path="🧬".repeat(corpus.unicode.repetitions)+"\n"+String.fromCharCode(0),message=javascriptDependencyBindingDiagnosticV1({sourceDirectory:path,expectedTarget:path,observedTarget:path,kind:"symlink"}),value=JSON.parse(message.slice(schema.$defs.Policy.const.prefix.length)),expected=db.query<{path:string},[string,number]>("select substr(?,1,?) path").get(path,corpus.unicode.expectedPoints)!.path;expect(new Ajv({strict:true}).validate(schema,value)).toBe(true);expect(value.sourceDirectory).toBe(expected);expect(value.expectedTarget).toBe(expected);expect(value.observedTarget).toBe(expected);expect(value.truncated).toBe(true);expect(message).not.toContain(String.fromCharCode(0));expect(Buffer.byteLength(message)).toBeLessThanOrEqual(schema.$defs.Policy.const.maximumMessageBytes);}finally{db.close();}});
