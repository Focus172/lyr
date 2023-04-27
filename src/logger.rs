//! Loging thing to help with debugging

pub struct Logger {
    buffer: String,
}

/// low-level error messages
impl Logger {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn log(&mut self, msg: &str) {
        self.buffer.push_str(msg);
    }
}

/*
    log[DGN_OK] = lang.err_dgn_oob;
    log[DGN_NULL] = lang.err_null;
    log[DGN_ALLOC] = lang.err_alloc;
    log[DGN_BOUNDS] = lang.err_bounds;
    log[DGN_DOMAIN] = lang.err_domain;
    log[DGN_MLOCK] = lang.err_mlock;
    log[DGN_XSESSIONS_DIR] = lang.err_xsessions_dir;
    log[DGN_XSESSIONS_OPEN] = lang.err_xsessions_open;
    log[DGN_PATH] = lang.err_path;
    log[DGN_CHDIR] = lang.err_chdir;
    log[DGN_PWNAM] = lang.err_pwnam;
    log[DGN_USER_INIT] = lang.err_user_init;
    log[DGN_USER_GID] = lang.err_user_gid;
    log[DGN_USER_UID] = lang.err_user_uid;
    log[DGN_PAM] = lang.err_pam;
    log[DGN_HOSTNAME] = lang.err_hostname;
*/
