Feature: Native OS MCP staging ingress
  Scenario Outline: Both staging ports retain the native command and caller environment
    Given the <platform> native repo installation
    When the synchronous or asynchronous OS MCP staging port builds its declared owner
    Then it invokes repo run @semio-tech/framework-os-mcp-rs:build --output=stream
    And the caller environment and working directory are preserved
    And asynchronous cancellation retains the supplied signal
    And child output is reported only through the progress observer
    Examples:
      | platform |
      | darwin   |
      | linux    |
      | win32    |
