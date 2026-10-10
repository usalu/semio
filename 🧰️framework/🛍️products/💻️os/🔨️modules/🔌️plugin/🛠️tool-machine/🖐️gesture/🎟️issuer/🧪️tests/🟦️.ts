import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🔣️.json";
import {gestureIssuanceReady} from "../🟦️.ts";
const oracle=new Ajv().compile(schema);
const cancellationOracle=new Ajv().compile(schema.definitions.cancellationIntent);
test("original gesture admission matches neutral grant oracle",()=>{for(const row of fixture.cases){expect(oracle(row)?"admitted":"pending",row.id).toBe(row.expected);expect(gestureIssuanceReady(row),row.id).toBe(oracle(row));}});
test("cancellation intent preserves pending and already settled originals",()=>{
 const issuer=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");
 const method=issuer.slice(issuer.indexOf("pub fn request_cancellation"),issuer.indexOf("pub(crate) fn reserve_live"));
 expect(method.includes("self.settled=true")).toBe(true);expect(method.includes("self.slot")).toBe(false);expect(method.includes("self.decision")).toBe(false);
 for(const row of fixture.cancellationCases){expect(cancellationOracle(row),row.id).toBe(true);expect(row.expectedSettled,row.id).toBe(true);}
});
test("mounted gesture keeps actual issuer and separately funded maintenance",()=>{
 const machine=readFileSync(resolve(import.meta.dir,"../../../🦀️.rs"),"utf8");
 const plugin=readFileSync(resolve(import.meta.dir,"../../../../🦀️.rs"),"utf8");
 const fields=readFileSync(resolve(import.meta.dir,"../../../../🧵️retained-command/🧬️context/📸️fields/🦀️.rs"),"utf8");
 for(const token of["GestureCapture","GestureOperation","gesture_retirement_demand","gesture_retirement_step(grant)","request_gesture_cancellation","begin_gesture_retirement"])expect(machine.includes(token),token).toBe(true);
 for(const token of["gesture_operations.remove","Arc::new(GestureSlot"])expect(machine.includes(token),token).toBe(false);
 for(const token of["gesture: GestureCapture<A::Mutation>","gesture: GestureCapture::detached()"])expect(plugin.includes(token),token).toBe(true);
 expect(plugin.includes("gesture: std::sync::Arc<GestureSlot")).toBe(false);expect(fields.includes("pub gesture:GestureCapture<M>")).toBe(true);
});
