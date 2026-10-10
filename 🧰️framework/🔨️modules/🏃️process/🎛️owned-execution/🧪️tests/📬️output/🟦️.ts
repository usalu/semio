import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {runOwnedCommand,type OwnedCommandOptions} from "../../🟦️.ts";
import domain from "../../🧬️schema/📬️output.json";
import fixture from "../../🧫️fixtures/📬️output/🔣️.json";

const oracle='const r=JSON.parse(process.argv[1]),a=Buffer.concat(r.stdout.map(x=>Buffer.from(x,"hex"))),b=Buffer.concat(r.stderr.map(x=>Buffer.from(x,"hex"))),lines=x=>Array.from(x).filter(v=>v===10).length+(x.length&&x[x.length-1]!==10?1:0);process.stdout.write(JSON.stringify({bytes:a.length+b.length,lines:lines(a)+lines(b),values:[...a.toString("utf8").split("\\n").filter((x,i,all)=>x.length||i<all.length-1),...b.toString("utf8").split("\\n").filter((x,i,all)=>x.length||i<all.length-1)]}));';
const child='const fs=require("node:fs"),r=JSON.parse(process.argv[1]);fs.writeFileSync(process.argv[2],String(process.pid));(async()=>{for(const [fd,chunks] of [[1,r.stdout],[2,r.stderr]])for(const chunk of chunks){fs.writeSync(fd,Buffer.from(chunk,"hex"));await new Promise(done=>setImmediate(done));}if(!r.accepted)setInterval(()=>{},10);})();';
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Caller-owned ticket output required");mkdirSync(output,{recursive:true});const storage=mkdtempSync(join(output,"owned-output-"));
const admit=new Ajv({strict:true}).compile(domain);
for(const row of fixture.cases)test("owned output "+row.id,async()=>{
 expect(admit(row.limits)).toBe(true);const expected=Bun.spawnSync(["node","-e",oracle,"--",JSON.stringify(row)]);expect(expected.exitCode).toBe(0);const independent=JSON.parse(new TextDecoder().decode(expected.stdout));expect(independent.bytes<=row.limits.maximumBytes&&independent.lines<=row.limits.maximumLines).toBe(row.accepted);
 const marker=join(storage,row.id+".pid"),observed:string[]=[],options={output:row.limits,stdout:row.mode??"inherit",...(row.observe?{onLine:(line:string)=>observed.push(line)}:{})} as OwnedCommandOptions;
 let failure:unknown;try{await runOwnedCommand("node",["-e",child,"--",JSON.stringify(row),marker],storage,"owned-output",fixture.policy.childMilliseconds,options);}catch(error){failure=error;}
 expect(existsSync(marker)).toBe(true);const pid=Number(readFileSync(marker,"utf8"));expect(()=>process.kill(pid,0)).toThrow();
 if(row.accepted){expect(failure).toBeUndefined();if(row.observe)expect(observed.sort()).toEqual(independent.values.sort());}else{expect(String(failure)).toContain("output");expect(String(failure)).not.toContain("timeout");}
 console.error("[DEBUG] Actual owned stream "+row.id+" bytes="+independent.bytes+" lines="+independent.lines+" accepted="+row.accepted+" original child closed, oracle=Node Buffer");
},fixture.policy.maximumElapsedMilliseconds);
test("owned output refuses foreign limits before acquiring a child",async()=>{
 for(const limits of fixture.invalidLimits){expect(admit(limits)).toBe(false);const marker=join(storage,"unacquired.pid");await expect(runOwnedCommand("node",["-e",'require("node:fs").writeFileSync(process.argv[1],"acquired")',"--",marker],storage,"owned-output",fixture.policy.childMilliseconds,{output:limits} as OwnedCommandOptions)).rejects.toThrow();expect(existsSync(marker)).toBe(false);}
 console.error("[DEBUG] Actual output domain independently admitted by Ajv; malformed limits refused before child acquisition");
});
