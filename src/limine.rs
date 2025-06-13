// limine requests, first try - probably wrong version
// read PROTOCOL.md quickly, using rev 0 for now

#[repr(C)]
pub struct BaseRevision {
    pub id: [u64; 2],
    pub rev: u64,
}

#[used]
#[link_section = ".limine_reqs"]
pub static BASE_REVISION: BaseRevision = BaseRevision {
    id: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
    rev: 0,
};

// trying memmap request without markers yet
#[repr(C)]
pub struct MemmapRequest {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: u64,
}

#[used]
#[link_section = ".limine_reqs"]
pub static MEMMAP_REQUEST: MemmapRequest = MemmapRequest {
    id: [
        0xc7b1dd30df4c8b88,
        0x0a82e883a194f07b,
        0x67cf3d9d378a806,
        0xe304acdfc50c3c4,
    ],
    revision: 0,
    response: 0,
};
