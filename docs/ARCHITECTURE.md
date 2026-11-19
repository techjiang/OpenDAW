# OpenDAW 架构说明

## 分层

- `opendaw-core`：纯 Rust 核心，负责引擎、轨道、片段、DSP、项目、导出。
- `opendaw-wasm`：把核心引擎绑定到浏览器环境。
- `opendaw-cli`：批处理与诊断入口。
- `app`：Tauri + React 桌面界面。

## M1 范围

M1 完成最小可验证链路：创建引擎 → 添加音轨 → 添加非破坏性音频片段 → 播放推进 → 项目快照 → WAV 导出。

后续里程碑在此骨架上扩展：内置效果器、录音、VST3/CLAP 宿主、MIDI、虚拟乐器、混音台与母带。
