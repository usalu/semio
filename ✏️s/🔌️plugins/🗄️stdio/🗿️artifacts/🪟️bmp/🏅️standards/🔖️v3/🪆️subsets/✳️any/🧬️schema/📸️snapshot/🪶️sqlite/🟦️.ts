/** 🪶️ Complete editable BMP v3 meaning over the actual byte snapshot owner. */
import type { BmpSnapshot } from "../🟦️.ts";
import { bmpByteLayout, bmpMaskShift, bmpWord } from "../../../🚪️io/🟦️.ts";
import { ArtifactSqliteProjection, artifactSqliteCheckpoint, artifactSqliteDocument, artifactSqliteTables, artifactSqliteText, artifactSqliteInteger, artifactSqliteDocumentReference, artifactSqliteOrderedRowsControlled, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import { ValueError, sqliteOperation, type SqliteDatabase, type SqliteRow } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
export const BMP_SQLITE_SCHEMA = "CREATE TABLE bmp_document (\n id INTEGER PRIMARY KEY CHECK(id=1),\n schema TEXT NOT NULL,\n state TEXT NOT NULL CHECK(state IN ('valid_layout','literal_octets')),\n diagnostic TEXT NOT NULL,\n CHECK((state='valid_layout' AND diagnostic='') OR (state='literal_octets' AND diagnostic<>''))\n);\nCREATE TABLE bmp_file_header (\n id INTEGER PRIMARY KEY CHECK(id=1), document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n declared_file_size INTEGER NOT NULL CHECK(declared_file_size BETWEEN 0 AND 4294967295),\n reserved_1 INTEGER NOT NULL CHECK(reserved_1 BETWEEN 0 AND 65535),\n reserved_2 INTEGER NOT NULL CHECK(reserved_2 BETWEEN 0 AND 65535),\n pixel_offset INTEGER NOT NULL CHECK(pixel_offset BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE bmp_info_header (\n id INTEGER PRIMARY KEY CHECK(id=1), document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n header_size INTEGER NOT NULL CHECK(header_size=40),\n width INTEGER NOT NULL CHECK(width BETWEEN 0 AND 2147483647),\n signed_height INTEGER NOT NULL CHECK(signed_height BETWEEN -2147483647 AND 2147483647),\n planes INTEGER NOT NULL CHECK(planes=1),\n bits_per_pixel INTEGER NOT NULL CHECK(bits_per_pixel IN (1,4,8,16,24,32)),\n compression INTEGER NOT NULL CHECK(compression IN (0,3)),\n declared_image_size INTEGER NOT NULL CHECK(declared_image_size BETWEEN 0 AND 4294967295),\n x_pixels_per_meter INTEGER NOT NULL CHECK(x_pixels_per_meter BETWEEN -2147483648 AND 2147483647),\n y_pixels_per_meter INTEGER NOT NULL CHECK(y_pixels_per_meter BETWEEN -2147483648 AND 2147483647),\n colors_used INTEGER NOT NULL CHECK(colors_used BETWEEN 0 AND 4294967295),\n colors_important INTEGER NOT NULL CHECK(colors_important BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE bmp_channel_mask (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 0 AND 2),\n channel TEXT NOT NULL CHECK(channel IN ('red','green','blue')),\n mask INTEGER NOT NULL CHECK(mask BETWEEN 1 AND 4294967295)\n);\nCREATE TABLE bmp_palette_entry (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n ordinal INTEGER NOT NULL CHECK(ordinal>=0),\n blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 255),\n red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 255), reserved INTEGER NOT NULL CHECK(reserved BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_pixel_index (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n x INTEGER NOT NULL CHECK(x>=0), y INTEGER NOT NULL CHECK(y>=0),\n palette_index INTEGER NOT NULL CHECK(palette_index BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_pixel_sample (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n x INTEGER NOT NULL CHECK(x>=0), y INTEGER NOT NULL CHECK(y>=0),\n red INTEGER NOT NULL CHECK(red BETWEEN 0 AND 4294967295), green INTEGER NOT NULL CHECK(green BETWEEN 0 AND 4294967295),\n blue INTEGER NOT NULL CHECK(blue BETWEEN 0 AND 4294967295), unused_bits INTEGER NOT NULL CHECK(unused_bits BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE bmp_row_tail_bits (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n y INTEGER NOT NULL CHECK(y>=0), unused_bits INTEGER NOT NULL CHECK(unused_bits BETWEEN 0 AND 127)\n);\nCREATE TABLE bmp_row_padding_octet (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n y INTEGER NOT NULL CHECK(y>=0), ordinal INTEGER NOT NULL CHECK(ordinal>=0),\n value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_gap_octet (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_trailer_octet (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_literal_octet (\n id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n ordinal INTEGER NOT NULL CHECK(ordinal>=0), value INTEGER NOT NULL CHECK(value BETWEEN 0 AND 255)\n);\n";

const invalid=(message:string):never=>{throw new ValueError("invalidValue",message);};
function number(row:SqliteRow,column:number,min=0,max=4294967295):number{const value=artifactSqliteInteger(row,column);if(value<BigInt(min)||value>BigInt(max))return invalid("BMP INTEGER is outside its native field width");return Number(value);}
function cells(values:readonly number[]):bigint[]{return values.map(value=>BigInt(value));}
function byte(value:number):number{if(!Number.isInteger(value)||value<0||value>255)return invalid("BMP byte is outside u8");return value;}
function storeWord(bytes:Uint8Array,at:number,width:number,value:number):void{for(let index=0;index<width;index++){bytes[at+index]=value%256;value=Math.floor(value/256);}}
function scoped(row:SqliteRow):void{artifactSqliteDocumentReference(row,1);}
function count(rows:readonly SqliteRow[],expected:number):void{if(rows.length!==expected)invalid("BMP semantic occurrence cardinality is incomplete");}

/** 📤️ Decomposes native fields, individual samples and uninterpreted native octet occurrences. */
export async function bmpSnapshotToSqliteDatabase(snapshot:BmpSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const operation=sqliteOperation(options);options=operation;const schema=snapshot.schema;await artifactSqliteCheckpoint(options,"projectSnapshot",0,snapshot.bytes.length);
 const bytes=operation.allocateBytes(snapshot.bytes.length);
 for(let index=0;index<bytes.length;index++){bytes[index]=byte(snapshot.bytes[index]!);if((index+1)%65536===0)await artifactSqliteCheckpoint(options,"projectSnapshot",index+1,bytes.length);}
 const state=bmpByteLayout(bytes),output=await ArtifactSqliteProjection.create(BMP_SQLITE_SCHEMA,options);
 await output.insert("bmp_document",[schema,state.state,state.diagnostic],1n);
 if(state.state==="literal_octets"){for(let ordinal=0;ordinal<bytes.length;ordinal++)await output.insert("bmp_literal_octet",cells([1,ordinal,bytes[ordinal]!]));return output.finish();}
 const layout=state.layout;
 await output.insert("bmp_file_header",cells([1,layout.fileSize,layout.reserved1,layout.reserved2,layout.dataOffset]),1n);
 await output.insert("bmp_info_header",cells([1,40,layout.width,layout.signedHeight,layout.planes,layout.bitsPerPixel,layout.compression,layout.imageSize,layout.xPixelsPerMeter,layout.yPixelsPerMeter,layout.colorsUsed,layout.colorsImportant]),1n);
 if(layout.compression===3)for(let ordinal=0;ordinal<3;ordinal++)await output.insert("bmp_channel_mask",[1n,BigInt(ordinal),["red","green","blue"][ordinal]!,BigInt(layout.masks[ordinal]!)]);
 for(let ordinal=0;ordinal<layout.paletteEntries;ordinal++){const at=layout.paletteOffset+ordinal*4;await output.insert("bmp_palette_entry",cells([1,ordinal,bytes[at]!,bytes[at+1]!,bytes[at+2]!,bytes[at+3]!]));}
 const union=(layout.masks[0]|layout.masks[1]|layout.masks[2])>>>0,tail=(8-(layout.width*layout.bitsPerPixel)%8)%8;
 for(let y=0;y<layout.height;y++){
  const row=layout.dataOffset+(layout.signedHeight<0?y:layout.height-1-y)*layout.rowStride;
  for(let x=0;x<layout.width;x++){
   if(layout.bitsPerPixel<=8){const bit=x*layout.bitsPerPixel,index=(bytes[row+Math.floor(bit/8)]!>>(8-layout.bitsPerPixel-bit%8))&(2**layout.bitsPerPixel-1);await output.insert("bmp_pixel_index",cells([1,x,y,index]));}
   else{const word=bmpWord(bytes,row+x*layout.bitsPerPixel/8,layout.bitsPerPixel/8),samples=layout.masks.map(mask=>((word&mask)>>>0)/2**bmpMaskShift(mask));await output.insert("bmp_pixel_sample",cells([1,x,y,samples[0]!,samples[1]!,samples[2]!,(word&~union)>>>0]));}
  }
  if(tail)await output.insert("bmp_row_tail_bits",cells([1,y,bytes[row+layout.rowPayload-1]!&(2**tail-1)]));
  for(let ordinal=0;ordinal<layout.rowStride-layout.rowPayload;ordinal++)await output.insert("bmp_row_padding_octet",cells([1,y,ordinal,bytes[row+layout.rowPayload+ordinal]!]));
 }
 for(let ordinal=0;ordinal<layout.dataOffset-layout.metadataEnd;ordinal++)await output.insert("bmp_gap_octet",cells([1,ordinal,bytes[layout.metadataEnd+ordinal]!]));
 for(let ordinal=0;ordinal<bytes.length-layout.pixelEnd;ordinal++)await output.insert("bmp_trailer_octet",cells([1,ordinal,bytes[layout.pixelEnd+ordinal]!]));
 return output.finish();
}

/** 📥️ Validates every occurrence and variant relation before publishing exact reconstructed bytes. */
export async function bmpSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<BmpSnapshot>{
 const operation=sqliteOperation(options);options=operation;
 const tables=await artifactSqliteTables(database,BMP_SQLITE_SCHEMA,options);
 const [documents,files,infos,maskRows,paletteRows,indexRows,sampleRows,tailRows,paddingRows,gapRows,trailerRows,literalRows]=tables as readonly (readonly SqliteRow[])[];
 const document=artifactSqliteDocument(documents!),schema=artifactSqliteText(document,1),state=artifactSqliteText(document,2),diagnostic=artifactSqliteText(document,3);
 const ordered=async(rows:readonly SqliteRow[])=>{const result=await artifactSqliteOrderedRowsControlled(rows,2,options);for(const row of result)scoped(row);return result;};
 if(state==="literal_octets"){
  if(diagnostic==="")invalid("BMP literal state requires its actual layout diagnostic");
  for(const rows of tables.slice(1,-1))count(rows,0);
  const rows=await ordered(literalRows!),bytes=operation.allocateBytes(rows.length);
  for(let ordinal=0;ordinal<rows.length;ordinal++){bytes[ordinal]=number(rows[ordinal]!,3,0,255);if((ordinal+1)%65536===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordinal+1,rows.length);}
  const classified=bmpByteLayout(bytes);if(classified.state!=="literal_octets"||classified.diagnostic!==diagnostic)invalid("BMP literal state disagrees with its actual layout diagnostic");
  return {schema,bytes:Array.from(bytes)};
 }
 if(state!=="valid_layout"||diagnostic!=="")invalid("BMP layout state is invalid");
 count(literalRows!,0);count(files!,1);count(infos!,1);
 const file=artifactSqliteDocument(files!),info=artifactSqliteDocument(infos!);scoped(file);scoped(info);
 const fileSize=number(file,2),reserved1=number(file,3,0,65535),reserved2=number(file,4,0,65535),dataOffset=number(file,5),headerSize=number(info,2),width=number(info,3,0,2147483647),signedHeight=number(info,4,-2147483647,2147483647),height=Math.abs(signedHeight),planes=number(info,5,0,65535),bpp=number(info,6,0,65535),compression=number(info,7),imageSize=number(info,8);
 if(headerSize!==40||planes!==1||(width===0)!==(height===0)||!(compression===0&&[1,4,8,16,24,32].includes(bpp)||compression===3&&[16,32].includes(bpp)))invalid("BMP header is outside the actual native grammar");
 const masks:number[]=bpp===16?[0x7c00,0x03e0,0x001f]:bpp>=24?[0xff0000,0xff00,0xff]:[0,0,0];
 if(compression===3){const rows=await ordered(maskRows!);count(rows,3);for(let ordinal=0;ordinal<3;ordinal++){if(artifactSqliteText(rows[ordinal]!,3)!==["red","green","blue"][ordinal])invalid("BMP channel mask ordinal has another channel");masks[ordinal]=number(rows[ordinal]!,4,1,2**bpp-1);const shifted=BigInt(masks[ordinal]!)/2n**BigInt(bmpMaskShift(masks[ordinal]!));if((shifted&(shifted+1n))!==0n)invalid("BMP channel mask is not contiguous");}if((masks[0]!&masks[1]!)!==0||(masks[0]!&masks[2]!)!==0||(masks[1]!&masks[2]!)!==0)invalid("BMP channel masks overlap");}else count(maskRows!,0);
 const colorsUsed=number(info,11),paletteEntries=bpp<=8?(colorsUsed===0?2**bpp:colorsUsed):0,metadataEnd=54+(compression===3?12:0)+paletteEntries*4;
 if(bpp<=8&&paletteEntries>2**bpp||dataOffset<metadataEnd)invalid("BMP palette and pixel offset overlap or exceed sample capacity");
 const rowBits=width*bpp,rowStride=Math.floor((rowBits+31)/32)*4,rowPayload=Math.floor((rowBits+7)/8),pixelBytesWord=BigInt(rowStride)*BigInt(height),pixelEndWord=BigInt(dataOffset)+pixelBytesWord,totalWord=pixelEndWord+BigInt(trailerRows!.length);
 if(totalWord>BigInt(Number.MAX_SAFE_INTEGER))throw new ValueError("ownershipLimit","BMP reconstructed byte length exceeds address space");
 const pixelBytes=Number(pixelBytesWord),pixelEnd=Number(pixelEndWord),total=Number(totalWord);
 if(fileSize!==0&&(fileSize<pixelEnd||fileSize>total)||imageSize!==0&&imageSize<pixelBytes)invalid("BMP declared native sizes do not contain the checked layout");
 const pixels=width*height;if(!Number.isSafeInteger(pixels))throw new ValueError("ownershipLimit","BMP pixel frontier exceeds address space");
 count(paletteRows!,paletteEntries);count(gapRows!,dataOffset-metadataEnd);
 count(bpp<=8?sampleRows!:indexRows!,0);count(bpp<=8?indexRows!:sampleRows!,pixels);
 const tail=(8-rowBits%8)%8;count(tailRows!,tail?height:0);count(paddingRows!,(rowStride-rowPayload)*height);
 const bytes=operation.allocateBytes(total);bytes[0]=66;bytes[1]=77;storeWord(bytes,2,4,fileSize);storeWord(bytes,6,2,reserved1);storeWord(bytes,8,2,reserved2);storeWord(bytes,10,4,dataOffset);
 for(const [at,field,width_]of [[14,headerSize,4],[18,width,4],[22,signedHeight<0?signedHeight+4294967296:signedHeight,4],[26,planes,2],[28,bpp,2],[30,compression,4],[34,imageSize,4],[38,number(info,9,-2147483648,2147483647),4],[42,number(info,10,-2147483648,2147483647),4],[46,colorsUsed,4],[50,number(info,12),4]])storeWord(bytes,at!,width_!,field!<0?field!+4294967296:field!);
 if(compression===3)for(let ordinal=0;ordinal<3;ordinal++)storeWord(bytes,54+ordinal*4,4,masks[ordinal]!);
 const palette=await ordered(paletteRows!);for(let ordinal=0;ordinal<palette.length;ordinal++){const row=palette[ordinal]!;for(let channel=0;channel<4;channel++)bytes[54+(compression===3?12:0)+ordinal*4+channel]=number(row,3+channel,0,255);}
 const physical=(y:number)=>dataOffset+(signedHeight<0?y:height-1-y)*rowStride;
 const seen=operation.allocateBytes(pixels),rows=bpp<=8?indexRows!:sampleRows!,union=(masks[0]!|masks[1]!|masks[2]!)>>>0;
 for(let ordinal=0;ordinal<rows.length;ordinal++){
  const row=rows[ordinal]!;scoped(row);const x=number(row,2,0,width-1),y=number(row,3,0,height-1),index=y*width+x;if(seen[index])invalid("BMP pixel coordinates are duplicated");seen[index]=1;
  if(bpp<=8){const sample=number(row,4,0,2**bpp-1),bit=x*bpp,at=physical(y)+Math.floor(bit/8);bytes[at]!|=sample*2**(8-bpp-bit%8);}
  else{let word=number(row,7,0,2**bpp-1);if((word&union)!==0)invalid("BMP unused sample bits overlap a channel");for(let channel=0;channel<3;channel++){const mask=masks[channel]!,shift=bmpMaskShift(mask),value=number(row,4+channel,0,mask/2**shift);word+=value*2**shift;}storeWord(bytes,physical(y)+x*bpp/8,bpp/8,word);}
  if((ordinal+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",ordinal+1,rows.length);
 }
 const tails=operation.allocateBytes(tail?height:0);for(const [index,row]of tailRows!.entries()){scoped(row);const y=number(row,2,0,height-1);if(tails[y])invalid("BMP row tail occurs twice");tails[y]=1;bytes[physical(y)+rowPayload-1]!|=number(row,3,0,2**tail-1);if((index+1)%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",index+1,tailRows!.length);}
 const paddingCount=(rowStride-rowPayload)*height,paddingSeen=operation.allocateBytes(paddingCount);
 for(const row of paddingRows!){scoped(row);const y=number(row,2,0,height-1),ordinal=number(row,3,0,rowStride-rowPayload-1),index=y*(rowStride-rowPayload)+ordinal;if(paddingSeen[index])invalid("BMP row padding occurs twice");paddingSeen[index]=1;bytes[physical(y)+rowPayload+ordinal]=number(row,4,0,255);}
 const gap=await ordered(gapRows!),trailer=await ordered(trailerRows!);for(let ordinal=0;ordinal<gap.length;ordinal++)bytes[metadataEnd+ordinal]=number(gap[ordinal]!,3,0,255);for(let ordinal=0;ordinal<trailer.length;ordinal++)bytes[pixelEnd+ordinal]=number(trailer[ordinal]!,3,0,255);
 if(bmpByteLayout(bytes).state!=="valid_layout")invalid("BMP reconstructed layout violates the actual native grammar");
 await artifactSqliteCheckpoint(options,"reconstructSnapshot",rows.length,rows.length);return {schema,bytes:Array.from(bytes)};
}
