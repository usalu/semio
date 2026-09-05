import { OwnedResidentLedger } from "../../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts";

const ledger = new OwnedResidentLedger({ bytes: 33554432, slots: 262144, owners: 262144, control: { bytes: 65536, slots: 1024, owners: 1024 } });

/** 🏘️ React and WGPU in this renderer composition share this exact ledger, including while it closes. */
export function rendererResidentLedger(): OwnedResidentLedger { return ledger; }
