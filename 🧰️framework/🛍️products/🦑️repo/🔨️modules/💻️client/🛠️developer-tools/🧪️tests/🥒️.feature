Feature: Supervised Developer Debugging
  Scenario: The LLDB profile debugs a native workspace program
    Given an installed LLDB and C compiler
    And the language-neutral LLDB runtime fixture
    When the supervised debugger launches the fixture with one spaced argument
    Then a breakpoint stops at main
    And argc is two
    And the first argument retains its exact spaced value
    And the debugger evaluates six times seven as forty-two
    And continuing runs the program to exit status zero
    And the original cancellation signal controls compilation and debugging

  Scenario: Native debugger setup uses fixed owned host package plans
    Given the typed LLDB setup control and platform manager vectors
    When the installed frontend evaluates42 through its Python batch interface
    Then setup performs no installation
    And missing native tooling uses the existing OS privilege contract
    And matching Windows LLVM and Python runtimes have explicit installation paths
    And only LLDB child work uses that matching Python environment
    And the same Original cancellation and deadline govern provisioning and verification
