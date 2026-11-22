//! 🎬️ Tutorial presentation laws over `🧫️fixtures/🎬️tutorial-presentation` and the ghost glyph asset table.

use semio_framework::onboarding::{IntroductionCursor, IntroductionGesture, IntroductionKeyModifier, IntroductionPoint, IntroductionPointerButton, TutorialAssetSrc, TutorialNarrationCue, TutorialOverlayRect, TutorialVideoCue};
use semio_renderer_wgpu_shell_surfaces::{TutorialMode, TutorialRuntime};
use semio_renderer_wgpu_shell_view_paint::{
    TUTORIAL_CAPTION_MAX_WIDTH_PX, TUTORIAL_GHOST_OPACITY, TUTORIAL_GHOST_SIZE_UI_SPACING, TutorialVideoOverlay, tutorial_caption_text, tutorial_chapter_fraction, tutorial_cue_index, tutorial_ghost_arc_point, tutorial_ghost_default_cursor, tutorial_ghost_glyph,
    tutorial_ghost_progress, tutorial_video_local_time_ms, tutorial_video_needs_reseek, tutorial_video_overlay, tutorial_video_rect,
};
use semio_ui_locale::{Locale, LocalizedLabel, Terminology};
use serde_json::Value;
use std::collections::HashMap;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🎬️tutorial-presentation/🔣️.json")).unwrap()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn runtime(playhead_ms: f64, mode: TutorialMode) -> TutorialRuntime {
    let mut definition = semio_framework::onboarding::TutorialDefinition {
        id: "presentation".into(),
        title: LocalizedLabel::data("Presentation"),
        description: None,
        duration_ms: 10_000,
        chapters: Vec::new(),
        base: semio_framework::onboarding::TutorialBase { document_dsl: None, example_id: None, ui: semio_framework::onboarding::TutorialUiSnapshot::default(), cameras: Vec::new() },
        tracks: semio_framework::onboarding::TutorialTracks::default(),
        recorded_at: None,
    };
    definition.tracks.narration.push(TutorialNarrationCue { id: "n1".into(), at: 1000, duration_ms: 2000, text: LocalizedLabel::native("Hello", "Hallo"), audio: None, voice: None, rate: 1.0, captions: Vec::new() });
    definition.tracks.video.push(TutorialVideoCue { at: 2000, duration_ms: 3000, src: TutorialAssetSrc::Url { url: "clip.mp4".into() }, rect: TutorialOverlayRect { x: 0.5, y: 0.5, width: 0.25, height: 0.25 }, muted: false, source_offset_ms: 400 });
    definition.tracks.video.push(TutorialVideoCue { at: 6000, duration_ms: 1000, src: TutorialAssetSrc::Blob { hash: "h".into(), size: 1, media_type: "video/mp4".into() }, rect: TutorialOverlayRect::default(), muted: false, source_offset_ms: 0 });
    TutorialRuntime {
        definition,
        mode,
        playhead_ms,
        rate: 1.5,
        muted: false,
        captions_on: true,
        applied_ms: playhead_ms,
        pre_sandbox_document_dsl: None,
        pre_sandbox_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        last_tick_wall_ms: 0.0,
        converge: HashMap::new(),
        recorder_last_camera_wall_ms: HashMap::new(),
        recorder_last_camera_pose: HashMap::new(),
        recorder_last_ui: semio_framework::onboarding::TutorialUiSnapshot::default(),
        recorder_last_ui_sample_wall_ms: 0.0,
    }
}

#[test]
fn cue_windows_are_half_open() {
    for case in fixture()["cueWindows"].as_array().unwrap() {
        let spans: Vec<(u64, u64)> = case["spans"].as_array().unwrap().iter().map(|span| (span[0].as_u64().unwrap(), span[1].as_u64().unwrap())).collect();
        assert_eq!(tutorial_cue_index(spans, number(&case["playheadMs"])).map(|index| index as u64), case["expect"].as_u64(), "{case}");
    }
}

#[test]
fn chapter_ticks_sit_at_their_share_of_the_timeline() {
    for case in fixture()["chapterFractions"].as_array().unwrap() {
        assert!((f64::from(tutorial_chapter_fraction(case["atMs"].as_u64().unwrap(), case["durationMs"].as_u64().unwrap())) - number(&case["expect"])).abs() < 1e-6, "{case}");
    }
}

