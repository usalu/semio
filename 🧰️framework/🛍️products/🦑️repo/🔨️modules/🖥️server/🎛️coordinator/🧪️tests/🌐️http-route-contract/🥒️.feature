@capability-repo-coordinator-http-route
@no-oracle-repo-coordinator-http-protocol
@comparison-ordered-json-v1
Feature: The coordinator HTTP surface answers every route identically in both implementations
  The repo CLI talks to the coordinator over one route table and nothing else, so a status code or a
  body that differs between the Go and the Rust server is a protocol break for every client. Each
  adapter starts its own server on an ephemeral port, replays the committed request fixture
  `shared://🌐️http-route-contract/🌐️requests.json` in order against it, and projects the status and the body of every
  answer with the volatile timestamp fields the fixture names blanked out.

  @id-every-route-answers-the-same-status-and-body
  @level-fundamental
  @mode-differential
  Scenario: Replaying the fixture in order yields one status and one body per request
    Given the request fixture shared://🌐️http-route-contract/🌐️requests.json
    When each implementation starts its own coordinator on an ephemeral port and issues the requests in order
    Then every request answers with the same status and the same body once the volatile fields are blanked

  @id-a-method-mismatch-and-a-missing-field-are-refused
  @level-fundamental
  @mode-error
  Scenario: A wrong method and a missing required field are refused the same way
    Given the request fixture shared://🌐️http-route-contract/🌐️requests.json
    When the refusing requests of the fixture are issued
    Then a method mismatch answers 405 with an empty body and a missing required field answers 400 with the same error text
