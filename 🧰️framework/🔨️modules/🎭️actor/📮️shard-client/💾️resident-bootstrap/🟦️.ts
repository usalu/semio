import type { ShardClientState } from "../🗃️state/🟦️.ts";
import type { UiResidentComposition } from "../../../🖱️ui/🧬️contract/🧵️retained/🔌️host/🏦️composition/🟦️.ts";
import type { ShardClientHubPort } from "../🧩️hub/🟦️.ts";
import { type OwnedResidentLedger, OwnedResidentRecordDetachment, OwnedResidentRetirement, type OwnedResidentAdmission, type OwnedResidentRecord, type ResidentGrant, type ResidentStep } from "@semio-tech/value-ts/resident";
import { OwnedUiResidentPool, OwnedUiResidentPoolRetirement } from "../../../🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts";
import { NO_RESIDENT_FAULT, type ShardSlot, poolControllerEnvelope, poolRecordEnvelope, residentChild, residentGrant, residentStep, workerControllerEnvelope } from "../🔐️activation/🟦️.ts";

/** 📋️ The phase fields of the UI resident pool admission and of the worker bootstrap admission; field names and order are the census the admission accounting prices. */
export class ResidentAdmissionState {
  uiResidentControllerCell: OwnedResidentAdmission | null = null;
  uiResidentControllerRecord: OwnedResidentRecord | null = null;
  uiResidentCell: OwnedResidentAdmission | null = null;
  uiResidentRecord: OwnedResidentRecord | null = null;
  uiResidentPool: OwnedUiResidentPool | null = null;
  uiResidentPhase: "controller-empty" | "controller-preparing" | "controller-prepare-refused" | "controller-cell-held" | "controller-claiming" | "controller-claimed" | "controller-record-admitting" | "controller-installing" | "controller-observing" | "controller-rejected" | "empty" | "preparing" | "prepare-refused" | "cell-held" | "claiming" | "claimed" | "record-admitting" | "prepared" | "rejected" | "unused-closing" | "record-observing" | "cell-closing" | "cell-observing" | "owned" | "pool-closing" | "pool-observing" | "pool-proved" | "closing" | "detached" | "retired" = "controller-empty";
  uiResidentWitness: OwnedUiResidentPoolRetirement | null = null;
  uiResidentFault: unknown = NO_RESIDENT_FAULT;
  uiResidentClosing = false;
  clientAdmissionPurpose: "none" | "ui-pool" | "worker-root" = "none";
  workerBootstrapCell: OwnedResidentAdmission | null = null;
  workerBootstrapRecord: OwnedResidentRecord | null = null;
  workerBootstrapPhase: "empty" | "preparing" | "prepare-refused" | "cell-held" | "claiming" | "claim-refused" | "claimed" | "record-admitting" | "record-refused" | "installing" | "observing" | "ready" | "close-preparing" | "close-prepare-refused" | "close-cell-held" | "close-claiming" | "close-record-admitting" | "close-record-refused" | "record-held" | "cell-closing" | "close-attempted" | "pending-release-observing" | "cell-observing" | "fault-held" | "cancelled" = "empty";
  workerBootstrapFault: unknown = NO_RESIDENT_FAULT;
  workerAdmissionCell: OwnedResidentAdmission | null = null;
  workerAdmissionRecord: OwnedResidentRecord | null = null;
  workerAdmissionIndex: number | null = null;
  workerAdmissionShell: ShardSlot | null = null;
}

/** 💾️ Drives the resident-ledger admission of the worker bootstrap metadata and of the shared UI resident pool as bounded, restartable steps; the phase fields live in {@link ResidentAdmissionState}. */
export class ShardResidentBootstrap {
  private readonly state: ShardClientState;
  private readonly resident = new ResidentAdmissionState();
  private readonly hub: ShardClientHubPort<UiResidentComposition>;
  constructor(hub: ShardClientHubPort<UiResidentComposition>) { this.hub = hub; this.state = hub.state; }

