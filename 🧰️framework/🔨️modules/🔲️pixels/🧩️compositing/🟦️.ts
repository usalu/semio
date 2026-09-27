/** 🧩️ Isolated RGBA compositing following https://www.w3.org/TR/compositing-1/. */
import {validateExtent,validateImage,type PixelImage,type PixelProgress} from "../✍️editing/🟦️.ts";

export type CompositeAffine = readonly [number,number,number,number,number,number];
export type CompositeBlend = "normal"|"multiply"|"screen"|"overlay"|"darken"|"lighten"|"colorDodge"|"colorBurn"|"hardLight"|"softLight"|"difference"|"exclusion"|"hue"|"saturation"|"color"|"luminosity";
export type CompositeMask = {width:number;height:number;coverage:Uint8Array;transform:CompositeAffine;invert:boolean};
type LayerProperties = {opacity:number;blend:CompositeBlend;visible:boolean;transform:CompositeAffine;mask:CompositeMask|null};
export type CompositeLayer = LayerProperties & ({kind:"pixels";image:string}|{kind:"group";children:CompositeLayer[]}|{kind:"adjustment";brightness:number;contrast:number});
export type CompositeInput = {width:number;height:number;origin:readonly[number,number];images:Record<string,PixelImage>;layers:CompositeLayer[]};
export type CompositeOptions = {signal?:AbortSignal;chunkPixels?:number;onProgress?:(progress:PixelProgress)=>void};
type Rgb = [number,number,number];
type Mask = Omit<CompositeMask,"transform"> & {inverse:CompositeAffine};
type Style = {opacity:number;blend:CompositeBlend;mask:Mask|null};
type Command = {kind:"begin";depth:number}|{kind:"commit";depth:0}|({depth:number;style:Style}&({kind:"draw";image:PixelImage;inverse:CompositeAffine}|{kind:"end"}|{kind:"adjust";slope:number;intercept:number}));

const identity:CompositeAffine=[1,0,0,1,0,0];
const blends:readonly string[]=["normal","multiply","screen","overlay","darken","lighten","colorDodge","colorBurn","hardLight","softLight","difference","exclusion","hue","saturation","color","luminosity"];
const clamp=(value:number)=>Math.max(0,Math.min(1,value));
const valid=(value:number,min:number,max:number)=>Number.isFinite(value)&&value>=min&&value<=max;
function invalid(message:string):never {throw new RangeError(message);}
const cancelled=():never=>{throw new DOMException("Compositing cancelled","AbortError");};

