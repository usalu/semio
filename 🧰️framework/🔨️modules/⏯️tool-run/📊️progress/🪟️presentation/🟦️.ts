/** 🪟️ Captures one immutable original presentation body without reconstructing any field. */
import type {ToolRunIdentity,ToolRunState} from "../../🟦️.ts";
import type {ToolRunPresentationFields} from "./🧬️update/🟦️.ts";
export class ToolRunPresentation<T> {
 private value:T|undefined;
 constructor(original:T){this.value=original;}
 get original(){if(!this.retained)throw Error("Original presentation has already returned");return this.value!;}
 get retained(){return this.value!==undefined;}
 capture(items:number){if(items<=0||!this.retained)return undefined;return new ToolRunPresentation(this.original);}
 close(items:number){if(items<=0||!this.retained)return 0;this.value=undefined;return 1;}
}
/** 👁️ Borrows the exact presentation leaves while retaining its captured scalar envelope. */
export class ToolRunCapturedView {
 constructor(private source:ToolRunPresentation<ToolRunPresentationFields>,private identity:ToolRunIdentity,private state:ToolRunState){}
 get original(){return this.source.original;}
 get view(){const body=this.original;return {toolId:body.toolId,identity:this.identity,state:this.state,provisionalEntities:body.provenance.entities,progress:body.progress,payload:body.payload};}
 close(items:number){return this.source.close(items);}
}
