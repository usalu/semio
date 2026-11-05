import { Box3, CylinderGeometry } from "three";

export interface ColumnDimensions { height:number; radius:number; sides:number }

export function columnBounds(input:ColumnDimensions):{height:number;radius:number} {
  const geometry = new CylinderGeometry(input.radius,input.radius,input.height,input.sides,1,false);
  try {
    geometry.rotateX(Math.PI/2);
    const positions = geometry.getAttribute("position");
    const box = new Box3().setFromBufferAttribute(positions);
    let radius = 0;
    for(let index=0;index<positions.count;index++) radius=Math.max(radius,Math.hypot(positions.getX(index),positions.getY(index)));
    return {height:box.max.z-box.min.z,radius};
  } finally { geometry.dispose(); }
}
