import type {DoctorProbeV1,DoctorRepairV1} from "./📋️contract/🟦️.ts";
import {toolsProbeV1} from "./🔧️tools/🟦️.ts";
import {storageProbeV1} from "./💾️storage/🟦️.ts";
import {leasesProbeV1} from "./🔒️leases/🟦️.ts";
import {capacityProbeV1} from "./🎫️capacity/🟦️.ts";
import {sessionsProbeV1} from "./🪪️sessions/🟦️.ts";
import {graphProbeV1} from "./🕸️graph/🟦️.ts";
import {dependenciesProbeV1} from "./📦️dependencies/🟦️.ts";
import {cachesProbeV1} from "./⚡️caches/🟦️.ts";
import {processesProbeV1} from "./🏃️processes/🟦️.ts";

/** 🗂️ Every probe in the order the report lists them; phase 2 needs the pids phase 1 found. */
export const DOCTOR_PROBES_V1:readonly DoctorProbeV1[]=Object.freeze([toolsProbeV1,storageProbeV1,leasesProbeV1,capacityProbeV1,sessionsProbeV1,graphProbeV1,dependenciesProbeV1,cachesProbeV1,processesProbeV1]);
/** 🧰️ Every declared repair by id. */
export const DOCTOR_REPAIRS_V1:ReadonlyMap<string,DoctorRepairV1>=new Map(DOCTOR_PROBES_V1.flatMap(probe=>(probe.repairs??[]).map(repair=>[repair.id,repair] as const)));
