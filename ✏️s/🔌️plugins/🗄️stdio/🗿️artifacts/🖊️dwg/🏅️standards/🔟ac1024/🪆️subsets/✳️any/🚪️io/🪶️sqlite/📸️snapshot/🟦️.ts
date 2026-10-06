/** 🖊️ DWG's complete explicit relational snapshot schema. */
import { DWG_DOCUMENT_SQL } from "./📄️document/🟦️.ts";
import { DWG_HEADER_SQL } from "./🔧️header/🟦️.ts";
import { DWG_DRAWING_SQL } from "./✏️drawing/🟦️.ts";
import { DWG_TABLES_SQL } from "./🗃️tables/🟦️.ts";
import { DWG_ENTITIES_SQL } from "./📐️entities/🟦️.ts";
import { DWG_OBJECTS_SQL } from "./📦️objects/🟦️.ts";
import { DWG_ASSOCIATIVITY_SQL } from "./🔗️associativity/🟦️.ts";
import { DWG_EVALUATION_SQL } from "./🧮️evaluation/🟦️.ts";
import { DWG_BLOCKS_SQL } from "./🧩️blocks/🟦️.ts";
import { DWG_ACTIONS_SQL } from "./🎬️actions/🟦️.ts";
import { DWG_VISUAL_STYLE_SQL } from "./🖌️styles/👁️visual/🟦️.ts";
import { DWG_MATERIAL_SQL } from "./🖌️styles/🧱️material/🟦️.ts";
import { DWG_TABLE_STYLE_SQL } from "./🖌️styles/🗃️table/🟦️.ts";
import { DWG_LAYOUT_SQL } from "./📃️layout/🟦️.ts";
import { DWG_MLEADER_SQL } from "./🖌️styles/↗️mleader/🟦️.ts";
import { DWG_CONSTRAINTS_SQL } from "./📏️constraints/🟦️.ts";
import type { DwgSnapshot,DwgLogicalObjectBody } from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import { DwgProjection } from "./🔢️number/🟦️.ts";
import { DwgReader } from "./🫳️reader/🟦️.ts";
import { dwgProjectDocument,dwgReconstructDocument } from "./📄️document/🟦️.ts";
import { dwgProjectHeader,dwgReconstructHeader } from "./🔧️header/🟦️.ts";
import { dwgProjectDrawing,dwgReconstructDrawing } from "./✏️drawing/🟦️.ts";
import * as tables from "./🗃️tables/🟦️.ts";
import { dwgProjectRecord,dwgReconstructRecord } from "./🗃️tables/📇️records/🟦️.ts";
import { dwgProjectEntity,dwgReconstructEntity } from "./📐️entities/🟦️.ts";
import * as objects from "./📦️objects/🟦️.ts";
import * as associativity from "./🔗️associativity/🟦️.ts";
import * as evaluation from "./🧮️evaluation/🟦️.ts";
import * as blocks from "./🧩️blocks/🟦️.ts";
import * as actions from "./🎬️actions/🟦️.ts";
import { dwgProjectVisualStyle,dwgReconstructVisualStyle } from "./🖌️styles/👁️visual/🟦️.ts";
import { dwgProjectMaterial,dwgReconstructMaterial,dwgProjectMline,dwgReconstructMline } from "./🖌️styles/🧱️material/🟦️.ts";
import { dwgProjectTableStyle,dwgReconstructTableStyle } from "./🖌️styles/🗃️table/🟦️.ts";
import { dwgProjectLayout,dwgReconstructLayout } from "./📃️layout/🟦️.ts";
import { dwgProjectMLeader,dwgReconstructMLeader } from "./🖌️styles/↗️mleader/🟦️.ts";
import { dwgProjectConstraints,dwgReconstructConstraints } from "./📏️constraints/🟦️.ts";
import { artifactSqliteCheckpoint,type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import type { ArtifactDialect } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts";

export const DWG_SQLITE_SCHEMA=[DWG_DOCUMENT_SQL,DWG_HEADER_SQL,DWG_DRAWING_SQL,DWG_TABLES_SQL,DWG_ENTITIES_SQL,DWG_OBJECTS_SQL,DWG_ASSOCIATIVITY_SQL,DWG_EVALUATION_SQL,DWG_BLOCKS_SQL,DWG_ACTIONS_SQL,DWG_VISUAL_STYLE_SQL,DWG_MATERIAL_SQL,DWG_TABLE_STYLE_SQL,DWG_LAYOUT_SQL,DWG_MLEADER_SQL,DWG_CONSTRAINTS_SQL].join("\n");

/** 📤️ Project every explicitly owned DWG domain into its authored relational schema. */
export async function dwgSnapshotToSqliteDatabase(snapshot:DwgSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
  const projection=await DwgProjection.create(DWG_SQLITE_SCHEMA,options);
  await dwgProjectDocument(projection,snapshot);await dwgProjectHeader(projection,snapshot.header);
  await dwgProjectDrawing(projection,snapshot.drawing,projectBody);return projection.finish();
}

/** 📥️ Reconstruct all DWG typed domains and reject every unconsumed semantic row. */
export async function dwgSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<DwgSnapshot>{
  const reader=await DwgReader.create(database,DWG_SQLITE_SCHEMA,options);
  const document=await dwgReconstructDocument(reader),header=await dwgReconstructHeader(reader),drawing=await dwgReconstructDrawing(reader,reconstructBody);await reader.finish();
  return{schema:document.schema,version:document.version,maintenanceVersion:document.maintenanceVersion,codepage:document.codepage,drawing,header,classes:document.classes,dependencies:document.dependencies,summary:document.summary,application:document.application,template:document.template,auxiliaryHeader:document.auxiliaryHeader,revisionHistory:document.revisionHistory,preview:document.preview,applicationHistory:document.applicationHistory};
}

/** 🛂️ Validate exact AC1018/AC1024 coordinates and every owned semantic field. */
export async function dwgSnapshotValidateSqliteSubset(snapshot:DwgSnapshot,dialect:ArtifactDialect,database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<void>{
  await artifactSqliteCheckpoint(options,"projectSnapshot",0,0);
  if(dialect.artifactKind!=="s.stdio.dwg"||(dialect.standard!=="ac1018"&&dialect.standard!=="ac1024")||dialect.subset!=="*")throw new Error("DWG owned SQLite dialect must be an exact AC1018 or AC1024 full snapshot");
  const candidate=await dwgSnapshotFromSqliteDatabase(database,options),expected=await dwgSnapshotToSqliteDatabase(snapshot,options),actual=await dwgSnapshotToSqliteDatabase(candidate,options);
  if(expected.tables.length!==actual.tables.length)throw new Error("DWG owned state differs from its semantic projection");
  const total=expected.tables.reduce((sum,table)=>sum+table.rows.length,0);let completed=0;
  for(let tableIndex=0;tableIndex<expected.tables.length;tableIndex++){
    const left=expected.tables[tableIndex]!,right=actual.tables[tableIndex]!;
    if(left.name!==right.name||left.sql!==right.sql||left.rows.length!==right.rows.length)throw new Error("DWG owned state differs from its semantic projection");
    for(let rowIndex=0;rowIndex<left.rows.length;rowIndex++){
      const a=left.rows[rowIndex]!,b=right.rows[rowIndex]!;
      if(a.rowid!==b.rowid||a.values.length!==b.values.length||a.values.some((value,column)=>!Object.is(value,b.values[column])))throw new Error("DWG owned state differs from its semantic projection");
      if(++completed%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total);
    }
  }
  await artifactSqliteCheckpoint(options,"projectSnapshot",total,total);
}

async function projectBody(p:DwgProjection,id:bigint,body:DwgLogicalObjectBody):Promise<void>{
  switch(body.kind){
    case"dictionary":return tables.dwgProjectDictionary(p,id,body.value);case"tableControl":return tables.dwgProjectControl(p,id,body.value);case"tableRecord":return dwgProjectRecord(p,id,body.value);case"xrecord":return tables.dwgProjectXRecord(p,id,body.value);case"entity":return dwgProjectEntity(p,id,body.value);
    case"associativeDependency":return associativity.dwgProjectDependency(p,id,body.value);case"associativeValueDependency":return associativity.dwgProjectValueDependency(p,id,body.value);case"associativeGeometryDependency":return associativity.dwgProjectGeometryDependency(p,id,body.value);
    case"blockGripLocationComponent":return evaluation.dwgProjectGripLocation(p,id,body.value);case"dynamicBlockProxyNode":return evaluation.dwgProjectProxy(p,id,body.value);case"associativeVariable":return associativity.dwgProjectVariable(p,id,body.value);
    case"associativeDimensionDependencyBody":return objects.dwgProjectDimensionDependency(p,id,body.value);case"visualStyle":return dwgProjectVisualStyle(p,id,body.value);case"blockParameterDependencyBody":return objects.dwgProjectParameterDependency(p,id,body.value);case"blockRepresentationData":return objects.dwgProjectRepresentation(p,id,body.value);case"dynamicBlockPurgePreventer":return objects.dwgProjectPurgePreventer(p,id,body.value);case"evaluationGraph":return objects.dwgProjectGraph(p,id,body.value);
    case"blockFlipParameter":return blocks.dwgProjectFlipParameter(p,id,body.value);case"blockVisibilityParameter":return blocks.dwgProjectVisibilityParameter(p,id,body.value);case"placeholder":return objects.dwgProjectPlaceholder(p,id,body.value);case"dictionaryVariable":return objects.dwgProjectDictionaryVariable(p,id,body.value);case"annotationScale":return objects.dwgProjectAnnotationScale(p,id,body.value);case"sortEntitiesTable":return objects.dwgProjectSortEntities(p,id,body.value);
    case"tableStyle":return dwgProjectTableStyle(p,id,body.value);case"mlineStyle":return dwgProjectMline(p,id,body.value);case"mLeaderStyle":return dwgProjectMLeader(p,id,body.value);case"material":return dwgProjectMaterial(p,id,body.value);case"blockMoveAction":return actions.dwgProjectMove(p,id,body.value);
    case"assocNetwork":return associativity.dwgProjectNetwork(p,id,body.value);case"assoc2dConstraintGroup":return dwgProjectConstraints(p,id,body.value);case"blockLinearParameter":return blocks.dwgProjectLinearParameter(p,id,body.value);case"blockLinearGrip":return blocks.dwgProjectLinearGrip(p,id,body.value);case"blockFlipGrip":return blocks.dwgProjectFlipGrip(p,id,body.value);case"blockVisibilityGrip":return blocks.dwgProjectVisibilityGrip(p,id,body.value);case"blockAlignmentParameter":return blocks.dwgProjectAlignmentParameter(p,id,body.value);case"blockAlignmentGrip":return blocks.dwgProjectAlignmentGrip(p,id,body.value);
    case"blockStretchAction":return actions.dwgProjectStretch(p,id,body.value);case"blockScaleAction":return actions.dwgProjectScale(p,id,body.value);case"blockFlipAction":return actions.dwgProjectFlip(p,id,body.value);case"blockBasePointParameter":return blocks.dwgProjectBasePoint(p,id,body.value);case"blockVerticalConstraintParameter":return blocks.dwgProjectConstraintParameter(p,id,body.value,"vertical");case"blockHorizontalConstraintParameter":return blocks.dwgProjectConstraintParameter(p,id,body.value,"horizontal");case"layout":return dwgProjectLayout(p,id,body.value);
    default:throw new Error("DWG logical object body kind is unknown");
  }
}

async function reconstructBody(r:DwgReader,id:bigint,kind:string):Promise<DwgLogicalObjectBody>{
  switch(kind){
    case"dictionary":return{kind:"dictionary",value:await tables.dwgReconstructDictionary(r,id)};case"table_control":return{kind:"tableControl",value:await tables.dwgReconstructControl(r,id)};case"table_record":return{kind:"tableRecord",value:await dwgReconstructRecord(r,id)};case"xrecord":return{kind:"xrecord",value:await tables.dwgReconstructXRecord(r,id)};case"entity":return{kind:"entity",value:await dwgReconstructEntity(r,id)};
    case"associative_dependency":return{kind:"associativeDependency",value:await associativity.dwgReconstructDependency(r,id)};case"associative_value_dependency":return{kind:"associativeValueDependency",value:await associativity.dwgReconstructValueDependency(r,id)};case"associative_geometry_dependency":return{kind:"associativeGeometryDependency",value:await associativity.dwgReconstructGeometryDependency(r,id)};
    case"block_grip_location_component":return{kind:"blockGripLocationComponent",value:await evaluation.dwgReconstructGripLocation(r,id)};case"dynamic_block_proxy_node":return{kind:"dynamicBlockProxyNode",value:await evaluation.dwgReconstructProxy(r,id)};case"associative_variable":return{kind:"associativeVariable",value:await associativity.dwgReconstructVariable(r,id)};
    case"associative_dimension_dependency_body":return{kind:"associativeDimensionDependencyBody",value:await objects.dwgReconstructDimensionDependency(r,id)};case"visual_style":return{kind:"visualStyle",value:await dwgReconstructVisualStyle(r,id)};case"block_parameter_dependency_body":return{kind:"blockParameterDependencyBody",value:await objects.dwgReconstructParameterDependency(r,id)};case"block_representation_data":return{kind:"blockRepresentationData",value:await objects.dwgReconstructRepresentation(r,id)};case"dynamic_block_purge_preventer":return{kind:"dynamicBlockPurgePreventer",value:await objects.dwgReconstructPurgePreventer(r,id)};case"evaluation_graph":return{kind:"evaluationGraph",value:await objects.dwgReconstructGraph(r,id)};
    case"block_flip_parameter":return{kind:"blockFlipParameter",value:await blocks.dwgReconstructFlipParameter(r,id)};case"block_visibility_parameter":return{kind:"blockVisibilityParameter",value:await blocks.dwgReconstructVisibilityParameter(r,id)};case"placeholder":return{kind:"placeholder",value:await objects.dwgReconstructPlaceholder(r,id)};case"dictionary_variable":return{kind:"dictionaryVariable",value:await objects.dwgReconstructDictionaryVariable(r,id)};case"annotation_scale":return{kind:"annotationScale",value:await objects.dwgReconstructAnnotationScale(r,id)};case"sort_entities_table":return{kind:"sortEntitiesTable",value:await objects.dwgReconstructSortEntities(r,id)};
    case"table_style":return{kind:"tableStyle",value:await dwgReconstructTableStyle(r,id)};case"mline_style":return{kind:"mlineStyle",value:await dwgReconstructMline(r,id)};case"mleader_style":return{kind:"mLeaderStyle",value:await dwgReconstructMLeader(r,id)};case"material":return{kind:"material",value:await dwgReconstructMaterial(r,id)};case"block_move_action":return{kind:"blockMoveAction",value:await actions.dwgReconstructMove(r,id)};
    case"assoc_network":return{kind:"assocNetwork",value:await associativity.dwgReconstructNetwork(r,id)};case"assoc_2d_constraint_group":return{kind:"assoc2dConstraintGroup",value:await dwgReconstructConstraints(r,id)};case"block_linear_parameter":return{kind:"blockLinearParameter",value:await blocks.dwgReconstructLinearParameter(r,id)};case"block_linear_grip":return{kind:"blockLinearGrip",value:await blocks.dwgReconstructLinearGrip(r,id)};case"block_flip_grip":return{kind:"blockFlipGrip",value:await blocks.dwgReconstructFlipGrip(r,id)};case"block_visibility_grip":return{kind:"blockVisibilityGrip",value:await blocks.dwgReconstructVisibilityGrip(r,id)};case"block_alignment_parameter":return{kind:"blockAlignmentParameter",value:await blocks.dwgReconstructAlignmentParameter(r,id)};case"block_alignment_grip":return{kind:"blockAlignmentGrip",value:await blocks.dwgReconstructAlignmentGrip(r,id)};
    case"block_stretch_action":return{kind:"blockStretchAction",value:await actions.dwgReconstructStretch(r,id)};case"block_scale_action":return{kind:"blockScaleAction",value:await actions.dwgReconstructScale(r,id)};case"block_flip_action":return{kind:"blockFlipAction",value:await actions.dwgReconstructFlip(r,id)};case"block_base_point_parameter":return{kind:"blockBasePointParameter",value:await blocks.dwgReconstructBasePoint(r,id)};case"block_vertical_constraint_parameter":return{kind:"blockVerticalConstraintParameter",value:await blocks.dwgReconstructConstraintParameter(r,id,"vertical")};case"block_horizontal_constraint_parameter":return{kind:"blockHorizontalConstraintParameter",value:await blocks.dwgReconstructConstraintParameter(r,id,"horizontal")};case"layout":return{kind:"layout",value:await dwgReconstructLayout(r,id)};
    default:throw new Error("DWG logical object body kind is unknown");
  }
}
