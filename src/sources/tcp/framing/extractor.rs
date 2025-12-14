use bytes::{Buf, Bytes, BytesMut};
use memchr::memchr;

/// Message framing extractor (line, length-prefixed RFC6587)
pub struct FramingExtractor;

impl FramingExtractor {
    /// Extract line-separated message from buffer (drops trailing \n and any \r)
    pub fn extract_line_message(buf: &mut BytesMut) -> Option<Bytes> {
        let newline_pos = memchr(b'\n', buf.as_ref())?;
        let mut chunk = buf.split_to(newline_pos + 1);
        chunk.truncate(newline_pos);
        if let Some(first_cr) = memchr(b'\r', chunk.as_ref()) {
            let mut write_idx = first_cr;
            for read_idx in (first_cr + 1)..chunk.len() {
                let byte = chunk[read_idx];
                if byte != b'\r' {
                    chunk[write_idx] = byte;
                    write_idx += 1;
                }
            }
            chunk.truncate(write_idx);
        }
        Some(chunk.freeze())
    }

    /// Extract length-prefixed message from buffer: "<len><SP><payload>"
    pub fn extract_length_prefixed_message(buf: &mut BytesMut) -> Option<Bytes> {
        let mut pos = 0;
        let mut len = 0;
        while pos < buf.len() && pos < 10 && buf[pos].is_ascii_digit() {
            len = len * 10 + (buf[pos] - b'0') as usize;
            pos += 1;
        }
        if pos == 0 || pos >= buf.len() || buf[pos] != b' ' {
            return None;
        }
        pos += 1;
        if pos + len > buf.len() {
            return None;
        }
        buf.advance(pos);
        let payload = buf.split_to(len);
        Some(payload.freeze())
    }

    pub fn has_length_prefix(buf: &BytesMut) -> bool {
        let mut pos = 0;
        let mut found_digit = false;
        while pos < buf.len() && pos < 10 {
            if buf[pos].is_ascii_digit() {
                found_digit = true;
                pos += 1;
            } else if buf[pos] == b' ' && found_digit {
                return true;
            } else {
                break;
            }
        }
        false
    }

    pub fn has_newline(buf: &BytesMut) -> bool {
        buf.contains(&b'\n')
    }
}
