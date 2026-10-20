Feature: Published Native Directory Device Namespace
  Scenario: Observe the same persisted directory after kernel device reassignment
    Given the published directory and backing belonged to one device namespace
    And the current Original retains the exact anchored backing and kernel mapping
    When the receiver binds published directory data to the current backing
    Then its inode and mode remain exact
    And its device is the current backing device
    And the binding grants no physical ownership and rewrites no publication
  Scenario: Refuse a directory from another published namespace
    Given the published directory device differs from its published backing device
    When a current directory binding is requested
    Then the binding is refused

  Scenario: Preserve production receiver guards during namespace binding
    Given the actual receiver under a genuine invocation and current publication
    When its published device differs from the anchored current device
    Then the inactive original directory is admitted without receipt writes
    But foreign live devices, old formats, changed ancestry, changed backing, changed receipt and wrong kernel mappings refuse
    And original inventory cancellation propagates unchanged

Scenario: Retire Current Publications Across Device Namespace Changes
  Given the eight closed retirement namespace and mutation cases
  When each of the four actual retirement functions is evaluated with real filesystem fixtures and unit kernel ports
  Then device reassignment preserves published inode and mode
  And changed ancestry, backing, receipt, mount directory and original cancellation refuse before removal
