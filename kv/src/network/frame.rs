use std::io::{Write, Read};

use bytes::{BufMut, BytesMut, Buf};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use prost::Message;
use tracing::debug;

use crate::{KvError, CommandRequest, CommandResponse};

/// 长度整个占用 4 个字节
pub const LEN_LEN: usize = 4;
/// 长度占 31 bit，所以最大的 frame 是 2G
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
/// 如果payload超过了1436字节，就需要压缩
const COMPRESSION_LIMIT: usize = 1436;
/// 代表压缩的 bit（整个长度 4 字节的最高位)
const COMPRESSION_BIT: usize = 1 << 31;

/// 处理 Frame 的 encode/decode
pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    /// 把一个Message ecode 成一个 Frame
fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
    let size = self.encoded_len();

    if size >= MAX_FRAME {
        return Err(KvError::FrameError);
    }
    // 写入长度
    buf.put_u32(size as _);

    if size > COMPRESSION_LIMIT {
        let mut buf1 = Vec::with_capacity(size);
        self.encode(&mut buf1)?;

        // BytesMut 支持逻辑上的 split（之后还能 unsplit） 
        // 所以我们先把长度这 4 字节拿走，清除
        let payload = buf.split_off(LEN_LEN);
        buf.clear();

        // 处理 gzip 压缩，具体可以参考 flate2 文档
        let mut encoder = GzEncoder::new(payload.writer(), Compression::default());
        encoder.write_all(&buf1[..])?;

        // 压缩完成后，从gzip encoder中把BytesMut再拿回来
        let payload = encoder.finish()?.into_inner();
        debug!("Encode a frame: size {}({})", size, payload.len());

        // 写入压缩后的长度
        buf.put_u32((payload.len() | COMPRESSION_BIT) as _); 
        
        // 把 BytesMut 再合并回来
        buf.unsplit(payload); 
        Ok(())
    } else {
        self.encode(buf)?;
        Ok(())
    }
}

    /// 把一个完整的Frame decode 成一个 Message
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError> {
        // 先取 4 字节，从中拿出长度和 compression bit
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        debug!("Got a frame: msg len {}, compressed {}", len, compressed);

        if compressed {
            // 解压缩
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf1 = Vec::with_capacity(len * 2);
            decoder.read_to_end(&mut buf1)?;
            buf.advance(len);

            // decode 成相应的消息
            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Value, CommandRequest, CommandResponse};
    use bytes::Bytes;

    #[test]
    fn command_request_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let cmd1 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd, cmd1);
    }

    #[test]
    fn command_response_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let values: Vec<Value> = vec![1.into(), "hello".into(), b"data".into()];
        let res: CommandResponse = values.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = Bytes::from(vec![0u8; COMPRESSION_LIMIT + 1]).into();
        let res: CommandResponse = value.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位设置了
        assert_eq!(is_compressed(&buf), true);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }
}
