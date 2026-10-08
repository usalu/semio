mod capability_tests {
    use super::*;
    use crate::tui::geometry::Pos;
    use crate::tui::widget::{CursorShape, CursorSpec};

    const PATCH: &str = "\x1b[3;4H\x1b[0;38;2;255;0;0;48;2;0;0;0mX";

    fn block(x: u16, y: u16) -> CursorSpec {
        CursorSpec { pos: Pos { x, y }, shape: CursorShape::Block, blink: false }
    }

    #[test]
    fn presenter_degrades_colours_and_places_the_cursor_after_the_patch() {
        let capabilities = Capabilities { color: ColorDepth::Ansi256, synchronized_output: false, unicode: UnicodeLevel::Full };
        let mut presenter = FramePresenter::new(capabilities);
        assert_eq!(presenter.capabilities(), capabilities);
        let frame = presenter.compose(&AnsiPatch(PATCH.into()), Some(block(1, 2)));
        assert_eq!(frame, "\x1b[3;4H\x1b[0;38;5;196;48;5;16mX\x1b[2 q\x1b[3;2H\x1b[?25h");
    }

    #[test]
    fn presenter_wraps_frames_in_synchronized_output_and_stays_silent_when_nothing_changes() {
        let capabilities = Capabilities { color: ColorDepth::TrueColor, synchronized_output: true, unicode: UnicodeLevel::Full };
        let mut presenter = FramePresenter::new(capabilities);
        let frame = presenter.compose(&AnsiPatch(PATCH.into()), None);
        assert_eq!(frame, format!("\x1b[?2026h{PATCH}\x1b[?2026l"));
        assert_eq!(presenter.compose(&AnsiPatch::default(), None), "");
        presenter.compose(&AnsiPatch::default(), Some(block(0, 0)));
        assert_eq!(presenter.compose(&AnsiPatch::default(), Some(block(0, 0))), "");
        presenter.reset();
        assert!(!presenter.compose(&AnsiPatch::default(), Some(block(0, 0))).is_empty(), "a re-entered terminal needs the cursor written again");
    }

    #[test]
    fn clipboard_payloads_are_validated_before_they_reach_the_terminal() {
        assert!(!osc52_payload_check("").unwrap());
        assert!(osc52_payload_check("hello").unwrap());
        assert_eq!(osc52_payload_check(&"x".repeat(OSC52_LIMIT + 1)).unwrap_err().kind(), io::ErrorKind::InvalidInput);
        assert!(osc52_payload_check(&"x".repeat(OSC52_LIMIT)).unwrap());
        assert_eq!(osc52_copy_sequence("hé"), "\x1b]52;c;aMOp\x07");
    }
}
