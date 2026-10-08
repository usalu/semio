/** 🌐️ Portable geometry lifetime laws exercised against the actual browser Wasm instance. */
import { strict as assert } from "node:assert";
import fixture from "../../🧫️fixtures/🏷️session-lifetime/🔣️.json";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import retirementFixture from "../../🧫️fixtures/🧹️retirement/🔣️.json";
import { SemioGeometrySession } from "../../🟦️.ts";
export async function browserSessionLaws(): Promise<void> {
  const validate = new Ajv({ strict:true,allErrors:true }).addKeyword("x-semio-formats").addSchema(schema).getSchema(`${schema.$id}#/$defs/SemioGeometryCloseReceiptV1`)!;
  const sessions = [new SemioGeometrySession(), new SemioGeometrySession()];
  let failure:unknown;
  try {
    for (const method of ["exportStep","importStep"]) await assert.rejects(sessions[0].invoke(method,{}),/unknown/);
    for (const [index, session] of sessions.entries()) {
      const box = fixture.independentBoxes[index];
      const { handle } = await session.invoke<{ handle: string }>("box", box);
      const { value } = await session.invoke<{ value: number }>("volume", { shape:handle });
      assert.equal(value, box.volume);
    }
    await assert.rejects(sessions[0].close({ cancelled:() => true }),/geometry.close-cancelled/);
    await assert.rejects(sessions[0].invoke("box",fixture.independentBoxes[0]),/geometry.session-closed/);
    let turns = 0;
    await sessions[0].close({ maximumItems:retirementFixture.closeGrant.items, onProgress:receipt => {
      assert(validate(receipt),JSON.stringify(validate.errors));
      turns += 1; assert(receipt.items <= retirementFixture.closeGrant.items);assert(receipt.copyBytes>=0 && receipt.capacityBytes>=0 && receipt.releaseBytes>=0);
    } });
    assert(turns > 1);
    await sessions[0].close();
    await assert.rejects(sessions[0].invoke("box", fixture.independentBoxes[0]), /geometry.session-closed/);
    const { handle } = await sessions[1].invoke<{ handle: string }>("box", fixture.independentBoxes[1]);
    assert.equal((await sessions[1].invoke<{ value: number }>("volume", { shape:handle })).value, fixture.independentBoxes[1].volume);
    const { BrowserSession } = await import("../../📦️packages/🦀️rust/🕸️bindings/semio_session.js");
    const native = new BrowserSession();
    const created = JSON.parse(native.brep_invoke("box",JSON.stringify(retirementFixture.box))) as { handle:string };
    for (const grant of retirementFixture.grants.slice(0,2)) {
      const receipt = JSON.parse(native.close_step(grant.items,grant.copyBytes,grant.capacityBytes,grant.releaseBytes,grant.depth));
      assert(validate(receipt),JSON.stringify(validate.errors));
      assert.deepEqual(receipt,{phase:"pending",items:0,copyBytes:0,capacityBytes:0,releaseBytes:0});
      assert.equal(JSON.parse(native.brep_invoke("volume",JSON.stringify({shape:created.handle}))).value,retirementFixture.box.volume);
    }
    native.begin_close();
    let closed = false;
    for (let turn = 0; turn < retirementFixture.maximumSteps; turn += 1) {
      const receipt = JSON.parse(native.close_step(1,native.next_close_copy_byte_demand(),native.next_close_capacity_byte_demand(native.next_close_copy_byte_demand()),native.next_close_release_byte_demand(),native.next_close_depth_demand()));
      assert(validate(receipt),JSON.stringify(validate.errors));
      assert(receipt.items<=1 && receipt.copyBytes>=0 && receipt.capacityBytes>=0 && receipt.releaseBytes>=0);
      if (receipt.phase === "complete") { closed = true; break; }
    }
    assert(closed && native.terminal_is_empty()); native.free();
    console.log("Semio geometry browser: two portable volumes, independent lifetime, bounded close receipts, cancellation/resume, zero-grant authority preservation, repeated close and terminal refusal passed");
  } catch (error) { failure = error; }
  finally {
    const outcomes = await Promise.allSettled(sessions.map(session => session.close()));
    if (failure) throw failure;
    const refused = outcomes.find(outcome => outcome.status === "rejected");
    if (refused?.status === "rejected") throw refused.reason;
  }
}
