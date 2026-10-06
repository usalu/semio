Feature: Nx commands inherit the acquired workspace Bun runtime
  Scenario Outline: Acquired Bun precedes an installed runtime
    Given the declared Bun executable on <platform>
    When an Nx producer or source watcher starts
    Then the acquired executable directory leads its executable search path
    And ordinary caller environment values are preserved
    Examples:
      | platform |
      | win32 |
      | linux |
      | darwin |
