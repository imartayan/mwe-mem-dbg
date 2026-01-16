use mem_dbg::{MemDbg, MemSize};
use sux::dict::{EliasFanoBuilder, elias_fano::EfSeq};

#[derive(Clone, MemSize, MemDbg)]
pub struct Buf {
    pub pos: Vec<usize>,
    pub hash: Vec<u32>,
}

#[derive(Clone, MemSize, MemDbg)]
pub struct CompactBuf {
    pub pos: EfSeq,
    pub hash: Vec<u32>,
}

impl Buf {
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pos: Vec::with_capacity(capacity),
            hash: Vec::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn compact(self) -> CompactBuf {
        let len = self.pos.len();
        let max_pos = self.pos[len - 1];
        let mut efb = EliasFanoBuilder::new(len, max_pos);
        efb.extend(self.pos);
        CompactBuf {
            pos: efb.build_with_seq(),
            hash: self.hash,
        }
    }
}
