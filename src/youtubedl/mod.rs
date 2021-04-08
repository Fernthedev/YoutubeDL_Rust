mod data;
mod database;

#[macro_use]
mod macros;
mod ffi;

#[cfg(test)]
mod tests {
    use super::*;
    use database::*;
    use stopwatch::Stopwatch;
    use serde_json::*;

    #[test]
    fn get_song_data() {
        let mut stopwatch = Stopwatch::start_new();
        println!("Getting db");
        let result = youtubedl_get_video_data("https://youtubed-dl-quest.herokuapp.com/v1/video?url=https://www.youtube.com/watch?v=_qwnHeMKbVA");
        assert!(result.is_ok());
        println!("Got song, took {0}ms", stopwatch.elapsed().as_millis());
        stopwatch.restart();

        println!("Json result: \n{0}", serde_json::to_string(&result.unwrap()).unwrap());
    }
}
