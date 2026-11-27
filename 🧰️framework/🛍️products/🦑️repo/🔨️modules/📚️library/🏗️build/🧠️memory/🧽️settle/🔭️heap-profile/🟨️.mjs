import{mkdirSync,renameSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const freeze=value=>{if(value!==null&&typeof value==="object"){for(const child of Object.values(value))freeze(child);Object.freeze(value);}return value;};

/** 📏️ The declared numbers of the heap profile: the schema file is the single source (first snapshot from `minimumManagedMegabytes`, a further one per `stepManagedMegabytes` more, at most `maxSnapshots` and `budgetMilliseconds` of snapshot time per process; how many retainers, classes and holder groups a summary names, how many hops of a path it prints and how deep a holder chain reads). */
export const HEAP_PROFILE_POLICY_V1=freeze(structuredClone(schema.definitions.Policy.const));
const P=HEAP_PROFILE_POLICY_V1,STRIDE=4,INTERNAL=-3,ELEMENT=-2,CLIMB=4,TICK=1<<20;

/** 🛑️ Thrown inside a summary when the caller's signal aborted. */
class HeapProfileAbortedV1 extends Error{}

/** 🔬️ Summarises a JavaScriptCore heap snapshot (`Bun.generateHeapSnapshot()`): the classes by self bytes and count, the retainers (the deepest nodes of the heavy chains of the breadth-first retention tree with their retained bytes, string count and path from the root) and, for the focus classes (strings, arrays, objects), the HOLDER GROUPS: how many nodes of a class hang from the same retaining edge name of the same owner class, by the chain of the nearest named owners above it (`Object.carry < Object.entry < ...`), with an example path from the root. A breadth-first tree is a cheap upper bound of the dominator tree: a node reachable by several paths is charged to the shortest one.
 * @param {unknown} snapshot
 * @param {{retainers?:number;classes?:number;hops?:number;groups?:number;depth?:number;focus?:readonly string[];signal?:AbortSignal}} [options]
 * @see https://github.com/oven-sh/bun/blob/main/docs/project/benchmarking.md heap snapshots of Bun */
export function summariseHeapSnapshotV1(snapshot,{retainers=P.retainers,classes=P.classes,hops=P.hops,groups=P.groups,depth=P.depth,focus=P.focus,signal}={}){
 const{nodes,edges,nodeClassNames,edgeNames,edgeTypes}=snapshot,count=nodes.length/STRIDE,total=edges.length/STRIDE;
 let ticks=0;const tick=()=>{if((++ticks&(TICK-1))===0&&signal?.aborted)throw new HeapProfileAbortedV1();};
 let highest=0;
 for(let at=0;at<count;at++){const id=nodes[at*STRIDE];if(id>highest)highest=id;tick();}
 const dense=highest<=count*8&&highest<=0x3fffffff,table=dense?new Int32Array(highest+1).fill(-1):undefined,sparse=dense?undefined:new Map();
 for(let at=0;at<count;at++){if(dense)table[nodes[at*STRIDE]]=at;else sparse.set(nodes[at*STRIDE],at);tick();}
 const slotOf=id=>dense?(id<=highest?table[id]:-1):(sparse.get(id)??-1);
 const first=new Uint32Array(count+1),fromOf=new Int32Array(total),toOf=new Int32Array(total);
 for(let at=0;at<total;at++){const from=slotOf(edges[at*STRIDE]),to=slotOf(edges[at*STRIDE+1]);fromOf[at]=from;toOf[at]=to;if(from>=0&&to>=0)first[from+1]++;tick();}
 for(let at=0;at<count;at++)first[at+1]+=first[at];
 const fill=first.slice(0,count),targets=new Uint32Array(first[count]),labels=new Int32Array(first[count]).fill(INTERNAL);let used=0;
 for(let at=0;at<total;at++){const from=fromOf[at],to=toOf[at];if(from<0||to<0)continue;const slot=fill[from]++;targets[slot]=to;const type=edgeTypes[edges[at*STRIDE+2]],data=edges[at*STRIDE+3];labels[slot]=type==="Index"?ELEMENT:type==="Property"||type==="Variable"?data:INTERNAL;used++;tick();}
 const parent=new Int32Array(count).fill(-1),via=new Int32Array(count).fill(INTERNAL),seen=new Uint8Array(count),order=[];
 seen[0]=1;order.push(0);
 for(let head=0;head<order.length;head++){const node=order[head];for(let slot=first[node];slot<first[node+1];slot++){const to=targets[slot];if(seen[to])continue;seen[to]=1;parent[to]=node;via[to]=labels[slot];order.push(to);}tick();}
 const className=at=>nodeClassNames[nodes[at*STRIDE+2]],retained=new Float64Array(count),strings=new Float64Array(count),perClass=new Map();
 for(const at of order){const size=nodes[at*STRIDE+1],name=className(at),row=perClass.get(name)??{name,count:0,bytes:0};row.count++;row.bytes+=size;perClass.set(name,row);retained[at]=size;strings[at]=name==="string"?1:0;tick();}
 for(let at=order.length-1;at>0;at--){const node=order[at],up=parent[node];retained[up]+=retained[node];strings[up]+=strings[node];tick();}
 const heavy=[];
 for(const node of order){let best=0;for(let slot=first[node];slot<first[node+1];slot++){const to=targets[slot];if(parent[to]===node&&retained[to]>best)best=retained[to];}if(best*2<retained[node]&&retained[node]>retained[0]*0.01)heavy.push(node);tick();}
 heavy.sort((a,b)=>retained[b]-retained[a]);
 const edgeLabel=edge=>edge>=0?edgeNames[edge]:edge===ELEMENT?"[]":"~";
 const label=at=>edgeLabel(via[at])+":"+className(at);
 const pathOf=at=>{const out=[];for(let node=at,step=0;node>0&&step<hops;node=parent[node],step++)out.push(label(node));return out.reverse().join(" > ");};
 const chainIds=new Map(),chainTexts=[],holderId=new Int32Array(count).fill(-1),radix=edgeNames.length+3;
 const holderOf=holder=>{
  if(holderId[holder]>=0)return holderId[holder];
  const names=[];let last="";
  for(let node=holder,raw=0;node>0&&names.length<depth&&raw<depth*CLIMB;node=parent[node],raw++){const edge=via[node];if(edge===INTERNAL)continue;const text=className(parent[node])+"."+edgeLabel(edge);if(text!==last)names.push(text);last=text;}
  const text=className(holder)+" < "+names.join(" < ");let id=chainIds.get(text);
  if(id===undefined){id=chainTexts.length;chainIds.set(text,id);chainTexts.push(text);}
  holderId[holder]=id;return id;
 };
 const wanted=new Set(focus),tally=new Map();
 for(const node of order){
  if(node===0||!wanted.has(className(node))){tick();continue;}
  const key=holderOf(parent[node])*radix+via[node]+3,row=tally.get(key)??{key,class:className(node),count:0,bytes:0,example:node};
  row.count++;row.bytes+=nodes[node*STRIDE+1];tally.set(key,row);tick();
 }
 const named=row=>{const holder=Math.floor(row.key/radix),edge=row.key-holder*radix-3;return{class:row.class,edge:edgeLabel(edge),holder:chainTexts[holder],count:row.count,bytes:row.bytes,path:pathOf(row.example)};};
 const holders={};
 for(const name of focus){const rows=[...tally.values()].filter(row=>row.class===name);holders[name]=rows.sort((a,b)=>b.count-a.count||b.bytes-a.bytes).slice(0,groups).map(named);}
 return Object.freeze({nodes:count,edges:used,totalBytes:retained[0],classes:[...perClass.values()].sort((a,b)=>b.bytes-a.bytes).slice(0,classes),retainers:heavy.slice(0,retainers).map(at=>({bytes:retained[at],strings:strings[at],path:pathOf(at)})),holders});
}

const runtimeCountersV1=()=>{try{const stats=process.getBuiltinModule?.("bun:jsc")?.heapStats?.();return{...process.memoryUsage(),heapCapacity:stats?.heapCapacity,protectedObjects:stats?.protectedObjectCount};}catch{return process.memoryUsage();}};

/** 📸️ The snapshots one process took: how many, the managed megabytes the next one waits for and the milliseconds all of them cost. */
export function createHeapProfileStateV1(){return{taken:0,next:undefined,milliseconds:0,busy:false};}
const processStateV1=createHeapProfileStateV1();

/** 📸️ Takes a snapshot of this process each time the managed megabytes passed a further step of the policy (the first at the minimum, then every `stepManagedMegabytes` more, at most `maxSnapshots` and `budgetMilliseconds` of snapshot time), when `directory` is given (environment `SEMIO_GRAPH_HEAP_PROFILE`). It writes the summary (heap counters of the process next to the snapshot total, so memory the snapshot cannot reach shows as a gap, plus the cost) there as JSON (temp and rename) and answers it; answers `undefined` when no step was due, a snapshot was running, the budget is spent, the signal aborted or the snapshot failed.
 * @param {{directory?:string;managedMegabytes?:number;generate?:()=>unknown;name?:(sequence:number)=>string;minimum?:number;step?:number;limit?:number;budget?:number;state?:ReturnType<typeof createHeapProfileStateV1>;signal?:AbortSignal;now?:()=>number;memory?:()=>Record<string,number>}} [options] */
export function profileHeapV1({directory=process.env.SEMIO_GRAPH_HEAP_PROFILE,managedMegabytes,generate=()=>globalThis.Bun.generateHeapSnapshot(),name=sequence=>"heap-profile-"+process.pid+"-"+sequence+".json",minimum=P.minimumManagedMegabytes,step=P.stepManagedMegabytes,limit=P.maxSnapshots,budget=P.budgetMilliseconds,state=processStateV1,signal,now=()=>performance.now(),memory=runtimeCountersV1}={}){
 if(!directory||!(managedMegabytes>=(state.next??minimum))||state.taken>=limit||state.milliseconds>=budget||state.busy||signal?.aborted)return undefined;
 state.busy=true;
 try{
  const sequence=state.taken+1,started=now(),snapshot=generate(),generated=now(),summary=summariseHeapSnapshotV1(snapshot,{signal}),finished=now(),counters=memory();
  state.taken=sequence;state.next=managedMegabytes+step;state.milliseconds+=finished-started;
  const megabytes=bytes=>Math.round(bytes/1048576),counted={heapUsedMegabytes:megabytes(counters.heapUsed??0),externalMegabytes:megabytes(counters.external??0),arrayBuffersMegabytes:megabytes(counters.arrayBuffers??0),residentMegabytes:megabytes(counters.rss??0),collectorCapacityMegabytes:megabytes(counters.heapCapacity??0),protectedObjects:counters.protectedObjects??0,snapshotMegabytes:megabytes(summary.totalBytes)};
  const record={sequence,managedMegabytes,process:{...counted,unreachedMegabytes:Math.max(0,managedMegabytes-counted.snapshotMegabytes)},cost:{generateMilliseconds:Math.round(generated-started),summariseMilliseconds:Math.round(finished-generated)},...summary};
  mkdirSync(directory,{recursive:true});const file=name(sequence),temp=join(directory,file+".tmp");writeFileSync(temp,JSON.stringify(record,null,1));renameSync(temp,join(directory,file));
  return summary;
 }catch(error){if(!(error instanceof HeapProfileAbortedV1)){state.taken++;state.next=managedMegabytes+step;}return undefined;}
 finally{state.busy=false;}
}
