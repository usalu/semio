import { hostContinuations } from "@semio-tech/framework/async/continuation";

/** 🪃️ Macrotask yield for the UI continuation loops, delegated to the host's ONE
 * {@link hostContinuations} scheduler — the same object the guest's `Effect::DispatchAction` re-arm
 * goes through, so there is a single owner of "run this again, soon" on this target rather than one
 * primitive per loop. The hazard it exists for is unchanged: a `setTimeout(0)` chain is throttled to
 * one tick per second in a hidden tab (once per minute under Chrome's intensive throttling), which
 * turned a few thousand intake steps into minutes of silence; a `MessageChannel` message is a
 * macrotask page visibility never throttles. */
export async function yieldPluginUiContinuation(): Promise<void> {
  await hostContinuations.yieldContinuation();
}
