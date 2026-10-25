import type {DoctorAreaV1,DoctorFactsV1,DoctorPolicyV1} from "../../🟦️.ts";
import type {DoctorPortsV1} from "../../🔌️ports/🟦️.ts";

/** 🧩️ The sections of the facts a probe may fill. */
export type DoctorSectionsV1=Partial<Pick<DoctorFactsV1,"host"|"storage"|"tools"|"leases"|"capacity"|"sessions"|"graph"|"dependencies"|"caches"|"processes">>;
/** 🎛️ What a probe knows besides the ports: the policy, the processes earlier probes found holding or waiting, where the library sources live and when it must stop. */
export type DoctorProbeContextV1=Readonly<{policy:DoctorPolicyV1;known:ReadonlySet<number>;libraryRoot:string;deadlineMs:number}>;
/** 📥️ What a probe returns: facts, or the reason there is nothing to read here. */
export type DoctorProbeResultV1=Readonly<{sections:DoctorSectionsV1}>|Readonly<{unavailable:string}>;
/** 🛠️ What a safe repair did, or would do in a dry run. */
export type DoctorRepairRunV1=Readonly<{removed:number;bytes:number;status?:"nothing"|"unavailable";reason?:string}>;
/** 🔧️ One declared safe repair; it re-derives its targets from the live state and never trusts an earlier report. */
export type DoctorRepairV1=Readonly<{id:string;area:DoctorAreaV1;run(ports:DoctorPortsV1,context:DoctorProbeContextV1,dryRun:boolean):Promise<DoctorRepairRunV1>}>;
/** 🔎️ One probe: it reads the live state of one area through the ports and never changes it. */
export type DoctorProbeV1=Readonly<{area:DoctorAreaV1;phase:1|2;read(ports:DoctorPortsV1,context:DoctorProbeContextV1):Promise<DoctorProbeResultV1>;repairs?:readonly DoctorRepairV1[]}>;
