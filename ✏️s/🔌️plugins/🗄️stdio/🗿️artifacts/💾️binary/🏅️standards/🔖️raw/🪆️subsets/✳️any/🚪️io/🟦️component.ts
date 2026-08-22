// 🚪️ IoEntryDescriptor[] mirror for `s.stdio.binary@raw/*` (ticket
// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Empty by the carrier law: this
// dialect IS `CARRIER_BINARY`, so it registers zero foreign io hops on its own side — see the
// Rust twin `🦀️component.rs`'s `io()` doc comment for the full reasoning. Shaped inline (no
// generated `IoEntryDescriptor` type checked in anywhere yet in this repo) so this file has no
// fragile forward reference; swap for a real import once owned schema generation lands.
export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

export const ioEntries: IoEntryDescriptorMirror[] = [];
