/** 📸️ Handcrafted JPEG raster and coding-table relationships. */
import type {JpgSnapshot,JfifThumbnail,JpgFrameHeader,JpgFrameComponent,JpgQuantTable,JpgHuffmanTable,JpgSegment} from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteCheckpoint,artifactSqliteTables,artifactSqliteDocument,artifactSqliteInteger,artifactSqliteText,artifactSqliteOrderedRows,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
/** 🏛️ Owns the explicit JPEG table definitions shared by both implementations. */
export const JPG_SQLITE_SCHEMA=String.raw`CREATE TABLE jpg_document (
 id INTEGER PRIMARY KEY CHECK (id = 1), schema TEXT NOT NULL,
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),
 re_encode_quality INTEGER CHECK (re_encode_quality BETWEEN 0 AND 255),
 jfif_major INTEGER NOT NULL CHECK (jfif_major BETWEEN 0 AND 255), jfif_minor INTEGER NOT NULL CHECK (jfif_minor BETWEEN 0 AND 255),
 density_units TEXT NOT NULL CHECK (density_units IN ('aspect','pixelsPerInch','pixelsPerCm')),
 x_density INTEGER NOT NULL CHECK (x_density BETWEEN 0 AND 65535), y_density INTEGER NOT NULL CHECK (y_density BETWEEN 0 AND 65535),
 sof_marker INTEGER NOT NULL CHECK (sof_marker BETWEEN 0 AND 255), arithmetic INTEGER NOT NULL CHECK (arithmetic IN (0,1)),
 restart_interval INTEGER CHECK (restart_interval BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_rgba_pixel (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id),
 x INTEGER NOT NULL CHECK (x >= 0), y INTEGER NOT NULL CHECK (y >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),
 blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255), alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)
);
CREATE TABLE jpg_thumbnail (
 id INTEGER PRIMARY KEY REFERENCES jpg_document(id) CHECK (id = 1),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 255), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 255)
);
CREATE TABLE jpg_thumbnail_rgb_pixel (
 id INTEGER PRIMARY KEY, thumbnail_id INTEGER NOT NULL REFERENCES jpg_thumbnail(id),
 x INTEGER NOT NULL CHECK (x >= 0), y INTEGER NOT NULL CHECK (y >= 0),
 red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255), green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255), blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255)
);
CREATE TABLE jpg_frame (
 id INTEGER PRIMARY KEY REFERENCES jpg_document(id) CHECK (id = 1),
 precision INTEGER NOT NULL CHECK (precision BETWEEN 0 AND 255),
 width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 65535), height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_quantizer (
 id INTEGER PRIMARY KEY CHECK (id BETWEEN 0 AND 255), document_id INTEGER NOT NULL REFERENCES jpg_document(id)
);
CREATE TABLE jpg_frame_component (
 id INTEGER PRIMARY KEY, frame_id INTEGER NOT NULL REFERENCES jpg_frame(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 component_id INTEGER NOT NULL CHECK (component_id BETWEEN 0 AND 255),
 h_sampling INTEGER NOT NULL CHECK (h_sampling BETWEEN 0 AND 255), v_sampling INTEGER NOT NULL CHECK (v_sampling BETWEEN 0 AND 255),
 quantizer_id INTEGER NOT NULL REFERENCES jpg_quantizer(id)
);
CREATE TABLE jpg_quantization_table (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 quantizer_id INTEGER NOT NULL REFERENCES jpg_quantizer(id), precision INTEGER NOT NULL CHECK (precision BETWEEN 0 AND 255)
);
CREATE TABLE jpg_quantization_coefficient (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_quantization_table(id),
 zigzag_ordinal INTEGER NOT NULL CHECK (zigzag_ordinal BETWEEN 0 AND 63), coefficient INTEGER NOT NULL CHECK (coefficient BETWEEN 0 AND 65535)
);
CREATE TABLE jpg_huffman_table (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 table_id INTEGER NOT NULL CHECK (table_id BETWEEN 0 AND 255), table_class TEXT NOT NULL CHECK (table_class IN ('dc','ac'))
);
CREATE TABLE jpg_huffman_code_length (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_huffman_table(id),
 length_ordinal INTEGER NOT NULL CHECK (length_ordinal BETWEEN 0 AND 15), symbol_count INTEGER NOT NULL CHECK (symbol_count BETWEEN 0 AND 255)
);
CREATE TABLE jpg_huffman_symbol (
 id INTEGER PRIMARY KEY, table_id INTEGER NOT NULL REFERENCES jpg_huffman_table(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), symbol INTEGER NOT NULL CHECK (symbol BETWEEN 0 AND 255)
);
CREATE TABLE jpg_segment (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES jpg_document(id), ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
 marker INTEGER NOT NULL CHECK (marker BETWEEN 0 AND 255)
);
CREATE TABLE jpg_segment_octet (
 id INTEGER PRIMARY KEY, segment_id INTEGER NOT NULL REFERENCES jpg_segment(id),
 ordinal INTEGER NOT NULL CHECK (ordinal >= 0), octet INTEGER NOT NULL CHECK (octet BETWEEN 0 AND 255)
);
`;

