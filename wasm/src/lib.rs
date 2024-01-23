pub mod util;

use cffnpwr::ffprobe::types::{StreamInfo, Tag};
use ffprobe_rs::{
    get_info,
    version::{libavcodec_version, libavformat_version, libavutil_version},
    FileInfo as FileInfoRs, StreamInfo as StreamInfoRs, Tag as TagRs,
};

wit_bindgen::generate!({
    world: "ffprobe",
    exports: {
        world: FFprobe,
    }
});

impl Into<FileInfo> for FileInfoRs {
    fn into(self) -> FileInfo {
        FileInfo {
            name: self.name,
            bit_rate: self.bit_rate,
            duration: self.duration,
            url: self.url,
            nb_streams: self.nb_streams,
            flag_s: self.flag_s,
            nb_chapters: self.nb_chapters,
            streams: self
                .streams
                .into_iter()
                .map(|stream| stream.into())
                .collect(),
        }
    }
}
impl Into<StreamInfo> for StreamInfoRs {
    fn into(self) -> StreamInfo {
        StreamInfo {
            id: self.id,
            start_time: self.start_time,
            duration: self.duration,
            codec_type: self.codec_type,
            codec_name: self.codec_name,
            format: self.format,
            bit_rate: self.bit_rate,
            profile: self.profile,
            level: self.level,
            width: self.width,
            height: self.height,
            channels: self.channels,
            sample_rate: self.sample_rate,
            frame_size: self.frame_size,
            tags: self.tags.into_iter().map(|tag| tag.into()).collect(),
        }
    }
}
impl Into<Tag> for TagRs {
    fn into(self) -> Tag {
        Tag {
            key: self.key,
            value: self.value,
        }
    }
}

struct FFprobe;
impl Guest for FFprobe {
    fn init() {
        util::set_panic_hook();
    }

    fn libavformat_version() -> String {
        libavformat_version()
    }

    fn libavcodec_version() -> String {
        libavcodec_version()
    }

    fn libavutil_version() -> String {
        libavutil_version()
    }

    fn get_info(path: String) -> Result<FileInfo, String> {
        get_info(path.as_str()).map(|info| info.into())
    }
}
