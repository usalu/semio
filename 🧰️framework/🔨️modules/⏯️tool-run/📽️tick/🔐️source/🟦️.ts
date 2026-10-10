/** 🔐️ Logical child captures keep the original Tick and prevent handback while borrowed. */
import type {ToolRunTick,ToolRunTracePage} from "../../🟦️.ts";
type Root={original:ToolRunTick,aliases:number};
class TickCapture<T>{
 private root:Root|undefined;
 constructor(root:Root,private projection:(original:ToolRunTick)=>T){this.root=root;root.aliases++;}
 get borrowed(){if(!this.root)throw Error("Original Tick child has returned");return this.projection(this.root.original);}
 close(items:number){if(items<=0||!this.root)return 0;this.root.aliases--;this.root=undefined;return 1;}
}
export class ToolRunTickSource{
 private root:Root|undefined;
 constructor(original:ToolRunTick){this.root={original,aliases:1};}
 get borrowed(){if(!this.root)throw Error("Original Tick has transferred");return this.root.original;}
 operation(ordinal:number,items:number):TickCapture<Uint8Array>|undefined{if(items<=0||!this.root||!Number.isInteger(ordinal)||ordinal<0||ordinal>=this.root.original.appendOps.length)return undefined;return new TickCapture(this.root,tick=>tick.appendOps[ordinal]!);}
 trace(ordinal:number,items:number):TickCapture<ToolRunTracePage>|undefined{if(items<=0||!this.root||!Number.isInteger(ordinal)||ordinal<0||ordinal>=this.root.original.trace.length)return undefined;return new TickCapture(this.root,tick=>tick.trace[ordinal]!);}
 take(items:number){if(items<=0||!this.root||this.root.aliases!==1)return undefined;const original=this.root.original;this.root.aliases=0;this.root=undefined;return original;}
 close(items:number){if(items<=0||!this.root)return 0;this.root.aliases--;this.root=undefined;return 1;}
}
