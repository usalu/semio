//! 🔬️ Sparse stages of the reconstruction engine: the feature extraction, pair matching and verification, track building, pose estimation and bundle adjustment steps with their preparations. See [`super`].

use super::*;

// #region 🔖️Pipeline
/// 🗻️ Pyramid levels the bounded ORB detection spreads its keypoints over.
const FEATURE_PYRAMID_LEVELS: usize = 3;

/// 🔁️ Upper bound on loop-closure candidate pairs a frame is matched against beyond its window.
const LOOP_CLOSURE_PARTNERS_PER_FRAME: usize = 24;

/// 🎯️ One phase of the bundle stage.
#[derive(Clone, Copy, Debug)]
enum BundleStagePhase {
    /// 🧹️ Prune points whose worst reprojection exceeds `tolerance` × the RANSAC threshold (or
    /// that fewer than two registered cameras see), then retriangulate.
    Cleanup { tolerance: f64 },
    /// 🌐️ Global adjustment; `Some(factor)` runs a Huber kernel at `factor` × the RANSAC threshold,
    /// `None` the document's configured loss.
    Adjust { huber_factor: Option<f64> },
}

/// 🎯️ The bundle stage's phases in order (see `step_bundle_adjusting`).
const BUNDLE_STAGE_PLAN: [BundleStagePhase; 5] = [
    BundleStagePhase::Cleanup { tolerance: f64::INFINITY },
    BundleStagePhase::Adjust { huber_factor: Some(16.0) },
    BundleStagePhase::Cleanup { tolerance: 12.0 },
    BundleStagePhase::Adjust { huber_factor: None },
    BundleStagePhase::Cleanup { tolerance: 3.0 },
];

/// 🎯️ Cameras the local bundle adjustment after each registration refines (the newest ones, in
/// registration order) and the iterations it spends. Without it the poses drift by about a degree
/// per registered view on the orbit fixture — 20° by the twenty-second camera and a scale that
/// doubled — because every registration inherits the errors of the points it was solved against.
const LOCAL_BUNDLE_WINDOW: usize = 5;

const LOCAL_BUNDLE_ITERATIONS: usize = 5;

