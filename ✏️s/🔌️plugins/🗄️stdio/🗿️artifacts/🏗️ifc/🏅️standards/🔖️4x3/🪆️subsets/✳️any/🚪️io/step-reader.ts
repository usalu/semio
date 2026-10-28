/** 📥️ Bounded lexical scans decode one unpublished IFC STEP graph. */
import type {IfcEntity,IfcValue,IfcComplexType} from "../🧬️schema/📸️snapshot/🟦️.ts";
import {ifcCheckpoint,ifcIoLimit,IfcStepError,type IfcIoControl} from "./control.ts";
import {ifcReal} from "./step-values.ts";
import type {IfcStepSource} from "./step-source.ts";
import {part21AlphabetCharacter} from "../../../../../../../📇️registry/🧬️contract/📐️part21/🔤️lexer-and-parse/alphabets/🟦️.ts";
const letter=(char:string|undefined):boolean=>char!==undefined&&/[A-Za-z_]/.test(char);
const digit=(char:string|undefined):boolean=>char!==undefined&&char>="0"&&char<="9";

export class IfcStepReader {
 at=0;values=0;#last=0;readonly batch:number;readonly depth:number;readonly numeric:number;readonly valueLimit:number;
 readonly text:string;
 constructor(readonly source:IfcStepSource,readonly control:IfcIoControl){this.text=source.text;this.batch=ifcIoLimit(control.batchSize,256,"batchSize");this.depth=ifcIoLimit(control.maxDepth,256,"maxDepth");this.numeric=ifcIoLimit(control.maxNumericCodeUnits,4096,"maxNumericCodeUnits");this.valueLimit=ifcIoLimit(control.maxValues,10_000_000,"maxValues");}
 origin():number{return this.source.offset(this.at);}
 fail(code:string,message:string):never{throw new IfcStepError(code,message,this.origin());}
 async checkpoint(phase:string):Promise<void>{if(this.at-this.#last>=4096){this.#last=this.at;await ifcCheckpoint(this.control,phase,this.origin(),this.source.originalLength);}}
 async skip():Promise<void>{
  while(this.at<this.text.length){if(this.text[this.at]===" "){this.at++;if(this.at-this.#last>=4096)await this.checkpoint("parse-trivia");continue;}if(this.text.startsWith("\\N\\",this.at)||this.text.startsWith("\\F\\",this.at)){this.at+=3;if(this.at-this.#last>=4096)await this.checkpoint("parse-trivia");continue;}if(!this.text.startsWith("/*",this.at))return;this.at+=2;while(!this.text.startsWith("*/",this.at)){if(this.at>=this.text.length)this.fail("step.comment","Unterminated comment");this.at++;if(this.at-this.#last>=4096)await this.checkpoint("parse-trivia");}this.at+=2;}
 }
 async is(token:string):Promise<boolean>{await this.skip();return this.text.slice(this.at,this.at+token.length).toUpperCase()===token;}
 async take(token:string):Promise<void>{if(!await this.is(token))this.fail("step.syntax",`Expected ${token}`);this.at+=token.length;}
 async word():Promise<string>{
  await this.skip();if(!letter(this.text[this.at]))this.fail("step.identifier","Expected identifier");let start=this.at,result="";
  while(letter(this.text[this.at])||digit(this.text[this.at])){this.at++;if(this.at-start===4096){result+=this.text.slice(start,this.at).toUpperCase();start=this.at;await this.checkpoint("parse-identifier");}}
  return result+this.text.slice(start,this.at).toUpperCase();
 }
 async id():Promise<bigint>{await this.take("#");const start=this.at;while(digit(this.text[this.at])){if(this.at-start===20)this.fail("step.id","Instance id exceeds unsigned 64-bit range");this.at++;}if(this.at===start)this.fail("step.id","Expected positive instance id");const id=BigInt(this.text.slice(start,this.at));if(id===0n||id>18446744073709551615n)this.fail("step.id","Instance id is outside unsigned 64-bit range");return id;}
 async string():Promise<string>{
  await this.take("'");let result="",chunk="",page="A",pending=false;
  const append=(value:string):void=>{for(let i=0;i<value.length;i++){const code=value.charCodeAt(i);if(pending){if(code<0xdc00||code>0xdfff)this.fail("step.string","Unpaired Unicode surrogate");pending=false;}else if(code>=0xd800&&code<=0xdbff)pending=true;else if(code>=0xdc00&&code<=0xdfff)this.fail("step.string","Unpaired Unicode surrogate");chunk+=value[i]!;if(chunk.length>=4096&&!pending){result+=chunk;chunk="";}}};
  while(this.at<this.text.length){if(this.at-this.#last>=4096)await this.checkpoint("parse-string");const char=this.text[this.at++]!;
   if(char==="'"){if(this.text[this.at]==="'"){this.at++;append("'");continue;}if(pending)this.fail("step.string","Unpaired Unicode surrogate");return result+chunk;}
   if(char!=="\\"){if(char.charCodeAt(0)<32)this.fail("step.string","Control character in string");append(char);continue;}
   if(pending)this.fail("step.string","Unpaired Unicode surrogate");if(this.text[this.at]==="\\"){this.at++;append("\\");continue;}if(this.text.startsWith("N\\",this.at)||this.text.startsWith("F\\",this.at)){this.at+=2;continue;}
   const directive=/^P([A-I])\\/.exec(this.text.slice(this.at,this.at+3));if(directive){page=directive[1]!;this.at+=3;continue;}
   if(this.text.startsWith("S\\",this.at)){this.at+=2;const character=part21AlphabetCharacter(page,this.text.charCodeAt(this.at++));if(character===undefined)this.fail("step.string",`Undefined ISO 8859 page ${page} character`);append(character);continue;}
   if(this.text.startsWith("X\\",this.at)){const hex=this.text.slice(this.at+2,this.at+4);if(!/^[0-9a-f]{2}$/i.test(hex))this.fail("step.string","Invalid hexadecimal escape");append(String.fromCharCode(parseInt(hex,16)));this.at+=4;continue;}
   const width=this.text.startsWith("X2\\",this.at)?4:this.text.startsWith("X4\\",this.at)?8:0;if(!width)this.fail("step.string","Unknown string escape");this.at+=3;let count=0;
   while(true){while(this.text.startsWith("\\N\\",this.at)||this.text.startsWith("\\F\\",this.at)){this.at+=3;if(this.at-this.#last>=4096)await this.checkpoint("parse-unicode");}if(this.text.startsWith("\\X0\\",this.at))break;let hex="";while(hex.length<width){if(this.text.startsWith("\\N\\",this.at)||this.text.startsWith("\\F\\",this.at)){this.at+=3;if(this.at-this.#last>=4096)await this.checkpoint("parse-unicode");continue;}const char=this.text[this.at];if(char===undefined||!/[0-9a-f]/i.test(char))this.fail("step.string","Invalid or unterminated Unicode escape");hex+=char;this.at++;}const code=parseInt(hex,16);if(code>0x10ffff||width===8&&code>=0xd800&&code<=0xdfff)this.fail("step.string","Invalid Unicode scalar");append(width===4?String.fromCharCode(code):String.fromCodePoint(code));count++;if(this.at-this.#last>=4096)await this.checkpoint("parse-unicode");}
   if(!count||pending)this.fail("step.string","Empty Unicode escape or unpaired Unicode surrogate");this.at+=4;
  }return this.fail("step.string","Unterminated string");
 }
 async args(depth=-1):Promise<IfcValue[]>{if(depth>this.depth)this.fail("step.depth","Argument nesting limit exceeded");await this.take("(");const values:IfcValue[]=[];if(!await this.is(")")){values.push(await this.value(depth+1));while(await this.is(",")){await this.take(",");values.push(await this.value(depth+1));}}await this.take(")");return values;}
 async value(depth:number):Promise<IfcValue>{
  if(depth>this.depth)this.fail("step.depth","Argument nesting limit exceeded");
  if(++this.values>this.valueLimit)this.fail("step.valueLimit","Value limit exceeded");if(this.values%this.batch===0)await ifcCheckpoint(this.control,"parse-values",this.origin(),this.source.originalLength);await this.skip();const char=this.text[this.at];
  if(char==="$"){this.at++;return{kind:"unset"};}if(char==="*"){this.at++;return{kind:"derived"};}if(char==="#")return{kind:"reference",value:await this.id()};if(char==="'")return{kind:"string",value:await this.string()};if(char==="(")return{kind:"aggregate",value:await this.args(depth)};
  if(char==="."&&letter(this.text[this.at+1])){this.at++;const value=await this.word();if(this.text[this.at]!==".")this.fail("step.syntax","Enumeration requires a contiguous closing dot");this.at++;return{kind:"enum",value};}
  if(digit(char)||char==="+"||char==="-"){const start=this.at;if(char==="+"||char==="-")this.at++;let real=false;const digits=async()=>{while(digit(this.text[this.at])){this.at++;if(this.at-start>this.numeric)this.fail("step.numericLimit","Numeric token exceeds admitted code units");if(this.at-this.#last>=4096)await this.checkpoint("parse-number");}};const first=this.at;await digits();if(this.at===first)this.fail("step.number","Numeric token requires digits");if(this.text[this.at]==="."){real=true;this.at++;await digits();}if(/[Ee]/.test(this.text[this.at]??"")){if(!real)this.fail("step.number","STEP real mantissa requires a decimal point");real=true;this.at++;if(/[+-]/.test(this.text[this.at]??""))this.at++;const exponent=this.at;await digits();if(this.at===exponent)this.fail("step.number","Exponent requires digits");}if(this.at-start>this.numeric)this.fail("step.numericLimit","Numeric token exceeds admitted code units");const token=this.text.slice(start,this.at);if(real)return ifcReal(Number(token));if(token.replace(/^[+-]/,"").length>19)this.fail("step.integer","Integer exceeds signed 64-bit range");const integer=BigInt(token);if(integer< -9223372036854775808n||integer>9223372036854775807n)this.fail("step.integer","Integer outside signed 64-bit range");return{kind:"integer",value:integer};}
  if(char==='"'){this.at++;let result="",chunk="";while(this.text[this.at]!=='"'){if(this.at>=this.text.length)this.fail("step.binary","Unterminated binary literal");if(this.text.startsWith("\\N\\",this.at)||this.text.startsWith("\\F\\",this.at)){this.at+=3;if(this.at-this.#last>=4096)await this.checkpoint("parse-binary");continue;}if(!/[0-9A-Fa-f]/.test(this.text[this.at]!))this.fail("step.binary","Invalid hexadecimal binary literal");chunk+=this.text[this.at++]!;if(chunk.length===4096){result+=chunk.toUpperCase();chunk="";await this.checkpoint("parse-binary");}}result+=chunk.toUpperCase();this.at++;if(!result.length||!/[0-3]/.test(result[0]!)||result.length===1&&result!=="0"||result.length>1&&parseInt(result[1]!,16)>=2**(4-Number(result[0])))this.fail("step.binary","Invalid binary literal or non-zero unused high bits");return{kind:"binary",value:result};}
  const name=await this.word();return{kind:"typedValue",value:{name,items:await this.args(depth)}};
 }
 async entity():Promise<IfcEntity>{const id=await this.id();await this.take("=");if(!await this.is("(")){const name=await this.word(),args=await this.args();await this.take(";");return{id,name,args,complex:[]};}await this.take("(");let first:IfcComplexType|undefined;const complex:IfcComplexType[]=[];while(!await this.is(")")){const name=await this.word(),member={name,args:await this.args()};if(first)complex.push(member);else first=member;}if(!first)this.fail("step.complex","Empty complex instance");await this.take(")");await this.take(";");return{id,name:first.name,args:first.args,complex};}
}
