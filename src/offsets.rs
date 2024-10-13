use once_cell::sync::OnceCell;

#[macro_export]
macro_rules! ensure_hooks {
    ($($f:expr),*) => {{
        let mut is_successful = true;
        $(
            if $f.get_offset_in_memory().is_none() && is_successful {
                skyline::error::show_error(
                    420,
                    "Less Delay failed to load.\0",
                    format!("Error: Failed to find {} in memory.\n\n{}\n\n{}\n\n{}\0",
                    $f.location_name,
                    "This may be the result of an incompatible mod being loaded, or SSBU being updated.",
                    "If you are unsure, head over to the issues page at\nhttps://github.com/Naxdy/less-delay/issues",
                    "Less Delay will NOT be enabled now, however you can continue playing normally."
                    ).as_str()
                );

                is_successful = false;
            }
        )*

        is_successful
    }};
}

///
/// Searches for a byte pattern in the text region of the process memory, outputs
/// the start address of the first match, or `None` in case of no match.
///
fn byte_search(needle: &[u8]) -> Option<usize> {
    let search_space = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const u8;
        let length = end.offset_from(start) as usize;

        std::slice::from_raw_parts(start, length)
    };

    search_space.windows(needle.len()).position(|w| w == needle)
}

pub struct SSBUMemoryLocation<'a> {
    ///
    /// A memory signature used to search for the given location in memory.
    ///
    signature: &'a [u8],

    ///
    /// Offset to the actual location (in bytes) that we are interested in hooking,
    /// relative to the start of the `signature`.
    /// If the first byte of `signature` is the exact location we want, this will be `0`.
    ///
    start_offset: isize,

    ///
    /// A human-readable function / location name, used to display an error message to
    /// the user if the location cannot be found for hooking.
    ///
    pub location_name: &'a str,

    cached_offset: OnceCell<Option<usize>>,
}

impl SSBUMemoryLocation<'_> {
    pub fn get_offset_in_memory(&self) -> Option<usize> {
        *self.cached_offset.get_or_init(|| unsafe {
            let r = Some(
                ((byte_search(self.signature)? as *const u8).offset(self.start_offset)) as usize,
            );

            if let Some(r) = r {
                println!("[less-delay] Found {} at {r:#09x?}", self.location_name);
            }

            r
        })
    }
}

pub static LOC_VSYNC_COUNT_THREAD: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xa8, 0xfe, 0x5f, 0xc8, 0x08, 0x05, 0x00, 0x91, 0xa8, 0xfe, 0x09, 0xc8, 0xa9, 0xff, 0xff,
        0x35, 0x88, 0xc2, 0x4e, 0x39, 0x28, 0xff, 0xff, 0x34, 0xfd, 0x7b, 0x42, 0xa9, 0xf4, 0x4f,
        0x41, 0xa9, 0xf5, 0x07, 0x43, 0xf8, 0xc0, 0x03, 0x5f, 0xd6,
    ],
    start_offset: 0,
    location_name: "vsync_count_thread",
    cached_offset: OnceCell::new(),
};

pub static LOC_RUN_SCENE_UPDATE: SSBUMemoryLocation = SSBUMemoryLocation {
    signature: &[
        0xae, 0xe1, 0x09, 0x94, 0x08, 0xe8, 0x01, 0x90, 0x08, 0x75, 0x40, 0xf9, 0x00, 0x00, 0x08,
        0xcb, 0xde, 0xe2, 0x09, 0x94, 0xe8, 0x37, 0x41, 0xf9, 0xff, 0x3b, 0x01, 0xf9, 0x08, 0x05,
        0x40, 0xf9, 0x01, 0x01, 0x40, 0xf9, 0xe0, 0x0b, 0x40, 0x91, 0x00, 0xc0, 0x0f, 0x91, 0xe2,
        0xc3, 0x09, 0x91, 0x2e, 0xa0, 0x04, 0x94, 0x88, 0xdf, 0x00, 0xd0, 0x08, 0x69, 0x43, 0xf9,
        0x08, 0x05, 0x40, 0xf9, 0x00, 0x01, 0x40, 0xf9, 0x40, 0x00, 0x00, 0xb4, 0xd8, 0xe2, 0x09,
        0x94,
    ],
    start_offset: 52,
    location_name: "run_scene_update",
    cached_offset: OnceCell::new(),
};
