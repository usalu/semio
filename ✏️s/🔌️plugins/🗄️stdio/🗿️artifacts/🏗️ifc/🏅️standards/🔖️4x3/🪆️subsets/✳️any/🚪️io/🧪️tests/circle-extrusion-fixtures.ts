/** 🏗️ Circle sweeps use the same declared product, unit and placement roles as rectangular sweeps. */
import {ifcRectangleFixture,type IfcRectangleCase} from './rectangle-extrusion-fixtures.ts';
import {ifcMappingFixture} from './mapping-fixtures.ts';
import {ifcReal} from '../step.ts';
export interface IfcCircleCase extends Omit<IfcRectangleCase,'x'|'y'> {radius:number}
/** ⭕️ Replace only the fixture profile declaration while preserving genuine inherited argument roles. */
export function ifcCircleFixture(row:IfcCircleCase){
 const fixture=ifcRectangleFixture({...row,x:row.radius,y:row.radius}),profile=fixture.snapshot.entities.find(entity=>entity.id===(fixture.profile.kind==='reference'?fixture.profile.value:-1n))!;
 profile.name='IFCCIRCLEPROFILEDEF';profile.args=profile.args.slice(0,4);
 return fixture;
}
export interface IfcMappedCircleCase{scales:number[];tilt:number;solidTilt:number;profileRotation:number;reflected:boolean}
/** 🌀️ Shared mapped-circle fixtures retain actual profile and solid placement declarations. */
export function ifcMappedCircleFixture(row:IfcMappedCircleCase,bounds:number[][]){
 const c=Math.cos(row.tilt),s=Math.sin(row.tilt),axes=[[c,0,-s],[0,row.reflected?-1:1,0],[s,0,c]],fixture=ifcMappingFixture({layers:[{origin:[0,0,0],originRotation:0,target:[10,20,30],axes,scale:row.scales[0]!,secondary:row.scales.slice(1)}]},bounds,'circle');
 if(row.profileRotation){const profileRef=fixture.solid.args[0];if(profileRef?.kind!=='reference')throw Error('Authored circle profile reference');const profile=fixture.snapshot.entities.find(entity=>entity.id===profileRef.value)!,point=fixture.add('IFCCARTESIANPOINT',[{kind:'aggregate',value:[ifcReal(0),ifcReal(0)]}]),direction=fixture.add('IFCDIRECTION',[{kind:'aggregate',value:[ifcReal(Math.cos(row.profileRotation)),ifcReal(Math.sin(row.profileRotation))]}]);profile.args[2]=fixture.add('IFCAXIS2PLACEMENT2D',[point,direction]);}
 if(row.solidTilt){const point=fixture.add('IFCCARTESIANPOINT',[{kind:'aggregate',value:[0,0,0].map(ifcReal)}]),z=fixture.add('IFCDIRECTION',[{kind:'aggregate',value:[Math.sin(row.solidTilt),0,Math.cos(row.solidTilt)].map(ifcReal)}]),x=fixture.add('IFCDIRECTION',[{kind:'aggregate',value:[Math.cos(row.solidTilt),0,-Math.sin(row.solidTilt)].map(ifcReal)}]);fixture.solid.args[1]=fixture.add('IFCAXIS2PLACEMENT3D',[point,z,x]);}
 return{...fixture,axes};
}
