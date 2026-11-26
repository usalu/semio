use super::*;
#[test]
fn retained_donor_survives_empty_crossings_and_newer_events() {
    let mut pending = PendingEvents::<Box<str>, 3>::new();
    let mut original = None;
    for line in include_str!("🧫️fixtures/crossings.txt").lines().filter(|line| !line.starts_with('#')) {
        let [incoming, consume, selected, waiting]: [&str; 4] = line.split(',').collect::<Vec<_>>().try_into().unwrap();
        let event = (incoming != "-").then(|| Box::<str>::from(incoming));
        if incoming == "completed" { original = event.as_ref().map(|event| event.as_ptr()); }
        let mut donor = pending.begin(event).unwrap();
        assert_eq!(donor.as_deref().unwrap_or("-"), selected);
        if selected == "completed" { assert_eq!(donor.as_ref().map(|event| event.as_ptr()), original); }
        assert!(pending.held());
        assert!(pending.begin(None).is_err());
        if consume == "yes" { donor.take(); }
        assert_eq!(pending.finish(donor), waiting != "0");
        assert_eq!(pending.len(), waiting.parse::<usize>().unwrap());
    }
}
#[test]
fn capacity_refusal_returns_the_new_event_and_preserves_the_original() {
    let mut pending=PendingEvents::<Box<str>,1>::new();
    let donor=pending.begin(Some("first".into())).unwrap();
    pending.finish(donor);
    let incoming:Box<str>="second".into();
    let pointer=incoming.as_ptr();
    let returned = pending.begin(Some(incoming)).unwrap_err().unwrap();
    assert_eq!(returned.as_ptr(),pointer);
    assert_eq!(pending.begin(None).unwrap().as_deref(),Some("first"));
    assert!(!pending.finish(None));
}
