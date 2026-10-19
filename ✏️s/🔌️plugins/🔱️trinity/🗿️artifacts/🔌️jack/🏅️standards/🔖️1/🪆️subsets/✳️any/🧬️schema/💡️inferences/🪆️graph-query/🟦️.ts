/** 🕸️ Jack-owned queries retain literal identities without importing a child representation. */
import type {JackArtifact} from "../../🟦️.ts";
import type {ArtifactChild} from "@semio-tech/framework/schema/artifact-reference";
import type {JackFlatPositionUv} from "../🟦️.ts";
import {ValueError} from "@semio-tech/framework/value/refusal";
export type {ArtifactChild};
export interface JackGraphEndpoint {readonly id:string;readonly source:string;readonly target:string}
export interface JackGraphQuery {
 readonly identity:ArtifactChild;
 nodeIds():Iterable<string>;
 edges():Iterable<JackGraphEndpoint>;
 displacement(edgeId:string):JackFlatPositionUv;
}
/** 🪪️ Admit the exact retained child and every target coordinate selected by its parent. */
export function retainedJackGraph(parent:JackArtifact,children:ReadonlyMap<string,JackGraphQuery>):JackGraphQuery {
 const expected=parent.content,child=children.get(expected.childId),actual=child?.identity;
 if(!child||!actual||actual.childId!==expected.childId||actual.target.artifactId!==expected.target.artifactId||actual.target.dialect.artifactKind!==expected.target.dialect.artifactKind||actual.target.dialect.standard!==expected.target.dialect.standard||actual.target.dialect.subset!==expected.target.dialect.subset)throw new ValueError("invalidValue","Jack inference requires exact retained child identity");
 return child;
}
