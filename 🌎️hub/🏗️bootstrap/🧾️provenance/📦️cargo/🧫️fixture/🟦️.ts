import {createHash} from "node:crypto";
import schemaDocument from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {readScriptProcessJsonV1} from "@semio-tech/process/routing/invocation/process/json";
const schema=structuredClone(schemaDocument);
export type CheckInProcessEnvelopeV1=Readonly<{mutationId:string;dependencies:readonly string[];diffSchema:"gis.map";diff:string;inverseSchema:"gis.map";inverse:string}>;
export type CheckInProcessFixtureV1=Readonly<{schema:"semio.hub.check-in-process-fixture/v1";profileId:"gis-map-integration-fixtures";generationId:string;documentId:string;package:Readonly<{pluginId:"gis";packageId:"semio:gis";version:string;componentSha256:string}>;artifact:Readonly<{kind:"s.gis.gismap";schema:"gis.map";packSchemaHash:string}>;surfaceId:string;payload:Readonly<{envelopes:Readonly<{path:"payload/envelopes.json";count:number;sha256:string}>}>}>;
export type CheckInProcessFixtureDataV1=Readonly<{fixture:CheckInProcessFixtureV1;envelopes:readonly CheckInProcessEnvelopeV1[]}>;
export const checkInProcessFixturePolicyV1=Object.freeze({...schema.$defs.Policy.const});
/** 🧫️ Captures closed native-fixture DATA without giving paths or receipts producer custody. */
export function readCheckInProcessFixtureDataV1(value:unknown,envelopeBytes:Uint8Array):CheckInProcessFixtureDataV1{
 if(!(envelopeBytes instanceof Uint8Array)||envelopeBytes.byteLength>checkInProcessFixturePolicyV1.maximumMetadataBytes)throw Error("Bounded check-in fixture envelope bytes required");
 const captured=readScriptProcessJsonV1({fixture:value}).fixture;
 if(validateJsonSchemaSubset(schema,captured).length)throw Error("Closed check-in process fixture required");
 const fixture=captured as unknown as CheckInProcessFixtureV1;
 if(createHash("sha256").update(envelopeBytes).digest("hex")!==fixture.payload.envelopes.sha256)throw Error("Exact check-in fixture payload hash required");
 const envelopes:unknown=JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(envelopeBytes));
 if(validateJsonSchemaSubset({$ref:"#/$defs/Envelopes",$defs:schema.$defs},envelopes).length)throw Error("Closed check-in ledger envelopes required");
 const rows=envelopes as readonly CheckInProcessEnvelopeV1[];
 if(rows.length!==fixture.payload.envelopes.count)throw Error("Exact check-in fixture payload count required");
 return Object.freeze({fixture:Object.freeze({...fixture,package:Object.freeze({...fixture.package}),artifact:Object.freeze({...fixture.artifact}),payload:Object.freeze({envelopes:Object.freeze({...fixture.payload.envelopes})})}),envelopes:Object.freeze(rows.map(row=>Object.freeze({...row,dependencies:Object.freeze([...row.dependencies])})))});
}

/** 🎯️ Captures the two authored consumers without accepting destination or source selection. */
export function readCheckInProcessFixtureUseV1(value:unknown):"runtime"|"publication"{const captured=readScriptProcessJsonV1({use:value}).use;if(validateJsonSchemaSubset({$ref:"#/$defs/Use",$defs:schema.$defs},captured).length)throw Error("Closed check-in fixture use required");return(captured as unknown as {purpose:"runtime"|"publication"}).purpose;}
