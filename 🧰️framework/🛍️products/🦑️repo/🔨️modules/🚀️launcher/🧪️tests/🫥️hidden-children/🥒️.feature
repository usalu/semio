Feature: Background children of the launcher open no console window on Windows
  Scenario: A captured helper is hidden
    Given a helper whose output the launcher captures
    When the launcher prepares it
    Then Windows starts it with CREATE_NO_WINDOW and a hidden window, and Unix leaves it unchanged
  Scenario: A runtime that shares the console of an interactive launcher keeps it
    Given a launcher that owns a console window
    When it starts the runtime as a child that shares the console
    Then the runtime is not detached from that console
