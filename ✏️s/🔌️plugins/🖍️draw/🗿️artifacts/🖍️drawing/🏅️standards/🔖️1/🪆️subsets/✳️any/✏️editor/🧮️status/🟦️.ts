/** 🗣️ Status labels come from the addressed view's explicit locale and terminology. */
export interface DrawingStatusLabels {
 readonly layerOne:string;
 readonly layerMany:string;
 readonly selected:string;
}

/** 🧮️ Top-level layer and framework selection counts describe the current window without stored UI state. */
export function drawingSelectionStatus(layerCount:number,selectedCount:number,labels:DrawingStatusLabels):string {
 if(![layerCount,selectedCount].every(count=>Number.isSafeInteger(count)&&count>=0))throw new RangeError("Invalid Draw status count");
 return (layerCount===1?labels.layerOne:labels.layerMany).replaceAll("{count}",String(layerCount))+" · "+labels.selected.replaceAll("{count}",String(selectedCount));
}
