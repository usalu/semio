import{createHash}from"node:crypto";
import{mkdirSync,readFileSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";

/** 🔬️ Hermetic probes for remote execution: each kind reads only the workspace files named on its command line, writes one JSON document and nothing else, so a sandbox or a remote worker can run it from its declared inputs alone. */
const[kind="",output="",...inputs]=process.argv.slice(2),root=resolve(process.env.SEMIO_WORKSPACE_ROOT??process.cwd()),sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex"),fail=(message:string):never=>{throw new Error(`[probe] ${message}`);};
if(!kind||!output)fail("kind and output required");
const read=(path:string)=>readFileSync(join(root,path)),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const document=(value:object)=>JSON.stringify(value,null,1)+"\n";
if(kind==="platform")write(output,document({kind,platform:process.platform,arch:process.arch,bun:Bun.version,hermetic:process.env.SEMIO_HERMETIC??null,timezone:process.env.TZ??null,locale:process.env.LC_ALL??null,epoch:process.env.SOURCE_DATE_EPOCH??null,home:process.env.HOME?"set":"unset"}));
else if(kind==="fanout"){
 const parts=Array.from({length:8},(_,index)=>({path:`${output}/part-${index}.txt`,text:sha(`part ${index}`)+"\n"}));
 for(const part of parts)write(part.path,part.text);
 write(`${output}/index.json`,document({kind,parts:parts.map(part=>({path:part.path,sha256:sha(part.text)}))}));
}
else if(kind==="chain"){
 const parts=Object.fromEntries(inputs.map(path=>[path,sha(read(path))]));
 write(output,document({kind,parts,digest:sha(JSON.stringify(Object.entries(parts).sort()))}));
}
else fail(`unknown kind: ${kind}`);
