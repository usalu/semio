import type {IfcValue} from '../../../🧬️schema/📸️snapshot/🟦️.ts';
import {ifcNumber} from '../../step.ts';
import {ifcCheckpoint,IfcStepError,type IfcIoControl} from '../../control.ts';

/** 🏋️ Project exact released ratios into the existing upright FEM case flag. */
export async function ifcFemSelfWeight(value:IfcValue|undefined,control:IfcIoControl):Promise<boolean> {
  await ifcCheckpoint(control,'fem-self-weight',0,3);
  if(value?.kind==='unset')return false;
  const refused=()=>new IfcStepError('fem.self-weight','Self-weight requires unset, three zero REAL ratios or exact global [0,0,-1]');
  if(value?.kind!=='aggregate'||value.value.length!==3)throw refused();
  let downward=true,zero=true;
  for(let i=0;i<3;i++){
    await ifcCheckpoint(control,'fem-self-weight',i+1,3);
    const scalar=value.value[i];
    if(scalar?.kind!=='real')throw refused();
    const number=ifcNumber(scalar);zero&&=number===0;downward&&=number===(i===2?-1:0);
  }
  if(!zero&&!downward)throw refused();
  return downward;
}