/// ⏱️ Residual terms one bundle-adjustment unit accumulates (each is ~20 reprojections).
const BUNDLE_TERMS_PER_STEP: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum FeaturePhase {
    Luma,
    Detect,
    Describe,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct FeaturePreparation {
    pub(super) frame: usize,
    pub phase: FeaturePhase,
    pub cursor: usize,
    pub(super) gray: Vec<f32>,
    /// 🧭️ The resumable detector, alive through [`FeaturePhase::Detect`].
    pub detection: Option<remodeling_feature::BoundedDetectionPreparation>,
    /// 🗻️ The detector's pyramid, kept through [`FeaturePhase::Describe`] for the descriptors.
    pyramid: Option<remodeling_image::Pyramid>,
    pub(super) keypoints: Vec<remodeling_feature::Keypoint>,
    descriptors: Vec<remodeling_feature::Descriptor256>,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct PairMatchPreparation {
    pub(super) frame_a: usize,
    pub(super) frame_b: usize,
    pub query: usize,
    pub candidate: usize,
    best_distance: u32,
    best_index: u32,
    second_distance: u32,
    reverse_candidate: usize,
    reverse_best_distance: u32,
    reverse_best_index: u32,
    pending: Option<(u32, u32)>,
    pub(super) matches: Vec<remodeling_feature::Match>,
    /// 🧭️ Geometric verification of the matched pair, once the descriptor matching is complete:
    /// hypothesis batches spent so far and the best essential-matrix estimate they produced.
    verification: Option<PairVerification>,
}

/// 🧭️ Running state of one pair's geometric verification.
#[derive(semio_framework_value::RetireOwned)]
struct PairVerification {
    correspondences: Vec<([f64; 2], [f64; 2])>,
    attempts: usize,
    /// The best polished model so far, by explained correspondences.
    pub(super) best: Option<semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::TwoViewResult>,
}

/// 🧭️ Hypotheses one verification call draws (each a five-point solve — a 10×10 elimination and a
/// degree-ten root finding, ~2 ms in a debug build — scored over the pair, then polished for
/// ~0.7 ms). One per unit: two draws measured 3.8 ms at best, and under the host's scheduling
/// jitter that unit overran the 8 ms law in runs the watchdog would quarantine.
const PAIR_VERIFICATION_HYPOTHESES_PER_STEP: usize = 1;

/// 🧭️ Calls one pair's verification spends drawing hypotheses before it commits the best model.
const PAIR_VERIFICATION_STEPS: usize = 64;

/// 🧭️ Sampson tolerance of the verification, in pixels; divided by the focal length into the
/// normalized coordinates the solver scores in, so a 96-pixel frame is judged as leniently as a
/// 320-pixel one. Tight on purpose: at 2 px a fifth of a 10° pair's kept matches were still wrong
/// and the registration chain drifted 20° around the orbit; at 1 px the kept matches are 98–100 %
/// true and a legitimate pair still keeps dozens.
const PAIR_VERIFICATION_THRESHOLD_PX: f64 = 1.0;

/// 🧭️ Fewest inliers a pair keeps, and the share of its matches they must make up; below either
/// the pair's matches are all dropped as noise. A five-point model fits five of any eight random
/// matches and picks up a few more by chance, so eight of eight is what a 60° pair of unrelated
/// views answers; a pair that shares a view answers dozens at well over half.
const PAIR_VERIFICATION_MIN_INLIERS: usize = 12;

const PAIR_VERIFICATION_MIN_INLIER_SHARE: f64 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum TrackPhase {
    Union,
    Group,
    Finish,
}

#[derive(semio_framework_value::RetireOwned)]
struct TrackGroup {
    pub(super) observations: Vec<(usize, u32)>,
    pub(super) frames: std::collections::BTreeSet<usize>,
    conflict: bool,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct TrackPreparation {
    pub phase: TrackPhase,
    pub pair: usize,
    pub matched: usize,
    /// Sized once for every observation the pair table can name (see [`Self::with_capacity`]): a
    /// table growing past 2¹⁵ keypoint nodes rehashed every one of them inside a single bounded union
    /// step (10 ms on the adversarial law's 200 000-match pair), and an ordered map instead spends as
    /// long freeing its nodes in the step that completes the tracks.
    node_of: std::collections::HashMap<(usize, u32), usize>,
    pub(super) observations: Vec<(usize, u32)>,
    parent: Vec<usize>,
    rank: Vec<u8>,
    /// 🎞️ Per root: bitmask of the frames its set already observes (the engine admits at most 64
    /// frames). A union whose two sets share a frame is refused — that match would put two
    /// keypoints of one frame in one track, which is a wrong match by definition — so a wrong
    /// match can no longer fuse two good tracks into a conflicting group that is then thrown away.
    pub(super) frames: Vec<u64>,
    pub grouping_cursor: usize,
    groups: std::collections::BTreeMap<usize, TrackGroup>,
    pub(super) tracks: Vec<Vec<(usize, u32)>>,
}

impl TrackPreparation {
    /// 🧮️ A preparation whose node table and per-node vectors hold `observations` entries without
    /// ever growing: two per match at most. The reservation is address space the union steps then
    /// touch page by page, so no step pays a whole-table rehash or vector doubling.
    pub(super) fn with_capacity(observations: usize) -> Self {
        Self {
            phase: TrackPhase::Union,
            pair: 0,
            matched: 0,
            node_of: std::collections::HashMap::with_capacity(observations),
            observations: Vec::with_capacity(observations),
            parent: Vec::with_capacity(observations),
            rank: Vec::with_capacity(observations),
            frames: Vec::with_capacity(observations),
            grouping_cursor: 0,
            groups: std::collections::BTreeMap::new(),
            tracks: Vec::new(),
        }
    }

    fn node(&mut self, observation: (usize, u32)) -> usize {
        if let Some(&node) = self.node_of.get(&observation) {
            return node;
        }
        let node = self.observations.len();
        self.node_of.insert(observation, node);
        self.observations.push(observation);
        self.parent.push(node);
        self.rank.push(0);
        self.frames.push(1u64 << (observation.0 as u32).min(63));
        node
    }

    fn root(&mut self, mut node: usize) -> usize {
        while self.parent[node] != node {
            let parent = self.parent[node];
            self.parent[node] = self.parent[parent];
            node = self.parent[node];
        }
        node
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left = self.root(left);
        let mut right = self.root(right);
        if left == right || self.frames[left] & self.frames[right] != 0 {
            return;
        }
        if self.rank[left] < self.rank[right] {
            std::mem::swap(&mut left, &mut right);
        }
        self.parent[right] = left;
        self.frames[left] |= self.frames[right];
        if self.rank[left] == self.rank[right] {
            self.rank[left] = self.rank[left].saturating_add(1);
        }
    }
}

fn descriptor_distance(a: &remodeling_feature::Descriptor256, b: &remodeling_feature::Descriptor256) -> u32 {
    a.0.iter().zip(b.0.iter()).map(|(&left, &right)| (left ^ right).count_ones()).sum()
}

pub(super) fn append_luma_slice(source: &remodeling_image::ImageRgba8, destination: &mut remodeling_image::ImageGray, cursor: &mut usize, pixel_budget: usize) -> bool {
    let pixels = source.width as usize * source.height as usize;
    let end = cursor.saturating_add(pixel_budget.max(1)).min(pixels);
    for index in *cursor..end {
        let offset = index * 4;
        destination.data.push((0.299 * f32::from(source.data[offset]) + 0.587 * f32::from(source.data[offset + 1]) + 0.114 * f32::from(source.data[offset + 2])) / 255.0);
    }
    *cursor = end;
    end == pixels
}

fn reset_pair_query(preparation: &mut PairMatchPreparation) {
    preparation.query += 1;
    preparation.candidate = 0;
    preparation.best_distance = u32::MAX;
    preparation.best_index = u32::MAX;
    preparation.second_distance = u32::MAX;
    preparation.reverse_candidate = 0;
    preparation.reverse_best_distance = u32::MAX;
    preparation.reverse_best_index = u32::MAX;
    preparation.pending = None;
}

impl ReconstructionEngine {
    /// 🎯️ One fuel-bounded luma/detect/describe slice; returns whether more frames remain.
    /// 🧭️ One frame's features in fuel-bounded slices: luma conversion by pixel runs, the resumable
    /// ORB/Harris detection ([`remodeling_feature::BoundedDetectionPreparation`]) by row bands, then
    /// rBRIEF description a few keypoints at a time; returns whether frames remain.
    pub fn step_extracting_features(&mut self) -> bool {
        const PIXELS_PER_STEP: usize = 4_096;
        const DETECTION_PIXELS_PER_STEP: usize = 512;
        const DESCRIPTORS_PER_STEP: usize = 4;
        if self.cursor >= self.frames.len() {
            return false;
        }
        let target_feature_count = self.params.target_feature_count.min(512);
        if self.feature_preparation.is_none() {
            let pixels = self.frames[self.cursor].image.width as usize * self.frames[self.cursor].image.height as usize;
            self.feature_preparation = Some(FeaturePreparation {
                frame: self.cursor,
                phase: FeaturePhase::Luma,
                cursor: 0,
                gray: Vec::with_capacity(pixels),
                detection: None,
                pyramid: None,
                keypoints: Vec::with_capacity(target_feature_count),
                descriptors: Vec::with_capacity(target_feature_count),
            });
        }
        let preparation = self.feature_preparation.as_mut().expect("feature preparation");
        let image = &self.frames[preparation.frame].image;
        match preparation.phase {
            FeaturePhase::Luma => {
                let pixels = image.width as usize * image.height as usize;
                let end = preparation.cursor.saturating_add(PIXELS_PER_STEP).min(pixels);
                for index in preparation.cursor..end {
                    let offset = index * 4;
                    preparation.gray.push((0.299 * f32::from(image.data[offset]) + 0.587 * f32::from(image.data[offset + 1]) + 0.114 * f32::from(image.data[offset + 2])) / 255.0);
                }
                preparation.cursor = end;
                if end == pixels {
                    let base = remodeling_image::ImageGray { width: image.width, height: image.height, data: std::mem::take(&mut preparation.gray) };
                    preparation.detection = Some(remodeling_feature::BoundedDetectionPreparation::new(base, FEATURE_PYRAMID_LEVELS, target_feature_count, self.params.detector, self.params.upright_descriptors));
                    preparation.phase = FeaturePhase::Detect;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Detect => {
                // ⏱️ One band is about DETECTION_PIXELS_PER_STEP pixels of the level: each of the
                // streamed passes costs a few dozen operations per pixel, FAST + suppression a
                // few hundred per corner.
                let rows = (DETECTION_PIXELS_PER_STEP / (image.width as usize).max(1)).max(1);
                let detection = preparation.detection.as_mut().expect("bounded detection");
                if detection.advance(rows) {
                    let (pyramid, keypoints) = preparation.detection.take().expect("completed detection").finish();
                    preparation.pyramid = Some(pyramid);
                    preparation.keypoints = keypoints;
                    preparation.phase = FeaturePhase::Describe;
                    preparation.cursor = 0;
                }
            }
            FeaturePhase::Describe => {
                let end = preparation.cursor.saturating_add(DESCRIPTORS_PER_STEP).min(preparation.keypoints.len());
                let pyramid = preparation.pyramid.as_ref().expect("detector pyramid");
                preparation.descriptors.extend(remodeling_feature::describe_orb(pyramid, &preparation.keypoints[preparation.cursor..end]));
                preparation.cursor = end;
                if end == preparation.keypoints.len() {
                    let complete = self.feature_preparation.take().expect("completed feature preparation");
                    self.record(EngineObservation::FeaturesExtracted { frame: complete.frame, keypoints: complete.keypoints.len() });
                    self.keypoints_per_frame.push(complete.keypoints);
                    self.descriptors_per_frame.push(complete.descriptors);
                    self.cursor += 1;
                }
            }
        }
        self.cursor < self.frames.len()
    }

    /// 🕸️ Sequential-window pair list `(i, j)` for `j in (i, i + sequential_window]`, built once when
    /// entering [`EngineStage::MatchingFeatures`].
    /// 🕸️ Sequential-window pairs `(i, j)` for `j in (i, i + sequential_window]`, plus an explicit
    /// `(0, f)`/`(1, f)` "anchor" pair for every later frame `f`: `IncrementalSfm::init_pair` triangulates
    /// only tracks directly spanning both anchor frames, so without a direct anchor↔f pair a later frame's
    /// `register_next` PnP can only find 2D-3D correspondences through incidental semio_hub-chained tracks —
    /// sparse enough on real matches to starve most frames of the 6 correspondences PnP needs. Explicit
    /// anchor pairs make every registerable frame's correspondence-to-the-seed-pair direct instead of
    /// coincidental.
    pub(super) fn step_build_match_pairs(&mut self, pair_budget: usize) -> bool {
        let n = self.frames.len();
        let window = self.params.sequential_window.max(1);
        let mut work = 0;
        while self.match_pair_i < n && work < pair_budget {
            let hi = self.match_pair_i.saturating_add(window).min(n.saturating_sub(1));
            if self.match_pair_j <= hi {
                self.match_pairs.push((self.match_pair_i, self.match_pair_j));
                self.match_pair_j += 1;
                work += 1;
            } else {
                self.match_pair_i += 1;
                self.match_pair_j = self.match_pair_i.saturating_add(1);
            }
        }
        // 🔁️ Loop-closure candidates beyond the window: every frame against every `stride`-th later
        // frame, at most ~24 per frame. A candidate that does not share a view (40°–180° apart on
        // the orbit fixture) is dropped whole by the pair's geometric verification, so the cost of a
        // miss is one bounded match + solve; a hit — the orbit's last views against its first — is
        // the constraint that lets the bundle adjustment close a loop instead of accumulating a
        // sequential chain's drift (6° over the fixture's 36 views without it). The old `(0, f)` /
        // `(1, f)` anchors are gone: with real descriptors those pairs were noise, and every wrong
        // match unioned two good tracks into one no triangulation could validate.
        let stride = (n.saturating_sub(window)).div_ceil(LOOP_CLOSURE_PARTNERS_PER_FRAME).max(2);
        while self.match_pair_i >= n && self.match_anchor_frame < n && work < pair_budget {
            let frame = self.match_anchor_frame;
            let mut partner = frame + window + 1;
            while partner < n {
                if (partner - frame) % stride == 0 {
                    self.match_pairs.push((frame, partner));
                    work += 1;
                }
                partner += 1;
            }
            self.match_anchor_frame += 1;
        }
        self.match_pair_i >= n && self.match_anchor_frame >= n
    }

    /// 🤝️ Consumes at most `COMPARISONS_PER_STEP` Hamming comparisons across a resumable pair match.
    pub fn step_matching_features(&mut self) -> bool {
        // ⏱️ A debug-profile unit of 2 048 comparisons measured 8.2–9.5 ms against the 8 ms ceiling
        // (`every_bounded_unit_stays_under_the_interactive_ceiling_on_every_example`); half that
        // leaves the headroom the law wants.
        const COMPARISONS_PER_STEP: usize = 1_024;
        if self.pair_cursor >= self.match_pairs.len() {
            return false;
        }
        if self.pair_match_preparation.is_none() {
            let (frame_a, frame_b) = self.match_pairs[self.pair_cursor];
            self.pair_match_preparation = Some(PairMatchPreparation {
                frame_a,
                frame_b,
                query: 0,
                candidate: 0,
                best_distance: u32::MAX,
                best_index: u32::MAX,
                second_distance: u32::MAX,
                reverse_candidate: 0,
                reverse_best_distance: u32::MAX,
                reverse_best_index: u32::MAX,
                pending: None,
                matches: Vec::new(),
                verification: None,
            });
        }
        let preparation = self.pair_match_preparation.as_mut().expect("pair match preparation");
        if preparation.query == self.descriptors_per_frame[preparation.frame_a].len() {
            return self.step_verifying_pair();
        }
        let desc_a = &self.descriptors_per_frame[preparation.frame_a];
        let desc_b = &self.descriptors_per_frame[preparation.frame_b];
        if desc_b.is_empty() {
            preparation.query = preparation.query.saturating_add(COMPARISONS_PER_STEP).min(desc_a.len());
            if preparation.query == desc_a.len() {
                let complete = self.pair_match_preparation.take().expect("completed empty pair match");
                self.record(EngineObservation::PairMatched { frame_a: complete.frame_a, frame_b: complete.frame_b, matches: 0 });
                self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
                self.pair_cursor += 1;
            }
            return self.pair_cursor < self.match_pairs.len();
        }
        let mut remaining = COMPARISONS_PER_STEP;
        let observations = &mut self.recorded;
        while remaining > 0 && preparation.query < desc_a.len() {
            if let Some((distance, best_index)) = preparation.pending {
                while remaining > 0 && preparation.reverse_candidate < desc_a.len() {
                    let candidate = preparation.reverse_candidate;
                    let back_distance = descriptor_distance(&desc_b[best_index as usize], &desc_a[candidate]);
                    if back_distance < preparation.reverse_best_distance {
                        preparation.reverse_best_distance = back_distance;
                        preparation.reverse_best_index = candidate as u32;
                    }
                    preparation.reverse_candidate += 1;
                    remaining -= 1;
                }
                if preparation.reverse_candidate == desc_a.len() {
                    let (frame_a, frame_b, query) = (preparation.frame_a, preparation.frame_b, preparation.query as u32);
                    if preparation.reverse_best_index == query {
                        preparation.matches.push(remodeling_feature::Match { a: query, b: best_index, distance });
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchAccepted { frame_a, frame_b, query, candidate: best_index, distance });
                        }
                    } else if let Some(recorded) = observations.as_mut() {
                        recorded.push(EngineObservation::MatchCrossCheckRejected { frame_a, frame_b, query, candidate: best_index });
                    }
                    reset_pair_query(preparation);
                }
                continue;
            }
            while remaining > 0 && preparation.candidate < desc_b.len() {
                let candidate = preparation.candidate;
                let distance = descriptor_distance(&desc_a[preparation.query], &desc_b[candidate]);
                if distance < preparation.best_distance {
                    preparation.second_distance = preparation.best_distance;
                    preparation.best_distance = distance;
                    preparation.best_index = candidate as u32;
                } else if distance < preparation.second_distance {
                    preparation.second_distance = distance;
                }
                preparation.candidate += 1;
                remaining -= 1;
            }
            if preparation.candidate == desc_b.len() {
                let passes = preparation.best_index != u32::MAX && (preparation.second_distance == u32::MAX || (preparation.best_distance as f32) < self.params.match_ratio * preparation.second_distance as f32);
                let (frame_a, frame_b, query) = (preparation.frame_a, preparation.frame_b, preparation.query as u32);
                if passes && self.params.match_mutual {
                    preparation.pending = Some((preparation.best_distance, preparation.best_index));
                    preparation.reverse_candidate = 0;
                    preparation.reverse_best_distance = u32::MAX;
                    preparation.reverse_best_index = u32::MAX;
                } else {
                    if passes {
                        preparation.matches.push(remodeling_feature::Match { a: query, b: preparation.best_index, distance: preparation.best_distance });
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchAccepted { frame_a, frame_b, query, candidate: preparation.best_index, distance: preparation.best_distance });
                        }
                    } else if preparation.best_index != u32::MAX {
                        if let Some(recorded) = observations.as_mut() {
                            recorded.push(EngineObservation::MatchRatioRejected { frame_a, frame_b, query, best: preparation.best_distance, second: preparation.second_distance });
                        }
                    }
                    reset_pair_query(preparation);
                }
            }
        }
        if preparation.query == desc_a.len() {
            return self.step_verifying_pair();
        }
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🧭️ Geometric verification of the pair whose descriptor matching just completed: a bounded
    /// five-point RANSAC over the matched keypoints, one hypothesis batch per call, and only the
    /// best model's inliers survive into the pair table. Descriptor matching alone leaves a fifth
    /// to a quarter of a 10° pair's matches wrong (and nearly all of a 40° pair's); every wrong
    /// match that reaches the track builder either fuses two good tracks or plants a wrong
    /// observation in one, and a registration drawing on such tracks found 3 consistent
    /// correspondences out of 20. A pair without keypoints (a descriptor-only test harness) or with
    /// fewer than [`PAIR_VERIFICATION_MIN_INLIERS`] matches skips the solve and keeps its matches.
    fn step_verifying_pair(&mut self) -> bool {
        let preparation = self.pair_match_preparation.as_mut().expect("pair match preparation");
        let verifiable = preparation.matches.len() >= PAIR_VERIFICATION_MIN_INLIERS && self.keypoints_per_frame.len() > preparation.frame_a.max(preparation.frame_b) && !self.frames.is_empty();
        if verifiable {
            let intrinsics = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio, self.params.distortion);
            let (frame_a, frame_b) = (preparation.frame_a, preparation.frame_b);
            let verification = preparation.verification.get_or_insert_with(|| PairVerification {
                correspondences: preparation
                    .matches
                    .iter()
                    .map(|matched| {
                        let a = self.keypoints_per_frame[frame_a][matched.a as usize];
                        let b = self.keypoints_per_frame[frame_b][matched.b as usize];
                        ([f64::from(a.x), f64::from(a.y)], [f64::from(b.x), f64::from(b.y)])
                    })
                    .collect(),
                attempts: 0,
                best: None,
            });
            let seed = ((frame_a as u64) << 32 | frame_b as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ verification.attempts as u64;
            let threshold = PAIR_VERIFICATION_THRESHOLD_PX / intrinsics.fx.max(intrinsics.fy).max(1.0);
            if verification.attempts < PAIR_VERIFICATION_STEPS {
                if let Some(candidate) = semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::estimate_essential_five_point_bounded(&verification.correspondences, &intrinsics, &intrinsics, threshold, seed, PAIR_VERIFICATION_HYPOTHESES_PER_STEP, 0) {
                    // A raw five-point fit through five integer-pixel keypoints explains only a
                    // handful of correspondences at a pixel's tolerance, and its raw MSAC score
                    // does not rank the true model above a wrong one reliably enough to polish only
                    // improvements: on a marginal pair (two thirds true matches) the true draws
                    // lost to earlier wrong ones by raw score and the pair was dropped whole. So
                    // every unit polishes its batch's best (the unit's one costlier part) and the
                    // comparison is between refined models, by explained correspondences; the
                    // survivor set at the end is the refined one.
                    let polished = semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::polish_essential(candidate, &verification.correspondences, &intrinsics, threshold);
                    if verification.best.as_ref().is_none_or(|best| polished.inliers.len() > best.inliers.len()) {
                        verification.best = Some(polished);
                    }
                }
                verification.attempts += 1;
                return true;
            }
            let survivors: Vec<usize> = verification.best.as_ref().map(|best| best.inliers.clone()).unwrap_or_default();
            let mut kept = std::collections::BTreeSet::new();
            kept.extend(survivors);
            let before = preparation.matches.len();
            if kept.len() >= PAIR_VERIFICATION_MIN_INLIERS && kept.len() as f64 >= PAIR_VERIFICATION_MIN_INLIER_SHARE * before as f64 {
                let mut index = 0usize;
                preparation.matches.retain(|_| {
                    let keep = kept.contains(&index);
                    index += 1;
                    keep
                });
            } else {
                preparation.matches.clear();
            }
            let dropped = before - preparation.matches.len();
            if dropped > 0 {
                self.record(EngineObservation::PairGeometryRejected { frame_a, frame_b, dropped });
            }
        }
        let complete = self.pair_match_preparation.take().expect("completed pair match");
        self.record(EngineObservation::PairMatched { frame_a: complete.frame_a, frame_b: complete.frame_b, matches: complete.matches.len() });
        self.pairwise_matches.push((complete.frame_a, complete.frame_b, complete.matches));
        self.pair_cursor += 1;
        self.pair_cursor < self.match_pairs.len()
    }

    /// 🧵️ Consumes at most 4,096 union/group observations or 64 finalized groups.
    pub fn step_build_tracks(&mut self) -> Option<semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::FeatureTracks> {
        // ⏱️ A union is two hashed node lookups; a thousand per step keeps a debug-profile step
        // inside the worker law with room for the host's scheduling jitter.
        const OBSERVATIONS_PER_STEP: usize = 1_024;
        const GROUPS_PER_STEP: usize = 64;
        let pairwise_matches = &self.pairwise_matches;
        let preparation = self.track_preparation.get_or_insert_with(|| TrackPreparation::with_capacity(pairwise_matches.iter().map(|(_, _, matches)| 2 * matches.len()).sum()));
        match preparation.phase {
            TrackPhase::Union => {
                let mut work = 0;
                while preparation.pair < self.pairwise_matches.len() && work < OBSERVATIONS_PER_STEP {
                    let (frame_a, frame_b, matches) = &self.pairwise_matches[preparation.pair];
                    if preparation.matched >= matches.len() {
                        preparation.pair += 1;
                        preparation.matched = 0;
                        continue;
                    }
                    let matched = matches[preparation.matched];
                    let frame_a = *frame_a;
                    let frame_b = *frame_b;
                    preparation.matched += 1;
                    let left = preparation.node((frame_a, matched.a));
                    let right = preparation.node((frame_b, matched.b));
                    preparation.union(left, right);
                    work += 1;
                }
                if preparation.pair == self.pairwise_matches.len() {
                    preparation.phase = TrackPhase::Group;
                }
            }
            TrackPhase::Group => {
                let end = preparation.grouping_cursor.saturating_add(OBSERVATIONS_PER_STEP).min(preparation.observations.len());
                for index in preparation.grouping_cursor..end {
                    let observation = preparation.observations[index];
                    let root = preparation.root(index);
                    let group = preparation.groups.entry(root).or_insert_with(|| TrackGroup { observations: Vec::new(), frames: std::collections::BTreeSet::new(), conflict: false });
                    if !group.frames.insert(observation.0) {
                        group.conflict = true;
                    }
                    group.observations.push(observation);
                }
                preparation.grouping_cursor = end;
                if end == preparation.observations.len() {
                    preparation.phase = TrackPhase::Finish;
                }
            }
            TrackPhase::Finish => {
                for _ in 0..GROUPS_PER_STEP {
                    let Some((_, group)) = preparation.groups.pop_first() else { break };
                    if !group.conflict {
                        preparation.tracks.push(group.observations);
                    }
                }
                if preparation.groups.is_empty() {
                    let complete = self.track_preparation.take().expect("completed tracks");
                    return Some(semio_s_artifact_remodel_remodeling_sfm_geometry::editor::remodeling::engine::sfm::geometry::FeatureTracks { tracks: complete.tracks });
                }
            }
        }
        None
    }

    /// 🏗️ Seeds [`crate::editor::remodeling::engine::sfm::IncrementalSfm`] on the adjacent pair with the most verified
    /// matches (first calls), then registers + triangulates one frame per bounded unit, walking
    /// outwards from the seed — forwards to the last frame, then backwards to the first — so every
    /// frame is registered against already-registered neighbours; each registration is followed by
    /// a local bundle adjustment. A frame that fails to register is skipped (best effort, like
    /// `run_all`). A seed pair that cannot be solved is not a fault either: the capture has no
    /// registrable geometry, the stage ends with zero cameras and the run completes with an empty
    /// reconstruction the trace explains, instead of a pipeline failure.
    pub(super) fn step_estimating_poses(&mut self) -> Result<bool, String> {
        if self.sfm.is_none() {
            let intr = default_intrinsics(self.frames[0].image.width, self.frames[0].image.height, self.params.assumed_focal_ratio, self.params.distortion);
            let tracks = self.tracks.clone().expect("tracks built before EstimatingPoses");
            let keypoints = self.keypoints_per_frame.clone();
            let mut sfm = semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::IncrementalSfm::new(intr, tracks, keypoints, self.params.sfm.clone());
            if self.recorded.is_some() {
                sfm.observe_points();
            }
            let seed = self.pairwise_matches.iter().filter(|&&(a, b, _)| b == a + 1).max_by_key(|&&(a, _, ref matches)| (matches.len(), std::cmp::Reverse(a))).map(|(a, b, matches)| (*a, *b, semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::SeedPairPreparation::new(*a, *b, matches)));
            sfm.set_pairwise_matches(std::mem::take(&mut self.pairwise_matches));
            self.sfm = Some(sfm);
            match seed {
                Some((a, b, preparation)) => {
                    self.seed_pair_preparation = Some(preparation);
                    let n = self.frames.len();
                    self.registration_order = ((b + 1)..n).chain((0..a).rev()).collect();
                }
                None => {
                    self.record(EngineObservation::SeedPairFailed { detail: "no adjacent pair of frames shares verified matches".to_string() });
                    self.pose_cursor = usize::MAX;
                }
            }
        }
        if let Some(preparation) = self.seed_pair_preparation.as_mut() {
            match self.sfm.as_mut().expect("seed SfM").advance_seed_pair(preparation, 1) {
                Ok(false) => {}
                Ok(true) => {
                    self.seed_pair_preparation = None;
                    self.pose_cursor = 0;
                    let seeded: Vec<usize> = (0..self.frames.len()).filter(|&frame| !self.registration_order.contains(&frame)).collect();
                    for frame in seeded {
                        if let Some(pose) = self.sfm.as_ref().and_then(|sfm| sfm.camera_pose(frame)) {
                            self.record(EngineObservation::CameraRegistered { frame, pose });
                        }
                    }
                }
                Err(error) => {
                    self.seed_pair_preparation = None;
                    self.pose_cursor = usize::MAX;
                    self.record(EngineObservation::SeedPairFailed { detail: error.to_string() });
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        let n = self.registration_order.len();
        let sfm = self.sfm.as_mut().expect("initialized SfM");
        // 🎯️ A local adjustment in flight finishes before anything else is decided — including the
        // camera cap below, or the last admitted camera would leave the stage half-refined with the
        // adjustment state still armed for the global pass to pick up.
        if let Some(bundle) = self.bundle_iterations.as_mut() {
            if sfm.advance_bundle_adjustment(bundle, BUNDLE_TERMS_PER_STEP) {
                self.bundle_iterations = None;
                let frame = self.registration_order[self.pose_cursor];
                if let Some(pose) = sfm.camera_pose(frame) {
                    self.record(EngineObservation::CameraRegistered { frame, pose });
                }
                self.registration_preparation = None;
                self.pose_cursor += 1;
            }
            return Ok(self.pose_cursor < n || self.registration_preparation.is_some());
        }
        let registered = sfm.registered_count();
        if self.params.max_registered_cameras > 0 && registered >= self.params.max_registered_cameras {
            return Ok(false);
        }
        if self.pose_cursor >= n {
            return Ok(false);
        }
        if self.registration_preparation.is_none() {
            self.registration_preparation = Some(semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::RegistrationPreparation::new(self.registration_order[self.pose_cursor]));
        }
        let result = sfm.advance_registration(self.registration_preparation.as_mut().expect("registration preparation"), 1);
        let frame = self.registration_order[self.pose_cursor];
        let registered = sfm.camera_pose(frame);
        match result {
            Ok(true) => {
                let local = registered.and_then(|_| {
                    // 🎯️ Refine the newest cameras and their points before the next registration
                    // draws on them; the registration is announced once the refinement settles.
                    let frames = sfm.local_bundle_frames(LOCAL_BUNDLE_WINDOW);
                    sfm.begin_bundle_adjustment(&frames, &frames[..frames.len().min(2)], sfm.robust_loss(), LOCAL_BUNDLE_ITERATIONS)
                });
                if local.is_some() {
                    self.bundle_iterations = local;
                } else if registered.is_some() {
                    self.record(EngineObservation::CameraRegistered { frame, pose: registered.expect("registered pose") });
                    self.registration_preparation = None;
                    self.pose_cursor += 1;
                } else {
                    self.registration_preparation = None;
                    self.pose_cursor += 1;
                }
            }
            Err(_) => {
                self.record(EngineObservation::CameraRejected { frame });
                self.registration_preparation = None;
                self.pose_cursor += 1;
            }
            Ok(false) => {}
        }
        Ok(self.pose_cursor < n || self.registration_preparation.is_some())
    }

    /// 🎯️ One bounded unit of the bundle stage, which walks [`BUNDLE_STAGE_PLAN`]: a cleanup pass
    /// (prune + retriangulate) or a global adjustment one iteration per step over every camera and
    /// point, up to the document's iteration budget per adjustment.
    ///
    /// 🔁️ Why two adjustments: the first runs with a wide Huber kernel on the unpruned points so the
    /// loop-closure observations — which a drifted chain reprojects tens of pixels off, and which are
    /// the only evidence that can pull the loop shut — act with their full residual (a pixel-scale
    /// kernel caps them to a constant the thousands of chain observations outweigh, and the loop
    /// stays open at ~6°); the cleanup between the passes prunes the gross outliers that pass
    /// tolerated, and the second adjustment converges on the configured kernel.
    pub(super) fn step_bundle_adjusting(&mut self) -> bool {
        let Some(plan) = BUNDLE_STAGE_PLAN.get(self.ba_substep) else { return false };
        let Some(sfm) = self.sfm.as_mut() else { return false };
        match *plan {
            BundleStagePhase::Cleanup { tolerance } => {
                let preparation = self.bundle_preparation.get_or_insert_with(|| sfm.begin_bundle_with_tolerance(tolerance));
                if sfm.advance_bundle(preparation, 1) {
                    self.bundle_preparation = None;
                    self.ba_substep += 1;
                }
            }
            BundleStagePhase::Adjust { huber_factor } => {
                if self.bundle_iterations.is_none() {
                    let loss = match huber_factor {
                        Some(factor) => semio_s_artifact_remodel_remodeling_optimize::standards::v1::subsets::any::schema::optimize_internals::RobustLoss::Huber(self.params.sfm.ransac_threshold_px * factor),
                        None => sfm.robust_loss(),
                    };
                    // 📌️ Only the first camera is held: holding the seed pair's second camera too
                    // would pin the reconstruction to the seed's five-point baseline (a few degrees
                    // off), and every other camera would bend to fit it. Scale is left to the damping.
                    let frames = sfm.registered_frames();
                    self.bundle_iterations = sfm.begin_bundle_adjustment(&frames, &frames[..frames.len().min(1)], loss, self.params.sfm.ba_max_iterations);
                    if self.bundle_iterations.is_none() {
                        self.ba_substep += 1;
                        return self.ba_substep < BUNDLE_STAGE_PLAN.len();
                    }
                }
                let bundle = self.bundle_iterations.as_mut().expect("an open adjustment");
                if sfm.advance_bundle_adjustment(bundle, BUNDLE_TERMS_PER_STEP) {
                    self.bundle_iterations = None;
                    self.ba_substep += 1;
                }
            }
        }
        self.ba_substep < BUNDLE_STAGE_PLAN.len()
    }
}
// #endregion 🔖️Pipeline
