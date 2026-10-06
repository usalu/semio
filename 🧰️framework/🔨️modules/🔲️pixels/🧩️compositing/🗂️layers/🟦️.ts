/** 🗂️ Retained preparation of centered raster layers for the shared compositor. */
import type {WorkRetirement} from "../../../◻️2d/🧹️retire/🟦️.ts";
import {validateExtent,validateImage,type PixelImage,type PixelProgress} from "../../✍️editing/🟦️.ts";
import {CompositeJob,inverse,multiply,type CompositeAffine,type CompositeBlend,type CompositeLayer,type CompositeMask} from "../🟦️.ts";

/** 🎭️ Effective coverage of imported RGBA masks, shared by compositing and mask authoring. */
export function maskCoverage(red:number,green:number,blue:number,alpha:number):number {
  return Math.round((0.2126*red+0.7152*green+0.0722*blue)*alpha/255);
}

export type RasterStackTransform={x:number;y:number;a:number;b:number;c:number;d:number};
export type RasterStackMask={enabled:boolean;linked:boolean;invert:boolean;transform:RasterStackTransform;width?:number|null;height?:number|null;imageKey?:string|null};
type Properties={id:string;visible:boolean;opacity:number;blendMode:CompositeBlend;transform:RasterStackTransform};
export type RasterStackLayer=Properties&({kind:"pixel";width?:number|null;height?:number|null;imageKey?:string|null;mask?:RasterStackMask|null}|{kind:"group";children:RasterStackLayer[];mask?:RasterStackMask|null}|{kind:"adjustment";adjustmentKind:"brightnessContrast";params:{brightness?:number;contrast?:number}});
export type RasterStackInput={layers:RasterStackLayer[];images:Record<string,PixelImage>};
export type RasterStackResult={image:PixelImage;origin:readonly[number,number];empty:boolean};
type Preparation={image:PixelImage;coverage:Uint8Array;offset:number};
const identity:CompositeAffine=[1,0,0,1,0,0];
const invalid=(message:string):never=>{throw new RangeError(message);};
function transform(value:RasterStackTransform):CompositeAffine {
  if(!value)invalid("Layer transform requires finite coordinates");
  const result:CompositeAffine=[value.a,value.b,value.c,value.d,value.x,value.y];
  inverse(result);return result;
}

function centered(value:CompositeAffine,width:number,height:number,sourceWidth:number,sourceHeight:number):CompositeAffine {
  validateExtent(width,height);
  return multiply(value,[width/sourceWidth,0,0,height/sourceHeight,-width/2,-height/2]);
}

/** 🧵️ Borrows immutable RGBA buffers and converts unique masks within each advance grant. */
export class RasterStackJob {
  private readonly preparations:Preparation[]=[];
  private readonly origin:readonly[number,number];
  private readonly empty:boolean;
  private composite:CompositeJob|null;private compositeRetirement:WorkRetirement|null=null;private output:PixelImage|null=null;private compositeCompleted=0;private compositeTotal=0;private done=false;
  private preparation=0;
  private prepared=0;
  private preparationTotal=0;
  private aborted=false;

