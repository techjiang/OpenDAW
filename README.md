# OpenDAW

OpenDAW · Open Digital Audio Workstation Core Engine  
音乐人的 LibreOffice — 开源、现代、专业级音频生产引擎。

## 目标

提供可嵌入的多轨音频引擎与 DAW 核心能力：低延迟音频 I/O、非破坏性编辑、效果器链、插件宿主、MIDI 序列、混音台、项目系统与导出。

## 当前状态

仓库正在按里程碑推进。M1 聚焦音频引擎核心抽象、基础编辑与 WAV 导出，当前先落地可编译的 Rust workspace、核心 API 与基础测试骨架。

## 工作区

- `crates/opendaw-core`：核心引擎、轨道、片段、DSP、插件与项目导出
- `crates/opendaw-wasm`：WASM 绑定入口
- `crates/opendaw-cli`：命令行工具
- `app`：Tauri + React 桌面 UI

## 开发

需要 Rust stable。

```bash
cargo test --workspace
cargo run -p opendaw-cli -- --help
```

## 协议

GPL-3.0 © 2026 科技酱

## v1.0.0 可用能力

```bash
cargo run -p opendaw-cli -- info
cargo run -p opendaw-cli -- new demo 48000 demo.opendaw.json
cargo run -p opendaw-cli -- render demo.opendaw.json demo.wav
```

发布包位于 `Release/`，发布说明见 `Release/RELEASE_NOTES.md`。
