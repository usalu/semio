//! 🗣️ Remodel play app — the complete UI label set. ONE `app_labels!` block, never split (TEMPLATE §4).

use crate::editor::remodel::config::RemodelConfig;
use semio_framework_plugin::app_labels;

app_labels! {
    /// 🗣️ Complete UI label set for the remodel play app; one field per label makes every locale
    /// combination compile-checked. Native-only (no reuse-terminology variant): remodel's domain nouns
    /// (video/reconstruction/mesh/vertices/triangles) do not map onto the Object/Vortex/Attraction
    /// reuse vocabulary. House convention: no umlauts in German strings (ae/oe/ue/ss).
    pub struct RemodelLabels {
        model: native_en "Model", native_de "Modell", reuse_en "Model", reuse_de "Modell";
        capture: native_en "Capture", native_de "Aufnahme", reuse_en "Capture", reuse_de "Aufnahme";
        analyze: native_en "Analyze", native_de "Analyse", reuse_en "Analyze", reuse_de "Analyse";
        default_example: native_en "Default", native_de "Standard", reuse_en "Default", reuse_de "Standard";
        reconstruction: native_en "Reconstruction", native_de "Rekonstruktion", reuse_en "Reconstruction", reuse_de "Rekonstruktion";
        error: native_en "error", native_de "Fehler", reuse_en "error", reuse_de "Fehler";
        status: native_en "Status", native_de "Status", reuse_en "Status", reuse_de "Status";
        running: native_en "Running", native_de "Läuft", reuse_en "Running", reuse_de "Läuft";
        idle: native_en "Idle", native_de "Leerlauf", reuse_en "Idle", reuse_de "Leerlauf";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        vertices: native_en "vertices", native_de "Vertices", reuse_en "vertices", reuse_de "Vertices";
        triangles: native_en "triangles", native_de "Dreiecke", reuse_en "triangles", reuse_de "Dreiecke";
        streams: native_en "Streams", native_de "Streams", reuse_en "Streams", reuse_de "Streams";
        assets: native_en "Assets", native_de "Assets", reuse_en "Assets", reuse_de "Assets";
        no_streams: native_en "No media streams imported yet", native_de "Noch keine Medien-Streams importiert", reuse_en "No media streams imported yet", reuse_de "Noch keine Medien-Streams importiert";
        stream_kind_video: native_en "video", native_de "Video", reuse_en "video", reuse_de "Video";
        stream_kind_image_sequence: native_en "image sequence", native_de "Bildsequenz", reuse_en "image sequence", reuse_de "Bildsequenz";
        frames: native_en "frames", native_de "Frames", reuse_en "frames", reuse_de "Frames";
        sync_offset: native_en "sync offset", native_de "Sync-Versatz", reuse_en "sync offset", reuse_de "Sync-Versatz";
        sparse_cloud: native_en "Sparse point cloud", native_de "Dünne Punktwolke", reuse_en "Sparse point cloud", reuse_de "Dünne Punktwolke";
        dense_cloud: native_en "Dense point cloud", native_de "Dichte Punktwolke", reuse_en "Dense point cloud", reuse_de "Dichte Punktwolke";
        results_none: native_en "none", native_de "keine", reuse_en "none", reuse_de "keine";
        trajectory: native_en "Trajectory", native_de "Trajektorie", reuse_en "Trajectory", reuse_de "Trajektorie";
        poses: native_en "poses", native_de "Posen", reuse_en "poses", reuse_de "Posen";
        geo_products: native_en "Geo products", native_de "Geo-Produkte", reuse_en "Geo products", reuse_de "Geo-Produkte";
        available: native_en "available", native_de "verfügbar", reuse_en "available", reuse_de "verfügbar";
        params_ingest: native_en "Ingest", native_de "Ingest", reuse_en "Ingest", reuse_de "Ingest";
        params_feature: native_en "Feature", native_de "Feature", reuse_en "Feature", reuse_de "Feature";
        params_matching: native_en "Matching", native_de "Matching", reuse_en "Matching", reuse_de "Matching";
        params_sfm: native_en "SfM", native_de "SfM", reuse_en "SfM", reuse_de "SfM";
        params_dense: native_en "Dense", native_de "Dense", reuse_en "Dense", reuse_de "Dense";
        params_mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        params_motion: native_en "Motion", native_de "Bewegung", reuse_en "Motion", reuse_de "Bewegung";
        params_geo: native_en "Geo", native_de "Geo", reuse_en "Geo", reuse_de "Geo";
        stride_short: native_en "stride", native_de "Schrittweite", reuse_en "stride", reuse_de "Schrittweite";
        max_short: native_en "max", native_de "max", reuse_en "max", reuse_de "max";
        downscale_short: native_en "downscale", native_de "Verkleinerung", reuse_en "downscale", reuse_de "Verkleinerung";
        target_short: native_en "target", native_de "Ziel", reuse_en "target", reuse_de "Ziel";
        octaves_short: native_en "octaves", native_de "Oktaven", reuse_en "octaves", reuse_de "Oktaven";
        ratio_short: native_en "ratio", native_de "Verhältnis", reuse_en "ratio", reuse_de "Verhältnis";
        window_short: native_en "window", native_de "Fenster", reuse_en "window", reuse_de "Fenster";
        ransac_short: native_en "ransac", native_de "Ransac", reuse_en "ransac", reuse_de "Ransac";
        min_track_short: native_en "min track", native_de "min. Spur", reuse_en "min track", reuse_de "min. Spur";
        ba_short: native_en "ba", native_de "BA", reuse_en "ba", reuse_de "BA";
        voxel_short: native_en "voxel", native_de "Voxel", reuse_en "voxel", reuse_de "Voxel";
        enabled: native_en "enabled", native_de "aktiviert", reuse_en "enabled", reuse_de "aktiviert";
        disabled: native_en "disabled", native_de "deaktiviert", reuse_en "disabled", reuse_de "deaktiviert";
        cameras_calibrated: native_en "Calibrated cameras", native_de "Kalibrierte Kameras", reuse_en "Calibrated cameras", reuse_de "Kalibrierte Kameras";
        rig_extrinsics: native_en "Rig extrinsics", native_de "Rig-Extrinsik", reuse_en "Rig extrinsics", reuse_de "Rig-Extrinsik";
        gcps: native_en "Ground control points", native_de "Passpunkte", reuse_en "Ground control points", reuse_de "Passpunkte";
        tracks: native_en "Motion tracks", native_de "Bewegungsspuren", reuse_en "Motion tracks", reuse_de "Bewegungsspuren";
        tracks_none: native_en "No motion tracks", native_de "Keine Bewegungsspuren", reuse_en "No motion tracks", reuse_de "Keine Bewegungsspuren";
        motion_not_implemented: native_en "Motion tracking is not yet driven by the reconstruction engine", native_de "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt", reuse_en "Motion tracking is not yet driven by the reconstruction engine", reuse_de "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt";
        qc_none: native_en "No quality report yet", native_de "Noch kein Qualitätsbericht", reuse_en "No quality report yet", reuse_de "Noch kein Qualitätsbericht";
        qc_reprojection: native_en "Mean reprojection error", native_de "Mittlerer Reprojektionsfehler", reuse_en "Mean reprojection error", reuse_de "Mittlerer Reprojektionsfehler";
        qc_track_length: native_en "Mean track length", native_de "Mittlere Spurlänge", reuse_en "Mean track length", reuse_de "Mittlere Spurlänge";
        qc_registered_ratio: native_en "Registered frame ratio", native_de "Anteil registrierter Frames", reuse_en "Registered frame ratio", reuse_de "Anteil registrierter Frames";
        qc_dense_coverage: native_en "Dense coverage ratio", native_de "Dense-Abdeckungsanteil", reuse_en "Dense coverage ratio", reuse_de "Dense-Abdeckungsanteil";
        qc_gcp_rmse: native_en "GCP checkpoint RMSE", native_de "Passpunkt-Kontroll-RMSE", reuse_en "GCP checkpoint RMSE", reuse_de "Passpunkt-Kontroll-RMSE";
        qc_watertight: native_en "Watertight", native_de "Wasserdicht", reuse_en "Watertight", reuse_de "Wasserdicht";
        qc_boundary_edges: native_en "Boundary edges", native_de "Ränder", reuse_en "Boundary edges", reuse_de "Ränder";
        qc_components: native_en "Connected components", native_de "Zusammenhangskomponenten", reuse_en "Connected components", reuse_de "Zusammenhangskomponenten";
        qc_euler: native_en "Euler characteristic", native_de "Euler-Charakteristik", reuse_en "Euler characteristic", reuse_de "Euler-Charakteristik";
        qc_genus: native_en "Genus", native_de "Genus", reuse_en "Genus", reuse_de "Genus";
        qc_closed_fallback: native_en "Closed via fallback", native_de "Über Fallback geschlossen", reuse_en "Closed via fallback", reuse_de "Über Fallback geschlossen";
        panel_media: native_en "Media", native_de "Medien", reuse_en "Media", reuse_de "Medien";
        panel_pipeline: native_en "Pipeline", native_de "Pipeline", reuse_en "Pipeline", reuse_de "Pipeline";
        panel_results: native_en "Results", native_de "Ergebnisse", reuse_en "Results", reuse_de "Ergebnisse";
        panel_parameters: native_en "Parameters", native_de "Parameter", reuse_en "Parameters", reuse_de "Parameter";
        panel_calibration: native_en "Calibration", native_de "Kalibrierung", reuse_en "Calibration", reuse_de "Kalibrierung";
        panel_tracks: native_en "Tracks", native_de "Spuren", reuse_en "Tracks", reuse_de "Spuren";
        panel_qc: native_en "Quality", native_de "Qualität", reuse_en "Quality", reuse_de "Qualität";
        window_frames: native_en "Frames", native_de "Frames", reuse_en "Frames", reuse_de "Frames";
        window_report: native_en "Report", native_de "Bericht", reuse_en "Report", reuse_de "Bericht";
        layers: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        layer_mesh: native_en "Mesh", native_de "Mesh", reuse_en "Mesh", reuse_de "Mesh";
        layer_dense: native_en "Dense cloud", native_de "Dichte Punktwolke", reuse_en "Dense cloud", reuse_de "Dichte Punktwolke";
        layer_sparse: native_en "Sparse cloud", native_de "Dünne Punktwolke", reuse_en "Sparse cloud", reuse_de "Dünne Punktwolke";
        layer_cameras: native_en "Cameras", native_de "Kameras", reuse_en "Cameras", reuse_de "Kameras";
        layer_gcps: native_en "GCPs", native_de "Passpunkte", reuse_en "GCPs", reuse_de "Passpunkte";
    }
}

/// 🌐️ Resolves the label set for the config's current BCP-47 locale tag.
pub fn remodel_labels(cfg: &RemodelConfig) -> &'static RemodelLabels {
    semio_framework_plugin::resolve_labels_for_locale::<RemodelLabels>(&cfg.locale)
}
