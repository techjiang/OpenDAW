# v1.0.0

OpenDAW 首个可验收版本。

## 已交付

- 修复工作区编译所需的错误类型模块
- 补齐核心引擎、WAV 导出、项目 JSON 读写
- 补齐 CLI：info / new / render / help
- 补齐 WASM 绑定入口
- 增加发布目录与 GitHub Release 自动发布工作流
- 增加可复用的源码归档打包脚本

## 验证范围

- Rust workspace 单元测试与集成测试
- CLI 帮助、项目创建、项目渲染流程
- WAV 文件头与基础音频数据写入
- GitHub Release 发布配置
