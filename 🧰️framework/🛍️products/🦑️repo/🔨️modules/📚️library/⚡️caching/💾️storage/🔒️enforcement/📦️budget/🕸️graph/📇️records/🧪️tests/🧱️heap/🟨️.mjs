import{encodeGraphRecordsV1,decodeGraphRecordsV1}from"../../🟨️.mjs";
import policy from"../../../🔣️policy.json"with{type:"json"};

const projectsCount=Number(process.argv[2]??5000),pathsPerArray=Number(process.argv[3]??4000);
const shared=[0,1,2].map(slot=>Array.from({length:pathsPerArray},(_,index)=>"{workspaceRoot}/framework/modules/m"+slot+"/path/to/source-"+index+".ts"));
const projects={};for(let index=0;index<projectsCount;index++)projects["p"+index]={name:"p"+index,root:"pkg/p"+index,targets:{build:{inputs:shared[index%3],outputs:["{projectRoot}/dist"],metadata:{index,note:"project "+index}}}};
const dependencies=Array.from({length:projectsCount*3},(_,index)=>({source:"p"+(index%projectsCount),target:"p"+((index*7+1)%projectsCount),type:"static"}));
const graph={version:1,projects,dependencies,admission:{selection:{kind:"all"},registry:"1".repeat(64),closure:"2".repeat(64)}};
let peak=0;const sample=()=>{peak=Math.max(peak,process.memoryUsage().rss);},timer=setInterval(sample,20);sample();
let raw=0;for(const name of Object.keys(projects))raw+=Buffer.byteLength(JSON.stringify(projects[name]));
const baseline=peak;
const encoded=await encodeGraphRecordsV1(graph,{policy:policy.records,maximumBytes:policy.maximumSnapshotBytes});
const encodePeak=peak;
const read=async digest=>encoded.records.get(digest);
const decoded=await decodeGraphRecordsV1(encoded.manifestBytes,read,{policy:policy.records});
const decodePeak=peak;sample();clearInterval(timer);
const sameShare=decoded.projects.p0.targets.build.inputs===decoded.projects.p3.targets.build.inputs;
console.log(JSON.stringify({projects:Object.keys(decoded.projects).length,raw,compressed:encoded.bytes,records:encoded.records.size,baseline,encodePeak,decodePeak,equal:JSON.stringify(decoded.projects.p4999??decoded.projects["p"+(projectsCount-1)])===JSON.stringify(projects["p"+(projectsCount-1)]),edges:decoded.dependencies.length===dependencies.length,sameShare,limit:policy.maximumSnapshotBytes}));
