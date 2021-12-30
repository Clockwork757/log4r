# log4r

## ⚠️⚠️ This was a joke for the rust subreddit, please don't actually use it for anything ⚠️⚠️

log4r is a simple, secure and extensible logging implementation for rust.

example:

```rust
use log4r::SECURE_EXTENSION_PREFIX;
use log::info;

fn low_level_http_handler(http_request: &[u8]) -> Result<(),()>{
    info!("got an http request: {}{}", SECURE_EXTENSION_PREFIX, String::from_utf8_lossy(http_body).unwrap())
}
```
