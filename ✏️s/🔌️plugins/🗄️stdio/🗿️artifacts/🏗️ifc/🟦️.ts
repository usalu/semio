/** 🗄️ stdio.ifc TypeScript facade. */
import definition from "./📜️artifact-definition.json" with { type: "json" };

export { definition };
export type ArtifactDefinition = typeof definition;
export * from "./🏅️standards/🔖️4x3/🪆️subsets/✳️any/🟦️.ts";
export type {IfcSnapshot,IfcValue,IfcEntity,IfcComplexType,IfcHeader} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts";
export type {IfcArtifact} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🟦️.ts";
export type {IfcDiff} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
export type {IfcMutation} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export {IFC_SQLITE_SCHEMA,ifcSnapshotToSqliteDatabase,ifcSnapshotFromSqliteDatabase} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";
export type {Ifc2x3Snapshot,Ifc2x3EdmPreamble,Part21Decimal,Part21Value,Part21Document,Part21Header,Part21Instance,Part21Entity} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
export type {Ifc2x3Artifact} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🟦️.ts";
export type {Ifc2x3Diff} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts";
export type {Ifc2x3Mutation} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts";
export {IFC2X3_SQLITE_SCHEMA,ifc2x3SnapshotToSqliteDatabase,ifc2x3SnapshotFromSqliteDatabase} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts";

export {applyIfcDiff as applyIfc4Diff,inverseIfcDiff as inverseIfc4Diff,absorbIfcDiff as absorbIfc4Diff} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";
export {diffIfcMutation as ifc4MutationDiff,applyIfcMutation as applyIfc4Mutation,inverseIfcMutation as inverseIfc4Mutation,ifcMutationInverseRows as ifc4MutationInverseRows} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts";
export {decodeIfc4 as parseIfc4Step,encodeIfc4 as writeIfc4Step} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🟦️.ts";
export type {Ifc4IoOptions} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/🟦️.ts";

export {project as projectIfc4Geometry,projectControlled as projectIfc4GeometryControlled,projectWith as projectIfc4GeometryWith,geometryBounds as ifc4GeometryBounds,geometryPayloads as ifc4GeometryPayloads,geometryPayloadIssue as ifc4GeometryPayloadIssue,DEFAULT_GEOMETRY_LIMITS as IFC4_GEOMETRY_LIMITS} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🟦️.ts";
export type {GeometryMesh as Ifc4GeometryMesh,GeometryIssue as Ifc4GeometryIssue,GeometryStatus as Ifc4GeometryStatus,GeometryControl as Ifc4GeometryControl,GeometryScene as Ifc4GeometryScene,GeometryLimits as Ifc4GeometryLimits,GeometryProgress as Ifc4GeometryProgress,GeometryPayloads as Ifc4GeometryPayloads} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🟦️.ts";
export type {IfcEntityRemoved,IfcEntityModified,IfcEntityAdded,IfcEntityDiff,IfcEntitiesDiff,IfcArgsDiff,IfcArgModified,IfcArgAdded} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts";

export {parseIfcGraphqlInteger as parseIfc4GraphqlInteger,printIfcGraphqlInteger as printIfc4GraphqlInteger,ifcGraphqlIndex as ifc4GraphqlIndex} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔢️integers/🟦️.ts";
export type {IfcGraphqlIntegerScalar as Ifc4GraphqlIntegerScalar} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔢️integers/🟦️.ts";

export {project as projectIfc2x3Geometry,projectControlled as projectIfc2x3GeometryControlled,projectWith as projectIfc2x3GeometryWith,geometryBounds as ifc2x3GeometryBounds,geometryPayloads as ifc2x3GeometryPayloads,geometryPayloadIssue as ifc2x3GeometryPayloadIssue,DEFAULT_GEOMETRY_LIMITS as IFC2X3_GEOMETRY_LIMITS} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📐️geometry/🟦️.ts";
export type {GeometryMesh as Ifc2x3GeometryMesh,GeometryIssue as Ifc2x3GeometryIssue,GeometryStatus as Ifc2x3GeometryStatus,GeometryControl as Ifc2x3GeometryControl,GeometryScene as Ifc2x3GeometryScene,GeometryLimits as Ifc2x3GeometryLimits,GeometryProgress as Ifc2x3GeometryProgress,GeometryPayloads as Ifc2x3GeometryPayloads} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📐️geometry/🟦️.ts";

export {projectSemantics as projectIfc4Semantics,editSemantic as planIfc4SemanticEdit,semanticIdentity as ifc4SemanticIdentity,parseSemanticIdentity as parseIfc4SemanticIdentity,SEMANTIC_CONTRACT as IFC4_SEMANTIC_CONTRACT} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔎️semantics/🟦️.ts";
export type {SemanticDescriptor as Ifc4SemanticDescriptor,SemanticContract as Ifc4SemanticContract,SemanticField as Ifc4SemanticField,SemanticProjection as Ifc4SemanticProjection,SemanticEdit as Ifc4SemanticEdit} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔎️semantics/🟦️.ts";

