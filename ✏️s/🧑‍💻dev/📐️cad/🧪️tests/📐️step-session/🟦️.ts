/** 📐️ Portable STEP round-trip and close authority exercised through the actual outward browser Wasm. */
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/📐️step-session/🔣️.json";
import schema from "../../🧬️schema/📐️step-session/🔣️.json";
import { createStepGeometrySession } from "../../🏘️composition/📐️step/🟦️.ts";
export async function stepBrowserLaws():Promise<void> {
  assert(new Ajv({strict:true}).validate(schema,fixture));
  const session=createStepGeometrySession();
  const isolated=createStepGeometrySession();
  try {
    const prior=(await session.invoke<{handle:string}>("box",fixture.box)).handle;
    const exported=(await session.invoke<{value:string}>("exportStep",{shapes:[prior]})).value;
    assert(exported.includes("MANIFOLD_SOLID_BREP"));
    const imported=(await session.invoke<{handles:string[]}>("importStep",{data:exported})).handles;
    assert(imported.length);
    for (const shape of [prior,...imported]) assert(Math.abs((await session.invoke<{value:number}>("volume",{shape})).value-fixture.expectedVolume)<1e-6);
    await assert.rejects(isolated.invoke("volume",{shape:prior}));
    await assert.rejects(session.close({cancelled:()=>true}),/close-cancelled/);
    await assert.rejects(session.invoke("exportStep",{shapes:[prior]}),/session-closed/);
    let turns=0;
    await session.close({maximumItems:1,maximumBytes:1_048_576,onProgress:receipt=>{ assert(receipt.items<=1); turns++; }});
    assert(turns>1);
    console.log("[DEBUG] outward STEP browser: exact volume round-trip, prior handles, independent authority, cancelled close/resume and bounded retirement passed");
  } finally { await Promise.all([session.close(),isolated.close()]); }
}
