use tokio_util::codec::length_delimited::{Builder, LengthDelimitedCodec};

#[inline]
pub fn make_protocol_builder() -> Builder {
    *LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(crate::MAX_FRAME_SIZE)
}