export {ifc4Catalog,ifc4Declaration,ifc4AttributeDeclarations,ifc4IsSubtype,ifc4Attribute,ifc4ResolveType,ifc4EnumMembers,ifc4SelectMembers,ifc4SelectAccepts} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts";
export type {IfcCatalog as Ifc4Catalog,IfcDeclaration as Ifc4Declaration,IfcEntityDeclaration as Ifc4EntityDeclaration,IfcAttributeDeclaration as Ifc4AttributeDeclaration,IfcTypeDeclaration as Ifc4TypeDeclaration,IfcTypeExpression as Ifc4TypeExpression} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts";

export {ifc2x3Catalog,ifc2x3Declaration,ifc2x3AttributeDeclarations,ifc2x3IsSubtype,ifc2x3Attribute,ifc2x3ResolveType,ifc2x3EnumMembers,ifc2x3SelectMembers,ifc2x3SelectAccepts} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🟦️.ts";
export type {IfcCatalog as Ifc2x3Catalog,IfcDeclaration as Ifc2x3Declaration,IfcEntityDeclaration as Ifc2x3EntityDeclaration,IfcAttributeDeclaration as Ifc2x3AttributeDeclaration,IfcTypeDeclaration as Ifc2x3TypeDeclaration,IfcTypeExpression as Ifc2x3TypeExpression} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📚️catalog/🟦️.ts";

export {detailsControl as ifcDetailsControl,parseReferenceId as parseIfcReferenceId,referenceMutation as planIfcReferenceMutation,typeSwitchMutations as planIfcEntityTypeSwitch,detailsWorkCapacity as ifcDetailsWorkCapacity,detailsPublicationRows as ifcDetailsPublicationRows} from "./🔎️details/🟦️.ts";
export type {DetailsPath as IfcDetailsPath,IfcDetailsControl,IfcDetailsWorkCapacity} from "./🔎️details/🟦️.ts";

export {ifc2x3DetailsControl,ifc2x3ReferenceMutation as planIfc2x3ReferenceMutation,ifc2x3TypeSwitchMutation as planIfc2x3TypeSwitchMutation} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/🟦️.ts";
export type {Ifc2x3DetailsPath} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🪟️details/🟦️.ts";

export {ifc2x3SelectionIdentity,parseIfc2x3SelectionIdentity,ifc2x3SelectionPath} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🚪️io/🔎️inspector/🟦️.ts";

export {booleanSolid as booleanIfcSolid,subtractSolid as subtractIfcSolid,subtractHalfSpace as subtractIfcHalfSpace} from "./📐️geometry/🧊️boolean/🟦️.ts";
export type {SolidMesh as IfcSolidMesh,SolidPoint as IfcSolidPoint,SolidTriangle as IfcSolidTriangle,SolidBooleanOptions as IfcSolidBooleanOptions,SolidBooleanOperation as IfcSolidBooleanOperation,SolidPlane as IfcSolidPlane} from "./📐️geometry/🧊️boolean/🟦️.ts";

export {LengthUnitWork as IfcLengthUnitWork,DEFAULT_LENGTH_UNIT_LIMITS as DEFAULT_IFC_LENGTH_UNIT_LIMITS} from "./📐️geometry/📏️units/🟦️.ts";
export type {LengthUnitResult as IfcLengthUnitResult,LengthUnitLimits as IfcLengthUnitLimits,LengthUnitControl as IfcLengthUnitControl} from "./📐️geometry/📏️units/🟦️.ts";

export {Ifc4GeometryGraph} from "./🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/📖️reader/🟦️.ts";
export {Ifc2x3GeometryGraph} from "./🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📐️geometry/📖️reader/🟦️.ts";
export type {GeometryGraph as IfcGeometryGraph,GeometryEntity as IfcGeometryEntity,GeometryConstituent as IfcGeometryConstituent,GeometryValues as IfcGeometryValues,GeometryStandard as IfcGeometryStandard} from "./📐️geometry/📖️graph/🟦️.ts";

export {checkIfcValueDomainSteps as checkIfcGraphValueDomainSteps,checkIfcValueChangeSteps as checkIfcGraphValueChangeSteps,ifcDomainEqualSteps as ifcGraphDomainEqualSteps} from "./📚️catalog/⚖️domain/🟦️.ts";
export {checkPart21ValueDomainSteps,checkPart21ValueChangeSteps,part21DomainEqualSteps} from "./📚️catalog/⚖️domain/part21.ts";
export type {IfcDomainSteps,IfcDomainControl,IfcValueChange} from "./📚️catalog/⚖️domain/core.ts";
export {ifcNameSteps,ifcSubtypeSteps,ifcResolveTypeSteps,ifcSelectAcceptsSteps} from "./📚️catalog/🟦️.ts";