  constructor(input:RasterStackInput){
    if(!input||!Array.isArray(input.layers)||!input.images||Object.keys(input.images).length>1024)invalid("Invalid raster stack");
    for(const image of Object.values(input.images))validateImage(image);
    const images:Record<string,PixelImage>=Object.create(null),masks=new Map<string,Preparation>(),ids=new Set<string>();
    let nodes=0,minX=Infinity,minY=Infinity,maxX=-Infinity,maxY=-Infinity;
    const imageAt=(key:string):PixelImage=>{
      if(typeof key!=="string"||!key||!Object.hasOwn(input.images,key))invalid("Layer image is missing");
      return input.images[key]!;
    };
    const maskFor=(mask:RasterStackMask|null|undefined,local:CompositeAffine,layer:CompositeAffine):CompositeMask|null=>{
      if(!mask)return null;
      if([mask.enabled,mask.linked,mask.invert].some(value=>typeof value!=="boolean"))invalid("Invalid layer mask flags");
      const placement=transform(mask.transform);
      if(mask.width!=null)validateExtent(mask.width,1);
      if(mask.height!=null)validateExtent(1,mask.height);
      if(mask.width!=null&&mask.height!=null)validateExtent(mask.width,mask.height);
      if(!mask.enabled||mask.imageKey==null)return null;
      const image=imageAt(mask.imageKey);
      let prep=masks.get(mask.imageKey);
      if(!prep){
        const count=image.width*image.height;
        if(this.preparationTotal+count>67108864)invalid("Mask coverage exceeds 64 MiB preparation budget");
        prep={image:{...image},coverage:new Uint8Array(count),offset:0};
        masks.set(mask.imageKey,prep);this.preparations.push(prep);this.preparationTotal+=count;
      }
      return {width:image.width,height:image.height,coverage:prep.coverage,invert:mask.invert,transform:multiply(inverse(local),multiply(mask.linked?layer:identity,centered(placement,mask.width??image.width,mask.height??image.height,image.width,image.height)))};
    };
    const compile=(layers:RasterStackLayer[],parent:CompositeAffine,depth:number,enabled:boolean):CompositeLayer[]=>{
      if(depth>32||!Array.isArray(layers))invalid("Invalid layer nesting");
      const output:CompositeLayer[]=[];
      for(const layer of layers){
        if(++nodes>1024||!layer||typeof layer.id!=="string"||!layer.id||ids.has(layer.id))invalid("Layer identifiers must be unique within 1024 nodes");
        ids.add(layer.id);
        const matrix=transform(layer.transform),active=enabled&&layer.visible;
        let local=matrix;
        let image:PixelImage|undefined;
        if(layer.kind==="pixel"){
          image=layer.imageKey==null?undefined:imageAt(layer.imageKey);
          const width=layer.width??image?.width??512,height=layer.height??image?.height??512;
          image??={width:1,height:1,pixels:new Uint8Array(4)};
          local=centered(matrix,width,height,image.width,image.height);
          const world=multiply(parent,local),w=image.width,h=image.height;
          if(active)for(const [x,y] of [[0,0],[w,0],[0,h],[w,h]] as const){
            const px=world[0]*x+world[2]*y+world[4],py=world[1]*x+world[3]*y+world[5];
            minX=Math.min(minX,px);maxX=Math.max(maxX,px);minY=Math.min(minY,py);maxY=Math.max(maxY,py);
          }
        }
        if(layer.kind==="adjustment"&&"mask" in layer&&layer.mask!=null)invalid("Adjustment layers cannot own masks");
        const mask=layer.kind==="adjustment"?null:layer.mask;
        const properties={visible:layer.visible,opacity:mask?.enabled&&mask.imageKey==null&&mask.invert?0:layer.opacity,blend:layer.blendMode,transform:local,mask:maskFor(mask,local,matrix)};
        if(!Number.isFinite(layer.opacity)||layer.opacity<0||layer.opacity>1)invalid("Invalid layer opacity");
        if(layer.kind==="pixel"){
          const key=String(nodes);images[key]={...image!};output.push({...properties,kind:"pixels",image:key});
        }else if(layer.kind==="group")output.push({...properties,kind:"group",children:compile(layer.children,multiply(parent,local),depth+1,active)});
        else if(layer.kind==="adjustment"){
          if(layer.adjustmentKind!=="brightnessContrast"||!layer.params)invalid("Unsupported layer adjustment");
          output.push({...properties,kind:"adjustment",brightness:layer.params.brightness??0,contrast:layer.params.contrast??0});
        }else invalid("Unknown layer kind");
      }
      return output;
    };
    const layers=compile(input.layers,identity,0,true);
    this.empty=minX===Infinity;this.origin=this.empty?[0,0]:[minX,minY];
    this.composite=new CompositeJob({width:this.empty?1:Math.max(1,Math.ceil(maxX-minX)),height:this.empty?1:Math.max(1,Math.ceil(maxY-minY)),origin:this.origin,layers,images});this.compositeTotal=this.composite.progress().total;
  }

  advance(budget=65536):PixelProgress {
    if(this.aborted)throw new DOMException("Raster stack cancelled","AbortError");
    if(!Number.isInteger(budget)||budget<1||budget>1048576)invalid("Stack grant must be between 1 and 1048576");
    while(budget>0&&this.preparation<this.preparations.length){
      const prep=this.preparations[this.preparation]!,end=Math.min(prep.coverage.length,prep.offset+budget),bytes=prep.image.pixels;
      budget-=end-prep.offset;this.prepared+=end-prep.offset;
      for(;prep.offset<end;prep.offset++){
        const at=prep.offset*4;
        prep.coverage[prep.offset]=maskCoverage(bytes[at]!,bytes[at+1]!,bytes[at+2]!,bytes[at+3]!);
      }
      if(end===prep.coverage.length)this.preparation++;
    }
    if(this.composite&&budget>0){const progress=this.composite.advance(budget);budget-=progress.completed-this.compositeCompleted;this.compositeCompleted=progress.completed;if(progress.done){const retired=this.composite.intoRetirement();this.compositeRetirement=retired.job;this.output=retired.output!;this.composite=null;}}
    while(budget-->0&&this.compositeRetirement){if(!this.compositeRetirement.terminalIsEmpty())this.compositeRetirement.advance(1);else{this.compositeRetirement=null;this.done=true;}}
    return {completed:this.prepared+this.compositeCompleted,total:this.preparationTotal+this.compositeTotal,done:this.done};
  }

  result():RasterStackResult {
    if(this.aborted)throw new DOMException("Raster stack cancelled","AbortError");
    if(!this.done)throw Error("Raster stack is incomplete");return {image:this.output!,origin:[...this.origin],empty:this.empty};
  }
  cancel():void {this.aborted=true;this.composite?.cancel();this.composite=null;this.compositeRetirement=null;this.output=null;this.preparations.length=0;}
}
