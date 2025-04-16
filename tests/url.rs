use devonrex::http::{equal_url, url_split};
#[test]
fn test_url_split() {
    let url = "/user/<id>".to_string();
    let url_ = vec!["/user".to_string(), "/<id>".to_string()];
    assert_eq!(url_split(url), url_)
}

#[test]
fn test_equal_url() {
    let url = "/user/<id>".to_string();
    let url_ = "/user/123".to_string();
    assert!(equal_url(url_split(url), url_split(url_)))
}
