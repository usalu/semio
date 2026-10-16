export type ResourceSpecV1=Readonly<{kind:"buffer"|"texture";bytes:number;width:number;height:number;layers:number;mips:number;samples:number}>;
export function validateResourceSpecV1(spec:ResourceSpecV1,limits:Readonly<{bytes:number;dimension:number}>):boolean{
  if(![spec.bytes,spec.width,spec.height,spec.layers,spec.mips,spec.samples,limits.bytes,limits.dimension].every(value=>Number.isSafeInteger(value)&&value>0)||spec.bytes>limits.bytes||spec.layers!==1||![1,4].includes(spec.samples))return false;
  if(spec.kind==="buffer")return spec.width===1&&spec.height===1&&spec.mips===1&&spec.samples===1;
  if(spec.kind!=="texture"||spec.width>limits.dimension||spec.height>limits.dimension||spec.mips>Math.floor(Math.log2(Math.max(spec.width,spec.height)))+1)return false;
  let bytes=0;for(let level=0;level<spec.mips;level++)bytes+=Math.max(1,Math.floor(spec.width/2**level))*Math.max(1,Math.floor(spec.height/2**level))*4*spec.samples;
  return Number.isSafeInteger(bytes)&&bytes===spec.bytes;
}
