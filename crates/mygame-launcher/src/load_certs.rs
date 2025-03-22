#![cfg(not(target_family = "wasm"))]

use std::path::Path;
use std::error::Error;
use lightyear::prelude::server::Identity;

pub fn load_certificate_from_files(cert_path: &Path, key_path: &Path) -> Result<Identity, Box<dyn Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;
    
    let identity = rt.block_on(async {
        Identity::load_pemfiles(cert_path, key_path).await
    })?;
    
    Ok(identity)
}
