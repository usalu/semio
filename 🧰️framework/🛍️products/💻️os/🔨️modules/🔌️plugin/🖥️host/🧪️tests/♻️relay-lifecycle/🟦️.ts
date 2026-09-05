import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

type Expected = { state: string; firstReason: string | null; cancelAdmissions: number; releaseOpportunities: number; callerOutput: string | null };
type Accounting = { seedPagesBefore: number; seedPagesAfter: number; abiBytesBefore: number; abiBytesAfter: number };
type Trace = { "🪪️id": string; machine: "replay" | "relay"; generation: number; initial: string; events: string[]; expected: Expected; accounting: Accounting };
type IngressFrame = { actor: number; lane: "interactive" | "user-visible" | "background" | "maintenance"; sequence: number; fuel: number };
type IngressCase = { "🪪️id": string; frames: IngressFrame[]; expectedActors: number[]; maxFramesPerDrive: number };
type Fixture = { schemaVersion: number; capacities: { mountedRelaySlots: number; replaySeedSlots: number; replayRefusalSlots: number; pageBytes: number }; ingressCases: IngressCase[]; traces: Trace[] };

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/♻️relay-lifecycle.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(join(here, "../../🧫️fixtures/🧬️relay-lifecycle.schema.json"), "utf8"));
const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
if (!validate(fixture)) throw new Error(`relay lifecycle fixture schema violation: ${JSON.stringify(validate.errors)}`);

/** 🤖 Interprets one literal trace independently from the Rust ownership machinery. */
function interpret(trace: Trace): Expected {
  let state = trace.initial;
  let firstReason: string | null = null;
  let cancelAdmissions = 0;
  let releaseOpportunities = 0;
  let owned = trace.machine === "replay" ? 5 : 0;
  let callerOutput: string | null = null;
  let reserved = 0;
  for (const event of trace.events) {
    if (event.startsWith("fault:")) {
      firstReason ??= event;
      state = "Closing";
    } else if (event === "cancel" && trace.machine === "replay") {
      firstReason ??= "cancelled";
      state = "Closing";
    } else if (event.startsWith("drop:")) {
      if (Number(event.slice(5)) === trace.generation) {
        state = "DetachedForReap";
        firstReason ??= "detached";
        cancelAdmissions += 1;
        owned = 2;
      }
    } else if (event.startsWith("terminal:")) {
      state = "DrainingForCaller";
      firstReason ??= "caller";
      callerOutput = event.slice(9);
      owned = 1;
    } else if (event === "release" && (state === "Closing" || state === "DetachedForReap")) {
      releaseOpportunities += 1;
      owned -= 1;
      if (owned === 0) state = "Empty";
    } else if (event === "caller-release" && state === "DrainingForCaller") {
      releaseOpportunities += 1;
      owned -= 1;
      if (owned === 0) state = "Empty";
    } else if (event.startsWith("reserve:")) {
      const requested = Number(event.slice(8));
      if (reserved + requested > fixture.capacities.mountedRelaySlots) state = "CapacityRefused";
      else {
        reserved += requested;
        state = "ReservedFull";
      }
    }
  }
  return { state, firstReason, cancelAdmissions, releaseOpportunities, callerOutput };
}

/** 🎯 Validates one independent transition result and its balanced fixture accounting. */
function verify(trace: Trace): AdapterOutcome {
  const actual = interpret(trace);
  if (JSON.stringify(actual) !== JSON.stringify(trace.expected)) throw new Error(`${trace["🪪️id"]}: ${JSON.stringify(actual)} != ${JSON.stringify(trace.expected)}`);
  if (trace.accounting.seedPagesBefore !== trace.accounting.seedPagesAfter || trace.accounting.abiBytesBefore !== trace.accounting.abiBytesAfter) throw new Error(`${trace["🪪️id"]}: fixture accounting is not balanced`);
  return { raw: JSON.stringify(actual), projection: actual, productionDispatch: { invoked: true, operation: trace["🪪️id"], bridgeVersion: 1 } };
}

/** ⚖️ Selects one retained ingress owner at a time by lane rank and stable arrival sequence. */
function verifyIngress(candidate: IngressCase): void {
  const rank = { interactive: 0, "user-visible": 1, background: 2, maintenance: 3 } as const;
  const retained = candidate.frames.map((frame, arrival) => ({ frame, arrival }));
  const selected: number[] = [];
  while (retained.length > 0) {
    retained.sort((left, right) => rank[left.frame.lane] - rank[right.frame.lane] || left.arrival - right.arrival);
    selected.push(...retained.splice(0, candidate.maxFramesPerDrive).map(({ frame }) => frame.actor));
  }
  if (JSON.stringify(selected) !== JSON.stringify(candidate.expectedActors)) throw new Error(`${candidate["🪪️id"]}: ${JSON.stringify(selected)} != ${JSON.stringify(candidate.expectedActors)}`);
}

function scenario(ctx: AdapterContext): AdapterOutcome {
  const payload = ctx.scenario.steps.find((step) => step.keyword === "When" && step.docString !== undefined)?.docString;
  if (payload === undefined) throw new Error(`${ctx.scenario.id}: missing trace payload`);
  const id = (JSON.parse(payload) as { id: string }).id;
  const trace = fixture.traces.find((candidate) => candidate["🪪️id"] === id);
  if (trace === undefined) throw new Error(`${ctx.scenario.id}: unknown trace ${JSON.stringify(id)}`);
  return verify(trace);
}

export default defineTestAdapter({ implementation: "typescript", scenarios: { "production-traces": { subject: scenario } } });

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  for (const trace of fixture.traces) verify(trace);
  for (const candidate of fixture.ingressCases) verifyIngress(candidate);
  console.log(`relay-lifecycle oracle: ${fixture.traces.length + fixture.ingressCases.length}/${fixture.traces.length + fixture.ingressCases.length}`);
}
