pub(super) const IMG_WIDTH: usize  = 600; // M02 pro (dpi 300)
pub(super) const LINE_BUFFER_LENGTH: usize = IMG_WIDTH / 8;

pub(super) const _INIT: &[u8; 2] = b"\x1b\x40";
pub(super) const _ALIGN_CENTER: &[u8; 3] = b"\x1b\x61\x01";
pub(super) const _GS_MODE: &[u8; 4] = b"\x1d\x76\x30\x00";
pub(super) const _WIDTH: &[u8; 2] = &((IMG_WIDTH / 8) as u16).to_le_bytes();
pub(super) const _PAPER_FEED_4: &[u8; 3] = b"\x1b\x64\x04";

pub(super) const PROP_MAGIC_HEADER: &[u8; 4] = b"\x1f\x11\x02\x04"; // looks like print colour density
#[allow(dead_code)]
pub(super) const PROP_MAGIC_FOOTER: &[u8; 12] = b"\x1f\x11\x08\x1f\x11\x0e\x1f\x11\x07\x1f\x11\x09"; //dunno