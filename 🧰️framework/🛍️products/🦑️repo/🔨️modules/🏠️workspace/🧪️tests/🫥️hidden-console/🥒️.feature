Feature: Registered tools such as git open no console window on Windows
  Scenario: A registered tool command is hidden
    Given a command made for a registered tool
    When the workspace builds it
    Then Windows starts it with CREATE_NO_WINDOW and a hidden window, and Unix leaves it unchanged
