use git_url_parse::*;

// Url Normalization
#[test]
fn git() {
    let test_url = "git://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "git://host.tld/user/project-name.git");
}

#[test]
fn http() {
    let test_url = "http://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "http://host.tld/user/project-name.git");
}

#[test]
fn https() {
    let test_url = "https://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "https://host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_scheme() {
    let test_url = "ssh://git@host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "ssh://git@host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_no_scheme() {
    let test_url = "git@host.tld:user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "ssh://git@host.tld/user/project-name.git"
    );
}

#[test]
fn ssh_no_scheme_no_user() {
    let test_url = "host.tld:user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "ssh://host.tld/user/project-name.git");
}

#[test]
fn file_scheme() {
    let test_url = "file:///user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file:///user/project-name.git");
}

#[test]
fn file_no_scheme() {
    let test_url = "/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(normalized.as_str(), "file:///user/project-name.git");
}

#[test]
fn multi_git_ssh() {
    let test_url = "git+ssh://host.tld/user/project-name.git";
    let normalized = normalize_url(test_url).expect("Normalizing url failed");

    assert_eq!(
        normalized.as_str(),
        "git+ssh://host.tld/user/project-name.git"
    );
}