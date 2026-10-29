/** 📨️ The shard worker lifecycle turn classification and message handler text. */
/** 📬 Returns the worker text of the lifecycle turn classification and the message handler. */
export function shardWorkerMessageHandlerSource(): string {
  return `const LIFECYCLE_TURN_EVENT_KINDS = ["instance-open", "instance-close", "instance-lifecycle-ack"];
const REACTOR_TURN_DEADLINE_CODE = "plugin.reactor-turn-deadline";

function guestFaultRecord(error) {
  for (const value of [error, error?.payload, error?.payload?.val, error?.val]) {
    if (!value) continue;
    if (typeof value.code === "string") return value;
    if (value instanceof Uint8Array) {
      try { return JSON.parse(new TextDecoder().decode(value)); } catch { return null; }
    }
  }
  return null;
}

function retryableLifecycleTurn(error, events, commandPage) {
  const fault = guestFaultRecord(error);
  if (!fault || fault.code !== REACTOR_TURN_DEADLINE_CODE || fault.retryable !== true) return false;
  return (commandPage === undefined || commandPage === null) && Array.isArray(events) && events.length <= 1 && events.every((entry) => entry && LIFECYCLE_TURN_EVENT_KINDS.includes(entry.kind));
}

self.addEventListener("message", async (event) => {
  // ⏱️ FIRST statement of the handler: everything after it is already this worker's own cost, and
  // \`postedAtEpochMs\` − this instant is the post crossing nobody could see from outside (the CDP
  // \`Performance\` domain answers nothing on a worker target —
  // \`📓️react-hop-latency-2026-09-14.md\` §1). Twin vocabulary: \`semio.hop.worker.*\`.
  const receivedAtEpochMs = hopEpochNow();
  // 📬️ The host said something. A drive in flight reads this counter every step and crosses back the
  // moment it moves, so an ingress message, a cancel or a view-state change never waits behind a
  // guest the worker is pumping. Bumped BEFORE any dispatch, for every kind without exception.
  hostInputSeq += 1;
  const msg = event.data ?? {};
  const { kind } = msg;
  if (kind === "attachHeartbeatSab") {
    heartbeatShardIndex = msg.shardIndex;
    heartbeatSabView = msg.sab ? new Int32Array(msg.sab) : null;
    return;
  }
  if (kind === "original-root-refusal") {
    const actor = actors.get(msg.actorId);
    if (actor && actor.activationGeneration === msg.activationGeneration && actor.api.originalSource.nativeRefusal === undefined) actor.api.originalSource.holdNativeRefusal(msg.refusal);
    return;
  }
  if (kind === "dispose") {
    const actor = actors.get(msg.actorId);
    if (!actor || actor.activationGeneration !== msg.activationGeneration) return;
    actor.disposeRequested = true;
    if (inFlightTurnActors.has(msg.actorId)) { actor.api.originalSource.cancel(); return; }
    await closeOriginalActor(msg.actorId, msg.activationGeneration);
    return;
  }
  // 🧪️ terra-web-bridges: an effect-complete/effect-error \`"frame"\` is a REPLY to something THIS
  // worker sent (\`🟨️.js\`'s \`effectRequest\`), never a request expecting a \`reply()\` of its
  // own — settled directly, before the generic requestId/actorId-gated dispatch below (which always
  // posts a \`"result"\` back, wrong for a message that is itself already an answer).
  if (kind === "frame" && msg.frame && msg.frame.kind === "Envelope" && msg.frame.envelope && msg.frame.envelope.payload && (msg.frame.envelope.payload.kind === "effect-complete" || msg.frame.envelope.payload.kind === "effect-error")) {
    deliverEffectResult(msg.actorId, msg.activationGeneration, msg.frame.envelope);
    return;
  }
  const { requestId, actorId } = msg;
  if (!requestId || !actorId) return;
  const timings = { postedAtEpochMs: typeof msg.postedAtEpochMs === "number" ? msg.postedAtEpochMs : null, receivedAtEpochMs, guestEnteredAtEpochMs: null, guestLeftAtEpochMs: null, repliedAtEpochMs: null, previousReplyCloneMs: lastReplyCloneMs, events: Array.isArray(msg.events) ? msg.events.length : 0, eventKinds: Array.isArray(msg.events) ? [...new Set(msg.events.map((entry) => (entry && typeof entry.kind === "string" ? entry.kind : "?")))].slice(0, 4).join("+") : "", patches: 0, commandPageBytes: msg.commandPage && msg.commandPage.bytes ? msg.commandPage.bytes.byteLength ?? msg.commandPage.bytes.length ?? 0 : 0 };
  beat();
  beginRequest();
  faultPhase = kind;
  faultActorId = actorId;
  try {
    if (kind === "activate") {
      const entry = await loadActor(actorId, msg.activationGeneration, msg.moduleUrl, msg.originalAdmission);
      entry.pendingAssets = msg.assets ?? [];
      reply(requestId, undefined);
      return;
    }
    const actor = actors.get(actorId);
    // 🛑️ A cancel for an actor that is already gone is a no-op the CALLER still has to be told about:
    // \`ShardClient.cancelJob\` awaits a \`"result"\`, so answering nothing leaves the request outstanding
    // in \`pending\` forever and the watchdog kills the shard over it (ticket
    // 26/09/02/PUZZLE-3D-END-TO-END wave B42). Every other kind genuinely needs the actor.
    if (!actor && kind === "cancelJob") {
      reply(requestId, undefined);
      return;
    }
    if (!actor) throw new Error(\`shard worker: actor \${actorId} not activated\`);
    switch (kind) {
      case "turn": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        faultPhase = (actor.turns ?? 0) === 0 ? "first-step" : "turn";
        actor.turns = (actor.turns ?? 0) + 1;
        try {
          const admitted = spliceInstanceOpenAssets(actor, msg.events);
          timings.guestEnteredAtEpochMs = hopEpochNow();
          let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);
          // 🚚️ THE worker-owned MoreWork drive. A \`more-work\` answer that carried nothing is not a
          // message for the host — it is the guest asking to be pumped, and in the browser the host
          // round trip IS the pump. So the WORKER owns that pump: it runs the next turn itself and
          // crosses only to deliver something the host can act on. Every discarded result satisfied
          // \`shardTurnCarriesNothing\`, so the drive is lossless by construction.
          //
          // Five other stops, and each one is a promise to somebody: the guest went \`idle\`;
          // \`hostInputSeq\` moved, so a host-owned input is pending behind the drive and must not wait
          // for it; the actor was disposed or re-activated; the derived step ceiling is spent; the
          // derived wall budget is spent. The ceiling and the budget come from
          // \`shardTurnDriveSteps\`/\`shardTurnDriveBudgetMs\` — the wall the host GRANTED for this
          // crossing divided by what one turn MEASURES — never from the reactor's own 8 ms executor
          // slice, which is smaller than one whole turn and therefore admitted no further turn at all.
          const originalDriveSource = actor.api.originalSource;
          const remainingUs = originalDriveSource.deadlineUs - originalDriveSource.operationClockUs();
          const grantWallMs = Number(remainingUs > 0n ? remainingUs : 0n) / 1000;
          // 🚚️ The WALL bounds this drive; \`DRIVE_STEP_CEILING\` is the hard backstop against an
          // unbounded loop, and \`shardTurnDriveSteps\` is the grant's derived EXPECTATION, reported so
          // the host can read what one crossing was meant to cover.
          const driveSteps = DRIVE_STEP_CEILING;
          const driveExpectedSteps = shardTurnDriveSteps(grantWallMs, GUEST_TURN_COST_MS);
          const driveDeadline = originalDriveSource.deadlineUs;
          const inputMark = hostInputSeq;
          let drivePolls = 0;
          let driveStopped = shardTurnStatusTag(result) === "more-work" ? (shardTurnCarriesNothing(result) ? "" : "carried") : "idle";
          while (driveStopped === "") {
            if (hostInputSeq !== inputMark) { driveStopped = "input"; break; }
            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = "closed"; break; }
            if (originalDriveSource.operationClockUs() >= driveDeadline || originalDriveSource.fuelRemaining === 0) { driveStopped = "budget"; break; }
            if (drivePolls + 1 >= driveSteps) { driveStopped = "steps"; break; }
            // 📬️ One macrotask before the next turn, so a message the host already posted is
            // DELIVERED and \`hostInputSeq\` can actually move. Without it the drive is
            // uninterruptible: an \`await\` settles on the microtask queue, which never drains the
            // message queue.
            await driveYield();
            if (hostInputSeq !== inputMark) { driveStopped = "input"; break; }
            if (actors.get(actorId) !== actor || actor.activationGeneration !== msg.activationGeneration) { driveStopped = "closed"; break; }
            result = await actor.api.poll([], undefined, undefined, msg.budget);
            drivePolls += 1;
            driveStopped = shardTurnStatusTag(result) === "more-work" ? (shardTurnCarriesNothing(result) ? "" : "carried") : "idle";
          }
          timings.guestLeftAtEpochMs = hopEpochNow();
          timings.drivePolls = drivePolls;
          timings.driveSteps = driveExpectedSteps;
          timings.driveStopped = driveStopped;
          timings.patches = result && Array.isArray(result.uiPatches) ? result.uiPatches.length : 0;
          timings.status = result && result.status && typeof result.status.tag === "string" ? result.status.tag : String(result && result.status);
          // 🫀️ THE step boundary. A guest running a BUDGETED job (a resumable tessellation, a
          // resumable boolean) crosses this point once per step and blocks the event loop in
          // between, so the while-busy ticker cannot fire and the only thing that distinguishes it
          // from a dead worker is a beat taken HERE, the moment the guest hands control back
          // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
          // \`📓️extension-evaluate-budget-2026-09-12.md\`). It RIDES the reply rather than costing its
          // own \`postMessage\`, because the reply is already crossing at this exact instant and
          // \`ShardClient\` reads liveness off every inbound message it receives.
          reply(requestId, result, timings, beat("turn-step"));
        } finally {
          inFlightTurnActors.delete(actorId);
          if (actor.disposeRequested) await closeOriginalActor(actorId, actor.activationGeneration);
        }
        break;
      }
      case "startJob":
        await actor.api.startJob(msg.job, msg.jobKind, msg.input);
        reply(requestId, undefined);
        break;
      case "stepJob": {
        // 🫀️ Same step boundary as \`turn\` above: a job step is exactly the unit a budgeted guest
        // yields at, so it is exactly where liveness is provable.
        const step = await actor.api.stepJob(msg.job, msg.budget);
        reply(requestId, step, undefined, beat("turn-step"));
        break;
      }
      case "cancelJob":
        await actor.api.cancelJob(msg.job);
        reply(requestId, undefined);
        break;
      case "takeSegmentedDownloadChunk": {
        if (!Number.isSafeInteger(msg.instanceId) || msg.instanceId < 0 || typeof msg.operationId !== "bigint" || msg.operationId <= 0n || msg.operationId > ((1n << 64n) - 1n)) throw new Error("segmented-download-authority-invalid");
        const chunk = await actor.api.takeSegmentedDownloadChunk(msg.instanceId, msg.operationId);
        // 📤️ One bound per refusal, never one "limit" for four different violations: a chunk of the wrong
        // TYPE is a wire-adaptation defect (the guest's \`option<list<u8>>\` reaching this worker as jco's
        // tagged \`{ tag, val }\` object instead of a \`Uint8Array\` — what the 2026-09-12 battery read as
        // an exceeded byte cap while nothing was over any cap), an empty or oversized one is a producer
        // that sliced by the wrong constant.
        if (chunk !== undefined && chunk !== null) {
          if (Object.prototype.toString.call(chunk) !== "[object Uint8Array]") throw new Error("segmented-download-chunk-type");
          if (chunk.byteLength === 0) throw new Error("segmented-download-chunk-empty");
          if (chunk.byteLength > SEGMENTED_DOWNLOAD_CHUNK_BYTES) throw new Error("segmented-download-chunk-over-cap");
        }
        reply(requestId, chunk ?? undefined);
        break;
      }
      case "checkpoint":
        reply(requestId, await actor.api.checkpoint());
        break;
      case "restore":
        await actor.api.restore(msg.state);
        reply(requestId, undefined);
        break;
      case "codec": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          reply(requestId, await actor.api.codec(msg.request), undefined, beat("turn-step"));
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
      case "frame": {
        if (actor.activationGeneration !== msg.activationGeneration) throw new Error("actor-lifecycle.activation-mismatch");
        const result = interpretFrame(msg.frame, actorId);
        if (result.action === "register") {
          reply(requestId, undefined);
          break;
        }
        if (result.action === "unregister") {
          grantedBudgets.delete(actorId);
          reply(requestId, undefined);
          break;
        }
        if (result.action === "unknown") {
          reply(requestId, { ignored: true });
          break;
        }
        if (inFlightTurnActors.has(actorId)) throw new Error(\`shard worker: actor \${actorId} already has a turn in flight\`);
        inFlightTurnActors.add(actorId);
        try {
          const events = spliceInstanceOpenAssets(actor, result.envelopes.map((envelope) => envelope.payload));
          timings.events = events.length;
          timings.guestEnteredAtEpochMs = hopEpochNow();
          // One guest turn per envelope, in frame order, like the native ShardLoop: a reactor turn admits at most one event. A frame of several envelopes answers one turn result per envelope.
          const polls = [];
          for (const batch of events.length === 0 ? [[]] : events.map((event) => [event])) polls.push(await actor.api.poll(batch, undefined, undefined, result.budget));
          timings.guestLeftAtEpochMs = hopEpochNow();
          reply(requestId, polls.length === 1 ? polls[0] : polls, timings);
        } finally {
          inFlightTurnActors.delete(actorId);
        }
        break;
      }
      default:
        throw new Error(\`unknown shard worker message kind: \${kind}\`);
    }
  } catch (error) {
    // 🩺️ Reported on BOTH channels on purpose: \`replyError\` answers the one caller that is awaiting
    // this request, \`reportWorkerFault\` names the phase/actor/module to the shell's console for the
    // boot faults nobody is awaiting a reply for. A retryable lifecycle-turn deadline is neither —
    // it is a yield the client replays, so it never reaches \`onActorTrap\`.
    const retryableLifecycle = kind === "turn" && retryableLifecycleTurn(error, msg.events, msg.commandPage);
    if (retryableLifecycle) console.log(\`shard worker: retryable lifecycle deadline on \${faultPhase} for actor \${actorId}; receipt retained, replaying next tick\`);
    else reportWorkerFault("handler", error, null);
    replyError(requestId, error, msg.events, retryableLifecycle);
  } finally {
    endRequest();
    faultPhase = "idle";
  }
});
`;
}
