/** 🧮️ How many parameters a function takes: the positional ones and whether a rest parameter follows. */
export type DeclarationArityV1=Readonly<{count:number;rest:boolean}>;
/** 🏷️ What an export is: a function, a class, another value, a name re-exported without its shape, or a type only the declaration holds. */
export type DeclarationKindV1="function"|"class"|"value"|"reexport"|"type";
/** 📤️ The exports of one source: by name, the kinds it holds (a function can be overloaded) and the arities of every function form; `star` is set when it re-exports a whole module. */
export type DeclarationExportsV1=Readonly<{names:ReadonlyMap<string,Readonly<{kinds:readonly DeclarationKindV1[];arities:readonly DeclarationArityV1[];parameters:readonly string[];source?:Readonly<{specifier:string;imported:string}>}>>;star:boolean}>;
/** 🚧️ One way a declaration file differs from the script it declares. */
export type DeclarationDriftV1=Readonly<{kind:"undeclared"|"phantom"|"arity";name:string;expected?:string;found?:string}>;

const name="[A-Za-z0-9_$]+",pairs:Readonly<Record<string,string>>={"(":")","[":"]","{":"}"};

const withoutComments=(text:string):string=>{
 let out="",previous="";
 for(let index=0;index<text.length;index++){
  const character=text[index]!,next=text[index+1];
  if(character==="/"&&next==="*"){const end=text.indexOf("*/",index+2),stop=end<0?text.length:end+2;out+=text.slice(index,stop).replace(/[^\n]/gu,"");index=stop-1;continue;}
  if(character==="/"&&next==="/"){while(index<text.length&&text[index]!=="\n")index++;index--;continue;}
  if(character==="\""||character==="'"||character==="`"){
   const start=index;
   for(index++;index<text.length&&text[index]!==character;index++){if(text[index]==="\\")index++;else if(character!=="`"&&text[index]==="\n")break;}
   out+=text.slice(start,index+1);previous="x";continue;
  }
  if(character==="/"&&(previous===""||"(,=:[!&|?{};+-*%<>~^".includes(previous))){
   const start=index;
   let inClass=false;
   for(index++;index<text.length&&text[index]!=="\n";index++){
    if(text[index]==="\\")index++;
    else if(text[index]==="[")inClass=true;
    else if(text[index]==="]")inClass=false;
    else if(text[index]==="/"&&!inClass)break;
   }
   out+=text.slice(start,index+1);previous="x";continue;
  }
  out+=character;
  if(!/\s/u.test(character))previous=character;
 }
 return out;
};

const closing=(text:string,open:number):number=>{
 const stack:string[]=[];
 for(let index=open;index<text.length;index++){
  const character=text[index]!;
  if(character==="\""||character==="'"||character==="`"){
   for(index++;index<text.length&&text[index]!==character;index++)if(text[index]==="\\")index++;
   continue;
  }
  if(pairs[character]!==undefined)stack.push(pairs[character]!);
  else if(stack.length&&character===stack[stack.length-1]){stack.pop();if(!stack.length)return index;}
 }
 return -1;
};

const angle=(text:string,open:number):number=>{
 let depth=0;
 for(let index=open;index<text.length;index++){
  const character=text[index]!;
  if(character==="<")depth++;
  else if(character===">"&&text[index-1]!=="="&&text[index-1]!=="-"){depth--;if(!depth)return index;}
 }
 return -1;
};

const split=(text:string,typed:boolean):readonly string[]=>{
 const parts:string[]=[];
 let depth=0,angles=0,start=0;
 for(let index=0;index<text.length;index++){
  const character=text[index]!;
  if(character==="\""||character==="'"||character==="`"){for(index++;index<text.length&&text[index]!==character;index++)if(text[index]==="\\")index++;continue;}
  if(character==="("||character==="["||character==="{")depth++;
  else if(character===")"||character==="]"||character==="}")depth--;
  else if(typed&&character==="<"&&/[A-Za-z0-9_$]/u.test(text[index-1]??""))angles++;
  else if(typed&&character===">"&&text[index-1]!=="="&&text[index-1]!=="-"&&angles>0)angles--;
  else if(character===","&&depth===0&&angles===0){parts.push(text.slice(start,index));start=index+1;}
 }
 parts.push(text.slice(start));
 return parts.map(part=>part.trim()).filter(part=>part.length>0);
};