#[test]
fn video_geometry_time_and_resync_follow_the_fixture() {
    let fixture = fixture();
    for case in fixture["videoRect"].as_array().unwrap() {
        let n = case["normalized"].as_array().unwrap();
        let rect = tutorial_video_rect([number(&n[0]), number(&n[1]), number(&n[2]), number(&n[3])], number(&case["viewport"][0]) as f32, number(&case["viewport"][1]) as f32);
        let expect = case["expect"].as_array().unwrap();
        assert_eq!([rect.x, rect.y, rect.w, rect.h].map(f64::from), [number(&expect[0]), number(&expect[1]), number(&expect[2]), number(&expect[3])], "{case}");
    }
    for case in fixture["videoLocalTime"].as_array().unwrap() {
        assert_eq!(tutorial_video_local_time_ms(number(&case["playheadMs"]), case["cueAtMs"].as_u64().unwrap(), case["sourceOffsetMs"].as_u64().unwrap()), number(&case["expect"]));
    }
    for case in fixture["videoReseek"].as_array().unwrap() {
        assert_eq!(tutorial_video_needs_reseek(number(&case["currentMs"]), number(&case["targetMs"])), case["expect"].as_bool().unwrap(), "{case}");
    }
}

#[test]
fn ghost_progress_and_arc_follow_the_fixture() {
    let fixture = fixture();
    for case in fixture["ghostProgress"].as_array().unwrap() {
        assert!((f64::from(tutorial_ghost_progress(number(&case["playheadMs"]), case["cueAtMs"].as_u64().unwrap(), case["durationMs"].as_u64().unwrap())) - number(&case["expect"])).abs() < 1e-6, "{case}");
    }
    for case in fixture["ghostArc"].as_array().unwrap() {
        let point = |value: &Value| (number(&value[0]) as f32, number(&value[1]) as f32);
        let (x, y) = tutorial_ghost_arc_point(point(&case["from"]), point(&case["to"]), number(&case["t"]) as f32);
        let expect = point(&case["expect"]);
        assert!((x - expect.0).abs() < 1e-3 && (y - expect.1).abs() < 1e-3, "{case}");
    }
    assert_eq!(f64::from(TUTORIAL_GHOST_OPACITY), number(&fixture["ghostOpacity"]).min(1.0) as f32 as f64);
    assert_eq!(f64::from(TUTORIAL_GHOST_SIZE_UI_SPACING), number(&fixture["ghostSizeUiSpacing"]));
    assert_eq!(f64::from(TUTORIAL_CAPTION_MAX_WIDTH_PX), number(&fixture["captionMaxWidthPx"]));
}

#[test]
fn the_default_ghost_cursor_follows_the_gesture_kind() {
    let at = || IntroductionPoint::Screen { x: 0.0, y: 0.0 };
    let (button, modifiers) = (IntroductionPointerButton::Left, Vec::<IntroductionKeyModifier>::new());
    let expected = &fixture()["defaultCursors"];
    let cursors = [
        ("leftClick", tutorial_ghost_default_cursor(&IntroductionGesture::LeftClick { at: at() })),
        ("rightClick", tutorial_ghost_default_cursor(&IntroductionGesture::RightClick { at: at() })),
        ("doubleClick", tutorial_ghost_default_cursor(&IntroductionGesture::DoubleClick { at: at() })),
        ("drag", tutorial_ghost_default_cursor(&IntroductionGesture::Drag { from: at(), to: at(), button, modifiers: modifiers.clone() })),
        ("scroll", tutorial_ghost_default_cursor(&IntroductionGesture::Scroll { at: at(), delta_y: 1.0 })),
        ("orbit", tutorial_ghost_default_cursor(&IntroductionGesture::Orbit { from: at(), to: at(), button, modifiers })),
    ];
    for (kind, cursor) in cursors {
        let name = match cursor {
            IntroductionCursor::Default => "default",
            IntroductionCursor::Pointer => "pointer",
            IntroductionCursor::Grab => "grab",
            IntroductionCursor::Grabbing => "grabbing",
            IntroductionCursor::Crosshair => "crosshair",
            IntroductionCursor::Move => "move",
        };
        assert_eq!(expected[kind].as_str().unwrap(), name, "{kind}");
    }
}

#[test]
fn captions_resolve_the_covering_cue_in_the_locale_and_honour_the_toggle() {
    let (terminology, en, de) = (Terminology::parse("native").unwrap(), Locale::parse("en").unwrap(), Locale::parse("de").unwrap());
    let playing = runtime(1500.0, TutorialMode::Playing);
    assert_eq!(tutorial_caption_text(&playing, terminology, en).as_deref(), Some("Hello"));
    assert_eq!(tutorial_caption_text(&playing, terminology, de).as_deref(), Some("Hallo"));
    assert_eq!(tutorial_caption_text(&runtime(3000.0, TutorialMode::Playing), terminology, en), None, "the window is half-open");
    assert_eq!(tutorial_caption_text(&runtime(500.0, TutorialMode::Playing), terminology, en), None);
    let mut hidden = runtime(1500.0, TutorialMode::Playing);
    hidden.captions_on = false;
    assert_eq!(tutorial_caption_text(&hidden, terminology, en), None);
    assert!(fixture()["defaults"]["captionsOn"].as_bool().unwrap() && !fixture()["defaults"]["muted"].as_bool().unwrap());
}

