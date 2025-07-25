// limine requests - fixed to rev 3 with markers after reading docs

#[repr(C)]
pub struct RequestsStartMarker {
    pub id: [u64; 4],
}

#[repr(C)]
pub struct RequestsEndMarker {
    pub id: [u64; 2],
}

#[repr(C)]
pub struct BaseRevision {
    pub id: [u64; 2],
    pub rev: u64,
}

#[used]
#[link_section = ".requests_start_marker"]
pub static START_MARKER: RequestsStartMarker = RequestsStartMarker {
    id: [
        0xf6b8f4b39de7d1ae,
        0xfab91a6940fcb9cf,
        0x785c6ed015d3e316,
        0x181e920a7852b9d9,
    ],
};

#[used]
#[link_section = ".requests_end_marker"]
pub static END_MARKER: RequestsEndMarker = RequestsEndMarker {
    id: [0xadc0e0531bb10d03, 0x9572709f31764c62],
};

#[used]
#[link_section = ".requests"]
pub static BASE_REVISION: BaseRevision = BaseRevision {
    id: [0xf9562b2d5c95a6c8, 0x6a7b384944536bdc],
    rev: 3,
};