export function multiply(a:CompositeAffine,b:CompositeAffine):CompositeAffine {
  return [a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]];
}
export function inverse(m:CompositeAffine):CompositeAffine {
  if(m.length!==6||!m.every(Number.isFinite))invalid("Transform must contain six finite numbers");
  const det=m[0]*m[3]-m[1]*m[2];
  if(!Number.isFinite(det)||Math.abs(det)<1e-12)invalid("Transform must be invertible");
  const result:CompositeAffine=[m[3]/det,-m[1]/det,-m[2]/det,m[0]/det,(m[2]*m[5]-m[3]*m[4])/det,(m[1]*m[4]-m[0]*m[5])/det];
  if(!result.every(Number.isFinite))invalid("Inverse transform exceeds numeric limits");
  return result;
}
function indexAt(m:CompositeAffine,x:number,y:number,width:number,height:number):number {
  const px=Math.floor(m[0]*x+m[2]*y+m[4]),py=Math.floor(m[1]*x+m[3]*y+m[5]);
  return px>=0&&px<width&&py>=0&&py<height?py*width+px:-1;
}
function coverage(mask:Mask|null,x:number,y:number):number {
  if(!mask)return 1;
  const index=indexAt(mask.inverse,x,y,mask.width,mask.height),value=index<0?0:mask.coverage[index]!/255;
  return mask.invert?1-value:value;
}
const luminosity=(c:Rgb)=>0.3*c[0]+0.59*c[1]+0.11*c[2];
const saturation=(c:Rgb)=>Math.max(...c)-Math.min(...c);
function setLuminosity(c:Rgb,value:number):Rgb {
  const delta=value-luminosity(c),out:Rgb=[c[0]+delta,c[1]+delta,c[2]+delta];
  const low=Math.min(...out),high=Math.max(...out);
  if(low<0)for(let i=0;i<3;i++)out[i]=value+(out[i]!-value)*value/(value-low);
  if(high>1)for(let i=0;i<3;i++)out[i]=value+(out[i]!-value)*(1-value)/(high-value);
  return out;
}
function setSaturation(c:Rgb,value:number):Rgb {
  const order=[0,1,2].sort((a,b)=>c[a]!-c[b]!),lo=order[0]!,mid=order[1]!,hi=order[2]!,out:Rgb=[0,0,0];
  if(c[hi]!>c[lo]!){out[mid]=(c[mid]!-c[lo]!)*value/(c[hi]!-c[lo]!);out[hi]=value;}
  return out;
}
function channel(back:number,front:number,mode:CompositeBlend):number {
  switch(mode){
    case "multiply":return back*front;
    case "screen":return back+front-back*front;
    case "overlay":return back<=0.5?2*back*front:1-2*(1-back)*(1-front);
    case "darken":return Math.min(back,front);
    case "lighten":return Math.max(back,front);
    case "colorDodge":return back===0?0:front===1?1:Math.min(1,back/(1-front));
    case "colorBurn":return back===1?1:front===0?0:1-Math.min(1,(1-back)/front);
    case "hardLight":return front<=0.5?2*back*front:1-2*(1-back)*(1-front);
    case "softLight":return front<=0.5?back-(1-2*front)*back*(1-back):back+(2*front-1)*((back<=0.25?((16*back-12)*back+4)*back:Math.sqrt(back))-back);
    case "difference":return Math.abs(back-front);
    case "exclusion":return back+front-2*back*front;
    default:return front;
  }
}
function blend(back:Rgb,front:Rgb,mode:CompositeBlend):Rgb {
  switch(mode){
    case "hue":return setLuminosity(setSaturation(front,saturation(back)),luminosity(back));
    case "saturation":return setLuminosity(setSaturation(back,saturation(front)),luminosity(back));
    case "color":return setLuminosity(front,luminosity(back));
    case "luminosity":return setLuminosity(back,luminosity(front));
    default:return [channel(back[0],front[0],mode),channel(back[1],front[1],mode),channel(back[2],front[2],mode)];
  }
}
function sourceOver(target:Float64Array,index:number,front:Rgb,alpha:number,mode:CompositeBlend):void {
  if(alpha===0)return;
  const back:Rgb=[target[index]!,target[index+1]!,target[index+2]!],ba=target[index+3]!,outAlpha=alpha+ba*(1-alpha),mixed=blend(back,front,mode);
  for(let c=0;c<3;c++)target[index+c]=(alpha*((1-ba)*front[c]!+ba*mixed[c]!)+(1-alpha)*ba*back[c]!)/outAlpha;
  target[index+3]=outAlpha;
}

/** 🧱️ Bounded tile job; input pixel and coverage buffers must remain immutable until completion. */
export class CompositeJob {
  private readonly commands:Command[]=[];
  private readonly buffers:Float64Array[]=[];
  private readonly width:number;
  private readonly height:number;
  private readonly origin:readonly[number,number];
  private readonly count:number;
  private readonly total:number;
  private pixels:Uint8Array|null;
  private tile=0;
  private command=0;
  private offset=0;
  private completed=0;
  private aborted=false;

  constructor(input:CompositeInput){
    this.count=validateExtent(input.width,input.height);this.width=input.width;this.height=input.height;
    if(input.origin.length!==2||!input.origin.every(Number.isFinite))invalid("Origin must contain two finite coordinates");
    this.origin=[...input.origin];
    if(Object.keys(input.images).length>1024)invalid("Image count exceeds compositor budget");
    for(const image of Object.values(input.images))validateImage(image);
    let nodes=0,maxDepth=0;
    const compile=(layers:CompositeLayer[],parent:CompositeAffine,depth:number,enabled:boolean)=>{
      if(depth>32)invalid("Layer nesting exceeds compositor budget");
      maxDepth=Math.max(maxDepth,depth);
      for(const layer of layers){
        if(++nodes>1024)invalid("Layer count exceeds compositor budget");
        if(!valid(layer.opacity,0,1)||!blends.includes(layer.blend)||typeof layer.visible!=="boolean")invalid("Invalid layer style");
        inverse(layer.transform);
        const world=multiply(parent,layer.transform),inv=inverse(world),active=enabled&&layer.visible&&layer.opacity>0;
        let mask:Mask|null=null;
        if(layer.mask){
          const m=layer.mask;
          if(!(m.coverage instanceof Uint8Array)||m.coverage.length!==validateExtent(m.width,m.height)||typeof m.invert!=="boolean")invalid("Invalid mask coverage");
          inverse(m.transform);
          mask={width:m.width,height:m.height,coverage:m.coverage,invert:m.invert,inverse:inverse(multiply(world,m.transform))};
        }
        const style:Style={opacity:layer.opacity,blend:layer.blend,mask};
        if(layer.kind==="pixels"){
          const image=Object.hasOwn(input.images,layer.image)?input.images[layer.image]:undefined;
          if(!image)invalid("Layer image is missing");
          if(active)this.commands.push({kind:"draw",depth,style,image:{...image},inverse:inv});
        }else if(layer.kind==="group"){
          if(active)this.commands.push({kind:"begin",depth:depth+1});
          compile(layer.children,world,depth+1,active);
          if(active)this.commands.push({kind:"end",depth:depth+1,style});
        }else if(layer.kind==="adjustment"){
          if(!valid(layer.brightness,-1,1)||!valid(layer.contrast,-1,1))invalid("Adjustment exceeds allowed range");
          const slope=2**(layer.contrast*4);
          if(active)this.commands.push({kind:"adjust",depth,style,slope,intercept:(layer.brightness-0.5)*slope+0.5});
        }else invalid("Unknown layer kind");
      }
    };
    compile(input.layers,identity,0,true);
    this.commands.push({kind:"commit",depth:0});
    this.total=this.commands.length*this.count;
    for(let i=0;i<=maxDepth;i++)this.buffers.push(new Float64Array(256*4));
    this.pixels=new Uint8Array(this.count*4);
  }

