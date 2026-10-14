//! 🎛️ remodeling workflow commands: calibration, GCPs, parameters, clearing, view state and QC export.

extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "."]
        pub mod commands {
            #[path = "../../../🧿️add-gcp/🦀️.rs"]
            pub mod add_gcp;
            #[path = "../../../🔭️calibrate-cameras/🦀️.rs"]
            pub mod calibrate_cameras;
            #[path = "../../../☁️clear-dense/🦀️.rs"]
            pub mod clear_dense;
            #[path = "../../../🗾️clear-geo-products/🦀️.rs"]
            pub mod clear_geo_products;
            #[path = "../../../🧱️clear-mesh-result/🦀️.rs"]
            pub mod clear_mesh_result;
            #[path = "../../../🧹️clear-result/🦀️.rs"]
            pub mod clear_result;
            #[path = "../../../⭐️clear-sparse/🦀️.rs"]
            pub mod clear_sparse;
            #[path = "../../../🚂️clear-tracks/🦀️.rs"]
            pub mod clear_tracks;
            #[path = "../../../🛠️edit-calibration/🦀️.rs"]
            pub mod edit_calibration;
            #[path = "../../../🧾️export-qc-report/🦀️.rs"]
            pub mod export_qc_report;
            #[path = "../../../🔎️place-gcp-observation/🦀️.rs"]
            pub mod place_gcp_observation;
            #[path = "../../../🚮️remove-gcp/🦀️.rs"]
            pub mod remove_gcp;
            #[path = "../../../♻️reset-placeholder-mesh/🦀️.rs"]
            pub mod reset_placeholder_mesh;
            #[path = "../../../📷️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "../../../🌁️set-dense-params/🦀️.rs"]
            pub mod set_dense_params;
            #[path = "../../../🌠️set-feature-params/🦀️.rs"]
            pub mod set_feature_params;
            #[path = "../../../⏱️set-frame-cursor/🦀️.rs"]
            pub mod set_frame_cursor;
            #[path = "../../../🌐️set-geo-params/🦀️.rs"]
            pub mod set_geo_params;
            #[path = "../../../🥣️set-ingest-params/🦀️.rs"]
            pub mod set_ingest_params;
            #[path = "../../../👓️set-layer-visibility/🦀️.rs"]
            pub mod set_layer_visibility;
            #[path = "../../../🪢️set-match-params/🦀️.rs"]
            pub mod set_match_params;
            #[path = "../../../🕸️set-mesh-params/🦀️.rs"]
            pub mod set_mesh_params;
            #[path = "../../../🏎️set-motion-params/🦀️.rs"]
            pub mod set_motion_params;
            #[path = "../../../📊️set-report-table/🦀️.rs"]
            pub mod set_report_table;
            #[path = "../../../🧮️set-sfm-params/🦀️.rs"]
            pub mod set_sfm_params;
        }
    }
}
