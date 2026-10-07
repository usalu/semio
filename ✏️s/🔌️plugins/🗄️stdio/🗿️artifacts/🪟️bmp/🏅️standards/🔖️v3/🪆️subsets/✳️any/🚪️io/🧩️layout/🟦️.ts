import {bmpMaskShift} from "../../🧬️schema/📸️snapshot/🟦️.ts";
/** 🪟️ Borrowed BMP v3 byte grammar with explicit layout refusal. */
export interface BmpByteLayout {
 fileSize:number;reserved1:number;reserved2:number;dataOffset:number;width:number;signedHeight:number;height:number;planes:number;bitsPerPixel:number;compression:number;imageSize:number;xPixelsPerMeter:number;yPixelsPerMeter:number;colorsUsed:number;colorsImportant:number;
 masks:readonly [number,number,number];paletteOffset:number;paletteEntries:number;metadataEnd:number;rowStride:number;rowPayload:number;pixelEnd:number;
}
export type BmpByteLayoutResult={state:"valid_layout";layout:BmpByteLayout;diagnostic:""}|{state:"literal_octets";diagnostic:string};
/** 🔢️ Reads an already bounded little-endian native word without copying source bytes. */
export function bmpWord(bytes:ArrayLike<number>,at:number,width:number):number{let result=0;for(let i=0;i<width;i++)result+=bytes[at+i]!*2**(8*i);return result;}
/** 🛂️ Classifies complete byte vectors according to the current uncompressed forty-byte DIB grammar. */
export function bmpByteLayout(bytes:ArrayLike<number>):BmpByteLayoutResult{
 const refused=(diagnostic:string):BmpByteLayoutResult=>({state:"literal_octets",diagnostic});
 const range=(at:number,length:number,name:string)=>at+length<=bytes.length?null:`bmp: truncated ${name}`;
 let failure=range(0,2,"signature");if(failure)return refused(failure);
 if(bytes[0]!==66||bytes[1]!==77)return refused("bmp: bad signature");
 const word=(at:number,width:number,name:string):number|string=>range(at,width,name)??bmpWord(bytes,at,width);
 const fileSize=word(2,4,"file size");if(typeof fileSize==="string")return refused(fileSize);
 const reserved1=word(6,2,"reserved 1");if(typeof reserved1==="string")return refused(reserved1);
 const reserved2=word(8,2,"reserved 2");if(typeof reserved2==="string")return refused(reserved2);
 const dataOffset=word(10,4,"pixel offset");if(typeof dataOffset==="string")return refused(dataOffset);
 const headerSize=word(14,4,"DIB header size");if(typeof headerSize==="string")return refused(headerSize);
 if(headerSize!==40)return refused(`bmp: v3 requires a 40-byte BITMAPINFOHEADER; DIB profile ${headerSize} must use its own standard`);
 failure=range(14,40,"BITMAPINFOHEADER");if(failure)return refused(failure);
 const signed=(at:number)=>{const value=bmpWord(bytes,at,4);return value>=2147483648?value-4294967296:value;};
 const width=signed(18),signedHeight=signed(22);
 if(width<0||signedHeight===-2147483648)return refused("bmp: width must be nonnegative and height must be representable");
 if((width===0)!==(signedHeight===0))return refused("bmp: empty BMP dimensions must both be zero");
 const height=Math.abs(signedHeight),planes=bmpWord(bytes,26,2),bitsPerPixel=bmpWord(bytes,28,2),compression=bmpWord(bytes,30,4);
 if(planes!==1)return refused(`bmp: planes must be 1, got ${planes}`);
 if(compression===0&&!([1,4,8,16,24,32].includes(bitsPerPixel)))return refused(`bmp: unsupported BI_RGB bit depth ${bitsPerPixel}`);
 if(compression===3&&bitsPerPixel!==16&&bitsPerPixel!==32)return refused(`bmp: BI_BITFIELDS requires 16 or 32 bits per pixel, got ${bitsPerPixel}`);
 if(compression!==0&&compression!==3)return refused(`bmp: compression profile ${compression} is outside the uncompressed v3 standard`);
 const imageSize=bmpWord(bytes,34,4),rowBits=width*bitsPerPixel,rowStride=Math.floor((rowBits+31)/32)*4,rowPayload=Math.floor((rowBits+7)/8),pixelBytesWord=BigInt(rowStride)*BigInt(height),pixelEndWord=BigInt(dataOffset)+pixelBytesWord;
 if(pixelEndWord>BigInt(bytes.length))return refused(`bmp: pixel storage ends at ${pixelEndWord}, beyond ${bytes.length} source bytes`);
 const pixelBytes=Number(pixelBytesWord),pixelEnd=Number(pixelEndWord);
 if(fileSize!==0&&(fileSize<pixelEnd||fileSize>bytes.length))return refused(`bmp: declared file size ${fileSize} does not contain the checked image and fit the source`);
 if(imageSize!==0&&imageSize<pixelBytes)return refused(`bmp: declared image size ${imageSize} is smaller than ${pixelBytes} checked row bytes`);
 let metadataEnd=54;
 let masks:readonly[number,number,number]=bitsPerPixel===16?[0x7c00,0x03e0,0x001f]:bitsPerPixel>=24?[0xff0000,0xff00,0xff]:[0,0,0];
 if(compression===3){
  const values:number[]=[];for(const [ordinal,name]of ["red mask","green mask","blue mask"].entries()){const value=word(metadataEnd+ordinal*4,4,name);if(typeof value==="string")return refused(value);values.push(value);}
  masks=[values[0]!,values[1]!,values[2]!];const limit=2**bitsPerPixel-1;
  if(masks.some(mask=>mask===0||mask>limit))return refused("bmp: BI_BITFIELDS RGB masks must be nonzero and fit the sample width");
  for(let ordinal=0;ordinal<3;ordinal++){const shifted=BigInt(masks[ordinal]!)/2n**BigInt(bmpMaskShift(masks[ordinal]!));if((shifted&(shifted+1n))!==0n)return refused(`bmp: channel mask ${ordinal} is not contiguous`);}
  if((masks[0]&masks[1])!==0||(masks[0]&masks[2])!==0||(masks[1]&masks[2])!==0)return refused("bmp: BI_BITFIELDS RGB masks overlap");
  metadataEnd+=12;
 }
 const paletteOffset=metadataEnd,colorsUsed=bmpWord(bytes,46,4),paletteEntries=bitsPerPixel<=8?(colorsUsed===0?2**bitsPerPixel:colorsUsed):0;
 if(bitsPerPixel<=8&&paletteEntries>2**bitsPerPixel)return refused(`bmp: palette has ${paletteEntries} declared entries, beyond ${bitsPerPixel}-bit capacity ${2**bitsPerPixel}`);
 metadataEnd+=paletteEntries*4;
 if(dataOffset<metadataEnd)return refused(`bmp: pixel offset ${dataOffset} overlaps metadata ending at ${metadataEnd}`);
 failure=range(paletteOffset,paletteEntries*4,"palette");if(failure)return refused(failure);
 return {state:"valid_layout",diagnostic:"",layout:{fileSize,reserved1,reserved2,dataOffset,width,signedHeight,height,planes,bitsPerPixel,compression,imageSize,xPixelsPerMeter:signed(38),yPixelsPerMeter:signed(42),colorsUsed,colorsImportant:bmpWord(bytes,50,4),masks,paletteOffset,paletteEntries,metadataEnd,rowStride,rowPayload,pixelEnd}};
}
