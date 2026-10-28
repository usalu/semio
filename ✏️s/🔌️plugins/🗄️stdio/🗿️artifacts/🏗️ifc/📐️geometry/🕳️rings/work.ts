/** 🕳️ One shared ring evaluator retains every source-sized loop across bounded continuations. */
import type {RingPoint,RingTriangle} from './🟦️.ts';
export type RingSteps<T>=Generator<void,T,void>;
type Uv=[number,number];
const EPS=128*Number.EPSILON,cross=(a:Uv,b:Uv,c:Uv)=>(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]),equal=(a:Uv,b:Uv)=>a[0]===b[0]&&a[1]===b[1];
const on=(a:Uv,b:Uv,p:Uv)=>Math.abs(cross(a,b,p))<=EPS&&p[0]>=Math.min(a[0],b[0])-EPS&&p[0]<=Math.max(a[0],b[0])+EPS&&p[1]>=Math.min(a[1],b[1])-EPS&&p[1]<=Math.max(a[1],b[1])+EPS;
const intersects=(a:Uv,b:Uv,c:Uv,d:Uv)=>{const x=cross(a,b,c),y=cross(a,b,d),z=cross(c,d,a),w=cross(c,d,b);return(x>EPS&&y< -EPS||x< -EPS&&y>EPS)&&(z>EPS&&w< -EPS||z< -EPS&&w>EPS)||on(a,b,c)||on(a,b,d)||on(c,d,a)||on(c,d,b);};
const edgeKey=(a:number,b:number)=>a<b?a+':'+b:b+':'+a,fail=(message:string):never=>{throw new Error(message);};
function* append<T>(target:T[],source:readonly T[],start=0,end=source.length):RingSteps<void>{for(let at=start;at<end;at++){yield;target.push(source[at]!);}}
function* reverse<T>(values:T[]):RingSteps<void>{for(let at=0;at<Math.floor(values.length/2);at++){yield;const other=values.length-at-1,value=values[at]!;values[at]=values[other]!;values[other]=value;}}

