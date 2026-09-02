//#region 🧬️OwnedReadSource
import { useCallback, useLayoutEffect, useRef, useSyncExternalStore } from "react";
import type { OwnedUiReadSource, OwnedUiReadSubscription, OwnedUiSceneReadSource, OwnedUiSceneRecordView, OwnedUiSceneTextView } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📖️read-lease/🟦️.ts";
import type { RetainedUiNodeRecord } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts";
import type { OwnedUiSceneDiagnostic } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🔗️binding/🟦️.ts";
import type { NumericIndexGrant } from "../../../../../../../../🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts";
import type { OwnedUiInstanceSurface } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts";
import type { OwnedUiSurfaceView } from "../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🧵️retained/🖼️surface/🟦️.ts";

export type { OwnedUiReadSource, OwnedUiReadSubscription };
//#endregion 🧬️OwnedReadSource

//#region 🪝️OwnedReadHook
/** 🪟️ The exact surface publishes immutable root metadata without capturing or copying its content tree. */
export function useOwnedUiView(source: OwnedUiInstanceSurface): OwnedUiSurfaceView {
  const subscribe = useCallback((notify: () => void) => { const subscription = source.subscribeView(notify); return () => source.unsubscribeNode(subscription); }, [source]);
  const snapshot = useCallback(() => source.view, [source]);
  return useSyncExternalStore(subscribe, snapshot, snapshot);
}

function useOwnedRead(source: OwnedUiReadSource, id: number) {
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
  const exact = current.current;
  return { snapshot, subscription: exact?.source === source && exact.id === id ? exact.subscription : null };
}
/** 🪝️ Subscriptions own reads outside speculative render; layout commit acknowledges the exact issued token. */
export function useOwnedUiNode(source: OwnedUiReadSource, id: number): RetainedUiNodeRecord | undefined { return useOwnedRead(source, id).snapshot?.record; }
//#endregion 🪝️OwnedReadHook

//#region 🎬️OwnedSceneEffect
export interface OwnedUiSceneEffectView {
  readonly record: RetainedUiNodeRecord;
  readonly diagnostic: OwnedUiSceneDiagnostic | null;
  openRecord(source?: number): OwnedUiSceneRecordView | null;
  openText(source: number): OwnedUiSceneTextView | null;
}

/** 🎬️ Effects borrow managed scene readers; exact cleanup transfers children to subscription-owned retirement. */
export function useOwnedUiScene(source: OwnedUiSceneReadSource, id: number, consume: (view: OwnedUiSceneEffectView) => () => void): RetainedUiNodeRecord | undefined {
  const { snapshot, subscription } = useOwnedRead(source, id);
  useLayoutEffect(() => {
    const record = snapshot?.record; if (!snapshot || !subscription || !record) return;
    const children: [({ close(): boolean }) | null, ({ close(): boolean }) | null] = [null, null]; let closed = false;
    const open = <T,>(factory: () => { advance(grant: NumericIndexGrant): T; close(): boolean } | null) => {
      if (closed) return null; const slot: 0 | 1 | undefined = children[0] === null ? 0 : children[1] === null ? 1 : undefined; if (slot === undefined) return null;
      const reader = factory(); if (!reader) return null;
      const view = Object.freeze({ advance: (grant: NumericIndexGrant) => reader.advance(grant), close: () => { const accepted = reader.close(); if (accepted) children[slot] = null; return accepted; } }); children[slot] = view; return view;
    };
    const view: OwnedUiSceneEffectView = Object.freeze({ record, diagnostic: snapshot.sceneDiagnostic, openRecord: (at = 0) => open(() => source.openSceneRecord(subscription, snapshot, at)), openText: (at: number) => open(() => source.openSceneText(subscription, snapshot, at)) });
    const release = () => { closed = true; for (let slot = 0; slot < 2; slot++) { children[slot]?.close(); children[slot] = null; } };
    let cleanup: () => void;
    try { cleanup = consume(view); } catch (error) { release(); throw error; }
    return () => { try { cleanup(); } finally { release(); } };
  }, [source, snapshot, subscription, consume]);
  return snapshot?.record;
}
//#endregion 🎬️OwnedSceneEffect
