/** 🎛️ Cheap selection eligibility for existing semantic inspector commands. */
export interface DrawingSelectionActionEligibility {
 count:number;
 complete:boolean;
 arrangementCount:number;
 arrangeable:boolean;
 locked:boolean;
 sameParent:boolean;
 convertibleShapes:boolean;
 ungroupableGroups:boolean;
}

/** 🧭️ Offers only commands supported by the selection's known structural preconditions. */
export function drawingSelectionActionAvailable(selection:DrawingSelectionActionEligibility,operation:string):boolean {
 if(selection.count===0||selection.locked||!selection.complete)return false;
 switch(operation){
  case "toPath":return selection.convertibleShapes;
  case "ungroup":return selection.ungroupableGroups;
  case "alignLeft":case "alignCenter":case "alignRight":case "alignTop":case "alignMiddle":case "alignBottom":return selection.arrangeable&&selection.arrangementCount>=2;
  case "distributeHorizontal":case "distributeVertical":return selection.arrangeable&&selection.arrangementCount>=3;
  case "group":case "duplicate":case "delete":case "bringForward":case "sendBackward":case "bringToFront":case "sendToBack":return selection.sameParent;
  default:return false;
 }
}
