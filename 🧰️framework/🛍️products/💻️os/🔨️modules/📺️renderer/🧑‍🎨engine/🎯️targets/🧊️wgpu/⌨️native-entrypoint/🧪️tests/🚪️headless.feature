Feature: Native Headless Entry Point
  Scenario: Complete a retained I/O operation without a window host
    Given an explicit locale and terminology
    And a bounded JSON document on the local filesystem
    When the native process entry point drives a page-read future
    Then its bytes match the independent filesystem and JSON parser
    And all mounted I/O owners retire
  Scenario: Admit the native entry point's Windows stack
    Given the Windows native renderer producer
    When its executable is linked
    Then its main stack reserves 8388608 bytes
    And a smoke boot reports its selected application
  Scenario: Boot an application before a presentation host exists
    Given a retained page read submitted from the native application worker
    When the worker runs without presentation callbacks
    Then it completes the page read within the worker's bounded turns
    And its result matches the independent filesystem and JSON parser
  Scenario: Boot a kernel before a presentation host exists
    Given a retained page read submitted directly from the native kernel worker
    When the worker runs without application or presentation callbacks
    Then it completes the page read within the worker's bounded turns
    And its result matches the independent filesystem and JSON parser
  Scenario: Read the unoptimized local development component
    Given a locally produced component above the network catalog limit
    And the local development producer admits up to 268435456 bytes
    When the native kernel reads it in retained pages
    Then its exact bytes match the independent filesystem and JSON parser
    And a local input of 268435457 bytes is refused
    And network catalog admission remains bounded to 67108864 bytes
  Scenario: A selected plugin has no bootable application
    Given a valid local plugin descriptor without applications
    And an explicit language and terminology
    When the native smoke boots that selected plugin
    Then it reports booted false and exits with status 1
