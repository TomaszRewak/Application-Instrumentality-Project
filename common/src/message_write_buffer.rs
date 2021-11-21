use bytes::{Buf, BytesMut};
use prost::Message;
use std::io::Write;

pub struct MessageWriteBuffer {
    buffer: Option<BytesMut>,
}

impl MessageWriteBuffer {
    pub fn new() -> MessageWriteBuffer {
        MessageWriteBuffer {
            buffer: Some(BytesMut::with_capacity(1024)),
        }
    }

    pub fn encode<T: Message>(&mut self, message: &T) {
        let mut buffer = self.buffer.take().unwrap();

        message.encode_length_delimited(&mut buffer).unwrap();

        self.buffer = Some(buffer);
    }

    pub fn write(&mut self, target: &mut impl Write) {
        let mut buffer = self.buffer.take().unwrap();

        let result = target.write(&mut buffer);

        match result {
            Ok(size) => buffer.advance(size),
            Err(_) => {
                result.unwrap();
                todo!()
            }
        }

        self.buffer = Some(buffer);
    }
}
