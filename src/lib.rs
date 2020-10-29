extern crate openmpt_sys;
extern crate sfml;

use crate::string::OpenMptString;
use openmpt_sys::*;
use sfml::audio::SoundStream;
use sfml::system::Time;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::ptr;

mod string;

pub struct ModStream {
    module: *mut openmpt_module,
    buffer: [i16; 2048],
}

impl ModStream {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(Self::from_memory(&buf))
    }
    pub fn from_memory(data: &[u8]) -> Self {
        unsafe {
            let module = openmpt_module_create_from_memory(
                data.as_ptr() as *const _,
                data.len(),
                None,
                ptr::null_mut(),
                ptr::null(),
            );
            if module.is_null() {
                panic!("Failed load module");
            }
            Self {
                module,
                buffer: [0; 2048],
            }
        }
    }
    pub fn get_duration_seconds(&self) -> f64 {
        unsafe { openmpt_module_get_duration_seconds(self.module) }
    }
    pub fn metadata<'a, K: Key<'a>>(&self, key: K) -> Option<OpenMptString> {
        let key = std::ffi::CString::new(key.to_str()).unwrap();
        let ret = unsafe { openmpt_module_get_metadata(self.module, key.as_ptr()) };
        OpenMptString::new(ret)
    }
}

pub trait Key<'a> {
    fn to_str(self) -> &'a str;
}

impl<'a> Key<'a> for &'a str {
    fn to_str(self) -> &'a str {
        self
    }
}

pub enum Metadata {
    Tracker,
    Artist,
    Title,
    Date,
    Message,
}

impl Key<'static> for Metadata {
    fn to_str(self) -> &'static str {
        match self {
            Self::Tracker => "tracker",
            Self::Artist => "artist",
            Self::Title => "title",
            Self::Date => "date",
            Self::Message => "message",
        }
    }
}

impl SoundStream for ModStream {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        unsafe {
            let keep_playing = openmpt_module_read_interleaved_stereo(
                self.module,
                44_100,
                1024,
                self.buffer.as_mut_ptr(),
            ) != 0;
            (&mut self.buffer[..], keep_playing)
        }
    }
    fn seek(&mut self, offset: Time) {
        unsafe {
            openmpt_module_set_position_seconds(self.module, f64::from(offset.as_seconds()));
        }
    }
    fn sample_rate(&self) -> u32 {
        44_100
    }
    fn channel_count(&self) -> u32 {
        2
    }
}

impl Drop for ModStream {
    fn drop(&mut self) {
        unsafe {
            openmpt_module_destroy(self.module);
        }
    }
}
