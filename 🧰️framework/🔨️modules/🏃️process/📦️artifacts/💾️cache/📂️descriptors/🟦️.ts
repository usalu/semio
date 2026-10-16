import{resolve,sep}from"node:path";
import schema from"../🧬️schema/🔣️.json";
export type NativeCacheDescriptorObservationV1=Readonly<{status:number|null;signal:string|null;reason?:"exit"|"timeout"|"cancelled"|"output-limit"|"spawn-error";stdout:string;stderr:string}>;
export type NativeCacheDescriptorFormatV1="nul-pcfatn"|"newline-pn";
const policy=Object.freeze({...schema.definitions.DescriptorPolicy.const});
/** 🔤️ Decodes the documented printable lsof field encodings with strict UTF-8 byte admission. */
export function decodeLsofPathV1(path:string):string{
 if(Buffer.byteLength(path)>policy.maximumFieldBytes)throw Error("Bounded lsof name required");const chunks:Buffer[]=[];let start=0;
 for(let i=0;i<path.length;i++){let byte:number|undefined,width=0;const character=path[i];if(character==="\\"){const next=path[i+1],escaped=next===undefined?undefined:({b:8,f:12,n:10,r:13,t:9,"\\":92}as Readonly<Record<string,number>>)[next];if(next==="x"){const hex=path.slice(i+2,i+4);if(!/^[0-9a-fA-F]{2}$/u.test(hex))throw Error("Exact lsof name encoding required");byte=Number.parseInt(hex,16);width=4;}else if(escaped!==undefined){byte=escaped;width=2;}else throw Error("Exact lsof name encoding required");}else if(character==="^"&&path[i+1]&&(path.charCodeAt(i+1)>=64&&path.charCodeAt(i+1)<=95||path[i+1]==="?")){byte=path[i+1]==="?"?127:path.charCodeAt(i+1)-64;width=2;}if(byte!==undefined){if(i>start)chunks.push(Buffer.from(path.slice(start,i),"utf8"));chunks.push(Buffer.from([byte]));i+=width-1;start=i+1;}}
 if(start<path.length)chunks.push(Buffer.from(path.slice(start),"utf8"));const decoded=new TextDecoder("utf-8",{fatal:true}).decode(Buffer.concat(chunks));if(decoded.includes("\0"))throw Error("NUL is not a filesystem name");return decoded;
}
/** 🔐️ Admits only complete explicitly selected field protocols and the exact physical guard's own descriptors. */
export function assertNativeCacheDescriptorCustodyV1(observation:NativeCacheDescriptorObservationV1,directory:string,guard:Readonly<{pid:number;paths:readonly string[]}>,format:NativeCacheDescriptorFormatV1):void{
 if(observation.status!==0||observation.signal!==null||observation.reason!=="exit"||observation.stderr.trim()||!Number.isSafeInteger(guard.pid)||guard.pid<=0||guard.paths.length>policy.maximumGuardPaths||!(format==="nul-pcfatn"||format==="newline-pn"))throw Error("Complete compiler descriptor inventory required");
 const wire=observation.stdout,root=resolve(directory),allowed=new Set(guard.paths),seen=new Set<string>();if(root===sep||!wire||Buffer.byteLength(wire)>policy.maximumBytes||!wire.endsWith("\n")||allowed.size!==guard.paths.length||[root,...guard.paths].some(path=>/[\\\x00-\x1f\x7f]/u.test(path)||/\^[@-_?]/u.test(path))||guard.paths.some(path=>path!==root&&!path.startsWith(root+sep)))throw Error("Complete compiler descriptor inventory required");
 let pid=0,fields=0,sets=0;const processIds=new Set<number>();
 const field=(value:string)=>{if(++fields>policy.maximumFields||!value||Buffer.byteLength(value)>policy.maximumFieldBytes||Buffer.from(value,"utf8").toString("utf8")!==value)throw Error("Bounded complete lsof fields required");};
 const processField=(value:string)=>{if(!/^[1-9][0-9]*$/u.test(value))throw Error("Exact lsof process identity required");pid=Number(value);if(!Number.isSafeInteger(pid)||processIds.has(pid))throw Error("Unambiguous lsof process identity required");processIds.add(pid);};
 const name=(value:string)=>{if(!pid||!value)throw Error("Owned lsof name field required");const path=decodeLsofPathV1(value);if(path===root||path.startsWith(root+sep)){if(pid!==guard.pid||!allowed.has(path))throw Error("Compiler store has foreign open descriptors");seen.add(path);}};
 if(format==="newline-pn"){
  if(wire.includes("\0"))throw Error("Selected newline lsof protocol required");let start=0;
  for(let end=wire.indexOf("\n");end>=0;end=wire.indexOf("\n",start)){const value=wire.slice(start,end);field(value);if(value[0]==="p"){if(++sets>policy.maximumSets)throw Error("Bounded lsof sets required");processField(value.slice(1));}else if(value[0]==="n"){if(++sets>policy.maximumSets)throw Error("Bounded lsof sets required");name(value.slice(1));}else throw Error("Selected newline lsof fields required");start=end+1;if(start===wire.length)break;}
 }else{
  let start=0,kind:"p"|"f"|undefined,identifiers=new Set<string>(),fileName:string|undefined;
  while(start<wire.length){const end=wire.indexOf("\0",start);if(end<0)throw Error("Complete NUL lsof field required");const value=wire.slice(start,end);field(value);const id=value.charAt(0),data=value.slice(1);if(!kind){if(++sets>policy.maximumSets)throw Error("Bounded lsof sets required");if(id==="p"){kind="p";processField(data);}else if(id==="f"&&pid&&data){kind="f";}else throw Error("Exact lsof set start required");}else if(kind==="p"?id!=="c":!"atn".includes(id))throw Error("Selected NUL lsof fields required");if(identifiers.has(id)||!data)throw Error("Unambiguous lsof field required");identifiers.add(id);if(id==="n")fileName=data;start=end+1;if(wire[start]==="\n"){if(kind==="f"){if(fileName===undefined)throw Error("Complete lsof file name required");name(fileName);}kind=undefined;identifiers=new Set();fileName=undefined;start++;}else if(start===wire.length)throw Error("Complete lsof set terminator required");}
  if(kind!==undefined)throw Error("Complete lsof set required");
 }
 if(!processIds.size||guard.paths.some(path=>!seen.has(path)))throw Error("Physical compiler guard is absent from the complete descriptor inventory");
}
