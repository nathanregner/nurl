use crate::{
    impl_fetcher,
    simple::{SimpleFetcher, SimpleFodFetcher},
    Url,
};

pub struct FetchPatch;
impl_fetcher!(FetchPatch);

impl SimpleFetcher<'_, 1> for FetchPatch {
    const KEYS: [&'static str; 1] = ["url"];
    const NAME: &'static str = "fetchpatch2";
    const REV_KEY: Option<&'static str> = None;

    fn get_values<'a>(&self, url: &'a Url) -> Option<[&'a str; 1]> {
        Some([url.as_str()])
    }
}

impl SimpleFodFetcher<'_, 1> for FetchPatch {}
