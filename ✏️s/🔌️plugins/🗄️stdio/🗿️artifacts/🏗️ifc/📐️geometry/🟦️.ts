import {isGeometryProduct} from "./📦️product/🟦️.ts";
export {LengthUnitWork,DEFAULT_LENGTH_UNIT_LIMITS} from "./📏️units/🟦️.ts";
export type {LengthUnitResult,LengthUnitLimits,LengthUnitControl} from "./📏️units/🟦️.ts";
import {blockPoint,rectanglePoint,BLOCK_TRIANGLES} from "./🧱️primitive/🟦️.ts";
import {add,subtract,scale,dot,cross,normalize,identity,matrix,multiply,transformPoint,determinant,type Matrix} from "./🧮️matrix/🟦️.ts";
import {subtractHalfSpace,booleanSolid,subtractSolid,type SolidBooleanOperation,type SolidMesh,type SolidPlane} from "./🧊️boolean/🟦️.ts";
export {subtractHalfSpace,booleanSolid,subtractSolid};
export type {SolidMesh,SolidPoint,SolidTriangle,SolidBooleanOptions,SolidBooleanOperation,SolidPlane} from "./🧊️boolean/🟦️.ts";
import {triangulateRings} from "./🕳️rings/🟦️.ts";
import type {GeometryGraph, GeometryEntity} from "./📖️graph/🟦️.ts";

export type Point = [number, number, number];
export type Triangle = [number, number, number];
export const CURVE_RELATIVE_TOLERANCE = 0.001;
export const CURVE_SEGMENTS = 72;

export interface GeometryMesh {product: bigint; representation: bigint; label: string; vertices: Point[]; triangles: Triangle[]}
export interface GeometryIssue {entity: bigint; reason: string}
export type GeometryStatus = "complete" | "cancelled" | "limited";
export type GeometryControl = "continue" | "cancel" | "limit";
export interface GeometryScene {meshes: GeometryMesh[]; issues: GeometryIssue[]; status: GeometryStatus; work: number}
export interface GeometryLimits {work: number; vertices: number; triangles: number; depth: number}
export interface GeometryProgress {work: number; budget: number}
export const DEFAULT_GEOMETRY_LIMITS: Readonly<GeometryLimits> = Object.freeze({work: 8_000_000, vertices: 200_000, triangles: 400_000, depth: 64});
export interface GeometryPayloads {cameraPosition: Point; cameraTarget: Point; meshesJson: string; instancesJson: string; statusJson: string}

