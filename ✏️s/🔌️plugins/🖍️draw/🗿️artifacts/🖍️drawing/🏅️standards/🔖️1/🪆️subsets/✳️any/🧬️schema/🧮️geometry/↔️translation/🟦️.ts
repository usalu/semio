/** ↔️ Convert a world displacement into a layer's parent coordinates. */
import {inverse} from "../🟦️.ts";
export function translate(transform:readonly number[],parent:readonly number[],delta:readonly number[]):number[]|null {
  if(transform.length!==5 || parent.length!==6 || delta.length!==2 || ![...transform,...delta].every(Number.isFinite)) return null;
  const matrix=inverse(parent as [number,number,number,number,number,number]);
  if(!matrix) return null;
  const result=[...transform];
  result[0]!+=matrix[0]*delta[0]!+matrix[2]*delta[1]!;
  result[1]!+=matrix[1]*delta[0]!+matrix[3]*delta[1]!;
  return result.every(Number.isFinite) ? result : null;
}
