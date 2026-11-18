/** 🪟️ A typed bilingual refusal that settlement code wrapped in aggregated faults and causes is rendered as itself at the boundary, never as an internal error. */
import {expect,test} from "bun:test";
import {reportBoundaryErrorV1} from "../../🟦️.ts";

const refusal=()=>Object.assign(new Error("The folder could not be moved"),{name:"WindowsRenameRefused",texts:{en:"The folder C:\\a could not be moved to C:\\b.",de:"Der Ordner C:\\a konnte nicht nach C:\\b verschoben werden."}});
const wrap=(inner:unknown)=>new AggregateError([new AggregateError([new AggregateError([inner,new Error("completed input validation failed")],"Producer reader and completed input validation failed")],"Protected maintenance and accepted drainage failed")],"Original physical body, drain and cleanup settlement failed");

function report(error:unknown,language:"en"|"de"){const lines:string[]=[];let status=-1;reportBoundaryErrorV1(error,{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>{status=code;}});return{lines,status};}

test("the refusal inside the production settlement chain is rendered in the chosen language with exit status 1",()=>{
 for(const language of ["en","de"] as const){const row=report(wrap(refusal()),language);expect(row.lines).toEqual([refusal().texts[language]]);expect(row.status).toBe(1);expect(row.lines.join("\n")).not.toMatch(/Internal error|Interner Fehler/u);}
});

test("the refusal as the cause of a plain error is found too, and a deep chain beyond the bound is not searched forever",()=>{
 const caused=new Error("outer",{cause:refusal()});expect(report(caused,"en").lines).toEqual([refusal().texts.en]);
 let deep:unknown=refusal();for(let level=0;level<20;level++)deep=new Error("level "+level,{cause:deep});
 expect(report(deep,"en").lines.join("\n")).toMatch(/Internal error/u);
});

test("an aggregate without any typed refusal stays an internal error naming every fault",()=>{
 const row=report(new AggregateError([new Error("one"),new Error("two")],"settlement failed"),"en");
 expect(row.lines.join("\n")).toContain("Internal error: settlement failed: one; two");expect(row.status).toBe(1);
});
