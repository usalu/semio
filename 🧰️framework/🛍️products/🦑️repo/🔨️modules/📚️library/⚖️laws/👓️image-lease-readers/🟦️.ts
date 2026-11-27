import type*as TsTypes from"typescript";
import{readdirSync,readFileSync}from"node:fs";
import{join}from"node:path";
import{typescriptCompilerV1}from"../../🏗️build/🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";
import{validateJsonSchemaSubset}from"../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const ts=typescriptCompilerV1() as unknown as typeof TsTypes;
declare namespace ts{type Node=TsTypes.Node;type Expression=TsTypes.Expression;type Identifier=TsTypes.Identifier;type ArrowFunction=TsTypes.ArrowFunction;type FunctionExpression=TsTypes.FunctionExpression;type SignatureDeclaration=TsTypes.SignatureDeclaration;type ScriptKind=TsTypes.ScriptKind;type ScriptTarget=TsTypes.ScriptTarget}
/** 📜️ The data of the law: where to scan, which words name the mount of a cache image, which helpers, values and types carry a path below an image (root patterns, root types), which calls open a file, which calls, parameters and arguments mean that the read runs under the shared lease of the image, and the documented call sites.
 * @see ./🧬️schema/🔣️.json */
export type ImageLeaseLawPolicyV1=Readonly<{version:1;roots:readonly string[];skipDirectories:readonly string[];extensions:readonly string[];skipSuffixes:readonly string[];mountPatterns:readonly string[];rootPatterns:readonly string[];rootTypes:readonly string[];openers:readonly string[];leaseCallees:readonly string[];leasedParameters:readonly string[];leasedArguments:readonly string[];carrierMethods:readonly string[];classes:readonly string[];entries:readonly Readonly<{path:string;site:string;class:string;reason:Readonly<{en:string;de:string}>}>[]}>;
/** 📍️ One call that opens a path built by hand below the mount of a cache image: `text` is the call without whitespace, as a documented entry names it. */
export type ImageLeaseSiteV1=Readonly<{path:string;line:number;opener:string;text:string}>;
/** ✅️ Validates a policy document and answers it typed: the law says that every call that opens a path below the mount of the image of a cache partition but the leases (the lease directory has no lease of its own, its readers cannot take one) runs inside the callback of a shared read (`withNativeImageSharedReadV1`, `readUnderImageLeaseV1`, a reader's `run`, a repository store scope) or inside a function that takes a leased port parameter, or is a documented site. */
export function readImageLeaseLawPolicyV1(value:unknown):ImageLeaseLawPolicyV1{
 const problems=validateJsonSchemaSubset(schema,value,schema);
 if(problems.length>0)throw Error("Valid image lease law policy required: "+problems.join("; "));
 const policy=value as ImageLeaseLawPolicyV1;
 for(const row of policy.entries)if(!policy.classes.includes(row.class))throw Error("Declared allowlist class required: "+row.class);
 return policy;
}
const compile=(patterns:readonly string[])=>patterns.map(pattern=>new RegExp(pattern,"u"));
const matches=(source:string,patterns:readonly string[]):boolean=>compile(patterns).some(pattern=>pattern.test(source));
const words=(text:string):readonly string[]=>text.match(/[A-Za-z_$][\w$]*/gu)??[];
/** 🧭️ Whether a source names a path below the mount of a cache image, or a helper, value or type that carries one. */
export function namesImageMountV1(source:string,policy:ImageLeaseLawPolicyV1):boolean{return matches(source,[...policy.mountPatterns,...policy.rootPatterns,...policy.rootTypes]);}
const calleeTextV1=(node:ts.Expression):string=>{
 const chain=(at:ts.Expression):string|undefined=>ts.isIdentifier(at)?at.text:at.kind===ts.SyntaxKind.ThisKeyword?"this":ts.isPropertyAccessExpression(at)?((left:string|undefined)=>left===undefined?undefined:left+"."+at.name.text)(chain(at.expression)):undefined;
 return chain(node)??(ts.isPropertyAccessExpression(node)?"."+node.name.text:"");
};
/** 🔎️ The calls of one source that open a path below a cache image outside every lease. A value carries such a path when its expression matches a mount or root pattern, or refers to a name that does; names are followed to a fixed point over the syntax tree through declarations (with their type annotations against the root types), parameters, assignments, loop bindings, the arguments of calls to functions declared in the same file that are not themselves leased and the arguments of carrier methods (`push`, `set`) into their receiver, whatever the shape of the expression (a name that only is a property name or an object key is not a reference). A declaration at the top level of a file counts in the whole file, any other inside its top-level statement only. A call is a violation when it opens such a path (or hands it to a documented helper) and is not leased: inside the arguments of a lease callee, inside a function with a leased parameter, or handing a leased argument (a reader of the image) to the callee. Judged call by call, so a file with one leased and one unleased read fails on the unleased one.
 * @see ./🧬️schema/🔣️.json */
