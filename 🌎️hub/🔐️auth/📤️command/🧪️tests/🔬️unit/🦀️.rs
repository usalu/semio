use super::*;

#[test]
fn only_the_exact_credential_verb_is_claimed() {
    assert!(selected(&[]).expect("empty").is_none());
    assert!(selected(&[OsString::from("trusted-catalog"), OsString::from("publish")]).expect("other verb").is_none());
    assert!(selected(&[OsString::from("credential")]).expect("bare noun").is_none());
    assert!(selected(&[OsString::from("credential"), OsString::from("rotate")]).is_err());
    assert!(selected(&[OsString::from("credential"), OsString::from("set")]).is_err());
    assert!(selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--email")]).is_err());
    assert!(selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--secret"), OsString::from("x")]).is_err());
}

#[test]
fn the_email_is_bounded_and_lowercased_exactly_as_the_route_lowercases_it() {
    let parsed = selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--email"), OsString::from("  Ada@Example.COM ")]).expect("bounded").expect("selected");
    assert_eq!(parsed.email, "ada@example.com");
    assert_eq!(parsed.display_name, None);
    for hostile in ["", "a", "no-at-sign", "two@at@signs", "@domain.example", "local@", "with space@example.com", &format!("{}@example.com", "a".repeat(250))] {
        assert!(selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--email"), OsString::from(hostile)]).is_err(), "{hostile}");
    }
}

#[test]
fn the_display_name_is_bounded() {
    let parsed = selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--email"), OsString::from("a@b.example"), OsString::from("--display-name"), OsString::from("Ada Lovelace")])
        .expect("bounded")
        .expect("selected");
    assert_eq!(parsed.display_name.as_deref(), Some("Ada Lovelace"));
    for hostile in ["", " leading", "trailing ", "line\nbreak", &"n".repeat(129)] {
        assert!(
            selected(&[OsString::from("credential"), OsString::from("set"), OsString::from("--email"), OsString::from("a@b.example"), OsString::from("--display-name"), OsString::from(hostile)]).is_err(),
            "{hostile:?}"
        );
    }
}

#[test]
fn the_password_comes_from_stdin_within_its_exact_bounds() {
    assert_eq!(read_password(&b"correct horse battery staple\n"[..]).expect("admitted"), "correct horse battery staple");
    assert_eq!(read_password(&b"trailing-crlf\r\n"[..]).expect("admitted"), "trailing-crlf");
    assert!(read_password(&b"short\n"[..]).is_err());
    assert!(read_password(&b""[..]).is_err());
    assert!(read_password(&b"with\ta\ttab\n"[..]).is_err());
    assert!(read_password(&b"\xff\xfe not utf8 at all\n"[..]).is_err());
    assert!(read_password(&vec![b'p'; PASSWORD_MAX_BYTES + 1][..]).is_err());
    assert_eq!(read_password(&vec![b'p'; PASSWORD_MAX_BYTES][..]).expect("at the ceiling").len(), PASSWORD_MAX_BYTES);
}

#[test]
fn the_iteration_count_is_bounded() {
    for (value, expected) in [(None, Some(DEFAULT_ITERATIONS)), (Some(""), Some(DEFAULT_ITERATIONS)), (Some("1000"), Some(MIN_ITERATIONS)), (Some("999"), None), (Some("x"), None), (Some("1000000000"), None)] {
        match value {
            Some(value) => unsafe { std::env::set_var("OS_HUB_PASSWORD_ITERATIONS", value) },
            None => unsafe { std::env::remove_var("OS_HUB_PASSWORD_ITERATIONS") },
        }
        assert_eq!(configured_iterations().ok(), expected, "{value:?}");
    }
    unsafe { std::env::remove_var("OS_HUB_PASSWORD_ITERATIONS") };
}
