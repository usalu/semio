import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";

/** 🧬️ Uses independent JSON schema admission and exact neutral/owner source boundaries. */
export async function testCredentialProtocolSourceV1(root: string): Promise<void> {
  const general = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️agent-credential");
  const schema = JSON.parse(readFileSync(join(general, "🧬️schema/🔣️.json"), "utf8"));
  const corpus = JSON.parse(readFileSync(join(general, "🧫️fixtures/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true });
  assert.ok(ajv.compile(schema)(corpus));
  const request = ajv.compile(schema.$defs.request);
  for (const vector of corpus.vectors) assert.equal(request({ path: vector.path, bodyBytes: 0 }), vector.valid);
  const source = readFileSync(join(general, "🦀️.rs"), "utf8");
  assert.ok(source.includes("pub struct CredentialExchangeProtocolV1"));
  assert.ok(!source.includes("semio.hub.agent-credential/v1"));
  assert.ok(!source.includes("delegation.v1."));
  const remote = readFileSync(join(general, "../🏠️workspace/🔗️remote/🦀️.rs"), "utf8");
  assert.ok(!remote.includes("/auth/agent-sessions"));
  const authSchema = JSON.parse(readFileSync(join(root, "🌎️hub/🔐️auth/🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(root, "🌎️hub/🔐️auth/🔌️client/🧫️fixtures/🔣️.json"), "utf8"));
  const validator = new Ajv({ strict: false }); validator.addSchema(authSchema);
  for (const [field, definition] of [["credential", "AgentCredentialFileV1"], ["request", "AgentSessionRequestV1"], ["grant", "AgentSessionMintResponseV1"]]) assert.ok(validator.getSchema(`${authSchema.$id}#/$defs/${definition}`)!(fixture[field]));
  for (const [document, vectors, definition] of [["credential", "credentialVectors", "AgentCredentialFileV1"], ["grant", "grantVectors", "AgentSessionMintResponseV1"]]) {
    const admit = validator.getSchema(`${authSchema.$id}#/$defs/${definition}`)!;
    for (const vector of fixture[vectors]) assert.equal(admit({ ...fixture[document], [vector.field]: vector.value }), vector.valid);
  }
  const owned = readFileSync(join(root, "🌎️hub/🔐️auth/🔌️client/🦀️.rs"), "utf8");
  assert.ok(owned.includes("/auth/agent-sessions"));
  const genericEntry = readFileSync(join(general, "../🏗️bootstrap/⌨️entrypoint/🦀️.rs"), "utf8");
  assert.ok(genericEntry.includes("Vec::new(), Vec::new()"));
  const entry = readFileSync(join(root, "✏️s/🧑‍💻dev/💡️services/🌉️mcp/⌨️entrypoint/🦀️.rs"), "utf8");
  assert.ok(entry.includes("semio_hub_auth_client::hub_agent_credential_protocol_v1()"));
}
