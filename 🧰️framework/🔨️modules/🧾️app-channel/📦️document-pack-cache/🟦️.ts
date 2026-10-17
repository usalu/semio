/** 📦️ App channel document pack cache: the per-instance `{pack, spr}` copy of the document, refreshed by every document reply. */

import type { AppFrameValue } from "../../🪢️wire-contracts/🟦️.ts";
import type { ChannelLifecycle } from "../🔌️channel-lifecycle/🟦️.ts";

//#region 🔖️AppChannelClient
/** 🗄️ Per-instance document-pack cache (ticket
 * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4: "the
 * browser host keeps NO document pack per instance today"). Populated from BOTH directions —
 * the archive load's ready root pair and every `AppFrame::Document` reply any sent command's outcome carries
 * (`ReadDocument`, or any future command that happens to include one) — so a transaction coordinator can
 * ask "what does this instance's document look like right now" without a dedicated round trip. */
export class DocumentPackCache {
  private readonly lifecycle: ChannelLifecycle;
  private pack: Uint8Array | null = null;
  private spr: Uint8Array | null = null;

  constructor(lifecycle: ChannelLifecycle) {
    this.lifecycle = lifecycle;
  }

  /** 💾️ Keeps exactly these bytes as the document's current pair. */
  remember(pack: Uint8Array, spr: Uint8Array): void {
    this.pack = pack;
    this.spr = spr;
  }

  /** 🧹️ Drops the pair: the document changed in a way no reply described, or the channel closed. */
  forget(): void {
    this.pack = null;
    this.spr = null;
  }

  /** 🧱️ The cached `{pack, spr}` for this instance's document, or `null` before any
   * ready archive load or `AppFrame::Document` reply has been observed. Surfaced to the
   * transaction coordinator through the `PluginWasmHandle` adapter's own `documentPack` accessor
   * (`PluginRuntime/🟦️.tsx`) — a contributor plan call needs the target's current snapshot
   * pack, and this is the only place that snapshot is retained host-side. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.pack && this.spr ? { pack: this.pack.slice(), spr: this.spr.slice() } : null;
  }

  /** 🔎️ Scans every frame one sent command's outcome carried for `AppFrame::Document` and refreshes
   * the pair — the "every `AppFrame::Document` reply" half of the cache-population contract. */
  capture(frames: readonly AppFrameValue[]): void {
    if (this.lifecycle.disposed || frames.some(frame => "Error" in frame)) return;
    for (const frame of frames) {
      if ("Document" in frame) {
        this.pack = new Uint8Array(frame.Document.pack);
        this.spr = new Uint8Array(frame.Document.spr);
      }
    }
  }
}
//#endregion 🔖️AppChannelClient
