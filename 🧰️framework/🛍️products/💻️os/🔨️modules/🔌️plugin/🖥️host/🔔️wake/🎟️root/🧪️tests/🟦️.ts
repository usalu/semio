import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import { turnAdmissionReadyFixtureUrl, turnAdmissionReadySchemaUrl } from "@semio-tech/framework-actor/testing/fixtures";
/** 🎟️ An outstanding guest receipt retains its exact initiating scope after expiry or cancellation. */
test("original Host root requires explicit policy and preserves the exact suspended receipt scope",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:true}).compile(schema)(law.policy)).toBe(true);for(const key of Object.keys(law.policy)){const missing={...law.policy};delete missing[key];expect(new Ajv({strict:true}).compile(schema)(missing)).toBe(false)}
 const db=new Database(":memory:");try{expect(db.query("SELECT ?1-?2 AS fuel,?3-?4 AS items,?5-?6 AS copy").get(law.scope.fuel,law.receipt.work+law.cancellationSpent.work,law.scope.grant[0],law.receipt.spent[0]+law.cancellationSpent.items,law.scope.grant[1],law.receipt.spent[1])).toEqual({fuel:law.expected.fuel,items:law.expected.grant[0],copy:law.expected.grant[1]});for(const same of [0,1])for(const renewed of [0,1])expect((db.query("SELECT (?1=1 AND ?2=0) AS admitted").get(same,renewed)as{admitted:number}).admitted).toBe(same&&!renewed?1:0)}finally{db.close()}
 expect(law.policy.turnExecutionFuel).not.toBe(law.policy.turnFuel);const dbExecution=new Database(":memory:");try{expect(dbExecution.query("SELECT ?1-?2 AS executionFuel, ?3-?4 AS logicalFuel").get(law.policy.turnExecutionFuel,law.receipt.executionSpent,law.scope.fuel,law.receipt.work+law.cancellationSpent.work)).toEqual({executionFuel:law.expected.executionFuel,logicalFuel:law.expected.fuel})}finally{dbExecution.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("OriginalActorContext");expect(source).toContain("borrow_pending_context(");expect(source).toContain("dispatch_pending_step");expect(source).toContain("pending_budget");expect(source).toContain("finish_turn");expect(source).not.toContain("StepContext::new");expect(source).not.toContain("root_cancel_token");expect(source).not.toContain("default_now_us");
});

/** 📤️ A ready original input transfers only after all guest currency has returned and its actual copy is paid. */
test("original admitted Actor input requires ready ownership and paid final move",async()=>{
 const law=await Bun.file(turnAdmissionReadyFixtureUrl).json(),schema=await Bun.file(turnAdmissionReadySchemaUrl).json();expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);const db=new Database(":memory:");try{for(const row of law.cuts){const taken=(db.query("SELECT (?1>=?2 AND ?3=1 AND ?4=0 AND ?5=0 AND ?6=0) AS taken").get(row.copyBytes,law.inputBytes,Number(row.ready),Number(row.issued),Number(row.guest),Number(row.refused))as{taken:number}).taken;expect(Boolean(taken)).toBe(row.expectedTaken)}}finally{db.close()}
 const source=readFileSync(new URL("../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/🦀️.rs",import.meta.url),"utf8");expect(source).toContain("take_admitted_input_step");expect(source).toContain("size_of::<T>()");expect(source).toContain("original_source_epoch()");
});

/** ⏳️ A later bounded callback retains the same source deadline after a narrower slice expires. */
test("original startup slice expiry preserves its immutable source deadline",async()=>{
 const law=await Bun.file(new URL("🧫️fixtures/🔣️.json",import.meta.url)).json(),row=law.startupDeadline;
 const db=new Database(":memory:");
 try {
  expect(db.query("SELECT ?1+?2 AS original, (?3>=?4) AS sliceExpired, (?3<?5) AS sourceLive, (?6=?5) AS unchanged").get(row.initiatingNow,law.policy.maximumTurnMicroseconds,row.expiredNow,row.initiatingDeadline,row.originalDeadline,row.nextRootDeadline)).toEqual({original:1500,sliceExpired:1,sourceLive:1,unchanged:1});
  expect(row.expiredVmSpent).toBe(0);expect(row.nextVmSpent).toBe(2);
 }finally{db.close()}
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");
 expect(source).toContain("absolute_deadline_us:self.absolute_deadline_us");
 expect(source).toContain("dispatch_actor_cleanup_step");
});
