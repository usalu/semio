/** 🔭️ The units a read tells the fleet pool about: its own open owners first (tier 0, the ones it waits for) and then the owners of its next waves that are not among them (tier 1, inferred ahead by machines that have nothing of tier 0 left to take); an owner the read took already is left out of both, an owner named twice keeps its earliest place.
 * @see ../🟦️.ts */
export function ownerPoolLookaheadV1<Job extends Readonly<{name:string}>>(input:Readonly<{wave:readonly Job[];ahead:readonly Job[];taken:ReadonlySet<string>}>):readonly Readonly<{job:Job;tier:0|1}>[]{
 const seen=new Set<string>(),rows:{job:Job;tier:0|1}[]=[];
 for(const [list,tier] of[[input.wave,0],[input.ahead,1]] as const)for(const job of list){
  if(input.taken.has(job.name)||seen.has(job.name))continue;
  seen.add(job.name);rows.push({job,tier});
 }
 return rows;
}
