#[derive(Copy, Clone, Debug)]
pub struct SmallByteBuf<const N: usize> {
    buf: [u8; N],
    len: u8,
}

impl<const N: usize> SmallByteBuf<N> {
    pub fn new(buf: [u8; N], len: u8) -> SmallByteBuf<N> {
        assert!(len as usize <= N);
        SmallByteBuf { buf, len }
    }

    pub fn from(buf: [u8; N], other: &[u8]) -> SmallByteBuf<N> {
        assert!(other.len() <= 255);
        assert!(other.len() <= N);
        let mut b = SmallByteBuf::new(buf, other.len() as u8);
        b.buf[0..other.len()].copy_from_slice(other);
        b
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[0..self.len as usize]
    }

    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.buf[0..self.len as usize]
    }
}
