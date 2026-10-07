/** 🛡️ Exact PresentationML profiles over literal OPC relationships and typed XML nodes. */
import type{PptxSnapshot,XmlDocument,XmlNode,XmlAttr}from"../🟦️.ts";
import {NativeDecodeControl,type NativeDecodeProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {textUtf8ByteLength} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📝️text/🟦️.ts";
export interface PptxProfileOptions{readonly maximumBytes?:number;readonly signal?:AbortSignal;readonly onProgress?:(progress:NativeDecodeProgress)=>void}
export interface PptxProfileDiagnostic{readonly code:string;readonly severity:"error"|"warning";readonly message:string}
const STRICT="http://purl.oclc.org/ooxml/presentationml/main",TRANS="http://schemas.openxmlformats.org/presentationml/2006/main",TRANS_DRAW="http://schemas.openxmlformats.org/drawingml/2006/main",STRICT_REL="http://purl.oclc.org/ooxml/officeDocument/relationships",TRANS_REL="http://schemas.openxmlformats.org/officeDocument/2006/relationships",VML="urn:schemas-microsoft-com:vml",MC="http://schemas.openxmlformats.org/markup-compatibility/2006";
class Control{
 readonly work:NativeDecodeControl;
 constructor(readonly options:PptxProfileOptions){this.work=new NativeDecodeControl(options.maximumBytes??Number.MAX_SAFE_INTEGER,event=>{options.onProgress?.(event);return true;},options.signal);}
 async admit(bytes:number):Promise<void>{await this.work.charge(bytes);}
 async tick(completed=0,total=0):Promise<void>{await this.work.beginStage(total);if(completed!==0)await this.work.advance(completed);}
 async textBytes(value:string):Promise<number>{let bytes=0;await this.tick(0,value.length);for(let start=0;start<value.length;){let end=Math.min(start+256,value.length);if(end<value.length&&value.charCodeAt(end-1)>=0xd800&&value.charCodeAt(end-1)<=0xdbff)end++;bytes+=textUtf8ByteLength(value.slice(start,end));await this.tick(end,value.length);start=end;}return bytes;}
 async equal(a:string,start:number,end:number,b:string,bStart=0):Promise<boolean>{if(end-start!==b.length-bStart)return false;await this.tick(0,end-start);for(let i=0;i<end-start;i++){if(a.charCodeAt(start+i)!==b.charCodeAt(bStart+i))return false;if((i+1)%256===0)await this.tick(i+1,end-start);}return true;}
 async text(parts:readonly string[]):Promise<string>{let bytes=0,total=0;for(const part of parts){bytes+=await this.textBytes(part);total+=part.length;}await this.admit(bytes);let output="",completed=0;await this.tick(0,total);for(const part of parts){for(let start=0;start<part.length;){let end=Math.min(start+256,part.length);if(end<part.length&&part.charCodeAt(end-1)>=0xd800&&part.charCodeAt(end-1)<=0xdbff)end++;output+=part.slice(start,end);completed+=end-start;await this.tick(completed,total);start=end;}}return output;}
 async range(value:string,start:number,end:number):Promise<string>{let output="";await this.tick(0,end-start);for(let position=start;position<end;){let next=Math.min(position+256,end);if(next<end&&value.charCodeAt(next-1)>=0xd800&&value.charCodeAt(next-1)<=0xdbff)next++;let bytes=0;for(let i=position;i<next;i++){const code=value.charCodeAt(i);if(code>=0xd800&&code<=0xdbff&&i+1<next&&value.charCodeAt(i+1)>=0xdc00&&value.charCodeAt(i+1)<=0xdfff){bytes+=4;i++;}else bytes+=code<128?1:code<2048?2:3;}await this.admit(bytes);output+=value.slice(position,next);await this.tick(next-start,end-start);position=next;}return output;}
}
interface Frame{readonly nodes:readonly XmlNode[];position:number;readonly attrs:readonly XmlAttr[]}
interface Flags{mainStrict:boolean;mainTrans:boolean;trans:boolean;vml:boolean;family:boolean;alternate:boolean;conformance:boolean}
async function inspect(document:XmlDocument,control:Control):Promise<Flags>{
 const flags:Flags={mainStrict:false,mainTrans:false,trans:false,vml:false,family:false,alternate:false,conformance:false};const frames:Frame[]=[];let capacity=0;
 const push=async(nodes:readonly XmlNode[],attrs:readonly XmlAttr[])=>{if(frames.length===capacity){await control.admit(64*32);capacity+=64;}frames.push({nodes,position:0,attrs});};
 for(const forest of[document.prolog,document.root?[document.root]:[],document.epilog]){await control.admit(8);await push(forest,[]);while(frames.length!==0){const frame=frames[frames.length-1]!,node=frame.nodes[frame.position++];if(node===undefined){frames.pop();continue;}await control.tick();if(node.kind!=="element")continue;
 for(const attr of node.attrs){await control.tick();if(attr.name==="xmlns"||attr.name.startsWith("xmlns:")){flags.trans ||=await control.equal(attr.value,0,attr.value.length,TRANS)||await control.equal(attr.value,0,attr.value.length,TRANS_DRAW);flags.vml ||=await control.equal(attr.value,0,attr.value.length,VML);flags.family ||=attr.value.startsWith("http://purl.oclc.org/ooxml/");}}
 if(node===document.root)for(const attr of node.attrs){await control.tick();if(attr.name==="conformance"&&attr.value==="strict")flags.conformance=true;}
 let separator=-1;for(let i=0;i<node.name.length;i++){if(node.name[i]===":"){separator=i;break;}if((i+1)%256===0)await control.tick(i+1,node.name.length);}
 const binding=async(attrs:readonly XmlAttr[]):Promise<string|undefined>=>{for(const attr of attrs){await control.tick();if(separator<0?attr.name==="xmlns":attr.name.length===separator+6&&attr.name.startsWith("xmlns:")&&await control.equal(node.name,0,separator,attr.name,6))return attr.value;}return undefined;};
 if(node===document.root&&await control.equal(node.name,separator+1,node.name.length,"presentation")){const namespace=await binding(node.attrs);if(namespace!==undefined){flags.mainStrict=await control.equal(namespace,0,namespace.length,STRICT);flags.mainTrans=await control.equal(namespace,0,namespace.length,TRANS);}}
 if(await control.equal(node.name,separator+1,node.name.length,"AlternateContent")){let namespace=await binding(node.attrs);if(namespace===undefined)for(let scope=frames.length-1;scope>=0;scope--){namespace=await binding(frames[scope]!.attrs);if(namespace!==undefined)break;}if(namespace!==undefined&&await control.equal(namespace,0,namespace.length,MC))flags.alternate=true;}
 await push(node.children,node.attrs);
 }}return flags;
}
async function mainPath(target:string,control:Control):Promise<string>{
 if(target.startsWith("/"))return control.range(target,1,target.length);const segments:{start:number;end:number}[]=[];let start=0;
 for(let position=0;position<=target.length;position++){if(position===target.length||target[position]==="/"){const width=position-start;if(width===2&&target[start]==="."&&target[start+1]===".")segments.pop();else if(width!==0&&!(width===1&&target[start]===".")){await control.admit(16);segments.push({start,end:position});}start=position+1;}if(position%256===0)await control.tick(position,target.length);}
 let output="";for(let i=0;i<segments.length;i++){if(i!==0){await control.admit(1);output+="/";}output+=await control.range(target,segments[i]!.start,segments[i]!.end);}return output;
}
/** 🚦️ Validates exact profile semantics without encoding the owned XML graph. */
export async function validatePptxSnapshotProfile(snapshot:PptxSnapshot,subset:"strict"|"transitional",options:PptxProfileOptions={}):Promise<readonly PptxProfileDiagnostic[]>{
 if(subset!=="strict"&&subset!=="transitional")throw Error("PPTX unknown exact profile");const control=new Control(options),strict=subset==="strict",output:PptxProfileDiagnostic[]=[];await control.tick();
 const add=async(suffix:string,severity:"error"|"warning",parts:readonly string[])=>{await control.admit(96);output.push({code:await control.text(["stdio.pptx.",subset,".",suffix]),severity,message:await control.text(parts)});};
 let main:string|undefined;for(const relationship of snapshot.opc.relationships[""]??[]){await control.tick();if(relationship.relType.endsWith("/officeDocument")){main=await mainPath(relationship.target,control);break;}}
 let mainPart:PptxSnapshot["xmlParts"][number]|undefined;if(main!==undefined)for(const part of snapshot.xmlParts){await control.tick();if(await control.equal(part.path,0,part.path.length,main)){mainPart=part;break;}}
 if(mainPart===undefined)await add(strict?"main-ns-not-strict":"main-ns-not-transitional","error",["package has no resolvable main PresentationML document"]);
 for(const part of snapshot.xmlParts){await control.tick();const flags=await inspect(part.document,control);
 if(part===mainPart){if(!(strict?flags.mainStrict:flags.mainTrans))await add(strict?"main-ns-not-strict":"main-ns-not-transitional","error",["root officeDocument part ",part.path," does not declare the expected PresentationML root namespace ",strict?STRICT:TRANS]);if(strict&&!flags.conformance)await add("conformance-attr-missing","warning",["root officeDocument part ",part.path,' does not declare conformance="strict"']);if(!strict&&flags.conformance)await add("conformance-attr-not-transitional","warning",["root officeDocument part ",part.path,' declares conformance="strict"']);}
 if(strict){if(flags.trans)await add("transitional-ns-present","error",["part ",part.path," declares a Transitional OOXML main namespace"]);if(flags.vml)await add("vml-present","error",["part ",part.path," declares VML markup ",VML]);if(flags.alternate)await add("alternate-content-present","warning",["part ",part.path," contains mc:AlternateContent markup"]);}else if(flags.family)await add("strict-ns-present","error",["part ",part.path," declares a Strict OOXML namespace"]);
 }
 for(const owner in snapshot.opc.relationships){await control.tick();for(const relationship of snapshot.opc.relationships[owner]??[]){await control.tick();if(strict&&relationship.relType.startsWith(TRANS_REL))await add("relationship-base-not-strict","error",["relationship ",relationship.id," owned by ",owner," uses Transitional relationship type ",relationship.relType]);if(!strict&&relationship.relType.startsWith(STRICT_REL))await add("strict-ns-present","error",["relationship ",relationship.id," owned by ",owner," uses Strict relationship type ",relationship.relType]);}}
 return output;
}
