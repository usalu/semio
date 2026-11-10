import {jobBreachLearnedPeakV1,jobPolicyForReservationV1,type JobLimitPolicyV1,type JobOutcomeV1} from "../🟦️.ts";

/** 🎫️ The limit of a host capacity grant: the memory it reserves is the memory its process tree may hold, so admission and enforcement are one number and an overrun fails this grant alone. */
export function grantJobLimitV1(grant:Readonly<{reservedMegabytes:number}>,options:Readonly<{cpuShares?:number;pids?:number}>={}):JobLimitPolicyV1{
 return jobPolicyForReservationV1(grant.reservedMegabytes,1,options);
}

/** 📈️ The run a grant reports to the learned estimates: a job stopped at its limit is recorded with the peak it needed (the limit grown by the breach growth), so the next reservation, and with it the next limit, is larger; any other outcome leaves the run as measured. */
export function learnedRunAfterOutcomeV1<Run extends Readonly<{peakMegabytes:number;status:string}>>(run:Run,outcome:JobOutcomeV1|undefined):Run{
 return outcome===undefined||outcome.kind!=="memory-exceeded"?run:{...run,peakMegabytes:Math.max(run.peakMegabytes,jobBreachLearnedPeakV1(outcome)),status:run.status==="cancelled"?run.status:"failed"};
}
