import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🗓️ Fair global build scheduler core: pure, deterministic and event-sourced. Commands in, events out; state is the fold of the events, so any process (a client under a file lock, the host daemon) can host it. Admission is preemption-free: a started job is never stopped to make room.
 * @see ./🧬️schema/🔣️.json the closed schema of requests, events, snapshot, policy and EN+DE texts, ./🧪️tests/🐍️.py the independent oracle */
export type SchedulerLanguageV1="en"|"de";
export type SchedulerPriorityV1="background"|"agent"|"interactive";
export type SchedulerExitV1="ok"|"failed"|"cancelled"|"memory-exceeded"|"runner-lost"|"expired";
export type JobEstimateV1=Readonly<{class:string;floorMegabytes:number;learnedPeakMegabytes?:number;limitMegabytes:number}>;
export type JobRequestV1=Readonly<{id:string;agent:string;workspace:string;class:"plan"|"compile"|"test"|"generate";priority:SchedulerPriorityV1;depth:number;key:string;lane?:string;weight?:number;estimate:JobEstimateV1;deadlineMs?:number}>;
export type SchedulerPolicyV1=Readonly<typeof schema.definitions.Policy.const>;
export type SchedulerProgressV1=Readonly<{phase:string;done:number;total:number}>;
export type SchedulerSubscriberV1=Readonly<{ticket:string;agent:string;workspace:string;priority:SchedulerPriorityV1;depth:number}>;
export type SchedulerEventV1=
 |Readonly<{type:"submitted";at:number;jobId:string;request:JobRequestV1}>
 |Readonly<{type:"attached";at:number;jobId:string;subscriber:SchedulerSubscriberV1}>
 |Readonly<{type:"started";at:number;jobId:string;runner:string;reserveMegabytes:number}>
 |Readonly<{type:"heartbeat";at:number;ticket:string}>
 |Readonly<{type:"progressed";at:number;ticket:string;progress:SchedulerProgressV1}>
 |Readonly<{type:"cancelled";at:number;ticket:string}>
 |Readonly<{type:"lost";at:number;ticket:string}>
 |Readonly<{type:"expired";at:number;jobId:string}>
 |Readonly<{type:"completed";at:number;jobId:string;exit:SchedulerExitV1;observedPeakMegabytes?:number}>
 |Readonly<{type:"pruned";at:number}>;
type TicketV1={agent:string;workspace:string;priority:SchedulerPriorityV1;depth:number;lastSeen:number;order:number;state:"live"|"cancelled"|"lost"};
export type SchedulerJobV1={id:string;key:string;class:JobRequestV1["class"];estimateClass:string;lane?:string;owner:string;weight:number;priority:SchedulerPriorityV1;depth:number;reserveMegabytes:number;submittedAt:number;seq:number;deadlineMs?:number;status:"queued"|"running";tickets:Record<string,TicketV1>;runner?:string;startedAt?:number;progress?:SchedulerProgressV1;stopping?:boolean};
export type SchedulerFinishedV1=Readonly<{jobId:string;key:string;exit:SchedulerExitV1;atMs:number;tickets:readonly string[];estimateClass:string;observedPeakMegabytes?:number}>;
export type SchedulerStateV1={version:1;seq:number;virtualTime:number;jobs:Record<string,SchedulerJobV1>;ticketJob:Record<string,string>;keyJob:Record<string,string>;agents:Record<string,{vt:number;served:number;lastActive:number}>;finished:SchedulerFinishedV1[]};
export type SchedulerHostV1=Readonly<{budgetMegabytes:number;availableMegabytes?:number}>;
export type SchedulerSubmitOutcomeV1=Readonly<{kind:"queued"|"attached"|"busy"|"duplicate"|"invalid";jobId?:string;details?:string}>;
export type SchedulerTicketStateV1=Readonly<{status:"queued"|"running"|"finished"|"cancelled"|"unknown";role?:"runner"|"watcher";position?:number;exit?:SchedulerExitV1;progress?:SchedulerProgressV1;stopping?:boolean}>;
export type SchedulerSnapshotJobV1=Readonly<{jobId:string;key:string;class:JobRequestV1["class"];priority:SchedulerPriorityV1;depth:number;owner:string;agents:readonly string[];tickets:number;reserveMegabytes:number;waitedMs:number;position?:number;runMs?:number;progress?:SchedulerProgressV1;stopping?:boolean}>;
export type SchedulerSnapshotV1=Readonly<{version:1;atMs:number;hostBudgetMegabytes:number;reservedMegabytes:number;running:readonly SchedulerSnapshotJobV1[];queued:readonly SchedulerSnapshotJobV1[];perAgentShare:Readonly<Record<string,Readonly<{running:number;queued:number;served:number;share:number}>>>;finished:readonly SchedulerFinishedV1[]}>;

