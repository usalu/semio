/** 🪦️ The declared shape of a per-process producer publication and its quiet time. */
export const PRODUCER_PUBLICATION_V1: Readonly<{prefix:string;quietMs:number}>;
/** 🧟️ Asks the operating system whether a process still exists; a process that may not be signalled still exists. */
export function producerAliveV1(pid:number):boolean;
/** 🏚️ Retires the calling process's own producer publication folder when the process ends. */
export function retireProducerPublicationAtExitV1(folder:string):void;
/** 📋️ What one sweep of the producer publications of gone processes found and did. */
export type ProducerPublicationSweepV1=Readonly<{candidates:readonly string[];removed:number;kept:number;bytes:number;cancelled:boolean}>;
/** 🧹️ Sweeps the producer publications of processes that are gone below every selection of `selections`. */
export function sweepDeadProducerPublicationsV1(selections:string,ports?:Readonly<{alive?:(pid:number)=>boolean;now?:()=>number;quietMs?:number;dryRun?:boolean;signal?:AbortSignal}>):Promise<ProducerPublicationSweepV1>;
