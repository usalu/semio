/** ⭕️ Sampling limits describe ideal circular chords in the SI profile plane before coordinate rounding. */
import {captureIfcIoControl,IfcStepError,type IfcIoControl} from '../control.ts';
export interface IfcCurveSampling {absoluteChordErrorMetres:number;relativeChordError:number;maxSegments:number}
export interface IfcBooleanEvaluationPolicy {maxWork:number;maxPolygons:number;maxDepth:number;maxIntermediateValues:number}
export interface IfcGeometryControl extends IfcIoControl {curveSampling?:Partial<IfcCurveSampling>;booleanEvaluation?:Partial<IfcBooleanEvaluationPolicy>}
/** 🧊️ Capture distinct whole-request Boolean ceilings before reading observer properties. */
function captureBooleanPolicy(source:Partial<IfcBooleanEvaluationPolicy>|undefined):IfcBooleanEvaluationPolicy{
 if(source!==undefined&&(source===null||typeof source!=='object'||Array.isArray(source)||Reflect.ownKeys(source).some(key=>typeof key!=='string'||!['maxWork','maxPolygons','maxDepth','maxIntermediateValues'].includes(key))))throw new IfcStepError('geometry.control','A closed Boolean evaluation policy is required');
 const{maxWork=2_000_000,maxPolygons=200_000,maxDepth=256,maxIntermediateValues=20_000_000}=source??{};
 if([maxWork,maxPolygons,maxDepth,maxIntermediateValues].some(value=>!Number.isSafeInteger(value)||value<1))throw new IfcStepError('geometry.control','Boolean limits require positive safe integers');
 return Object.freeze({maxWork,maxPolygons,maxDepth,maxIntermediateValues});
}
/** 🫳️ Copy the original curve policy before any caller callback can replace its fields. */
export function captureIfcGeometryControl(source:IfcGeometryControl):IfcGeometryControl&{curveSampling:IfcCurveSampling;booleanEvaluation:IfcBooleanEvaluationPolicy}{
 const booleanEvaluation=captureBooleanPolicy(source.booleanEvaluation),control=captureIfcIoControl(source),sampling=source.curveSampling;
 if(sampling!==undefined&&(sampling===null||typeof sampling!=='object'||Array.isArray(sampling)||Reflect.ownKeys(sampling).some(key=>typeof key!=='string'||!['absoluteChordErrorMetres','relativeChordError','maxSegments'].includes(key))))throw new IfcStepError('geometry.control','A declared curve sampling policy is required');
 const absolute=sampling?.absoluteChordErrorMetres,relative=sampling?.relativeChordError,maximum=sampling?.maxSegments,absoluteChordErrorMetres=absolute===undefined?0:absolute,relativeChordError=relative===undefined?.001:relative,maxSegments=maximum===undefined?4096:maximum;
 if(!Number.isFinite(absoluteChordErrorMetres)||absoluteChordErrorMetres<0||!Number.isFinite(relativeChordError)||relativeChordError<0||relativeChordError>1||absoluteChordErrorMetres===0&&relativeChordError===0||!Number.isSafeInteger(maxSegments)||maxSegments<4||maxSegments>65536)throw new IfcStepError('geometry.control','Finite nonnegative chord errors, one positive tolerance and 4..65536 integral segments are required');
 return{...control,curveSampling:{absoluteChordErrorMetres,relativeChordError,maxSegments},booleanEvaluation};
}
/** 📏️ The first admissible multiple of four uses sin(x) <= x and an upward-rounded pi bound. */
export function ifcCircleSegments(radiusMetres:number,policy:IfcCurveSampling):number{
 const tolerance=Math.min(1,Math.max(policy.relativeChordError,policy.absoluteChordErrorMetres/radiusMetres)),ceiling=Math.floor(policy.maxSegments/4);
 const admitted=(quadrants:number)=>{const angle=3.1415926535897936/(4*quadrants);return angle*angle*.5*(1+8*Number.EPSILON)<=tolerance;};
 if(!admitted(ceiling))throw new IfcStepError('geometry.accuracy','The original segment grant cannot certify the requested circle chord error');
 let first=1,last=ceiling;while(first<last){const middle=Math.floor((first+last)/2);if(admitted(middle))last=middle;else first=middle+1;}return 4*first;
}