  advance(budget=65536):PixelProgress {
    if(this.aborted)cancelled();
    if(!Number.isInteger(budget)||budget<1||budget>1048576)invalid("Compositing grant must be between 1 and 1048576");
    while(budget-->0&&this.tile<this.count){
      const command=this.commands[this.command]!,p=this.tile+this.offset,index=this.offset*4;
      const x=this.origin[0]+p%this.width+0.5,y=this.origin[1]+Math.floor(p/this.width)+0.5,target=this.buffers[command.depth]!;
      if(command.kind==="begin")target.fill(0,index,index+4);
      else if(command.kind==="commit"){
        for(let c=0;c<4;c++)this.pixels![p*4+c]=Math.round(clamp(target[index+c]!)*255);
        target.fill(0,index,index+4);
      }else {
        const amount=command.style.opacity*coverage(command.style.mask,x,y);
        if(command.kind==="draw"){
          const at=indexAt(command.inverse,x,y,command.image.width,command.image.height)*4;
          if(at>=0){const data=command.image.pixels;sourceOver(target,index,[data[at]!/255,data[at+1]!/255,data[at+2]!/255],data[at+3]!/255*amount,command.style.blend);}
        }else if(command.kind==="end")sourceOver(this.buffers[command.depth-1]!,index,[target[index]!,target[index+1]!,target[index+2]!],target[index+3]!*amount,command.style.blend);
        else if(target[index+3]!>0){
          const back:Rgb=[target[index]!,target[index+1]!,target[index+2]!],front=back.map(c=>clamp(c*command.slope+command.intercept)) as Rgb,mixed=blend(back,front,command.style.blend);
          for(let c=0;c<3;c++)target[index+c]=back[c]!*(1-amount)+mixed[c]!*amount;
        }
      }
      this.completed++;
      if(++this.offset===Math.min(256,this.count-this.tile)){
        this.offset=0;
        if(++this.command===this.commands.length){this.command=0;this.tile+=Math.min(256,this.count-this.tile);}
      }
    }
    return this.progress();
  }

  progress():PixelProgress {return {completed:this.completed,total:this.total,done:this.completed===this.total};}
  cancel():void {this.aborted=true;this.pixels=null;this.buffers.length=0;this.commands.length=0;}
  result():PixelImage {
    if(this.aborted)cancelled();
    if(this.completed!==this.total)throw new Error("Compositing is incomplete");
    return {width:this.width,height:this.height,pixels:this.pixels!};
  }
}

/** ⏳️ Publishes only a completed candidate and yields between bounded compositing grants. */
export async function compositeImage(input:CompositeInput,options:CompositeOptions={}):Promise<PixelImage> {
  if(options.signal?.aborted)cancelled();
  const job=new CompositeJob(input);
  try {
    for(;;){
      if(options.signal?.aborted)cancelled();
      const progress=job.advance(options.chunkPixels);options.onProgress?.(progress);
      if(options.signal?.aborted)cancelled();
      if(progress.done)return job.result();
      await new Promise<void>(resolve=>setTimeout(resolve,0));
    }
  }catch(error){job.cancel();throw error;}
}
