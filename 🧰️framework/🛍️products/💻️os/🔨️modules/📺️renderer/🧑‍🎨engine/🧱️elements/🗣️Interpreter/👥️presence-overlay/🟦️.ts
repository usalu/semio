/** 👥️ This session's own hover, selection and preview marks on a node, keyed by node key, plus the host row-action refusals and the live guest presence merged into the overlay a rendered tree reads.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { createContext, useContext, useMemo, useSyncExternalStore } from "../../⚛️react-port/🟦️.ts";
import { guestPresenceTableV1, subscribeGuestPresenceV1 } from "../../📃️UiDocumentStore/🟦️.tsx";

//#region 🔖️UiInterpreter
//#region PresenceOverlay
/** 👥️ This session's own hover/selection/preview state on one node, keyed by `UiNodeRecord.key` (NOT
 * `UiNodeId` — presence must land on the right element across a reconciliation that reassigns ids but
 * keeps keys stable, mirroring `crate::PresenceUpdate`'s own doc). Populated from `PresenceUpdate`
 * wire messages by whoever owns the transport (host-side, outside this element); never derived from
 * or written into the `📃️UiDocumentStore` — presence changes at input frequency and must never touch a
 * document revision. */
export type UiPresenceOverlayEntry = {
  readonly hovered?: boolean;
  readonly selected?: boolean;
  readonly previewed?: boolean;
  /** 👥️ Host-localized lines about peers acting on this node (e.g. "Ada is editing this in time travel"), read out
   * after a tree row's own description. */
  readonly notes?: readonly string[];
};

export type UiPresenceOverlayValue = {
  readonly byKey: ReadonlyMap<string, UiPresenceOverlayEntry>;
};

const EMPTY_PRESENCE_OVERLAY: UiPresenceOverlayValue = { byKey: new Map() };

export const UiPresenceOverlayContext = createContext<UiPresenceOverlayValue>(EMPTY_PRESENCE_OVERLAY);

/** 🛑️ Row actions the HOST itself refuses, by verb, each with the reason it names — a state the host holds before the
 * guest's refreshed rows say it (the stage of an open history-edit session); `null`: the host refuses none. A refused
 * action reads and behaves exactly as one the guest published disabled with that reason: focusable, `aria-disabled`,
 * described by the reason, never dispatched — so nothing changes when the guest's rows arrive and say the same. */
export const RowActionRefusalsContext = createContext<ReadonlyMap<string, string> | null>(null);

export function usePresenceOverlayEntry(key: string): UiPresenceOverlayEntry {
  const overlay = useContext(UiPresenceOverlayContext);
  return overlay.byKey.get(key) ?? {};
}

/** 👥️ The overlay a rendered tree actually reads: whatever a caller provided through
 * {@link UiPresenceOverlayContext} (the component tests' own channel), overlaid by the live table the
 * plugin runtime fills from each turn's `presence` array. Without the live half the context had no
 * production provider at all and every retained row rendered `aria-selected="false"` however the
 * guest's interaction domain moved (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gap F1). */
export function useUiPresenceOverlay(): UiPresenceOverlayValue {
  const provided = useContext(UiPresenceOverlayContext);
  const guest = useSyncExternalStore(subscribeGuestPresenceV1, guestPresenceTableV1, guestPresenceTableV1);
  return useMemo(() => {
    if (guest.byKey.size === 0) return provided;
    if (provided.byKey.size === 0) return guest;
    const byKey = new Map(provided.byKey);
    for (const [key, entry] of guest.byKey) byKey.set(key, { ...byKey.get(key), ...entry });
    return { byKey };
  }, [provided, guest]);
}
//#endregion PresenceOverlay
//#endregion 🔖️UiInterpreter
