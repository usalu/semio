/** 🧭️ Paint surface coordinates and compact selection transport. */
export type PixelLayer = { id: string; name: string; visible: boolean; width: number; height: number; imageKey: string | null; matrix: readonly number[] };
type LayerInput = { kind?: string; id?: string; name?: string; visible?: boolean; width?: number; height?: number; imageKey?: string; transform?: {x?:number;y?:number;rotation?:number;scaleX?:number;scaleY?:number}; children?: LayerInput[] };
const identity = [1,0,0,1,0,0];
function multiply(a: readonly number[], b: readonly number[]): number[] {
  return [a[0]!*b[0]!+a[2]!*b[1]!,a[1]!*b[0]!+a[3]!*b[1]!,a[0]!*b[2]!+a[2]!*b[3]!,a[1]!*b[2]!+a[3]!*b[3]!,a[0]!*b[4]!+a[2]!*b[5]!+a[4]!,a[1]!*b[4]!+a[3]!*b[5]!+a[5]!];
}
export function pixelLayers(json: string,assetExtentsJson="{}"): PixelLayer[] {
  const root = JSON.parse(json) as {layers?:LayerInput[]};
  const assets=JSON.parse(assetExtentsJson) as Record<string,{width?:number;height?:number}>;
  const result:PixelLayer[] = [];
  const visit = (layers:LayerInput[],parent:readonly number[],visible:boolean,depth:number) => {
    if (depth > 32) throw new Error("Layer nesting exceeds the editor limit");
    for (const layer of layers) {
      const t = layer.transform ?? {}, angle=(t.rotation ?? 0)*Math.PI/180, cos=Math.cos(angle),sin=Math.sin(angle);
      const matrix=multiply(parent,[cos*(t.scaleX ?? 1),sin*(t.scaleX ?? 1),-sin*(t.scaleY ?? 1),cos*(t.scaleY ?? 1),t.x ?? 0,t.y ?? 0]);
      const shown=visible && layer.visible !== false;
      if (layer.kind === "pixel" && layer.id) {
        const extent=layer.imageKey?assets[layer.imageKey]:undefined;
        const displayWidth=layer.width ?? extent?.width ?? 512,displayHeight=layer.height ?? extent?.height ?? 512;
        const width=extent?.width ?? displayWidth,height=extent?.height ?? displayHeight;
        result.push({id:layer.id,name:layer.name ?? layer.id,visible:shown,width,height,imageKey:layer.imageKey ?? null,matrix:multiply(matrix,[displayWidth/width,0,0,displayHeight/height,-displayWidth/2,-displayHeight/2])});
      }
      if (layer.kind === "group") visit(layer.children ?? [],matrix,shown,depth+1);
    }
  };
  visit(root.layers ?? [],identity,true,0);
  return result;
}
export function layerPoint(layer:PixelLayer,x:number,y:number): readonly [number,number] {
  const [a,b,c,d,e,f]=layer.matrix as [number,number,number,number,number,number], determinant=a*d-b*c;
  if (!Number.isFinite(determinant) || Math.abs(determinant)<1e-12) throw new Error("Layer transform is not invertible");
  return [(d*(x-e)-c*(y-f))/determinant,(-b*(x-e)+a*(y-f))/determinant];
}
export type SelectionScanOptions = {signal?:AbortSignal;onProgress?:(progress:{completed:number;total:number;done:boolean})=>void};
function checkSelectionScan(options:SelectionScanOptions):void {
  if(options.signal?.aborted) throw new DOMException("Cancelled","AbortError");
}
async function scanSelection(mask:Uint8Array,visit:(index:number,value:number)=>void,options:SelectionScanOptions):Promise<void> {
  checkSelectionScan(options);
  if(mask.length>16777216) throw new Error("Selection exceeds the pixel budget");
  for(let start=0;start<mask.length;start+=32768) {
    checkSelectionScan(options);
    const end=Math.min(mask.length,start+32768);
    for(let index=start;index<end;index++) visit(index,mask[index]!);
    options.onProgress?.({completed:end,total:mask.length,done:end===mask.length});
    checkSelectionScan(options);
    if(end<mask.length) await new Promise<void>(resolve=>setTimeout(resolve,0));
  }
  checkSelectionScan(options);
}
export async function selectionSpans(mask:Uint8Array|undefined,options:SelectionScanOptions={}):Promise<string|null> {
  checkSelectionScan(options);
  if(!mask) return null;
  const spans:string[]=[];let start=0,value=0,length=2;
  const append=(end:number)=>{
    if(!value) return;
    const span=JSON.stringify([start,end-start,value]);length+=span.length+(spans.length?1:0);
    if(length>40000) throw new Error("Selection is too detailed for one edit; simplify the selection");
    spans.push(span);
  };
  await scanSelection(mask,(index,next)=>{if(next!==value){append(index);start=index;value=next;}},options);
  checkSelectionScan(options);
  append(mask.length);
  return "["+spans.join(",")+"]";
}
export async function selectionBounds(mask:Uint8Array,width:number,options:SelectionScanOptions={}):Promise<{x:number;y:number;width:number;height:number}|null> {
  checkSelectionScan(options);
  if(!Number.isInteger(width)||width<1||mask.length%width!==0) throw new Error("Selection dimensions are invalid");
  let left=width,top=mask.length/width,right=-1,bottom=-1;
  await scanSelection(mask,(index,value)=>{
    if(!value) return;
    const x=index%width,y=Math.floor(index/width);
    left=Math.min(left,x);top=Math.min(top,y);right=Math.max(right,x);bottom=Math.max(bottom,y);
  },options);
  checkSelectionScan(options);
  return right<0?null:{x:left,y:top,width:right-left+1,height:bottom-top+1};
}