export const SCHEDULER_POLICY_V1:SchedulerPolicyV1=schema.definitions.Policy.const;
const messages=schema.definitions.Messages.const as Readonly<Record<string,Readonly<Record<SchedulerLanguageV1,string>>>>;

/** 🗣️ One text of the schema in the chosen language. */
export function schedulerTextV1(language:SchedulerLanguageV1,key:string,values:Readonly<Record<string,string|number>>={}):string{
 const row=messages[key];if(!row)throw Error(`scheduler-text-unknown:${key}`);
 return row[language].replace(/\{([A-Za-z]+)\}/gu,(match,name:string)=>name in values?String(values[name]):match);
}

/** 🌱 The empty state. */
export const emptySchedulerStateV1=():SchedulerStateV1=>({version:1,seq:0,virtualTime:0,jobs:{},ticketJob:{},keyJob:{},agents:{},finished:[]});
const live=(job:SchedulerJobV1):[string,TicketV1][]=>Object.entries(job.tickets).filter(([,ticket])=>ticket.state==="live");
const rank=(policy:SchedulerPolicyV1,priority:SchedulerPriorityV1):number=>policy.priorityBands.indexOf(priority);
const cost=(policy:SchedulerPolicyV1,reserveMegabytes:number):number=>Math.max(policy.minimumCostMilli,Math.floor(reserveMegabytes*1000/policy.referenceMegabytes));

function finish(policy:SchedulerPolicyV1,state:SchedulerStateV1,job:SchedulerJobV1,exit:SchedulerExitV1,at:number,observedPeakMegabytes?:number):void{
 for(const ticket of Object.keys(job.tickets))delete state.ticketJob[ticket];
 delete state.keyJob[job.key];delete state.jobs[job.id];
 state.finished.push({jobId:job.id,key:job.key,exit,atMs:at,tickets:Object.keys(job.tickets),estimateClass:job.estimateClass,...observedPeakMegabytes===undefined?{}:{observedPeakMegabytes}});
 if(state.finished.length>policy.maxFinished)state.finished.splice(0,state.finished.length-policy.maxFinished);
 const agent=state.agents[job.owner];if(agent)agent.lastActive=at;
}
function settle(policy:SchedulerPolicyV1,state:SchedulerStateV1,job:SchedulerJobV1,at:number,lostTicket?:string):void{
 if(job.status==="running"){
  if(job.runner!==undefined&&job.tickets[job.runner]!.state==="lost"&&job.runner===lostTicket)finish(policy,state,job,"runner-lost",at);
  else if(job.runner!==undefined&&job.tickets[job.runner]!.state==="cancelled")job.stopping=true;
 }else if(live(job).length===0)finish(policy,state,job,"cancelled",at);
}

