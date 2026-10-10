/** ♻️ Mirrors the source, original node body, and final alias custody boundary. */
export class CancelReturnCursor {
 state:"source"|"body"|"empty"="source";
 constructor(readonly aliases:number,readonly nodes:number,readonly waiters:number,public emptyCapacity:number,readonly frameBytes:number,readonly waiterBytes:number){}
 returnedNodes=0;
 demands(){return this.state==="source"?{copy:48,release:this.frameBytes,depth:1}:this.state==="body"?this.waiters?null:{copy:this.returnedNodes<this.nodes-1?8:0,release:this.emptyCapacity*this.waiterBytes,depth:1}:{copy:0,release:0,depth:0};}
 advance(items:number,copy:number,release:number,depth:number):{release:number;retained:boolean}{
  const demand=this.demands();if(demand===null)return{release:0,retained:true};if(items===0||copy<demand.copy||release<demand.release||depth<demand.depth)return{release:0,retained:true};
  if(this.state==="source"){this.state=this.aliases>1?"empty":"body";return{release:this.aliases>1?0:this.frameBytes,retained:this.state!=="empty"};}
  if(this.state==="body"){if(this.emptyCapacity){const bytes=this.emptyCapacity*this.waiterBytes;this.emptyCapacity=0;return{release:bytes,retained:true};}this.returnedNodes++;this.state=this.returnedNodes===this.nodes?"empty":"source";}
  return{release:0,retained:this.state!=="empty"};
 }
}
