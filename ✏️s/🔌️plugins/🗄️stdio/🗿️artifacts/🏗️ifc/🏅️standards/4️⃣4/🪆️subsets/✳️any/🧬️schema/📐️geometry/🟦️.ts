import type {IfcSnapshot} from "../📸️snapshot/🟦️.ts";
import {Ifc4GeometryGraph} from "./📖️reader/🟦️.ts";
import {project as projectGraph,projectControlled as projectGraphControlled,projectWith as projectGraphWith,type GeometryLimits,type GeometryProgress,type GeometryControl,type GeometryScene,DEFAULT_GEOMETRY_LIMITS} from "../../../../../../📐️geometry/🟦️.ts";
export * from "../../../../../../📐️geometry/🟦️.ts";
/** 🏗️ Project the native IFC4 graph in source length units. */
export function project(snapshot:IfcSnapshot):GeometryScene{return projectGraph(new Ifc4GeometryGraph(snapshot));}
export function projectControlled(snapshot:IfcSnapshot,limits:GeometryLimits=DEFAULT_GEOMETRY_LIMITS,progress:(value:GeometryProgress)=>GeometryControl=()=>"continue"):GeometryScene{return projectGraphControlled(new Ifc4GeometryGraph(snapshot),limits,progress);}
export function projectWith(snapshot:IfcSnapshot,limits:GeometryLimits,progress:(value:GeometryProgress)=>boolean):GeometryScene{return projectGraphWith(new Ifc4GeometryGraph(snapshot),limits,progress);}
