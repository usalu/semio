import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import type { DwgLogicalObjectBody,DwgAssociativeDependency,DwgAssociativeAction,DwgEvaluationExpression } from "../../../../🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import * as objects from "../../../🪶️sqlite/📦️objects/🟦️.ts";
import * as evaluation from "../../../🪶️sqlite/🧮️evaluation/🟦️.ts";
import * as associative from "../../../🪶️sqlite/🔗️associativity/🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import { dwgBodyKind } from "../../../🪶️sqlite/✏️drawing/🟦️.ts";
import fixture from "../../../🧫️fixtures/🪶️sqlite/📦️objects/🔣️.json";
import { exportSqliteDatabase,importSqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import { dwgObjectBodies as bodies } from "../../../🧫️fixtures/🪶️sqlite/📦️objects/🟦️.ts";
const max=18446744073709551615n;
async function projectBody(p:DwgProjection,id:bigint,b:DwgLogicalObjectBody){switch(b.kind){
  case"placeholder":return objects.dwgProjectPlaceholder(p,id,b.value);case"dictionaryVariable":return objects.dwgProjectDictionaryVariable(p,id,b.value);case"annotationScale":return objects.dwgProjectAnnotationScale(p,id,b.value);case"sortEntitiesTable":return objects.dwgProjectSortEntities(p,id,b.value);case"blockParameterDependencyBody":return objects.dwgProjectParameterDependency(p,id,b.value);case"associativeDimensionDependencyBody":return objects.dwgProjectDimensionDependency(p,id,b.value);case"blockRepresentationData":return objects.dwgProjectRepresentation(p,id,b.value);case"dynamicBlockPurgePreventer":return objects.dwgProjectPurgePreventer(p,id,b.value);case"evaluationGraph":return objects.dwgProjectGraph(p,id,b.value);
  case"associativeDependency":return associative.dwgProjectDependency(p,id,b.value);case"associativeValueDependency":return associative.dwgProjectValueDependency(p,id,b.value);case"associativeGeometryDependency":return associative.dwgProjectGeometryDependency(p,id,b.value);case"associativeVariable":return associative.dwgProjectVariable(p,id,b.value);case"assocNetwork":return associative.dwgProjectNetwork(p,id,b.value);
  case"blockGripLocationComponent":return evaluation.dwgProjectGripLocation(p,id,b.value);case"dynamicBlockProxyNode":return evaluation.dwgProjectProxy(p,id,b.value);default:throw new Error("test body unknown");
}}
async function reconstructBody(r:DwgReader,id:bigint,kind:string):Promise<DwgLogicalObjectBody>{switch(kind){
  case"placeholder":return{kind:"placeholder",value:await objects.dwgReconstructPlaceholder(r,id)};case"dictionary_variable":return{kind:"dictionaryVariable",value:await objects.dwgReconstructDictionaryVariable(r,id)};case"annotation_scale":return{kind:"annotationScale",value:await objects.dwgReconstructAnnotationScale(r,id)};case"sort_entities_table":return{kind:"sortEntitiesTable",value:await objects.dwgReconstructSortEntities(r,id)};case"block_parameter_dependency_body":return{kind:"blockParameterDependencyBody",value:await objects.dwgReconstructParameterDependency(r,id)};case"associative_dimension_dependency_body":return{kind:"associativeDimensionDependencyBody",value:await objects.dwgReconstructDimensionDependency(r,id)};case"block_representation_data":return{kind:"blockRepresentationData",value:await objects.dwgReconstructRepresentation(r,id)};case"dynamic_block_purge_preventer":return{kind:"dynamicBlockPurgePreventer",value:await objects.dwgReconstructPurgePreventer(r,id)};case"evaluation_graph":return{kind:"evaluationGraph",value:await objects.dwgReconstructGraph(r,id)};
  case"associative_dependency":return{kind:"associativeDependency",value:await associative.dwgReconstructDependency(r,id)};case"associative_value_dependency":return{kind:"associativeValueDependency",value:await associative.dwgReconstructValueDependency(r,id)};case"associative_geometry_dependency":return{kind:"associativeGeometryDependency",value:await associative.dwgReconstructGeometryDependency(r,id)};case"associative_variable":return{kind:"associativeVariable",value:await associative.dwgReconstructVariable(r,id)};case"assoc_network":return{kind:"assocNetwork",value:await associative.dwgReconstructNetwork(r,id)};
  case"block_grip_location_component":return{kind:"blockGripLocationComponent",value:await evaluation.dwgReconstructGripLocation(r,id)};case"dynamic_block_proxy_node":return{kind:"dynamicBlockProxyNode",value:await evaluation.dwgReconstructProxy(r,id)};default:throw new Error("test body unknown");
}}
async function read(native:Database){const r=await DwgReader.create(await importSqliteDatabase(native.serialize()),DWG_SQLITE_SCHEMA);await r.one("dwg_document");await r.one("dwg_drawing");const result:DwgLogicalObjectBody[]=[];for(const row of await r.list("dwg_object",1,1n,2))result.push(await reconstructBody(r,row.rowid,row.text(12)));await r.finish();return result;}
test("DWG named objects, graph topology, associative state and eight expression variants survive SQLite",async()=>{
  const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["DWG.SQLite.semantic","AC1024",0n,0n]);await p.insert("dwg_drawing",[1n]);
  for(const[index,b]of bodies.entries()){const id=await p.insert("dwg_object",[1n,BigInt(index),0n,BigInt(index),0n,"","object",null,null,null,null,dwgBodyKind(b)]);await projectBody(p,id,b);}
  const native=Database.deserialize(await exportSqliteDatabase(await p.finish()));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await read(native)).toEqual(bodies);
    expect(native.query("SELECT value_kind FROM dwg_evaluation_expression ORDER BY id LIMIT 8").all()).toEqual(["empty","double","point_group_10","point_group_11","string","integer32","object_reference","integer16"].map(value_kind=>({value_kind})));
    expect(BigInt(fixture.unsignedMaximum)).toBe(max);
    native.run("UPDATE dwg_associative_variable SET name='edited'; UPDATE dwg_evaluation_graph_edge SET reference_count=17 WHERE ordinal=1");const edited=await read(native);const variable=edited[12]!;expect(variable.kind).toBe("associativeVariable");if(variable.kind!=="associativeVariable")throw new Error("variable kind");expect(variable.value.name).toBe("edited");
    native.run("INSERT INTO dwg_evaluation_point_group_10_coordinate(evaluation_expression_id,ordinal,coordinate_class,coordinate_ieee754_bits,coordinate) VALUES(15,0,'negative_zero',-9223372036854775808,NULL)");expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(read(native)).rejects.toThrow();
    console.log("[DEBUG] DWG named objects, graph edges, associative joins and exact eight expression variants verified");
  }finally{native.close();}
},20000);
