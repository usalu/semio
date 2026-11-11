import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {writeHostServiceLogV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
const command=readFileSync(new URL("../../../⌨️command/🌐️host/🟦️.ts",import.meta.url),"utf8");
test("the host service log line carries the event and its detail on stderr",()=>{
 const lines:string[]=[],write=process.stderr.write;
 process.stderr.write=((text:string)=>{lines.push(text);return true;}) as typeof process.stderr.write;
 try{writeHostServiceLogV1("plan-failed","abcd1234 Error: boom");writeHostServiceLogV1("ready");}finally{process.stderr.write=write;}
 expect(lines[0]).toMatch(/^\d{4}-\d\d-\d\dT[\d:.]+Z plan-failed abcd1234 Error: boom\n$/u);
 expect(lines[1]).toMatch(/Z ready\n$/u);
});
test("the resident host starts with that log, so plan-failed details reach the service log",()=>{
 expect(command).toContain("runHostForegroundV1({implementation,log:writeHostServiceLogV1,");
});
