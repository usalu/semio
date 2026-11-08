import type {OriginalReceivingPrimary} from "../../🛬️decode/📬️outcome/🛬️receiving/🟦️.ts";
import type {ValueError} from "@semio-tech/value-ts/refusal";
import type {NativeEncodeAllocationOutcome} from "../🛂️allocation/🟦️.ts";
export type {NativeEncodeAllocationOutcome} from "../🛂️allocation/🟦️.ts";
/** 🧮️ Keeps physical, local, work and allocation failures in their actual channels. */
export interface NativeEncodeEffectFailure{readonly primary:ValueError|null;readonly context:ValueError|null;readonly invariant:ValueError|null;readonly physicalInvariant:ValueError|null;readonly capacityInvariant:ValueError|null;readonly markerFailure:ValueError|null;readonly work:ValueError|null;readonly workOverrun:ValueError|null;readonly local:ValueError|null;readonly physical:ValueError|null;readonly physicalReceipt:OriginalReceivingPrimary|null;readonly allocation:NativeEncodeAllocationOutcome|null}
