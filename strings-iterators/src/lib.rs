pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let p = prefix.as_bytes();
    let r = request_path.as_bytes();
    let mut i = 0;
    let mut j = 0;

    while i < p.len() {
        if j >= r.len() {
            return false;
        }
        if p[i] == r[j] {
            i += 1;
            j += 1;
            continue;
        }
        if p[i] == b'*' {
            if r[j] != b'/' {
                j += 1;
                continue;
            } else {
                i += 1;
                continue;
            }
        }
        return false;
    }
    j >= r.len() || r[j] == b'/'
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books",
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books",
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1",
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor",
    ));
}
