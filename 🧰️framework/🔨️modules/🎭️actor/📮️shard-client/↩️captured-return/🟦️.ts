import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { ShardClientHub } from "../🧩️wiring/🟦️.ts";
import { OwnedResidentRecordDetachment, OwnedResidentRetirement, type ResidentGrant, type ResidentStep } from "@semio-tech/value-ts/resident";
import { decodeActorReturnResult, encodeActorReturnDrive, type ActorReturnDrive, type ActorReturnResult } from "../../📤️return/🟦️.ts";
import { OwnedActorTurnOutput, OwnedActorTurnOutputs } from "../../🪪️activation/🚪️instance/📥️output/🟦️.ts";
import { type CapturedReturn, type CapturedReturnWork, NO_RETURN_FAULT, type ShardInstanceOwner, type ShardReturnAdmission, type ShardReturnReport, mintCapturedReturn, mintCapturedReturnPage, residentChild, residentGrant, residentStep, returnAdmission, returnDomainEnvelope, sameReturnIdentity, sameReturnOrigin } from "../🔐️activation/🟦️.ts";
import type { OutboundMessage } from "../📨️messages/🟦️.ts";
import type { ShardBudget } from "../📜️wire/🟦️.ts";

/** ↩️ Admits, drives and retires the fixed-response captured return of an instance. */
export class ShardCapturedReturns {
  private readonly state: ShardClientState;
  constructor(private readonly hub: ShardClientHub) { this.state = hub.state; }

