//! 🏟️ The test-owned `Ui`: its serialized UI arena scope and the retirement it owes inside it.
use super::{Ui, UiLayoutStep, UiSurfaceCloseStep, UiSurfaceToken, UI_LAYOUT_SURFACE_SLOTS};
use semio_ui_contract_values::testkit::UiArenaFixture;
use std::ops::{Deref, DerefMut};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// 🏟️ Owns one `Ui` together with the serialized [`UiArenaFixture`] its retained documents live in; dropping it first pays every window's surface close and the contract page close under that scope's own Source, so no law can forget the scope or leak a document past it. One per test: the scope is process-serialized, so a test mounts its `Ui` windows in one fixture and drops it before opening another.
pub struct UiFixture {
    ui: Ui,
    arena: UiArenaFixture,
}

impl UiFixture {
    pub fn new(locale: semio_ui_locale::Locale) -> Self {
        let arena = UiArenaFixture::new();
        Self { ui: Ui::new(locale), arena }
    }
}

impl Deref for UiFixture {
    type Target = Ui;
    fn deref(&self) -> &Ui {
        &self.ui
    }
}

impl DerefMut for UiFixture {
    fn deref_mut(&mut self) -> &mut Ui {
        &mut self.ui
    }
}

impl UiFixture {
    /// 🧵️ Runs one real `step_layouts` opportunity inside one funded turn of this scope's own original caller, the Source every window's layout worker admission binds to.
    pub fn step_layouts(&mut self, pool: &semio_framework_async::WorkerPool, atlas: &mut semio_ui_text::FontAtlas) -> UiLayoutStep {
        let Self { ui, arena } = self;
        Self::layout_turn(ui, arena, pool, atlas)
    }

    fn layout_turn(ui: &mut Ui, arena: &mut UiArenaFixture, pool: &semio_framework_async::WorkerPool, atlas: &mut semio_ui_text::FontAtlas) -> UiLayoutStep {
        arena.turn(|cx| Ok(ui.step_layouts(pool, atlas, cx))).expect("the fixture layout turn is funded")
    }

    /// 📐️ Drives `window_id`'s real layout queue under this scope's own original caller until the window is clean, answering whether it settled inside the finite opportunity ceiling; a refused layout fails the law with its refusal.
    pub fn settle_layout(&mut self, window_id: &str, pool: &semio_framework_async::WorkerPool, atlas: &mut semio_ui_text::FontAtlas) -> bool {
        let Self { ui, arena } = self;
        let mut last = String::new();
        for _ in 0..262_144 {
            if ui.layout_is_dirty(window_id) {
                ui.request_layout(window_id);
            }
            let step = Self::layout_turn(ui, arena, pool, atlas);
            let label = format!("{step:?}");
            if label != last {
                eprintln!("[DEBUG] settle_layout {window_id}: {label} dirty={} {} {}", ui.layout_is_dirty(window_id), ui.paint_stall_census(window_id), arena.debug_remaining());
                last = label;
            }
            match step {
                UiLayoutStep::Refused { refusal, .. } => panic!("{window_id}: layout refused: {refusal:?}"),
                UiLayoutStep::Idle if !ui.layout_is_dirty(window_id) => return true,
                _ => {}
            }
        }
        false
    }

    fn close(&mut self) {
        let Self { ui, arena } = self;
        let tokens: Vec<UiSurfaceToken> = (0..UI_LAYOUT_SURFACE_SLOTS).filter_map(|index| ui.windows.token_at(index)).collect();
        for token in tokens {
            assert!(ui.begin_surface_close(token));
            let mut closed = false;
            for _ in 0..262_144 {
                match arena.turn(|cx| ui.close_surface_one(token, cx)).expect("the fixture surface close is funded") {
                    UiSurfaceCloseStep::Complete => {
                        closed = true;
                        break;
                    }
                    UiSurfaceCloseStep::Refused(refusal) => panic!("the fixture surface close was refused: {refusal:?}"),
                    UiSurfaceCloseStep::Pending | UiSurfaceCloseStep::RetiredScene(_) => {}
                }
            }
            assert!(closed, "the fixture surface close finishes inside its finite opportunity ceiling");
        }
        assert!((0..262_144).any(|_| ui_contract::close_ui_document_page_one()), "the fixture contract pages close inside their finite opportunity ceiling");
    }
}

impl Drop for UiFixture {
    fn drop(&mut self) {
        if std::thread::panicking() {
            let _ = catch_unwind(AssertUnwindSafe(|| self.close()));
            return;
        }
        self.close()
    }
}
