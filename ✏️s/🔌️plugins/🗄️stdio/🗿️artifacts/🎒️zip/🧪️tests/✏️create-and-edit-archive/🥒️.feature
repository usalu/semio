@capability-zip-archive
@oracle-zip
@comparison-semantic-archive-v1
Feature: Create and edit a ZIP archive
  The reference implementation writes the archive; this repository decodes it, optionally edits the
  member list, and re-encodes. Both results are read back by the INDEPENDENT reader before the
  `semantic-archive-v1` profile compares them as a SET of members.

  Entry order, compression method and level, timestamps, external attributes and the extra field are
  writer choices. Member names, uncompressed sizes and content digests are normative.

  @id-three-members-round-trip
  @level-quick
  @mode-round-trip
  Scenario: Every member survives decode and re-encode
    Given the archive
    """
    { "entries": [
        { "name": "a.txt", "content": "alpha" },
        { "name": "nested/b.txt", "content": "beta beta beta" },
        { "name": "c.bin", "content": "0123456789" }
    ] }
    """
    When the archive is written and read back
    Then every member name, size and content digest is unchanged

  @id-removing-a-member
  @level-quick
  @mode-differential
  Scenario: Removing one member leaves the others untouched
    Given the archive
    """
    { "entries": [
        { "name": "a.txt", "content": "alpha" },
        { "name": "nested/b.txt", "content": "beta beta beta" },
        { "name": "c.bin", "content": "0123456789" }
    ], "remove": "nested/b.txt" }
    """
    When the named member is removed
    Then the reference implementation and this repository agree on the remaining members

  @id-empty-archive-round-trips
  @level-fundamental
  @mode-round-trip
  Scenario: An archive with a single empty member round trips
    Given the archive
    """
    { "entries": [ { "name": "empty.txt", "content": "" } ] }
    """
    When the archive is written and read back
    Then every member name, size and content digest is unchanged
