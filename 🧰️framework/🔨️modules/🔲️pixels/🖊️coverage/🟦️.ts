/** 🖊️ Exact transformed polygon coverage for https://www.w3.org/TR/SVG11/painting.html#FillProperties. */
import {validateExtent} from "../✍️editing/🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../◻️2d/🧹️retire/🟦️.ts";
export type CoverageInput={width:number;height:number;transform:readonly number[];rule:"nonzero"|"evenodd";contours:readonly (readonly (readonly number[])[])[]};
export type CoverageMask={width:number;height:number;coverage:Uint8Array};
export type CoverageProgress={phase:"preparing"|"sorting"|"rasterizing"|"complete";completed:number;total:number;work:number;done:boolean};
type Point=readonly [number,number];
type Edge={x:number;y:number;dx:number;winding:number};
type SweepEvent={y:number;edge:number;add:boolean};
type Stage="prepare"|"events"|"apply"|"copy"|"sort"|"cross"|"wind"|"span"|"finish"|"write"|"done";
const EPS=1e-10,MAX_POINTS=65536,MAX_CONTOURS=65536,MAX_COORDINATE=1e9;
const invalid=(message:string):never=>{throw new RangeError(message);};
const coordinate=(value:number)=>Number.isFinite(value)&&Math.abs(value)<=MAX_COORDINATE;
const clamp=(value:number)=>Math.max(0,Math.min(1,value));
const primitive=(value:number)=>value<=0?0:value<1?value*value/2:value-.5;
function integral(a:number,b:number,height:number):number {
  if(Math.max(a,b)<=0)return 0;
  if(Math.min(a,b)>=1)return height;
  return Math.abs(b-a)<EPS?clamp((a+b)/2)*height:(primitive(b)-primitive(a))/(b-a)*height;
}
const xAt=(edge:Edge,y:number)=>edge.x+(y-edge.y)*edge.dx;
class MergeSort {
  target:number[];width=1;left=0;i=0;j=0;k=0;mid=0;end=0;writes=0;
  constructor(public source:number[]) {this.target=new Array(source.length);this.setup();}
  get done():boolean {return this.width>=this.source.length;}
  setup():void {this.i=this.left;this.mid=Math.min(this.left+this.width,this.source.length);this.j=this.mid;this.k=this.left;this.end=Math.min(this.left+2*this.width,this.source.length);}
  step(compare:(a:number,b:number)=>number):void {
    if(this.done)return;
    this.target[this.k++]=this.i<this.mid&&(this.j>=this.end||compare(this.source[this.i]!,this.source[this.j]!)<=0)?this.source[this.i++]!:this.source[this.j++]!;
    this.writes++;
    if(this.k<this.end)return;
    this.left+=2*this.width;
    if(this.left>=this.source.length){[this.source,this.target]=[this.target,this.source];this.width*=2;this.left=0;}
    this.setup();
  }
}
/** ⏱️ Budgeted sweep and exact area integration; input remains immutable until completion. */
export class CoverageJob {
  private mask:CoverageMask;private areas:Float64Array;
  private edges:Edge[]=[];private events:SweepEvent[]=[];private positions:number[]=[];private active:number[]=[];private eventIds:number[]=[];
  private sorted:number[]=[];private copied:number[]=[];private sorter:MergeSort|null=null;private stage:Stage="prepare";
  private cancelled=false;private transferred=false;private failed:unknown=null;private work=0;private contour=0;private point=0;private prepared=0;private readonly preparationTotal:number;
  private first:Point|null=null;private previous:Point|null=null;
  private event=0;private copy=0;private scan=0;private row=0;private y=0;private end=0;private winding=0;private left=-1;private spanLeft=-1;private spanRight=-1;private pixel=0;private pixelEnd=0;private dirtyStart:number;private dirtyEnd=0;
  constructor(private input:CoverageInput) {
    const count=validateExtent(input.width,input.height);
    if(!["nonzero","evenodd"].includes(input.rule)||input.transform.length!==6||!input.transform.every(coordinate)||input.contours.length>MAX_CONTOURS)invalid("Invalid vector coverage contract");
    this.preparationTotal=input.contours.reduce((total,points)=>total+points.length+1,0);
    if(this.preparationTotal-input.contours.length>MAX_POINTS)invalid("Coverage geometry exceeds point budget");
    this.mask={width:input.width,height:input.height,coverage:new Uint8Array(count)};this.areas=new Float64Array(input.width);this.dirtyStart=input.width;
  }
  private edge(from:Point,to:Point):void {
    if(from[1]===to[1])return;
    const low=from[1]<to[1]?from:to,high=from[1]<to[1]?to:from,start=Math.max(0,low[1]),end=Math.min(this.input.height,high[1]);
    if(start>=end)return;
    const dx=(high[0]-low[0])/(high[1]-low[1]);if(!Number.isFinite(dx))invalid("Coverage edge exceeds numeric limits");
    const id=this.edges.length;this.edges.push({x:low[0],y:low[1],dx,winding:from[1]<to[1]?1:-1});this.positions.push(-1);
    this.eventIds.push(this.events.length,this.events.length+1);this.events.push({y:start,edge:id,add:true},{y:end,edge:id,add:false});
  }
  private step():void {
    switch(this.stage) {
      case "prepare": {
        const points=this.input.contours[this.contour];
        if(!points){this.sorter=new MergeSort(this.eventIds);this.stage="events";break;}
        if(this.point<points.length) {
          const value=points[this.point++]!;if(value.length!==2||!value.every(coordinate))invalid("Invalid coverage point");
          const m=this.input.transform,p:Point=[m[0]!*value[0]!+m[2]!*value[1]!+m[4]!,m[1]!*value[0]!+m[3]!*value[1]!+m[5]!];
          if(!p.every(coordinate))invalid("Transformed coverage exceeds coordinate budget");
          if(this.previous)this.edge(this.previous,p);else this.first=p;this.previous=p;
        }else {if(this.first&&this.previous)this.edge(this.previous,this.first);this.contour++;this.point=0;this.first=null;this.previous=null;}
        this.prepared++;break;
      }
      case "events": {
        if(this.sorter!.done){this.sorted=this.sorter!.source;this.sorter=null;this.stage="apply";break;}
        this.sorter!.step((a,b)=>this.events[a]!.y-this.events[b]!.y||Number(this.events[a]!.add)-Number(this.events[b]!.add));break;
      }
      case "apply": {
        if(this.y>=this.input.height){this.stage="done";break;}
        const event=this.events[this.sorted[this.event]??-1];
        if(event&&event.y<=this.y) {
          this.event++;
          if(event.add){this.positions[event.edge]=this.active.length;this.active.push(event.edge);}
          else {const at=this.positions[event.edge]!,last=this.active.pop()!;if(at<this.active.length){this.active[at]=last;this.positions[last]=at;}this.positions[event.edge]=-1;}
          break;
        }
        this.end=Math.min(this.row+1,event?.y??this.input.height,this.input.height);this.copy=0;this.copied=[];this.stage="copy";break;
      }
      case "copy": {
        if(this.copy<this.active.length){this.copied.push(this.active[this.copy++]!);break;}
        this.sorter=new MergeSort(this.copied);this.stage="sort";break;
      }
      case "sort": {
        if(this.sorter!.done){this.copied=this.sorter!.source;this.sorter=null;this.scan=0;this.stage="cross";break;}
        this.sorter!.step((a,b)=>{const x=this.edges[a]!,z=this.edges[b]!,delta=xAt(x,this.y)-xAt(z,this.y);return Math.abs(delta)>EPS?delta:x.dx-z.dx||a-b;});break;
      }
      case "cross": {
        if(this.scan+1<this.copied.length) {
          const left=this.edges[this.copied[this.scan]!]!,right=this.edges[this.copied[++this.scan]!]!,slope=left.dx-right.dx;
          if(slope>0){const cross=this.y+(xAt(right,this.y)-xAt(left,this.y))/slope;if(cross>this.y+EPS&&cross<this.end)this.end=cross;}
          break;
        }
        this.scan=0;this.winding=0;this.left=-1;this.stage="wind";break;
      }
      case "wind": {
        if(this.scan>=this.copied.length){this.stage="finish";break;}
        const id=this.copied[this.scan++]!,before=this.input.rule==="nonzero"?this.winding!==0:Math.abs(this.winding)%2===1;
        this.winding+=this.edges[id]!.winding;
        const after=this.input.rule==="nonzero"?this.winding!==0:Math.abs(this.winding)%2===1;
        if(!before&&after)this.left=id;
        if(before&&!after) {
          this.spanLeft=this.left;this.spanRight=id;const left=this.edges[this.left]!,right=this.edges[id]!;
          this.pixel=Math.max(0,Math.floor(Math.min(xAt(left,this.y),xAt(left,this.end))));this.pixelEnd=Math.min(this.input.width,Math.ceil(Math.max(xAt(right,this.y),xAt(right,this.end))));this.stage="span";
        }
        break;
      }
      case "span": {
        if(this.pixel>=this.pixelEnd){this.stage="wind";break;}
        const x=this.pixel++,left=this.edges[this.spanLeft]!,right=this.edges[this.spanRight]!,height=this.end-this.y;
        this.areas[x]!+=integral(xAt(right,this.y)-x,xAt(right,this.end)-x,height)-integral(xAt(left,this.y)-x,xAt(left,this.end)-x,height);
        this.dirtyStart=Math.min(this.dirtyStart,x);this.dirtyEnd=Math.max(this.dirtyEnd,x+1);break;
      }
      case "finish": {
        this.y=this.end;if(this.y>=this.row+1){this.pixel=this.dirtyStart;this.stage="write";}else this.stage="apply";break;
      }
      case "write": {
        if(this.pixel<this.dirtyEnd){const x=this.pixel++;this.mask.coverage[this.row*this.input.width+x]=Math.round(clamp(this.areas[x]!)*255);this.areas[x]=0;break;}
        this.row++;this.dirtyStart=this.input.width;this.dirtyEnd=0;this.stage="apply";break;
      }
      case "done":break;
    }
  }
  advance(budget:number):CoverageProgress {
    if(!Number.isSafeInteger(budget)||budget<=0)invalid("Coverage work grant must be a positive integer");
    if(this.cancelled)throw new DOMException("Coverage job cancelled","AbortError");if(this.failed)throw this.failed;
    try {for(let count=0;count<budget&&this.stage!=="done";count++){this.step();this.work++;}}catch(error){this.failed=error;throw error;}
    const phase=this.stage==="prepare"?"preparing":this.stage==="events"?"sorting":this.stage==="done"?"complete":"rasterizing";
    return {phase,completed:phase==="preparing"?this.prepared:phase==="sorting"?this.sorter!.writes:this.row*this.input.width,total:phase==="preparing"?this.preparationTotal:phase==="sorting"?this.eventIds.length*Math.ceil(Math.log2(Math.max(1,this.eventIds.length))):this.mask.coverage.length,work:this.work,done:this.stage==="done"};
  }
  cancel():void {this.cancelled=true;}
  result():CoverageMask {
    if(this.cancelled)throw new DOMException("Coverage job cancelled","AbortError");if(this.failed)throw this.failed;
    if(this.stage!=="done")throw Error("Coverage job is incomplete");return this.mask;
  }
  /** 🧹️ Moves a completed mask and retires actual private sweep owners through structural grants. */
  intoRetirement():{job:WorkRetirement;output:CoverageMask|null}{
    if(this.transferred)invalid("Coverage ownership already transferred");this.transferred=true;
    const output=this.stage==="done"&&!this.cancelled&&!this.failed?this.mask:null;if(output)this.mask={width:output.width,height:output.height,coverage:new Uint8Array(0)};this.cancelled=true;let slot=0;
    return {output,job:new UnitRetirement(()=>{let complete=true;switch(slot){
      case 0:this.input={...this.input,contours:[],transform:[]};break;
      case 1:this.mask={width:this.mask.width,height:this.mask.height,coverage:new Uint8Array(0)};break;
      case 2:this.areas=new Float64Array(0);break;
      case 3:if(this.edges.length){this.edges.pop();complete=false;}else this.edges=[];break;
      case 4:if(this.events.length){this.events.pop();complete=false;}else this.events=[];break;
      case 5:this.positions=[];break;
      case 6:this.active=[];break;
      case 7:this.eventIds=[];break;
      case 8:this.sorted=[];break;
      case 9:this.copied=[];break;
      case 10:if(this.sorter)this.sorter.source=[];break;
      case 11:if(this.sorter)this.sorter.target=[];break;
      case 12:this.sorter=null;break;
      case 13:this.first=null;this.previous=null;break;
    }if(complete)slot++;return slot===14;})};
  }
}

export type CoverageOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:CoverageProgress)=>void};
/** 🕰️ Yields between work grants and publishes only completed vector coverage. */
export async function polygonCoverage(input:CoverageInput,options:CoverageOptions={}):Promise<CoverageMask> {
  const abort=()=>{if(options.signal?.aborted)throw new DOMException("Coverage job cancelled","AbortError");};
  abort();const job=new CoverageJob(input);
  try {
    for(;;) {
      abort();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);abort();
      if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));
    }
  }catch(error){job.cancel();throw error;}finally{const retired=job.intoRetirement(),grant=Number.isSafeInteger(options.workBudget)&&options.workBudget!>0?options.workBudget!:4096;while(!retired.job.advance(grant).done)await new Promise<void>(resolve=>setTimeout(resolve,0));}
}
