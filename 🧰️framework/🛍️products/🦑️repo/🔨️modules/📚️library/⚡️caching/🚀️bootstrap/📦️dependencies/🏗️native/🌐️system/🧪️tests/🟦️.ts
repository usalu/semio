import {expect,test} from "bun:test";
import Ajv from "ajv";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {delimiter,join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import policy from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

/** 🌐️ Makes the canonical browser gate own required system provisioning. */
test("canonical browser gate awaits system prerequisites before download",()=>{
 const native=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8");
 const shell=readFileSync(new URL("../../../../../../../🔩️native/🥾️bootstrap/🐚️.sh",import.meta.url),"utf8");
 expect(native).toContain("await prepareBrowserDependenciesV1(");
 expect(shell).not.toContain("ensure_browser_libraries");
 expect(shell).not.toContain("linux_browser_packages");
});

/** 🧪️ Compares exact neutral commands and failures with independent JSON Schema validation. */
test("browser system package policy is closed and agrees with Ajv",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(schema,policy)).toBe(true);
 expect(validateJsonSchemaSubset(schema,policy)).toEqual([]);
 for(const input of [{...policy,extra:true},{...policy,version:2},{...policy,order:["dnf"]},{...policy,packages:[policy.packages[0],policy.packages[0],policy.packages[0]]},{...policy,packages:policy.packages.map(row=>({...row,arguments:["install"]}))}]){
  expect(ajv.validate(schema,input)).toBe(false);
  expect(validateJsonSchemaSubset(schema,input).length).toBeGreaterThan(0);
 }
});
for(const row of fixture.cases)test("browser provisioning "+row.name,async()=>{
 expect(new Ajv({strict:true}).validate(schema.definitions.Case,row)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.Case,row)).toEqual([]);
 const {prepareBrowserDependenciesV1}=await import("../🟦️.ts");
 const controller=new AbortController(),reason=Error("canceled"),failure=Error("installer status 7");
 const calls:{command:string;args:string[];env:Record<string,string>}[]=[];
 const base={ORIGINAL:"same"},port={platform:row.platform,root:row.root,resolve:(name:string)=>row.available.includes(name)?"/tools/"+name:null,isFile:()=>row.cli};
 if(row.cancel==="before")controller.abort(reason);
 let error:unknown;
 try{await prepareBrowserDependenciesV1(port,async(command,args,environment)=>{
  expect(controller.signal.aborted).toBe(false);
  expect(environment.ORIGINAL).toBe("same");
  const env:Record<string,string>={};if(environment.DEBIAN_FRONTEND)env.DEBIAN_FRONTEND=environment.DEBIAN_FRONTEND;
  calls.push({command,args:[...args],env});
  if(row.cancel==="during"){controller.abort(reason);throw reason;}
  if(row.cancel==="return")controller.abort(reason);
  if(row.fail)throw failure;
 },"/tools/runtime","/source/cli",base,controller.signal,row.offline);}
 catch(caught){error=caught;}
 expect<unknown>(calls).toEqual(row.expected);
 expect(new Ajv({strict:true}).validate({const:row.expected},calls)).toBe(true);
 expect(validateJsonSchemaSubset({const:row.expected},calls)).toEqual([]);
 if(row.error){expect(String(error)).toContain(row.error);if(row.fail)expect(error).toBe(failure);if(row.cancel!=="none")expect(error).toBe(reason);}
 else expect(error).toBeUndefined();
});

/** 🔎️ Checks the real system adapter against filesystem executable permission and explicit PATH resolution. */
test("browser system host port resolves executable files from the caller environment",async()=>{
 const {browserSystemPortV1}=await import("../🟦️.ts");
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Caller test artifacts required");
 const root=mkdtempSync(join(resolve(artifacts),"browser-port-")),first=join(root,"a"),second=join(root,"b");
 try{
  mkdirSync(first);mkdirSync(second);
  writeFileSync(join(first,"manager"),"fixture");writeFileSync(join(second,"manager"),"fixture");
  chmodSync(join(first,"manager"),0o600);chmodSync(join(second,"manager"),0o700);
  const port=browserSystemPortV1({PATH:["relative",first,second].join(delimiter)});
  expect(port.platform).toBe(process.platform);
  expect(port.root).toBe(process.getuid?.()===0);
  expect(port.isFile(join(first,"manager"))).toBe(true);
  expect(port.isFile(first)).toBe(false);
  expect(port.isFile(join(root,"missing"))).toBe(false);
  expect(port.resolve("missing")).toBeNull();
  expect(port.resolve("manager")).toBe(join(process.platform==="win32"?first:second,"manager"));
  expect(browserSystemPortV1({}).resolve("manager")).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});}
});
