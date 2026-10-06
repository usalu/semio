use super::{should_start_introduction, IntroductionEligibility};
use semio_framework_pack_json::{parse, JsonMemberPolicy};

#[test]
fn shared_introduction_cases_preserve_every_required_fact() {
    let corpus = parse(include_str!("../../🧫️fixtures/🔣️.json"), JsonMemberPolicy::Reject).expect("closed introduction corpus");
    let rows = corpus["cases"].as_array().expect("shared cases");
    assert_eq!(rows.len(), 130);
    for row in rows {
        let input = &row["input"];
        let actual = should_start_introduction(&IntroductionEligibility {
            app_id: input["appId"].as_str().expect("app identity"),
            has_introduction: input["hasIntroduction"].as_bool().expect("authored introduction"),
            tutorial_active: input["tutorialActive"].as_bool().expect("tutorial activity"),
            suppressed: input["suppressed"].as_bool().expect("host suppression"),
            replay_on_load: input["replayOnLoad"].as_bool().expect("replay policy"),
            seen_on_device: input["seenOnDevice"].as_bool().expect("device observation"),
            dismissed_in_session: input["dismissedInSession"].as_bool().expect("session answer"),
        });
        assert_eq!(actual, row["expected"].as_bool().expect("expected eligibility"), "{}", row["id"].as_str().expect("case identity"));
    }
    println!("[DEBUG] neutral introduction: 130 shared vectors");
}
