/** 🧾 PresentationML outline derived from the actual OPC relationships and retained XML. */
import type {PptxSnapshot,XmlNode} from "../../📸️snapshot/🟦️.ts";
import {ValueError} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts";
export interface PptxOutline { slideCount:number;shapeCount:number;wordCount:number }
type Element=Extract<XmlNode,{kind:"element"}>;
type Scope=ReadonlyMap<string,string>;
const p=["http://schemas.openxmlformats.org/presentationml/2006/main","http://purl.oclc.org/ooxml/presentationml/main"];
const r=["http://schemas.openxmlformats.org/officeDocument/2006/relationships","http://purl.oclc.org/ooxml/officeDocument/relationships"];
const a=["http://schemas.openxmlformats.org/drawingml/2006/main","http://purl.oclc.org/ooxml/drawingml/main"];
const invalid=(message:string):never=>{throw new ValueError("invalidValue",message);};
const scope=(parent:Scope,node:XmlNode):Scope=>{const result=new Map(parent);if(node.kind==="element")for(const attr of node.attrs){if(attr.name==="xmlns")result.set("",attr.value);else if(attr.name.startsWith("xmlns:"))result.set(attr.name.slice(6),attr.value);}return result;};
const name=(raw:string,namespaces:Scope,attribute=false):[string,string]=>{const colon=raw.indexOf(":");if(colon<0)return[attribute?"":namespaces.get("")??"",raw];const uri=namespaces.get(raw.slice(0,colon));if(uri===undefined)invalid("PPTX XML prefix is not declared");return[uri!,raw.slice(colon+1)];};
const matches=(node:XmlNode,namespaces:Scope,uris:readonly string[],local:string):node is Element=>{if(node.kind!=="element")return false;const expanded=name(node.name,namespaces);return uris.includes(expanded[0])&&expanded[1]===local;};
const child=(node:Element,namespaces:Scope,uris:readonly string[],local:string):Element=>{for(const candidate of node.children)if(matches(candidate,scope(namespaces,candidate),uris,local))return candidate;return invalid("PPTX XML has no "+local);};
const attribute=(node:Element,namespaces:Scope,uris:readonly string[],local:string):string|undefined=>{for(const attr of node.attrs){if(attr.name==="xmlns"||attr.name.startsWith("xmlns:"))continue;const expanded=name(attr.name,namespaces,true);if(uris.includes(expanded[0])&&expanded[1]===local)return attr.value;}};
const target=(owner:string,raw:string):string=>{const pieces=(raw.startsWith("/")?raw.slice(1):owner.slice(0,owner.lastIndexOf("/")+1)+raw).split("/"),output:string[]=[];for(const piece of pieces){if(piece==="..")output.pop();else if(piece!=="."&&piece!=="")output.push(piece);}return output.join("/");};
const count=(left:number,right:number):number=>{const result=left+right;if(!Number.isSafeInteger(result)||result>4294967295)throw new ValueError("workLimit","PPTX outline count exceeds unsigned32");return result;};
/** 🧭 Counts a retained PresentationML owner and refuses missing OPC or XML authority. */
export function computePptxOutline(snapshot:PptxSnapshot):PptxOutline{
 const office=snapshot.opc.relationships[""]?.find(relation=>relation.targetMode==="internal"&&r.some(uri=>relation.relType===uri+"/officeDocument"));if(!office)invalid("PPTX package has no officeDocument relationship");
 const rootAt=(path:string):Element=>{const root=snapshot.xmlParts.find(part=>part.path===path)?.document.root;if(root?.kind!=="element")return invalid("PPTX XML part "+path+" has no root");return root;};
 const presentationPath=target("",office!.target),root=rootAt(presentationPath),rootScope=scope(new Map(),root);if(!matches(root,rootScope,p,"presentation"))invalid("PPTX officeDocument target is not a PresentationML presentation");
 const list=child(root,rootScope,p,"sldIdLst"),listScope=scope(rootScope,list);let slideCount=0,shapeCount=0,wordCount=0;
 for(const entry of list.children){const entryScope=scope(listScope,entry);if(!matches(entry,entryScope,p,"sldId"))continue;const id=attribute(entry,entryScope,r,"id");if(id===undefined)invalid("PPTX slide entry has no relationship id");const relation=snapshot.opc.relationships[presentationPath]?.find(relation=>relation.id===id);if(!relation)invalid("PPTX slide relationship does not exist");const slide=rootAt(target(presentationPath,relation!.target)),slideScope=scope(new Map(),slide);if(!matches(slide,slideScope,p,"sld"))invalid("PPTX slide root is not PresentationML sld");
  const content=child(slide,slideScope,p,"cSld"),contentScope=scope(slideScope,content),tree=child(content,contentScope,p,"spTree"),treeScope=scope(contentScope,tree);slideCount=count(slideCount,1);
  for(const shape of tree.children){const shapeScope=scope(treeScope,shape);if(shape.kind!=="element")continue;const expanded=name(shape.name,shapeScope);if(!p.includes(expanded[0])||["nvGrpSpPr","grpSpPr"].includes(expanded[1]))continue;
   let identified=false;for(const nonvisual of shape.children){const ns=scope(shapeScope,nonvisual);if(nonvisual.kind!=="element"||!name(nonvisual.name,ns)[1].startsWith("nv"))continue;for(const candidate of nonvisual.children){const cs=scope(ns,candidate);if(matches(candidate,cs,p,"cNvPr")&&attribute(candidate,cs,[""],"id")!==undefined)identified=true;}}
   if(!identified)continue;shapeCount=count(shapeCount,1);let text="",paragraph=false;const pending:[XmlNode,Scope][]=[[shape,shapeScope]];while(pending.length){const[node,ns]=pending.pop()!;if(node.kind!=="element")continue;if(matches(node,ns,a,"p")){if(paragraph)text+="\n";paragraph=true;}if(matches(node,ns,a,"t")){for(const leaf of node.children)if(leaf.kind==="text"||leaf.kind==="cData")text+=leaf.text;continue;}for(let index=node.children.length-1;index>=0;index--){const next=node.children[index]!;pending.push([next,scope(ns,next)]);}}
   if(paragraph)wordCount=count(wordCount,text.trim()?text.trim().split(/\s+/u).length:0);
  }
 }
 return{slideCount,shapeCount,wordCount};
}
