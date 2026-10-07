import artifactReferenceSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🔣️.json";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import fg from "fast-glob";
import { applyPatch } from "fast-json-patch";
import { addSemioMutationLeafSchemasV1, semioSchemaAjvV1 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import * as mutationContract from "../../🧬️mutations/🟦️.ts";
import {transformFixture} from "../../../../✉️base/🧬️schema/🧮️geometry/🧪️tests/🧰️support/🟦️.ts";

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
function pinFixture(value:any):any{return value?.kind==="snapshot"&&value.blob!==null&&typeof value.blob==="object"?{...value,blob:{...value.blob,size:typeof value.blob.size==="number"?BigInt(value.blob.size):value.blob.size}}:value;}
function pieceFixture(value:any):any{return value!==null&&typeof value==="object"&&Object.hasOwn(value,"transform")?{...value,transform:transformFixture(value.transform)}:value;}
function designFixture(value:any):any{return value!==null&&typeof value==="object"&&Array.isArray(value.pieces)?{...value,pieces:value.pieces.map(pieceFixture)}:value;}
function linkFixture(value:any):any{return value!==null&&typeof value==="object"&&Object.hasOwn(value,"pin")?{...value,pin:pinFixture(value.pin)}:value;}
function snapshotFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,...(Array.isArray(value.designs)?{designs:value.designs.map(designFixture)}:{}),...(Array.isArray(value.representations)?{representations:value.representations.map(linkFixture)}:{})}:value;}
function diffFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,...(Array.isArray(value.designs?.values)?{designs:{...value.designs,values:value.designs.values.map(designFixture)}}:{}),...(Array.isArray(value.representations?.values)?{representations:{...value.representations,values:value.representations.values.map(linkFixture)}}:{})}:value;}
function mutationFixture(value:any):any{if(value?.SetSnapshot)return{...value,SetSnapshot:{...value.SetSnapshot,snapshot:snapshotFixture(value.SetSnapshot.snapshot)}};if(value?.EditDesign)return{...value,EditDesign:{...value.EditDesign,...(Array.isArray(value.EditDesign.pieces)?{pieces:value.EditDesign.pieces.map(pieceFixture)}:{})}};if(value?.BindRepresentation)return{...value,BindRepresentation:linkFixture(value.BindRepresentation)};if(value?.ChangeRepresentationPin)return{...value,ChangeRepresentationPin:linkFixture(value.ChangeRepresentationPin)};return value;}

function referenceIntegrity(value: any): boolean {
  const typeIds = new Set(value.types.map((entry: any) => entry.id));
  if (typeIds.size !== value.types.length || new Set(value.designs.map((entry: any) => entry.id)).size !== value.designs.length) return false;
  for (const design of value.designs) {
    const pieceIds = new Set(design.pieces.map((entry: any) => entry.id));
    if (pieceIds.size !== design.pieces.length || !design.pieces.every((piece: any) => typeIds.has(piece.typeId))) return false;
    if (!design.connections.every((connection: any) => pieceIds.has(connection.connectingPieceId) && pieceIds.has(connection.connectedPieceId))) return false;
  }
  return value.representations.every((entry: any) => typeIds.has(entry.role));
}

function childIdentity(value: any): boolean {
  const lists = [["objects", "object"], ["models", "model"]] as const;
  for (const [field, subset] of lists) for (const child of value[field] ?? []) {
    if (child.target.dialect.artifactKind !== "s.stdio.semio" || child.target.dialect.standard !== "v1" || child.target.dialect.subset !== subset) return false;
  }
  const child = value.properties;
  return !child || child.target.dialect.artifactKind === "s.stdio.semio" && child.target.dialect.standard === "v1" && child.target.dialect.subset === "value";
}

function mutationChildIdentity(value: any): boolean {
  const subsets: Record<string, string> = { CreateObject: "object", CreateModel: "model", CreateProperties: "value" };
  const variant = Object.keys(value)[0];
  if (!Object.hasOwn(subsets, variant)) return true;
  const payload = value[variant];
  return typeof payload.child_id === "string"
    && payload.target.dialect.artifactKind === "s.stdio.semio"
    && payload.target.dialect.standard === "v1"
    && payload.target.dialect.subset === subsets[variant];
}