export function unleasedImageSitesV1(path:string,source:string,policy:ImageLeaseLawPolicyV1):readonly ImageLeaseSiteV1[]{
 const mounts=compile([...policy.mountPatterns,...policy.rootPatterns]),types=compile(policy.rootTypes),openers=compile(policy.openers),callees=compile(policy.leaseCallees),parameters=compile(policy.leasedParameters),carried=compile(policy.leasedArguments),carriers=new Set(policy.carrierMethods);
 const file=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true,path.endsWith(".tsx")?ts.ScriptKind.TSX:/\.m?js$/u.test(path)?ts.ScriptKind.JS:ts.ScriptKind.TS);
 const unitOf=(node:ts.Node):ts.Node=>{let up=node;while(up.parent!==undefined&&up.parent!==file)up=up.parent;return up;};
 const keyName=(node:ts.Identifier):boolean=>{const parent=node.parent;return ts.isPropertyAccessExpression(parent)&&parent.name===node||ts.isQualifiedName(parent)&&parent.right===node||(ts.isPropertyAssignment(parent)||ts.isPropertySignature(parent)||ts.isMethodDeclaration(parent)||ts.isMethodSignature(parent))&&parent.name===node||ts.isBindingElement(parent)&&parent.propertyName===node;};
 const refsOf=(nodes:readonly ts.Node[]):readonly string[]=>{const found:string[]=[];const walk=(node:ts.Node):void=>{if(ts.isIdentifier(node)&&!keyName(node))found.push(node.text);ts.forEachChild(node,walk);};for(const node of nodes)walk(node);return found;};
 const namesOf=(node:ts.Node):readonly string[]=>ts.isIdentifier(node)?[node.text]:ts.isObjectBindingPattern(node)||ts.isArrayBindingPattern(node)?node.elements.flatMap(element=>ts.isBindingElement(element)?namesOf(element.name):[]):[];
 type Rule={names:readonly string[];text:string;refs:readonly string[];type:boolean;reads:ts.Node;writes:ts.Node;top:boolean};
 const rules:Rule[]=[],topNames=new Set<string>(),functions=new Map<string,ts.SignatureDeclaration>();
 const isFunction=(node:ts.Node|undefined):node is ts.ArrowFunction|ts.FunctionExpression=>node!==undefined&&(ts.isArrowFunction(node)||ts.isFunctionExpression(node));
 const collect=(node:ts.Node):void=>{
  if(ts.isVariableDeclaration(node)){
   const unit=unitOf(node),list=node.parent,loop=ts.isVariableDeclarationList(list)&&(ts.isForOfStatement(list.parent)||ts.isForInStatement(list.parent))?list.parent:undefined,top=ts.isVariableDeclarationList(list)&&ts.isVariableStatement(list.parent)&&list.parent.parent===file,names=namesOf(node.name);
   if(top)for(const name of names)topNames.add(name);
   const source=loop?.expression??node.initializer;
   if(source!==undefined)rules.push({names,text:source.getText(file),refs:refsOf([source]),type:false,reads:unit,writes:unit,top});
   if(node.type!==undefined)rules.push({names,text:node.type.getText(file),refs:[],type:true,reads:unit,writes:unit,top});
   if(ts.isIdentifier(node.name)&&isFunction(node.initializer))functions.set(node.name.text,node.initializer);
  }
  if(ts.isFunctionDeclaration(node)&&node.name!==undefined)functions.set(node.name.text,node);
  if(ts.isParameter(node)){
   const unit=unitOf(node),names=namesOf(node.name);
   if(node.initializer!==undefined)rules.push({names,text:node.initializer.getText(file),refs:refsOf([node.initializer]),type:false,reads:unit,writes:unit,top:false});
   if(node.type!==undefined)rules.push({names,text:node.type.getText(file),refs:[],type:true,reads:unit,writes:unit,top:false});
  }
  ts.forEachChild(node,collect);
 };
 collect(file);
 const leased=(node:ts.Node):boolean=>{
  let child=node;
  for(let up=node.parent as ts.Node|undefined;up!==undefined;child=up,up=up.parent){
   if(ts.isCallExpression(up)&&up.expression!==child&&callees.some(pattern=>pattern.test(calleeTextV1(up.expression))))return true;
   if(ts.isFunctionLike(up)&&parameters.some(pattern=>pattern.test((up as ts.SignatureDeclaration).parameters.map(parameter=>parameter.getText(file)).join(","))))return true;
  }
  return false;
 };
 const flow=(node:ts.Node):void=>{
  if(ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.EqualsToken&&ts.isIdentifier(node.left)){const unit=unitOf(node);rules.push({names:[node.left.text],text:node.right.getText(file),refs:refsOf([node.right]),type:false,reads:unit,writes:unit,top:topNames.has(node.left.text)});}
  if(ts.isCallExpression(node)&&ts.isPropertyAccessExpression(node.expression)&&ts.isIdentifier(node.expression.expression)&&carriers.has(node.expression.name.text)){const receiver=node.expression.expression.text,unit=unitOf(node);rules.push({names:[receiver],text:node.arguments.map(argument=>argument.getText(file)).join(","),refs:refsOf([...node.arguments]),type:false,reads:unit,writes:unit,top:topNames.has(receiver)});}
  if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)){
   const target=functions.get(node.expression.text);
   if(target!==undefined&&!leased(node))node.arguments.forEach((argument,at)=>{const parameter=target.parameters[at];if(parameter!==undefined)rules.push({names:namesOf(parameter.name),text:argument.getText(file),refs:refsOf([argument]),type:false,reads:unitOf(node),writes:unitOf(target),top:false});});
  }
  ts.forEachChild(node,flow);
 };
 flow(file);
 const shared=new Set<string>(),local=new Map<ts.Node,Set<string>>();
 const tainted=(name:string,unit:ts.Node):boolean=>shared.has(name)||local.get(unit)?.has(name)===true;
 const bears=(text:string,names:readonly string[],unit:ts.Node):boolean=>mounts.some(pattern=>pattern.test(text))||names.some(name=>tainted(name,unit));
 for(let changed=true;changed;){changed=false;for(const row of rules){const target=row.top?shared:local.get(row.writes)??local.set(row.writes,new Set()).get(row.writes)!;if(row.names.some(name=>!target.has(name))&&(row.type?types.some(pattern=>pattern.test(row.text)):bears(row.text,row.refs,row.reads))){for(const name of row.names)target.add(name);changed=true;}}}
 const found:ImageLeaseSiteV1[]=[];
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)){
   const opener=calleeTextV1(node.expression);
   if(openers.some(pattern=>pattern.test(opener))){
    const args=source.slice(node.arguments.pos,node.arguments.end);
    if(bears(args,refsOf([...node.arguments]),unitOf(node))&&!carried.some(pattern=>pattern.test(args))&&!leased(node)){const name=ts.isPropertyAccessExpression(node.expression)?node.expression.name:node.expression;found.push(Object.freeze({path,line:file.getLineAndCharacterOfPosition(name.getStart(file)).line+1,opener,text:opener+"("+args.replace(/\s+/gu,"")+")"}));}
   }
  }
  ts.forEachChild(node,visit);
 };
 visit(file);
 return found;
}
/** 📂️ Lists the scanned source files below the roots, relative to the repository and with `/` separators, in code point order. */
export function listImageLeaseLawFilesV1(repository:string,policy:ImageLeaseLawPolicyV1):readonly string[]{
 const found:string[]=[],skip=new Set(policy.skipDirectories);
 const walk=(directory:string,relative:string):void=>{
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   const child=relative===""?entry.name:relative+"/"+entry.name;
   if(entry.isDirectory()){if(!skip.has(entry.name))walk(join(directory,entry.name),child);continue;}
   if(entry.isFile()&&policy.extensions.some(extension=>entry.name.endsWith(extension))&&!policy.skipSuffixes.some(suffix=>entry.name.endsWith(suffix)))found.push(child);
  }
 };
 for(const root of policy.roots)walk(join(repository,...root.split("/")),root);
 return found.sort((left,right)=>left<right?-1:left>right?1:0);
}
/** ⚖️ The verdict over a set of sources: `named` are the files that name a mount path, `sites` the undocumented calls that open such a path outside every lease, `unrouted` the files of those calls, and `stale` the documented sites (`path` and `site`) that no longer match an unleased call. */
export type ImageLeaseVerdictV1=Readonly<{named:readonly string[];unrouted:readonly string[];sites:readonly ImageLeaseSiteV1[];stale:readonly string[]}>;
const order=(left:string,right:string):number=>left<right?-1:left>right?1:0;
/** ⚖️ Judges the sources of the repository against the policy. Pure and language-agnostic. */
export function judgeImageLeaseReadersV1(input:Readonly<{sources:ReadonlyMap<string,string>;policy:ImageLeaseLawPolicyV1}>):ImageLeaseVerdictV1{
 const named:string[]=[],sites:ImageLeaseSiteV1[]=[],used=new Set<number>(),entries=input.policy.entries;
 for(const[path,source]of input.sources){
  if(!namesImageMountV1(source,input.policy))continue;
  named.push(path);
  for(const site of unleasedImageSitesV1(path,source,input.policy)){
   const hits=entries.flatMap((row,at)=>row.path===path&&site.text.includes(row.site)?[at]:[]);
   for(const at of hits)used.add(at);
   if(hits.length===0)sites.push(site);
  }
 }
 sites.sort((left,right)=>order(left.path,right.path)||left.line-right.line);
 const stale=entries.flatMap((row,at)=>used.has(at)?[]:[row.path+"::"+row.site]);
 return Object.freeze({named:Object.freeze(named.sort()),unrouted:Object.freeze([...new Set(sites.map(site=>site.path))].sort()),sites:Object.freeze(sites),stale:Object.freeze(stale.sort())});
}
/** 🗺️ Reads every scanned file of the repository and judges it. */
export function judgeImageLeaseRepositoryV1(repository:string,policy:ImageLeaseLawPolicyV1):ImageLeaseVerdictV1{
 const sources=new Map<string,string>();
 for(const path of listImageLeaseLawFilesV1(repository,policy))sources.set(path,readFileSync(join(repository,...path.split("/")),"utf8"));
 return judgeImageLeaseReadersV1({sources,policy});
}
/** 🧾️ The text of a refusal in both languages that names every unleased call site. */
export function imageLeaseRefusalV1(verdict:ImageLeaseVerdictV1):string{
 const list=verdict.sites.map(site=>site.path+":"+site.line+" "+site.opener).join(", ");
 return "Calls that open a path below the mount of a cache image outside its shared lease: "+list+" / Aufrufe, die einen Pfad unter dem Mount eines Cache-Images ausserhalb dessen geteilter Lease öffnen: "+list;
}
