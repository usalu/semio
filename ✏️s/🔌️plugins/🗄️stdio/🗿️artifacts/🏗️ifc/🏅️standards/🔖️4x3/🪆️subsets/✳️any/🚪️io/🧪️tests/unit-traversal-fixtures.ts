/** 🏗️ Independent unit oracle populations and controlled local-placement chains. */
import type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
const unset:IfcValue={kind:'unset'},e=(value:string):IfcValue=>({kind:'enum',value}),a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),i=(value:number):IfcValue=>({kind:'integer',value:BigInt(value)});
/** 📐️ Construct actual unit roles without depending on implementation inference. */
export function ifcUnitTraversalFixture(){
 const builder=createIfc4x3ExchangeGraph('Unit traversal'),{add,snapshot}=builder;
 const si=(type:string,name:string,prefix:IfcValue=unset)=>add('IFCSIUNIT',[{kind:'derived'},e(type),prefix,e(name)]);
 const metre=si('LENGTHUNIT','METRE'),millimetre=si('LENGTHUNIT','METRE',e('MILLI')),kilogram=si('MASSUNIT','GRAM',e('KILO')),second=si('TIMEUNIT','SECOND');
 const dimensions=add('IFCDIMENSIONALEXPONENTS',[i(1),i(0),i(0),i(0),i(0),i(0),i(0)]);
 const conversion=(name:string,factor:number,base:IfcValue)=>add('IFCCONVERSIONBASEDUNIT',[dimensions,e('LENGTHUNIT'),{kind:'string',value:name},add('IFCMEASUREWITHUNIT',[{kind:'typedValue',value:{name:'IFCLENGTHMEASURE',items:[ifcReal(factor)]}},base])]);
 const foot=conversion('foot',.3048,metre),inch=conversion('inch',1/12,foot);
 const derived=(type:string,items:[IfcValue,number][])=>add('IFCDERIVEDUNIT',[a(items.map(([unit,power])=>add('IFCDERIVEDUNITELEMENT',[unit,i(power)]))),e(type),unset,unset]);
 const density=derived('MASSDENSITYUNIT',[[kilogram,1],[millimetre,-3]]),pressure=derived('MODULUSOFELASTICITYUNIT',[[kilogram,1],[metre,-1],[second,-2]]);
 const units:Record<string,IfcValue>={metre,millimetre,foot,'inch-via-foot':inch,'density-kg-per-mm3':density,'pressure-kg-per-m-s2':pressure};
 const assignment=snapshot.entities.find(row=>row.name==='IFCUNITASSIGNMENT')!;assignment.args[0]=a([metre]);
 const point=add('IFCCARTESIANPOINT',[a([ifcReal(1),ifcReal(0),ifcReal(0)])]),axis=add('IFCAXIS2PLACEMENT3D',[point,unset,unset]);
 let placement:IfcValue=unset;for(let k=0;k<24;k++)placement=add('IFCLOCALPLACEMENT',[placement,axis]);
 return{...builder,units,assignment,dimensions,placement,derived,conversion,e,a,i,unset};
}
