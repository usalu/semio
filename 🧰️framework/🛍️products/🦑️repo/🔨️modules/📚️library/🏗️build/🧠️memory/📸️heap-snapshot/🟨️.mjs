import{mkdirSync,readFileSync,renameSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📸️ The declared policy of the slice heap diagnostic: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const HEAP_SNAPSHOT_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

/** 🗂️ The control file of the diagnostic for a service that starts without the environment of its caller (a launchd service): `{directory,thresholdMegabytes}` in the machine directory of the repo tools; its fields stand in for the missing environment variables. A missing or unreadable file is no control. */
export function heapSnapshotControlV1({directory=join(tmpdir(),"semio-repo",String(process.getuid?.()??"")),read=readFileSync}={},policy=HEAP_SNAPSHOT_POLICY_V1){
 try{const value=JSON.parse(String(read(join(directory,policy.controlFile),"utf8")));return value!==null&&typeof value==="object"?value:undefined;}catch{return undefined;}
}

/** ⚖️ Whether a worker with `heapBytes` writes its heap summary now: only when the directory is named (environment first, then `control`), only once, and only above the threshold (the declared default where the threshold is empty or `1`). */
export function heapSnapshotDecisionV1({environment=process.env,heapBytes,taken=false,control},policy=HEAP_SNAPSHOT_POLICY_V1){
 if(taken||!Number.isFinite(heapBytes))return undefined;
 const named=environment[policy.directoryEnvironment],directory=typeof named==="string"&&named.length>0?named:control?.directory;
 if(typeof directory!=="string"||directory.length===0)return undefined;
 const asked=Number(environment[policy.thresholdEnvironment]??control?.thresholdMegabytes),limit=Number.isFinite(asked)&&asked>1?asked:policy.defaultThresholdMegabytes;
 return heapBytes/1048576>limit?Object.freeze({directory,thresholdMegabytes:limit,heapMegabytes:Math.round(heapBytes/1048576)}):undefined;
}

/** 🔬️ What a heap snapshot of the inspector format holds, reduced to the rows that name a retainer: the classes by count and bytes, and the edges grouped as retaining class, edge name and retained class with the bytes they retain directly. The first incoming edge of each node gives its retainer path (`pathDepth` levels, grouped with the bytes they hold, and the largest single objects with two more levels). Nodes are four numbers (id, size, class index, flags), edges four numbers (from id, to id, type index, name or index). */
export function summarizeHeapSnapshotV1(snapshot,policy=HEAP_SNAPSHOT_POLICY_V1){
 const {nodes,nodeClassNames,edges,edgeNames,edgeTypes}=snapshot;let highest=0;
 for(let at=0;at<nodes.length;at+=4)if(nodes[at]>highest)highest=nodes[at];
 const classOf=new Uint32Array(highest+1),sizeOf=new Uint32Array(highest+1),classes=new Map();let totalBytes=0;
 for(let at=0;at<nodes.length;at+=4){
  const id=nodes[at],size=nodes[at+1],name=nodeClassNames[nodes[at+2]];
  classOf[id]=nodes[at+2];sizeOf[id]=size;totalBytes+=size;
  const row=classes.get(name)??{name,count:0,bytes:0};row.count++;row.bytes+=size;classes.set(name,row);
 }
 const retainers=new Map(),parentOf=new Uint32Array(highest+1),parentEdge=new Array(highest+1);
 for(let at=0;at<edges.length;at+=4){
  const from=edges[at],to=edges[at+1];if(from>highest||to>highest)continue;
  if(parentOf[to]===0&&from!==0&&from!==to){parentOf[to]=from;const kind=edgeTypes[edges[at+2]];parentEdge[to]=kind==="Property"||kind==="Variable"?edgeNames[edges[at+3]]:kind==="Index"?"[]":"~";}
  const type=edgeTypes[edges[at+2]],name=type==="Property"||type==="Variable"?edgeNames[edges[at+3]]:type,key=nodeClassNames[classOf[from]]+"."+name+" -> "+nodeClassNames[classOf[to]];
  const row=retainers.get(key)??{retainer:key,count:0,bytes:0};row.count++;row.bytes+=sizeOf[to];retainers.set(key,row);
 }
 const chain=(id,depth)=>{const parts=[nodeClassNames[classOf[id]]];for(let at=id,level=0;level<depth&&parentOf[at]!==0;level++){parts.push(parentEdge[at]+" <- "+nodeClassNames[classOf[parentOf[at]]]);at=parentOf[at];}return parts.join(" ");};
 const paths=new Map(),biggest=[];
 for(let at=0;at<nodes.length;at+=4){
  const id=nodes[at],size=nodes[at+1];if(id>highest||id===0)continue;
  const key=chain(id,policy.pathDepth),row=paths.get(key)??{path:key,count:0,bytes:0};row.count++;row.bytes+=size;paths.set(key,row);
  if(biggest.length<policy.topRows||size>biggest[biggest.length-1].bytes){biggest.push({bytes:size,path:chain(id,policy.pathDepth+2)});biggest.sort((a,b)=>b.bytes-a.bytes);if(biggest.length>policy.topRows)biggest.pop();}
 }
 const top=rows=>[...rows.values()].sort((a,b)=>b.bytes-a.bytes||b.count-a.count).slice(0,policy.topRows);
 return{version:1,nodes:nodes.length/4,edges:edges.length/4,totalBytes,classes:top(classes),retainers:top(retainers),retainerPaths:top(paths),largestObjects:biggest};
}

/** 💾️ Takes the heap snapshot of this process, reduces it and writes the summary as `<prefix><pid>-<milliseconds>.json` into `directory` (through a temporary file); answers the path. `generate` is the snapshot source (the runtime's by default). */
export function writeHeapSummaryV1({directory,pid=process.pid,now=Date.now(),generate=()=>globalThis.Bun.generateHeapSnapshot(),extra={}},policy=HEAP_SNAPSHOT_POLICY_V1){
 const summary=summarizeHeapSnapshotV1(generate(),policy);
 mkdirSync(directory,{recursive:true});
 const path=join(directory,policy.filePrefix+pid+"-"+now+".json"),temporary=path+".tmp";
 writeFileSync(temporary,JSON.stringify({...extra,pid,takenAtMilliseconds:now,...summary},null,1));renameSync(temporary,path);
 return path;
}