const parameters=(list:string,typed:boolean):readonly string[]=>split(list,typed).filter(part=>!/^this\s*[:?]/u.test(part));

const arityOf=(list:string,typed:boolean):DeclarationArityV1=>{
 const rows=parameters(list,typed);
 return Object.freeze({count:rows.filter(row=>!row.startsWith("...")).length,rest:rows.some(row=>row.startsWith("..."))});
};

const parameterName=(row:string,index:number):Readonly<{name:string;optional:boolean;rest:boolean}>=>{
 const rest=row.startsWith("..."),body=rest?row.slice(3):row,head=/^([A-Za-z_$][A-Za-z0-9_$]*)/u.exec(body);
 return {name:head?.[1]??(rest?"rest":"argument"+(index+1)),optional:/^[^:={]*\?/u.test(body)||/^[^:]*=/u.test(body)&&!/^[{[]/u.test(body)||/^[{[]/u.test(body)&&closing(body,0)>=0&&/^\s*=/u.test(body.slice(closing(body,0)+1)),rest};
};

const arrowParameters=(text:string,from:number):string|undefined=>{
 const head=/^\s*(?:async\b\s*)?(?:function\s*\*?\s*(?:[A-Za-z0-9_$]+)?\s*)?(?:<[^>(]*>\s*)?/u.exec(text.slice(from))!,start=from+head[0].length;
 if(text[start]==="("){
  const end=closing(text,start);
  if(end<0)return undefined;
  const after=text.slice(end+1).replace(/^\s*:[^={]*?(?==>|\{)/u,"");
  return /^\s*=>/u.test(after)||/function/u.test(head[0])?text.slice(start+1,end):undefined;
 }
 const single=/^([A-Za-z_$][A-Za-z0-9_$]*)\s*=>/u.exec(text.slice(start));
 return single?single[1]:undefined;
};

const siblingDeclarators=(text:string,from:number):readonly Readonly<{name:string;listed?:string}>[]=>{
 const found:{name:string;listed?:string}[]=[];
 let depth=0;
 for(let index=from;index<text.length;index++){
  const character=text[index]!;
  if(character==="\""||character==="'"||character==="`"){for(index++;index<text.length&&text[index]!==character;index++)if(text[index]==="\\")index++;continue;}
  if(character==="("||character==="["||character==="{")depth++;
  else if(character===")"||character==="]"||character==="}")depth--;
  else if(character===";"&&depth<=0)break;
  else if(character==="\n"&&depth<=0&&/^\n(?:export|function|const|let|var|import|class|async|\/\*|\/\/)/u.test(text.slice(index,index+12)))break;
  else if(character===","&&depth===0){const next=new RegExp(`^\\s*(${name})\\s*(=|,|;|$)`,"u").exec(text.slice(index+1));if(next)found.push({name:next[1]!,listed:next[2]==="="?arrowParameters(text,index+1+next[0].length):undefined});}
 }
 return found;
};

const add=(table:Map<string,{kinds:DeclarationKindV1[];arities:DeclarationArityV1[];parameters:string[];source?:{specifier:string;imported:string}}>,key:string,kind:DeclarationKindV1,arity?:DeclarationArityV1,rows?:readonly string[]):void=>{
 const row=table.get(key)??{kinds:[],arities:[],parameters:[]};
 if(!row.kinds.includes(kind))row.kinds.push(kind);
 if(arity!==undefined)row.arities.push(arity);
 if(rows!==undefined&&!row.parameters.length)row.parameters.push(...rows);
 table.set(key,row);
};

const braces=(table:Map<string,{kinds:DeclarationKindV1[];arities:DeclarationArityV1[];parameters:string[];source?:{specifier:string;imported:string}}>,list:string,typeOnly:boolean,specifier?:string):void=>{
 for(const part of list.split(",")){
  const item=part.trim().replace(/^type\s+/u,m=>m),typed=typeOnly||/^type\s+/u.test(item),clean=item.replace(/^type\s+/u,"").trim();
  if(!clean)continue;
  const parts=clean.split(/\s+as\s+/u).map(part=>part.trim()),exported=parts[parts.length-1]!;
  add(table,exported,typed?"type":"reexport");
  if(specifier!==undefined&&!typed)table.get(exported)!.source={specifier,imported:parts[0]!};
 }
};

/** 📜️ Reads what a `.mjs` script exports: functions with their parameter lists, arrow and function constants likewise, classes, other values, names re-exported in braces and the default export. */
export function scriptExportsV1(source:string):DeclarationExportsV1{
 const text=withoutComments(source),table=new Map<string,{kinds:DeclarationKindV1[];arities:DeclarationArityV1[];parameters:string[];source?:{specifier:string;imported:string}}>();
 let star=false;
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:async\\s+)?function\\s*\\*?\\s*(${name})\\s*\\(`,"gmu"))){
  const open=match.index!+match[0].length-1,end=closing(text,open);
  if(end>=0)add(table,match[1]!,"function",arityOf(text.slice(open+1,end),false),parameters(text.slice(open+1,end),false));
 }
 for(const match of text.matchAll(new RegExp(`^export\\s+class\\s+(${name})`,"gmu")))add(table,match[1]!,"class");
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:const|let|var)\\s+(${name})\\s*(?:=|;|,|$)`,"gmu"))){
  const equals=text.indexOf("=",match.index!+match[0].length-1),listed=equals>=0&&equals<match.index!+match[0].length?arrowParameters(text,equals+1):undefined;
  if(listed!==undefined)add(table,match[1]!,"function",arityOf(listed,false),parameters(listed,false));
  else add(table,match[1]!,"value");
  for(const other of siblingDeclarators(text,match.index!+match[0].length-1))if(other.listed!==undefined)add(table,other.name,"function",arityOf(other.listed,false),parameters(other.listed,false));else add(table,other.name,"value");
 }
 for(const match of text.matchAll(/^export\s+default\s+(?:async\s+)?function\s*\*?\s*(?:[A-Za-z0-9_$]+)?\s*\(/gmu)){
  const open=match.index!+match[0].length-1,end=closing(text,open);
  if(end>=0)add(table,"default","function",arityOf(text.slice(open+1,end),false),parameters(text.slice(open+1,end),false));
 }
 for(const match of text.matchAll(/^export\s*\{([^}]*)\}\s*(?:from\s*["']([^"']+)["'])?/gmu))braces(table,match[1]!,false,match[2]);
 if(/^export\s+default\b/mu.test(text)&&!table.has("default"))add(table,"default","value");
 for(const match of text.matchAll(new RegExp(`^export\\s*\\*\\s*(?:as\\s+(${name})\\s+)?from`,"gmu"))){if(match[1]!==undefined)add(table,match[1],"reexport");else star=true;}
 return Object.freeze({names:table as never,star});
}

/** 🗒️ Reads what a `.d.mts` declaration file declares: functions (every overload), constants (a function type counts as a function), classes, types, braces, the default export. */
export function declarationExportsV1(source:string):DeclarationExportsV1{
 const text=withoutComments(source),table=new Map<string,{kinds:DeclarationKindV1[];arities:DeclarationArityV1[];parameters:string[];source?:{specifier:string;imported:string}}>();
 let star=false;
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:declare\\s+)?(?:async\\s+)?function\\s*\\*?\\s*(${name})\\s*(<)?`,"gmu"))){
  let open=match.index!+match[0].length-(match[2]===undefined?0:1);
  if(match[2]!==undefined){const end=angle(text,open);if(end<0)continue;open=text.indexOf("(",end);}
  else open=text.indexOf("(",open);
  const end=open<0?-1:closing(text,open);
  if(end>=0)add(table,match[1]!,"function",arityOf(text.slice(open+1,end),true),parameters(text.slice(open+1,end),true));
 }
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:declare\\s+)?(?:abstract\\s+)?class\\s+(${name})`,"gmu")))add(table,match[1]!,"class");
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:declare\\s+)?(?:const|let|var)\\s+(${name})\\s*(:)?`,"gmu"))){
  if(match[2]===undefined){add(table,match[1]!,"value");continue;}
  const from=match.index!+match[0].length,generic=/^\s*<[^>(]*>/u.exec(text.slice(from)),start=from+(generic?.[0].length??0),opening=/^\s*\(/u.exec(text.slice(start));
  const open=opening?start+opening[0].length-1:-1,end=open>=0?closing(text,open):-1;
  if(end>=0&&/^\s*=>/u.test(text.slice(end+1)))add(table,match[1]!,"function",arityOf(text.slice(open+1,end),true),parameters(text.slice(open+1,end),true));
  else add(table,match[1]!,"value");
 }
 for(const match of text.matchAll(new RegExp(`^export\\s+(?:declare\\s+)?(?:const\\s+)?(?:enum|namespace|interface|type|module)\\s+(${name})`,"gmu")))add(table,match[1]!,"type");
 for(const match of text.matchAll(/^export\s*(type\s*)?\{([^}]*)\}/gmu))braces(table,match[2]!,match[1]!==undefined);
 for(const match of text.matchAll(/^export\s+default\s+(?:declare\s+)?(?:async\s+)?function\s*\*?\s*(?:[A-Za-z0-9_$]+)?\s*(?:<[^(]*>)?\s*\(/gmu)){
  const open=match.index!+match[0].length-1,end=closing(text,open);
  if(end>=0)add(table,"default","function",arityOf(text.slice(open+1,end),true),parameters(text.slice(open+1,end),true));
 }
 if(/^export\s+default\b/mu.test(text)&&!table.has("default"))add(table,"default","value");
 for(const match of text.matchAll(new RegExp(`^export\\s*(?:type\\s*)?\\*\\s*(?:as\\s+(${name})\\s+)?from`,"gmu"))){if(match[1]!==undefined)add(table,match[1],"reexport");else star=true;}
 return Object.freeze({names:table as never,star});
}

const show=(arity:DeclarationArityV1):string=>arity.count+(arity.rest?"+":"");

/** 🔍️ Compares a declaration file with its script: every runtime export of the script is declared, every function, constant or class the declaration states exists in the script, and a function takes as many parameters in both. */
export function declarationDriftV1(script:string,declaration:string):readonly DeclarationDriftV1[]{
 const exported=scriptExportsV1(script),declared=declarationExportsV1(declaration),rows:DeclarationDriftV1[]=[];
 for(const [key,row] of exported.names){
  const other=declared.names.get(key);
  if(other===undefined){rows.push({kind:"undeclared",name:key});continue;}
  if(!row.arities.length||!other.arities.length)continue;
  const want=row.arities[0]!;
  if(!other.arities.some(arity=>arity.count===want.count&&arity.rest===want.rest))rows.push({kind:"arity",name:key,expected:show(want),found:other.arities.map(show).join("|")});
 }
 if(!exported.star)for(const [key,row] of declared.names)if(!exported.names.has(key)&&row.kinds.some(kind=>kind==="function"||kind==="class"||kind==="value"))rows.push({kind:"phantom",name:key});
 return Object.freeze(rows.sort((left,right)=>left.name<right.name?-1:left.name>right.name?1:0));
}

/** 🏗️ Writes the declarations a script's undeclared exports need, from the script itself: a name re-exported from another module is re-exported by the declaration from the same module (so it carries that module's declaration), a function keeps its parameter names and count, a class its constructor, any other value `any`; the one place a declaration is derived instead of written by hand. */
export function declarationStubsV1(script:string,names:readonly string[]):string{
 const exported=scriptExportsV1(script),lines:string[]=[],reexports=new Map<string,string[]>();
 for(const key of names){
  const row=exported.names.get(key);
  if(row===undefined)continue;
  if(row.source!==undefined){(reexports.get(row.source.specifier)??reexports.set(row.source.specifier,[]).get(row.source.specifier)!).push(row.source.imported===key?key:`${row.source.imported} as ${key}`);continue;}
  if(key==="default"){lines.push("declare const defaultExportV1: any;","export default defaultExportV1;");continue;}
  if(row.kinds.includes("function")){
   const rows=row.parameters.map(parameterName);
   const lastRequired=rows.reduce((found,item,index)=>item.optional||item.rest?found:index,-1),text=rows.map((item,index)=>item.rest?`...${item.name}: any[]`:`${item.name}${item.optional&&index>lastRequired?"?":""}: any`).join(", ");
   lines.push(`export function ${key}(${text}): any;`);
  }else if(row.kinds.includes("class"))lines.push(`export class ${key} { constructor(...parameters: any[]); [member: string]: any; }`);
  else lines.push(`export const ${key}: any;`);
 }
 const heads=[...reexports].map(([specifier,list])=>`export {${list.join(", ")}} from ${JSON.stringify(specifier)};`),all=[...heads,...lines];
 return all.join("\n")+(all.length?"\n":"");
}
