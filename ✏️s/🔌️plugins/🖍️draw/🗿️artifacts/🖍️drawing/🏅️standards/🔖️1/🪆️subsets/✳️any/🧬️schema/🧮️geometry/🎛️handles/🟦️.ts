/** 🎛️ Absolute world-space transforms for eight resize handles and one rotation handle. */
type Point=readonly [number,number];
type Bounds=readonly [number,number,number,number];
const anchors: readonly Point[]=[[0,0],[.5,0],[1,0],[1,.5],[1,1],[.5,1],[0,1],[0,.5]];
export function handlePoints(bounds: Bounds,zoom: number): Point[] {
  const [x,y,w,h]=bounds;
  return [...anchors.map(([u,v]):Point=>[x+u*w,y+v*h]),[x+w/2,y-28/Math.max(zoom,1e-6)]];
}
export function hitHandle(bounds: Bounds,point: Point,zoom: number): number|null {
  let best: number|null=null,distance=8/Math.max(zoom,1e-6);
  handlePoints(bounds,zoom).forEach((center,index)=>{
    const next=Math.hypot(point[0]-center[0],point[1]-center[1]);
    if(next<=distance){distance=next;best=index;}
  });
  return best;
}
export function handleMatrix(handle: number,bounds: Bounds,start: Point,end: Point,constrained: boolean,centered: boolean): [number,number,number,number,number,number]|null {
  if(![...bounds,...start,...end].every(Number.isFinite)||!Number.isInteger(handle)||handle<0||handle>8)return null;
  const [x,y,w,h]=bounds;
  if(handle===8){
    const cx=x+w/2,cy=y+h/2;
    let angle=Math.atan2(end[1]-cy,end[0]-cx)-Math.atan2(start[1]-cy,start[0]-cx);
    angle=Math.atan2(Math.sin(angle),Math.cos(angle));
    if(constrained)angle=Math.sign(angle)*Math.round(Math.abs(angle)/(Math.PI/12))*Math.PI/12;
    const c=Math.cos(angle),s=Math.sin(angle);
    const matrix:[number,number,number,number,number,number]=[c,s,-s,c,cx-c*cx+s*cy,cy-s*cx-c*cy];
    return matrix.every(Number.isFinite)?matrix:null;
  }
  const [u,v]=anchors[handle]!,ax=x+(centered?.5:1-u)*w,ay=y+(centered?.5:1-v)*h,factor=centered?2:1;
  let sx=u===.5||w===0?1:1+factor*(end[0]-start[0])/((2*u-1)*w);
  let sy=v===.5||h===0?1:1+factor*(end[1]-start[1])/((2*v-1)*h);
  if(constrained){const ratio=Math.abs(sx-1)>=Math.abs(sy-1)?sx:sy;sx=ratio;sy=ratio;}
  const matrix:[number,number,number,number,number,number]=[sx,0,0,sy,ax*(1-sx),ay*(1-sy)];
  return matrix.every(Number.isFinite)?matrix:null;
}
