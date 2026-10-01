/** 🎥️ Complete retained MP4 domain with exact native integer widths. */
export interface Mp4Ftyp {majorBrand:string;minorVersion:number;compatibleBrands:string[]}
export interface Mp4AvcExtension {chromaFormat:number;bitDepthLumaMinus8:number;bitDepthChromaMinus8:number;spsExt:number[][]}
export type Mp4CodecFormat="avc1"|"avc3"|"hvc1"|"hev1"|"jpeg"|"mjpa";
export interface Mp4HevcNalArray {arrayCompleteness:boolean;nalUnitType:number;nalUnits:number[][]}
export interface Mp4HevcConfig {generalProfileSpace:number;generalTierFlag:boolean;generalProfileIdc:number;generalProfileCompatibilityFlags:number;generalConstraintIndicatorFlags:bigint;generalLevelIdc:number;minSpatialSegmentationIdc:number;parallelismType:number;chromaFormatIdc:number;bitDepthLumaMinus8:number;bitDepthChromaMinus8:number;avgFrameRate:number;constantFrameRate:number;numTemporalLayers:number;temporalIdNested:boolean;arrays:Mp4HevcNalArray[]}
export interface Mp4Codec {format:Mp4CodecFormat;sps:number[][];pps:number[][];nalLengthSize:number;extension:Mp4AvcExtension|null;hevc:Mp4HevcConfig|null}
export interface Mp4Sample {data:number[];duration:number;ctsOffset:number;sync:boolean}
export interface Mp4Movie {creationTime:bigint;modificationTime:bigint;timescale:number;duration:bigint;rate:number;volume:number;matrix:number[];nextTrackId:number;title:string|null;encoder:string|null}
export interface Mp4Edit {segmentDuration:bigint;mediaTime:bigint;mediaRateInteger:number;mediaRateFraction:number}
export interface Mp4VisualSampleEntry {dataReferenceIndex:number;version:number;revisionLevel:number;vendor:number;temporalQuality:number;spatialQuality:number;horizontalResolution:number;verticalResolution:number;frameCount:number;compressorName:string;depth:number;colorTableId:number}
export interface Mp4Color {colorType:string;primaries:number;transfer:number;matrix:number;fullRange:boolean|null}
export interface Mp4PixelAspectRatio {horizontalSpacing:number;verticalSpacing:number}
export interface Mp4Bitrate {bufferSize:number;maximum:number;average:number}
export interface Mp4TrackMetadata {creationTime:bigint;modificationTime:bigint;flags:number;duration:bigint;layer:number;alternateGroup:number;volume:number;matrix:number[];mediaDuration:bigint;mediaCreationTime:bigint;mediaModificationTime:bigint;language:string;quality:number;handlerName:string;edits:Mp4Edit[];visual:Mp4VisualSampleEntry;color:Mp4Color|null;pixelAspectRatio:Mp4PixelAspectRatio|null;bitrate:Mp4Bitrate|null}
export interface Mp4Track {trackId:number;timescale:number;codec:Mp4Codec;width:number;height:number;metadata:Mp4TrackMetadata;chunkSampleCounts:number[];samples:Mp4Sample[]}
export interface Mp4Snapshot {schema:string;ftyp:Mp4Ftyp;movie:Mp4Movie;tracks:Mp4Track[]}
function object(value:unknown,keys:readonly string[]):Record<string,unknown>{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("MP4 record requires an object");const record=value as Record<string,unknown>;if(Object.keys(record).some(key=>!keys.includes(key))||keys.some(key=>!(key in record)))throw Error("MP4 record requires exactly its owned fields");return record}
function integer(value:unknown,min:number,max:number):number{if(typeof value!=="number"||!Number.isInteger(value)||value<min||value>max)throw Error("MP4 integer exceeds its native width");return value}
function u8(value:unknown):number{return integer(value,0,255)}
function u16(value:unknown):number{return integer(value,0,65535)}
function u32(value:unknown):number{return integer(value,0,4294967295)}
function i16(value:unknown):number{return integer(value,-32768,32767)}
function i32(value:unknown):number{return integer(value,-2147483648,2147483647)}
function wide(value:unknown,min:bigint,max:bigint):bigint{if(typeof value==="string"){if(value.length>20||! /^(?:0|-?[1-9][0-9]*)$/.test(value))throw Error("MP4 wide integer requires canonical decimal text");value=BigInt(value)}if(typeof value!=="bigint"||value<min||value>max)throw Error("MP4 wide integer exceeds its native width");return value}
function u64(value:unknown):bigint{return wide(value,0n,18446744073709551615n)}
function i64(value:unknown):bigint{return wide(value,-9223372036854775808n,9223372036854775807n)}
function text(value:unknown):string{if(typeof value!=="string")throw Error("MP4 requires text");return value}
function boolean(value:unknown):boolean{if(typeof value!=="boolean")throw Error("MP4 requires boolean state");return value}
function list<T>(value:unknown,parse:(value:unknown)=>T):T[]{if(!Array.isArray(value))throw Error("MP4 requires a list");return value.map(parse)}
function optional<T>(value:unknown,parse:(value:unknown)=>T):T|null{return value===null?null:parse(value)}
function bytes(value:unknown):number[]{return list(value,u8)}
function matrix(value:unknown):number[]{const result=list(value,i32);if(result.length!==9)throw Error("MP4 matrix requires nine components");return result}
/** 🏷️ Admits all file-type brand occurrences. */
export function parseMp4Ftyp(value:unknown):Mp4Ftyp{const v=object(value,["majorBrand","minorVersion","compatibleBrands"]);return{majorBrand:text(v.majorBrand),minorVersion:u32(v.minorVersion),compatibleBrands:list(v.compatibleBrands,text)}}
/** 🧩️ Admits the complete optional AVC extension. */
export function parseMp4AvcExtension(value:unknown):Mp4AvcExtension{const v=object(value,["chromaFormat","bitDepthLumaMinus8","bitDepthChromaMinus8","spsExt"]);return{chromaFormat:u8(v.chromaFormat),bitDepthLumaMinus8:u8(v.bitDepthLumaMinus8),bitDepthChromaMinus8:u8(v.bitDepthChromaMinus8),spsExt:list(v.spsExt,bytes)}}
/** 🧱️ Admits one complete ordered HEVC array. */
export function parseMp4HevcNalArray(value:unknown):Mp4HevcNalArray{const v=object(value,["arrayCompleteness","nalUnitType","nalUnits"]);return{arrayCompleteness:boolean(v.arrayCompleteness),nalUnitType:u8(v.nalUnitType),nalUnits:list(v.nalUnits,bytes)}}
/** 🎛️ Admits every HEVC configuration scalar and ordered NAL group. */
export function parseMp4HevcConfig(value:unknown):Mp4HevcConfig{const v=object(value,["generalProfileSpace","generalTierFlag","generalProfileIdc","generalProfileCompatibilityFlags","generalConstraintIndicatorFlags","generalLevelIdc","minSpatialSegmentationIdc","parallelismType","chromaFormatIdc","bitDepthLumaMinus8","bitDepthChromaMinus8","avgFrameRate","constantFrameRate","numTemporalLayers","temporalIdNested","arrays"]);return{generalProfileSpace:u8(v.generalProfileSpace),generalTierFlag:boolean(v.generalTierFlag),generalProfileIdc:u8(v.generalProfileIdc),generalProfileCompatibilityFlags:u32(v.generalProfileCompatibilityFlags),generalConstraintIndicatorFlags:u64(v.generalConstraintIndicatorFlags),generalLevelIdc:u8(v.generalLevelIdc),minSpatialSegmentationIdc:u16(v.minSpatialSegmentationIdc),parallelismType:u8(v.parallelismType),chromaFormatIdc:u8(v.chromaFormatIdc),bitDepthLumaMinus8:u8(v.bitDepthLumaMinus8),bitDepthChromaMinus8:u8(v.bitDepthChromaMinus8),avgFrameRate:u16(v.avgFrameRate),constantFrameRate:u8(v.constantFrameRate),numTemporalLayers:u8(v.numTemporalLayers),temporalIdNested:boolean(v.temporalIdNested),arrays:list(v.arrays,parseMp4HevcNalArray)}}
/** 🎞️ Admits retained configurations independently of native writer format checks. */
export function parseMp4Codec(value:unknown):Mp4Codec{const v=object(value,["format","sps","pps","nalLengthSize","extension","hevc"]);if(!["avc1","avc3","hvc1","hev1","jpeg","mjpa"].includes(text(v.format)))throw Error("MP4 codec format is unknown");return{format:v.format as Mp4CodecFormat,sps:list(v.sps,bytes),pps:list(v.pps,bytes),nalLengthSize:u8(v.nalLengthSize),extension:optional(v.extension,parseMp4AvcExtension),hevc:optional(v.hevc,parseMp4HevcConfig)}}
/** 🎬️ Admits exact movie counters and nullable descriptive fields. */
export function parseMp4Movie(value:unknown):Mp4Movie{const v=object(value,["creationTime","modificationTime","timescale","duration","rate","volume","matrix","nextTrackId","title","encoder"]);return{creationTime:u64(v.creationTime),modificationTime:u64(v.modificationTime),timescale:u32(v.timescale),duration:u64(v.duration),rate:i32(v.rate),volume:i16(v.volume),matrix:matrix(v.matrix),nextTrackId:u32(v.nextTrackId),title:optional(v.title,text),encoder:optional(v.encoder,text)}}
/** ✂️ Admits exact unsigned duration and signed media time. */
export function parseMp4Edit(value:unknown):Mp4Edit{const v=object(value,["segmentDuration","mediaTime","mediaRateInteger","mediaRateFraction"]);return{segmentDuration:u64(v.segmentDuration),mediaTime:i64(v.mediaTime),mediaRateInteger:i16(v.mediaRateInteger),mediaRateFraction:i16(v.mediaRateFraction)}}
/** 🖼️ Admits all twelve visual sample entry fields. */
export function parseMp4VisualSampleEntry(value:unknown):Mp4VisualSampleEntry{const v=object(value,["dataReferenceIndex","version","revisionLevel","vendor","temporalQuality","spatialQuality","horizontalResolution","verticalResolution","frameCount","compressorName","depth","colorTableId"]);return{dataReferenceIndex:u16(v.dataReferenceIndex),version:u16(v.version),revisionLevel:u16(v.revisionLevel),vendor:u32(v.vendor),temporalQuality:u32(v.temporalQuality),spatialQuality:u32(v.spatialQuality),horizontalResolution:u32(v.horizontalResolution),verticalResolution:u32(v.verticalResolution),frameCount:u16(v.frameCount),compressorName:text(v.compressorName),depth:u16(v.depth),colorTableId:i16(v.colorTableId)}}
/** 🎨️ Admits color metadata with absent, false and true range state. */
export function parseMp4Color(value:unknown):Mp4Color{const v=object(value,["colorType","primaries","transfer","matrix","fullRange"]);return{colorType:text(v.colorType),primaries:u16(v.primaries),transfer:u16(v.transfer),matrix:u16(v.matrix),fullRange:optional(v.fullRange,boolean)}}
/** 📐️ Admits exact pixel spacing counters. */
export function parseMp4PixelAspectRatio(value:unknown):Mp4PixelAspectRatio{const v=object(value,["horizontalSpacing","verticalSpacing"]);return{horizontalSpacing:u32(v.horizontalSpacing),verticalSpacing:u32(v.verticalSpacing)}}
/** 📊️ Admits the complete bitrate record. */
export function parseMp4Bitrate(value:unknown):Mp4Bitrate{const v=object(value,["bufferSize","maximum","average"]);return{bufferSize:u32(v.bufferSize),maximum:u32(v.maximum),average:u32(v.average)}}
/** 🪪️ Admits every retained track metadata owner and relation. */
export function parseMp4TrackMetadata(value:unknown):Mp4TrackMetadata{const v=object(value,["creationTime","modificationTime","flags","duration","layer","alternateGroup","volume","matrix","mediaDuration","mediaCreationTime","mediaModificationTime","language","quality","handlerName","edits","visual","color","pixelAspectRatio","bitrate"]);return{creationTime:u64(v.creationTime),modificationTime:u64(v.modificationTime),flags:u32(v.flags),duration:u64(v.duration),layer:i16(v.layer),alternateGroup:i16(v.alternateGroup),volume:i16(v.volume),matrix:matrix(v.matrix),mediaDuration:u64(v.mediaDuration),mediaCreationTime:u64(v.mediaCreationTime),mediaModificationTime:u64(v.mediaModificationTime),language:text(v.language),quality:u16(v.quality),handlerName:text(v.handlerName),edits:list(v.edits,parseMp4Edit),visual:parseMp4VisualSampleEntry(v.visual),color:optional(v.color,parseMp4Color),pixelAspectRatio:optional(v.pixelAspectRatio,parseMp4PixelAspectRatio),bitrate:optional(v.bitrate,parseMp4Bitrate)}}
/** 🎟️ Admits intrinsic sample octets and timing state. */
export function parseMp4Sample(value:unknown):Mp4Sample{const v=object(value,["data","duration","ctsOffset","sync"]);return{data:bytes(v.data),duration:u32(v.duration),ctsOffset:i32(v.ctsOffset),sync:boolean(v.sync)}}
/** 🛤️ Admits a complete retained track occurrence. */
export function parseMp4Track(value:unknown):Mp4Track{const v=object(value,["trackId","timescale","codec","width","height","metadata","chunkSampleCounts","samples"]);return{trackId:u32(v.trackId),timescale:u32(v.timescale),codec:parseMp4Codec(v.codec),width:u32(v.width),height:u32(v.height),metadata:parseMp4TrackMetadata(v.metadata),chunkSampleCounts:list(v.chunkSampleCounts,u32),samples:list(v.samples,parseMp4Sample)}}
/** 📸️ Admits the complete canonical movie snapshot. */
export function parseMp4Snapshot(value:unknown):Mp4Snapshot{const v=object(value,["schema","ftyp","movie","tracks"]);return{schema:text(v.schema),ftyp:parseMp4Ftyp(v.ftyp),movie:parseMp4Movie(v.movie),tracks:list(v.tracks,parseMp4Track)}}
