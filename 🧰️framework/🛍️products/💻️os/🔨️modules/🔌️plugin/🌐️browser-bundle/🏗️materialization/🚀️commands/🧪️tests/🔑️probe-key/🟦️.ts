import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{PLUGIN_DESCRIPTOR_PROBE_SOURCE}from"../../../../🛂️descriptor/🟦️.ts";
import{probeStageRequestV1}from"../../🟦️.ts";
/** 🔑️ The probe stage key follows the probe source: a changed source gives another key, the same source the same key. */
test("the probe stage cache key includes the sha256 of the probe source",()=>{
 const inputs={"a.js":"x"},first=probeStageRequestV1(inputs,"v22.0.0"),again=probeStageRequestV1(inputs,"v22.0.0"),other=probeStageRequestV1(inputs,"v22.0.0",PLUGIN_DESCRIPTOR_PROBE_SOURCE+"\n// changed");
 expect(first.tools.probe).toBe(createHash("sha256").update(PLUGIN_DESCRIPTOR_PROBE_SOURCE).digest("hex"));
 expect(again).toEqual(first);
 expect(JSON.stringify(other)).not.toBe(JSON.stringify(first));
 expect(first.schemaVersion).toBeGreaterThan(1);
});
