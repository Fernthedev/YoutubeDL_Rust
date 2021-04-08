use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::time::Duration;
use stopwatch::Stopwatch;
use ureq::{Agent, Response};

use crate::youtubedl::data::YoutubeDlVideoJson;
use crate::youtubedl::ffi::YoutubeDlVideo;

extern crate chrono;

const HTTP_OK: u16 = 200;

lazy_static! {
    static ref AGENT: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(15))
        .timeout_write(Duration::from_secs(15))
        .build();
}

///
/// Get the song based on hash
///
///
#[no_mangle]
pub unsafe extern "C" fn e_YoutubeDL_GetVideo(url: *const c_char) -> *mut YoutubeDlVideo {
    if url.is_null() {
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(url);

    let hash_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match youtubedl_get_video_data(hash_str) {
        Ok(e) => Box::into_raw(Box::new(YoutubeDlVideo::convert(e))),
        Err(e) => panic!(
            "Unable to fetch from database {0}",
            e.into_string().unwrap()
        ),
    }
}

///
/// Gets a song based on it's hash
///
pub fn youtubedl_get_video_data(url: &str) -> Result<YoutubeDlVideoJson, Response> {
    println!("Fetching from internet");
    let mut stopwatch = Stopwatch::start_new();
    let response = AGENT.get(url).call().unwrap();
    println!(
        "Received data from internet in {0}ms",
        stopwatch.elapsed().as_millis()
    );

    if response.status() == HTTP_OK {
        let mut stopwatch2 = Stopwatch::start_new();
        let body: YoutubeDlVideoJson = response.into_json().unwrap();
        println!(
            "Parsed youtube video into json data in {0}ms ({1}ms)",
            stopwatch.elapsed().as_millis(),
            stopwatch2.elapsed().as_millis()
        );

        stopwatch.stop();
        stopwatch2.stop();

        Ok(body)
    } else {
        Err(response)
    }
}