  reserveInstanceReturn(instance: ShardInstanceOwner, maximumResponses: number, grant: ResidentGrant): ShardReturnAdmission {
    if (instance.returnPhase === "closing") return returnAdmission("rejected", "actor-return.closing");
    if (!Number.isSafeInteger(maximumResponses) || maximumResponses < 1 || maximumResponses > 0xffffffff) return returnAdmission("rejected", "actor-return.capacity");
    if (!residentGrant(grant, 64)) return returnAdmission("blocked", "actor-return.admission");
    if (instance.returnCapacity !== 0 && instance.returnCapacity !== maximumResponses || instance.activation.returned !== null && instance.activation.returned.instance !== instance) return returnAdmission("rejected", "actor-return.original-owner");
    let spent = 64;
    try {
      const ledger = this.state.residentLedger;
      if (instance.returnPhase === "preparing") {
        const cell = ledger.preparedAdmission(instance);
        if (!cell) {
          if (instance.returnFault !== NO_RETURN_FAULT) return returnAdmission("blocked", "actor-return.cell-handoff");
          instance.returnPhase = "empty"; instance.returnCapacity = 0; return returnAdmission("pending", "actor-return.empty-admission", 64);
        }
        instance.returnCell = cell; instance.returnPhase = "cell-held"; return returnAdmission("pending", "actor-return.cell-observation", 64);
      }
      const cell = instance.returnCell;
      if (instance.returnPhase === "record-admitting" && cell) {
        const result = cell.result; instance.returnRecord = result?.record ?? null;
        const ready = instance.returnRecord !== null && result?.step.kind === "ready" && !cell.hasFailure && instance.returnFault === NO_RETURN_FAULT;
        instance.returnPhase = ready ? "record-held" : "rejected"; return returnAdmission(ready ? "pending" : "rejected", "actor-return.record-observation", 64);
      }
      if (instance.returnPhase === "installing" && instance.returnRecord) {
        if (!instance.returnRecord.matchesShell(instance)) return returnAdmission("blocked", "actor-return.parent-installation", 64);
        instance.returnPhase = "installed"; return returnAdmission("pending", "actor-return.parent-observation", 64);
      }
      if (instance.returnFault !== NO_RETURN_FAULT) {
        if (cell && !cell.hasFailure) return { step: residentChild(cell.retainFailure(instance.returnFault, grant), grant), source: null };
        return returnAdmission("rejected", "actor-return.construction-fault");
      }
      if (cell?.hasFailure || instance.returnPhase === "rejected") return returnAdmission("rejected", "actor-return.admission-refused");
      const state = instance.activation.returned;
      if ((state !== null || instance.returnPhase === "installed") && !instance.returnRecord?.matchesLiveShell(instance)) return returnAdmission("rejected", "actor-return.parent-not-live");
      if (instance.returnPhase === "published") return state?.facade && !state.failed ? { step: residentStep("ready", "actor-return.original-source"), source: state.facade } : returnAdmission("rejected", "actor-return.owner-fault");
      instance.operation.assertActive();
      if (instance.inFlight) return returnAdmission("blocked", "actor-return.request-pending");
      if (instance.returnPhase === "empty") {
        if (!residentGrant(grant, 296)) return returnAdmission("blocked", "actor-return.bootstrap");
        spent = 296;
        instance.returnCapacity = maximumResponses; instance.returnPhase = "preparing"; const current = ledger.prepareAdmission(instance, "data", grant);
        if (current.kind === "blocked") { instance.returnPhase = "empty"; instance.returnCapacity = 0; }
        return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "cell-held" && cell) {
        instance.returnPhase = "claiming"; const current = ledger.claimAdmission(instance, cell, grant);
        if (current.kind === "blocked") instance.returnPhase = "cell-held"; return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "claiming" && cell) {
        if (!cell.claimed) return returnAdmission("rejected", "actor-return.unclaimed");
        instance.returnPhase = "claimed"; return returnAdmission("pending", "actor-return.claim-observation", 64);
      }
      if (instance.returnPhase === "claimed" && cell) {
        if (!residentGrant(grant, 264)) return returnAdmission("blocked", "actor-return.record");
        spent = 264;
        instance.returnPhase = "record-admitting"; const admitted = ledger.reserveRecord("data", returnDomainEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") instance.returnPhase = "claimed"; return { step: residentChild(admitted.step, grant), source: null };
      }
      if (instance.returnPhase === "record-held" && instance.returnRecord) {
        instance.returnPhase = "installing"; const current = instance.returnRecord.install(instance, grant);
        if (current.kind === "blocked") instance.returnPhase = "record-held"; return { step: residentChild(current, grant), source: null };
      }
      if (instance.returnPhase === "installed") {
        if (!residentGrant(grant, 320)) return returnAdmission("blocked", "actor-return.state");
        spent = 320;
        const created: CapturedReturn = { instance, outputs: null, client: this.hub.shell, facade: null, origin: null, identity: null, events: null, latest: null, page: null, content: null, inFlight: false, retry: false, failed: false, fault: NO_RETURN_FAULT, cancelled: false, retired: false };
        instance.activation.returned = created; instance.returnPhase = "state-held"; Object.seal(created); return returnAdmission("pending", "actor-return.state", 320);
      }
      if (instance.returnPhase === "state-held" && state) {
        if (!residentGrant(grant, 256)) return returnAdmission("blocked", "actor-return.roster");
        spent = 256;
        state.outputs = new OwnedActorTurnOutputs(instance, instance.returnCapacity, ledger); instance.returnPhase = "roster-held"; Object.freeze(state.outputs); return returnAdmission("pending", "actor-return.roster", 256);
      }
      if (instance.returnPhase === "roster-held" && state) {
        if (!residentGrant(grant, 80)) return returnAdmission("blocked", "actor-return.facade");
        spent = 80;
        instance.returnPhase = "facade-held"; mintCapturedReturn(state); return returnAdmission("pending", "actor-return.facade", 80);
      }
      if (instance.returnPhase === "facade-held" && state?.facade && state.outputs && instance.returnRecord?.matchesShell(instance)) {
        instance.returnPhase = "published"; return { step: residentStep("ready", "actor-return.publication", 64), source: state.facade };
      }
      return returnAdmission("rejected", "actor-return.admission-phase");
    } catch (error) {
      if (instance.returnFault !== NO_RETURN_FAULT && !Object.is(instance.returnFault, error)) throw error;
      instance.returnFault = error; const state = instance.activation.returned;
      if (state?.instance === instance) { state.failed = true; state.fault = error; }
      return returnAdmission("rejected", "actor-return.construction-fault", spent);
    }
  }

  /** 🧺️ Only a never-executed original return can retire without a guest return/page/content discharge. */
  retireUnusedInstanceReturn(instance: ShardInstanceOwner, grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-return.close-grant");
    const state = instance.activation.returned;
    if (state && state.instance !== instance && !(state.instance === null && instance.returnPhase === "closing")) return residentStep("rejected", "actor-return.foreign-owner");
    if (instance.inFlight || state?.inFlight) return residentStep("blocked", "actor-return.request-pending");
    if (instance.returnFault !== NO_RETURN_FAULT || state && (state.origin !== null || state.identity !== null || state.events !== null || state.page !== null || state.content !== null || state.retry || state.failed || state.fault !== NO_RETURN_FAULT)) return residentStep("blocked", "actor-return.domain-discharge-required");
    if (instance.returnPhase === "empty" && state === null && instance.returnCell === null && instance.returnRecord === null && instance.returnCapacity === 0) return residentStep("complete", "actor-return.unused-retired");
    if (!instance.returnCell) {
      const cell = this.state.residentLedger.preparedAdmission(instance);
      if (!cell) return residentStep("blocked", "actor-return.cell-handoff");
      instance.returnCell = cell; instance.returnPhase = "closing";
      return residentStep("pending", "actor-return.cell-observation", 64);
    }
    instance.returnPhase = "closing";
    const cell = instance.returnCell;
    if (cell.hasFailure) return residentStep("blocked", "actor-return.admission-fault");
    if (state?.outputs) {
      if (!state.outputs.terminalIsEmpty()) { state.outputs.beginClose(); return residentChild(state.outputs.closeStep(grant), grant); }
      state.outputs = null; state.latest = null;
      return residentStep("pending", "actor-return.roster-detachment", 64);
    }
    if (state?.latest) return residentStep("blocked", "actor-return.response-held");
    if (state && (state.instance !== null || state.client !== null || state.facade !== null)) {
      if (!residentGrant(grant, 128)) return residentStep("blocked", "actor-return.state-detachment");
      state.instance = null; state.client = null; state.facade = null;
      return residentStep("pending", "actor-return.state-detachment", 128);
    }
    const record = instance.returnRecord ?? cell.result?.record ?? null;
    if (record && instance.returnRecord !== record) { instance.returnRecord = record; return residentStep("pending", "actor-return.record-observation", 64); }
    if (record?.matchesShell(instance)) { record.beginClose(); return record.detach(instance, grant); }
    if (!cell.terminalIsEmpty()) { cell.beginClose(); return residentChild(cell.closeStep(grant), grant); }
    if (!OwnedResidentRetirement.matches(cell.retirement, cell) || record && (!record.terminalIsEmpty() || !OwnedResidentRetirement.matches(record.retirement, record) || record.detachment !== null && !OwnedResidentRecordDetachment.matches(record.detachment, record, instance))) return residentStep("blocked", "actor-return.retirement-proof");
    if (!residentGrant(grant, 128)) return residentStep("blocked", "actor-return.parent-detachment");
    if (instance.activation.returned !== state) return residentStep("rejected", "actor-return.replaced-owner");
    instance.activation.returned = null; instance.returnCell = null; instance.returnRecord = null; instance.returnCapacity = 0; instance.returnPhase = "empty";
    return residentStep("complete", "actor-return.unused-retired", 128);
  }

  reserveReturnResponse(state: CapturedReturn, grant: ResidentGrant): ResidentStep {
    const instance = state.instance;
    if (!instance) return residentStep("rejected", "actor-return.closed");
    const activation = instance.activation; const slot = activation.slot;
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-return.response-grant");
    if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) return residentStep("rejected", "actor-return.worker-lost");
    if (state.inFlight || instance.inFlight) return residentStep("blocked", "actor-return.request-pending");
    if (instance.returnPhase !== "published" || !state.outputs || !instance.returnRecord?.matchesLiveShell(instance)) return residentStep("rejected", "actor-return.parent-not-live");
    if (OwnedActorTurnOutput.reserved(state.latest, instance)) return residentStep("ready", "actor-return.response-ready");
    try {
      const current = state.outputs.reserve(grant);
      if (current.step.kind === "ready" && current.output) state.latest = current.output;
      if (current.step.kind === "rejected") state.failed = true;
      return current.step;
    } catch (error) {
      state.failed = true; if (state.fault === NO_RETURN_FAULT) state.fault = error; throw error;
    }
  }

