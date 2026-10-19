/** 🪟️ Captures one immutable original presentation body without reconstructing any field. */
import type { ToolRunIdentity } from "../../🟦️.ts";
import type { ToolRunState } from "../../🚦️lifecycle/🟦️.ts";
import type {ToolRunPresentationFields} from "./🧬️update/🟦️.ts";
import {ToolRunPresentation} from "./📸️capture/🟦️.ts";
export {ToolRunPresentation};
/** 👁️ Borrows the exact presentation leaves while retaining its captured scalar envelope. */
export class ToolRunCapturedView {
 constructor(private source:ToolRunPresentation<ToolRunPresentationFields>,private identity:ToolRunIdentity,private state:ToolRunState){}
 get original(){return this.source.original;}
 get view(){const body=this.original;return {toolId:body.toolId,identity:this.identity,state:this.state,provisionalEntities:body.provenance.entities,progress:body.progress,payload:body.payload};}
 close(items:number){return this.source.close(items);}
}
