#[path = "common.rs"]
mod common;

#[cfg(test)]
mod test_alexa_top1m {
    use std::path::Path;
    use rstest::*;
    use jarm_online::alexa_top1m::{AlexaTop1M, RankedDomain};
    use crate::common::alexa_top1m_path;

    #[rstest]
    fn build_alexa_top1m_struct_zero_hash(alexa_top1m_path: &Path) {
        let top1m = AlexaTop1M::new(alexa_top1m_path).expect("built successfully");

        assert_eq!(top1m.len(), 8);
        let hash = "123";
        let domains = top1m.get(hash);
        assert_eq!(domains, None);
    }

    #[rstest]
    fn build_alexa_top1m_struct_single_hash(alexa_top1m_path: &Path) {
        let top1m = AlexaTop1M::new(alexa_top1m_path).expect("built successfully");

        assert_eq!(top1m.len(), 8);
        let hash = "3fd3fd20d3fd3fd21c3fd3fd3fd3fd2b66a312d81ed1efa0f55830f7490cb2";
        let domains = top1m.get(hash).unwrap();
        assert_eq!(domains, &vec![RankedDomain {
            rank: 9,
            domain: "zhihu.com".to_string(),
        }]);
    }

    #[rstest]
    fn build_alexa_top1m_struct_multiple_hash(alexa_top1m_path: &Path) {
        let top1m = AlexaTop1M::new(alexa_top1m_path).expect("built successfully");

        assert_eq!(top1m.len(), 8);
        let hash = "29d3fd00029d29d21c42d43d00041d188e8965256b2536432a9bd447ae607f";
        let domains = top1m.get(hash).unwrap();
        assert_eq!(domains, &vec![
            RankedDomain {
                rank: 1,
                domain: "google.com".to_string(),
            }, RankedDomain {
                rank: 2,
                domain: "youtube.com".to_string(),
            },
        ]);
    }

    #[test]
    fn build_alexa_top1m_struct_invalid_path() {
        let fixture_path = Path::new("invalid_path.csv");
        let err = AlexaTop1M::new(fixture_path).unwrap_err();
        assert_eq!(format!("{err}"), "No such file or directory (os error 2)");
    }
}
