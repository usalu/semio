/** 📐️ Affine geometry twin, sharing the language-neutral extrema fixtures with Rust. */
import type { PathSegment } from "../🟦️.ts";
export type Matrix = [number, number, number, number, number, number];
export type Point = [number, number];
export function multiply(a: Matrix, b: Matrix): Matrix {
  return [a[0]*b[0]+a[2]*b[1], a[1]*b[0]+a[3]*b[1], a[0]*b[2]+a[2]*b[3], a[1]*b[2]+a[3]*b[3], a[0]*b[4]+a[2]*b[5]+a[4], a[1]*b[4]+a[3]*b[5]+a[5]];
}
export function cubicBounds(points: [Point, Point, Point, Point]): [number, number, number, number] {
  const result: [number, number, number, number] = [0, 0, 0, 0];
  for (const axis of [0, 1] as const) {
    const [p0, p1, p2, p3] = points.map(point => point[axis]) as [number, number, number, number];
    const a = -p0+3*p1-3*p2+p3, b = 2*(p0-2*p1+p2), c = p1-p0;
    const d = b*b-4*a*c;
    const roots = Math.abs(a)<1e-12 ? (Math.abs(b)<1e-12 ? [] : [-c/b]) : d<0 ? [] : [(-b+Math.sqrt(d))/(2*a), (-b-Math.sqrt(d))/(2*a)];
    const values = [p0,p3,...roots.filter(t => t>0 && t<1).map(t => (1-t)**3*p0+3*(1-t)**2*t*p1+3*(1-t)*t*t*p2+t**3*p3)];
    result[axis] = Math.min(...values);
    result[axis+2] = Math.max(...values)-result[axis];
  }
  return result;
}

export function inverse(matrix: Matrix): Matrix | null {
  if (!matrix.every(Number.isFinite)) return null;
  const [a,b,c,d,e,f] = matrix, determinant = a*d-b*c;
  if (!Number.isFinite(determinant) || determinant === 0) return null;
  const output: Matrix = [d/determinant,-b/determinant,-c/determinant,a/determinant,(c*f-d*e)/determinant,(b*e-a*f)/determinant];
  return output.every(Number.isFinite) ? output : null;
}

export function splitCubic(points: [Point, Point, Point, Point], t: number): [[Point, Point, Point, Point], [Point, Point, Point, Point]] | null {
  if (!Number.isFinite(t) || t<0 || t>1 || !points.flat().every(Number.isFinite)) return null;
  const mix = (a: Point,b: Point): Point => [a[0]*(1-t)+b[0]*t,a[1]*(1-t)+b[1]*t];
  const [a,b,c,d] = points, ab = mix(a,b), bc = mix(b,c), cd = mix(c,d), abc = mix(ab,bc), bcd = mix(bc,cd), center = mix(abc,bcd);
  const output: [[Point, Point, Point, Point], [Point, Point, Point, Point]] = [[a,ab,abc,center],[center,bcd,cd,d]];
  return output.flat(2).every(Number.isFinite) ? output : null;
}

