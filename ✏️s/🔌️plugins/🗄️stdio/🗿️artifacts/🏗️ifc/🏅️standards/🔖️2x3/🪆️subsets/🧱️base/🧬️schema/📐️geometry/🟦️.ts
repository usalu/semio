import type {Ifc2x3Snapshot} from "../📸️snapshot/🟦️.ts";
import {Ifc2x3GeometryGraph} from "./📖️reader/🟦️.ts";
import {project as projectGraph,projectControlled as projectGraphControlled,projectWith as projectGraphWith,type GeometryLimits,type GeometryProgress,type GeometryControl,type GeometryScene,DEFAULT_GEOMETRY_LIMITS} from "../../../../../../📐️geometry/🟦️.ts";
export * from "../../../../../../📐️geometry/🟦️.ts";
/** 🧱️ Project native IFC2x3 instances without converting them to another IFC schema. */
export function project(snapshot:Ifc2x3Snapshot):GeometryScene{return projectGraph(new Ifc2x3GeometryGraph(snapshot.document));}
export function projectControlled(snapshot:Ifc2x3Snapshot,limits:GeometryLimits=DEFAULT_GEOMETRY_LIMITS,progress:(value:GeometryProgress)=>GeometryControl=()=>"continue"):GeometryScene{return projectGraphControlled(new Ifc2x3GeometryGraph(snapshot.document),limits,progress);}
export function projectWith(snapshot:Ifc2x3Snapshot,limits:GeometryLimits,progress:(value:GeometryProgress)=>boolean):GeometryScene{return projectGraphWith(new Ifc2x3GeometryGraph(snapshot.document),limits,progress);}
