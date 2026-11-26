/** ⌛️ Contract: a rustc wrapper waits for the admission broker's headroom BEFORE it takes the single compile slot, with a bound, and never blocks when no broker answers.
 * @see ../../🛰️broker/🟦️.ts (`awaitBrokerHeadroomV1`)
 * @see ../../📜️script.ts (`compile`) */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {createServer,type Server} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import type {AdmitFactsV1} from "../../🔏️key/🟦️.ts";
import {awaitBrokerHeadroomV1} from "../../🛰️broker/🟦️.ts";

const roots:string[]=[],servers:Server[]=[];
afterAll(()=>{for(const server of servers)server.close();for(const root of roots)rmSync(root,{recursive:true,force:true});});
const facts={crate:"syn",package:"syn",version:"2.0.0",target:"host",crateTypes:["lib"],outDir:"/o",extraFilename:"-1",source:"/s/lib.rs",manifestDir:"/s",primary:false,buildScriptOutDir:null,externs:[]} as unknown as AdmitFactsV1;
const fakeBroker=async(answers:readonly (boolean|"silent"|"fleet-only")[])=>{
 const root=mkdtempSync(join(tmpdir(),"semio-headroom-")),endpoint=join(root,"b.sock"),seen:Record<string,unknown>[]=[];roots.push(root);
 const server=createServer(socket=>{let text="";socket.on("error",()=>undefined);socket.on("data",chunk=>{text+=chunk;if(!text.includes("\n"))return;seen.push(JSON.parse(text.split("\n")[0]!));text="";const answer=answers[Math.min(seen.length-1,answers.length-1)]!;if(answer==="silent")return;socket.end(JSON.stringify(answer==="fleet-only"?{v:1,op:"headroom",wait:true,fleetOnly:true}:{v:1,op:"headroom",wait:answer})+"\n");});});
 servers.push(server);
 await new Promise<void>(done=>server.listen(endpoint,done));
 return {environment:{SEMIO_BUILD_BROKER:endpoint,SEMIO_BUILD_PASS:"p1"},seen};
};

test("the wrapper polls the broker until it would admit the unit, and sends the unit's own facts",async()=>{
 const broker=await fakeBroker([true,true,false]);
 expect(await awaitBrokerHeadroomV1(broker.environment,facts,{pauseMilliseconds:10,maximumWaitMilliseconds:5000})).toBe("admit");
 expect(broker.seen).toHaveLength(3);
 expect(broker.seen[0]).toMatchObject({v:1,op:"probe",pass:"p1",crate:"syn",target:"host"});
});

test("a broker that always says wait is waited out at most the bound, then the caller goes on",async()=>{
 const broker=await fakeBroker([true]),started=Date.now();
 expect(await awaitBrokerHeadroomV1(broker.environment,facts,{pauseMilliseconds:20,maximumWaitMilliseconds:300})).toBe("timeout");
 expect(Date.now()-started).toBeLessThan(3000);
 expect(broker.seen.length).toBeGreaterThanOrEqual(2);
});

test("no broker in the environment, or one that does not answer a valid headroom, never blocks the wrapper",async()=>{
 expect(await awaitBrokerHeadroomV1({},facts)).toBe("none");
 expect(await awaitBrokerHeadroomV1({SEMIO_BUILD_BROKER:join(tmpdir(),"semio-no-such-broker.sock"),SEMIO_BUILD_PASS:"p1"},facts)).toBe("none");
});

test("a fleet-only pass has no headroom to wait for: the wrapper goes on at once",async()=>{
 const broker=await fakeBroker(["fleet-only"]);
 expect(await awaitBrokerHeadroomV1(broker.environment,facts,{pauseMilliseconds:10,maximumWaitMilliseconds:5000})).toBe("none");
 expect(broker.seen).toHaveLength(1);
});

test("the wrapper asks for headroom before it acquires the compile slot",()=>{
 const source=readFileSync(new URL("../../📜️script.ts",import.meta.url),"utf8");
 expect(source.indexOf("awaitBrokerHeadroomV1(environment,facts)")).toBeGreaterThan(0);
 expect(source.indexOf("awaitBrokerHeadroomV1(environment,facts)")).toBeLessThan(source.indexOf("slot=await acquireSlotV1("));
});
