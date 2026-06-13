# 桌面录屏工具 - MVP

## 1. 安装 ffmpeg（Windows）

1. 访问 https://www.gyan.dev/ffmpeg/builds/ 下载 `ffmpeg-release-essentials.zip`
2. 解压到例如 `C:\ffmpeg`
3. 将 `C:\ffmpeg\bin` 添加到系统环境变量 Path
4. 打开新的命令行窗口，执行 `ffmpeg -version` 确认安装成功

## 2. 环境准备

- 安装 Node.js: https://nodejs.org
- 安装 Rust: https://www.rust-lang.org/tools/install
- 安装 Tauri 依赖（Windows 需要 Microsoft Visual Studio C++ Build Tools，
  参考 https://tauri.app/start/prerequisites/）

## 3. 安装依赖并运行

在项目根目录下执行：

```bash
npm install
npm run tauri dev
```

首次运行会自动下载 Rust 依赖并编译，耗时较长，请耐心等待。

## 4. 测试录屏

1. 启动应用后点击【开始录屏】
2. 等待几秒（屏幕会被持续录制）
3. 点击【停止录屏】
4. 打开桌面，应看到 `recording.mp4` 文件，双击播放验证内容

## 注意事项

- ffmpeg 必须在系统 Path 中，否则点击开始录屏会报错
- 该方案仅在 Windows 上有效（使用 gdigrab）
- 录制过程中不要直接关闭程序，否则 ffmpeg 进程可能残留，
  需手动用任务管理器结束 ffmpeg.exe