/** ➕ Folds one event into the state in place; the only way a state changes. */
export function applySchedulerEventV1(policy:SchedulerPolicyV1,state:SchedulerStateV1,event:SchedulerEventV1):void{
 switch(event.type){
  case"submitted":{
   const request=event.request;state.seq++;
   state.jobs[event.jobId]={id:event.jobId,key:request.key,class:request.class,estimateClass:request.estimate.class,...request.lane===undefined?{}:{lane:request.lane},owner:request.agent,weight:request.weight??policy.defaultWeight,priority:request.priority,depth:request.depth,reserveMegabytes:request.estimate.limitMegabytes,submittedAt:event.at,seq:state.seq,...request.deadlineMs===undefined?{}:{deadlineMs:request.deadlineMs},status:"queued",tickets:{[request.id]:{agent:request.agent,workspace:request.workspace,priority:request.priority,depth:request.depth,lastSeen:event.at,order:state.seq,state:"live"}}};
   state.ticketJob[request.id]=event.jobId;state.keyJob[request.key]=event.jobId;
   const agent=state.agents[request.agent]??={vt:0,served:0,lastActive:event.at};agent.lastActive=event.at;
   return;
  }
  case"attached":{
   const job=state.jobs[event.jobId]!,subscriber=event.subscriber;
   job.tickets[subscriber.ticket]={agent:subscriber.agent,workspace:subscriber.workspace,priority:subscriber.priority,depth:subscriber.depth,lastSeen:event.at,order:++state.seq,state:"live"};
   state.ticketJob[subscriber.ticket]=event.jobId;
   if(rank(policy,subscriber.priority)>rank(policy,job.priority))job.priority=subscriber.priority;
   if(subscriber.depth>job.depth)job.depth=subscriber.depth;
   return;
  }
  case"started":{
   const job=state.jobs[event.jobId]!,agent=state.agents[job.owner]??={vt:0,served:0,lastActive:event.at},start=Math.max(agent.vt,state.virtualTime);
   job.status="running";job.runner=event.runner;job.startedAt=event.at;job.reserveMegabytes=event.reserveMegabytes;
   agent.vt=start+Math.floor(cost(policy,event.reserveMegabytes)*1000/job.weight);agent.served++;agent.lastActive=event.at;state.virtualTime=start;
   return;
  }
  case"heartbeat":{const id=state.ticketJob[event.ticket],ticket=id===undefined?undefined:state.jobs[id]?.tickets[event.ticket];if(ticket)ticket.lastSeen=event.at;return;}
  case"progressed":{const id=state.ticketJob[event.ticket],job=id===undefined?undefined:state.jobs[id];if(job){job.tickets[event.ticket]!.lastSeen=event.at;if(job.runner===event.ticket)job.progress=event.progress;}return;}
  case"cancelled":case"lost":{
   const id=state.ticketJob[event.ticket],job=id===undefined?undefined:state.jobs[id];if(!job)return;
   const ticket=job.tickets[event.ticket]!;if(ticket.state!=="live")return;
   ticket.state=event.type==="lost"?"lost":"cancelled";settle(policy,state,job,event.at,event.type==="lost"?event.ticket:undefined);return;
  }
  case"expired":{const job=state.jobs[event.jobId];if(job)finish(policy,state,job,"expired",event.at);return;}
  case"completed":{const job=state.jobs[event.jobId];if(job)finish(policy,state,job,event.exit,event.at,event.observedPeakMegabytes);return;}
  case"pruned":{
   const busy=new Set(Object.values(state.jobs).map(job=>job.owner)),idle=Object.entries(state.agents).filter(([name])=>!busy.has(name)).sort((a,b)=>a[1].lastActive-b[1].lastActive||(a[0]<b[0]?-1:1));
   let excess=Math.max(0,Object.keys(state.agents).length-policy.maxAgents);
   for(const[name,agent]of idle)if(excess>0||event.at-agent.lastActive>=policy.agingMs*10){delete state.agents[name];excess--;}
   return;
  }
 }
}

/** ⏪ Rebuilds the state from an event log. */
export function replaySchedulerV1(policy:SchedulerPolicyV1,events:readonly SchedulerEventV1[],from:SchedulerStateV1=emptySchedulerStateV1()):SchedulerStateV1{for(const event of events)applySchedulerEventV1(policy,from,event);return from;}

/** 🧪 Validates a job request against the closed rules of the schema; empty when valid. */
export function validateJobRequestV1(policy:SchedulerPolicyV1,request:JobRequestV1):string[]{
 const problems:string[]=[],estimate=request.estimate;
 if(!request.id||!request.agent||!request.workspace||!request.key)problems.push("identity");
 if(!policy.priorityBands.includes(request.priority))problems.push("priority");
 if(!Number.isSafeInteger(request.depth)||request.depth<0)problems.push("depth");
 if(!estimate||![estimate.floorMegabytes,estimate.learnedPeakMegabytes??0,estimate.limitMegabytes].every(value=>Number.isSafeInteger(value)&&value>=0))problems.push("estimate");
 return problems;
}

