/** 🧮️ Controlled baseline JPEG component projection before color conversion. */
import {NativeDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
export interface JpgDecodedComponents{width:number;height:number;componentIds:number[];samples:Uint8Array}
interface Component{id:number;h:number;v:number;quant:number;dc:number;ac:number;previous:number;stride:number;plane:Float64Array}
interface Huffman{codes:ReadonlyMap<number,number>;length:number}
const zigzag=[0,1,8,16,9,2,3,10,17,24,32,25,18,11,4,5,12,19,26,33,40,48,41,34,27,20,13,6,7,14,21,28,35,42,49,56,57,50,43,36,29,22,15,23,30,37,44,51,58,59,52,45,38,31,39,46,53,60,61,54,47,55,62,63] as const;
function refusal(message:string):never{throw Error("JPEG components: "+message)}
function extent(...factors:number[]):number{const value=factors.reduce((a,b)=>a*b,1);if(!Number.isSafeInteger(value)||value<0)refusal("working extent overflow");return value}
class Entropy{
 at:number;private remaining=0;private accumulator=0;
 constructor(private source:Uint8Array,start:number){this.at=start}
 bit():number{if(this.remaining===0){let byte=this.source[this.at++];if(byte===undefined)refusal("truncated entropy");if(byte===255){if(this.source[this.at++]!==0)refusal("marker inside entropy");byte=255;}this.accumulator=byte;this.remaining=8;}return(this.accumulator>>>--this.remaining)&1;}
 bits(count:number):number{let value=0;for(let i=0;i<count;i++)value=value*2+this.bit();return value}
 symbol(table:Huffman):number{let code=0;for(let length=1;length<=table.length;length++){code=code*2+this.bit();const value=table.codes.get(length*65536+code);if(value!==undefined)return value;}return refusal("Huffman code has no symbol")}
 restart(sequence:number):void{this.remaining=0;if(this.source[this.at++]!==255)refusal("missing restart marker");while(this.source[this.at]===255)this.at++;if(this.source[this.at++]!==208+sequence)refusal("restart sequence");}
 finish():void{while(this.source[this.at]===255&&this.source[this.at+1]===255)this.at++;if(this.source[this.at]!==255||this.source[this.at+1]!==217)refusal("missing EOI after completed scan")}
}
const signed=(value:number,size:number)=>size===0?0:value<2**(size-1)?value-2**size+1:value;
function transform(input:Float64Array,temporary:Float64Array,output:Float64Array):void{
 for(let row=0;row<8;row++)for(let x=0;x<8;x++){let sum=0;for(let u=0;u<8;u++)sum+=(u===0?Math.SQRT1_2:1)*input[row*8+u]!*Math.cos((2*x+1)*u*Math.PI/16);temporary[row*8+x]=sum/2;}
 for(let column=0;column<8;column++)for(let y=0;y<8;y++){let sum=0;for(let u=0;u<8;u++)sum+=(u===0?Math.SQRT1_2:1)*temporary[u*8+column]!*Math.cos((2*y+1)*u*Math.PI/16);output[y*8+column]=sum/2+128;}
}
/** 📥️ Admits exact original SOF component planes with bounded native work and cumulative ownership. */
export async function decodeJpgComponents(source:Uint8Array,c=new NativeDecodeControl(512*1024*1024,()=>true)):Promise<JpgDecodedComponents>{
 await c.beginStage(0);if(source[0]!==255||source[1]!==216)refusal("missing SOI");
 const byte=(at:number)=>{const value=source[at];if(value===undefined)refusal("truncated marker");return value},u16=(at:number)=>byte(at)*256+byte(at+1);
 const quant=new Map<number,Uint16Array>(),huffman=new Map<number,Huffman>();await c.charge(512);
 let width=0,height=0,restart=0,at=2,components:Component[]=[],scan:Component[]=[];
 for(;;){await c.checkpoint();if(byte(at++)!==255)refusal("non-marker header byte");while(byte(at)===255)at++;const marker=byte(at++);if(marker===217)refusal("EOI before scan");if(marker===216||marker===1||marker>=208&&marker<=215)continue;
  const length=u16(at);if(length<2||at+length>source.length)refusal("marker extent");let position=at+2;const end=at+length;
  if(marker===192){if(components.length||length<11||byte(position)!==8)refusal("baseline precision/frame");height=u16(position+1);width=u16(position+3);const count=byte(position+5);if(!width||!height||count<1||count>4||length!==8+3*count)refusal("frame extent");await c.admitSlots(count,96);components=[];for(let i=0;i<count;i++){const p=position+6+i*3,id=byte(p),sampling=byte(p+1),h=sampling>>>4,v=sampling&15;if(!h||!v||h>4||v>4||components.some(component=>component.id===id))refusal("component sampling/identity");components.push({id,h,v,quant:byte(p+2),dc:0,ac:0,previous:0,stride:0,plane:new Float64Array(0)});}}
  else if(marker===219){while(position<end){const info=byte(position++),precision=info>>>4,id=info&15;if(precision>1||id>3||position+64*(precision+1)>end)refusal("quantization table extent");await c.charge(128+48);const values=new Uint16Array(64);for(let z=0;z<64;z++){values[z]=precision===0?byte(position++):u16(position);if(precision===1)position+=2;if(values[z]===0)refusal("zero quantizer");}quant.set(id,values);}}
  else if(marker===196){while(position<end){const info=byte(position++),kind=info>>>4,id=info&15;if(kind>1||id>3||position+16>end)refusal("Huffman selector/extent");let code=0,index=position+16,maximum=0;await c.charge(48);const codes=new Map<number,number>();for(let length=1;length<=16;length++){const count=byte(position+length-1);if(code+count>2**length||index+count>end)refusal("oversubscribed/truncated Huffman table");await c.admitSlots(count,48);for(let n=0;n<count;n++)codes.set(length*65536+code++,byte(index++));if(count)maximum=length;code*=2;}if(maximum===0)refusal("empty Huffman table");huffman.set(kind*16+id,{codes,length:maximum});position=index;}}
  else if(marker===221){if(length!==4)refusal("restart interval extent");restart=u16(position);}
  else if(marker===218){if(!components.length)refusal("scan before frame");const count=byte(position);if(count!==components.length||length!==6+2*count)refusal("non-interleaved baseline scan");await c.admitSlots(count,8);for(let i=0;i<count;i++){const id=byte(position+1+i*2),tables=byte(position+2+i*2),component=components.find(value=>value.id===id);if(!component||scan.includes(component))refusal("scan identity");component.dc=tables>>>4;component.ac=tables&15;scan.push(component);}const spectral=position+1+2*count;if(byte(spectral)!==0||byte(spectral+1)!==63||byte(spectral+2)!==0)refusal("non-baseline scan parameters");at=end;break;}
  else if(marker===204||marker>=193&&marker<=207&&![196,200,204].includes(marker))refusal("non-baseline JPEG coding");
  else if(!(marker>=224&&marker<=239||marker===254))refusal("unsupported marker");
  at=end;
 }
 const hmax=Math.max(...components.map(v=>v.h)),vmax=Math.max(...components.map(v=>v.v)),across=Math.ceil(width/(8*hmax)),down=Math.ceil(height/(8*vmax));
 for(const component of components){component.stride=extent(across,component.h,8);const count=extent(component.stride,down,component.v,8);await c.charge(extent(count,8));component.plane=new Float64Array(count);}
 await c.charge(3*64*8);const natural=new Float64Array(64),temporary=new Float64Array(64),spatial=new Float64Array(64),entropy=new Entropy(source,at);await c.beginStage(extent(across,down));let sequence=0;
 for(let mcu=0;mcu<across*down;mcu++){if(restart&&mcu&&mcu%restart===0){entropy.restart(sequence);sequence=(sequence+1)%8;for(const component of components)component.previous=0;}const x=mcu%across,y=Math.floor(mcu/across);
  for(const component of scan){const dc=huffman.get(component.dc),ac=huffman.get(16+component.ac),q=quant.get(component.quant);if(!dc||!ac||!q)refusal("missing coding table");
   for(let by=0;by<component.v;by++)for(let bx=0;bx<component.h;bx++){natural.fill(0);const size=entropy.symbol(dc);if(size>11)refusal("baseline DC category");component.previous+=signed(entropy.bits(size),size);natural[0]=component.previous*q[0]!;let z=1;
    while(z<64){const symbol=entropy.symbol(ac),run=symbol>>>4,size=symbol&15;if(size===0){if(run===15){z+=16;if(z>64)refusal("zero run extent");continue;}if(run!==0)refusal("invalid zero AC symbol");break;}if(size>10)refusal("baseline AC category");z+=run;if(z>=64)refusal("coefficient run extent");natural[zigzag[z]!]=signed(entropy.bits(size),size)*q[z]!;z++;}
    transform(natural,temporary,spatial);const ox=(x*component.h+bx)*8,oy=(y*component.v+by)*8;for(let row=0;row<8;row++)component.plane.set(spatial.subarray(row*8,row*8+8),(oy+row)*component.stride+ox);await c.checkpoint();
   }
  }await c.step();
 }
 entropy.finish();const count=extent(width,height,components.length);await c.charge(count);const samples=new Uint8Array(count);await c.beginStage(extent(width,height));
 for(let y=0;y<height;y++)for(let x=0;x<width;x++){for(let lane=0;lane<components.length;lane++){const component=components[lane]!,sx=Math.floor(x*component.h/hmax),sy=Math.floor(y*component.v/vmax);samples[(y*width+x)*components.length+lane]=Math.max(0,Math.min(255,Math.round(component.plane[sy*component.stride+sx]!)));}await c.step();}
 return{width,height,componentIds:components.map(v=>v.id),samples};
}
