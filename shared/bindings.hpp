#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace youtubedl_rust {

template<typename T = void>
struct Vec;

struct RustCStringWrapper {
  char *string_data;

  bool operator==(const RustCStringWrapper& other) const {
    return string_data == other.string_data;
  }
  bool operator!=(const RustCStringWrapper& other) const {
    return string_data != other.string_data;
  }
};

struct YoutubeDlFormat {
  uint32_t *height;
  uint32_t *width;
  RustCStringWrapper *acodec;
  RustCStringWrapper *format_note;
  RustCStringWrapper url;
  RustCStringWrapper ext;

  bool operator==(const YoutubeDlFormat& other) const {
    return height == other.height &&
           width == other.width &&
           acodec == other.acodec &&
           format_note == other.format_note &&
           url == other.url &&
           ext == other.ext;
  }
  bool operator!=(const YoutubeDlFormat& other) const {
    return height != other.height ||
           width != other.width ||
           acodec != other.acodec ||
           format_note != other.format_note ||
           url != other.url ||
           ext != other.ext;
  }
};

struct YoutubeDlVideo {
  RustCStringWrapper id;
  RustCStringWrapper title;
  const Vec<YoutubeDlFormat> *formats;

  bool operator==(const YoutubeDlVideo& other) const {
    return id == other.id &&
           title == other.title &&
           formats == other.formats;
  }
  bool operator!=(const YoutubeDlVideo& other) const {
    return id != other.id ||
           title != other.title ||
           formats != other.formats;
  }
};

extern "C" {

///
/// Get the song based on hash
///
///
YoutubeDlVideo *e_YoutubeDL_GetVideo(const char *url);

RustCStringWrapper RustCStringWrapper_c_new(char *c_str);

void YoutubeDlVideo_drop(YoutubeDlVideo *self_i);

const YoutubeDlFormat *YoutubeDlVideo_FormatsGet(const YoutubeDlVideo *self_i, uintptr_t index);

uintptr_t YoutubeDlVideo_FormatsLen(const YoutubeDlVideo *self_i);

} // extern "C"

} // namespace youtubedl_rust
