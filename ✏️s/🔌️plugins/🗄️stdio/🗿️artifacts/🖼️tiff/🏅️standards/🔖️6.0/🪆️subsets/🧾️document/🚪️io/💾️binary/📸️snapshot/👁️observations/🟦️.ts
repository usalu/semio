/** 👁️ Ephemeral native carrier observations. */
export interface TiffNativeObservations {
 ifdCount:number; raster:boolean;
 compression:number[]|null;
 photometric:number[]|null;
 bitsPerSample:number[]|null;
 tileWidth:number[]|null;
 tileLength:number[]|null;
 stripOffsets:number[]|null;
}
