import{mkdirSync,renameSync,writeFileSync}from"node:fs";
import{join}from"node:path";

/** 📏️ The declared numbers of the heap profile: the managed megabytes from which one snapshot is summarised, how many retainers and classes the summary names, how many hops of a path it prints. */
export const HEAP_PROFILE_POLICY_V1=Object.freeze({minimumManagedMegabytes:1200,retainers:12,classes:12,hops:14});
const P=HEAP_PROFILE_POLICY_V1,STRIDE=4,VARIABLE=3,PROPERTY=1,INDEX=2;

/** 🔬️ Summarises a JavaScriptCore heap snapshot (`Bun.generateHeapSnapshot()`): the classes by self bytes and count, and the retainers, the deepest nodes of the heavy chains of the breadth-first retention tree with their retained bytes, string count and the path from the root. A breadth-first tree is a cheap upper bound of the dominator tree: a node reachable by several paths is charged to the shortest one.
 * @see https://github.com/oven-sh/bun/blob/main/docs/project/benchmarking.md heap snapshots of Bun */
export function summariseHeapSnapshotV1(snapshot,{retainers=P.retainers,classes=P.classes,hops=P.hops}={}){
 const{nodes,edges,nodeClassNames,edgeNames,edgeTypes}=snapshot,count=nodes.length/STRIDE,total=edges.length/STRIDE,index=new Map();
 for(let at=0;at<count;at++)index.set(nodes[at*STRIDE],at);
 const first=new Uint32Array(count+1);
 for(let at=0;at<total;at++){const from=index.get(edges[at*STRIDE]);if(from!==undefined)first[from+1]++;}
 for(let at=0;at<count;at++)first[at+1]+=first[at];
 const fill=first.slice(0,count),targets=new Uint32Array(total),labels=new Int32Array(total).fill(-1);let used=0;
 for(let at=0;at<total;at++){const from=index.get(edges[at*STRIDE]),to=index.get(edges[at*STRIDE+1]);if(from===undefined||to===undefined)continue;const slot=fill[from]++;targets[slot]=to;const type=edgeTypes[edges[at*STRIDE+2]],data=edges[at*STRIDE+3];labels[slot]=type==="Index"?-2:type==="Property"||type==="Variable"?data:-3;used++;}
 const parent=new Int32Array(count).fill(-1),via=new Int32Array(count).fill(-3),seen=new Uint8Array(count),order=[];
 seen[0]=1;order.push(0);
 for(let head=0;head<order.length;head++){const node=order[head];for(let slot=first[node];slot<first[node+1];slot++){const to=targets[slot];if(seen[to])continue;seen[to]=1;parent[to]=node;via[to]=labels[slot];order.push(to);}}
 const className=at=>nodeClassNames[nodes[at*STRIDE+2]],retained=new Float64Array(count),strings=new Float64Array(count),perClass=new Map();
 for(const at of order){const size=nodes[at*STRIDE+1],name=className(at),row=perClass.get(name)??{name,count:0,bytes:0};row.count++;row.bytes+=size;perClass.set(name,row);retained[at]=size;strings[at]=name==="string"?1:0;}
 for(let at=order.length-1;at>0;at--){const node=order[at],up=parent[node];retained[up]+=retained[node];strings[up]+=strings[node];}
 const heavy=[];
 for(const node of order){let best=0;for(let slot=first[node];slot<first[node+1];slot++){const to=targets[slot];if(parent[to]===node&&retained[to]>best)best=retained[to];}if(best*2<retained[node]&&retained[node]>retained[0]*0.01)heavy.push(node);}
 heavy.sort((a,b)=>retained[b]-retained[a]);
 const label=at=>{const edge=via[at];return(edge>=0?edgeNames[edge]:edge===-2?"[]":"~")+":"+className(at);};
 const pathOf=at=>{const out=[];for(let node=at,step=0;node>0&&step<hops;node=parent[node],step++)out.push(label(node));return out.reverse().join(" > ");};
 return Object.freeze({nodes:count,edges:used,totalBytes:retained[0],classes:[...perClass.values()].sort((a,b)=>b.bytes-a.bytes).slice(0,classes),retainers:heavy.slice(0,retainers).map(at=>({bytes:retained[at],strings:strings[at],path:pathOf(at)}))});
}

/** 📸️ Takes one snapshot of this process when the managed megabytes passed the policy minimum and `directory` is given (environment `SEMIO_GRAPH_HEAP_PROFILE`), writes the summary there as JSON (temp and rename) and answers it; once per process, quiet otherwise. */
let takenV1=false;
export function profileHeapOnceV1({directory=process.env.SEMIO_GRAPH_HEAP_PROFILE,managedMegabytes,generate=()=>globalThis.Bun.generateHeapSnapshot(),name="heap-profile-"+process.pid+".json",minimum=P.minimumManagedMegabytes}={}){
 if(takenV1||!directory||!(managedMegabytes>=minimum))return undefined;
 takenV1=true;
 try{
  const summary=summariseHeapSnapshotV1(generate());
  mkdirSync(directory,{recursive:true});const temp=join(directory,name+".tmp");writeFileSync(temp,JSON.stringify({managedMegabytes,...summary},null,1));renameSync(temp,join(directory,name));
  return summary;
 }catch{return undefined;}
}
