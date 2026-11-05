import { setTimeout as pause } from "node:timers/promises";
import { receiveScriptProcessInvocation, readIssuedScriptProcessEnvelopeV1 } from "../../../../🟦️.ts";
import corpus from "../🔣️.json";
await receiveScriptProcessInvocation(process.env, async invocation => {
  const first = readIssuedScriptProcessEnvelopeV1(invocation);
  console.log("issued-deadline:" + first.deadlineEpochMilliseconds);
  for (let pulse = 0; pulse < corpus.realChild.pulses; pulse++) {
    await pause(corpus.realChild.pulseMilliseconds, undefined, { signal: invocation.control.signal });
    console.log("work:" + pulse);
  }
  const last = readIssuedScriptProcessEnvelopeV1(invocation);
  if (last.deadlineEpochMilliseconds !== first.deadlineEpochMilliseconds) throw Error("A running child deadline must not change");
  console.log("issued-deadline:" + last.deadlineEpochMilliseconds);
});
