/** 🗜️ Exact RFC1950/1951 recipes preserve codes, matches, alignment and final bits. */
export interface DeflateToken { readonly symbol:number; readonly extra:number; readonly distanceSymbol:number|null; readonly distanceExtra:number|null }
export interface DeflateLength { readonly symbol:number; readonly extra:number }
export interface DeflateBlock { readonly final:number; readonly kind:number; readonly alignment:number; readonly hlit:number; readonly hdist:number; readonly hclen:number; readonly codes:readonly number[]; readonly lengths:readonly DeflateLength[]; readonly tokens:readonly DeflateToken[] }
export interface DeflateStream { readonly cmf:number; readonly flg:number; readonly adler:number; readonly padding:number; readonly paddingBits:number; readonly blocks:readonly DeflateBlock[] }
export type CompressionCheckpoint=(completed:number)=>Promise<void>;
const lengthBase=[3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258];
const lengthExtra=[0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0];
const distanceBase=[1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577];
const distanceExtra=[0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
const order=[16,17,18,0,8,7,9,6,10,5,11,4,12,3,13,2,14,1,15];
export class CompressionSyntaxError extends Error {}
function invalid(reason:string):never{throw new CompressionSyntaxError("PNG compression "+reason)}
function integer(value:number,maximum:number):number{if(!Number.isInteger(value)||value<0||value>maximum)invalid("integer range");return value}
class Reader {
 at=0;
 constructor(private readonly bytes:readonly number[]){}
 get bitLength():number{return this.bytes.length*8}
 take(bits:number):number{let value=0;for(let bit=0;bit<bits;bit++){if(this.at>=this.bytes.length*8)invalid("truncated bits");value|=((this.bytes[this.at>>>3]!>>>(this.at&7))&1)<<bit;this.at++}return value}
}
class Writer {
 readonly bytes:number[]=[];at=0;
 put(value:number,bits:number):void{integer(value,2**bits-1);for(let bit=0;bit<bits;bit++){const at=this.at>>>3;if(at===this.bytes.length)this.bytes.push(0);this.bytes[at]!|=((value>>>bit)&1)<<(this.at&7);this.at++}}
}
class Huffman {
 readonly codes:number[]=[];private readonly decode:Map<number,number>[]=[];
 constructor(readonly lengths:readonly number[],allowEmpty=false){
  const counts=new Array<number>(16).fill(0),next=new Array<number>(16).fill(0);for(const length of lengths)counts[integer(length,15)]!++;counts[0]=0;
  let left=1,code=0;for(let length=1;length<=15;length++){left=left*2-counts[length]!;if(left<0)invalid("oversubscribed Huffman code");code=(code+counts[length-1]!)*2;next[length]=code;this.decode[length]=new Map()}
  if(left!==0&&!(counts[1]===1&&counts.slice(2).every(count=>count===0))&&!(allowEmpty&&counts.every(count=>count===0)))invalid("incomplete Huffman code");
  for(let symbol=0;symbol<lengths.length;symbol++){const length=lengths[symbol]!;if(length){this.codes[symbol]=next[length]!;this.decode[length]!.set(next[length]!,symbol);next[length]!++}}
 }
 read(reader:Reader):number{let code=0;for(let length=1;length<=15;length++){code=code*2+reader.take(1);const symbol=this.decode[length]?.get(code);if(symbol!==undefined)return symbol}return invalid("unknown Huffman symbol")}
 write(writer:Writer,symbol:number):void{integer(symbol,this.lengths.length-1);const length=this.lengths[symbol]!;if(length===0)invalid("missing Huffman symbol");const code=this.codes[symbol]!;for(let bit=length-1;bit>=0;bit--)writer.put((code>>>bit)&1,1)}
}
const fixed=Array.from({length:288},(_,symbol)=>symbol<144?8:symbol<256?9:symbol<280?7:8);
function expand(lengths:readonly DeflateLength[],total:number):number[]{const result:number[]=[];for(const item of lengths){let count=1,value=item.symbol;if(item.symbol===16){if(result.length===0)invalid("repeat without predecessor");count=3+integer(item.extra,3);value=result.at(-1)!}else if(item.symbol===17){count=3+integer(item.extra,7);value=0}else if(item.symbol===18){count=11+integer(item.extra,127);value=0}else{integer(item.symbol,15);if(item.extra!==0)invalid("literal code length extra")}
 if(result.length+count>total)invalid("code length repeat overflow");for(let at=0;at<count;at++)result.push(value)}if(result.length!==total)invalid("code length count");return result}
function tables(block:DeflateBlock):[Huffman,Huffman]{if(block.kind===1)return[new Huffman(fixed),new Huffman(new Array<number>(32).fill(5))];const lengths=expand(block.lengths,block.hlit+block.hdist);if(lengths[256]===0)invalid("missing end of block");return[new Huffman(lengths.slice(0,block.hlit)),new Huffman(lengths.slice(block.hlit),true)]}
function u32(bytes:readonly number[],at:number):number{return(bytes[at]!*16777216+bytes[at+1]!*65536+bytes[at+2]!*256+bytes[at+3]!)>>>0}
export function adler(bytes:readonly number[]):number{let a=1,b=0;for(const byte of bytes){a=(a+byte)%65521;b=(b+a)%65521}return(b*65536+a)>>>0}
function header(cmf:number,flg:number):number{integer(cmf,255);integer(flg,255);if((cmf&15)!==8||(cmf>>>4)>7||(cmf*256+flg)%31!==0||(flg&32)!==0)invalid("unsupported zlib header");return 2**((cmf>>>4)+8)}
function append(output:number[],token:DeflateToken,window:number):void{
 if(token.symbol<256){integer(token.symbol,255);if(token.extra!==0||token.distanceSymbol!==null||token.distanceExtra!==null)invalid("literal token fields");output.push(token.symbol);return}
 if(token.symbol===256){if(token.extra!==0||token.distanceSymbol!==null||token.distanceExtra!==null)invalid("end token fields");return}
 const index=integer(token.symbol-257,28),length=lengthBase[index]!+integer(token.extra,2**lengthExtra[index]!-1),symbol=integer(token.distanceSymbol!,29),distance=distanceBase[symbol]!+integer(token.distanceExtra!,2**distanceExtra[symbol]!-1);
 if(distance>Math.min(window,output.length))invalid("distance exceeds history");for(let at=0;at<length;at++)output.push(output[output.length-distance]!);
}
export async function parseCompression(bytes:readonly number[],checkpoint:CompressionCheckpoint):Promise<{stream:DeflateStream;raw:number[]}>{
 if(bytes.length<6)invalid("short zlib stream");const cmf=bytes[0]!,flg=bytes[1]!,window=header(cmf,flg),reader=new Reader(bytes.slice(2,-4)),blocks:DeflateBlock[]=[],raw:number[]=[];let final=0,work=0;
 do{
  final=reader.take(1);const kind=reader.take(2);if(kind===3)invalid("reserved block type");const tokens:DeflateToken[]=[],codes:number[]=[],lengths:DeflateLength[]=[];let alignment=0,hlit=0,hdist=0,hclen=0;
  if(kind===0){alignment=reader.take((8-reader.at%8)%8);const length=reader.take(16);if(reader.take(16)!==(length^65535))invalid("stored length complement");for(let at=0;at<length;at++){const token={symbol:reader.take(8),extra:0,distanceSymbol:null,distanceExtra:null};tokens.push(token);append(raw,token,window);if(++work%256===0)await checkpoint(work)}}
  else{
   if(kind===2){hlit=reader.take(5)+257;hdist=reader.take(5)+1;hclen=reader.take(4)+4;if(hlit>286)invalid("dynamic literal count");const ordered=new Array<number>(19).fill(0);for(let at=0;at<hclen;at++){const length=reader.take(3);codes.push(length);ordered[order[at]!]=length}const codeTable=new Huffman(ordered);let total=0;while(total<hlit+hdist){const symbol=codeTable.read(reader),extra=symbol===16?reader.take(2):symbol===17?reader.take(3):symbol===18?reader.take(7):0;lengths.push({symbol,extra});total+=symbol===16?3+extra:symbol===17?3+extra:symbol===18?11+extra:1;if(++work%256===0)await checkpoint(work)}}
   const block={final,kind,alignment,hlit,hdist,hclen,codes,lengths,tokens},[literal,distance]=tables(block);for(;;){const symbol=literal.read(reader);let extra=0,distanceSymbol:number|null=null,distanceValue:number|null=null;if(symbol>256){const index=integer(symbol-257,28);extra=reader.take(lengthExtra[index]!);distanceSymbol=distance.read(reader);integer(distanceSymbol,29);distanceValue=reader.take(distanceExtra[distanceSymbol]!)}const token={symbol,extra,distanceSymbol,distanceExtra:distanceValue};tokens.push(token);append(raw,token,window);if(++work%256===0)await checkpoint(work);if(symbol===256)break}
  }
  blocks.push({final,kind,alignment,hlit,hdist,hclen,codes,lengths,tokens});if(++work%256===0)await checkpoint(work);
 }while(!final);
 const paddingBits=(8-reader.at%8)%8,padding=reader.take(paddingBits);if(reader.at!==reader.bitLength)invalid("trailing compressed octets");const checksum=u32(bytes,bytes.length-4);if(checksum!==adler(raw))invalid("Adler32 mismatch");await checkpoint(work);return{stream:{cmf,flg,adler:checksum,padding,paddingBits,blocks},raw};
}
export async function encodeCompression(stream:DeflateStream,checkpoint:CompressionCheckpoint):Promise<{bytes:number[];raw:number[]}>{
 const window=header(stream.cmf,stream.flg),writer=new Writer(),raw:number[]=[];let work=0;if(stream.blocks.length===0)invalid("missing blocks");
 for(let at=0;at<stream.blocks.length;at++){
  const block=stream.blocks[at]!;if(block.final!==(at===stream.blocks.length-1?1:0))invalid("final block order");writer.put(block.final,1);writer.put(block.kind,2);
  if(block.kind===0){if(block.hlit||block.hdist||block.hclen||block.codes.length||block.lengths.length)invalid("stored block fields");writer.put(block.alignment,(8-writer.at%8)%8);integer(block.tokens.length,65535);writer.put(block.tokens.length,16);writer.put(block.tokens.length^65535,16);for(const token of block.tokens){integer(token.symbol,255);writer.put(token.symbol,8);append(raw,token,window);if(++work%256===0)await checkpoint(work)}}
  else{
   if(block.kind!==1&&block.kind!==2)invalid("block kind");if(block.alignment!==0||block.kind===1&&(block.hlit||block.hdist||block.hclen||block.codes.length||block.lengths.length))invalid("Huffman block fields");if(block.kind===2){writer.put(integer(block.hlit-257,29),5);writer.put(integer(block.hdist-1,31),5);writer.put(integer(block.hclen-4,15),4);if(block.codes.length!==block.hclen)invalid("code alphabet count");const ordered=new Array<number>(19).fill(0);for(let index=0;index<block.hclen;index++){writer.put(block.codes[index]!,3);ordered[order[index]!]=block.codes[index]!}const alphabet=new Huffman(ordered);for(const item of block.lengths){alphabet.write(writer,item.symbol);if(item.symbol>=16)writer.put(item.extra,item.symbol===16?2:item.symbol===17?3:7)}}
   const[literal,distance]=tables(block);if(block.tokens.at(-1)?.symbol!==256)invalid("missing terminal token");for(let index=0;index<block.tokens.length;index++){const token=block.tokens[index]!;if(token.symbol===256&&index!==block.tokens.length-1)invalid("early terminal token");literal.write(writer,token.symbol);if(token.symbol>256){writer.put(token.extra,lengthExtra[token.symbol-257]!);distance.write(writer,token.distanceSymbol!);writer.put(token.distanceExtra!,distanceExtra[token.distanceSymbol!]!)}append(raw,token,window);if(++work%256===0)await checkpoint(work)}
  }
 }
 if(stream.paddingBits!==(8-writer.at%8)%8)invalid("final padding width");writer.put(stream.padding,stream.paddingBits);if(stream.adler!==adler(raw))invalid("recipe checksum mismatch");const bytes=[stream.cmf,stream.flg,...writer.bytes,(stream.adler>>>24)&255,(stream.adler>>>16)&255,(stream.adler>>>8)&255,stream.adler&255];await checkpoint(work);return{bytes,raw};
}
