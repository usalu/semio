use super::*;
use quote::quote;

#[semio_framework_async_macros::async_test]
async fn statechart_expands_minimal_machine_to_valid_rust() {
    let input = quote! {
        machine toggle {
            context: ToggleContext;
            event ToggleEvent { Flip }
            input: ();
            output: ();
            effect: ();
            context_from_input: build_context;
            initial: off;
            state off {
                on Flip => on;
            }
            state on {
                on Flip => off;
            }
        }
    };
    let expanded = expand_statechart(input).expect("expansion should succeed");
    syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
}

#[semio_framework_async_macros::async_test]
async fn statechart_rejects_duplicate_state_names() {
    let input = quote! {
        machine dup {
            context: Ctx;
            event Ev { Go }
            input: ();
            output: ();
            effect: ();
            context_from_input: build;
            initial: a;
            state a { on Go => b; }
            state a { on Go => b; }
        }
    };
    let err = expand_statechart(input).expect_err("duplicate state names must be rejected");
    assert!(err.to_string().contains("duplicate state name"));
}

#[semio_framework_async_macros::async_test]
async fn statechart_rejects_unknown_initial_state() {
    let input = quote! {
        machine bad_initial {
            context: Ctx;
            event Ev { Go }
            input: ();
            output: ();
            effect: ();
            context_from_input: build;
            initial: nonexistent;
            state a { on Go => a; }
        }
    };
    let err = expand_statechart(input).expect_err("unknown initial state must be rejected");
    assert!(err.to_string().contains("unknown initial state"));
}

#[semio_framework_async_macros::async_test]
async fn statechart_rejects_unknown_transition_target() {
    let input = quote! {
        machine bad_target {
            context: Ctx;
            event Ev { Go }
            input: ();
            output: ();
            effect: ();
            context_from_input: build;
            initial: a;
            state a { on Go => nowhere; }
        }
    };
    let err = expand_statechart(input).expect_err("unknown transition target must be rejected");
    assert!(err.to_string().contains("unknown transition target"));
}

#[semio_framework_async_macros::async_test]
async fn statechart_rejects_compound_without_initial() {
    let input = quote! {
        machine bad_compound {
            context: Ctx;
            event Ev { Go }
            input: ();
            output: ();
            effect: ();
            context_from_input: build;
            initial: a;
            state a {
                state nested { on Go => nested; }
            }
        }
    };
    let err = expand_statechart(input).expect_err("compound state without `initial:` must be rejected");
    assert!(err.to_string().contains("needs `initial:"));
}

#[semio_framework_async_macros::async_test]
async fn statechart_expands_hierarchical_parallel_history_machine() {
    let input = quote! {
        machine media {
            context: MediaContext;
            event MediaEvent { Open, Pause, Play, Stop, Start, AudioDone, VideoDone }
            input: ();
            output: Receipt;
            effect: ();
            context_from_input: build_media;
            output_from_context: make_receipt;
            initial: closed;
            state closed {
                on Open => open;
            }
            state open {
                initial: playing;
                entry on_open_enter;
                exit on_open_exit;
                invoke ticker;
                after 30000 => closed do on_timeout;
                history opened_history shallow;
                state playing {
                    on Pause => paused;
                }
                state paused {
                    on Play if can_resume => playing;
                }
                on Stop => closed;
            }
            parallel recording {
                state audio {
                    initial: audio_capturing;
                    state audio_capturing { on AudioDone => audio_done; }
                    final audio_done;
                }
                state video {
                    initial: video_capturing;
                    state video_capturing { on VideoDone => video_done; }
                    final video_done;
                }
                on_done => closed do on_recording_done;
            }
        }
    };
    let expanded = expand_statechart(input).expect("expansion of a hierarchical/parallel/history machine should succeed");
    syn::parse2::<syn::File>(quote! { #expanded }).expect("expanded code should parse as valid Rust items");
}

#[semio_framework_async_macros::async_test]
async fn derive_statechart_event_expands_mixed_variant_kinds() {
    let input: DeriveInput = syn::parse_quote! {
        enum Ev {
            Unit,
            Struct { field: i32 },
            Tuple(i32, i32),
        }
    };
    let expanded = expand_statechart_event(&input).expect("derive should succeed");
    syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
    let text = expanded.to_string();
    assert!(text.contains("EVENT_COUNT"));
    assert!(text.contains("3u16") || text.contains("3"));
}

#[semio_framework_async_macros::async_test]
async fn derive_statechart_event_rejects_non_enum() {
    let input: DeriveInput = syn::parse_quote! {
        struct NotAnEnum;
    };
    let err = expand_statechart_event(&input).expect_err("deriving on a struct must be rejected");
    assert!(err.to_string().contains("enums"));
}

#[semio_framework_async_macros::async_test]
async fn derive_statechart_schema_expands_named_fields() {
    let input: DeriveInput = syn::parse_quote! {
        struct Ctx {
            order_id: String,
            attempts: u32,
        }
    };
    let expanded = expand_statechart_schema(&input).expect("derive should succeed");
    syn::parse2::<syn::File>(quote! { #expanded }).expect("derived impl should parse as valid Rust");
    let text = expanded.to_string();
    assert!(text.contains("SCHEMA_JSON"));
    assert!(text.contains("order_id"));
}
