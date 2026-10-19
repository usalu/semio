/** 🚦️ The shard worker request beats, turn status helpers and drive pacing text. */
/** ⏱ Returns the worker text of the request beats, the turn status helpers and the drive step pacing. */
export function shardWorkerDrivePacingSource(): string {
  return `const hopEpochNow = () => (typeof performance === "object" && typeof performance.now === "function" ? (typeof performance.timeOrigin === "number" ? performance.timeOrigin : 0) + performance.now() : Date.now());

// 🏷️ The ONE spelling of a turn status. jco lifts \`more-work\` kebab-cased, the host's own fixtures
// spell it \`moreWork\`, and \`🖼️wire-turn/🟦️.ts\`'s \`wireTurnStatusTag\` reconciles both — this is its
// verbatim twin, because a drive that misreads the status would hand back a turn nobody asked for.
function shardTurnStatusTag(result) {
  const tag = result && typeof result === "object" && result.status && typeof result.status === "object" ? result.status.tag : undefined;
  return typeof tag === "string" ? tag.replace(/([a-z])([A-Z])/g, "$1-$2").toLowerCase() : "";
}

// 🤫️ Whether ONE reactor turn result carried nothing the host could act on: no ui patch, no effect,
// no presence, no wake, neither receipt, and both ingress lanes idle. Every field of \`turn-result\`
// (\`🔌️plugin/🧬️schema/📜️reactor.wit\`) is named here on purpose — a carrier this predicate forgot would be
// DROPPED by the drive below, so the list is exhaustive by construction and asserted as such by
// \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s law. \`fuelUsed\` is the one field with no host reader.
function shardTurnCarriesNothing(result) {
  if (!result || typeof result !== "object") return false;
  if (!Array.isArray(result.uiPatches) || result.uiPatches.length !== 0) return false;
  if (!Array.isArray(result.effects) || result.effects.length !== 0) return false;
  if (result.presence !== undefined && (!Array.isArray(result.presence) || result.presence.length !== 0)) return false;
  if (result.nextWake !== null && result.nextWake !== undefined) return false;
  if (result.lifecycleReceipt !== undefined && result.lifecycleReceipt !== null) return false;
  if (result.uiPatchReceipt !== undefined && result.uiPatchReceipt !== null) return false;
  if (!result.commandIngress || result.commandIngress.tag !== "idle") return false;
  if (!result.coldPairIngress || result.coldPairIngress.tag !== "idle") return false;
  return true;
}

// 🚚️ The drive's two bounds, DERIVED from the wall the host granted for this crossing and what one
// guest turn measures — never a constant of this worker's own. Verbatim twins of
// \`🎭️actor/🖼️wire-turn/🟦️.ts\`'s \`shardTurnDriveStepsV1\`/\`shardTurnDriveBudgetMsV1\`.
function shardTurnDriveSteps(grantWallMs, guestTurnCostMs) {
  if (!Number.isFinite(grantWallMs) || !Number.isFinite(guestTurnCostMs) || guestTurnCostMs <= 0) return 1;
  return Math.max(1, Math.min(DRIVE_STEP_CEILING, Math.floor(grantWallMs / guestTurnCostMs)));
}

function shardTurnDriveBudgetMs(grantWallMs, guestTurnCostMs) {
  if (!Number.isFinite(grantWallMs) || grantWallMs <= 0) return Math.max(0, guestTurnCostMs);
  return Math.max(grantWallMs, guestTurnCostMs);
}

// 📬️ ONE macrotask, so a message the host already posted is DELIVERED before the drive takes its next
// step. Without it \`hostInputSeq\` could never move mid-drive: an \`await\` settles on the microtask
// queue, which never drains the message queue, so the drive would be uninterruptible by construction.
// A \`MessageChannel\` task and not \`setTimeout(0)\`, for the reason the host's own
// \`hostContinuations\` gives: a timer chain is throttled to one tick per second in a hidden tab (once
// per minute under intensive throttling) and a channel message is a macrotask visibility never
// throttles. Waiters are a QUEUE because two actors may be driving this worker at the same time.
`;
}
