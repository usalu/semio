@capability-mcp-initialize-handshake
@oracle-modelcontextprotocol-sdk
@comparison-ordered-json-v1
Feature: The MCP initialize handshake accepts every conformant client
  A params object is open for extension. Real clients send `clientInfo.title`, capability objects the
  server has never heard of, and `_meta` keys; a server that rejects them fails the handshake with
  -32602 and is unusable. The reference implementation of the protocol — Anthropic's TypeScript SDK —
  is the oracle: an SDK server answering the same handshake must project the same result as ours.
  A client may also pipeline `initialize` and `notifications/initialized` without waiting in between,
  so the notification can reach the session while the request that promotes it is still in flight; a
  server that drops the notification in that window answers -32002 for everything afterwards. The same
  client may then close its input as soon as the burst is written — `printf … | server` and every
  short-lived script does — so end of input must mean "no more requests", never "forget the requests
  already delivered". A burst is a single write, so the requests that follow `initialize` on the wire
  must be answered in the phase that `initialize` and the notification ahead of them established: a
  server that hands them to a worker pool without a handshake latch lets one overtake the promotion
  and refuses it with -32002 even though the client sequenced it correctly.

  @id-initialize-accepts-unknown-members
  @level-fundamental
  @mode-differential
  Scenario: A handshake carrying unknown members succeeds
    Given the initialize vector shared://🤝️initialize-lenient.json
    When the client initializes the server with that vector's extra client members
    Then every implementation reports the handshake accepted with the same server capabilities

  @id-initialize-and-initialized-pipelined
  @level-fundamental
  @mode-differential
  Scenario: A pipelined initialized notification still leaves the session usable
    Given the initialize vector shared://🤝️initialize-lenient.json
    When the client pipelines the initialized notification ahead of the initialize response
    Then every implementation answers a following ping instead of reporting an uninitialized session

  @id-initialize-echoes-a-supported-version
  @level-fundamental
  @mode-conformance
  Scenario: A supported protocol version is echoed back unchanged
    Given the initialize vector shared://🤝️initialize-lenient.json
    When the client asks for the protocol version the vector declares
    Then every implementation echoes exactly that version instead of substituting its own

  @id-pipelined-burst-serves-requests-after-initialized
  @level-fundamental
  @mode-differential
  Scenario: A request that follows the initialized notification in one burst is served
    Given the initialize vector shared://🤝️initialize-lenient.json
    When the client writes initialize, the initialized notification and a request as one burst
    Then every implementation serves that request instead of refusing it as an uninitialized session

  @id-eof-after-burst-completes-queued-requests
  @level-fundamental
  @mode-differential
  Scenario: A burst whose input ends immediately still answers every request it delivered
    Given the initialize vector shared://🤝️initialize-lenient.json
    When the client writes the whole burst and ends its input before any reply is read
    Then every implementation answers both request ids and refuses none of them as a closed session
