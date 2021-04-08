#include "Main.hpp"
#include "../shared/bindings.hpp"
#include "Stopwatch.hpp"
#include <iostream>

using namespace std;

int main() {
    auto stopwatch = stopwatch::Stopwatch();
    stopwatch.start();

    cout << "Fetching from youtube the prince\n";


    auto vid = youtubedl_rust::e_YoutubeDL_GetVideo("https://youtubed-dl-quest.herokuapp.com/v1/video?url=https://www.youtube.com/watch?v=_qwnHeMKbVA");
    cout << "Time taken to fetch from youtube: " << stopwatch.elapsed<stopwatch::milliseconds>() << "ms\n";

    cout << "Got the video: " << vid->title.string_data << "\n";

    for (int i = 0; i < youtubedl_rust::YoutubeDlVideo_FormatsLen(vid); i++) {
        auto format = youtubedl_rust::YoutubeDlVideo_FormatsGet(vid, i);
        cout << "Format extension: " << format->ext.string_data << "\n";
        cout << "Format audio: " << (format->acodec ? format->acodec->string_data : "null") << "\n";
        cout << "Format video: " << (format->format_note ? format->format_note->string_data : "null") << "\n";
    }

    youtubedl_rust::YoutubeDlVideo_drop(vid);

    return 0;
}