const fail = (reason: string): never => {throw new Error(reason);};
const cross2 = (a: number[], b: number[], c: number[]): number => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
function createProjector<V>(graph:GeometryGraph<V>,limits:GeometryLimits,progress:(value:GeometryProgress)=>GeometryControl){
const values=graph.values;
const ref=(entity:GeometryEntity<V>,index:number):bigint|undefined=>values.reference(entity.args[index]);
const optionalRef=(entity:GeometryEntity<V>,index:number):bigint|undefined=>{const value=entity.args[index];if(value===undefined||values.unset(value))return undefined;return values.reference(value)??fail("invalid-optional-reference");};
const aggregate=(value:V|undefined):readonly V[]=>values.aggregate(value)??fail("missing-aggregate");
const text=(value:V|undefined):string|undefined=>values.text(value);
const real=(value:V|undefined):number=>{const result=values.real(value);return result!==undefined&&Number.isFinite(result)?result:fail("missing-finite-number");};
const optionalReal=(entity:GeometryEntity<V>,index:number,fallback:number):number=>entity.args[index]===undefined||values.unset(entity.args[index])?fallback:real(entity.args[index]);
const coordinates=(value:V):Point=>{const list=aggregate(value);if(list.length<2||list.length>3)fail("coordinate-dimension");return [real(list[0]),real(list[1]),list.length===3?real(list[2]):0];};
const indexValue=(value:V):number=>{const index=values.integer(value);if(index===undefined||index<=0n||index>BigInt(Number.MAX_SAFE_INTEGER))return fail("invalid-one-based-index");return Number(index-1n);};
function productHint(entity:GeometryEntity<V>):boolean{return isGeometryProduct(graph.standard,entity.name);}

class Projector {
  readonly scene: GeometryScene = {meshes: [], issues: [], status: "complete", work: 0};
  readonly entities = new Map<bigint, GeometryEntity<V>>();
  readonly vertexIndices = new Map<string, number>();
  readonly openings=new Map<bigint,bigint[]>();
  vertices = 0;
  triangles = 0;
  constructor(readonly limits: GeometryLimits, readonly progress: (value: GeometryProgress) => GeometryControl) {}
  tick(): void {
    if (this.scene.work >= this.limits.work) {this.scene.status = "limited"; fail("work-limit");}
    this.scene.work++;
    const control = this.progress({work: this.scene.work, budget: this.limits.work});
    if (control === "cancel") {this.scene.status = "cancelled"; fail("cancelled");}
    if (control === "limit") {this.scene.status = "limited"; fail("caller-work-limit");}
  }
  entity(id: bigint): GeometryEntity<V> {this.tick();const entity=this.entities.get(id)??fail("missing-reference-#"+id);if(entity.constituentCount!==1)fail("unsupported-complex-representation-#"+id);return entity;}
  references(entity: GeometryEntity<V>, index: number): bigint[] {
    const ids: bigint[] = [];
    for (const item of aggregate(entity.args[index])) {this.tick(); ids.push(values.reference(item)??fail("invalid-reference-list"));}
    return ids;
  }
  point(id: bigint, direction = false): Point {
    const entity = this.entity(id);
    if (entity.name !== (direction ? "IFCDIRECTION" : "IFCCARTESIANPOINT")) fail("invalid-point-#" + id);
    return coordinates(entity.args[0]);
  }
  requireDimension(id: bigint, dimension: number): void {
    if (aggregate(this.entity(id).args[0]).length !== dimension) fail("invalid-"+dimension+"d-coordinate-dimension");
  }
  axis(id: bigint): Matrix {
    const entity = this.entity(id), origin = this.point(ref(entity, 0) ?? fail("missing-location"));
    const a = optionalRef(entity, 1), b = optionalRef(entity, 2);
    if (entity.name === "IFCAXIS2PLACEMENT3D") {
      this.requireDimension(ref(entity,0)!,3);
      if (a!==undefined) this.requireDimension(a,3);
      if (b!==undefined) this.requireDimension(b,3);
      if ((a !== undefined) !== (b !== undefined)) fail("axis-and-ref-direction-provision");
      const z = normalize(a === undefined ? [0, 0, 1] : this.point(a, true));
      const hint: Point = b !== undefined ? this.point(b, true) : Math.abs(z[0]) < 0.9 ? [1, 0, 0] : [0, 1, 0];
      const x = normalize(subtract(hint, scale(z, dot(hint, z))));
      return matrix(origin, x, cross(z, x), z);
    }
    if (entity.name === "IFCAXIS2PLACEMENT2D") {
      if (aggregate(this.entity(ref(entity, 0)!).args[0]).length !== 2 || a !== undefined && aggregate(this.entity(a).args[0]).length !== 2) fail("invalid-2d-coordinate-dimension");
      const x = normalize(a === undefined ? [1, 0, 0] : this.point(a, true));
      if (Math.abs(x[2]) > 1e-9) fail("invalid-2d-axis");
      return matrix(origin, x, [-x[1], x[0], 0], [0, 0, 1]);
    }
    return fail("unsupported-placement-" + entity.name);
  }
  placement(id: bigint, path = new Set<bigint>()): Matrix {
    if (path.size >= this.limits.depth || path.has(id)) fail("placement-cycle-or-depth");
    path.add(id);
    const entity = this.entity(id);
    if (entity.name !== "IFCLOCALPLACEMENT") fail("unsupported-placement-" + entity.name);
    const parent = ref(entity, 0), local = this.axis(ref(entity, 1) ?? fail("missing-relative-placement"));
    const result = parent === undefined ? local : multiply(this.placement(parent, path), local);
    path.delete(id);
    return result;
  }
  context(id: bigint, depth = 0): Matrix {
    if (depth >= this.limits.depth) fail("context-cycle-or-depth");
    const entity = this.entity(id);
    if (entity.name === "IFCGEOMETRICREPRESENTATIONCONTEXT") return this.axis(ref(entity, 4) ?? fail("missing-context-placement"));
    if (entity.name === "IFCGEOMETRICREPRESENTATIONSUBCONTEXT") return this.context(ref(entity, 6) ?? fail("missing-parent-context"), depth + 1);
    return fail("unsupported-representation-context");
  }
  productMeshes(product: GeometryEntity<V>, shape: GeometryEntity<V>): GeometryMesh[] {
    if(shape.constituentCount!==1)fail("unsupported-complex-representation-#"+shape.id);
    const p = ref(product, 5), placement = p === undefined ? identity() : this.placement(p);
    const representations = this.references(shape, 2);
    const hasBody = representations.some(id => text(this.entities.get(id)?.args[1])?.toLowerCase() === "body");
    const meshes: GeometryMesh[] = [];
    for (const id of representations) {
      const representation = this.entity(id);
      if (!["IFCSHAPEREPRESENTATION", "IFCSHAPEMODEL"].includes(representation.name)) fail("unsupported-product-representation-" + representation.name + "-#" + id);
      const identifier = text(representation.args[1]);
      if ((identifier !== undefined && identifier.toLowerCase() !== "body") || hasBody && identifier === undefined) {this.scene.issues.push({entity: id, reason: "non-body-representation-skipped"}); continue;}
      const items = this.references(representation, 3), context = ref(representation, 0);
      const transform = context === undefined ? placement : multiply(this.context(context), placement);
      let label = "", characters = 0;
      for (const character of text(product.args[2]) ?? product.name) {if (characters++ === 256) break; label += character;}
      const mesh: GeometryMesh = {product: product.id, representation: id, label, vertices: [], triangles: []};
      this.vertexIndices.clear();
      for (const item of items) this.item(item, transform, mesh, new Set());
      if (mesh.triangles.length) meshes.push(mesh);
    }
    if (!meshes.length) fail("no-supported-body-geometry");
    return meshes;
  }
  product(product:GeometryEntity<V>,shape:GeometryEntity<V>):void{
    let meshes=this.productMeshes(product,shape);
    for(const id of this.openings.get(product.id)??[]){
      this.tick();const opening=this.entity(id),representation=ref(opening,6),openingShape=representation===undefined?undefined:this.entities.get(representation);
      if(!openingShape||!["IFCPRODUCTDEFINITIONSHAPE","IFCPRODUCTREPRESENTATION"].includes(openingShape.name))fail("missing-opening-representation-#"+id);
      let cutters:GeometryMesh[];try{cutters=this.productMeshes(opening,openingShape!);}catch(error){return fail("opening-#"+id+"-"+(error instanceof Error?error.message:String(error)));}
      const next:GeometryMesh[]=[];
      for(const mesh of meshes){let result={vertices:mesh.vertices,triangles:mesh.triangles};for(const cutter of cutters){this.tick();result=this.solidOperation("difference",result,cutter);if(this.vertices+result.vertices.length>this.limits.vertices||this.triangles+result.triangles.length>this.limits.triangles){this.scene.status="limited";fail("boolean-output-limit");}this.vertices+=result.vertices.length;this.triangles+=result.triangles.length;}if(result.triangles.length)next.push({...mesh,...result});}meshes=next;
    }
    for(const mesh of meshes)this.scene.meshes.push(mesh);
  }
  vertex(mesh: GeometryMesh, point: Point, transform: Matrix): number {
    this.tick();
    point = transformPoint(transform, point);
    if (!point.every(Number.isFinite)) fail("nonfinite-vertex");
    const key = point.join(","), previous = this.vertexIndices.get(key);
    if (previous !== undefined) return previous;
    if (this.vertices >= this.limits.vertices || mesh.vertices.length >= 0xffffffff) {this.scene.status = "limited"; fail("vertex-limit");}
    this.vertices++;
    const index = mesh.vertices.length;
    mesh.vertices.push(point); this.vertexIndices.set(key, index);
    return index;
  }
  triangle(mesh: GeometryMesh, indices: Triangle): void {
    this.tick();
    if (this.triangles >= this.limits.triangles) {this.scene.status = "limited"; fail("triangle-limit");}
    if (indices.some(index => index < 0 || index >= mesh.vertices.length)) fail("invalid-triangle-index");
    this.triangles++; mesh.triangles.push(indices);
  }
  polygon(mesh: GeometryMesh, indices: number[]): void {this.polygonRings(mesh, [indices]);}
  polygonRings(mesh: GeometryMesh, rings: number[][]): void {
    const triangles = triangulateRings(mesh.vertices, rings, () => this.tick());
    for (const triangle of triangles) this.triangle(mesh, triangle);
  }
  item(id: bigint, transform: Matrix, mesh: GeometryMesh, path: Set<bigint>): void {
    if (path.size >= this.limits.depth || path.has(id)) fail("representation-cycle-or-depth");
    path.add(id);
    const entity = this.entity(id), firstTriangle = mesh.triangles.length;
    switch (entity.name) {
      case "IFCTRIANGULATEDFACESET": case "IFCPOLYGONALFACESET": if(graph.standard!=="IFC4")fail("representation-requires-ifc4");this.faceSet(entity, transform, mesh); break;
      case "IFCEXTRUDEDAREASOLID": this.extrusion(entity, transform, mesh); break;
      case "IFCBOOLEANRESULT": this.booleanResult(entity,transform,mesh,path);break;
      case "IFCBOOLEANCLIPPINGRESULT":this.clippingResult(entity,transform,mesh,path);break;
      case "IFCBLOCK": this.block(entity,transform,mesh);break;
      case "IFCCSGSOLID": {if(entity.args.length!==1)fail("invalid-csg-argument-count");const root=this.entity(ref(entity,0)??fail("missing-csg-root"));if(!["IFCBOOLEANRESULT","IFCBOOLEANCLIPPINGRESULT","IFCBLOCK"].includes(root.name))fail("unsupported-csg-root-"+root.name);this.item(root.id,transform,mesh,path);break;}
      case "IFCFACETEDBREP": this.item(ref(entity, 0) ?? fail("missing-shell"), transform, mesh, path); break;
      case "IFCCLOSEDSHELL": case "IFCOPENSHELL": case "IFCCONNECTEDFACESET": case "IFCSHELLBASEDSURFACEMODEL": case "IFCFACEBASEDSURFACEMODEL":
        for (const child of this.references(entity, 0)) this.item(child, transform, mesh, path); break;
      case "IFCFACE": this.face(entity, transform, mesh); break;
      case "IFCMAPPEDITEM": {
        const source = this.entity(ref(entity, 0) ?? fail("missing-mapping-source")), target = ref(entity, 1) ?? fail("missing-mapping-target");
        if (source.name !== "IFCREPRESENTATIONMAP") fail("invalid-representation-map");
        const origin = this.axis(ref(source, 0) ?? fail("missing-mapping-origin")), representation = ref(source, 1) ?? fail("missing-mapped-representation");
        this.item(representation, multiply(multiply(transform, this.operator(target)), origin), mesh, path); break;
      }
      case "IFCSHAPEREPRESENTATION": for (const item of this.references(entity, 3)) this.item(item, transform, mesh, path); break;
      default: fail("unsupported-representation-" + entity.name + "-#" + id);
    }
    if (["IFCTRIANGULATEDFACESET", "IFCPOLYGONALFACESET", "IFCFACE", "IFCEXTRUDEDAREASOLID", "IFCBLOCK"].includes(entity.name) && determinant(transform) < 0) for (let i = firstTriangle; i < mesh.triangles.length; i++) {const triangle = mesh.triangles[i]; [triangle[1], triangle[2]] = [triangle[2], triangle[1]];}
    path.delete(id);
  }
  appendSolid(mesh:GeometryMesh,result:SolidMesh):void{this.vertexIndices.clear();for(let i=0;i<mesh.vertices.length;i++){this.tick();this.vertexIndices.set(mesh.vertices[i].join(","),i);}const remap=result.vertices.map(p=>this.vertex(mesh,p,identity()));for(const triangle of result.triangles)this.triangle(mesh,triangle.map(i=>remap[i]) as Triangle);}
  clippingResult(entity:GeometryEntity<V>,transform:Matrix,mesh:GeometryMesh,path:Set<bigint>):void{
    if(entity.args.length!==3||values.enumeration(entity.args[0])!=="DIFFERENCE")fail("clipping-requires-difference");const first=this.entity(ref(entity,1)??fail("missing-clipping-first-operand"));if(!["IFCEXTRUDEDAREASOLID","IFCBOOLEANCLIPPINGRESULT"].includes(first.name))fail("unsupported-clipping-first-operand-"+first.name);const half=this.entity(ref(entity,2)??fail("missing-clipping-halfspace"));if(half.name!=="IFCHALFSPACESOLID")fail("unsupported-halfspace-"+half.name);if(half.args.length!==2)fail("invalid-halfspace-argument-count");const agreement=values.enumeration(half.args[1]);if(agreement!=="T"&&agreement!=="F")fail("invalid-halfspace-agreement");const surface=this.entity(ref(half,0)??fail("missing-halfspace-surface"));if(surface.name!=="IFCPLANE"||surface.args.length!==1)fail("unsupported-halfspace-surface-"+surface.name);const position=this.entity(ref(surface,0)??fail("missing-plane-position"));if(position.name!=="IFCAXIS2PLACEMENT3D")fail("plane-placement-must-be-3d");const world=multiply(transform,this.axis(position.id)),columns=[0,1,2].map(j=>{const p:Point=[world[0][j],world[1][j],world[2][j]],extent=Math.max(...p.map(Math.abs));if(!(extent>0)||!Number.isFinite(extent))fail("invalid-plane-transform");return p.map(v=>v/extent) as Point;});let normal=normalize(cross(columns[0],columns[1]));const side=dot(normal,columns[2]);if(!Number.isFinite(side)||side===0)fail("singular-plane-transform");if((side<0)!==(agreement==="T"))normal=scale(normal,-1);const plane:SolidPlane={origin:transformPoint(world,[0,0,0]),normal},temporary:GeometryMesh={product:mesh.product,representation:mesh.representation,label:mesh.label,vertices:[],triangles:[]};this.vertexIndices.clear();this.item(first.id,transform,temporary,path);let result:SolidMesh;try{result=subtractHalfSpace(temporary,plane,{maxWork:Math.max(1,this.limits.work-this.scene.work),maxPolygons:Math.max(1,Math.min(200000,this.limits.triangles)),tick:()=>this.tick()});}catch(error){if(error instanceof Error&&/^boolean-(work|polygon|depth)-limit$/.test(error.message))this.scene.status="limited";throw error;}this.appendSolid(mesh,result);
  }
  solidOperation(operation:SolidBooleanOperation,left:SolidMesh,right:SolidMesh):SolidMesh{try{return booleanSolid(operation,left,right,{maxWork:Math.max(1,this.limits.work-this.scene.work),maxPolygons:Math.max(1,Math.min(200000,this.limits.triangles)),tick:()=>this.tick()});}catch(error){if(error instanceof Error&&/^boolean-(work|polygon|depth)-limit$/.test(error.message))this.scene.status="limited";throw error;}}
  block(entity:GeometryEntity<V>,transform:Matrix,mesh:GeometryMesh):void{
    if(entity.args.length!==4)fail("invalid-block-argument-count");const position=this.entity(ref(entity,0)??fail("missing-block-position"));if(position.name!=="IFCAXIS2PLACEMENT3D")fail("block-placement-must-be-3d");const[x,y,z]=[1,2,3].map(i=>real(entity.args[i]));if(Math.min(x,y,z)<=0)fail("invalid-block-dimensions");transform=multiply(transform,this.axis(position.id));const indices=Array.from({length:8},(_,i)=>this.vertex(mesh,blockPoint([x,y,z],i),transform));for(const triangle of BLOCK_TRIANGLES)this.triangle(mesh,triangle.map(i=>indices[i]) as Triangle);
  }
  booleanResult(entity:GeometryEntity<V>,transform:Matrix,mesh:GeometryMesh,path:Set<bigint>):void{
    if(entity.args.length!==3)fail("invalid-boolean-argument-count");const operation=values.enumeration(entity.args[0]);if(!["UNION","INTERSECTION","DIFFERENCE"].includes(operation??""))fail("unsupported-boolean-operation");
    const operands:GeometryMesh[]=[];for(const index of [1,2]){const id=ref(entity,index)??fail("missing-boolean-operand"),operand=this.entity(id);if(!["IFCBOOLEANRESULT","IFCBOOLEANCLIPPINGRESULT","IFCCSGSOLID","IFCBLOCK","IFCEXTRUDEDAREASOLID","IFCFACETEDBREP","IFCTRIANGULATEDFACESET","IFCPOLYGONALFACESET"].includes(operand.name))fail("unsupported-boolean-operand-"+operand.name);if(operand.name==="IFCFACETEDBREP"&&this.entity(ref(operand,0)??fail("missing-shell")).name!=="IFCCLOSEDSHELL")fail("boolean-brep-requires-closed-shell");if(["IFCTRIANGULATEDFACESET","IFCPOLYGONALFACESET"].includes(operand.name)&&(graph.standard!=="IFC4"||values.enumeration(operand.args[operand.name==="IFCTRIANGULATEDFACESET"?2:1])!=="T"))fail("boolean-tessellation-must-be-closed");const temporary:GeometryMesh={product:mesh.product,representation:mesh.representation,label:mesh.label,vertices:[],triangles:[]};this.vertexIndices.clear();this.item(id,transform,temporary,path);operands.push(temporary);}
    const result=this.solidOperation(operation!.toLowerCase() as SolidBooleanOperation,operands[0],operands[1]);this.appendSolid(mesh,result);
  }
  operator(id: bigint): Matrix {
    const entity = this.entity(id);
    if (!["IFCCARTESIANTRANSFORMATIONOPERATOR3D", "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM"].includes(entity.name)) fail("unsupported-mapping-" + entity.name);
    const a = optionalRef(entity, 0), b = optionalRef(entity, 1), c = optionalRef(entity, 4), location = ref(entity, 2) ?? fail("missing-mapping-location");
    this.requireDimension(location,3);
    for(const id of [a,b,c]) if(id!==undefined) this.requireDimension(id,3);
    const s = optionalReal(entity, 3, 1), sy = entity.name.endsWith("NONUNIFORM") ? optionalReal(entity, 5, s) : s, sz = entity.name.endsWith("NONUNIFORM") ? optionalReal(entity, 6, s) : s;
    if (Math.min(s, sy, sz) <= 0) fail("invalid-mapping-scale");
    const z = normalize(c === undefined ? [0, 0, 1] : this.point(c, true)), hx: Point = a !== undefined ? this.point(a, true) : z[0] === 1 && z[1] === 0 && z[2] === 0 ? [0, 1, 0] : [1, 0, 0];
    const x = normalize(subtract(hx, scale(z, dot(hx, z)))), hy: Point = b !== undefined ? this.point(b, true) : [0, 1, 0];
    const y = normalize(subtract(subtract(hy, scale(z, dot(hy, z))), scale(x, dot(hy, x))));
    return matrix(this.point(location), scale(x, s), scale(y, sy), scale(z, sz));
  }
  faceSet(entity: GeometryEntity<V>, transform: Matrix, mesh: GeometryMesh): void {
    const triangulated = entity.name === "IFCTRIANGULATEDFACESET", coords = ref(entity, 0) ?? fail("missing-point-list");
    const faces = aggregate(entity.args[triangulated ? 3 : 2]), pnValue = entity.args[triangulated ? 4 : 3];
    const pointList = this.entity(coords);
    if (pointList.name !== "IFCCARTESIANPOINTLIST3D") fail("invalid-point-list");
    const points = aggregate(pointList.args[0]), vertices = new Map<number, number>();
    const pn = values.absent(pnValue) ? undefined : aggregate(pnValue);
    const vertex = (value: V): number => {
      this.tick(); let index = indexValue(value);
      if (pn) index = indexValue(pn[index] ?? fail("invalid-pn-index"));
      if (index >= points.length) fail("face-index-out-of-range");
      let result = vertices.get(index);
      if (result === undefined) {result = this.vertex(mesh, coordinates(points[index]), transform); vertices.set(index, result);}
      return result;
    };
    for (const face of faces) {
      this.tick();
      if (triangulated) {
        const indices = aggregate(face).map(vertex);
        if (indices.length !== 3) fail("triangle-needs-three-indices"); this.triangle(mesh, indices as Triangle);
      } else {
        const indexed = this.entity(values.reference(face)??fail("invalid-indexed-face"));
        if (!["IFCINDEXEDPOLYGONALFACE", "IFCINDEXEDPOLYGONALFACEWITHVOIDS"].includes(indexed.name)) fail("unsupported-indexed-face-" + indexed.name);
        const rings = [aggregate(indexed.args[0]).map(vertex)];
        if (indexed.name === "IFCINDEXEDPOLYGONALFACEWITHVOIDS") for (const hole of aggregate(indexed.args[1])) {this.tick(); rings.push(aggregate(hole).map(vertex));}
        this.polygonRings(mesh, rings);
      }
    }
  }
  face(entity: GeometryEntity<V>, transform: Matrix, mesh: GeometryMesh): void {
    const bounds = this.references(entity, 0), rings: number[][] = [];
    let outer = -1;
    for (const id of bounds) {
      const bound = this.entity(id);
      if (!["IFCFACEOUTERBOUND", "IFCFACEBOUND"].includes(bound.name)) fail("invalid-face-bound");
      if (bound.name === "IFCFACEOUTERBOUND") {if (outer >= 0) fail("multiple-face-outer-bounds"); outer = rings.length;}
      const curve = this.entity(ref(bound, 0) ?? fail("missing-face-loop"));
      if (curve.name !== "IFCPOLYLOOP") fail("unsupported-face-loop");
      const indices = this.references(curve, 0).map(id => this.vertex(mesh, this.point(id), transform));
      const orientation = bound.args[1];
      if (!["T", "F"].includes(values.enumeration(orientation)??"")) fail("invalid-bound-orientation");
      if (values.enumeration(orientation) === "F") indices.reverse();
      rings.push(indices);
    }
    if (!rings.length) fail("missing-face-bound");
    if (outer < 0) {
      let largest = -1;
      for (let i = 0; i < rings.length; i++) {
        this.tick(); const span: Point = [0, 0, 0];
        for (let axis = 0; axis < 3; axis++) {let low = Infinity, high = -Infinity; for (const id of rings[i]) {this.tick(); low = Math.min(low, mesh.vertices[id][axis]); high = Math.max(high, mesh.vertices[id][axis]);} span[axis] = high - low;}
        const size = Math.hypot(...span); if (size > largest) {largest = size; outer = i;}
      }
    }
    [rings[0], rings[outer]] = [rings[outer], rings[0]];
    this.polygonRings(mesh, rings);
  }
  ellipseProfile(a: number, b: number, position?: bigint): Point[] {
    if (!Number.isFinite(a) || !Number.isFinite(b) || a <= 0 || b <= 0) fail("invalid-ellipse-dimensions");
    if (position !== undefined && this.entity(position).name !== "IFCAXIS2PLACEMENT2D") fail("profile-placement-must-be-2d");
    const transform = position === undefined ? identity() : this.axis(position), points: Point[] = [];
    for (let i = 0; i < CURVE_SEGMENTS; i++) {
      this.tick();
      const angle = 2 * Math.PI * i / CURVE_SEGMENTS, point = transformPoint(transform, [a * Math.cos(angle), b * Math.sin(angle), 0]);
      if (Math.abs(point[2]) > 1e-9) fail("profile-must-be-planar-2d");
      points.push(point);
    }
    return points;
  }
  arcPoints(start: Point, middle: Point, end: Point): Point[] {
    const u = subtract(middle, start), v = subtract(end, start), extent = Math.max(Math.abs(u[0]), Math.abs(u[1]), Math.abs(v[0]), Math.abs(v[1]));
    if (!(extent > 0) || !Number.isFinite(extent)) return fail("invalid-profile-arc");
    const a = [u[0]/extent,u[1]/extent], b = [v[0]/extent,v[1]/extent], determinant = 2*(a[0]*b[1]-a[1]*b[0]);
    if (determinant === 0) return [middle,end];
    if (Math.abs(determinant) < 128*Number.EPSILON) return fail("ill-conditioned-profile-arc");
    const aa=a[0]*a[0]+a[1]*a[1],bb=b[0]*b[0]+b[1]*b[1],center=[(aa*b[1]-bb*a[1])/determinant,(a[0]*bb-b[0]*aa)/determinant],radius=Math.hypot(...center),orientation=Math.sign(determinant),tau=2*Math.PI;
    const begin=Math.atan2(-center[1],-center[0]),mid=Math.atan2(a[1]-center[1],a[0]-center[0]),last=Math.atan2(b[1]-center[1],b[0]-center[0]);
    const span=(from:number,to:number):number=>orientation>0?((to-from)%tau+tau)%tau:-(((from-to)%tau+tau)%tau);
    const points:Point[]=[];
    const sample=(from:number,to:number,exact:Point):void=>{
      const angle=span(from,to),count=Math.max(1,Math.ceil(Math.abs(angle)/(tau/CURVE_SEGMENTS)-1e-12));
      for(let i=1;i<=count;i++){this.tick();if(i===count)points.push(exact);else {const t=from+angle*i/count;points.push([start[0]+(center[0]+radius*Math.cos(t))*extent,start[1]+(center[1]+radius*Math.sin(t))*extent,0]);}}
    };
    sample(begin,mid,middle);sample(mid,last,end);return points;
  }
  indexedProfileCurve(curve: GeometryEntity<V>): Point[] {
    if(graph.standard!=="IFC4")return fail("unsupported-profile-curve-"+curve.name);
    const list=this.entity(ref(curve,0)??fail("missing-profile-point-list"));
    if(list.name!=="IFCCARTESIANPOINTLIST2D")return fail("profile-point-list-must-be-2d");
    const coordinates=aggregate(list.args[0]),points:Point[]=[];
    for(const row of coordinates){this.tick();const xy=aggregate(row);if(xy.length!==2)fail("invalid-2d-coordinate-dimension");points.push([real(xy[0]),real(xy[1]),0]);}
    const segments=curve.args[1],polygon:Point[]=[];
    if(values.unset(segments))for(const point of points){this.tick();polygon.push(point);}
    else {
      let first:number|undefined,last:number|undefined;
      for(const segment of aggregate(segments)){
        this.tick();const typed=values.typed(segment);if(!typed||typed.items.length!==1)fail("invalid-indexed-profile-segment");
        const record=typed!,indices=aggregate(record.items[0]).map(value=>{this.tick();const index=indexValue(value);if(index>=points.length)fail("profile-index-out-of-range");return index;});
        if(record.name!=="IFCLINEINDEX"&&record.name!=="IFCARCINDEX")fail("unsupported-indexed-profile-segment-"+record.name);
        if(record.name==="IFCARCINDEX"?indices.length!==3:indices.length<2)fail("invalid-indexed-profile-segment-size");
        if(first===undefined){first=indices[0];polygon.push(points[first]);}else if(indices[0]!==last)fail("disconnected-indexed-profile-segments");
        if(record.name==="IFCARCINDEX")polygon.push(...this.arcPoints(points[indices[0]],points[indices[1]],points[indices[2]]));
        else for(const index of indices.slice(1)){this.tick();polygon.push(points[index]);}
        last=indices[indices.length-1];
      }
      if(first===undefined||first!==last)fail("indexed-profile-must-close-by-index");
    }
    if(!polygon.length||!polygon[0].every((value,index)=>value===polygon[polygon.length-1][index]))fail("profile-must-be-closed");
    polygon.pop();return polygon;
  }
  profileCurve(id: bigint): Point[] {
    const curve = this.entity(id);
    if (curve.name === "IFCINDEXEDPOLYCURVE") return this.indexedProfileCurve(curve);
    if (curve.name === "IFCCIRCLE" || curve.name === "IFCELLIPSE") {
      const a = real(curve.args[1]), b = curve.name === "IFCCIRCLE" ? a : real(curve.args[2]);
      return this.ellipseProfile(a, b, ref(curve, 0) ?? fail("missing-profile-curve-placement"));
    }
    if (curve.name !== "IFCPOLYLINE") return fail("unsupported-profile-curve-" + curve.name);
    const polygon = this.references(curve, 0).map(id => this.point(id));
    if (!polygon.length || !polygon[0].every((value, index) => value === polygon[polygon.length - 1][index])) fail("profile-must-be-closed");
    polygon.pop();
    if (polygon.some(point => Math.abs(point[2]) > 1e-9)) fail("profile-must-be-planar-2d");
    return polygon;
  }
  roundedRectangle(entity:GeometryEntity<V>,position?:bigint):Point[]{
    if(entity.args.length!==6||values.enumeration(entity.args[0])!=="AREA")fail("invalid-rounded-rectangle-profile");const x=real(entity.args[3])/2,y=real(entity.args[4])/2,r=real(entity.args[5]);if(x<=0||y<=0||r<=0||r>x||r>y)fail("invalid-rounded-rectangle-radius-or-dimensions");if(position!==undefined&&this.entity(position).name!=="IFCAXIS2PLACEMENT2D")fail("profile-placement-must-be-2d");const transform=position===undefined?identity():this.axis(position),points:Point[]=[],centers:Point[]=[[x-r,-y+r,0],[x-r,y-r,0],[-x+r,y-r,0],[-x+r,-y+r,0]],cardinal:Point[]=[[0,-1,0],[1,0,0],[0,1,0],[-1,0,0],[0,-1,0]],segments=CURVE_SEGMENTS/4;for(let corner=0;corner<4;corner++)for(let i=0;i<=segments;i++){this.tick();const angle=(corner-1)*Math.PI/2+i*Math.PI/2/segments,d=i===0?cardinal[corner]:i===segments?cardinal[corner+1]:[Math.cos(angle),Math.sin(angle),0] as Point,point=transformPoint(transform,add(centers[corner],scale(d,r))),previous=points[points.length-1];if(!previous||point.some((v,j)=>v!==previous[j]))points.push(point);}if(points.length>1&&points[0].every((v,j)=>v===points[points.length-1][j]))points.pop();return points;
  }
  profile(id: bigint): Point[] {
    const entity = this.entity(id), position = optionalRef(entity, 2);
    if(entity.name==="IFCROUNDEDRECTANGLEPROFILEDEF")return this.roundedRectangle(entity,position);
    if (["IFCCIRCLEPROFILEDEF", "IFCELLIPSEPROFILEDEF"].includes(entity.name)) {
      if (values.enumeration(entity.args[0]) !== "AREA") fail("profile-must-be-area");
      const a = real(entity.args[3]), b = entity.name === "IFCCIRCLEPROFILEDEF" ? a : real(entity.args[4]);
      return this.ellipseProfile(a, b, position);
    }
    if (entity.name === "IFCRECTANGLEPROFILEDEF") {
      const x = real(entity.args[3]) / 2, y = real(entity.args[4]) / 2;
      if (x <= 0 || y <= 0) fail("invalid-rectangle-dimensions");
      const transform = position === undefined ? identity() : this.axis(position);
      return Array.from({length:4},(_,i)=>transformPoint(transform,rectanglePoint(x*2,y*2,i)));
    }
    if (entity.name === "IFCARBITRARYCLOSEDPROFILEDEF") {
      if (values.enumeration(entity.args[0]) !== "AREA") fail("profile-must-be-area");
      return this.profileCurve(position ?? fail("missing-profile-curve"));
    }
    return fail("unsupported-profile-" + entity.name);
  }
  profileRings(id: bigint): Point[][] {
    const entity = this.entity(id);
    if (values.enumeration(entity.args[0]) !== "AREA") fail("profile-must-be-area");
    if (entity.name === "IFCARBITRARYPROFILEDEFWITHVOIDS") {
      const rings = [this.profileCurve(ref(entity, 2) ?? fail("missing-profile-curve"))];
      for (const hole of this.references(entity, 3)) rings.push(this.profileCurve(hole));
      if (rings.length === 1) fail("voided-profile-needs-inner-curve");
      return rings;
    }
    if (entity.name === "IFCCIRCLEHOLLOWPROFILEDEF") {
      const radius = real(entity.args[3]), wall = real(entity.args[4]);
      if (wall <= 0 || wall >= radius) fail("invalid-hollow-circle-wall");
      const position = optionalRef(entity, 2);
      return [this.ellipseProfile(radius, radius, position), this.ellipseProfile(radius - wall, radius - wall, position)];
    }
    return [this.profile(id)];
  }
  extrusion(entity: GeometryEntity<V>, transform: Matrix, mesh: GeometryMesh): void {
    const profile = ref(entity, 0) ?? fail("missing-swept-area"), position = ref(entity, 1), directionId = ref(entity, 2) ?? fail("missing-extrusion-direction"), depth = real(entity.args[3]);
    if (depth <= 0) fail("invalid-extrusion-depth");
    const polygons = this.profileRings(profile), direction = normalize(this.point(directionId, true));
    if (Math.abs(direction[2]) < 1e-12) fail("extrusion-direction-in-profile-plane");
    const points: Point[] = [], rings: number[][] = [];
    for (const polygon of polygons) {
      if (polygon.length < 3) fail("profile-needs-three-points");
      const origin = polygon[0]; let extent = 0;
      for (const point of polygon) {this.tick(); extent = Math.max(extent, Math.abs(point[0] - origin[0]), Math.abs(point[1] - origin[1]));}
      let area = 0;
      for (let i = 0; i < polygon.length; i++) {this.tick(); const a = polygon[i], b = polygon[(i + 1) % polygon.length]; area += (a[0]-origin[0])/extent * ((b[1]-origin[1])/extent) - (b[0]-origin[0])/extent * ((a[1]-origin[1])/extent);}
      const order = area * (rings.length === 0 ? 1 : -1) > 0 ? polygon : [...polygon].reverse();
      rings.push(order.map(point => {this.tick(); points.push(point); return points.length - 1;}));
    }
    const caps = triangulateRings(points, rings, () => this.tick());
    if (position !== undefined) transform = multiply(transform, this.axis(position));
    const bottom = points.map(point => this.vertex(mesh, point, transform)), top = points.map(point => this.vertex(mesh, add(point, scale(direction, depth)), transform));
    for (const triangle of caps) {
      const [a, b, c] = triangle;
      this.triangle(mesh, direction[2] > 0 ? [bottom[c], bottom[b], bottom[a]] : [bottom[a], bottom[b], bottom[c]]);
      this.triangle(mesh, direction[2] > 0 ? [top[a], top[b], top[c]] : [top[c], top[b], top[a]]);
    }
    for (const ring of rings) for (let i = 0; i < ring.length; i++) {
      this.tick(); const a = ring[i], b = ring[(i + 1) % ring.length];
      this.triangle(mesh, direction[2] > 0 ? [bottom[a], bottom[b], top[b]] : [bottom[b], bottom[a], top[a]]);
      this.triangle(mesh, direction[2] > 0 ? [bottom[a], top[b], top[a]] : [bottom[b], top[a], top[b]]);
    }
  }
}

return {projector:new Projector(limits,progress),ref,productHint};
}
/** 🚦️ Project complete products with original caller cancellation and finite source-work limits. */
export function projectControlled<V>(graph: GeometryGraph<V>, limits: GeometryLimits = DEFAULT_GEOMETRY_LIMITS, progress: (value: GeometryProgress) => GeometryControl = () => "continue"): GeometryScene {
  if (Object.values(limits).some(value => !Number.isSafeInteger(value) || value < 0)) return {meshes: [], issues: [{entity: 0n, reason: "invalid-geometry-limits"}], status: "limited", work: 0};
  const {projector,ref,productHint}=createProjector(graph,limits,progress);
  try {
    projector.tick();
    for(let i=0;i<graph.length;i++){
      const entity=graph.entity(i)??fail("missing-graph-instance");
      projector.tick();
      if (projector.entities.has(entity.id)) {projector.scene.issues.push({entity: entity.id, reason: "duplicate-entity-id"}); return projector.scene;}
      projector.entities.set(entity.id, entity);
    }
    const excluded = new Map<bigint, string>();
    for (const relation of projector.entities.values()) {
      projector.tick();
      if(relation.name==="IFCRELVOIDSELEMENT"){const host=ref(relation,4),opening=ref(relation,5);if(host!==undefined){if(opening===undefined||relation.constituentCount!==1)excluded.set(host,"invalid-opening-relation-#"+relation.id);else {const ids=projector.openings.get(host)??[];if(!ids.includes(opening))ids.push(opening);projector.openings.set(host,ids);}}if(opening!==undefined)excluded.set(opening,"opening-is-a-void-volume");}
    }
    for (const product of projector.entities.values()) {
      projector.tick();
      const representation = ref(product, 6), shape = representation === undefined ? undefined : projector.entities.get(representation);
      if (representation === undefined) continue;
      const validShape = shape !== undefined && ["IFCPRODUCTDEFINITIONSHAPE", "IFCPRODUCTREPRESENTATION"].includes(shape.name);
      if (!productHint(product)) continue;
      if(product.constituentCount!==1){projector.scene.issues.push({entity:product.id,reason:"unsupported-complex-product"});continue;}
      if (!shape) {projector.scene.issues.push({entity: product.id, reason: "missing-product-representation-#" + representation}); continue;}
      if (!validShape) {projector.scene.issues.push({entity: product.id, reason: "invalid-product-representation-" + shape.name + "-#" + representation}); continue;}
      const reason = excluded.get(product.id) ?? (["IFCOPENINGELEMENT", "IFCOPENINGSTANDARDCASE"].includes(product.name) ? "opening-is-a-void-volume" : undefined);
      if (reason) {projector.scene.issues.push({entity: product.id, reason}); continue;}
      try {projector.product(product, shape);} catch (error) {projector.scene.issues.push({entity: product.id, reason: error instanceof Error ? error.message : String(error)});}
      if (projector.scene.status !== "complete") break;
    }
  } catch (error) {if (projector.scene.status === "complete") throw error;}
  return projector.scene;
}
export function project<V>(graph: GeometryGraph<V>): GeometryScene {return projectControlled(graph);}
export function projectWith<V>(graph: GeometryGraph<V>, limits: GeometryLimits, progress: (value: GeometryProgress) => boolean): GeometryScene {return projectControlled(graph, limits, value => progress(value) ? "continue" : "cancel");}
export function geometryBounds(scene: GeometryScene): [Point, Point] | undefined {
  let bounds: [Point, Point] | undefined;
  for (const mesh of scene.meshes) for (const point of mesh.vertices) {if (!bounds) bounds = [[...point], [...point]]; else for (let i = 0; i < 3; i++) {bounds[0][i] = Math.min(bounds[0][i], point[i]); bounds[1][i] = Math.max(bounds[1][i], point[i]);}}
  return bounds;
}
function payloadCamera(bounds: [Point, Point]): [Point, Point] | undefined {
  const span = subtract(bounds[1],bounds[0]);
  if (!span.every(Number.isFinite)) return undefined;
  const target = bounds[0].map((value,axis)=>value+span[axis]/2) as Point, extent = Math.max(1,...span);
  const position = target.map((value,axis)=>{const offset=[1.5,-1.5,1.2][axis]*extent,preferred=value+offset;return Number.isFinite(preferred)?preferred:value-offset;}) as Point;
  return [...position,...target].every(Number.isFinite)?[position,target]:undefined;
}
/** 📷️ Refuse public scene values that cannot produce a finite indexed payload and camera. */
export function geometryPayloadIssue(scene: GeometryScene): GeometryIssue | undefined {
  if (scene.status !== "complete") return undefined;
  for (const mesh of scene.meshes) {
    for (const point of mesh.vertices) if(!Array.isArray(point)||point.length!==3||[0,1,2].some(axis=>!Number.isFinite(point[axis])))return {entity:mesh.product,reason:"nonfinite-mesh-coordinate"};
    for (const triangle of mesh.triangles) if(!Array.isArray(triangle)||triangle.length!==3||[0,1,2].some(axis=>!Number.isSafeInteger(triangle[axis])||triangle[axis]<0||triangle[axis]>=mesh.vertices.length)||triangle[0]===triangle[1]||triangle[1]===triangle[2]||triangle[2]===triangle[0])return {entity:mesh.product,reason:"invalid-mesh-index"};
  }
  if(!payloadCamera(geometryBounds(scene)??[[0,0,0],[0,0,0]]))return {entity:0n,reason:"unrepresentable-camera-range"};
  for(const mesh of scene.meshes){
    const origin=mesh.vertices[0]??[0,0,0];let extent=0;
    for(const point of mesh.vertices)for(let axis=0;axis<3;axis++)extent=Math.max(extent,Math.abs(point[axis]-origin[axis]));
    for(const triangle of mesh.triangles){const [a,b,c]=triangle.map(index=>mesh.vertices[index]),normal=cross(subtract(b,a).map(value=>value/extent) as Point,subtract(c,a).map(value=>value/extent) as Point);if(!normal.every(Number.isFinite)||!normal.some(value=>value!==0))return {entity:mesh.product,reason:"unrepresentable-mesh-normal"};}
  }
  return undefined;
}
export function geometryPayloads(scene: GeometryScene): GeometryPayloads {
  const issue = geometryPayloadIssue(scene), status: GeometryStatus = issue ? "limited" : scene.status;
  const meshes = status === "complete" ? scene.meshes : [], issues = issue ? [...scene.issues,issue] : scene.issues;
  const [position,target] = payloadCamera(geometryBounds({...scene,meshes})??[[0,0,0],[0,0,0]])!;
  const meshesJson = JSON.stringify(meshes.map(mesh => {
    const normals: Point[] = mesh.vertices.map(() => [0, 0, 0]);
    let length = 0; const origin = mesh.vertices[0] ?? [0, 0, 0];
    for (const point of mesh.vertices) for (let i=0;i<3;i++) length = Math.max(length, Math.abs(point[i]-origin[i]));
    for (const triangle of mesh.triangles) {
      const [a, b, c] = triangle.map(index => mesh.vertices[index]), ab = subtract(b,a).map(value => value / length) as Point, ac = subtract(c,a).map(value => value / length) as Point, normal = cross(ab,ac);
      for (const index of triangle) normals[index] = add(normals[index], normal);
    }
    return {id: "ifc-" + mesh.product + "-" + mesh.representation, role: "solid", data: {positions: mesh.vertices.flat(), normals: normals.flatMap(value => value.every(Number.isFinite) && value.some(coordinate => coordinate !== 0) ? normalize(value) : [0, 0, 0]), indices: mesh.triangles.flat(), edgePositions: []}};
  }));
  const instancesJson = JSON.stringify(meshes.map(mesh => ({id: "ifc-" + mesh.product + "-" + mesh.representation, meshId: "ifc-" + mesh.product + "-" + mesh.representation, interactionId: "ifc-" + mesh.product, position: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], label: mesh.label, smoothShading: false})));
  return {cameraPosition: position, cameraTarget: target, meshesJson, instancesJson, statusJson: JSON.stringify({status, work: scene.work, issues: issues.map(issue => ({entity: issue.entity.toString(), reason: issue.reason})), label: {en: "IFC Representation Geometry", de: "IFC-Repräsentationsgeometrie"}})};
}
