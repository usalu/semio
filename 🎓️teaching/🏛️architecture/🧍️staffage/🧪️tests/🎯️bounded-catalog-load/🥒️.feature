@capability-staffage-bounded-catalog-load
Feature: A command or a suite reads only the catalog assets it names
  The catalog holds 774 MB of JSON, parsed about 4 GB, with up to 23 MB for one asset. A process that needs one figure must never parse the other documents, and a pass over every asset must hold one asset at a time.

  THE REFERENCE is a catalog folder whose other asset documents are not JSON at all: any read of them fails.

  Scenario: One asset of a catalog is loaded alone
    Given a catalog folder with the document of `tealight` and a second folder whose document is not JSON
    When the loader is asked for `tealight`
    Then it returns `tealight` and never opens the other document

  Scenario: An export command reads the asset it exports alone
    Given the same catalog folder
    When the drawing of `tealight` is prepared by the export command
    Then the command succeeds without opening the other document

  Scenario: A pass over the catalog is lazy
    Given the same catalog folder
    When the stream is advanced to its first asset and stopped
    Then no later document has been opened
