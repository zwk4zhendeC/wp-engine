use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Debug)]
pub struct KnowdbHandler {
    root: Arc<PathBuf>,
    conf: Arc<PathBuf>,
    authority_uri: Arc<String>,
    initialized: Arc<AtomicBool>,
}

impl KnowdbHandler {
    pub fn new(root: &Path, conf: &Path, authority_uri: &str) -> Self {
        Self {
            root: Arc::new(root.to_path_buf()),
            conf: Arc::new(conf.to_path_buf()),
            authority_uri: Arc::new(authority_uri.to_string()),
            initialized: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn mark_initialized(&self) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub fn ensure_thread_ready(&self) {
        if self.initialized.load(Ordering::SeqCst) {
            return;
        }
        match wp_knowledge::facade::init_thread_cloned_from_knowdb(
            &self.root,
            &self.conf,
            &self.authority_uri,
        ) {
            Ok(_) => {
                self.initialized.store(true, Ordering::SeqCst);
                info_ctrl!("init thread-cloned knowdb provider success ");
            }
            Err(err) => {
                warn_ctrl!("init thread-cloned knowdb provider failed: {}", err);
            }
        }
    }
}
