/** 🪟️ Law: the authored recipe of the tools (its real `package.json`, also with the line endings an editor on Windows writes) is closed by the Recipe definition, so no field a peer adds can turn the capture of the tools into a refusal on a worker; a recipe or lock that is not closed is the typed bilingual refusal that names the offending field, never an internal error. Checked against ajv, an independent validator. */
import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {NxRecipeRefusedV1,nxDependencyInstallFilesV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import {reportBoundaryErrorV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";

const tools=join(import.meta.dir,"../../../../.."),recipeText=readFileSync(join(tools,"package.json"),"utf8"),lockText=readFileSync(join(tools,"bun.lock"),"utf8");
const recipe=JSON.parse(recipeText) as Record<string,unknown>,lock={workspaces:{"":{name:recipe.name}},packages:{}};
const ajvVerdict=(value:unknown)=>new Ajv({strict:false}).validate({...schema,$ref:"#/definitions/Recipe"},value);
const refusal=(value:unknown,locked:unknown=lock):NxRecipeRefusedV1|undefined=>{try{nxDependencyInstallFilesV1(value,locked);return undefined;}catch(error){return error as NxRecipeRefusedV1;}};
const render=(error:unknown,language:"en"|"de")=>{const lines:string[]=[],codes:number[]=[];reportBoundaryErrorV1(error,{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>codes.push(code)});return{lines,codes};};

test("the real recipe of the tools is closed: every field of its package.json is declared and ajv agrees",()=>{
 expect(existsSync(join(tools,"package.json"))).toBe(true);
 const declared=Object.keys(schema.definitions.Recipe.properties);
 expect(Object.keys(recipe).filter(name=>!declared.includes(name))).toEqual([]);
 expect(ajvVerdict(recipe)).toBe(true);
 expect(refusal(recipe)).toBeUndefined();
 expect(JSON.parse(lockText.replace(/,(\s*[}\]])/gu,"$1"))).toBeTypeOf("object");
});

test("a recipe written with Windows line endings projects the same inputs as the one with Unix line endings",()=>{
 const windows=JSON.parse(recipeText.replace(/\r?\n/gu,"\r\n")),unix=JSON.parse(recipeText);
 const a=nxDependencyInstallFilesV1(windows,lock),b=nxDependencyInstallFilesV1(unix,lock);
 expect([...a.keys()]).toEqual([...b.keys()]);
 for(const [name,bytes] of a)expect(bytes.equals(b.get(name)!)).toBe(true);
});

test("every field a peer may add is refused with a typed bilingual text that names it, and ajv refuses it too",()=>{
 for(const [name,value] of [["type","commonjs"],["scripts",{}],["version","1.0.0"],["bin","x"]] as const){
  const mutated={...recipe,[name]:value},error=refusal(mutated);
  expect(ajvVerdict(mutated),name).toBe(false);expect(error,name).toBeInstanceOf(NxRecipeRefusedV1);
  expect(error!.message).toBe("Closed pinned dependency recipe required");expect(error!.name).toBe("NxRecipeRefused");
  expect(error!.violations.some(row=>row.includes(name)),name).toBe(true);
  expect(error!.texts.en).toContain(name);expect(error!.texts.de).toContain(name);expect(error!.texts.en).not.toBe(error!.texts.de);
  for(const language of ["en","de"] as const){const rendered=render(error,language);expect(rendered.lines).toEqual([error!.texts[language]]);expect(rendered.codes).toEqual([1]);expect(rendered.lines.join("\n")).not.toMatch(/Internal error|Interner Fehler|reference|Referenz/u);}
 }
});

test("the type field the recipe carries today is the module marker and nothing else",()=>{
 expect(recipe.type).toBe("module");
 expect(refusal({...recipe,type:"module"})).toBeUndefined();
 expect(refusal({...recipe,type:"commonjs"})?.violations.join(" ")).toContain("type");
});

test("a lock that is no JSON object is the typed refusal of the lock, in both languages, and not the recipe text",()=>{
 for(const bad of [null,undefined,[],"x",1]){
  let error:NxRecipeRefusedV1|undefined;try{nxDependencyInstallFilesV1(recipe,bad);}catch(thrown){error=thrown as NxRecipeRefusedV1;}expect(error).toBeInstanceOf(NxRecipeRefusedV1);expect(error!.violations).toEqual([]);
  expect(error!.texts.en).toContain("bun.lock");expect(error!.texts.de).toContain("bun.lock");
  expect(render(error,"de").lines).toEqual([error!.texts.de]);
 }
});
