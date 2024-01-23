mod bindings;
mod util;
pub mod version;

use bindings::{
    avcodec::avcodec_profile_name,
    avformat::{
        av_dict_get, avformat_alloc_context, avformat_close_input, avformat_find_stream_info,
        avformat_open_input, AVDictionary, AVDictionaryEntry, AVFormatContext, AVInputFormat,
        AV_DICT_IGNORE_SUFFIX,
    },
    avutil::av_get_pix_fmt_name,
};
use std::ffi::{c_char, CStr, CString};
use util::NULL;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub bit_rate: f32,
    pub duration: f32,
    pub url: String,
    pub nb_streams: i32,
    pub flag_s: i32,
    pub nb_chapters: i32,
    pub streams: Vec<StreamInfo>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub id: i32,
    pub start_time: f32,
    pub duration: f32,
    pub codec_type: i32,
    pub codec_name: String,
    pub format: String,
    pub bit_rate: f32,
    pub profile: String,
    pub level: i32,
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub sample_rate: i32,
    pub frame_size: i32,
    pub tags: Vec<Tag>,
}

pub fn get_info(path: &str) -> Result<FileInfo, String> {
    let mut context = unsafe { avformat_alloc_context() };
    let cstr = CString::new(path).unwrap();
    let cstr_ptr: *const i8 = cstr.as_ptr();
    let ret = unsafe {
        avformat_open_input(
            &mut context as *mut *mut AVFormatContext,
            cstr_ptr as *const i8,
            NULL as *const AVInputFormat,
            NULL as *mut *mut AVDictionary,
        )
    };
    if ret < 0 {
        return Err(format!("Error: error code {}", ret));
    }

    let ret = unsafe { avformat_find_stream_info(context, NULL as *mut *mut AVDictionary) };
    if ret < 0 {
        return Err(format!("Could not find stream information"));
    }

    // FileInfoResponse r = {
    //     .name = pFormatContext->iformat->name,
    //     .bit_rate = (float)pFormatContext->bit_rate,
    //     .duration = (float)pFormatContext->duration,
    //     .url = pFormatContext->url,
    //     .nb_streams = (int)pFormatContext->nb_streams,
    //     .flags = pFormatContext->flags,
    //     .nb_chapters = (int)pFormatContext->nb_chapters
    // };
    let mut file_info = FileInfo {
        name: unsafe {
            CStr::from_ptr((*context).iformat.as_ref().unwrap().name)
                .to_str()
                .map_err(|e| e.to_string())?
                .to_string()
        },
        bit_rate: unsafe { (*context).bit_rate as f32 },
        duration: unsafe { (*context).duration as f32 },
        url: unsafe { CStr::from_ptr((*context).url).to_str().unwrap().to_string() },
        nb_streams: unsafe { (*context).nb_streams as i32 },
        flag_s: unsafe { (*context).flags },
        nb_chapters: unsafe { (*context).nb_chapters as i32 },
        streams: vec![],
    };

    for i in 0..file_info.nb_streams {
        let av_stream = unsafe {
            (*context)
                .streams
                .offset(i as isize)
                .as_ref()
                .ok_or("streams are null")?
                .as_ref()
                .ok_or("stream is null")?
        };
        let local_codec_parameters = if unsafe { (*av_stream).codecpar.as_ref() }.is_some() {
            unsafe { (*av_stream).codecpar.as_ref().unwrap() }
        } else {
            continue;
        };

        let codec_tag = local_codec_parameters.codec_tag;
        let fourcc = [
            (codec_tag & 0xFF) as u8 as char,
            (codec_tag >> 8 & 0xFF) as u8 as char,
            (codec_tag >> 16 & 0xFF) as u8 as char,
            (codec_tag >> 24 & 0xFF) as u8 as char,
        ]
        .iter()
        .collect::<String>();

        // Stream stream = {
        //     .id = (int)pFormatContext->streams[i]->id,
        //     .start_time = (float)pFormatContext->streams[i]->start_time,
        //     .duration = (float)pFormatContext->streams[i]->duration,
        //     .codec_type = (int)pLocalCodecParameters->codec_type,
        //     .codec_name = fourcc,
        //     .format = av_get_pix_fmt_name((AVPixelFormat)pLocalCodecParameters->format),
        //     .bit_rate = (float)pLocalCodecParameters->bit_rate,
        //     .profile = avcodec_profile_name(pLocalCodecParameters->codec_id, pLocalCodecParameters->profile),
        //     .level = (int)pLocalCodecParameters->level,
        //     .width = (int)pLocalCodecParameters->width,
        //     .height = (int)pLocalCodecParameters->height,
        //     .channels = (int)pLocalCodecParameters->channels,
        //     .sample_rate = (int)pLocalCodecParameters->sample_rate,
        //     .frame_size = (int)pLocalCodecParameters->frame_size,
        // };
        let mut stream = StreamInfo {
            id: av_stream.id,
            start_time: av_stream.start_time as f32,
            duration: av_stream.duration as f32,
            codec_type: local_codec_parameters.codec_type as i32,
            codec_name: fourcc,
            format: match unsafe { av_get_pix_fmt_name(local_codec_parameters.format) } {
                ptr if ptr.is_null() => "".to_string(),
                ptr => unsafe {
                    CStr::from_ptr(ptr)
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string()
                },
            },
            bit_rate: local_codec_parameters.bit_rate as f32,
            profile: match unsafe {
                avcodec_profile_name(
                    local_codec_parameters.codec_id,
                    local_codec_parameters.profile,
                )
            } {
                ptr if ptr.is_null() => "".to_string(),
                ptr => unsafe {
                    CStr::from_ptr(ptr)
                        .to_str()
                        .map_err(|e| e.to_string())?
                        .to_string()
                },
            },
            level: local_codec_parameters.level as i32,
            width: local_codec_parameters.width as i32,
            height: local_codec_parameters.height as i32,
            channels: local_codec_parameters.channels as i32,
            sample_rate: local_codec_parameters.sample_rate as i32,
            frame_size: local_codec_parameters.frame_size as i32,
            tags: vec![],
        };

        // const AVDictionaryEntry *tag = NULL;
        // while ((tag = av_dict_get(pFormatContext->streams[i]->metadata, "", tag, AV_DICT_IGNORE_SUFFIX))) {
        //     Tag t = {
        //     .key = tag->key,
        //     .value = tag->value,
        //     };
        //     stream.tags.push_back(t);
        // }
        let mut tag = NULL as *mut AVDictionaryEntry;
        loop {
            match unsafe {
                av_dict_get(
                    av_stream.metadata,
                    "".as_ptr() as *const c_char,
                    tag,
                    AV_DICT_IGNORE_SUFFIX as i32,
                )
            } {
                ptr if ptr.is_null() => break,
                new_tag => {
                    tag = new_tag;
                    stream.tags.push(Tag {
                        key: unsafe {
                            CStr::from_ptr((*new_tag).key)
                                .to_str()
                                .map_err(|e| e.to_string())?
                                .to_string()
                        },
                        value: unsafe {
                            CStr::from_ptr((*new_tag).value)
                                .to_str()
                                .map_err(|e| e.to_string())?
                                .to_string()
                        },
                    });
                }
            }
        }

        file_info.streams.push(stream);
    }

    unsafe {
        avformat_close_input(&mut context);
    }

    Ok(file_info)
}
