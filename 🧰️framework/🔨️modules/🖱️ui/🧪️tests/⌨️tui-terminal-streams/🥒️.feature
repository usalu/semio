@capability-tui-terminal-screen
@oracle-pyte
@comparison-screen-grid-v1
Feature: A byte stream a child writes leaves the screen grid a reference VT emulator shows
  The dashboard's embedded terminal feeds everything a child writes through its own VT parser: cursor addressing,
  erase and insert/delete families, scroll regions, tab stops, origin mode, saved cursors, UTF-8 and wide
  characters, charset and control strings. Nothing here is checked against its own reading: pyte (a Python VT100
  and xterm screen emulator) replays each stream, and the rows (trailing blanks trimmed) and the cursor it ends
  on must equal what the Rust screen shows.

  The streams are the ones `shared://🔣️.json` pins for every language twin. A null cursor marks streams that end in
  a pending wrap, where pyte reports column == width and xterm reports the last column, so only the grid is
  compared there. Streams on which pyte departs from xterm (NEL without carriage return, a wide character that is
  split across the right margin, CUP outside the origin-mode region) are not in the fixture; the Rust unit tests
  pin the xterm behaviour for them.

  @id-streams-reproduce-pyte-screens
  @level-fundamental
  @mode-differential
  Scenario: Every shared stream shows the same rows and cursor in pyte and in the Rust screen
    Given every case of `shared://🔣️.json` with its columns, rows and byte stream
    When pyte feeds the stream into a screen of that size and the Rust screen is fed the same bytes
    Then the rows, trailing blanks trimmed, are equal and the cursor is equal wherever the case pins one