  /** 🏗️ Prepares only the original client's separately charged worker metadata. */
  prepareWorkerBootstrap(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-worker.prepare");
    switch (this.resident.workerBootstrapPhase) {
      case "close-preparing": case "close-prepare-refused": case "close-cell-held": case "close-claiming": case "close-record-admitting": case "close-record-refused": case "record-held": case "cell-closing": case "close-attempted": case "pending-release-observing": case "cell-observing": case "fault-held": case "cancelled": return residentStep("rejected", "actor-worker.stopped");
    }
    try {
      const recovered = this.recoverWorkerBootstrap(); if (recovered) return recovered;
      if (this.resident.workerBootstrapFault !== NO_RESIDENT_FAULT || this.resident.workerBootstrapCell?.hasFailure) return residentStep("rejected", "actor-worker.fault-held");
    } catch (error) { this.captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.recovery-fault"); }
    try { const shared = this.prepareSharedResidentController(grant); if (shared) return shared; }
    catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-worker.shared-fault"); }
    try {
      if (this.resident.workerBootstrapPhase === "ready") return residentStep(this.resident.workerBootstrapRecord?.matchesLiveShell(this.hub.shell) ? "ready" : "rejected", "actor-worker.prepared");
      if (this.resident.workerBootstrapPhase === "empty") {
        if (this.resident.clientAdmissionPurpose !== "none") return residentStep("blocked", "actor-worker.foreign-purpose");
        if (this.resident.workerAdmissionCell || this.resident.workerAdmissionRecord || this.resident.workerAdmissionIndex !== null || this.resident.workerAdmissionShell) return residentStep("rejected", "actor-worker.child-held");
        if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-worker.bootstrap");
        this.resident.clientAdmissionPurpose = "worker-root"; this.resident.workerBootstrapPhase = "preparing";
        const current = this.state.residentLedger.prepareAdmission(this.hub.shell, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected") this.resident.workerBootstrapPhase = "prepare-refused";
        return residentChild(current, grant);
      }
      const cell = this.resident.workerBootstrapCell; if (!cell) return residentStep("rejected", "actor-worker.cell");
      if (this.resident.workerBootstrapPhase === "cell-held") {
        if (this.resident.clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
        this.resident.workerBootstrapPhase = "claiming"; const current = this.state.residentLedger.claimAdmission(this.hub.shell, cell, grant);
        if (current.kind === "blocked") this.resident.workerBootstrapPhase = "cell-held";
        else if (current.kind === "rejected") this.resident.workerBootstrapPhase = "claim-refused";
        return residentChild(current, grant);
      }
      if (this.resident.workerBootstrapPhase === "claimed") {
        if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-worker.record");
        this.resident.workerBootstrapPhase = "record-admitting"; const admitted = this.state.residentLedger.reserveRecord("data", workerControllerEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") this.resident.workerBootstrapPhase = "claimed";
        else if (admitted.step.kind === "rejected") this.resident.workerBootstrapPhase = "record-refused";
        return residentChild(admitted.step, grant);
      }
      if (this.resident.workerBootstrapPhase === "installing" && this.resident.workerBootstrapRecord) {
        this.resident.workerBootstrapPhase = "observing"; return residentChild(this.resident.workerBootstrapRecord.install(this.hub.shell, grant), grant);
      }
      return residentStep("rejected", "actor-worker.admission");
    } catch (error) { this.captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.prepare-fault"); }
  }

  private captureWorkerBootstrapFault(error: unknown): void {
    if (this.resident.workerBootstrapFault === NO_RESIDENT_FAULT) this.resident.workerBootstrapFault = error;
    else if (!Object.is(this.resident.workerBootstrapFault, error)) throw error;
  }

  private recoverWorkerBootstrap(): ResidentStep | null {
    const phase = this.resident.workerBootstrapPhase;
    if (phase === "preparing" || phase === "prepare-refused" || phase === "close-preparing" || phase === "close-prepare-refused") {
      if (this.resident.clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
      const cell = this.state.residentLedger.preparedAdmission(this.hub.shell);
      if (!cell) {
        if ((phase !== "prepare-refused" && phase !== "close-prepare-refused") || this.resident.workerBootstrapFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-worker.admission-handoff");
        this.resident.clientAdmissionPurpose = "none"; this.resident.workerBootstrapPhase = phase === "close-prepare-refused" ? "cancelled" : "empty";
        return residentStep("pending", "actor-worker.empty-admission-observation", 64);
      }
      this.resident.workerBootstrapCell = cell; this.resident.workerBootstrapPhase = phase === "close-preparing" || phase === "close-prepare-refused" ? "close-cell-held" : "cell-held";
      return residentStep("pending", "actor-worker.cell-observation", 64);
    }
    const cell = this.resident.workerBootstrapCell;
    if ((phase === "claiming" || phase === "close-claiming") && cell) {
      if (this.resident.clientAdmissionPurpose !== "worker-root") return residentStep("blocked", "actor-worker.foreign-purpose");
      const pending = this.state.residentLedger.preparedAdmission(this.hub.shell);
      if (cell.claimed && pending === null) {
        this.resident.clientAdmissionPurpose = "none"; this.resident.workerBootstrapPhase = phase === "close-claiming" ? "close-cell-held" : "claimed";
        return residentStep("pending", "actor-worker.claim-observation", 64);
      }
      if (phase === "close-claiming" && pending === cell && !cell.claimed) { this.resident.workerBootstrapPhase = "close-cell-held"; return residentStep("pending", "actor-worker.unclaimed-close-observation", 64); }
      return residentStep("blocked", "actor-worker.unclaimed", 64);
    }
    if ((phase === "record-admitting" || phase === "close-record-admitting" || phase === "record-refused" || phase === "close-record-refused") && cell) {
      const result = cell.result; this.resident.workerBootstrapRecord = result?.record ?? null;
      if (!result && (phase === "record-refused" || phase === "close-record-refused") && this.resident.workerBootstrapFault === NO_RESIDENT_FAULT) { this.resident.workerBootstrapPhase = phase === "close-record-refused" ? "close-cell-held" : "claimed"; return residentStep("pending", "actor-worker.unused-record-refusal-observation", 64); }
      const ready = phase !== "close-record-admitting" && this.resident.workerBootstrapRecord !== null && result?.step.kind === "ready" && !cell.hasFailure && this.resident.workerBootstrapFault === NO_RESIDENT_FAULT;
      this.resident.workerBootstrapPhase = ready ? "installing" : "record-held";
      return residentStep(ready || phase === "close-record-admitting" ? "pending" : "rejected", "actor-worker.record-observation", 64);
    }
    if (phase === "observing" && this.resident.workerBootstrapRecord) {
      if (!this.resident.workerBootstrapRecord.matchesShell(this.hub.shell)) return residentStep("blocked", "actor-worker.installation", 64);
      const live = this.resident.workerBootstrapFault === NO_RESIDENT_FAULT && this.resident.workerBootstrapRecord.matchesLiveShell(this.hub.shell);
      this.resident.workerBootstrapPhase = live ? "ready" : "record-held";
      return residentStep(live ? "pending" : "rejected", "actor-worker.installation-observation", 64);
    }
    return null;
  }

  /** 🛑️ Stops this metadata admission; admitted controller records require a later whole-client witness. */
  closeWorkerBootstrapStep(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-worker.close");
    if (this.resident.workerBootstrapPhase === "cancelled") return residentStep("complete", "actor-worker.close");
    switch (this.resident.workerBootstrapPhase) {
      case "empty": this.resident.workerBootstrapPhase = "cancelled"; return residentStep("complete", "actor-worker.unstarted-close", 64);
      case "preparing": this.resident.workerBootstrapPhase = "close-preparing"; break;
      case "prepare-refused": this.resident.workerBootstrapPhase = "close-prepare-refused"; break;
      case "claiming": this.resident.workerBootstrapPhase = "close-claiming"; break;
      case "record-admitting": this.resident.workerBootstrapPhase = "close-record-admitting"; break;
      case "record-refused": this.resident.workerBootstrapPhase = "close-record-refused"; break;
      case "cell-held": case "claim-refused": case "claimed": this.resident.workerBootstrapPhase = "close-cell-held"; break;
      case "installing": case "observing": case "ready": this.resident.workerBootstrapPhase = "record-held"; return residentStep("pending", "actor-worker.record-retained", 64);
    }
    try {
      const recovered = this.recoverWorkerBootstrap(); if (recovered) return recovered;
      const cell = this.resident.workerBootstrapCell;
      if (this.resident.workerBootstrapFault !== NO_RESIDENT_FAULT) {
        if (!cell) return residentStep("blocked", "actor-worker.fault-without-cell");
        if (!cell.hasFailure) return cell.retainFailure(this.resident.workerBootstrapFault, grant);
        if (!Object.is(cell.failure, this.resident.workerBootstrapFault)) return residentStep("blocked", "actor-worker.distinct-fault");
      }
      if (this.resident.workerBootstrapRecord || cell?.result || this.resident.workerBootstrapPhase === "record-held") return residentStep("blocked", "actor-worker.record-retained");
      if (!cell) return residentStep("blocked", "actor-worker.cell-proof");
      if (this.resident.workerBootstrapPhase === "close-cell-held") { this.resident.workerBootstrapPhase = "cell-closing"; cell.beginClose(); return residentStep("pending", "actor-worker.cell-begin-close", 64); }
      if (this.resident.workerBootstrapPhase === "pending-release-observing") {
        if (this.state.residentLedger.preparedAdmission(this.hub.shell) === cell) return residentStep("blocked", "actor-worker.pending-release-proof", 64);
        if (cell.hasFailure) { if (this.resident.clientAdmissionPurpose === "worker-root") this.resident.clientAdmissionPurpose = "none"; this.resident.workerBootstrapPhase = "fault-held"; }
        else this.resident.workerBootstrapPhase = "cell-closing";
        return residentStep("pending", "actor-worker.pending-release-observation", 64);
      }
      if (this.resident.workerBootstrapPhase === "cell-observing") {
        if (cell.hasFailure || this.resident.workerBootstrapFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty()) return residentStep("blocked", "actor-worker.cell-retirement", 64);
        this.resident.workerBootstrapCell = null; if (this.resident.clientAdmissionPurpose === "worker-root") this.resident.clientAdmissionPurpose = "none"; this.resident.workerBootstrapPhase = "cancelled";
        return residentStep("complete", "actor-worker.cell-unlink-observation", 64);
      }
      if (this.resident.workerBootstrapPhase === "close-attempted" || this.resident.workerBootstrapPhase === "fault-held") return residentStep("blocked", "actor-worker.close-handoff");
      if (this.resident.workerBootstrapPhase === "cell-closing") {
        this.resident.workerBootstrapPhase = "close-attempted"; const current = cell.closeStep(grant);
        this.resident.workerBootstrapPhase = current.kind === "complete" ? "cell-observing" : current.kind === "pending" && current.phase === "resident-admission-bootstrap-release" ? "pending-release-observing" : "cell-closing";
        return residentChild(current, grant);
      }
      return residentStep("blocked", "actor-worker.close-phase");
    } catch (error) { this.captureWorkerBootstrapFault(error); return residentStep("rejected", "actor-worker.close-fault"); }
  }

  prepareUiResidentPool(ledger: OwnedResidentLedger, grant: ResidentGrant): ResidentStep {
    if (ledger !== this.state.residentLedger) return residentStep("rejected", "actor-resident.foreign-ledger");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-prepare");
    if (this.resident.uiResidentClosing || this.resident.uiResidentPool) return residentStep("rejected", "actor-resident.pool-owned");
    try {
      const recovered = this.recoverUiResidentPool(); if (recovered) return recovered;
      const shared = this.prepareSharedResidentController(grant); if (shared) return shared;
      if (this.resident.uiResidentFault !== NO_RESIDENT_FAULT || this.resident.uiResidentCell?.hasFailure) return residentStep("rejected", "actor-resident.pool-fault-retirement");
      if (this.resident.uiResidentPhase === "prepared") return residentStep("ready", "actor-resident.pool-prepare");
      if (this.resident.uiResidentPhase === "empty") {
        if (this.resident.clientAdmissionPurpose !== "none") return residentStep("blocked", "actor-resident.foreign-purpose");
        if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-resident.pool-bootstrap");
        this.resident.clientAdmissionPurpose = "ui-pool";
        this.resident.uiResidentPhase = "preparing"; const current = ledger.prepareAdmission(this.hub.shell, "data", grant);
        if (current.kind === "blocked" || current.kind === "rejected") this.resident.uiResidentPhase = "prepare-refused"; return residentChild(current, grant);
      }
      const cell = this.resident.uiResidentCell; if (!cell) return residentStep("rejected", "actor-resident.pool-cell");
      if (this.resident.uiResidentPhase === "cell-held") {
        if (this.resident.clientAdmissionPurpose !== "ui-pool") return residentStep("blocked", "actor-resident.foreign-purpose");
        this.resident.uiResidentPhase = "claiming"; const current = ledger.claimAdmission(this.hub.shell, cell, grant);
        if (current.kind === "blocked") this.resident.uiResidentPhase = "cell-held"; return residentChild(current, grant);
      }
      if (this.resident.uiResidentPhase === "claimed") {
        if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-resident.pool-record");
        this.resident.uiResidentPhase = "record-admitting"; const admitted = ledger.reserveRecord("data", poolRecordEnvelope, cell, grant);
        if (admitted.step.kind === "blocked") this.resident.uiResidentPhase = "claimed"; return residentChild(admitted.step, grant);
      }
      return residentStep("rejected", "actor-resident.pool-owned");
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-prepare-fault"); }
  }

  captureUiResidentPoolFault(error: unknown): void {
    if (this.resident.uiResidentFault === NO_RESIDENT_FAULT) this.resident.uiResidentFault = error;
    else if (!Object.is(this.resident.uiResidentFault, error)) throw error;
  }

  private prepareSharedResidentController(grant: ResidentGrant): ResidentStep | null {
    const recovered = this.recoverUiResidentController(); if (recovered) return recovered;
    if (this.resident.uiResidentFault !== NO_RESIDENT_FAULT || this.resident.uiResidentControllerCell?.hasFailure) return residentStep("rejected", "actor-resident.controller-fault-held");
    if (this.resident.uiResidentPhase === "retired" && !this.resident.uiResidentControllerCell && !this.resident.uiResidentControllerRecord) { this.resident.uiResidentPhase = "controller-empty"; return residentStep("pending", "actor-resident.shared-unstarted-observation", 64); }
    const prepared = this.prepareUiResidentController(grant); if (prepared) return prepared;
    return this.resident.uiResidentControllerRecord?.matchesLiveShell(this.hub.shell) ? null : residentStep("rejected", "actor-resident.controller-not-live");
  }

  private recoverUiResidentController(): ResidentStep | null {
    if (this.resident.uiResidentPhase === "controller-preparing" || this.resident.uiResidentPhase === "controller-prepare-refused") {
      const cell = this.state.residentLedger.preparedAdmission(this.hub.shell);
      if (!cell) {
        if (this.resident.uiResidentPhase !== "controller-prepare-refused" || this.resident.uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.controller-admission-handoff");
        this.resident.uiResidentPhase = "controller-empty"; return residentStep("pending", "actor-resident.controller-empty-admission", 64);
      }
      this.resident.uiResidentControllerCell = cell; this.resident.uiResidentPhase = "controller-cell-held"; return residentStep("pending", "actor-resident.controller-cell-observation", 64);
    }
    if (this.resident.uiResidentPhase === "controller-claiming" && this.resident.uiResidentControllerCell) {
      if (!this.resident.uiResidentControllerCell.claimed || this.state.residentLedger.preparedAdmission(this.hub.shell) !== null) return residentStep("blocked", "actor-resident.controller-unclaimed", 64);
      this.resident.uiResidentPhase = "controller-claimed"; return residentStep("pending", "actor-resident.controller-claim-observation", 64);
    }
    if (this.resident.uiResidentPhase === "controller-record-admitting" && this.resident.uiResidentControllerCell) {
      const result = this.resident.uiResidentControllerCell.result; this.resident.uiResidentControllerRecord = result?.record ?? null;
      const ready = this.resident.uiResidentControllerRecord !== null && result?.step.kind === "ready" && !this.resident.uiResidentControllerCell.hasFailure && this.resident.uiResidentFault === NO_RESIDENT_FAULT;
      this.resident.uiResidentPhase = ready ? "controller-installing" : "controller-rejected"; return residentStep(ready ? "pending" : "rejected", "actor-resident.controller-record-observation", 64);
    }
    if (this.resident.uiResidentPhase === "controller-observing" && this.resident.uiResidentControllerRecord) {
      if (!this.resident.uiResidentControllerRecord.matchesShell(this.hub.shell)) return residentStep("blocked", "actor-resident.controller-installation", 64);
      if (this.resident.uiResidentFault !== NO_RESIDENT_FAULT || !this.resident.uiResidentControllerRecord.matchesLiveShell(this.hub.shell)) { this.resident.uiResidentPhase = "controller-rejected"; return residentStep("rejected", "actor-resident.controller-not-live", 64); }
      this.resident.uiResidentPhase = "empty"; return residentStep("pending", "actor-resident.controller-installation", 64);
    }
    return null;
  }

  private prepareUiResidentController(grant: ResidentGrant): ResidentStep | null {
    if (this.resident.uiResidentPhase === "controller-empty") {
      if (!residentGrant(grant, 296)) return residentStep("blocked", "actor-resident.controller-bootstrap");
      this.resident.uiResidentPhase = "controller-preparing"; const current = this.state.residentLedger.prepareAdmission(this.hub.shell, "data", grant);
      if (current.kind === "blocked" || current.kind === "rejected") this.resident.uiResidentPhase = "controller-prepare-refused"; return residentChild(current, grant);
    }
    const cell = this.resident.uiResidentControllerCell;
    if (this.resident.uiResidentPhase === "controller-cell-held" && cell) {
      this.resident.uiResidentPhase = "controller-claiming"; const current = this.state.residentLedger.claimAdmission(this.hub.shell, cell, grant);
      if (current.kind === "blocked") this.resident.uiResidentPhase = "controller-cell-held"; return residentChild(current, grant);
    }
    if (this.resident.uiResidentPhase === "controller-claimed" && cell) {
      if (!residentGrant(grant, 264)) return residentStep("blocked", "actor-resident.controller-record");
      this.resident.uiResidentPhase = "controller-record-admitting"; const admitted = this.state.residentLedger.reserveRecord("data", poolControllerEnvelope, cell, grant);
      if (admitted.step.kind === "blocked") this.resident.uiResidentPhase = "controller-claimed"; return residentChild(admitted.step, grant);
    }
    if (this.resident.uiResidentPhase === "controller-installing" && this.resident.uiResidentControllerRecord) {
      this.resident.uiResidentPhase = "controller-observing"; return residentChild(this.resident.uiResidentControllerRecord.install(this.hub.shell, grant), grant);
    }
    return this.resident.uiResidentPhase === "controller-rejected" ? residentStep("rejected", "actor-resident.controller-admission") : null;
  }

  private recoverUiResidentPool(): ResidentStep | null {
    if (!this.resident.uiResidentCell && (this.resident.uiResidentPhase === "preparing" || this.resident.uiResidentPhase === "prepare-refused")) {
      if (this.resident.clientAdmissionPurpose !== "ui-pool") return residentStep("blocked", "actor-resident.foreign-purpose");
      const cell = this.state.residentLedger.preparedAdmission(this.hub.shell);
      if (!cell) {
        if (this.resident.uiResidentPhase !== "prepare-refused" || this.resident.uiResidentFault !== NO_RESIDENT_FAULT) return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.resident.clientAdmissionPurpose = "none";
        this.resident.uiResidentPhase = "empty"; return residentStep("pending", "actor-resident.pool-empty-admission", 64);
      }
      this.resident.uiResidentCell = cell; this.resident.uiResidentPhase = "cell-held"; return residentStep("pending", "actor-resident.pool-cell-observation", 64);
    }
    if (this.resident.uiResidentPhase === "claiming" && this.resident.uiResidentCell) {
      if (this.resident.clientAdmissionPurpose !== "ui-pool" || !this.resident.uiResidentCell.claimed || this.state.residentLedger.preparedAdmission(this.hub.shell) !== null) return residentStep("blocked", "actor-resident.pool-unclaimed", 64);
      this.resident.clientAdmissionPurpose = "none"; this.resident.uiResidentPhase = "claimed"; return residentStep("pending", "actor-resident.pool-claim-observation", 64);
    }
    if (this.resident.uiResidentCell && this.resident.uiResidentPhase === "record-admitting") {
      const result = this.resident.uiResidentCell.result; this.resident.uiResidentRecord = result?.record ?? null;
      const ready = this.resident.uiResidentRecord !== null && result?.step.kind === "ready" && !this.resident.uiResidentCell.hasFailure && this.resident.uiResidentFault === NO_RESIDENT_FAULT;
      this.resident.uiResidentPhase = ready ? "prepared" : "rejected"; return residentStep(ready ? "pending" : "rejected", "actor-resident.pool-record-observation", 64);
    }
    return null;
  }

  private handoffUiResidentFault(grant: ResidentGrant): ResidentStep | null {
    if (this.resident.uiResidentFault === NO_RESIDENT_FAULT) return null; const cell = this.resident.uiResidentCell ?? this.resident.uiResidentControllerCell;
    if (!cell) return residentStep("blocked", "actor-resident.pool-fault-retirement");
    if (cell.hasFailure) return Object.is(cell.failure, this.resident.uiResidentFault) ? null : residentStep("blocked", "actor-resident.pool-distinct-fault");
    return cell.retainFailure(this.resident.uiResidentFault, grant);
  }

  private closeUiResidentAdmission(grant: ResidentGrant): ResidentStep {
    const cell = this.resident.uiResidentCell; const record = this.resident.uiResidentRecord;
    if (!cell) return residentStep("blocked", "actor-resident.pool-cell-proof");
    if (this.resident.uiResidentPhase === "record-observing") {
      if (!record || !OwnedResidentRetirement.matches(record.retirement, record)) return residentStep("blocked", "actor-resident.pool-record-proof", 64);
      cell.beginClose(); this.resident.uiResidentPhase = "cell-closing"; return residentStep("pending", "actor-resident.pool-record-observation", 64);
    }
    if (this.resident.uiResidentPhase === "cell-closing") {
      const current = cell.closeStep(grant); if (current.kind === "complete") this.resident.uiResidentPhase = "cell-observing"; return residentChild(current, grant);
    }
    if (this.resident.uiResidentPhase === "cell-observing") {
      if (this.resident.uiResidentFault !== NO_RESIDENT_FAULT || !OwnedResidentRetirement.matches(cell.retirement, cell) || !cell.terminalIsEmpty() || record !== null && (!OwnedResidentRetirement.matches(record.retirement, record) || !record.terminalIsEmpty())) return residentStep("blocked", "actor-resident.pool-cell-proof", 64);
      this.resident.uiResidentRecord = null; this.resident.uiResidentCell = null; this.resident.uiResidentPool = null; this.resident.uiResidentWitness = null; if (this.resident.clientAdmissionPurpose === "ui-pool") this.resident.clientAdmissionPurpose = "none"; this.resident.uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-release", 64);
    }
    return residentStep("blocked", "actor-resident.pool-cell-phase");
  }

  ownsUiResidentPool(pool: unknown): boolean { return pool !== null && this.resident.uiResidentPool === pool; }

  closeUiResidentPoolStep(grant: ResidentGrant): ResidentStep {
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-parent-close");
    if (this.resident.uiResidentPhase === "retired") return residentStep("complete", "actor-resident.pool-parent-close");
    this.resident.uiResidentClosing = true;
    try {
      const controller = this.recoverUiResidentController(); if (controller) return controller;
      const recovered = this.recoverUiResidentPool(); if (recovered) return recovered;
      const handoff = this.handoffUiResidentFault(grant); if (handoff) return handoff;
      if (this.resident.uiResidentControllerCell?.hasFailure) {
        if (!this.resident.uiResidentControllerRecord && !this.resident.uiResidentControllerCell.claimed) { this.resident.uiResidentControllerCell.beginClose(); return this.resident.uiResidentControllerCell.closeStep(grant); }
        return residentStep("rejected", "actor-resident.controller-fault-held");
      }
      if (this.resident.uiResidentPhase === "controller-empty" && !this.resident.uiResidentControllerCell) { this.resident.uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-unstarted-close", 64); }
      const prepared = this.prepareUiResidentController(grant); if (prepared) return prepared;
      const cell = this.resident.uiResidentCell; const record = this.resident.uiResidentRecord; const pool = this.resident.uiResidentPool;
      if (!cell) {
        if (this.resident.uiResidentPhase !== "empty") return residentStep("blocked", "actor-resident.pool-admission-handoff");
        this.resident.uiResidentPhase = "retired"; return residentStep("complete", "actor-resident.pool-parent-close", 64);
      }
      if (this.resident.uiResidentPhase === "record-observing" || this.resident.uiResidentPhase === "cell-closing" || this.resident.uiResidentPhase === "cell-observing") return this.closeUiResidentAdmission(grant);
      if (!pool) {
        if (!record) { cell.beginClose(); this.resident.uiResidentPhase = "cell-closing"; return residentStep("pending", "actor-resident.pool-unused-cell-close", 64); }
        if (this.resident.uiResidentPhase === "prepared" || this.resident.uiResidentPhase === "rejected") { this.resident.uiResidentPhase = "unused-closing"; record.beginClose(); return residentStep("pending", "actor-resident.pool-unused-close", 64); }
        if (this.resident.uiResidentPhase === "unused-closing") { const current = record.closeStep(grant); if (current.kind === "complete") this.resident.uiResidentPhase = "record-observing"; return residentChild(current, grant); }
        return residentStep("blocked", "actor-resident.pool-unused-proof");
      }
      if (this.resident.uiResidentPhase === "owned") { this.resident.uiResidentPhase = "pool-closing"; pool.beginClose(); return residentStep("pending", "actor-resident.pool-begin-close", 64); }
      if (this.resident.uiResidentPhase === "pool-closing") {
        const current = pool.closeStep(grant); const forwarded = residentChild(current, grant);
        if (current.kind === "complete" && forwarded.kind === "pending") this.resident.uiResidentPhase = "pool-observing"; return forwarded;
      }
      if (this.resident.uiResidentPhase === "pool-observing") {
        const witness = pool.retirement; if (!OwnedUiResidentPoolRetirement.matches(witness, pool, this.hub.shell, this.state.residentLedger)) return residentStep("blocked", "actor-resident.pool-private-proof", 64);
        this.resident.uiResidentWitness = witness; this.resident.uiResidentPhase = "pool-proved"; return residentStep("pending", "actor-resident.pool-observation", 64);
      }
      return this.releaseUiResidentPool(pool, this.resident.uiResidentWitness, grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-parent-fault"); }
  }

  installUiResidentPool(pool: OwnedUiResidentPool, grant: ResidentGrant): ResidentStep {
    const record = this.resident.uiResidentRecord;
    if (!record || (this.resident.uiResidentPhase !== "prepared" && this.resident.uiResidentPhase !== "owned") || !OwnedUiResidentPool.matchesComposition(pool, this.hub.shell, this.state.residentLedger) || this.resident.uiResidentPool !== null && this.resident.uiResidentPool !== pool) return residentStep("rejected", "actor-resident.pool-install");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-install");
    this.resident.uiResidentPool = pool; this.resident.uiResidentPhase = "owned";
    try {
      if (record.matchesShell(pool)) return residentStep("ready", "actor-resident.pool-installed");
      if (this.resident.uiResidentFault !== NO_RESIDENT_FAULT || this.resident.uiResidentCell?.hasFailure) return residentStep("blocked", "actor-resident.pool-fault-retirement");
      return record.install(pool, grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-install-fault"); }
  }

  releaseUiResidentPool(pool: OwnedUiResidentPool, witness: unknown, grant: ResidentGrant): ResidentStep {
    const record = this.resident.uiResidentRecord;
    if (!record || this.resident.uiResidentPool !== pool || !OwnedUiResidentPoolRetirement.matches(witness, pool, this.hub.shell, this.state.residentLedger) || this.resident.uiResidentWitness !== null && this.resident.uiResidentWitness !== witness) return residentStep("rejected", "actor-resident.pool-witness");
    if (!residentGrant(grant, 64)) return residentStep("blocked", "actor-resident.pool-release");
    try {
      const handoff = this.handoffUiResidentFault(grant); if (handoff) return handoff;
      this.resident.uiResidentWitness = witness;
      if (this.resident.uiResidentPhase === "closing" && OwnedResidentRecordDetachment.matches(record.detachment, record, pool)) { this.resident.uiResidentPhase = "detached"; return residentStep("pending", "actor-resident.pool-detachment", 64); }
      if (this.resident.uiResidentPhase === "owned" || this.resident.uiResidentPhase === "pool-proved") { this.resident.uiResidentPhase = "closing"; record.beginClose(); return residentStep("pending", "actor-resident.pool-close-record", 64); }
      if (this.resident.uiResidentPhase === "closing") return record.detach(pool, grant);
      if (this.resident.uiResidentPhase === "detached") {
        const current = record.closeStep(grant); if (current.kind === "complete") this.resident.uiResidentPhase = "record-observing"; return residentChild(current, grant);
      }
      return this.closeUiResidentAdmission(grant);
    } catch (error) { this.captureUiResidentPoolFault(error); return residentStep("rejected", "actor-resident.pool-release-fault"); }
  }
}
