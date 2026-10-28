import equal from 'fast-deep-equal/es6/index.js';
import {mat2d,glMatrix} from 'gl-matrix';
glMatrix.setMatrixArrayType(Array);
const witness=v=>Array.isArray(v)?Array.from(v,witness):v&&typeof v==='object'?Object.entries(v).sort(([a],[b])=>a<b?-1:a>b?1:0).map(([k,x])=>[k,witness(x)]):v;
export const vectorSourceOracle={equal:(a,b)=>a===undefined||b===undefined?a===b:equal(witness(a),witness(b)),affine(t){const r=mat2d.create();mat2d.translate(r,r,[t.x,t.y]);mat2d.rotate(r,r,t.rotation);mat2d.multiply(r,r,[t.scaleX,0,t.shear,t.scaleY,0,0]);return Array.from(r)}};
