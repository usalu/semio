@capability-tui-terminal-keys
@oracle-terminfo-xterm-256color
@comparison-key-bytes-v1
Feature: Every key the embedded terminal forwards is spelled the way the terminal's own definition promises
  A child that runs under TERM=xterm-256color reads keys through the strings its terminfo entry names: cursor keys
  in application mode, Home and End, Insert and Delete, Page keys, F1 to F63 (F13 and up are the modified F1 to F12),
  the modified cursor and editing keys with their modifier parameter, the numeric keypad in application mode and
  the focus reports. The ncurses terminfo database is the reference: `infocmp -x -1 xterm-256color` names the bytes,
  the Rust encoder must produce exactly them, with no tolerance.

  The cases are the ones `shared://🔣️.json` pins for every language twin; each carries the capability name, the key
  with its modifiers, the child modes in force and the bytes terminfo lists.

  @id-keys-encode-to-terminfo-bytes
  @level-fundamental
  @mode-differential
  Scenario: Each terminfo key capability is produced from its key and modes
    Given every case of `shared://🔣️.json` with a key, modifiers and child modes
    When `infocmp -x -1 xterm-256color` lists the capability and the Rust encoder encodes the key under those modes
    Then the encoded bytes equal the terminfo string for the capability
