// 🚪️ IoEntryDescriptor[] mirror for `s.stdio.txt@utf-8/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Empty by the carrier law: this
// dialect IS `CARRIER_TEXT`, so it registers zero foreign io hops on its own side — see the Rust
// twin `🦀️component.rs`'s `io()` doc comment (mirrors `💾️binary`'s own, same reasoning).
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

export const ioEntries: IoEntryDescriptorMirror[] = [];
