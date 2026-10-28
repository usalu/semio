export type ConvexProfilePoint=[number,number];
export type ConvexProfileBoundary={kind:'circle';radius:number;chords:number}|{kind:'rounded-rectangle';width:number;height:number;radius:number;chords:number};
const CIRCLE_CARDINALS=[[1,0],[0,1],[-1,0],[0,-1]] as const;
/** 🔲️ Count the exact indexed boundary, including nonzero straight joins and equality limits. */
export function convexProfileSize(source:ConvexProfileBoundary):number|undefined{
 const{radius,chords}=source;if(!(radius>0)||!Number.isFinite(radius)||!Number.isSafeInteger(chords)||chords<4||chords%4!==0)return undefined;
 if(source.kind==='circle')return chords;
 if(source.kind!=='rounded-rectangle'||!(source.width>0)||!(source.height>0)||!Number.isFinite(source.width)||!Number.isFinite(source.height)||radius>source.width/2||radius>source.height/2)return undefined;
 const size=chords+(radius<source.width/2?2:0)+(radius<source.height/2?2:0);return Number.isSafeInteger(size)?size:undefined;
}
/** 🧭️ Produce one bounded CCW sample with exact cardinals and no fabricated radius defaults. */
export function convexProfilePoint(source:ConvexProfileBoundary,index:number):ConvexProfilePoint|undefined{
 const size=convexProfileSize(source);if(size===undefined||!Number.isSafeInteger(index)||index<0||index>=size)return undefined;
 const{radius,chords}=source,q=chords/4;
 if(source.kind==='circle'){if(index%q===0){const cardinal=CIRCLE_CARDINALS[index/q]!;return[radius*cardinal[0],radius*cardinal[1]];}const angle=2*Math.PI*index/chords;return[radius*Math.cos(angle),radius*Math.sin(angle)];}
 const x=source.width/2,y=source.height/2,a=x-radius,b=y-radius;
 for(let quadrant=0;quadrant<4;quadrant++){const count=q+((quadrant%2===0?a:b)>0?1:0);if(index>=count){index-=count;continue;}
  if(index===0)return quadrant===0?[x,b]:quadrant===1?[-a,y]:quadrant===2?[-x,-b]:[a,-y];
  if(index===q)return quadrant===0?[a,y]:quadrant===1?[-x,b]:quadrant===2?[-a,-y]:[x,-b];
  const angle=Math.PI/2*(quadrant+index/q);return[(quadrant===0||quadrant===3?a:-a)+radius*Math.cos(angle),(quadrant<2?b:-b)+radius*Math.sin(angle)];
 }return undefined;
}
