#[cfg(test)]
mod tests {
    use jarm_online::sanitize_host;

    #[test]
    fn test_sanitize_host_good() {
        let input = "test.com".to_string();
        assert_eq!(sanitize_host(&input), input);
    }

    #[test]
    fn test_sanitize_host_start_with_extra_white_space() {
        let input = "  test.com".to_string();
        assert_eq!(sanitize_host(&input), "test.com".to_string());
    }

    #[test]
    fn test_sanitize_host_extra_white_space() {
        let input = "  test.com \t  ".to_string();
        assert_eq!(sanitize_host(&input), "test.com".to_string());
    }

    #[test]
    fn test_sanitize_host_transform_url_to_fqdn() {
        let input = "https://test.com/".to_string();
        assert_eq!(sanitize_host(&input), "test.com".to_string());
    }

    #[test]
    fn test_sanitize_host_with_port() {
        let input = "https://test.com:443/wow/index.html".to_string();
        assert_eq!(sanitize_host(&input), "test.com".to_string());
    }
}