#[test]
fn the_video_overlay_tracks_the_cue_the_mute_and_the_transport() {
    let overlay = tutorial_video_overlay(&runtime(3000.0, TutorialMode::Playing), 1000.0, 400.0).unwrap();
    assert_eq!(overlay, TutorialVideoOverlay { rect: semio_ui_theme::geometry::Rect::new(500.0, 200.0, 250.0, 100.0), src: "clip.mp4".into(), muted: false, playing: true, rate: 1.5, local_time_ms: 1400.0 });
    assert!(!tutorial_video_overlay(&runtime(3000.0, TutorialMode::Paused), 1000.0, 400.0).unwrap().playing);
    let mut muted = runtime(3000.0, TutorialMode::Playing);
    muted.muted = true;
    assert!(tutorial_video_overlay(&muted, 1000.0, 400.0).unwrap().muted);
    assert_eq!(tutorial_video_overlay(&runtime(1000.0, TutorialMode::Playing), 1000.0, 400.0), None);
    assert_eq!(tutorial_video_overlay(&runtime(6500.0, TutorialMode::Playing), 1000.0, 400.0), None, "a blob source has no resolver in the shell chrome");
}

#[test]
fn the_ghost_glyph_table_equals_the_cursor_asset_fixture() {
    let assets: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🖼️assets/👆️cursor/👻️ghost-glyphs/🔣️.json")).unwrap();
    let glyphs = [("default", IntroductionCursor::Default), ("pointer", IntroductionCursor::Pointer), ("grab", IntroductionCursor::Grab), ("grabbing", IntroductionCursor::Grabbing), ("crosshair", IntroductionCursor::Crosshair), ("move", IntroductionCursor::Move)];
    for (appearance, dark) in [("light", false), ("dark", true)] {
        for (name, cursor) in glyphs {
            let expected = &assets["appearances"][appearance][name];
            let glyph = tutorial_ghost_glyph(cursor, dark);
            assert!((f64::from(glyph.width) - number(&expected["width"])).abs() < 1e-3 && (f64::from(glyph.height) - number(&expected["height"])).abs() < 1e-3, "{appearance} {name}");
            let polygons = expected["polygons"].as_array().unwrap();
            assert_eq!(glyph.polygons.len(), polygons.len(), "{appearance} {name}");
            for (polygon, expected) in glyph.polygons.iter().zip(polygons) {
                let hex = |value: &Value| {
                    let text = value.as_str().unwrap().trim_start_matches('#');
                    [0, 2, 4].map(|index| u8::from_str_radix(&text[index..index + 2], 16).unwrap())
                };
                assert_eq!(polygon.fill, hex(&expected["fill"]), "{appearance} {name}");
                assert_eq!(polygon.stroke, hex(&expected["stroke"]), "{appearance} {name}");
                let points = expected["points"].as_array().unwrap();
                assert_eq!(polygon.points.len(), points.len(), "{appearance} {name}");
                for (point, expected) in polygon.points.iter().zip(points) {
                    assert!((f64::from(point[0]) - number(&expected[0])).abs() < 1e-3 && (f64::from(point[1]) - number(&expected[1])).abs() < 1e-3, "{appearance} {name}");
                }
            }
        }
    }
}

#[test]
fn a_recording_adds_react_numbered_chapters_at_the_rounded_playhead() {
    for case in fixture()["recording"]["chapterCases"].as_array().unwrap() {
        let mut recording = runtime(number(&case["playheadMs"]), TutorialMode::Recording);
        for index in 0..case["existing"].as_u64().unwrap() {
            recording.definition.chapters.push(semio_framework::onboarding::TutorialChapter { id: format!("chapter-{}", index + 1), at: 0, title: LocalizedLabel::data("x"), body: None });
        }
        let chapter = recording.next_chapter().unwrap();
        assert_eq!((chapter.id.as_str(), chapter.at), (case["id"].as_str().unwrap(), case["at"].as_u64().unwrap()));
        for locale in ["en", "de"] {
            assert_eq!(chapter.title.resolve(Terminology::parse("native").unwrap(), Locale::parse(locale).unwrap()), case["title"].as_str().unwrap());
        }
    }
    assert!(runtime(0.0, TutorialMode::Playing).next_chapter().is_none(), "React's addChapter is a no-op without a recorder");
}

#[test]
fn the_record_controls_use_the_shared_en_de_labels() {
    for (key, words) in fixture()["recording"]["labels"].as_object().unwrap() {
        let key: &'static str = Box::leak(key.clone().into_boxed_str());
        assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_string(key, false), words["en"].as_str().unwrap(), "{key} en");
        assert_eq!(semio_renderer_wgpu_shell_vocab::shell_chrome_string(key, true), words["de"].as_str().unwrap(), "{key} de");
    }
}
