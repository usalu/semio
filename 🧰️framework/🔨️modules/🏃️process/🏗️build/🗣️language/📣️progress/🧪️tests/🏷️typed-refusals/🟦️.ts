/** 🏷️ Law: every typed refusal class of the boundary list renders as its own text, in the chosen language, with its exit status and no internal-error text, also inside aggregated faults and causes; only an unexpected fault is an internal error, and it carries a stable reference. */
import {expect,test} from "bun:test";
import {TYPED_REFUSAL_CLASSES_V1,faultReferenceV1,reportBoundaryErrorV1} from "../../🟦️.ts";
import source from "../../🟦️.ts" with {type:"text"};
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {CargoLockRefusedV1} from "../../../../../🦀️cargo-workspace/🔒️lock/🟦️.ts";
import {PhysicalLockErrorV1} from "../../../../../🔒️leases/📁️physical/🟦️.ts";
import {ResourceLeaseTimeoutErrorV1} from "../../../../../🔒️leases/🟦️.ts";
import {ToolHookTimeoutV1} from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⏱️hook-bound/🟦️.ts";
import {NxRecipeRefusedV1} from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🟦️.ts";
import {refuseProcessV1} from "../../../../../📖️texts/🟦️.ts";
import {ScriptProcessInterruptedV1} from "../../../../../🛑️interruption/🟦️.ts";

const INTERNAL=/Internal error|Interner Fehler|reference|Referenz/u;
const windows=()=>Object.assign(new Error("The folder could not be moved"),{name:"WindowsRenameRefused",texts:{en:"The folder C:\\a could not be moved to C:\\b.",de:"Der Ordner C:\\a konnte nicht nach C:\\b verschoben werden."}});
const sealed=(language:"en"|"de")=>Object.assign(Error(language==="en"?"This machine holds no sealed generation of the whole workspace.":"Diese Maschine hält keine versiegelte Generation des ganzen Workspace."),{code:"GENERATION_NONE_SEALED"});
const unknown=()=>Object.assign(Error("Unknown project in the closure: zz"),{code:"GENERATION_UNKNOWN_OWNER"});

type Sample=Readonly<{make:(language:"en"|"de")=>Error;text:(language:"en"|"de")=>string}>;
const samples:Readonly<Record<string,Sample>>={
 "cargo-lock":{make:()=>new CargoLockRefusedV1("en",null,{en:"Short resolver refusal",de:"Kurze Resolver-Ablehnung"},{version:1,status:101,stderr:"",stdout:"",truncated:false}) as unknown as Error,text:language=>language==="en"?"Short resolver refusal":"Kurze Resolver-Ablehnung"},
 "nx-recipe":{make:()=>new NxRecipeRefusedV1(["/type is not allowed"]) as unknown as Error,text:language=>new NxRecipeRefusedV1(["/type is not allowed"]).texts[language]},
 "tool-hook-timeout":{make:()=>new ToolHookTimeoutV1("tectonic","verify","C:\\semio\\script.ts",300000) as unknown as Error,text:language=>new ToolHookTimeoutV1("tectonic","verify","C:\\semio\\script.ts",300000).texts[language]},
 "windows-rename":{make:windows,text:language=>windows().texts[language]},
 "physical-lock":{make:()=>new PhysicalLockErrorV1("identity"),text:language=>new PhysicalLockErrorV1("identity").texts[language]},
 "lease-timeout":{make:()=>new ResourceLeaseTimeoutErrorV1({directory:"/nowhere/leases",resource:"native-storage:abc",mode:"exclusive"},62000) as unknown as Error,text:language=>new ResourceLeaseTimeoutErrorV1({directory:"/nowhere/leases",resource:"native-storage:abc",mode:"exclusive"},62000).texts[language]},
 "generation-none-sealed":{make:sealed,text:language=>sealed(language).message},
 "generation-unknown-owner":{make:unknown,text:()=>unknown().message}
};

function report(error:unknown,language:"en"|"de"){const lines:string[]=[];const codes:number[]=[];reportBoundaryErrorV1(error,{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>codes.push(code)});return{lines,codes};}
const wrap=(inner:unknown)=>new AggregateError([new Error("drain failed"),new Error("outer",{cause:inner})],"settlement failed");

test("the list has a sample for every class and no class without one",()=>{
 expect(Object.keys(samples).sort()).toEqual(TYPED_REFUSAL_CLASSES_V1.map(kind=>kind.id).sort());
 expect(TYPED_REFUSAL_CLASSES_V1).toEqual(schema.definitions.TypedRefusals.const as never);
 for(const kind of TYPED_REFUSAL_CLASSES_V1){expect([kind.name,kind.code].filter(value=>value!==undefined)).toHaveLength(1);expect(["texts","message"]).toContain(kind.text);}
});

for(const kind of TYPED_REFUSAL_CLASSES_V1){
 test(`class ${kind.id}: its own text in both languages, its exit status, no internal error, found in aggregates and causes`,()=>{
  for(const language of ["en","de"] as const){
   const sample=samples[kind.id]!;
   for(const thrown of [sample.make(language),wrap(sample.make(language)),new Error("outer",{cause:sample.make(language)})]){
    const row=report(thrown,language);
    expect(row.lines).toEqual([sample.text(language)]);
    expect(row.codes).toEqual([kind.exit]);
    expect(row.lines.join("\n")).not.toMatch(INTERNAL);
   }
  }
 });
}

test("the other typed refusals of the boundary render without the internal-error text too: catalog refusal, failed child, disk full, expired loan, cancellation",()=>{
 const refusal=refuseProcessV1("usage","execution.child-failed",{label:"x",status:3});
 for(const language of ["en","de"] as const){
  const rows=[report(refusal,language),report(Object.assign(new Error("No space left on device"),{code:"ENOSPC"}),language),report(Object.assign(new Error("loan"),{name:"ResourceLoanDeadlineExhausted"}),language),report(new ScriptProcessInterruptedV1(),language)];
  for(const row of rows)expect(row.lines.join("\n")).not.toMatch(INTERNAL);
  expect(rows[0]!.codes).toEqual([2]);expect(rows[3]!.codes).toEqual([130]);expect(rows[3]!.lines).toEqual([]);
 }
});

test("an unexpected fault is the only internal error: it names its message and a reference that is stable and differs between faults",()=>{
 for(const [language,prefix,word] of [["en","Internal error","reference"],["de","Interner Fehler","Referenz"]] as const){
  const row=report(new Error("boom"),language);
  expect(row.lines).toEqual([`${prefix}: boom (${word} f5cdfe79)`]);expect(row.codes).toEqual([1]);
  expect(report(new AggregateError([new Error("one"),new Error("two")],"settlement failed"),language).lines).toEqual([`${prefix}: settlement failed: one; two (${word} 8255cc34)`]);
 }
 expect(faultReferenceV1(new Error("boom"))).toBe(faultReferenceV1(new Error("boom")));
 expect(faultReferenceV1(new Error("boom"))).not.toBe(faultReferenceV1(new Error("bang")));
 expect(faultReferenceV1(new TypeError("x is not a function"))).toBe("c9ebeb38");
 expect(report(new Error("boom"),"en").lines.join("\n")).not.toMatch(/\n\s+at |Bun v/u);
});

test("the boundary names no refusal class itself: the list is the only place",()=>{
 for(const kind of TYPED_REFUSAL_CLASSES_V1)for(const literal of [kind.name,kind.code])if(literal!==undefined)expect(source).not.toContain(literal);
});