function integer(value:number,maximum=4294967295):bigint{if(!Number.isSafeInteger(value)||value<0||value>maximum)throw new Error("JPEG integer exceeds its owned field width");return BigInt(value);}
function field(row:SqliteRow,column:number,maximum=4294967295):number{const value=artifactSqliteInteger(row,column);if(value<0n||value>BigInt(maximum))throw new Error("JPEG integer exceeds its owned field width");return Number(value);}
async function checkpoint(options:ArtifactSqliteOptions,position:number,total:number):Promise<void>{if(position%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",position,total);}
async function entities(rows:readonly SqliteRow[],options:ArtifactSqliteOptions):Promise<Set<bigint>>{const keys=new Set<bigint>();for(let position=0;position<rows.length;position++){await checkpoint(options,position,rows.length);const row=rows[position]!;if(row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||keys.has(row.rowid))throw new Error("JPEG entities require unique positive aliased identities");keys.add(row.rowid);}return keys;}
async function groups(rows:readonly SqliteRow[],parents:ReadonlySet<bigint>,options:ArtifactSqliteOptions):Promise<Map<bigint,SqliteRow[]>>{await entities(rows,options);const groups=new Map<bigint,SqliteRow[]>();for(let position=0;position<rows.length;position++){await checkpoint(options,position,rows.length);const row=rows[position]!,owner=artifactSqliteInteger(row,1);if(!parents.has(owner))throw new Error("JPEG child names an unknown owner");const children=groups.get(owner);if(children)children.push(row);else groups.set(owner,[row]);}for(const[owner,rows]of groups){await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,rows.length);groups.set(owner,artifactSqliteOrderedRows(rows,2));}return groups;}
async function ordered(rows:readonly SqliteRow[],options:ArtifactSqliteOptions):Promise<SqliteRow[]>{return(await groups(rows,new Set([1n]),options)).get(1n)??[];}
async function optional(rows:readonly SqliteRow[],options:ArtifactSqliteOptions):Promise<SqliteRow|undefined>{await entities(rows,options);if(rows.length>1||(rows.length===1&&rows[0]!.rowid!==1n))throw new Error("JPEG optional identity must be one");return rows[0];}
function gridLength(width:number,height:number,channels:number,length:number):void{if(integer(width)*integer(height)*BigInt(channels)!==BigInt(length))throw new Error("JPEG pixels must fill their declared grid");}
async function readGrid(rows:readonly SqliteRow[],width:number,height:number,channels:number,present:boolean,options:ArtifactSqliteOptions):Promise<number[]>{await entities(rows,options);if(!present&&rows.length!==0)throw new Error("JPEG absent thumbnail cannot own pixels");gridLength(width,height,channels,rows.length*channels);const pixels=new Array<number>(rows.length*channels).fill(0),seen=new Set<number>();for(let position=0;position<rows.length;position++){await checkpoint(options,position,rows.length);const row=rows[position]!,x=field(row,2),y=field(row,3);if(artifactSqliteInteger(row,1)!==1n||x>=width||y>=height)throw new Error("JPEG pixel lies outside its owner or grid");const ordinal=y*width+x;if(seen.has(ordinal))throw new Error("JPEG pixels require unique coordinates");seen.add(ordinal);for(let channel=0;channel<channels;channel++)pixels[ordinal*channels+channel]=field(row,4+channel,255);}return pixels;}
async function octets(rows:readonly SqliteRow[],options:ArtifactSqliteOptions):Promise<number[]>{const bytes:number[]=[];for(let position=0;position<rows.length;position++){await checkpoint(options,position,rows.length);bytes.push(field(rows[position]!,3,255));}return bytes;}

/** 📤️ Exposes complete owned JPEG fields without encoding a JPEG stream. */
export async function jpgSnapshotToSqliteDatabase(snapshot:JpgSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(JPG_SQLITE_SCHEMA,options);if(!["aspect","pixelsPerInch","pixelsPerCm"].includes(snapshot.jfifDensityUnits)||typeof snapshot.arithmetic!=="boolean"||snapshot.jfifVersion.length!==2)throw new Error("JPEG header has an unknown typed value");
 await out.insert("jpg_document",[snapshot.schema,integer(snapshot.width),integer(snapshot.height),snapshot.reEncodeQuality===undefined?null:integer(snapshot.reEncodeQuality,255),integer(snapshot.jfifVersion[0],255),integer(snapshot.jfifVersion[1],255),snapshot.jfifDensityUnits,integer(snapshot.jfifXDensity,65535),integer(snapshot.jfifYDensity,65535),integer(snapshot.sofMarker,255),snapshot.arithmetic?1n:0n,snapshot.restartInterval===undefined?null:integer(snapshot.restartInterval,65535)]);
 gridLength(snapshot.width,snapshot.height,4,snapshot.pixels.length);for(let ordinal=0;ordinal<snapshot.pixels.length/4;ordinal++)await out.insert("jpg_rgba_pixel",[1n,BigInt(ordinal%snapshot.width),BigInt(Math.floor(ordinal/snapshot.width)),integer(snapshot.pixels[ordinal*4]!,255),integer(snapshot.pixels[ordinal*4+1]!,255),integer(snapshot.pixels[ordinal*4+2]!,255),integer(snapshot.pixels[ordinal*4+3]!,255)]);
 if(snapshot.jfifThumbnail!==undefined){const thumbnail=snapshot.jfifThumbnail;await out.insert("jpg_thumbnail",[integer(thumbnail.width,255),integer(thumbnail.height,255)]);gridLength(thumbnail.width,thumbnail.height,3,thumbnail.rgbData.length);for(let ordinal=0;ordinal<thumbnail.rgbData.length/3;ordinal++)await out.insert("jpg_thumbnail_rgb_pixel",[1n,BigInt(ordinal%thumbnail.width),BigInt(Math.floor(ordinal/thumbnail.width)),integer(thumbnail.rgbData[ordinal*3]!,255),integer(thumbnail.rgbData[ordinal*3+1]!,255),integer(thumbnail.rgbData[ordinal*3+2]!,255)]);}
 const quantizers=new Set<bigint>();for(let position=0;position<snapshot.quantTables.length;position++){if(position%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",position,snapshot.quantTables.length);quantizers.add(integer(snapshot.quantTables[position]!.id,255));}if(snapshot.frame!==undefined){for(let position=0;position<snapshot.frame.components.length;position++){if(position%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",position,snapshot.frame.components.length);quantizers.add(integer(snapshot.frame.components[position]!.quantTableId,255));}}for(const id of [...quantizers].sort((a,b)=>a<b?-1:a>b?1:0))await out.insert("jpg_quantizer",[1n],id);
 if(snapshot.frame!==undefined){const frame=snapshot.frame;await out.insert("jpg_frame",[integer(frame.precision,255),integer(frame.width,65535),integer(frame.height,65535)]);for(let ordinal=0;ordinal<frame.components.length;ordinal++){const component=frame.components[ordinal]!;await out.insert("jpg_frame_component",[1n,BigInt(ordinal),integer(component.id,255),integer(component.hSampling,255),integer(component.vSampling,255),integer(component.quantTableId,255)]);}}
 for(let ordinal=0;ordinal<snapshot.quantTables.length;ordinal++){const table=snapshot.quantTables[ordinal]!;if(table.values.length!==64)throw new Error("JPEG quantization table requires64coefficients");const id=await out.insert("jpg_quantization_table",[1n,BigInt(ordinal),integer(table.id,255),integer(table.precision,255)]);for(let zigzag=0;zigzag<64;zigzag++)await out.insert("jpg_quantization_coefficient",[id,BigInt(zigzag),integer(table.values[zigzag]!,65535)]);}
 for(let ordinal=0;ordinal<snapshot.huffmanTables.length;ordinal++){const table=snapshot.huffmanTables[ordinal]!;if(!["dc","ac"].includes(table.class)||table.bits.length!==16)throw new Error("JPEG Huffman table has an unknown class or incomplete code-length counts");const id=await out.insert("jpg_huffman_table",[1n,BigInt(ordinal),integer(table.id,255),table.class]);for(let length=0;length<16;length++)await out.insert("jpg_huffman_code_length",[id,BigInt(length),integer(table.bits[length]!,255)]);for(let ordinal=0;ordinal<table.values.length;ordinal++)await out.insert("jpg_huffman_symbol",[id,BigInt(ordinal),integer(table.values[ordinal]!,255)]);}
 for(let ordinal=0;ordinal<snapshot.otherSegments.length;ordinal++){const segment=snapshot.otherSegments[ordinal]!,id=await out.insert("jpg_segment",[1n,BigInt(ordinal),integer(segment.marker,255)]);for(let ordinal=0;ordinal<segment.data.length;ordinal++)await out.insert("jpg_segment_octet",[id,BigInt(ordinal),integer(segment.data[ordinal]!,255)]);}return out.finish();
}

