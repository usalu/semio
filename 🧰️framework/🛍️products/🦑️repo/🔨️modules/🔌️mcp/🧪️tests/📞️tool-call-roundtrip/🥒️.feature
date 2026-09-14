@capability-mcp-tool-call-roundtrip
@oracle-modelcontextprotocol-sdk
@comparison-ordered-json-v1
Feature: A tool, a resource and a prompt round trip through the protocol
  The `2️⃣g2-contract.json` vectors pin the exact request and response of the whole call surface. The
  reference SDK server, given the same tool, resource and prompt, must answer the same requests with
  the same results and the same JSON-RPC error codes.

  @id-tool-resource-prompt-roundtrip
  @level-fundamental
  @mode-differential
  Scenario: The success vectors round trip unchanged
    Given the golden vectors shared://2️⃣g2-contract.json
    When each vector's request is dispatched against a ready session
    Then every implementation projects the same tool content, resource contents and prompt messages

  @id-protocol-error-vectors
  @level-quick
  @mode-error
  Scenario: An unknown method is refused with the standard code
    Given the golden vectors shared://2️⃣g2-contract.json
    When the unknown-method vector is dispatched against a ready session
    Then every implementation reports -32601 instead of answering

  # 📌️ The malformed-call-params vector is deliberately NOT part of the differential above. JSON-RPC 2.0
  # requires -32602 for a params object that fails the method's own shape, and both implementations
  # return it — the byte-exact `invalid-params` vector in `2️⃣g2-contract.json` pins that, and is asserted
  # by `go test` and `cargo test`. The reference SDK answers -32603 there, so comparing that vector
  # would encode the reference's own deviation as this repository's contract.