/** 🧪️ Kit catalog records, child identities and shared links agree with independent validators. */
export function testSemioKitDocumentContract(): void {
  const ajv = semioSchemaAjvV1({ allErrors: true }).addSchema(artifactReferenceSchema);
  for (const path of [
    "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json",
    "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🔣️.json",
    "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json",
    "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json",
    "../../../../✉️base/🧬️schema/🪆️child/🔣️.json",
    "../../../../✉️base/🧬️schema/🧮️geometry/🔣️.json",
  ]) ajv.addSchema(read(path));
  const fixtures = read("../../🧫️fixtures/🪪️document-contract/🔣️.json");
  const snapshotSchema = ajv.compile(read("../../📸️snapshot/🔣️.json"));
  const artifactSchema = ajv.compile(read("../../🔣️.json"));
  const diffSchema = ajv.compile(read("../../🔺️diff/🔣️.json"));
  const parseArtifact = artifact.parseSemioKitArtifact;
  const parseSnapshot = snapshot.parseSemioKitSnapshot;
  const parseDiff = diff.parseSemioKitDiff;
  for (const entry of fixtures.snapshotCases) {
    const admitted = artifactSchema(entry.input) && snapshotSchema(entry.input) && childIdentity(entry.input) && referenceIntegrity(entry.input);
    assert.equal(admitted, entry.valid, "independent artifact/snapshot oracle");
    if (entry.valid) {
      assert.deepEqual(parseArtifact(snapshotFixture(entry.input)), snapshotFixture(entry.input));
      assert.deepEqual(parseSnapshot(snapshotFixture(entry.input)), snapshotFixture(entry.input));
    } else {
      assert.throws(() => parseArtifact(snapshotFixture(entry.input)), JSON.stringify(entry.input));
      assert.throws(() => parseSnapshot(snapshotFixture(entry.input)), JSON.stringify(entry.input));
    }
  }
  for (const entry of fixtures.diffCases) {
    assert.equal(diffSchema(entry.input) && childIdentity({ types: [], designs: [], objects: entry.input.objects?.values ?? [], models: entry.input.models?.values ?? [], properties: entry.input.properties, representations: [] }), entry.valid, "independent diff oracle");
    if (entry.valid) assert.deepEqual(parseDiff(diffFixture(entry.input)), diffFixture(entry.input));
    else assert.throws(() => parseDiff(diffFixture(entry.input)), JSON.stringify(entry.input));
  }
  for (const entry of fixtures.patchCases) {
    const expected = applyPatch(structuredClone(entry.before), entry.patch, true).newDocument;
    assert.deepEqual(expected, entry.after, "independent patch oracle");
    assert.deepEqual(diff.applySemioKitDiff(parseArtifact(snapshotFixture(entry.before)), parseDiff(diffFixture(entry.diff))), snapshotFixture(expected), "Kit parent edit");
  }

  addSemioMutationLeafSchemasV1(ajv, new URL("../../🧬️mutations/", import.meta.url));
  const mutationSchema = ajv.compile(read("../../🧬️mutations/🔣️.json"));
  const parseMutation = mutationContract.parseSemioKitMutation;
  let snapshots = 0, diffs = 0, mutations = 0;
  const mutationVariants = new Set<string>();
  const corpusRoot = fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url));
  for (const file of fg.sync("**/🔣️.json", { cwd: corpusRoot, absolute: true })) {
    const value = JSON.parse(readFileSync(file, "utf8"));
    if (file.includes("/📸️snapshot/")) {
      assert(snapshotSchema(value) && childIdentity(value) && referenceIntegrity(value), file + ": snapshot oracle");
      assert.deepEqual(parseSnapshot(snapshotFixture(value)), snapshotFixture(value), file);
      snapshots++;
    } else if (file.includes("/🔺️diff/")) {
      assert(diffSchema(value), file + ": diff oracle");
      assert.deepEqual(parseDiff(diffFixture(value)), diffFixture(value), file);
      diffs++;
    } else if (file.includes("/🦠️mutation/")) {
      assert(mutationSchema(value) && mutationChildIdentity(value), file + ": mutation schema and child identity oracle");
      assert.deepEqual(parseMutation(mutationFixture(value)), mutationFixture(value), file);
      const mutationValue = value as Record<string, any>;
      const variant = Object.keys(mutationValue)[0]!;
      mutationVariants.add(variant);
      const subset = ({ CreateObject: "object", CreateModel: "model", CreateProperties: "value" } as Record<string, string>)[variant];
      if (subset) {
        const payload = mutationValue[variant];
        const alias = { ...mutationValue, [variant]: { ...payload, child_id: "local-alias" } };
        assert(mutationSchema(alias) && mutationChildIdentity(alias), file + ": independent local alias oracle");
        assert.deepEqual(parseMutation(mutationFixture(alias)), mutationFixture(alias), file + ": local alias parser");
        for (const invalid of [
          { ...mutationValue, [variant]: { ...payload, target: { ...payload.target, dialect: { ...payload.target.dialect, artifactKind: "other" } } } },
          { ...mutationValue, [variant]: { ...payload, target: { ...payload.target, dialect: { ...payload.target.dialect, standard: "v2" } } } },
          { ...mutationValue, [variant]: { ...payload, target: { ...payload.target, dialect: { ...payload.target.dialect, subset: subset === "object" ? "model" : "object" } } } },
        ]) {
          assert(!(mutationSchema(invalid) && mutationChildIdentity(invalid)), file + ": invalid child mutation oracle");
          assert.throws(() => parseMutation(mutationFixture(invalid)), file + ": invalid child mutation parser");
        }
      }
      const unknown = { ...mutationValue, [variant]: { ...mutationValue[variant], locale: "de" } };
      assert(!mutationSchema(unknown), file + ": mutation unknown-field schema oracle");
      assert.throws(() => parseMutation(mutationFixture(unknown)), file + ": mutation unknown-field parser");
      mutations++;
    }
  }
  assert.equal(snapshots, 30, "every committed Kit snapshot");
  assert.equal(diffs, 15, "every committed Kit diff");
  assert.equal(mutations, 16, "every committed Kit mutation, the set-snapshot wire witness included");
  assert.equal(mutationVariants.size, 16, "every typed mutation variant");
  assert(!mutationSchema({ Unknown: {} }), "unknown mutation variant schema oracle");
  assert.throws(() => parseMutation({ Unknown: {} }), "unknown mutation variant parser");
}
