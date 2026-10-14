//! 📥️ remodeling ingest commands: frame, video and stream import, abort and the example picker.

extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "."]
        pub mod commands {
            #[path = "../../../🌱️add-stream/🦀️.rs"]
            pub mod add_stream;
            #[path = "../../../🛑️import-abort/🦀️.rs"]
            pub mod import_abort;
            #[path = "../../../🖼️import-frame-payload/🦀️.rs"]
            pub mod import_frame_payload;
            #[path = "../../../🎞️import-frames/🦀️.rs"]
            pub mod import_frames;
            #[path = "../../../🎥️import-video/🦀️.rs"]
            pub mod import_video;
            #[path = "../../../💽️import-video-bytes-payload/🦀️.rs"]
            pub mod import_video_bytes_payload;
            #[path = "../../../✅️import-video-done/🦀️.rs"]
            pub mod import_video_done;
            #[path = "../../../📼️import-video-frame-payload/🦀️.rs"]
            pub mod import_video_frame_payload;
            #[path = "../../../🪓️remove-stream/🦀️.rs"]
            pub mod remove_stream;
            #[path = "../../../🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "../../../🔄️set-stream-sync/🦀️.rs"]
            pub mod set_stream_sync;
        }
    }
}