/** 📥️ Restores all typed entities while enforcing complete relational ownership. */
export async function jpgSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<JpgSnapshot>{
 const[documents,pixelRows,thumbnailRows,thumbnailPixels,frameRows,quantizerRows,componentRows,quantRows,coefficientRows,huffRows,lengthRows,symbolRows,segmentRows,segmentOctets]=await artifactSqliteTables(database,JPG_SQLITE_SCHEMA,options);const document=artifactSqliteDocument(documents!),width=field(document,2),height=field(document,3),pixels=await readGrid(pixelRows!,width,height,4,true,options);
 const thumbnail=await optional(thumbnailRows!,options),tw=thumbnail===undefined?0:field(thumbnail,1,255),th=thumbnail===undefined?0:field(thumbnail,2,255);const rgbData=await readGrid(thumbnailPixels!,tw,th,3,thumbnail!==undefined,options);const jfifThumbnail:JfifThumbnail|undefined=thumbnail===undefined?undefined:{width:tw,height:th,rgbData};
 const frameRow=await optional(frameRows!,options);const components=await groups(componentRows!,new Set(frameRow===undefined?[]:[1n]),options);const quantizers=new Set<number>();for(let position=0;position<quantizerRows!.length;position++){await checkpoint(options,position,quantizerRows!.length);const row=quantizerRows![position]!,id=field(row,0,255);if(row.rowid!==BigInt(id)||artifactSqliteInteger(row,1)!==1n||quantizers.has(id))throw new Error("JPEG quantizer identity or document reference is invalid");quantizers.add(id);}const used=new Set<number>(),frameComponents:JpgFrameComponent[]=[];
 for(const row of components.get(1n)??[]){await checkpoint(options,frameComponents.length,componentRows!.length);const quantTableId=field(row,6,255);if(!quantizers.has(quantTableId))throw new Error("JPEG component references an unknown quantizer");used.add(quantTableId);frameComponents.push({id:field(row,3,255),hSampling:field(row,4,255),vSampling:field(row,5,255),quantTableId});}const frame:JpgFrameHeader|undefined=frameRow===undefined?undefined:{precision:field(frameRow,1,255),width:field(frameRow,2,65535),height:field(frameRow,3,65535),components:frameComponents};
 const quant=await ordered(quantRows!,options),quantIds=new Set(quant.map(row=>row.rowid)),coefficients=await groups(coefficientRows!,quantIds,options);const quantTables:JpgQuantTable[]=[];for(const row of quant){await checkpoint(options,quantTables.length,quant.length);const id=field(row,3,255);if(!quantizers.has(id))throw new Error("JPEG quantization definition references an unknown quantizer");used.add(id);const values=coefficients.get(row.rowid)??[];if(values.length!==64)throw new Error("JPEG quantization definition requires all64zigzag coefficients");quantTables.push({id,precision:field(row,4,255),values:values.map(row=>field(row,3,65535))});}if(quantizers.size!==used.size)throw new Error("JPEG quantizer has no component or definition reference");
 const huff=await ordered(huffRows!,options),huffIds=new Set(huff.map(row=>row.rowid)),lengths=await groups(lengthRows!,huffIds,options),symbols=await groups(symbolRows!,huffIds,options);const huffmanTables:JpgHuffmanTable[]=[];for(const row of huff){await checkpoint(options,huffmanTables.length,huff.length);const tableClass=artifactSqliteText(row,4);if(tableClass!=="dc"&&tableClass!=="ac")throw new Error("JPEG Huffman table class must be DC or AC");const counts=lengths.get(row.rowid)??[];if(counts.length!==16)throw new Error("JPEG Huffman table requires all16code-length counts");huffmanTables.push({id:field(row,3,255),class:tableClass,bits:counts.map(row=>field(row,3,255)),values:await octets(symbols.get(row.rowid)??[],options)});}
 const segments=await ordered(segmentRows!,options),segmentIds=new Set(segments.map(row=>row.rowid)),bytes=await groups(segmentOctets!,segmentIds,options);const otherSegments:JpgSegment[]=[];for(const row of segments){await checkpoint(options,otherSegments.length,segments.length);otherSegments.push({marker:field(row,3,255),data:await octets(bytes.get(row.rowid)??[],options)});}
 const units=artifactSqliteText(document,7);if(units!=="aspect"&&units!=="pixelsPerInch"&&units!=="pixelsPerCm")throw new Error("JPEG density unit is unknown");const arithmetic=artifactSqliteInteger(document,11);if(arithmetic!==0n&&arithmetic!==1n)throw new Error("JPEG arithmetic flag must be boolean");const reEncodeQuality=document.values[4]===null?undefined:field(document,4,255),restartInterval=document.values[12]===null?undefined:field(document,12,65535);const result:JpgSnapshot={schema:artifactSqliteText(document,1),width,height,pixels,jfifVersion:[field(document,5,255),field(document,6,255)],jfifDensityUnits:units,jfifXDensity:field(document,8,65535),jfifYDensity:field(document,9,65535),sofMarker:field(document,10,255),arithmetic:arithmetic===1n,quantTables,huffmanTables,otherSegments,...(jfifThumbnail===undefined?{}:{jfifThumbnail}),...(frame===undefined?{}:{frame}),...(reEncodeQuality===undefined?{}:{reEncodeQuality}),...(restartInterval===undefined?{}:{restartInterval})};await artifactSqliteCheckpoint(options,"reconstructSnapshot",1,1);return result;
}
