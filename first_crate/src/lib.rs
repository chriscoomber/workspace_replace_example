#[macro_use]
extern crate log;
extern crate url;

pub fn log_url(url: url::Url) {
    debug!("{:?}", url);
}
