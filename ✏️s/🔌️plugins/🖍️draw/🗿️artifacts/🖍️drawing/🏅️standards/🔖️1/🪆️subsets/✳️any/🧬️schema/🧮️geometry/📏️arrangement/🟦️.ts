/** 📏️ Document-axis alignment and equal edge spacing, preserving input order. */
export function arrange(bounds:readonly (readonly number[])[],operation:string):[number,number][]|null {
  if(!["alignLeft","alignCenter","alignRight","alignTop","alignMiddle","alignBottom","distributeHorizontal","distributeVertical"].includes(operation)) return null;
  const distributing=operation.startsWith("distribute"),horizontal=["alignLeft","alignCenter","alignRight","distributeHorizontal"].includes(operation),axis=horizontal?0:1;
  if(bounds.length<(distributing?3:2) || bounds.some(b=>b.length!==4 || !b.every(Number.isFinite) || b[2]!<0 || b[3]!<0 || !Number.isFinite(b[axis]!+b[axis+2]!))) return null;
  const order=bounds.map((_,i)=>i),starts=bounds.map(b=>b[axis]!),sizes=bounds.map(b=>b[axis+2]!);
  const min=starts.reduce((a,b)=>Math.min(a,b),Infinity),max=bounds.reduce((a,b)=>Math.max(a,b[axis]!+b[axis+2]!),-Infinity);
  if(distributing) order.sort((a,b)=>starts[a]!-starts[b]!);
  const gap=(max-min-sizes.reduce((a,b)=>a+b,0))/(bounds.length-1),result=bounds.map(()=>[0,0] as [number,number]);
  let cursor=min;
  for(const index of order) {
    const destination=distributing?cursor:["alignCenter","alignMiddle"].includes(operation)?min/2+max/2-sizes[index]!/2:["alignRight","alignBottom"].includes(operation)?max-sizes[index]!:min;
    result[index]![axis]=destination-starts[index]!;
    if(distributing) cursor+=sizes[index]!+gap;
  }
  return result.every(delta=>delta.every(Number.isFinite))?result:null;
}
