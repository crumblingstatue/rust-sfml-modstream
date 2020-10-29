// Excerpt from the documentation:
// All strings returned from libopenmpt are encoded in UTF-8.
// All strings passed to libopenmpt should also be encoded in UTF-8.
// Behaviour in case of invalid UTF-8 is unspecified.
// libopenmpt does not enforce or expect any particular Unicode normalization form.
// All strings returned from libopenmpt are dynamically allocated and must be freed with
// openmpt_free_string().
// Do NOT use the C standard library free() for libopenmpt strings as that would make your code
// invalid on windows when dynamically linking against libopenmpt which itself
// statically links to the C runtime.
// All strings passed to libopenmpt are copied. No ownership is assumed or transferred.

pub struct OpenMptString {
    raw: *const std::os::raw::c_char,
}

impl OpenMptString {
    pub(crate) fn new(raw: *const std::os::raw::c_char) -> Option<Self> {
        Some(Self {
            raw: if raw.is_null() {
                return None;
            } else {
                raw
            },
        })
    }
    pub fn as_str(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.raw).to_str().unwrap() }
    }
}

impl std::fmt::Display for OpenMptString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Debug for OpenMptString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl Drop for OpenMptString {
    fn drop(&mut self) {
        unsafe {
            openmpt_sys::openmpt_free_string(self.raw);
        }
    }
}