/** ➡️ Decides a submission: attach to the live job with the same key (dedupe), queue a new job, or refuse when the bounded queue is full. */
export function decideSubmitV1(policy:SchedulerPolicyV1,state:SchedulerStateV1,request:JobRequestV1,at:number):Readonly<{events:SchedulerEventV1[];outcome:SchedulerSubmitOutcomeV1}>{
 const problems=validateJobRequestV1(policy,request);
 if(problems.length>0)return{events:[],outcome:{kind:"invalid",details:problems.join(",")}};
 const known=state.ticketJob[request.id]??state.finished.find(entry=>entry.tickets.includes(request.id))?.jobId;
 if(known!==undefined)return{events:[],outcome:{kind:"duplicate",jobId:known}};
 const existing=state.keyJob[request.key],job=existing===undefined?undefined:state.jobs[existing];
 if(job){
  if(Object.keys(job.tickets).length>=policy.maxAttached)return{events:[],outcome:{kind:"busy",jobId:job.id}};
  return{events:[{type:"attached",at,jobId:job.id,subscriber:{ticket:request.id,agent:request.agent,workspace:request.workspace,priority:request.priority,depth:request.depth}}],outcome:{kind:"attached",jobId:job.id}};
 }
 if(Object.values(state.jobs).filter(row=>row.status==="queued").length>=policy.maxQueued)return{events:[],outcome:{kind:"busy"}};
 return{events:[{type:"submitted",at,jobId:request.id,request}],outcome:{kind:"queued",jobId:request.id}};
}

const effective=(policy:SchedulerPolicyV1,job:SchedulerJobV1,at:number):number=>Math.min(policy.priorityBands.length-1,rank(policy,job.priority)+Math.floor((at-job.submittedAt)/policy.agingMs));
const startTag=(state:SchedulerStateV1,job:SchedulerJobV1):number=>Math.max(state.agents[job.owner]?.vt??0,state.virtualTime);

function pick(policy:SchedulerPolicyV1,state:SchedulerStateV1,jobs:readonly SchedulerJobV1[],at:number):SchedulerJobV1|undefined{
 if(jobs.length===0)return undefined;
 const top=Math.max(...jobs.map(job=>effective(policy,job,at))),band=jobs.filter(job=>effective(policy,job,at)===top),least=Math.min(...band.map(job=>startTag(state,job)));
 return band.filter(job=>startTag(state,job)<=least+policy.fairnessSlack*1000).sort((a,b)=>b.depth-a.depth||startTag(state,a)-startTag(state,b)||a.seq-b.seq)[0];
}

/** ⏱️ Decides everything that is due at `at`: lost heartbeats, expired deadlines, then as many starts as fairness, priority, lanes and the host budget allow; stale agents are pruned last. */
export function decideTickV1(policy:SchedulerPolicyV1,source:SchedulerStateV1,at:number,host:SchedulerHostV1):SchedulerEventV1[]{
 const state=structuredClone(source),events:SchedulerEventV1[]=[],emit=(event:SchedulerEventV1):void=>{events.push(event);applySchedulerEventV1(policy,state,event);};
 for(const job of Object.values(source.jobs).sort((a,b)=>a.seq-b.seq))
  for(const[ticket,row]of Object.entries(job.tickets)){
   if(at-row.lastSeen<policy.heartbeatTimeoutMs)continue;
   if(row.state==="live")emit({type:"lost",at,ticket});
   else if(row.state==="cancelled"&&job.runner===ticket&&job.stopping)emit({type:"completed",at,jobId:job.id,exit:"runner-lost"});
  }
 for(const job of Object.values(state.jobs).sort((a,b)=>a.seq-b.seq))if(job.status==="queued"&&job.deadlineMs!==undefined&&at-job.submittedAt>=job.deadlineMs)emit({type:"expired",at,jobId:job.id});
 for(;;){
  const running=Object.values(state.jobs).filter(job=>job.status==="running"),reserved=running.reduce((sum,job)=>sum+job.reserveMegabytes,0);
  if(running.length>=policy.maxConcurrent)break;
  const lanes=policy.sharedCompilerLanes as Readonly<Record<string,number>>,eligible=Object.values(state.jobs).filter(job=>job.status==="queued"&&running.filter(row=>row.owner===job.owner).length<policy.maxPerAgent&&(job.lane===undefined||lanes[job.lane]===undefined||running.filter(row=>row.lane===job.lane).length<lanes[job.lane]!));
  const fits=(job:SchedulerJobV1):boolean=>running.length===0||reserved+job.reserveMegabytes<=host.budgetMegabytes&&(host.availableMegabytes===undefined||job.reserveMegabytes+policy.availableHeadroomMegabytes<=host.availableMegabytes);
  const head=pick(policy,state,eligible,at);if(!head)break;
  let chosen:SchedulerJobV1|undefined;
  if(fits(head))chosen=head;else if(at-head.submittedAt<policy.agingMs)chosen=pick(policy,state,eligible.filter(fits),at);
  if(!chosen)break;
  emit({type:"started",at,jobId:chosen.id,runner:live(chosen).sort((a,b)=>a[1].order-b[1].order)[0]![0],reserveMegabytes:chosen.reserveMegabytes});
 }
 const busy=new Set(Object.values(state.jobs).map(job=>job.owner)),agents=Object.entries(state.agents);
 if(agents.length>policy.maxAgents||agents.some(([name,agent])=>!busy.has(name)&&at-agent.lastActive>=policy.agingMs*10))emit({type:"pruned",at});
 return events;
}

