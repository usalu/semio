/** 🧭️ The React host's answer to a guest's typed activation request (`Effect::PublishEvent` on `os.on-demand.request`): the request goes to the host's activator door and the effect pass moves on, because the activator reports its own progress, cancel and notices and never rejects. The WGPU shell hands the same request to `semioWgpuActivateOnDemand`. */
import { guestActivationRequest, type Effect } from "@semio-tech/framework";

type ActivationDoor = { semioReactActivateOnDemand?: (requestJson: string) => Promise<string> };

/** 🧭️ Whether `effect` was an activation request, and if so starts it; an effect of any other topic is left to the caller, an invalid payload is dropped silently, never thrown into the effect pass. */
export function applyOnDemandRequestEffect(effect: Extract<Effect, Readonly<Record<"publishEvent", unknown>>>, door: ActivationDoor = globalThis as ActivationDoor): boolean {
  let request;
  try { request = guestActivationRequest(effect.publishEvent); }
  catch { return true; }
  if (request === undefined) return false;
  void door.semioReactActivateOnDemand?.(JSON.stringify(request)).catch(() => undefined);
  return true;
}
