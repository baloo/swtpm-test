use std::{
    fs::create_dir,
    io::Write,
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::Duration,
};
use tempfile::TempDir;
use tss_esapi::{
    constants::startup_type::StartupType, tcti_ldr::TpmSimulatorConfig, Context, TctiNameConf,
};

pub use swtpm_test_macros::test;

pub fn run_with<B>(body: B)
where
    B: FnOnce(&mut Context) -> (),
{
    let tmp_dir = TempDir::with_prefix("swtpm-").expect("create swtpm temp dir");

    let state_dir = tmp_dir.path().join("state");
    create_dir(&state_dir).expect("create state dir for swtpm");

    let swtpm_pid = tmp_dir.path().join("pid");

    // `swtpm_sock` and `swtpm_ctrl_sock` are expected to be named the
    // same, but the ctrl with a `.ctrl` suffix.
    let swtpm_sock = tmp_dir.path().join("sock");
    let swtpm_ctrl_sock = tmp_dir.path().join("sock.ctrl");

    let child = Command::new("swtpm")
        .args([
            "socket",
            "--tpm2",
            "--tpmstate",
            &format!("dir={}", state_dir.display()),
            "--server",
            &format!("type=unixio,path={}", swtpm_sock.display()),
            "--ctrl",
            &format!("type=unixio,path={}", swtpm_ctrl_sock.display()),
            "--log",
            "level=20",
            "--flags",
            "startup-clear",
            "--pid",
            &format!("file={}", swtpm_pid.display()),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute child");

    let child = SubprocessKiller {
        child,
        ctrl_sock: swtpm_ctrl_sock,
    };
    child.wait_pidfile(&swtpm_pid);

    let config = TctiNameConf::Swtpm(TpmSimulatorConfig::Unix {
        path: swtpm_sock.to_str().unwrap().to_string(),
    });

    let mut context = Context::new(config).unwrap();
    context.startup(StartupType::Clear).unwrap();

    body(&mut context)
}

/// Watches for the creation of a pid file
///
/// When it goes out of scope, the pid is killed
struct SubprocessKiller {
    child: Child,
    ctrl_sock: PathBuf,
}

impl SubprocessKiller {
    fn wait_pidfile(&self, pid_file: &Path) {
        for _ in 0..30 {
            if pid_file.exists() {
                return;
            }
            sleep(Duration::from_millis(100));
        }
        panic!("swtpm didn't start as expected");
    }
}

impl Drop for SubprocessKiller {
    fn drop(&mut self) {
        // Send a PTM_STOP command to the swtpm
        if let Ok(mut s) = UnixStream::connect(&self.ctrl_sock) {
            let _ = s.write_all(&3u32.to_be_bytes()[..]);
        }

        let _ = self.child.kill();
    }
}
