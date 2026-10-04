/** 🖊️ Stroke regions for https://www.w3.org/TR/SVG11/painting.html#StrokeProperties. */
import type {Vec2} from "../../🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../🧹️retire/🟦️.ts";
export type StrokeGeometryStyle={width:number;cap:"butt"|"round"|"square";join:"miter"|"round"|"bevel";miterLimit:number;dash:readonly number[];dashOffset:number};
export type StrokeOutlineInput={contours:readonly {points:readonly Vec2[];closed:boolean}[];transform:readonly number[];tolerance:number;style:StrokeGeometryStyle};
export type StrokeOutlineProgress={phase:"preparing"|"dashing"|"outlining"|"complete";completed:number;total:number;points:number;work:number;done:boolean};
type Source={points:Vec2[];closed:boolean;raw:number};
type Run={points:Vec2[];closed:boolean;capStart:boolean;capEnd:boolean;painted:boolean;tangent:Vec2};
type Join=readonly [Vec2,Vec2,Vec2];
type Primitive={kind:"polygon";points:Vec2[]}|{kind:"round";center:Vec2;a:number;b:number};
type Round={center:Vec2;points:Vec2[];stack:{a:number;b:number;depth:number}[]};
type Stage="prepare"|"source"|"offset"|"walk"|"finish"|"outline"|"done";
const MAX_POINTS=65536,MAX_CONTOURS=65536,MAX_COORDINATE=1e9;
const valid=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=MAX_COORDINATE;
const same=(a:Vec2,b:Vec2)=>a[0]===b[0]&&a[1]===b[1];
const invalid=(message:string):never=>{throw new RangeError(message);};
function unit(a:Vec2,b:Vec2):Vec2 {const x=b[0]-a[0],y=b[1]-a[1],length=Math.hypot(x,y);return length?[x/length,y/length]:[1,0];}
const normal=(t:Vec2):Vec2=>[-t[1],t[0]];
const shifted=(p:Vec2,t:Vec2,d:number):Vec2=>[p[0]+t[0]*d,p[1]+t[1]*d];
/** 🧱️ Budgeted dash runs and positive polygon unions retain crossings and short-segment joins. */
export class StrokeOutlineJob {
 private sources:Source[]=[];private prepared:Vec2[]=[];private runs:Run[]=[];private seams:Join[]=[];private polygons:Vec2[][]=[];private queue:Primitive[]=[];private round:Round|null=null;
 private stage:Stage="prepare";private contour=0;private point=0;private source=0;private edge=0;private position=0;private pattern:number[];private patternTotal:number;private dash=0;private remaining=0;private offset=0;private active:Run|null=null;private first=-1;private last=-1;
 private run=0;private geomEdge=0;private cap=0;private seam=0;private count=0;private runPoints=0;private work=0;private cancelled=false;private transferred=false;private outputExposed=false;private failed:unknown=null;private readonly radius:number;private readonly roundBound:number;
 constructor(private input:StrokeOutlineInput) {
  const s=input.style;
  if(input.transform.length!==6||!input.transform.every(valid)||!Number.isFinite(input.tolerance)||input.tolerance<1e-6||input.tolerance>16||input.contours.length>4096||input.contours.reduce((n,c)=>n+c.points.length,0)>MAX_POINTS||!valid(s.width)||s.width<0||!["butt","round","square"].includes(s.cap)||!["miter","round","bevel"].includes(s.join)||!Number.isFinite(s.miterLimit)||s.miterLimit<1||s.miterLimit>1000||s.dash.length>1024||!s.dash.every(v=>valid(v)&&v>=0)||!valid(s.dashOffset))invalid("Invalid stroke geometry contract");
  this.pattern=[...s.dash,...(s.dash.length%2?s.dash:[])];this.patternTotal=this.pattern.reduce((a,b)=>a+b,0);if(this.patternTotal===0||this.pattern.every((v,at)=>at%2===0||v===0))this.pattern=[];
  this.radius=s.width/2;const m=input.transform;this.roundBound=this.radius*(Math.hypot(m[0]!,m[1]!)+Math.hypot(m[2]!,m[3]!));
 }
 private check(p:Vec2):void {
  const m=this.input.transform;if(p.length!==2||!p.every(valid)||!valid(m[0]!*p[0]+m[2]!*p[1]+m[4]!)||!valid(m[1]!*p[0]+m[3]!*p[1]+m[5]!))invalid("Invalid stroke geometry point");
 }
 private runPoint(run:Run,p:Vec2):void {if(this.runPoints>=MAX_POINTS)invalid("Stroke dashing exceeds point budget");run.points.push(p);this.runPoints++;}
 private createRun(a:Vec2,b:Vec2,positive:boolean):void {
  const src=this.sources[this.source]!,n=src.points.length,edge=Math.min(this.edge,src.closed?n-1:n-2),tangent=edge>=0?unit(src.points[edge]!,src.points[(edge+1)%n]!):[1,0] as Vec2;
  const run:Run={points:[],closed:false,capStart:true,capEnd:true,painted:positive,tangent};this.runPoint(run,a);this.runPoint(run,b);this.runs.push(run);this.active=run;
  if(positive){const at=this.runs.length-1;if(this.first<0)this.first=at;this.last=at;}
 }
 private location():Vec2 {
  const src=this.sources[this.source]!,count=src.closed?src.points.length:src.points.length-1;
  if(this.edge>=count)return src.closed?src.points[0]!:src.points.at(-1)!;
  const a=src.points[this.edge]!,b=src.points[(this.edge+1)%src.points.length]!,length=Math.hypot(b[0]-a[0],b[1]-a[1]);
  return this.position===0?a:[a[0]+(b[0]-a[0])*this.position/length,a[1]+(b[1]-a[1])*this.position/length];
 }
 private polygon(points:Vec2[]):void {
  let area=0;const origin=points[0]!;for(let at=0;at<points.length;at++){this.check(points[at]!);const a=points[at]!,b=points[(at+1)%points.length]!;area+=(a[0]-origin[0])*(b[1]-origin[1])-(a[1]-origin[1])*(b[0]-origin[0]);}
  if(area===0)return;if(this.count+points.length>MAX_POINTS||this.polygons.length>=MAX_CONTOURS)invalid("Stroke outline exceeds geometry budget");
  this.count+=points.length;this.polygons.push(area<0?points.reverse():points);
 }
 private circlePoint(center:Vec2,angle:number):Vec2 {return [center[0]+this.radius*Math.cos(angle),center[1]+this.radius*Math.sin(angle)];}
 private roundPoint(p:Vec2):void {this.check(p);if(this.count>=MAX_POINTS)invalid("Stroke outline exceeds point budget");this.round!.points.push(p);this.count++;}
 private join([before,p,after]:Join):void {
  const a=unit(before,p),b=unit(p,after),cross=a[0]*b[1]-a[1]*b[0],dot=Math.max(-1,Math.min(1,a[0]*b[0]+a[1]*b[1]));if(cross===0&&dot>0)return;
  const side=cross<0?1:-1,n=normal(a),z=normal(b),from=shifted(p,n,side*this.radius),to=shifted(p,z,side*this.radius);
  if(this.input.style.join==="round") {
   const start=cross<0?to:from,theta=Math.atan2(start[1]-p[1],start[0]-p[0]),span=Math.atan2(Math.abs(cross),dot);
   this.queue.push({kind:"round",center:p,a:theta,b:theta+span});return;
  }
  const denominator=1+dot,ratio=denominator>0?Math.sqrt(2/denominator):Infinity;
  if(this.input.style.join==="miter"&&ratio<=this.input.style.miterLimit)this.queue.push({kind:"polygon",points:[p,from,[p[0]+side*this.radius*(n[0]+z[0])/denominator,p[1]+side*this.radius*(n[1]+z[1])/denominator],to]});
  else this.queue.push({kind:"polygon",points:[p,from,to]});
 }
 private endpoint(p:Vec2,t:Vec2,start:boolean):void {
  if(this.input.style.cap==="butt")return;const n=normal(t),a=shifted(p,n,this.radius),b=shifted(p,n,-this.radius);
  if(this.input.style.cap==="square"){const extension=shifted([0,0],t,(start?-1:1)*this.radius);this.queue.push({kind:"polygon",points:[a,b,[b[0]+extension[0],b[1]+extension[1]],[a[0]+extension[0],a[1]+extension[1]]]});}
  else {const theta=Math.atan2(start?n[1]:-n[1],start?n[0]:-n[0]);this.queue.push({kind:"round",center:p,a:theta,b:theta+Math.PI});}
 }
 private outline():void {
  if(this.round) {
   const node=this.round.stack.pop();if(!node){this.round=null;return;}
   const span=node.b-node.a,error=2*this.roundBound*Math.sin(span/4)**2;
   if(span<=Math.PI&&error<=this.input.tolerance){this.roundPoint(this.circlePoint(this.round.center,node.b));return;}
   if(node.depth>=32)invalid("Stroke outline exceeds subdivision budget");const middle=(node.a+node.b)/2;
   this.round.stack.push({a:middle,b:node.b,depth:node.depth+1},{a:node.a,b:middle,depth:node.depth+1});return;
  }
  const primitive=this.queue.shift();
  if(primitive){if(primitive.kind==="polygon")this.polygon(primitive.points);else {if(this.polygons.length>=MAX_CONTOURS)invalid("Stroke outline exceeds contour budget");this.round={center:primitive.center,points:[],stack:[{a:primitive.a,b:primitive.b,depth:0}]};this.polygons.push(this.round.points);this.roundPoint(primitive.center);this.roundPoint(this.circlePoint(primitive.center,primitive.a));}return;}
  const run=this.runs[this.run];
  if(!run){const seam=this.seams[this.seam++];if(seam)this.join(seam);else this.stage="done";return;}
  const points=run.points,count=run.closed?points.length:points.length-1;
  if(points.length<2||same(points[0]!,points.at(-1)!)&&points.length===2) {
   const p=points[0]!;if(this.input.style.cap==="round")this.queue.push({kind:"round",center:p,a:0,b:2*Math.PI});
   else if(this.input.style.cap==="square"){const t=run.tangent,n=normal(t),a=shifted(p,t,-this.radius),b=shifted(p,t,this.radius);this.queue.push({kind:"polygon",points:[shifted(a,n,this.radius),shifted(b,n,this.radius),shifted(b,n,-this.radius),shifted(a,n,-this.radius)]});}
   this.run++;return;
  }
  if(this.geomEdge<count){const at=this.geomEdge++,a=points[at]!,b=points[(at+1)%points.length]!,n=normal(unit(a,b));this.queue.push({kind:"polygon",points:[shifted(a,n,this.radius),shifted(b,n,this.radius),shifted(b,n,-this.radius),shifted(a,n,-this.radius)]});if(at>0||run.closed)this.join([points[(at-1+points.length)%points.length]!,a,b]);return;}
  if(this.cap===0){this.cap++;if(!run.closed&&run.capStart)this.endpoint(points[0]!,unit(points[0]!,points[1]!),true);return;}
  if(this.cap===1){this.cap++;if(!run.closed&&run.capEnd)this.endpoint(points.at(-1)!,unit(points.at(-2)!,points.at(-1)!),false);return;}
  this.run++;this.geomEdge=0;this.cap=0;
 }
 private step():void {
  switch(this.stage){
   case "prepare":{
    const contour=this.input.contours[this.contour];if(!contour){this.stage=this.radius===0?"done":"source";break;}
    if(typeof contour.closed!=="boolean")invalid("Invalid stroke contour closure");
    if(this.point<contour.points.length){const p=contour.points[this.point++]!;this.check(p);if(!this.prepared.length||!same(this.prepared.at(-1)!,p))this.prepared.push([p[0],p[1]]);}
    else {if(contour.closed&&this.prepared.length>1&&same(this.prepared[0]!,this.prepared.at(-1)!))this.prepared.pop();if(this.prepared.length>=2||this.prepared.length===1&&(contour.points.length>=2||contour.closed))this.sources.push({points:this.prepared,closed:contour.closed,raw:contour.points.length});this.contour++;this.point=0;this.prepared=[];}break;
   }
   case "source":{
    const src=this.sources[this.source];if(!src){this.stage="outline";break;}
    if(this.pattern.length===0){this.runs.push({points:src.points,closed:src.closed,capStart:true,capEnd:true,painted:true,tangent:[1,0]});this.source++;break;}
    this.edge=0;this.position=0;this.dash=0;this.offset=((this.input.style.dashOffset%this.patternTotal)+this.patternTotal)%this.patternTotal;this.active=null;this.first=-1;this.last=-1;this.stage="offset";break;
   }
   case "offset":{const length=this.pattern[this.dash]!;if(this.offset>0&&this.offset>=length){this.offset-=length;this.dash=(this.dash+1)%this.pattern.length;}else{this.remaining=length-this.offset;this.offset=0;this.stage="walk";}break;}
   case "walk":{
    const src=this.sources[this.source]!,count=src.closed?src.points.length:src.points.length-1;
    if(this.remaining===0){if(this.dash%2===0&&!this.active){const p=this.location();this.createRun(p,p,false);}this.dash=(this.dash+1)%this.pattern.length;this.remaining=this.pattern[this.dash]!;break;}
    if(this.dash%2===1)this.active=null;
    if(this.edge>=count){if(src.points.length===1&&this.dash%2===0&&!this.active)this.createRun(src.points[0]!,src.points[0]!,false);this.stage="finish";break;}
    const a=src.points[this.edge]!,b=src.points[(this.edge+1)%src.points.length]!,length=Math.hypot(b[0]-a[0],b[1]-a[1]),left=length-this.position,take=Math.min(left,this.remaining),from=this.location();
    const endEdge=take===left,endDash=take===this.remaining;if(take>0&&this.position+take===this.position)invalid("Stroke dashing exceeds numeric limits");
    this.position+=take;const to=endEdge?b:this.location();
    if(this.dash%2===0){if(this.active){if(!this.active.painted){this.active.points.pop();this.active.painted=true;const at=this.runs.length-1;if(this.first<0)this.first=at;this.last=at;}this.runPoint(this.active,to);}else this.createRun(from,to,true);}
    this.remaining-=take;if(endDash){this.dash=(this.dash+1)%this.pattern.length;this.remaining=this.pattern[this.dash]!;}
    if(endEdge){this.edge++;this.position=0;}break;
   }
   case "finish":{
    const src=this.sources[this.source]!;
    if(src.closed&&this.first>=0&&this.last>=0){const first=this.runs[this.first]!,last=this.runs[this.last]!;
     if(same(first.points[0]!,src.points[0]!)&&same(last.points.at(-1)!,src.points[0]!)){
      if(first===last){first.closed=true;first.points.pop();}else{first.capStart=false;last.capEnd=false;this.seams.push([last.points.at(-2)!,src.points[0]!,first.points[1]!]);}
     }
    }
    this.active=null;this.source++;this.stage="source";break;
   }
   case "outline":this.outline();break;
   case "done":break;
  }
 }
 advance(budget:number):StrokeOutlineProgress {
  if(!Number.isSafeInteger(budget)||budget<=0)invalid("Stroke work grant must be a positive integer");if(this.cancelled)throw new DOMException("Stroke preparation cancelled","AbortError");if(this.failed)throw this.failed;
  try{for(let at=0;at<budget&&this.stage!=="done";at++){this.step();this.work++;}}catch(error){this.failed=error;throw error;}
  const phase=this.stage==="prepare"?"preparing":this.stage==="outline"?"outlining":this.stage==="done"?"complete":"dashing";
  return {phase,completed:phase==="preparing"?this.contour:phase==="dashing"?this.source:this.run,total:phase==="preparing"?this.input.contours.length:phase==="dashing"?this.sources.length:this.runs.length,points:this.count,work:this.work,done:this.stage==="done"};
 }
 result():Vec2[][] {if(this.cancelled)throw new DOMException("Stroke preparation cancelled","AbortError");if(this.failed)throw this.failed;if(this.stage!=="done")throw Error("Stroke preparation is incomplete");this.outputExposed=true;return this.polygons;}
 /** 🧹️ Transfers completed polygons and drains actual private owners through structural work grants. */
 intoRetirement():{job:WorkRetirement;output:Vec2[][]|null}{
  if(this.transferred)invalid("Stroke preparation ownership already transferred");this.transferred=true;const output=this.stage==="done"&&!this.cancelled&&!this.failed?this.polygons:null;if(output)this.polygons=[];this.cancelled=true;this.active=null;let slot=0;
  const step=():boolean=>{let complete=true;switch(slot){
   case 0:this.input={...this.input,contours:[],transform:[]};break;
   case 1:if(this.sources.length){this.sources.pop();complete=false;}else this.sources=[];break;
   case 2:this.prepared=[];break;
   case 3:if(this.runs.length){this.runs.pop();complete=false;}else this.runs=[];break;
   case 4:this.seams=[];break;
   case 5:if(this.polygons.length){this.polygons.pop();complete=false;}else this.polygons=[];break;
   case 6:if(this.queue.length){this.queue.pop();complete=false;}else this.queue=[];break;
   case 7:this.round=null;break;
   case 8:this.input={...this.input,style:{...this.input.style,dash:[]}};break;
   case 9:this.pattern=[];break;
  }if(complete)slot++;return slot===10;};return{job:new UnitRetirement(step),output};
 }
 cancel():void {this.cancelled=true;this.active=null;if(!this.transferred&&this.outputExposed)this.polygons=[];}
}

export type StrokeOutlineOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:StrokeOutlineProgress)=>void};
/** ⏳️ Yields between dash/outline grants and publishes only completed stroke candidates. */
export async function prepareStroke(input:StrokeOutlineInput,options:StrokeOutlineOptions={}):Promise<Vec2[][]> {
 const abort=()=>{if(options.signal?.aborted)throw new DOMException("Stroke preparation cancelled","AbortError");};
 abort();const job=new StrokeOutlineJob(input);
 try {
  for(;;){abort();const progress=job.advance(options.workBudget??4096);options.onProgress?.(progress);abort();if(progress.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}
 }catch(error){job.cancel();throw error;}finally{const retired=job.intoRetirement(),grant=Number.isSafeInteger(options.workBudget)&&options.workBudget!>0?options.workBudget!:4096;while(!retired.job.advance(grant).done)await new Promise<void>(resolve=>setTimeout(resolve,0));}
}
