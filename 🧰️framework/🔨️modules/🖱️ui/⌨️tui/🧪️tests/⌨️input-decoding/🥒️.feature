Feature: Terminal Input Decoding And Output Degradation
  The terminal target decodes raw terminal bytes into keys, pointer reports, pastes and focus changes
  and degrades painted frames to what the attached terminal renders. The shared fixture lists every
  byte stream with the events it must produce and every environment with the capabilities it must
  detect; the Rust adapter, a Node readline oracle, a Python prompt_toolkit oracle, a TextDecoder
  oracle, a specification oracle, the ncurses terminfo database and the color-convert library all
  read the same file.

  Scenario: Keys decode the same in CSI, SS3 and modified forms
    Given the shared input decoding vectors
    When arrow, home, end, function and tilde keys arrive as CSI, SS3 and modified sequences
    Then every vector projects to the listed key and modifier events
    And Node readline and Python prompt_toolkit independently decode the same events for the vectors they support

  Scenario: Numeric keypad keys keep their identity
    Given keypad keys arrive as application keypad SS3 codes or as kitty keypad code points
    When the parser decodes them
    Then every key is a keypad key and a text widget sees the character or Enter it types

  Scenario: Control bytes and Alt prefixes keep their identity
    Given a carriage return, a line feed, ctrl-letters, ctrl-punctuation, ESC prefixed characters and a double ESC
    When the parser has decoded them and the escape timeout has passed
    Then Enter and Ctrl+J are different keys and Alt applies to Enter, Backspace, ctrl-letters and UTF-8 characters
    And two ESC bytes are two Escape keys

  Scenario: Terminal replies never type into the application
    Given device attribute, mode report, cursor report and OSC replies arrive between ordinary keys
    When the parser decodes them
    Then only the ordinary keys are emitted and both OSC terminators end the reply

  Scenario: Pointer reports cover every button, motion, wheel and encoding
    Given SGR, X10 and urxvt mouse encodings
    When buttons are pressed, dragged, released, moved without a button and the wheel turns in four directions
    Then motion without a button is a Move, buttons above the third are ignored and positions are zero based
    And a specification oracle independently produces the same pointer events

  Scenario: Consecutive presses count as double and triple clicks
    Given presses of one button on one cell
    When they arrive within 400 milliseconds of each other
    Then the click count rises to three and starts again, and a different cell or button restarts it

  Scenario: Bracketed paste is decoded once as UTF-8 and capped
    Given a bracketed paste containing multi-byte text, line breaks and escape bytes
    When the parser decodes it in one feed or byte by byte
    Then exactly one paste event carries the exact text and a TextDecoder produces the same text
    And a paste beyond the limit is cut on a character boundary and a paste that never ends is delivered on timeout

  Scenario: Capabilities follow the terminal environment
    Given COLORTERM, TERM, TERM_PROGRAM, WT_SESSION, KITTY_WINDOW_ID, VTE_VERSION, NO_COLOR and the locale variables
    When the backend detects what the attached terminal can do
    Then colour depth, glyph repertoire and synchronized output equal the shared table
    And the ncurses terminfo database independently reproduces the depth of every TERM it describes

  Scenario: Colours degrade to the terminal depth
    Given truecolor frames and the capabilities detected from the environment
    When a terminal supports 256 colours, 16 colours or none
    Then the frame uses palette indices that the color-convert library independently produces
    And NO_COLOR, a dumb terminal or a non-UTF-8 locale selects the plain repertoire

  Scenario: Cursor and synchronized output frame every repaint
    Given a cursor request and a patch
    When the frame is presented
    Then the cursor is hidden while painting, then positioned, shaped and shown, and repeated frames write nothing
    And terminals that support synchronized output receive the frame inside one update
