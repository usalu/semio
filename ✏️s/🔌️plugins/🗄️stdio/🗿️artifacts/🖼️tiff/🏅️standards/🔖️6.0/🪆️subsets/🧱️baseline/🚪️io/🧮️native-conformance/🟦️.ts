/** 🛡️ Native TIFF6 Baseline storage class, evaluated at IO. */
import type {TiffNativeObservations} from "../../../🧾️document/🚪️io/💾️binary/📸️snapshot/👁️observations/🟦️.ts";
export function classifyTiffNativeBaseline(o:TiffNativeObservations):string[]{
 const codes:string[]=[];const add=(code:string)=>codes.push("stdio.tiff.baseline."+code);
 if(!o.raster)add("degenerate-raster");
 if(o.compression?.[0]!==undefined&&![1,2,32773].includes(o.compression[0]))add("unsupported-compression");
 if(o.photometric?.[0]!==undefined&&o.photometric[0]>3)add("unsupported-photometric");
 if(o.bitsPerSample?.some(bit=>![1,4,8].includes(bit)))add("unsupported-bits-per-sample");
 if(o.tileWidth!==null||o.tileLength!==null)add("tiled-not-baseline");else if(o.stripOffsets===null)add("missing-strip-offsets");
 return codes;
}
