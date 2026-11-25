/** 📏️ The child of the peak law: builds a worker-size sealed read in the shapes of the observation layer (lstat, exists, readdir, file and analysis rows with shared nested rows, under a root of emoji path segments like the workspace), collects, then publishes it through `streamSelectedDocumentsV1` while sampling its own resident size every 20 ms, and prints one JSON line.
 * @see ./🟦️.ts */
import{streamSelectedDocumentsV1}from"../🟦️.ts";

const MB=1048576,root="/Users/ci/🗂️semio",host={root,windows:false},owners=2_800,distinct=270_000,perEntry=350;
const owner=(at:number)=>"@semio-tech/owner-"+at,dir=(at:number)=>root+"/🧰️framework/🛍️products/🗂️area-"+(at%40)+"/🔨️modules/📦️owner-"+(at%owners);
const rows:{kind:string;path:string;fact:unknown}[]=[];
for(let at=0;at<distinct;at++){
 const path=dir(at)+"/🟦️src/🧩️part-"+at+".ts",kind=["lstat","exists","readdir","file","analysis"][at%5]!;
 const fact=kind==="lstat"||kind==="file"?{size:1000+at,mtimeMs:1795550000000+at,ctimeMs:1795550000000+at,ino:at,sha256:(at.toString(16)+"0".repeat(64)).slice(0,64)}:kind==="exists"?at%3!==0:kind==="readdir"?["🟦️.ts","🧪️tests","🧬️schema","🔣️.json","part-"+at]:{provider:"typescript",revision:"r1",resultSha256:"f".repeat(64),source:{size:at,mtimeMs:at,ctimeMs:at,ino:at,sha256:"e".repeat(64)},nestedObservations:[rows[at-1],rows[at-2]].filter(Boolean)};
 rows.push({kind,path,fact});
}
const admission=Array.from({length:owners},(_,at)=>({owner:owner(at),suffix:owner(at)+"\0[[\"build\"],[]]",facts:{project:{name:owner(at),root:dir(at).slice(root.length+1)},files:[dir(at).slice(root.length+1)+"/📋️project.json"],dependencies:[owner((at+1)%owners)],dependencyInputs:{}},rows:Array.from({length:perEntry},(_,row)=>rows[(at*97+row*773)%distinct]!)}));
const projects:Record<string,unknown>={};let size=0;
for(let at=0;size<28*MB;at++){const project={root:dir(at).slice(root.length+1),name:owner(at),targets:{build:{inputs:Array.from({length:60},(_,file)=>"{projectRoot}/🟦️src/🧩️part-"+file+".ts"),outputs:["{projectRoot}/dist"],dependsOn:["^build"]}}};projects[owner(at)+"#"+at]=project;size+=JSON.stringify(project).length+40;}
const graph=JSON.stringify({projects,dependencies:[]}),ownerOf=(relative:string)=>{const match=/📦️owner-(\d+)/u.exec(relative);return match?owner(Number(match[1])):undefined;};
Bun.gc(true);await new Promise(done=>setTimeout(done,500));Bun.gc(true);
const before=process.memoryUsage().rss;let top=before;const timer=setInterval(()=>{top=Math.max(top,process.memoryUsage().rss);},20);
let blobs=0;const result=await streamSelectedDocumentsV1({host,sealed:{graph,rows,admission},ownerOf,put:async()=>{blobs++;top=Math.max(top,process.memoryUsage().rss);return true;}});
clearInterval(timer);top=Math.max(top,process.memoryUsage().rss);
if(!("owners" in result))throw Error(result.text);
console.log(JSON.stringify({rows:rows.length,entries:admission.length,references:owners*perEntry,graphMegabytes:Math.round(graph.length/MB),beforeMegabytes:Math.round(before/MB),topMegabytes:Math.round(top/MB),addedMegabytes:Math.round((top-before)/MB),blobs}));
