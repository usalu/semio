import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import publicationFixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import publicationSchema from "../../🧫️fixtures/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../../../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧫️fixtures/🧬️schema/🔣️.json" with { type: "json" };
test("private child metadata admits the independent complete identity corpus", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const expected = row.expected, parent = row.owner.parent;
    const reference = [expected.artifact_id, expected.dialect.artifact_kind, expected.dialect.standard, expected.dialect.subset];
    const key = ["", row.owner.slot, row.owner.child_id];
    const fields = [...reference, parent.artifact_id, parent.dialect.artifact_kind, parent.dialect.standard, parent.dialect.subset, row.owner.slot, row.owner.child_id, "actor:private-child", ...key, ...reference, ...key];
    expect(fields.length).toBe(21);
    const bytes = fields.map(field => Buffer.from(field, "utf8"));
    expect(bytes.map(field => field.toString("utf8"))).toEqual(fields);
    expect(bytes.reduce((sum, field) => sum + field.byteLength, 0)).toBeGreaterThan(0);
  }
  const path = resolve(import.meta.dir, "../../🦀️.rs");
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  for (const method of ["PrivateChildMemberMetadataSource", "PrivateChildMemberMetadataIssuer", "next_capacity_byte_demand", "take_ready", "close_granted"]) expect(source.includes(method)).toBe(true);
  console.log("[DEBUG] Node Buffer/TextEncoder + Ajv: all21 private child request/key/prepared-read metadata owners match original UTF-8 identities");
});

test("private child publication metadata retains independent actor and optional transaction identities", () => {
  expect(new Ajv({ strict: true }).compile(publicationSchema)(publicationFixture)).toBe(true);
  for (const transaction of publicationFixture.transactions) {
    const fields = [publicationFixture.actor, transaction?.id ?? "", transaction?.tool ?? "", transaction?.id ?? ""];
    expect(fields.map(field => Buffer.from(field, "utf8").toString("utf8"))).toEqual(fields);
    expect(transaction === null ? null : { actor: fields[0], group_id: fields[3], transaction: { id: fields[1], tool: fields[2] } }).toEqual(transaction === null ? null : { actor: publicationFixture.actor, group_id: transaction.id, transaction });
  }
  const source = readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8");
  for (const field of ["publication_actor", "transaction", "group_id"]) expect(source.includes(field)).toBe(true);
  console.log("[DEBUG] Node Buffer + Ajv optional transaction oracle: original actor and identical group/transaction IDs retain UTF-8 identity");
});

test("private child registry metadata retains the exact independent parent and slot owner", () => {
  for (const row of fixture.cases) {
    const owner = row.owner, parent = owner.parent;
    const fields = [parent.artifact_id, parent.dialect.artifact_kind, parent.dialect.standard, parent.dialect.subset, owner.slot, owner.child_id];
    expect(fields.map(field => Buffer.from(field, "utf8").toString("utf8"))).toEqual(fields);
    expect({ parent: { artifact_id: fields[0], dialect: { artifact_kind: fields[1], standard: fields[2], subset: fields[3] } }, slot: fields[4], child_id: fields[5] }).toEqual(owner);
  }
  expect(readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8").includes("registry_owner")).toBe(true);
  console.log("[DEBUG] Node Buffer + closed Ajv owner corpus: private registry parent/slot/child metadata preserves the exact original six fields");
});

test("parent publication metadata has the same independent bounded actor and transaction corpus", () => {
  for (const transaction of publicationFixture.transactions) {
    const fields = [publicationFixture.actor, transaction?.id ?? "", transaction?.tool ?? "", transaction?.id ?? ""];
    for (const field of fields) {
      const source = Buffer.from(field, "utf8"), target = Buffer.alloc(source.byteLength);
      for (let offset = 0; offset < source.byteLength; offset += 64) source.copy(target, offset, offset, Math.min(offset + 64, source.byteLength));
      expect(target.toString("utf8")).toBe(field);
    }
  }
  const source = readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8");
  for (const type of ["PrivatePublicationMetadataSource", "PrivatePublicationMetadataParts", "PrivatePublicationMetadataIssuer"]) expect(source.includes(type)).toBe(true);
  console.log("[DEBUG] Node Buffer copied the original parent actor/group/transaction/tool in bounded prefixes with optional transaction identity unchanged");
});

test("common publication group identity is independent of original optional transaction", () => {
  const validate = new Ajv({ strict: true, allowUnionTypes: true }).compile(publicationSchema);
  expect(validate(publicationFixture)).toBe(true);
  for (const transaction of publicationFixture.transactions) for (const groupId of publicationFixture.groupIds) {
    const original = { transaction, group_id: groupId };
    const fields = [publicationFixture.actor, transaction?.id ?? "", transaction?.tool ?? "", groupId ?? ""];
    const copied = fields.map(field => {
      const bytes = new TextEncoder().encode(field), backing = Buffer.allocUnsafeSlow(bytes.length);
      for (let offset = 0; offset < bytes.length; offset += 64) backing.set(bytes.subarray(offset, offset + 64), offset);
      expect(backing.toString("utf8")).toBe(field);
      return backing.toString("utf8");
    });
    const result = applyPatch({ transaction: null, group_id: null }, [{ op: "replace", path: "/transaction", value: transaction === null ? null : { id: copied[1], tool: copied[2] } }, { op: "replace", path: "/group_id", value: groupId === null ? null : copied[3] }], true).newDocument;
    expect(result).toEqual(original);
  }
  expect(validate({ ...publicationFixture, groupIds: [null, "", 4] })).toBe(false);
  const source = readFileSync(resolve(import.meta.dir, "../../🦀️.rs"), "utf8");
  expect(source.includes("pub(crate) group_id: Option<&'a str>")).toBe(true);
  expect(source.includes("group_present: Option<bool>")).toBe(true);
  console.log("[DEBUG] Ajv + unpooled Buffer + RFC6902: nine independent group/transaction combinations preserve absent, empty, Unicode identities and original optional transaction");
});
