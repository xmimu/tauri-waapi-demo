# Tauri WAAPI Demo

Tauri 桌面演示应用：通过 **waapi_rs** 调用 **WAAPI（Wwise Authoring API）** 连接并访问已运行的 **Wwise** 设计工具，用于查询 Wwise 信息、工程信息与执行 WAQL 查询。

## 实现方式

- **后端（Rust）**：使用 [waapi_rs](https://github.com/xmimu/waapi-rs)（当前依赖 `git = "https://github.com/xmimu/waapi-rs.git", branch = "dev"`）建立与 Wwise 的本地 WAAPI 连接。
- **前端**：通过 Tauri 的 `invoke` 调用 Rust 命令；Rust 侧使用 `WaapiClient::connect()` 与 Wwise 通信，调用 `ak.wwise.core.getInfo`、`ak.wwise.core.getProjectInfo`、`ak.wwise.core.object.get` 等。

## 已实现页面

| 路由 | 页面 | 说明 |
|------|------|------|
| `/` | 首页 | 欢迎与导航说明。 |
| `/wwise` | Wwise 信息 | 调用 `get_wwise_info`，展示 Wwise 会话、版本、目录等信息（对应 `ak.wwise.core.getInfo`）。 |
| `/project` | 工程信息 | 调用 `get_project_info`，展示当前工程名称、路径、语言、平台、目录等（对应 `ak.wwise.core.getProjectInfo`）。 |
| `/waql` | WAQL 查询 | 调用 `object_get`，输入 WAQL 与可选 return 属性，以分页表格展示查询结果（对应 `ak.wwise.core.object.get`）。 |

## 技术栈

- **桌面壳**：Tauri 2
- **后端**：Rust，waapi_rs、serde/serde_json、tokio；入口与命令在 `src-tauri/src/lib.rs`
- **前端**：Vue 3、Vue Router、TypeScript、Vite；UI 使用 Element Plus 与 Tailwind CSS；布局含顶栏导航与深色/浅色主题切换

## 环境与运行

### 前置条件

- 已安装 [Rust](https://www.rust-lang.org/)、[Node.js](https://nodejs.org/)、[Tauri 所需系统依赖](https://v2.tauri.app/start/install/)
- **需先启动 Wwise 并打开工程**，否则 WAAPI 连接会失败

### 安装与运行

```bash
npm install
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

## 许可证

本项目采用 [MIT](LICENSE) 许可证。
