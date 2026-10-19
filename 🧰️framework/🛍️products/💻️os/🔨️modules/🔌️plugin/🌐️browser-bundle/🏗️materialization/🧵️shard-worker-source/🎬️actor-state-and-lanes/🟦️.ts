/** 🎬️ The shard worker actor state, pacing constants, lane order and frame interpretation text. */
import { SEGMENTED_DOWNLOAD_CHUNK_BYTES, SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS, SHARD_TURN_DRIVE_STEP_CEILING, SHARD_TURN_GUEST_COST_MS } from "../../🧾️contract/🟦️.ts";

/** 🗂 Returns the worker text of the actor tables, the pacing constants, the lane ordering and the frame interpretation. */
export function shardWorkerActorStateSource(): string {
  return `const actors = new Map(); // actorId -> { api, moduleUrl }
const activatingActors = new Set();
let lastActivationGeneration = 0n;
const inFlightTurnActors = new Set();
let turnSeq = 0;
let heartbeatSabView = null;
let heartbeatShardIndex = -1;
// 📤️ Per-chunk byte cap of the segmented-download lane, interpolated from the ONE schema-owned chunk
// contract (\`🎭️actor/📮️shard-client/📤️segmented-download/🧫️fixtures/🔣️.json\`'s \`contract\`, mirrored by
// \`SEGMENTED_DOWNLOAD_CONTRACT\`) that the guest producer slices by and the host drain admits against —
// never a literal of this worker's own.
const SEGMENTED_DOWNLOAD_CHUNK_BYTES = ${SEGMENTED_DOWNLOAD_CHUNK_BYTES};
// 🫀️ Progress-heartbeat cadence, interpolated from the ONE schema-owned liveness policy
// (\`🎭️actor/📮️shard-client/🧫️fixtures/🔣️.json\`'s \`policy\`, mirrored by \`SHARD_LIVENESS_POLICY\`) the
// host watchdog reads — never a literal of this worker's own.
const PROGRESS_HEARTBEAT_INTERVAL_MS = ${SHARD_PROGRESS_HEARTBEAT_INTERVAL_MS};
// 🚚️ The worker-owned MoreWork drive, interpolated from \`SHARD_TURN_GUEST_COST_MS\`/
// \`SHARD_TURN_DRIVE_STEP_CEILING\` — the MEASURED cost of one whole guest turn, which is what the
// drive's step ceiling is derived FROM. Twin of \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s
// \`driveShardTurnMoreWorkV1\`/\`shardTurnDriveStepsV1\`, which own the law.
const GUEST_TURN_COST_MS = ${SHARD_TURN_GUEST_COST_MS};
const DRIVE_STEP_CEILING = ${SHARD_TURN_DRIVE_STEP_CEILING};
let progressHandle = null;
let inFlightRequests = 0;
// 📬️ Every message the host posts bumps this, and a drive that started at one value crosses back the
// moment it sees another — that is the whole "a host-owned input interrupts the drive" contract. It
// counts messages rather than naming kinds on purpose: a kind this worker has never heard of is still
// the host speaking, and the drive must never be the reason it waits.
let hostInputSeq = 0;

// 📨️ terra-web-shardframe: ShardFrame::Grant/Envelope support — see this file's own header doc.
const MAINTENANCE_LANE_DEFAULT_BUDGET = { fuel: 80000000, wallMs: 200, memoryBytes: 256 * 1024 * 1024, uiNodes: 4000, mailboxLen: 1024, maxEffects: 512, maxPatchBytes: 2097152,maxFrames:8 };
const SHARD_FRAME_LANE_ORDER = ["Interactive", "UserVisible", "Background", "Maintenance"];
const grantedBudgets = new Map(); // actorId -> last ShardFrame::Grant budget, mirrors ShardLoop::granted_budgets

function orderEnvelopesByLane(envelopes) {
  return envelopes
    .map((envelope, index) => ({ envelope, index }))
    .sort((left, right) => {
      const rank = SHARD_FRAME_LANE_ORDER.indexOf(left.envelope.lane) - SHARD_FRAME_LANE_ORDER.indexOf(right.envelope.lane);
      return rank !== 0 ? rank : left.index - right.index;
    })
    .map((entry) => entry.envelope);
}

// 🧠️ Mirrors 🧵️shard-client.ts's interpretShardFrame — see that function's own doc for the semantics.
function interpretFrame(frame, actorId) {
  switch (frame.kind) {
    case "Register":
      return { action: "register" };
    case "Unregister":
      return { action: "unregister" };
    case "Grant":
      grantedBudgets.set(frame.actor, frame.budget);
      return { action: "runEnvelopes", budget: frame.budget, envelopes: orderEnvelopesByLane(frame.envelopes) };
    case "Envelope":
      return { action: "runEnvelopes", budget: grantedBudgets.has(actorId) ? grantedBudgets.get(actorId) : MAINTENANCE_LANE_DEFAULT_BUDGET, envelopes: [frame.envelope] };
    default:
      return { action: "unknown" };
  }
}

// 🫀️ ONE beat door for every liveness signal this worker emits. It advances the sequence and mirrors
// it into the shared \`Atomics\` slot, and it RETURNS the beat instead of posting it — because a beat
// taken at a boundary that is about to cross anyway rides the crossing (\`reply\`/\`replyError\` carry
// \`beat\`), and a message that carries liveness is one message, not two.
//
// 🐛️ It used to post unconditionally, so every turn crossing cost THREE main-thread messages: a
// start-of-request \`heartbeat\`, a \`turn-step\` \`heartbeat\`, and the \`result\` itself — on a main
// thread the hop measurement shows is the binding constraint
// (\`📓️reactor-reconcile-spin-2026-09-14.md\` §7 item 3, which named this and left it). Neither posted
// beat told the host anything the reply does not: \`ShardClient.noteLiveness\` treats EVERY inbound
// message as proof of life, and the request's own start instant already counts as proven-alive in
// \`evaluateShardLiveness\`. What is NOT foldable is a beat with no crossing to ride — \`loadActor\`'s
// await boundaries and the while-busy ticker — so those still call \`postBeat\`.
function beat(phase) {
  turnSeq += 1;
  if (heartbeatSabView) Atomics.store(heartbeatSabView, heartbeatShardIndex, turnSeq);
  return { turnSeq, phase: phase === undefined ? null : phase };
}

function postBeat(phase) {
  self.postMessage({ kind: "heartbeat", ...beat(phase) });
}

// 🫀️ The whole busy-versus-dead discriminator: while ANY request is outstanding, an interval beats
// every PROGRESS_HEARTBEAT_INTERVAL_MS. A worker legitimately parked on one multi-second \`await\`
// (fetching, compiling and instantiating a multi-MB wasm component) still runs its event loop, so it
// keeps beating and the host's watchdog keeps its miss count at zero; a worker wedged inside
// synchronous guest code cannot run this callback at all, so it still dies after the same
// \`missedLimit\` windows. Guarded on \`setInterval\` existing so a bare VM/test context is unaffected.
function beginRequest() {
  inFlightRequests += 1;
  if (progressHandle !== null || typeof setInterval !== "function") return;
  progressHandle = setInterval(() => postBeat("progress"), PROGRESS_HEARTBEAT_INTERVAL_MS);
}

function endRequest() {
  inFlightRequests -= 1;
  if (inFlightRequests > 0 || progressHandle === null) return;
  clearInterval(progressHandle);
  progressHandle = null;
}

// ⏱️ The ONE clock this worker shares with the page. A Worker's \`performance.now()\` counts from its
// OWN \`timeOrigin\`, so only \`timeOrigin + now()\` — the same Unix-epoch millisecond on both sides at
// sub-millisecond resolution — lets the CROSSINGS be measured (post → receive, reply → receive)
// rather than inferred by subtracting the parts from the whole. Twin of
// \`🔨️modules/⏱️trace/🟦️.ts\`'s \`hopTraceEpochNowMs\`, which is what reads these back on the page.
`;
}