/** 🎟️ What one ticket may see: waiting position, role, progress or the final exit. */
export function schedulerTicketStateV1(policy:SchedulerPolicyV1,state:SchedulerStateV1,ticket:string,at:number):SchedulerTicketStateV1{
 const id=state.ticketJob[ticket],job=id===undefined?undefined:state.jobs[id];
 if(job){
  const row=job.tickets[ticket]!;
  if(row.state==="cancelled"&&job.status==="queued")return{status:"cancelled"};
  const role=job.runner===undefined?undefined:job.runner===ticket?"runner":"watcher";
  if(job.status==="running")return{status:"running",...role?{role}:{},...job.progress?{progress:job.progress}:{},...job.stopping||row.state==="cancelled"?{stopping:true}:{}};
  const queued=orderQueue(policy,state,at);
  return{status:"queued",position:queued.findIndex(row2=>row2.id===job.id)+1};
 }
 const done=state.finished.findLast(entry=>entry.tickets.includes(ticket));
 return done?{status:"finished",exit:done.exit}:{status:"unknown"};
}

function orderQueue(policy:SchedulerPolicyV1,state:SchedulerStateV1,at:number):SchedulerJobV1[]{
 const rest=Object.values(state.jobs).filter(job=>job.status==="queued"),shadow=structuredClone(state),ordered:SchedulerJobV1[]=[];
 while(rest.length>0){
  const next=pick(policy,shadow,rest,at)!;ordered.push(next);rest.splice(rest.indexOf(next),1);
  const agent=shadow.agents[next.owner]??={vt:0,served:0,lastActive:at},start=Math.max(agent.vt,shadow.virtualTime);
  agent.vt=start+Math.floor(cost(policy,next.reserveMegabytes)*1000/next.weight);shadow.virtualTime=start;
 }
 return ordered;
}

/** 📸 The read model shown by `repo fleet status` and `repo scheduler status`. */
export function schedulerSnapshotV1(policy:SchedulerPolicyV1,state:SchedulerStateV1,at:number,host:SchedulerHostV1):SchedulerSnapshotV1{
 const row=(job:SchedulerJobV1,position?:number):SchedulerSnapshotJobV1=>({jobId:job.id,key:job.key,class:job.class,priority:job.priority,depth:job.depth,owner:job.owner,agents:[...new Set(Object.values(job.tickets).map(ticket=>ticket.agent))].sort(),tickets:Object.keys(job.tickets).length,reserveMegabytes:job.reserveMegabytes,waitedMs:Math.max(0,(job.startedAt??at)-job.submittedAt),...position===undefined?{}:{position},...job.startedAt===undefined?{}:{runMs:Math.max(0,at-job.startedAt)},...job.progress?{progress:job.progress}:{},...job.stopping?{stopping:true}:{}});
 const jobs=Object.values(state.jobs),running=jobs.filter(job=>job.status==="running").sort((a,b)=>a.seq-b.seq).map(job=>row(job)),queued=orderQueue(policy,state,at).map((job,index)=>row(job,index+1));
 const names=new Set([...Object.keys(state.agents),...jobs.map(job=>job.owner)]),total=Object.values(state.agents).reduce((sum,agent)=>sum+agent.served,0),perAgentShare:Record<string,{running:number;queued:number;served:number;share:number}>={};
 for(const name of[...names].sort()){const served=state.agents[name]?.served??0;perAgentShare[name]={running:jobs.filter(job=>job.owner===name&&job.status==="running").length,queued:jobs.filter(job=>job.owner===name&&job.status==="queued").length,served,share:total===0?0:Math.round(served*1000/total)/1000};}
 return{version:1,atMs:at,hostBudgetMegabytes:host.budgetMegabytes,reservedMegabytes:running.reduce((sum,job)=>sum+job.reserveMegabytes,0),running,queued,perAgentShare,finished:state.finished.slice(-16)};
}

