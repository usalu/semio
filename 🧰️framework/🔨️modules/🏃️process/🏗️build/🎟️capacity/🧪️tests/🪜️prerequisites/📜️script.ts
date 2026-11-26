import { acquireCompilerCapacityLeaseV1, scriptCompilerAdmissionControlV1 } from "../../🟦️.ts";
import { createRequestedIssuedResourceControlV1, receiveScriptProcessInvocation } from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from "./🧫️cases.json" with { type: "json" };

const [directory, minimum] = process.argv.slice(2);
await receiveScriptProcessInvocation(process.env, async original => {
  const root = scriptCompilerAdmissionControlV1(original, "test:prerequisite", 30_000);
  const cancel = new AbortController();
  const timeout = setTimeout(() => cancel.abort(Error("Independent child could not enter")), fixture.runtime.childWaitMs);
  try {
    const control = createRequestedIssuedResourceControlV1(root, 0, cancel.signal);
    const lease = await acquireCompilerCapacityLeaseV1({ directory: directory!, control, environment: {},
      request: { kind: "exact", parallelism: Number(minimum) } });
    try { console.log(JSON.stringify({ slots: lease.parallelism })); } finally { lease.release(); }
  } finally { clearTimeout(timeout); }
});
