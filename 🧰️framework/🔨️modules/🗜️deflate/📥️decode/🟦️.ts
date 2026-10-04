/** 🗜️ First-party DEFLATE cursor with one-byte input admission and output. */
export type InflateStep={value?:number;done:boolean;consumed:boolean};
type Table={counts:Uint16Array;symbols:Uint16Array;complete:boolean;single:boolean;empty:boolean};
const order=[16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15];
const lengthBase=[3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258];
const lengthExtra=[0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0];
const distanceBase=[1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577];
const distanceExtra=[0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
function table(lengths:Uint8Array):Table{
 const counts=new Uint16Array(16),symbols=new Uint16Array(lengths.length),offsets=new Uint16Array(16);
 for(const n of lengths){if(n>15)throw Error("Invalid Huffman length");counts[n]!++;}
 counts[0]=0;let left=1;
 for(let n=1;n<16;n++){left=left*2-counts[n]!;if(left<0)throw Error("Oversubscribed Huffman tree");offsets[n]=offsets[n-1]!+counts[n-1]!;}
 for(let i=0;i<lengths.length;i++){const n=lengths[i]!;if(n)symbols[offsets[n]!] = i,offsets[n]!++;}
 return {counts,symbols,complete:left===0,single:counts[1]===1&&counts.subarray(2).every(n=>n===0),empty:counts.every(n=>n===0)};
}
const fixedLengths=Uint8Array.from({length:288},(_,i)=>i<144?8:i<256?9:i<280?7:8);
const fixedLiteral=table(fixedLengths),fixedDistance=table(new Uint8Array(32).fill(5));
export class InflateCursor{
 private buffer=0;private bits=0;private pending:number|undefined;private complete=false;private consumed=false;
 private phase="header";private final=false;private remaining=0;private hlit=0;private hdist=0;private hclen=0;private at=0;
 private codeLengths=new Uint8Array(19);private lengths=new Uint8Array(0);private codeTable:Table|undefined;
 private literal:Table=fixedLiteral;private distance:Table=fixedDistance;private repeat=0;private repeatValue=0;private extra=0;
 private matchLength=0;private matchDistance=0;private history:Uint8Array;private output=0;
 constructor(window:number){if(!Number.isInteger(window)||window<256||window>32768)throw Error("Invalid DEFLATE history window");this.history=new Uint8Array(window);}
 get unusedWholeBytes():number{return Math.floor(this.bits/8);}
 private ensure(n:number):boolean{
  if(this.bits<n&&this.pending!==undefined){this.buffer|=this.pending<<this.bits;this.bits+=8;this.pending=undefined;this.consumed=true;}
  return this.bits>=n;
 }
 private take(n:number):number{const value=this.buffer&((1<<n)-1);this.buffer>>>=n;this.bits-=n;return value;}
 private symbol(t:Table):number|undefined{
  if(!this.ensure(15)&&!this.complete)return undefined;
  let code=0,first=0,index=0;
  for(let len=1;len<=15;len++){
   if(len>this.bits)throw Error("Truncated Huffman symbol");
   code|=(this.buffer>>>(len-1))&1;const count=t.counts[len]!;
   if(code>=first&&code-first<count){const value=t.symbols[index+code-first]!;this.take(len);return value;}
   index+=count;first=(first+count)*2;code*=2;
  }
  throw Error("Invalid Huffman symbol");
 }
 private write(value:number):InflateStep{this.history[this.output%this.history.length]=value;this.output++;return {value,done:false,consumed:this.consumed};}
 advance(pending:number|undefined,complete:boolean):InflateStep{
  this.pending=pending;this.complete=complete;this.consumed=false;
  const need=():InflateStep=>{if(complete&&this.pending===undefined)throw Error("Truncated DEFLATE stream");return {done:false,consumed:this.consumed};};
  for(;;){
   switch(this.phase){
    case "header":{
     if(!this.ensure(3))return need();const value=this.take(3);this.final=!!(value&1);const kind=value>>>1;
     if(kind===0){this.take(this.bits%8);this.phase="storedLength";}
     else if(kind===1){this.literal=fixedLiteral;this.distance=fixedDistance;this.phase="literal";}
     else if(kind===2)this.phase="dynamicCounts";else throw Error("Reserved DEFLATE block type");
     break;
    }
    case "storedLength":if(!this.ensure(16))return need();this.remaining=this.take(16);this.phase="storedCheck";break;
    case "storedCheck":if(!this.ensure(16))return need();if(this.take(16)!==(this.remaining^0xffff))throw Error("Invalid stored block length");this.phase="stored";break;
    case "stored":if(this.remaining===0){this.phase=this.final?"done":"header";break;}if(!this.ensure(8))return need();this.remaining--;return this.write(this.take(8));
    case "dynamicCounts":{
     if(!this.ensure(14))return need();const value=this.take(14);this.hlit=(value&31)+257;this.hdist=((value>>>5)&31)+1;this.hclen=(value>>>10)+4;
     if(this.hlit>286)throw Error("Reserved dynamic literal count");
     this.codeLengths.fill(0);this.lengths=new Uint8Array(this.hlit+this.hdist);this.at=0;this.phase="codeLengths";break;
    }
    case "codeLengths":{
     if(this.at===this.hclen){this.codeTable=table(this.codeLengths);if(!this.codeTable.complete)throw Error("Incomplete code length tree");this.at=0;this.phase="lengths";break;}
     if(!this.ensure(3))return need();this.codeLengths[order[this.at++]!]=this.take(3);break;
    }
    case "lengths":{
     if(this.at===this.lengths.length){if(!this.lengths[256])throw Error("Missing end-of-block code");this.literal=table(this.lengths.subarray(0,this.hlit));this.distance=table(this.lengths.subarray(this.hlit));if(!this.literal.complete&&!this.literal.single||!this.distance.complete&&!this.distance.single&&!this.distance.empty)throw Error("Incomplete literal or distance tree");this.phase="literal";break;}
     const s=this.symbol(this.codeTable!);if(s===undefined)return need();
     if(s<16){this.lengths[this.at++]=s;break;}
     if(s===16){if(this.at===0)throw Error("Repeat without prior code");this.repeatValue=this.lengths[this.at-1]!;this.repeat=3;this.extra=2;}
     else if(s===17){this.repeatValue=0;this.repeat=3;this.extra=3;}
     else if(s===18){this.repeatValue=0;this.repeat=11;this.extra=7;}
     else throw Error("Invalid code length repeat");this.phase="repeat";break;
    }
    case "repeat":{if(!this.ensure(this.extra))return need();const count=this.repeat+this.take(this.extra);if(this.at+count>this.lengths.length)throw Error("Code length repeat overflow");this.lengths.fill(this.repeatValue,this.at,this.at+count);this.at+=count;this.phase="lengths";break;}
    case "literal":{
     const s=this.symbol(this.literal);if(s===undefined)return need();
     if(s<256)return this.write(s);
     if(s===256){this.phase=this.final?"done":"header";break;}
     if(s>285)throw Error("Reserved literal code");const i=s-257;this.matchLength=lengthBase[i]!;this.extra=lengthExtra[i]!;this.phase="lengthExtra";break;
    }
    case "lengthExtra":if(!this.ensure(this.extra))return need();this.matchLength+=this.take(this.extra);this.phase="distance";break;
    case "distance":{const s=this.symbol(this.distance);if(s===undefined)return need();if(s>29)throw Error("Reserved distance code");this.matchDistance=distanceBase[s]!;this.extra=distanceExtra[s]!;this.phase="distanceExtra";break;}
    case "distanceExtra":if(!this.ensure(this.extra))return need();this.matchDistance+=this.take(this.extra);if(this.matchDistance>Math.min(this.output,this.history.length))throw Error("Distance exceeds retained history");this.phase="copy";break;
    case "copy":{const value=this.history[(this.output-this.matchDistance)%this.history.length]!;if(--this.matchLength===0)this.phase="literal";return this.write(value);}
    case "done":return {done:true,consumed:this.consumed};
    default:throw Error("Invalid DEFLATE phase");
   }
  }
 }
}
