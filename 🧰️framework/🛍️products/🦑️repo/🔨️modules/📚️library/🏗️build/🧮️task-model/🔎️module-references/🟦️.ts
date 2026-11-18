/** 🔎️ One literal reference of a source text: a module specifier, or a file or directory named with `new URL("./…", import.meta.url)`.
 * @see ./🧬️schema/🔣️.json */
export type ModuleReferenceV1=Readonly<{kind:"module"|"file-url"|"directory-url";specifier:string}>;

const EOF=1,OTHER=0,WORD=2,STR=3,NOSUB=4,HEAD=5,MIDDLE=6,TAIL=7,LBRACE=8,RBRACE=9,LPAREN=10,RPAREN=11,LBRACKET=12,RBRACKET=13,COMMA=14,DOT=15,EQUALS=16,STAR=17,VALUE=18;
const OPERATORS=["**=","...","!==","===","&&=","||=","??=","<<=","=>","==","!=","<=","<<","+=","-=","*=","/=","%=","&=","|=","^=","++","--","&&","||","??","**"];
const SINGLE:Readonly<Record<string,number>>={"{":LBRACE,"}":RBRACE,"(":LPAREN,")":RPAREN,"[":LBRACKET,"]":RBRACKET,",":COMMA,".":DOT,"=":EQUALS,"*":STAR};
const IDENTIFIER_START=/[\p{ID_Start}$_]/u,IDENTIFIER_PART=/[\p{ID_Continue}$‌‍]/u,NUMBER=/^(?:0[xX][0-9a-fA-F_]*|0[bB][01_]*|0[oO][0-7_]*|(?:\d[\d_]*)?(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?)n?/u,CONFLICT=/^(.)\1{6}/u;
const WHITESPACE=new Set([9,11,12,32,133,160,5760,8239,8287,12288,65279]),isWhitespace=(code:number)=>WHITESPACE.has(code)||code>=8192&&code<=8203,isLineBreak=(code:number)=>code===10||code===13||code===8232||code===8233,isDigit=(code:number)=>code>=48&&code<=57,isOctal=(code:number)=>code>=48&&code<=55,isHex=(code:number)=>isDigit(code)||code>=65&&code<=70||code>=97&&code<=102;
const REGEX_AFTER=new Set(["return","typeof","instanceof","in","of","new","delete","void","throw","case","do","else","yield","await"]);

type Token=Readonly<{kind:number;value:string;start:number;end:number}>;

/** 🔤️ Scans JavaScript and TypeScript text into the tokens the reference rules read, with the lexical rules of the TypeScript scanner: string and template literals with their cooked values, comments as trivia, one-token operators, no regular-expression literals unless asked for.
 * @see https://github.com/microsoft/TypeScript/blob/main/src/compiler/scanner.ts */
function createTokenizerV1(text:string,regularExpressions:boolean){
 let pos=0,kind=OTHER,value="",start=0,previous:Token|undefined;
 const end=text.length,at=(index:number)=>text.charCodeAt(index),point=(index:number)=>text.codePointAt(index)??0;
 const escape=(report:boolean):string=>{
  const from=pos;pos++;if(pos>=end)return"";const ch=at(pos);pos++;
  const octal=(more:number)=>{for(let count=0;count<more&&pos<end&&isOctal(at(pos));count++)pos++;return report?String.fromCharCode(parseInt(text.substring(from+1,pos),8)):text.substring(from,pos);};
  switch(ch){
   case 48:if(pos>=end||!isDigit(at(pos)))return"\0";return octal(2);
   case 49:case 50:case 51:return octal(2);
   case 52:case 53:case 54:case 55:return octal(1);
   case 56:case 57:return report?String.fromCharCode(ch):text.substring(from,pos);
   case 98:return"\b";case 116:return"\t";case 110:return"\n";case 118:return"\v";case 102:return"\f";case 114:return"\r";case 39:return"'";case 34:return'"';
   case 117:{
    if(pos<end&&at(pos)===123){pos-=2;return unicode();}
    for(;pos<from+6;pos++)if(!(pos<end&&isHex(at(pos))))return text.substring(from,pos);
    return String.fromCharCode(parseInt(text.substring(from+2,pos),16));
   }
   case 120:for(;pos<from+4;pos++)if(!(pos<end&&isHex(at(pos))))return text.substring(from,pos);return String.fromCharCode(parseInt(text.substring(from+2,pos),16));
   case 13:if(pos<end&&at(pos)===10)pos++;return"";
   case 10:case 8232:case 8233:return"";
   default:return String.fromCharCode(ch);
  }
 };
 const unicode=():string=>{
  const from=pos;pos+=3;const digits=pos;while(pos<end&&isHex(at(pos)))pos++;
  const code=pos>digits?parseInt(text.substring(digits,pos),16):-1;let invalid=code<0||code>1114111;
  if(pos>=end)invalid=true;else if(at(pos)===125)pos++;else invalid=true;
  return invalid?text.substring(from,pos):String.fromCodePoint(code);
 };
 const identifierParts=():string=>{
  let result="";
  while(pos<end){
   const code=point(pos),char=String.fromCodePoint(code);
   if(IDENTIFIER_PART.test(char)){result+=char;pos+=char.length;continue;}
   if(code!==92)break;
   const extended=/^\\u\{([0-9a-fA-F]+)\}/u.exec(text.slice(pos,pos+14)),short=/^\\u([0-9a-fA-F]{4})/u.exec(text.slice(pos,pos+6)),cooked=extended?parseInt(extended[1]!,16):short?parseInt(short[1]!,16):-1;
   if(cooked<0||cooked>1114111||!IDENTIFIER_PART.test(String.fromCodePoint(cooked)))break;
   result+=String.fromCodePoint(cooked);pos+=(extended??short)![0].length;
  }
  return result;
 };
 const string=():string=>{
  const quote=at(pos);pos++;let result="",from=pos;
  for(;;){
   if(pos>=end){result+=text.substring(from,pos);break;}
   const ch=at(pos);
   if(ch===quote){result+=text.substring(from,pos);pos++;break;}
   if(ch===92){result+=text.substring(from,pos);result+=escape(true);from=pos;continue;}
   if(ch===10||ch===13){result+=text.substring(from,pos);break;}
   pos++;
  }
  return result;
 };
 const template=():number=>{
  const opening=at(pos)===96;pos++;let from=pos,contents="";
  for(;;){
   if(pos>=end){contents+=text.substring(from,pos);value=contents;return opening?NOSUB:TAIL;}
   const ch=at(pos);
   if(ch===96){contents+=text.substring(from,pos);pos++;value=contents;return opening?NOSUB:TAIL;}
   if(ch===36&&pos+1<end&&at(pos+1)===123){contents+=text.substring(from,pos);pos+=2;value=contents;return opening?HEAD:MIDDLE;}
   if(ch===92){contents+=text.substring(from,pos);contents+=escape(false);from=pos;continue;}
   if(ch===13){contents+=text.substring(from,pos);pos++;if(pos<end&&at(pos)===10)pos++;contents+="\n";from=pos;continue;}
   pos++;
  }
 };
 const conflict=():boolean=>{
  if(!(pos===0||isLineBreak(at(pos-1)))||!(pos+7<end))return false;
  const match=CONFLICT.exec(text.slice(pos,pos+7));
  return match!==null&&(match[1]==="="||at(pos+7)===32);
 };
 const skipConflict=()=>{
  const ch=text[pos]!;
  if(ch==="<"||ch===">"){while(pos<end&&!isLineBreak(at(pos)))pos++;return;}
  while(pos<end){
   const current=text[pos]!;
   if((current==="="||current===">")&&current!==ch){const saved=pos;if(conflict())break;pos=saved;}
   pos++;
  }
 };
 const regexAllowed=():boolean=>previous===undefined||[OTHER,COMMA,LPAREN,LBRACKET,LBRACE,EQUALS,STAR,HEAD,MIDDLE].includes(previous.kind)||previous.kind===WORD&&REGEX_AFTER.has(previous.value);
 const regex=():boolean=>{
  let index=pos+1,inClass=false;
  for(;index<end;index++){
   const ch=at(index);
   if(isLineBreak(ch))return false;
   if(ch===92){index++;continue;}
   if(ch===91)inClass=true;else if(ch===93)inClass=false;else if(ch===47&&!inClass){index++;while(index<end&&IDENTIFIER_PART.test(text[index]!))index++;pos=index;return true;}
  }
  return false;
 };
 const scan=():number=>{
  for(;;){
   start=pos;value="";
   if(pos>=end)return kind=EOF;
   const ch=at(pos);
   if(pos===0&&ch===35&&at(1)===33){while(pos<end&&!isLineBreak(at(pos)))pos++;continue;}
   if(isWhitespace(ch)||isLineBreak(ch)){pos++;continue;}
   if(ch===34||ch===39){value=string();return finish(STR);}
   if(ch===96)return finish(template());
   if(ch===47){
    if(at(pos+1)===47){pos+=2;while(pos<end&&!isLineBreak(at(pos)))pos++;continue;}
    if(at(pos+1)===42){pos+=2;while(pos<end&&!(at(pos)===42&&at(pos+1)===47))pos++;pos=Math.min(end,pos+2);continue;}
    if(regularExpressions&&regexAllowed()&&regex())return finish(VALUE);
    pos+=at(pos+1)===61?2:1;return finish(OTHER);
   }
   if((ch===60||ch===61||ch===62||ch===124)&&conflict()){skipConflict();continue;}
   if(isDigit(ch)||ch===46&&isDigit(at(pos+1))){const match=NUMBER.exec(text.slice(pos,pos+256))!;pos+=Math.max(1,match[0].length);return finish(VALUE);}
   if(ch===46){if(at(pos+1)===46&&at(pos+2)===46){pos+=3;return finish(OTHER);}pos++;return finish(DOT);}
   if(ch===62){pos++;return finish(OTHER);}
   if(ch===63&&at(pos+1)===46&&!isDigit(at(pos+2))){pos+=2;return finish(OTHER);}
   const operator=OPERATORS.find(candidate=>text.startsWith(candidate,pos));
   if(operator!==undefined){pos+=operator.length;return finish(OTHER);}
   const char=text[pos]!;
   if(char in SINGLE){pos++;return finish(SINGLE[char]!);}
   if(ch===65533){pos=end;return finish(OTHER);}
   if(ch===35){pos++;const code=point(pos);if(IDENTIFIER_START.test(String.fromCodePoint(code))||code===92){const first=String.fromCodePoint(code);pos+=first.length;identifierParts();}return finish(VALUE);}
   const code=point(pos),first=String.fromCodePoint(code);
   if(IDENTIFIER_START.test(first)){pos+=first.length;value=first+identifierParts();return finish(WORD);}
   if(code===92){
    const extended=/^\\u\{([0-9a-fA-F]+)\}/u.exec(text.slice(pos,pos+14)),short=/^\\u([0-9a-fA-F]{4})/u.exec(text.slice(pos,pos+6)),cooked=extended?parseInt(extended[1]!,16):short?parseInt(short[1]!,16):-1;
    if(cooked>=0&&cooked<=1114111&&IDENTIFIER_START.test(String.fromCodePoint(cooked))){pos+=(extended??short)![0].length;value=String.fromCodePoint(cooked)+identifierParts();return finish(WORD);}
   }
   pos+=first.length;return finish(OTHER);
  }
 };
 const finish=(next:number):number=>{kind=next;return kind;};
 const remember=()=>{previous={kind,value,start,end:pos};};
 return{
  scan:()=>{const result=scan();remember();return result;},
  rescanTemplate:()=>{pos=start;const result=template();kind=result;remember();return result;},
  lookAhead:<T>(body:()=>T):T=>{const saved={pos,kind,value,start,previous};try{return body();}finally{({pos,kind,value,start,previous}=saved);}},
  get kind(){return kind;},get value(){return value;},get start(){return start;},get end(){return pos;}
 };
}

/** 🧾️ The module specifiers of one source text, in order of occurrence, as the TypeScript `preProcessFile(text, true, true)` reads them: import and export declarations with a string, `import(...)`, `require(...)`, AMD `define([...])` and ambient `declare module "…"` names. The lexical limits are kept on purpose (no regular-expression literals, no scopes), so the rows equal the compiler's rows for every text.
 * @see https://github.com/microsoft/TypeScript/blob/main/src/services/preProcess.ts */
export function scanModuleSpecifiersV1(text:string):readonly string[]{
 const scanner=createTokenizerV1(text,false),imported:string[]=[],ambient:{value:string;depth:number}[]=[];
 let last=OTHER,current=OTHER,braces=0,external=false;
 const next=():number=>{last=current;current=scanner.scan();if(current===LBRACE)braces++;else if(current===RBRACE)braces--;return current;};
 const word=(token:number,expected:string)=>token===WORD&&scanner.value===expected;
 const record=()=>{imported.push(scanner.value);if(braces===0)external=true;};
 const afterFrom=(token:number)=>{if(word(token,"from")){token=next();if(token===STR)record();}};
 const consumeDeclare=():boolean=>{
  if(!word(scanner.kind,"declare"))return false;
  let token=next();
  if(word(token,"module")){token=next();if(token===STR)ambient.push({value:scanner.value,depth:braces});}
  return true;
 };
 const consumeRequire=(skipCurrent:boolean,templates:boolean):boolean=>{
  let token=skipCurrent?next():scanner.kind;
  if(word(token,"require")){token=next();if(token===LPAREN){token=next();if(token===STR||templates&&token===NOSUB)record();}return true;}
  return false;
 };
 const consumeImport=():boolean=>{
  if(last===DOT)return false;
  let token=scanner.kind;
  if(!word(token,"import"))return false;
  token=next();
  if(token===LPAREN){token=next();if(token===STR||token===NOSUB){record();return true;}}
  else if(token===STR){record();return true;}
  else{
   if(word(token,"type")&&scanner.lookAhead(()=>{const token2=scanner.scan();return token2===STAR||token2===LBRACE||token2===WORD&&scanner.value!=="from";}))token=next();
   if(token===WORD){
    token=next();
    if(word(token,"from")){token=next();if(token===STR){record();return true;}}
    else if(token===EQUALS){if(consumeRequire(true,false))return true;}
    else if(token===COMMA)token=next();
    else return true;
   }
   if(token===LBRACE){
    token=next();while(token!==RBRACE&&token!==EOF)token=next();
    if(token===RBRACE){token=next();afterFrom(token);}
   }else if(token===STAR){
    token=next();
    if(word(token,"as")){token=next();if(token===WORD){token=next();afterFrom(token);}}
   }
  }
  return true;
 };
 const consumeExport=():boolean=>{
  if(!word(scanner.kind,"export"))return false;
  if(braces===0)external=true;
  let token=next();
  if(word(token,"type")&&scanner.lookAhead(()=>{const token2=scanner.scan();return token2===STAR||token2===LBRACE;}))token=next();
  if(token===LBRACE){
   token=next();while(token!==RBRACE&&token!==EOF)token=next();
   if(token===RBRACE){token=next();afterFrom(token);}
  }else if(token===STAR){
   token=next();afterFrom(token);
  }else if(word(token,"import")){
   token=next();
   if(word(token,"type")&&scanner.lookAhead(()=>scanner.scan()===WORD))token=next();
   if(token===WORD){token=next();if(token===EQUALS)if(consumeRequire(true,false))return true;}
  }
  return true;
 };
 const consumeDefine=():boolean=>{
  if(!word(scanner.kind,"define"))return false;
  let token=next();
  if(token!==LPAREN)return true;
  token=next();
  if(token===STR||token===NOSUB){token=next();if(token===COMMA)token=next();else return true;}
  if(token!==LBRACKET)return true;
  token=next();
  while(token!==RBRACKET&&token!==EOF){if(token===STR||token===NOSUB)record();token=next();}
  return true;
 };
 next();
 for(;;){
  if(scanner.kind===EOF)break;
  if(scanner.kind===HEAD){
   const stack=[HEAD];
   loop:while(stack.length){
    const token=scanner.scan();
    switch(token){
     case EOF:break loop;
     case WORD:if(scanner.value==="import")consumeImport();break;
     case HEAD:stack.push(token);break;
     case LBRACE:stack.push(token);break;
     case RBRACE:
      if(stack[stack.length-1]===HEAD){if(scanner.rescanTemplate()===TAIL)stack.pop();}else stack.pop();
      break;
    }
   }
   next();
  }
  if(consumeDeclare()||consumeImport()||consumeExport()||consumeRequire(false,true)||consumeDefine())continue;
  next();
 }
 if(external)return[...imported,...ambient.map(row=>row.value)];
 return[...imported,...ambient.filter(row=>row.depth!==0).map(row=>row.value)];
}

/** 🔗️ The literal paths a source text resolves against itself with `new URL("./…", import.meta.url)`, in order of occurrence; the expression is read as tokens, with regular-expression literals recognised, so a comment, a string or a regular expression never counts.
 * @see ./🧬️schema/🔣️.json */
export function scanUrlReferencesV1(text:string):readonly string[]{
 if(!text.includes("URL"))return[];
 const scanner=createTokenizerV1(text,true),tokens:Token[]=[],stack:number[]=[];
 for(let kind=scanner.scan();kind!==EOF;kind=scanner.scan()){
  tokens.push({kind,value:scanner.value,start:scanner.start,end:scanner.end});
  if(kind===HEAD||kind===LBRACE)stack.push(kind);
  else if(kind===RBRACE){
   if(stack[stack.length-1]===HEAD){const rescanned=scanner.rescanTemplate();tokens[tokens.length-1]={kind:rescanned,value:scanner.value,start:scanner.start,end:scanner.end};if(rescanned===TAIL)stack.pop();}
   else stack.pop();
  }
 }
 const urls:string[]=[],is=(index:number,kind:number,value?:string)=>tokens[index]?.kind===kind&&(value===undefined||tokens[index]!.value===value),exact=(from:number,count:number)=>{for(let index=from;index<from+count-1;index++)if(tokens[index]!.end!==tokens[index+1]!.start)return false;return true;};
 for(let index=0;index<tokens.length;index++){
  if(!(is(index,WORD,"new")&&is(index+1,WORD,"URL")&&is(index+2,LPAREN)&&(is(index+3,STR)||is(index+3,NOSUB))&&is(index+4,COMMA)&&is(index+5,WORD,"import")&&is(index+6,DOT)&&is(index+7,WORD,"meta")&&is(index+8,DOT)&&is(index+9,WORD,"url")&&exact(index+5,5)))continue;
  const closed=is(index+10,RPAREN)||is(index+10,COMMA)&&is(index+11,RPAREN);
  if(closed&&tokens[index+3]!.value.startsWith("."))urls.push(tokens[index+3]!.value);
 }
 return urls;
}

/** 🔎️ Classifies the exact literal module and URL references of one source text as portable source facts, each pair once, modules before URLs.
 * @see ./🧬️schema/🔣️.json */
export function classifyModuleReferencesV1(text:string):readonly ModuleReferenceV1[]{
 const rows:ModuleReferenceV1[]=[],seen=new Set<string>(),add=(kind:ModuleReferenceV1["kind"],specifier:string)=>{const identity=kind+"\0"+specifier;if(seen.has(identity))return;seen.add(identity);rows.push({kind,specifier});};
 for(const specifier of scanModuleSpecifiersV1(text))add("module",specifier);
 for(const specifier of scanUrlReferencesV1(text))add(new URL(specifier,"file:///engine.ts").pathname.endsWith("/")?"directory-url":"file-url",specifier);
 return rows;
}
