use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use sysinfo::{ProcessesToUpdate, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamProcessStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub memory_mb: Option<f64>,
}

pub fn check_steam_status() -> SteamProcessStatus {
    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    let mut running = false;
    let mut main_pid = None;
    let mut total_memory_bytes: u64 = 0;

    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().to_lowercase();
        if name == "steam.exe" {
            running = true;
            main_pid = Some(pid.as_u32());
            total_memory_bytes += process.memory();
        } else if name == "steamwebhelper.exe" {
            total_memory_bytes += process.memory();
        }
    }

    let memory_mb = if running {
        Some((total_memory_bytes as f64) / (1024.0 * 1024.0))
    } else {
        None
    };

    SteamProcessStatus {
        is_running: running,
        pid: main_pid,
        memory_mb,
    }
}

pub fn kill_steam() -> Result<(), String> {
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "steam.exe", "/T"])
        .creation_flags_detached()
        .output();

    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "steamwebhelper.exe", "/T"])
        .creation_flags_detached()
        .output();

    // Wait up to 3 seconds for process to exit
    for _ in 0..15 {
        thread::sleep(Duration::from_millis(200));
        let status = check_steam_status();
        if !status.is_running {
            return Ok(());
        }
    }

    Ok(())
}

pub fn start_steam(steam_path: &Path) -> Result<(), String> {
    let steam_exe = steam_path.join("steam.exe");
    if !steam_exe.exists() {
        return Err(format!("未找到 Steam 可执行程序: {}", steam_exe.display()));
    }

    Command::new(&steam_exe)
        .current_dir(steam_path)
        .creation_flags_detached()
        .spawn()
        .map_err(|e| format!("启动 Steam 失败: {}", e))?;

    Ok(())
}

pub fn restart_steam(steam_path: &Path) -> Result<String, String> {
    let _ = kill_steam();
    thread::sleep(Duration::from_millis(1500));
    start_steam(steam_path)?;
    Ok("Steam 进程已重启并成功启动！".to_string())
}

pub fn launch_steam_game(appid: u32) -> Result<(), String> {
    let url = format!("steam://rungameid/{}", appid);
    Command::new("cmd")
        .args(["/c", "start", &url])
        .creation_flags_detached()
        .spawn()
        .map_err(|e| format!("拉起游戏失败: {}", e))?;
    Ok(())
}

trait CommandExt {
    fn creation_flags_detached(&mut self) -> &mut Self;
}

impl CommandExt for Command {
    fn creation_flags_detached(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            self.creation_flags(CREATE_NO_WINDOW);
        }
        self
    }
}
