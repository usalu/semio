/** 🔬️ Law: with the stage variable an internal fault also writes its cause chain with stack frames before the internal-error line; the ready supervisor sets the variable and mirrors that line whole; without the variable no stack is printed. */
import {expect,test} from "bun:test";
import {faultDetailLineV1,faultReferenceV1,reportBoundaryErrorV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json"with{type:"json"};
import ready from "../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🧬️schema/🔣️.json"with{type:"json"};

const report=(error:unknown,environment:Record<string,string>)=>{const lines:string[]=[];reportBoundaryErrorV1(error,{environment:{SEMIO_LANGUAGE:"en",...environment},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:()=>{}});return lines;};
const fault=()=>new Error("outer",{cause:new AggregateError([new Error("first"),new RangeError("second")],"inner")});

test("the stage variable adds the cause chain before the internal error and nothing else prints a stack",()=>{
 const policy=schema.definitions.FaultDetail.const,on=report(fault(),{[policy.environment]:policy.enabled}),off=report(fault(),{});
 expect(off).toHaveLength(1);
 expect(off[0]).toContain("Internal error");
 expect(off.join("\n")).not.toContain(" at ");
 expect(on).toHaveLength(2);
 expect(on[0]!.startsWith(`${policy.prefix}${faultReferenceV1(fault())} `)).toBe(true);
 const chain=JSON.parse(on[0]!.slice(policy.prefix.length+9)) as {name:string;message:string;stack:string[]}[];
 expect(chain.map(row=>row.message)).toEqual(["outer","inner","first","second"]);
 expect(chain[3]!.name).toBe("RangeError");
 expect(chain[0]!.stack.length).toBeGreaterThan(0);
 expect(on[1]).toContain("Internal error");
 expect(on[0]!.length).toBeLessThanOrEqual(policy.maximumCharacters);
 expect(faultDetailLineV1(fault()).startsWith(`${policy.prefix}${faultReferenceV1(fault())} `)).toBe(true);
});

test("the ready supervisor sets the variable and mirrors the line whole",()=>{
 const policy=schema.definitions.FaultDetail.const,stage=ready.definitions.Policy.const,line=`${policy.prefix}29072ae7 []`;
 expect(stage.faultDetailEnvironment).toBe(policy.environment);
 expect(stage.faultDetailEnabled).toBe(policy.enabled);
 for(const patterns of [stage.mirrorErrorPatterns,stage.longRefusalPatterns])expect(patterns.some(pattern=>new RegExp(pattern,"u").test(line))).toBe(true);
});
