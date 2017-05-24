extern crate sfml;
extern crate openmpt_sys;

use openmpt_sys::*;

use sfml::audio::SoundStream;
use sfml::system::Time;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::ptr;

pub struct ModStream {
    module: *mut openmpt_module,
    buffer: [i16; 2048],
}

impl ModStream {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        unsafe {
            let module = openmpt_module_create_from_memory(buf.as_ptr() as *const _,
                                                           buf.len(),
                                                           None,
                                                           ptr::null_mut(),
                                                           ptr::null());
            if module.is_null() {
                panic!("Failed load module");
            }
            Ok(ModStream {
                   module: module,
                   buffer: [0; 2048],
               })
        }
    }
    pub fn get_duration_seconds(&self) -> f64 {
        unsafe { openmpt_module_get_duration_seconds(self.module) }
    }
}

impl SoundStream for ModStream {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        unsafe {
            let keep_playing = openmpt_module_read_interleaved_stereo(self.module,
                                                                      44100,
                                                                      1024,
                                                                      self.buffer.as_mut_ptr()) !=
                               0;
            (&mut self.buffer[..], keep_playing)
        }
    }
    fn seek(&mut self, offset: Time) {
        unsafe {
            openmpt_module_set_position_seconds(self.module, offset.as_seconds() as f64);
        }
    }
    fn sample_rate(&self) -> u32 {
        44100
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