  async sendCapturedReturn(state: CapturedReturn, work: CapturedReturnWork, budget: ShardBudget): Promise<ShardReturnReport> {
    const instance = state.instance;
    if (!instance) throw new Error("actor-return.closed");
    const activation = instance.activation; const slot = activation.slot;
    if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) throw new Error("actor-return.worker-lost");
    if (state.inFlight || instance.inFlight) throw new Error("actor-return.request-pending");
    if (state.failed && work.kind !== "cancel") throw new Error("actor-return.owner-fault");
    if (instance.returnPhase !== "published" || state.outputs === null) throw new Error("actor-return.construction-pending");
    if (work.kind === "execute" && state.origin !== null) throw new Error("actor-return.execute-already-owned");
    const execution = work.kind === "execute" || work.kind === "retry";
    if (execution) {
      instance.operation.assertActive();
      if (work.kind === "retry" && !state.retry) throw new Error("actor-return.retry-not-admitted");
    } else if (state.identity === null) throw new Error("actor-return.identity-pending");
    const output = state.latest;
    if (!OwnedActorTurnOutput.reserved(output, instance)) throw new Error("actor-return.response-admission-required");
    let requestId: string;
    try { requestId = this.hub.transport.nextRequestId(); } catch (error) { output.cancelEmpty(); throw error; }
    if (work.kind === "execute") { state.origin = Object.freeze({ activationGeneration: activation.generation, requestSequence: this.state.requestSeq }); state.events = work.events; }
    const drive: ActorReturnDrive = execution ? { kind: "execute", origin: state.origin! } : { kind: "control", control: { kind: work.kind as "poll" | "cancel", identity: state.identity! } };
    const message: OutboundMessage & { readonly returnDrive: Uint8Array } = { kind: "turn", requestId, actorId: activation.actorId, activationGeneration: activation.generation, events: execution ? state.events! : [], budget, returnDrive: encodeActorReturnDrive(drive) };
    state.inFlight = true; state.retry = false; state.latest = output;
    let posted = false;
    try {
      const raw = await output.run(() => this.hub.transport.send<unknown>(slot, message, requestId, () => { posted = true; }, output));
      if (!activation.available || !slot.available || this.state.shards[slot.index] !== slot) throw new Error("actor-return.worker-lost");
      if (!(raw instanceof Uint8Array)) throw new Error("actor-return.fixed-result-required");
      const result = decodeActorReturnResult(raw);
      this.acceptCapturedReturn(state, drive, result, output);
      return result.kind === "page" ? Object.freeze({ kind: "page", receipt: result.receipt }) : result;
    } catch (error) {
      state.retry = execution && !posted;
      if (posted) state.failed = true;
      throw error;
    } finally { state.inFlight = false; }
  }

  private acceptCapturedReturn(state: CapturedReturn, drive: ActorReturnDrive, result: ActorReturnResult, output: OwnedActorTurnOutput): void {
    if (!state.instance) throw new Error("actor-return.closed");
    if (result.kind === "protocolFault") { state.failed = true; return; }
    const identity = result.kind === "page" ? result.receipt.identity : result.kind === "pending" || result.kind === "retired" ? result.identity : result.kind === "control" ? result.control.kind === "inputAck" ? result.control.receipt.identity : result.control.identity : null;
    const origin = result.kind === "refused" ? result.origin : identity!.origin;
    if (!state.origin || !sameReturnOrigin(state.origin, origin)) throw new Error("actor-return.foreign-origin");
    if (drive.kind === "execute") {
      if (result.kind === "control") throw new Error("actor-return.unexpected-control");
      if (result.kind === "refused") { state.retry = true; return; }
    } else {
      if (result.kind === "refused") throw new Error("actor-return.unexpected-refusal");
      if (drive.control.kind !== "poll" && result.kind !== "control") throw new Error("actor-return.control-result-required");
      if (result.kind === "control") {
        const expected = encodeActorReturnDrive(drive); const actual = encodeActorReturnDrive({ kind: "control", control: result.control });
        if (expected.length !== actual.length || expected.some((byte, index) => byte !== actual[index])) throw new Error("actor-return.foreign-control");
      }
    }
    if (identity) {
      if (state.identity === null) {
        if (identity.returnSequence <= state.instance.activation.lastReturnSequence) throw new Error("actor-return.stale-sequence");
        state.identity = identity; state.instance.activation.lastReturnSequence = identity.returnSequence;
      } else if (!sameReturnIdentity(state.identity, identity)) throw new Error("actor-return.foreign-identity");
    }
    if (result.kind === "page") {
      if (state.page !== null || state.cancelled || state.retired) throw new Error("actor-return.page-already-owned");
      state.page = mintCapturedReturnPage(state, output, result.receipt, result.page);
    } else if (result.kind === "control" && result.control.kind === "cancel" && (result.outcome === "accepted" || result.outcome === "duplicate")) state.cancelled = true;
    else if (result.kind === "retired") {
      if (state.page !== null) throw new Error("actor-return.input-retirement-pending");
      state.retired = true;
    }
  }
}
