use bytes::{Buf, BytesMut};
use prost::Message;
use std::io::Read;

pub struct MessageReadBuffer {
    buffer: Option<BytesMut>,

    next_message_length: usize,
    is_next_message_length_complete: bool,
    number_of_next_message_length_chunks: usize,
}

impl MessageReadBuffer {
    pub fn new() -> MessageReadBuffer {
        MessageReadBuffer {
            buffer: Some(BytesMut::with_capacity(1024)),
            next_message_length: 0,
            is_next_message_length_complete: false,
            number_of_next_message_length_chunks: 0,
        }
    }

    pub fn read(&mut self, source: &mut impl Read) {
        let mut buffer = self.buffer.take().unwrap();

        source.read(&mut buffer).unwrap();

        self.buffer = Some(buffer);
    }

    pub fn decode<T: Message + Default>(&mut self) -> Option<T> {
        self.process_message_size();
        self.process_message()
    }

    fn process_message_size(&mut self) {
        let mut buffer = self.buffer.take().unwrap();

        while !self.is_next_message_length_complete && !buffer.is_empty() {
            let message_size_chunk = buffer.get_u8() as usize;

            self.next_message_length = self.next_message_length
                | ((message_size_chunk & 0b0111_1111)
                    << (self.number_of_next_message_length_chunks * 7));

            if message_size_chunk & 0b1000_0000 != 0 {
                self.is_next_message_length_complete = true;
            }

            self.number_of_next_message_length_chunks += 1;
        }

        self.buffer = Some(buffer);
    }

    fn process_message<T: Message + Default>(&mut self) -> Option<T> {
        let buffer = self.buffer.take().unwrap();

        if !self.is_next_message_length_complete || buffer.len() < self.next_message_length {
            self.buffer = Some(buffer);
            return None;
        }

        let mut sub_buffer = buffer.take(self.next_message_length);
        let message = T::decode(&mut sub_buffer).unwrap();
        self.buffer = Some(sub_buffer.into_inner());

        Some(message)
    }
}
