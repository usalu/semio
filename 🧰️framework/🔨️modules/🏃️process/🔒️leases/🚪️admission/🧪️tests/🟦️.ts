import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,acquireResourceLease,claimSweepDueV1,claimSweepIntervalV1,leaseStoreFreeRatioV1,resourceLeasePathV1} from "../../🟦️.ts";
import corpus from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import authored from "../../🧭️order/🔣️policy.json";
import ticketSchema from "../../🧬️schema/📜️ticket.json";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"la-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const admissionResource=ticketSchema.definitions.RetirementPolicy.properties.admissionResource.const;
const validate=new Ajv({strict:true,allowUnionTypes:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Corpus"} as never);

test("the admission corpus satisfies its closed schema in an independent validator",()=>{
 expect(validate(corpus)).toBe(true);expect(validate({...corpus,extra:1})).toBe(false);
 expect(validate({...corpus,sweep:[{...corpus.sweep[0],calls:[{atMs:0,timeoutMs:-1,expect:true}]}]})).toBe(false);
});

for(const row of corpus.sweep)test(`claim sweep cadence: ${row.id}`,()=>{
 const sweeps=new Map<string,number>();
 for(const call of row.calls)expect(`${call.atMs}:${claimSweepDueV1(sweeps,"/lease",call.atMs,row.intervalMilliseconds,call.timeoutMs??undefined)}`).toBe(`${call.atMs}:${call.expect}`);
});

const oracle=new Database(":memory:");
afterAll(()=>oracle.close());
for(const row of corpus.pressure)test(`claim sweep pressure: ${row.id}`,()=>{
 const expected=oracle.query("select case when ?1<?2 then min(?3,?4) else ?3 end as interval").get(row.freeRatio,row.gate.claimSweepPressureFreeRatio,row.gate.claimSweepIntervalMilliseconds,row.gate.claimSweepPressureIntervalMilliseconds) as {interval:number};
 expect(claimSweepIntervalV1(row.gate,row.freeRatio)).toBe(expected.interval);expect(expected.interval).toBe(row.expectIntervalMilliseconds);
});
test("the free ratio of a real lease store is a fraction and an absent store counts as free",()=>{
 const ratio=leaseStoreFreeRatioV1(root);expect(ratio).toBeGreaterThan(0);expect(ratio).toBeLessThanOrEqual(1);expect(leaseStoreFreeRatioV1(join(root,"absent"))).toBe(1);
});

let counter=0;
for(const row of corpus.gate)test(`admission gate: ${row.id}`,async()=>{
 const directory=join(root,`d${++counter}`),file=join(root,`p${counter}.json`);mkdirSync(directory,{recursive:true});
 writeFileSync(file,JSON.stringify({...authored,gate:{...authored.gate,admissionPatienceMilliseconds:row.patienceMilliseconds,claimSweepIntervalMilliseconds:0}}));
 process.env.SEMIO_LEASE_ORDER_POLICY=file;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,`r${counter}`);
 const signal=()=>new AbortController().signal;
 (await acquireResourceLease({directory,resource:"warm",mode:"shared",signal:signal()})).release();
 const database=new Database(resourceLeasePathV1(directory,admissionResource));database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");
 const timer=setTimeout(()=>{try{database.exec("ROLLBACK");}catch{}},row.gateHeldMilliseconds),started=Date.now();
 const options={directory,resource:"try",mode:"exclusive" as const,signal:signal(),timeoutMs:0};
 const outcome=await (row.api==="queued"?acquireQueuedResourceLease({...options,owner:"corpus"}):acquireResourceLease(options)).then(lease=>{lease.release();return "granted";},()=>"timed-out");
 const elapsed=Date.now()-started;
 clearTimeout(timer);try{database.exec("ROLLBACK");}catch{}database.close();
 expect(outcome).toBe(row.expect.outcome);expect(elapsed).toBeGreaterThanOrEqual(row.expect.minElapsedMilliseconds);expect(elapsed).toBeLessThanOrEqual(row.expect.maxElapsedMilliseconds);
});
