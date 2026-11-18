/** 🔎️ Static census of the lease acquisition sites of the build tooling: every `acquire…ResourceLease` call, the resource name it can be resolved to (a literal, a same-file definition or a constant of the nearest schema), its mode, whether it can wait, and the sites that follow it inside its lexical extent. A site whose name only a caller knows is `dynamic`. */
import{readdirSync,readFileSync,existsSync}from"node:fs";
import{join,dirname}from"node:path";
const SKIP=new Set(["🧪️tests","node_modules","🧫️fixtures","target","dist","🤖️generated"]);
const NAMES=/\b(acquireResourceLease|acquireQueuedResourceLease|acquireUngatedResourceLease|acquireFreshResourceLeaseV1|acquireOrderedResourceLeaseV1|acquireOrderedQueuedResourceLeaseV1)\(/gu;
export function listSources(root,out=[]){let es;try{es=readdirSync(root,{withFileTypes:true});}catch{return out;}for(const e of es){if(e.name.startsWith(".")||SKIP.has(e.name))continue;const p=join(root,e.name);if(e.isDirectory())listSources(p,out);else if(/\.(ts|mjs|cjs)$/u.test(e.name)&&!/\.d\.m?ts$/u.test(e.name))out.push(p);}return out;}
function balanced(text,open){let depth=0,q=null;for(let i=open;i<text.length;i++){const c=text[i];if(q){if(c==="\\"){i++;continue;}if(c===q)q=null;continue;}if(c==='"'||c==="'"||c==="`"){q=c;continue;}if(c==="("||c==="{"||c==="[")depth++;else if(c===")"||c==="}"||c==="]"){depth--;if(depth===0)return i;}}return text.length;}
function topSplit(text){const out=[];let depth=0,q=null,cur="";for(let i=0;i<text.length;i++){const c=text[i];if(q){cur+=c;if(c==="\\"){cur+=text[++i]??"";continue;}if(c===q)q=null;continue;}if(c==='"'||c==="'"||c==="`"){q=c;cur+=c;continue;}if("([{".includes(c))depth++;if(")]}".includes(c))depth--;if(c===","&&depth===0){out.push(cur);cur="";continue;}cur+=c;}if(cur.trim())out.push(cur);return out;}
function schemaConstant(file,key){let dir=dirname(file);for(let up=0;up<3;up++){const path=join(dir,"🧬️schema","🔣️.json");if(existsSync(path)){const m=new RegExp('"'+key+'"\\s*:\\s*"([^"]+)"',"u").exec(readFileSync(path,"utf8"));if(m)return m[1];}dir=dirname(dir);}return undefined;}
export function leadingLiteral(text,expr,file,depth=0){
 expr=expr.trim();if(depth>4)return undefined;
 let m=/^(["'`])((?:\\.|(?!\1)[^\\$])*)(\1|\$\{)?/u.exec(expr);if(m)return m[2];
 m=/^JSON\.stringify\(\[\s*([^,\]]+)/u.exec(expr);if(m)return leadingLiteral(text,m[1],file,depth+1)===undefined?undefined:"json:"+leadingLiteral(text,m[1],file,depth+1);
 m=/^(\w+(?:\.\w+)*)(\(|\s*\+|$)/u.exec(expr);
 if(m){const id=m[1];
  const prop=/^[\w]+\.(\w+)$/u.exec(id);if(prop&&/resource|prefix|owner|registry|namespace|picker/iu.test(prop[1])){const lit=schemaConstant(file,prop[1]);if(lit!==undefined){const rest=/^[\w.]+\s*\+\s*(["'`])([^"'`$]*)/u.exec(expr);return lit+(rest?rest[2]:"");}}
  const defs=[new RegExp("(?:const|let|var|,)\\s*"+id.replace(/\./gu,"\\.")+"\\s*(?::[^=]+)?=\\s*([^;\\n]{1,400})","u"),new RegExp("function\\s+"+id+"\\s*\\([^)]*\\)\\s*(?::[^{]+)?\\{\\s*return\\s+([^;\\n]{1,400})","u"),new RegExp("(?:const|let)\\s+"+id+"\\s*=\\s*(?:async\\s*)?\\([^)]*\\)\\s*(?::[^=]+)?=>\\s*\\{?\\s*(?:return\\s+)?([^;\\n]{1,400})","u"),new RegExp("(?:const|let)\\s+"+id+"\\s*=\\s*\\w+\\s*=>\\s*([^;\\n]{1,400})","u")];
  for(const d of defs){const r=d.exec(text);if(r){const lit=leadingLiteral(text,r[1],file,depth+1);if(lit!==undefined)return lit;}}
 }
 return undefined;
}
export function censusOf(files){
 const sites=[];
 for(const file of files){const text=readFileSync(file,"utf8");if(!/Lease/u.test(text))continue;NAMES.lastIndex=0;let m;
  while((m=NAMES.exec(text))){const open=m.index+m[0].length-1;if(/function\s+\w*$/u.test(text.slice(Math.max(0,m.index-14),m.index)))continue;
   const close=balanced(text,open),arg=text.slice(open+1,close).trim(),body=arg.replace(/^\{/u,"").replace(/\}$/u,""),props=topSplit(body);
   const resProp=props.find(p=>/^\s*resource\b/u.test(p));let expr;
   if(resProp){const mm=/^\s*resource\s*:\s*([\s\S]*)$/u.exec(resProp);expr=mm?mm[1]:"resource";}
   const modeProp=props.find(p=>/^\s*mode\b/u.test(p)),mode=modeProp&&/exclusive/u.test(modeProp)?(/shared/u.test(modeProp)?"either":"exclusive"):modeProp&&/shared/u.test(modeProp)?"shared":"unknown",timeoutZero=props.some(p=>/^\s*timeoutMs\s*:\s*0\s*$/u.test(p));
   const line=text.slice(0,m.index).split("\n").length,before=text.slice(Math.max(0,m.index-80),m.index),varm=/(?:const|let)\s+(\w+)\s*(?::[^=]+)?=\s*(?:await\s+)?$/u.exec(before)||/(\w+)\s*=\s*(?:await\s+)?$/u.exec(before);
   let literal=expr===undefined?undefined:leadingLiteral(text,expr==="resource"?"resource":expr,file);
   sites.push({file,line,index:m.index,end:close,call:m[1],expr:expr?.slice(0,120),literal,mode,nonblocking:timeoutZero,variable:varm?.[1]});}
 }
 return sites;
}

export function enclosingEnd(text,index){let depth=0,q=null;for(let i=index;i<text.length;i++){const c=text[i];if(q){if(c==="\\"){i++;continue;}if(c===q)q=null;continue;}if(c==='"'||c==="'"||c==="`"){q=c;continue;}if(c==="{")depth++;else if(c==="}"){if(depth===0)return i;depth--;}}return text.length;}
export function pairsOf(sites){
 const byFile=new Map();for(const s of sites){if(!byFile.has(s.file))byFile.set(s.file,[]);byFile.get(s.file).push(s);}
 const pairs=[];
 for(const[file,list]of byFile){const text=readFileSync(file,"utf8");
  for(const a of list){if(a.literal===undefined||a.nonblocking&&false)continue;
   let end=enclosingEnd(text,a.end);
   if(a.variable){const rel=text.indexOf(a.variable+".release(",a.end);if(rel>=0&&rel<end)end=rel;}
   for(const b of list){if(b===a||b.index<=a.end||b.index>=end)continue;pairs.push({file,held:a,wanted:b});}
  }}
 return pairs;
}