/** 🖨️ Compact text of a snapshot in the chosen language. */
export function formatSchedulerSnapshotV1(language:SchedulerLanguageV1,snapshot:SchedulerSnapshotV1):string[]{
 if(snapshot.running.length===0&&snapshot.queued.length===0)return[schedulerTextV1(language,"idle")];
 const text=(key:string,values:Record<string,string|number>)=>schedulerTextV1(language,key,values),lines=[text("header",{running:snapshot.running.length,queued:snapshot.queued.length,reserved:snapshot.reservedMegabytes,budget:snapshot.hostBudgetMegabytes})];
 for(const job of snapshot.running)lines.push(text("runningRow",{key:job.key,agents:job.agents.join(","),reserve:job.reserveMegabytes,class:job.class,seconds:Math.floor((job.runMs??0)/1000),progress:job.progress?text("progress",job.progress):"",stopping:job.stopping?text("stopping",{}):""}));
 for(const job of snapshot.queued)lines.push(text("queuedRow",{position:job.position!,key:job.key,agents:job.agents.join(","),reserve:job.reserveMegabytes,class:job.class,priority:job.priority,depth:job.depth,seconds:Math.floor(job.waitedMs/1000)}));
 for(const[agent,share]of Object.entries(snapshot.perAgentShare))lines.push(text("share",{agent,running:share.running,queued:share.queued,served:share.served,percent:Math.round(share.share*100)}));
 return lines;
}

/** 🚢️ The facts `repo fleet status` reads from a snapshot (seconds instead of milliseconds, shares as percent). */
export function fleetFactOfSnapshotV1(snapshot:SchedulerSnapshotV1){
 const job=(row:SchedulerSnapshotJobV1)=>({key:row.key,class:row.class as string,priority:row.priority as string,depth:row.depth,agents:[...row.agents],tickets:row.tickets,reserveMegabytes:row.reserveMegabytes,waitedSeconds:Math.floor(row.waitedMs/1000),...row.position===undefined?{}:{position:row.position},...row.runMs===undefined?{}:{runSeconds:Math.floor(row.runMs/1000)},...row.progress?{progress:{...row.progress}}:{},...row.stopping?{stopping:true}:{}});
 return{budgetMegabytes:snapshot.hostBudgetMegabytes,reservedMegabytes:snapshot.reservedMegabytes,running:snapshot.running.map(job),queued:snapshot.queued.map(job),agents:Object.entries(snapshot.perAgentShare).map(([agent,share])=>({agent,running:share.running,queued:share.queued,served:share.served,percent:Math.round(share.share*100)}))};
}

/** 📈 What the job-limits lane learns from a batch of events: per finished job the estimate class and its observed peak (a killed job teaches `jobBreachLearnedPeakV1` of its outcome, already folded into `observedPeakMegabytes` by the runner). */
export type SchedulerLearnedPeakV1=Readonly<{class:string;peakMegabytes:number;exit:SchedulerExitV1}>;
export function schedulerLearnedPeaksV1(events:readonly SchedulerEventV1[],state:SchedulerStateV1):SchedulerLearnedPeakV1[]{
 return events.flatMap(event=>{
  if(event.type!=="completed"||event.observedPeakMegabytes===undefined)return[];
  const done=state.finished.findLast(entry=>entry.jobId===event.jobId);
  return done?[{class:done.estimateClass,peakMegabytes:event.observedPeakMegabytes,exit:event.exit}]:[];
 });
}
