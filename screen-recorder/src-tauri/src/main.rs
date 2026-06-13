#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::State;

struct RecorderState {
    child: Mutex<Option<Child>>,
    output_path: Mutex<String>,
}

fn get_output_path() -> String {
    let desktop = dirs_desktop();
    format!("{}\\recording.mp4", desktop)
}

fn dirs_desktop() -> String {
    let user_profile = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
    format!("{}\\Desktop", user_profile)
}

#[tauri::command]
fn start_recording(state: State<RecorderState>) -> Result<(), String> {
    let mut child_guard = state.child.lock().unwrap();
    if child_guard.is_some() {
        return Err("已经在录制中".to_string());
    }

    let output_path = get_output_path();
    let _ = std::fs::remove_file(&output_path);

    let child = Command::new("ffmpeg")
        .args([
            "-f", "gdigrab",
            "-framerate", "30",
            "-i", "desktop",
            "-pix_fmt", "yuv420p",
            "-y",
            &output_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("启动 ffmpeg 失败: {}", e))?;

    *child_guard = Some(child);
    *state.output_path.lock().unwrap() = output_path;

    Ok(())
}

#[tauri::command]
fn stop_recording(state: State<RecorderState>) -> Result<String, String> {
    let mut child_guard = state.child.lock().unwrap();

    if let Some(mut child) = child_guard.take() {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(b"q\n");
        }

        let _ = child.wait();

        let path = state.output_path.lock().unwrap().clone();
        Ok(path)
    } else {
        Err("当前未在录制".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(RecorderState {
            child: Mutex::new(None),
            output_path: Mutex::new(String::new()),
        })
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
