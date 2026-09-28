/** 🗂️ Stable stack moves for selected sibling layers, from back to front. */
export function stackMoves(order:readonly string[],ids:readonly string[],operation:string):{id:string;index:number}[]|null {
  const selected=new Set(ids),unique=new Set(order);
  if(unique.size!==order.length || ids.some(id=>!unique.has(id)) || !["bringForward","sendBackward","bringToFront","sendToBack"].includes(operation)) return null;
  const working=[...order],moves:{id:string;index:number}[]=[];
  const step=(index:number,to:number)=>{const id=working[index]!;[working[index],working[to]]=[working[to]!,id];moves.push({id,index:to});};
  if(operation==="bringForward") {
    for(let i=working.length-2;i>=0;i--) if(selected.has(working[i]!)&&!selected.has(working[i+1]!)) step(i,i+1);
  } else if(operation==="sendBackward") {
    for(let i=1;i<working.length;i++) if(selected.has(working[i]!)&&!selected.has(working[i-1]!)) step(i,i-1);
  } else {
    const chosen=order.filter(id=>selected.has(id)),others=order.filter(id=>!selected.has(id));
    const desired=operation==="bringToFront"?[...others,...chosen]:[...chosen,...others];
    if(order.every((id,i)=>id===desired[i])) return [];
    const indices=order.map((_,i)=>i).filter(i=>selected.has(order[i]!));
    if(operation==="sendToBack") indices.reverse();
    for(const original of indices) {
      const index=original+(operation==="bringToFront"?-moves.length:moves.length),to=operation==="bringToFront"?order.length-1:0;
      if(index!==to) moves.push({id:order[original]!,index:to});
    }
  }
  return moves;
}