/** 🥚 Center-parameterized SVG ellipse with corrected radii. */
export interface ArcGeometry { center: Point; radii: Point; rotation: number; start: number; sweep: number }
export function arcPoint(arc: ArcGeometry, t: number): Point {
  const angle = arc.start + arc.sweep*t, sin = Math.sin(angle), cos = Math.cos(angle), sr = Math.sin(arc.rotation), cr = Math.cos(arc.rotation);
  return [arc.center[0]+arc.radii[0]*cos*cr-arc.radii[1]*sin*sr,arc.center[1]+arc.radii[0]*cos*sr+arc.radii[1]*sin*cr];
}
export function arcGeometry(from: Point, radii: Point, rotationDegrees: number, largeArc: boolean, sweep: boolean, to: Point): ArcGeometry | null {
  if (![...from,...radii,rotationDegrees,...to].every(Number.isFinite) || radii.some(value => value<=0) || (from[0]===to[0] && from[1]===to[1])) return null;
  const rotation=rotationDegrees*Math.PI/180,sin=Math.sin(rotation),cos=Math.cos(rotation),dx=(from[0]-to[0])/2,dy=(from[1]-to[1])/2,x=cos*dx+sin*dy,y=-sin*dx+cos*dy;
  const factor=Math.max(1,Math.hypot(x/radii[0],y/radii[1])),rx=radii[0]*factor,ry=radii[1]*factor,nx=x/rx,ny=y/ry,length=nx*nx+ny*ny;
  if (length<=0 || !Number.isFinite(length)) return null;
  const coefficient=Math.sqrt(Math.max(0,1-length)/length)*(largeArc===sweep?-1:1),cx=coefficient*rx*ny,cy=-coefficient*ry*nx;
  const center: Point=[cos*cx-sin*cy+(from[0]+to[0])/2,sin*cx+cos*cy+(from[1]+to[1])/2];
  const a: Point=[(x-cx)/rx,(y-cy)/ry],b: Point=[(-x-cx)/rx,(-y-cy)/ry],start=Math.atan2(a[1],a[0]);
  let delta=Math.atan2(a[0]*b[1]-a[1]*b[0],a[0]*b[0]+a[1]*b[1]);
  if (sweep && delta<0) delta+=Math.PI*2;
  if (!sweep && delta>0) delta-=Math.PI*2;
  return [...center,rx,ry,rotation,start,delta].every(Number.isFinite)?{center,radii:[rx,ry],rotation,start,sweep:delta}:null;
}

/** 🎯 Exact affine bounds for a single path segment. */
export function segmentBounds(segment: PathSegment, from: Point, contourStart: Point, matrix: Matrix): [number,number,number,number] {
  const map=(point: Point): Point=>[matrix[0]*point[0]+matrix[2]*point[1]+matrix[4],matrix[1]*point[0]+matrix[3]*point[1]+matrix[5]];
  const line=(a: Point,b: Point): [number,number,number,number]=>{a=map(a);b=map(b);return [Math.min(a[0],b[0]),Math.min(a[1],b[1]),Math.abs(a[0]-b[0]),Math.abs(a[1]-b[1])];};
  switch(segment.kind) {
    case "move": { const to=map(segment.to); return [...to,0,0]; }
    case "line": return line(from,segment.to);
    case "close": return line(from,contourStart);
    case "cubic": return cubicBounds([map(from),map(segment.ctrl1),map(segment.ctrl2),map(segment.to)]);
    case "quad": {
      const a: Point=[from[0]+(segment.ctrl[0]-from[0])*2/3,from[1]+(segment.ctrl[1]-from[1])*2/3],b: Point=[segment.to[0]+(segment.ctrl[0]-segment.to[0])*2/3,segment.to[1]+(segment.ctrl[1]-segment.to[1])*2/3];
      return cubicBounds([map(from),map(a),map(b),map(segment.to)]);
    }
    case "arc": {
      const arc=arcGeometry(from,[segment.rx,segment.ry],segment.rotation,segment.largeArc,segment.sweep,segment.to);
      if (!arc) return line(from,segment.to);
      const [rx,ry]=arc.radii,sin=Math.sin(arc.rotation),cos=Math.cos(arc.rotation),u=[matrix[0]*rx*cos+matrix[2]*rx*sin,matrix[1]*rx*cos+matrix[3]*rx*sin],v=[-matrix[0]*ry*sin+matrix[2]*ry*cos,-matrix[1]*ry*sin+matrix[3]*ry*cos];
      const result=line(from,segment.to),tau=Math.PI*2;
      for (const axis of [0,1] as const) {
        const angle=Math.atan2(v[axis]!,u[axis]!);
        for (const candidate of [angle,angle+Math.PI]) {
          const raw=arc.sweep>=0?candidate-arc.start:arc.start-candidate,distance=((raw%tau)+tau)%tau;
          if (distance<=Math.abs(arc.sweep)) {
            const point=map(arcPoint(arc,distance/Math.abs(arc.sweep))),max=Math.max(result[axis]+result[axis+2]!,point[axis]);
            result[axis]=Math.min(result[axis],point[axis]);result[axis+2]=max-result[axis];
          }
        }
      }
      return result;
    }
  }
}
