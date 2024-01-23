use crate::bindings::{
    avcodec::{LIBAVCODEC_VERSION_MAJOR, LIBAVCODEC_VERSION_MICRO, LIBAVCODEC_VERSION_MINOR},
    avformat::{LIBAVFORMAT_VERSION_MAJOR, LIBAVFORMAT_VERSION_MICRO, LIBAVFORMAT_VERSION_MINOR},
    avutil::{LIBAVUTIL_VERSION_MAJOR, LIBAVUTIL_VERSION_MICRO, LIBAVUTIL_VERSION_MINOR},
};

pub fn libavformat_version() -> String {
    let (major, minor, patch) = (
        LIBAVFORMAT_VERSION_MAJOR,
        LIBAVFORMAT_VERSION_MINOR,
        LIBAVFORMAT_VERSION_MICRO,
    );

    format!("{}.{}.{}", major, minor, patch)
}

pub fn libavcodec_version() -> String {
    let (major, minor, patch) = (
        LIBAVCODEC_VERSION_MAJOR,
        LIBAVCODEC_VERSION_MINOR,
        LIBAVCODEC_VERSION_MICRO,
    );

    format!("{}.{}.{}", major, minor, patch)
}

pub fn libavutil_version() -> String {
    let (major, minor, patch) = (
        LIBAVUTIL_VERSION_MAJOR,
        LIBAVUTIL_VERSION_MINOR,
        LIBAVUTIL_VERSION_MICRO,
    );

    format!("{}.{}.{}", major, minor, patch)
}
