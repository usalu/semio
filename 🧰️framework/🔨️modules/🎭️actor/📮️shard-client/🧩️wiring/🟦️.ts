import { ShardResidentBootstrap } from "../💾️resident-bootstrap/🟦️.ts";
import { ShardRequestTransport } from "../📬️request-transport/🟦️.ts";
import { ShardRouting } from "../🧭️routing/🟦️.ts";
import { ShardExactActivations } from "../🪪️exact-activation/🟦️.ts";
import { ShardInstanceLifecycles } from "../🚪️instance-lifecycle/🟦️.ts";
import { ShardCapturedReturns } from "../↩️captured-return/🟦️.ts";
import { ShardUiAcknowledgements } from "../📥️ui-acknowledgement/🟦️.ts";
import { ShardHostEffectBridge } from "../🌉️host-effect-bridge/🟦️.ts";
import { ShardWatchdog } from "../⏱️watchdog/🟦️.ts";
import type { ShardClient } from "../🟦️.ts";
import type { ShardClientState } from "../🗃️state/🟦️.ts";

/** 🧩️ Wires the collaborators of one {@link ShardClient} around its shell and its single private state; collaborators reach each other only through this hub, and only at call time, never while they are constructed. */
export class ShardClientHub {
  readonly residents: ShardResidentBootstrap;
  readonly transport: ShardRequestTransport;
  readonly routing: ShardRouting;
  readonly activations: ShardExactActivations;
  readonly lifecycles: ShardInstanceLifecycles;
  readonly returns: ShardCapturedReturns;
  readonly uiAcknowledgements: ShardUiAcknowledgements;
  readonly effects: ShardHostEffectBridge;
  readonly watchdog: ShardWatchdog;
  constructor(readonly shell: ShardClient, readonly state: ShardClientState) {
    this.residents = new ShardResidentBootstrap(this);
    this.transport = new ShardRequestTransport(this);
    this.routing = new ShardRouting(this);
    this.activations = new ShardExactActivations(this);
    this.lifecycles = new ShardInstanceLifecycles(this);
    this.returns = new ShardCapturedReturns(this);
    this.uiAcknowledgements = new ShardUiAcknowledgements(this);
    this.effects = new ShardHostEffectBridge(this);
    this.watchdog = new ShardWatchdog(this);
  }
}
