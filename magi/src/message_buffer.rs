use std::io::Read;

use bytes::{Buf, BytesMut};
use prost::Message;

pub struct MessageBuffer {
    buffer: BytesMut,

    // let mut message_buffer = MessageBuffer::new();
    // let mut buffer = BytesMut::with_capacity(1024);
    next_message_length: usize,
    is_next_message_length_complete: bool,
    number_of_next_message_length_chunks: usize,
}

impl MessageBuffer {
    pub fn new() -> MessageBuffer {
        MessageBuffer {
            buffer: BytesMut::with_capacity(1024),
            next_message_length: 0,
            is_next_message_length_complete: false,
            number_of_next_message_length_chunks: 0,
        }
    }

    pub fn read(&mut self, source: &mut impl Read) {
        source.read(&mut self.buffer).unwrap();
    }

    fn decode<T: Message + Default>(&mut self) -> Option<T> {
        self.process_message_size();

        if self.is_next_message_length_complete && self.buffer.len() == self.next_message_length {
            let mut sub_buffer = self.buffer.take(self.next_message_length);
            let message = T::decode(&mut sub_buffer).unwrap();
            self.buffer = sub_buffer.into_inner();

            Some(message)
        } else {
            None
        }
    }

    fn process_message_size(&mut self) {
        while !self.is_next_message_length_complete && !self.buffer.is_empty() {
            let message_size_chunk = self.buffer.get_u8() as usize;

            self.next_message_length = self.next_message_length
                | ((message_size_chunk & 0b0111_1111)
                    << (self.number_of_next_message_length_chunks * 7));

            if message_size_chunk & 0b1000_0000 != 0 {
                self.is_next_message_length_complete = true;
            }

            self.number_of_next_message_length_chunks += 1;
        }
    }
}