/** 📐️ Preserve original index topology, hole tie order and checked coverage without cold roster operations. */
export function* triangulateRingsSteps(vertices:readonly RingPoint[],input:readonly(readonly number[])[]):RingSteps<RingTriangle[]>{
 yield;if(!input.length||input[0]!.length<3)fail('polygon-needs-three-points');
 const origin=vertices[input[0]![0]!];if(!origin)fail('invalid-polygon-index');
 let extent=0;const points=new Map<number,RingPoint>();
 for(const ring of input)for(const id of ring){yield;const point=vertices[id];if(!point||!point.every(Number.isFinite))fail('invalid-polygon-index');const value=point.map((n,k)=>n-origin[k]!) as RingPoint;if(!value.every(Number.isFinite))fail('polygon-coordinate-range');for(const n of value)extent=Math.max(extent,Math.abs(n));points.set(id,value);}
 if(!(extent>0))fail('degenerate-polygon');
 for(const[id,point]of points){yield;points.set(id,point.map(n=>n/extent) as RingPoint);}
 const normal:RingPoint=[0,0,0],outer=input[0]!;
 for(let at=0;at<outer.length;at++){yield;const a=points.get(outer[at]!)!,b=points.get(outer[(at+1)%outer.length]!)!;normal[0]+=a[1]*b[2]-a[2]*b[1];normal[1]+=a[2]*b[0]-a[0]*b[2];normal[2]+=a[0]*b[1]-a[1]*b[0];}
 let axis=0;for(let k=1;k<3;k++)if(Math.abs(normal[k]!)>Math.abs(normal[axis]!))axis=k;
 const length=Math.hypot(...normal);if(length<=EPS)fail('degenerate-polygon');
 const uv=new Map<number,Uv>();
 for(const[id,point]of points){yield;if(Math.abs(point.reduce((sum,n,k)=>sum+n*normal[k]!/length,0))>1e-9)fail('nonplanar-polygon');uv.set(id,[point[(axis+1)%3]!,point[(axis+2)%3]!]);}
 const p=(id:number):Uv=>uv.get(id)!;
 function* area(ring:number[]):RingSteps<number>{let sum=0;for(let at=0;at<ring.length;at++){yield;const a=p(ring[at]!),b=p(ring[(at+1)%ring.length]!);sum+=a[0]*b[1]-b[0]*a[1];}return sum/2;}
 function* inside(q:Uv,ring:number[]):RingSteps<number>{let value=false;for(let at=0,prior=ring.length-1;at<ring.length;prior=at++){yield;const a=p(ring[prior]!),b=p(ring[at]!);if(on(a,b,q))return 0;if((a[1]>q[1])!==(b[1]>q[1])&&q[0]<(b[0]-a[0])*(q[1]-a[1])/(b[1]-a[1])+a[0])value=!value;}return value?1:-1;}
 const rings:number[][]=[];
 for(const source of input){yield;const ring:number[]=[];yield* append(ring,source);if(ring.length>1&&equal(p(ring[0]!),p(ring[ring.length-1]!)))ring.pop();rings.push(ring);}
 for(const ring of rings){yield;if(ring.length<3||Math.abs(yield* area(ring))<=EPS)fail('degenerate-polygon');for(let at=0;at<ring.length;at++){yield;const a=p(ring[at]!),b=p(ring[(at+1)%ring.length]!);if(equal(a,b))fail('zero-length-polygon-edge');for(let other=at+1;other<ring.length;other++){yield;if(other===at+1||at===0&&other===ring.length-1)continue;if(intersects(a,b,p(ring[other]!),p(ring[(other+1)%ring.length]!)))fail('self-intersecting-polygon');}}}
 for(let i=0;i<rings.length;i++)for(let j=i+1;j<rings.length;j++){
  yield;for(let a=0;a<rings[i]!.length;a++)for(let b=0;b<rings[j]!.length;b++){yield;if(intersects(p(rings[i]![a]!),p(rings[i]![(a+1)%rings[i]!.length]!),p(rings[j]![b]!),p(rings[j]![(b+1)%rings[j]!.length]!)))fail('intersecting-polygon-rings');}
  if(i===0&&(yield* inside(p(rings[j]![0]!),rings[0]!))!==1)fail('hole-outside-polygon');
  if(i>0&&((yield* inside(p(rings[i]![0]!),rings[j]!))!==-1||(yield* inside(p(rings[j]![0]!),rings[i]!))!==-1))fail('nested-polygon-holes');
 }
 const winding=Math.sign(yield* area(rings[0]!));
 for(let at=0;at<rings.length;at++){yield;if(Math.sign(yield* area(rings[at]!))!==(at===0?1:-1))yield* reverse(rings[at]!);}
 function* filled(q:Uv):RingSteps<boolean>{if((yield* inside(q,rings[0]!))!==1)return false;for(let at=1;at<rings.length;at++){yield;if((yield* inside(q,rings[at]!))!==-1)return false;}return true;}
 function* blocked(a:number,b:number,ring:number[]):RingSteps<boolean>{for(let at=0;at<ring.length;at++){yield;const c=ring[at]!,d=ring[(at+1)%ring.length]!;if(!intersects(p(a),p(b),p(c),p(d)))continue;if((a===c&&b===d)||(a===d&&b===c))return true;const shared=[a,b].filter(n=>n===c||n===d);if(!shared.length)return true;const other=c===shared[0]?d:c;if(on(p(a),p(b),p(other)))return true;}return false;}
 const locallyInside=(ring:number[],at:number,target:number)=>{const a=p(ring[(at+ring.length-1)%ring.length]!),b=p(ring[at]!),c=p(ring[(at+1)%ring.length]!),q=p(target);return cross(a,b,c)>EPS?cross(b,q,c)<=EPS&&cross(b,a,q)<=EPS:cross(b,q,a)>EPS||cross(b,c,q)>EPS;};
 let merged:number[]=[];yield* append(merged,rings[0]!);
 const holes:{ring:number[];at:number}[]=[],compare=(a:{ring:number[];at:number},b:{ring:number[];at:number})=>p(a.ring[a.at]!)[0]-p(b.ring[b.at]!)[0]||p(a.ring[a.at]!)[1]-p(b.ring[b.at]!)[1];
 for(let ordinal=1;ordinal<rings.length;ordinal++){yield;const ring=rings[ordinal]!;let at=0;for(let i=1;i<ring.length;i++){yield;const a=p(ring[i]!),b=p(ring[at]!);if(a[0]<b[0]||a[0]===b[0]&&a[1]<b[1])at=i;}const hole={ring,at};let insertion=holes.length;holes.push(hole);while(insertion>0){yield;if(compare(holes[insertion-1]!,hole)<=0)break;holes[insertion]=holes[insertion-1]!;insertion--;}holes[insertion]=hole;}
 for(const{ring,at}of holes){
  yield;const h=ring[at]!;let best=-1,distance=Infinity;
  for(let i=0;i<merged.length;i++){yield;const o=merged[i]!,a=p(h),b=p(o),d=(a[0]-b[0])**2+(a[1]-b[1])**2;if(d>=distance||!locallyInside(merged,i,h)||!locallyInside(ring,at,o)||!(yield* filled([(a[0]+b[0])/2,(a[1]+b[1])/2])))continue;let blockedRing=false;for(const boundary of rings){yield;if(yield* blocked(h,o,boundary)){blockedRing=true;break;}}if(blockedRing||(yield* blocked(h,o,merged)))continue;best=i;distance=d;}
  if(best<0)fail('polygon-hole-bridge-failed');
  const next:number[]=[];yield* append(next,merged,0,best+1);yield* append(next,ring,at);yield* append(next,ring,0,at);next.push(h,merged[best]!);yield* append(next,merged,best+1);merged=next;
 }
 const triangles:RingTriangle[]=[];
 while(merged.length>3){yield;let ear=-1;for(let at=0;at<merged.length;at++){
  yield;const triangle:RingTriangle=[merged[(at+merged.length-1)%merged.length]!,merged[at]!,merged[(at+1)%merged.length]!],a=p(triangle[0]),b=p(triangle[1]),c=p(triangle[2]);if(cross(a,b,c)<=EPS)continue;
  let contains=false;for(const id of merged){yield;const q=p(id);if(equal(q,a)||equal(q,b)||equal(q,c))continue;if(cross(a,b,q)>= -EPS&&cross(b,c,q)>= -EPS&&cross(c,a,q)>= -EPS){contains=true;break;}}
  if(!contains){ear=at;triangles.push(triangle);break;}
 }if(ear<0)fail('polygon-triangulation-failed');for(let at=ear;at<merged.length-1;at++){yield;merged[at]=merged[at+1]!;}merged.pop();}
 if(merged.length!==3||cross(p(merged[0]!),p(merged[1]!),p(merged[2]!))<=EPS)fail('degenerate-polygon-triangle');
 triangles.push(merged as RingTriangle);let expected=0,covered=0;const edges=new Map<string,number>();
 for(const ring of rings){yield;expected+=yield* area(ring);}
 for(const triangle of triangles){yield;const a=p(triangle[0]),b=p(triangle[1]),c=p(triangle[2]);covered+=cross(a,b,c)/2;if(!(yield* filled([(a[0]+b[0]+c[0])/3,(a[1]+b[1]+c[1])/3])))fail('polygon-triangle-outside-domain');for(let at=0;at<3;at++){const key=edgeKey(triangle[at]!,triangle[(at+1)%3]!);edges.set(key,(edges.get(key)??0)+1);}}
 if(Math.abs(covered-expected)>Math.max(EPS,Math.abs(expected)*1e-9))fail('polygon-coverage-mismatch');
 const boundary=new Set<string>();for(const ring of rings)for(let at=0;at<ring.length;at++){yield;boundary.add(edgeKey(ring[at]!,ring[(at+1)%ring.length]!));}
 for(const[key,count]of edges){yield;if(count!==(boundary.has(key)?1:2))fail('polygon-edge-incidence');}
 for(const key of boundary){yield;if(edges.get(key)!==1)fail('polygon-boundary-missing');}
 for(let i=0;i<triangles.length;i++)for(let j=i+1;j<triangles.length;j++)for(let a=0;a<3;a++)for(let b=0;b<3;b++){yield;const u=triangles[i]![a]!,v=triangles[i]![(a+1)%3]!,x=triangles[j]![b]!,y=triangles[j]![(b+1)%3]!;if(u===x||u===y||v===x||v===y)continue;if(intersects(p(u),p(v),p(x),p(y)))fail('polygon-triangles-overlap');}
 if(winding<0)for(let at=0;at<triangles.length;at++){yield;const[a,b,c]=triangles[at]!;triangles[at]=[a,c,b];}
 return triangles;
}
