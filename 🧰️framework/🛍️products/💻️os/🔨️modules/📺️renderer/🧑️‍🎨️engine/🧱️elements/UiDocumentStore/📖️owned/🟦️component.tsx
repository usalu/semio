//#region 🧬️OwnedReadSource
import { useCallback, useLayoutEffect, useRef, useSyncExternalStore } from "react";
import type { OwnedUiReadSource, OwnedUiReadSubscription } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️component.ts";
import type { RetainedUiNodeRecord } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️component.ts";

export type { OwnedUiReadSource, OwnedUiReadSubscription };
//#endregion 🧬️OwnedReadSource

//#region 🪝️OwnedReadHook
/** 🪝️ Subscriptions own reads outside speculative render; layout commit acknowledges the exact issued token. */
export function useOwnedUiNode(source: OwnedUiReadSource, id: number): RetainedUiNodeRecord | undefined {
  const current = useRef<{ source: OwnedUiReadSource; id: number; subscription: OwnedUiReadSubscription } | null>(null);
  const subscribe = useCallback((notify: () => void) => {
    const subscription = source.subscribeNode(id, notify);
    const exact = { source, id, subscription }; current.current = exact; notify();
    return () => { if (current.current === exact) current.current = null; source.unsubscribeNode(subscription); };
  }, [source, id]);
  const getSnapshot = useCallback(() => {
    const exact = current.current;
    return exact?.source === source && exact.id === id ? exact.subscription.snapshot : null;
  }, [source, id]);
  const snapshot = useSyncExternalStore(subscribe, getSnapshot, () => null);
  useLayoutEffect(() => {
    const exact = current.current;
    if (snapshot && exact?.source === source && exact.id === id) source.acknowledgeRead(exact.subscription, snapshot);
  }, [source, id, snapshot]);
  return snapshot?.record;
}
//#endregion 🪝️OwnedReadHook
