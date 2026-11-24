import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";

export type OwnerPoolLifeV1=Readonly<{startedMs:number;ready:boolean;running:number;drained:boolean}>;

/** 🚦️ Whether the pool may start another child now: never while its live children (draining ones and the survivors of earlier pools included) already reach the target, never while a child of its own that is not draining still has no owner in flight and has not had `childMinimumAgePolls` looks to claim one (more children do not make an idle one work), and a child that has not said `ready` yet (it is still loading under a loaded machine) counts as starting for `startWarmupPolls` pool looks, at most `startingMaximum` start at once and two starts are `startSpacingPolls` pool looks apart, so a pool never fans out eleven children that each take minutes to open and are drained before their first owner.
 * @see ../🧪️tests/🧮️lifecycle/🟦️.ts */
export function ownerPoolMayStartV1(input:Readonly<{children:readonly OwnerPoolLifeV1[];survivors:number;target:number;ramp?:boolean;lastStartMs:number;now:number;pollMilliseconds:number}>,policy:typeof P=P):boolean{
 if(input.children.length+input.survivors>=input.target)return false;
 if(input.ramp===false)return true;
 if(input.children.some(child=>!child.drained&&child.running===0&&input.now-child.startedMs<policy.childMinimumAgePolls*input.pollMilliseconds))return false;
 if(input.now-input.lastStartMs<policy.startSpacingPolls*input.pollMilliseconds)return false;
 return input.children.filter(child=>!child.ready&&!child.drained&&input.now-child.startedMs<policy.startWarmupPolls*input.pollMilliseconds).length<policy.startingMaximum;
}

/** 🚰️ The child to drain when there are more than the target: under `pressure` any child (memory is short), otherwise only a child that is at least `childMinimumAgePolls` pool looks old (a look is `targetPollMilliseconds`, 2 s) (a child younger than that has not made its start pay yet), and among the candidates one that runs no owner first, then the newest. */
export function ownerPoolDrainPickV1<C extends OwnerPoolLifeV1>(input:Readonly<{children:readonly C[];reason:string|null;now:number;pollMilliseconds:number}>,policy:typeof P=P):C|undefined{
 const candidates=input.children.filter(child=>!child.drained&&(input.reason==="pressure"||input.now-child.startedMs>=policy.childMinimumAgePolls*input.pollMilliseconds));
 return[...candidates].sort((a,b)=>Math.sign(a.running)-Math.sign(b.running)||b.startedMs-a.startedMs)[0];
}

/** ⏳️ How long until the pool may start another child although the target asks for one: the longest of the spacing since the last start, the minimum age of a child that still has no owner and, while the allowed number of children is loading, the shortest warm-up left; `0` when a start is allowed now. The number the pool states when fewer children run than the target.
 * @see ../🧪️tests/🧮️lifecycle/🟦️.ts */
export function ownerPoolStartDelayV1(input:Readonly<{children:readonly OwnerPoolLifeV1[];lastStartMs:number;now:number;pollMilliseconds:number}>,policy:typeof P=P):number{
 const spacing=policy.startSpacingPolls*input.pollMilliseconds-(input.now-input.lastStartMs),minimumAge=policy.childMinimumAgePolls*input.pollMilliseconds,warmup=policy.startWarmupPolls*input.pollMilliseconds;
 const young=input.children.filter(child=>!child.drained&&child.running===0&&input.now-child.startedMs<minimumAge).map(child=>minimumAge-(input.now-child.startedMs));
 const loading=input.children.filter(child=>!child.ready&&!child.drained&&input.now-child.startedMs<warmup).map(child=>warmup-(input.now-child.startedMs));
 return Math.max(0,spacing,young.length===0?0:Math.max(...young),loading.length<policy.startingMaximum?0:Math.min(...loading));
}

export type OwnerPoolSharelessV1=Readonly<{count:number;until:number}>;

/** 🚫️ Whether a machine may keep claiming owners at all: a machine whose finished records cannot be shared (code `unshareable`, a record that names its root in a spelling that cannot be neutralized, as a Windows checkout can) burns every claim it makes, so after `unshareableLimit` such refusals without a published record in between it claims nothing for `unshareablePauseMilliseconds` (`until` is the end of the pause, 0 when none runs) and then probes again with the next refusal tripping it at once. `tripped` is true on the step that starts a pause.
 * @see ../🧪️tests/🧮️lifecycle/🚫️unshareable/🟦️.ts */
export function ownerPoolSharelessStepV1(state:OwnerPoolSharelessV1,event:"unshareable"|"published"|"tick",now:number,policy:typeof P=P):Readonly<{state:OwnerPoolSharelessV1;tripped:boolean}>{
 if(event==="published")return{state:{count:0,until:state.until},tripped:false};
 if(event==="tick")return{state:state.until>0&&now>=state.until?{count:policy.unshareableLimit-1,until:0}:state,tripped:false};
 const count=state.count+1;
 if(count>=policy.unshareableLimit&&state.until===0)return{state:{count,until:now+policy.unshareablePauseMilliseconds},tripped:true};
 return{state:{count,until:state.until},tripped:false};
}
