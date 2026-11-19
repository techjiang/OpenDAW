# OpenDAW v1.0.0 Release

这是 OpenDAW 的可验收源码版本，面向“核心引擎可用、命令工具可运行、项目可保存、WAV 可导出”的 M1 上线目标。

## 包含内容

- `opendaw-core`：多轨音频引擎、片段编辑、DSP 缓冲与效果链、混音、时钟与走带、项目管理、WAV 导出
- `opendaw-cli`：项目创建、项目保存、WAV 渲染命令
- `opendaw-wasm`：浏览器/WASM 绑定的引擎入口
- `app/`：Tauri + React 桌面 UI 组件骨架

## 快速开始

```bash
cargo test --workspace
cargo run -p opendaw-cli -- info
cargo run -p opendaw-cli -- new demo 48000 demo.opendaw.json
cargo run -p opendaw-cli -- render demo.opendaw.json demo.wav
```

## 说明

本版本不是完整商业 DAW 替代品；VST3/CLAP/AU 宿主、MIDI 编辑器、虚拟乐器、母带处理等仍属于后续里程碑接口/骨架。
