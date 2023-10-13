static mut FINALIZED_BLOCK_HASH: Option<String> = None;
static mut BLOCK_HEADER: Option<String> = None;
static mut EXTRINSICS: Option<String> = None;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub fn set_finalized_block_hash(hash: String) {
    unsafe {
        FINALIZED_BLOCK_HASH = Some(hash);
    }
}

pub fn get_finalized_block_hash() -> Option<String> {
    unsafe { FINALIZED_BLOCK_HASH.clone() }
}

pub fn set_block_header(header: String) {
    unsafe {
        BLOCK_HEADER = Some(header);
    }
}

pub fn get_block_header() -> Option<String> {
    unsafe { BLOCK_HEADER.clone() }
}

pub fn set_extrinsics(extrinsics: String) {
    unsafe {
        EXTRINSICS = Some(extrinsics);
    }
}

pub fn get_extrinsics() -> Option<String> {
    unsafe { EXTRINSICS.clone() }
}


static CHAIN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub fn set_chain(chain: String) {
    let mut chain_data = CHAIN.lock().unwrap();
    *chain_data = Some(chain);
}

pub fn get_chain() -> Option<String> {
    let chain_data = CHAIN.lock().unwrap();
    chain_data.clone()
}

static EXTRINSIC_HASH: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

pub fn set_extrinsic_hash(hash: String) {
    let mut extrinsic_hash = EXTRINSIC_HASH.lock().unwrap();
    *extrinsic_hash = Some(hash);
}

pub fn get_extrinsic_hash() -> Option<String> {
    let extrinsic_hash = EXTRINSIC_HASH.lock().unwrap();
    extrinsic_hash.clone()
}