use bytes::BytesMut;

pub struct MessageBuffer {
    buffer: BytesMut,
}

impl MessageBuffer {
    pub fn new() -> MessageBuffer {
        MessageBuffer {
            buffer: BytesMut::with_capacity(1024),
        }
    }
}
