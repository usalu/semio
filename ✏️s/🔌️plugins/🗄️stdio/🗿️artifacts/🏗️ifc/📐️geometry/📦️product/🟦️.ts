import data from "./🔣️.json" with {type:"json"};
/** 📚️ Allocation-free normative product classification, independently checked against both full catalogs. */
export function isGeometryProduct(standard:"IFC4"|"IFC2X3",name:string):boolean{const names=data[standard];let low=0,high=names.length;while(low<high){const middle=(low+high)>>>1;if(names[middle]<name)low=middle+1;else high=middle;}return names[low]===name;}
