<div align="center">

# 🎮 SteamAuto

**基于 Tauri 2 + Vue 3 + Rust 构建的现代化 Steam 游戏管理与假入库工具**

[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8D8?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app/)
[![Vue 3](https://img.shields.io/badge/Vue-3.x-4FC08D?style=flat-square&logo=vuedotjs&logoColor=white)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.80+-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

</div>

---

## ✨ 核心特性

- 🚀 **智能极速入库**
  - **清单压缩包一键导入**：支持直接拖拽 `.zip` 下载清单压缩包，自动解压 Lua 脚本与 `depotcache` 清单文件并精准部署。
  - **单 Lua 脚本导入**：直接导入单个 `.lua` 脚本文件，智能提取游戏真名、AppID、DLC 清单与 DepotKey 密匙。
  - **Steam 商店链接抓取**：粘贴任意 Steam 商店游戏链接（如 `https://store.steampowered.com/app/2868840/`），自动抓取封面海报、游戏信息并一键生成标准 Lua 解锁脚本。
  - **自定义脚本编辑**：内置轻量代码编辑器，支持快速插入 `addappid`、`setManifestid`、`addtoken` 等指令。

- 🛡️ **内置 OpenSteamTool 核心套件**
  - 集成完整 Release 级 DLL 核心套件（`OpenSteamTool.dll`、`dwmapi.dll`、`xinput1_4.dll` 及 `opensteamtool.toml`）。
  - 支持**解锁模式**与**原生纯净模式**一键无缝切换，无需手动拷贝配置 DLL。
  - 支持一键环境检测、注册表 Steam 安装路径自动嗅探与 DLL 钩子修复。

- 👥 **多账号快速切换**
  - 自动检测本地所有已登录 Steam 账号的昵称、AccountID 及 SteamID64。
  - 一键免密快速切换账号并自动拉起重启 Steam 客户端。

- ⚡ **原生级性能与现代化交互**
  - 基于 **Tauri 2 + Rust** 驱动，内存占用极低（仅数十 MB），秒级秒启。
  - 赛博深色霓虹与紧凑型界面设计，完美适配分屏与小窗口桌面操作。
  - 实时监控 Steam 客户端进程状态与内存占用。

---

## 🖥️ 界面预览

- **已入库游戏（网格 / 列表双视图）**：直观展示游戏封面、AppID、已解锁 DLC 数量、Manifest 清单状态，支持一键在 Steam 中启动游戏。
- **添加游戏**：多模式导入（压缩包 / 单 Lua / 商店链接 / 自定义脚本）。
- **账号管理**：本地已登录账号列表与一键切换。
- **设置与工具**：Steam 安装路径配置、OpenSteamTool 核心套件状态监控、缓存一键清理与进程管理。

---

## 🛠️ 技术栈架构

- **前端界面 (Frontend)**:
  - Framework: [Vue 3](https://vuejs.org/) (Composition API, `<script setup>`)
  - Build Tool: [Vite](https://vitejs.dev/)
  - Language: [TypeScript](https://www.typescriptlang.org/)
  - Icons: [Lucide Vue Next](https://lucide.dev/)
  - Styling: 纯手写现代化响应式 CSS（玻璃拟态、暗色霓虹设计系统）

- **后端与系统交互 (Backend)**:
  - Framework: [Tauri 2.0](https://tauri.app/)
  - Core Language: [Rust](https://www.rust-lang.org/)
  - Crates: `winreg` (注册表检测), `sysinfo` (进程管理), `zip` / `flate2` (压缩包解析), `reqwest` (商店元数据接口), `encoding_rs` (多编码适配)

- **核心解锁底层**:
  - OpenSteamTool 核心钩子套件 (Detours / LuaJIT / 清单校验与密钥解密拦截)

---

## 📦 本地开发与构建

### 1. 环境准备
- [Node.js](https://nodejs.org/) (>= 18.0.0)
- [Rust & Cargo](https://www.rust-lang.org/) (>= 1.80.0)
- Windows 开发环境（推荐安装 C++ 编译工具链或 LLVM-MinGW）

### 2. 克隆仓库与安装依赖
```bash
git clone https://github.com/cww978/SteamAuto.git
cd SteamAuto

# 安装前端依赖
npm install
```

### 3. 本地启动开发环境
```bash
npm run tauri dev
```

### 4. 打包发布应用
```bash
npm run tauri build
```
编译产物将输出至 `src-tauri/target/release/` 目录。

---

## 📂 项目结构

```text
SteamAuto/
├── src/                        # 前端 Vue 3 源码
│   ├── api/                    # Tauri IPC 接口封装
│   ├── components/             # 通用组件（顶部导航、弹窗、编辑器等）
│   ├── styles/                 # 样式系统与变量
│   ├── types/                  # TypeScript 类型定义
│   ├── views/                  # 核心视图（游戏库、添加游戏、账号管理、设置）
│   ├── App.vue                 # 根组件
│   └── main.ts                 # 入口文件
├── src-tauri/                  # Rust 后端与 Tauri 配置
│   ├── icons/                  # 软件多分辨率应用图标
│   ├── resources/              # OpenSteamTool 嵌入式核心套件 DLL
│   ├── src/
│   │   ├── steam/              # Steam 业务模块（检测、进程、解析器、解锁器、商店 API 等）
│   │   ├── commands.rs         # Tauri IPC 指令分发
│   │   └── lib.rs              # Tauri 启动入口
│   └── tauri.conf.json         # Tauri 2 应用配置
├── public/                     # 静态公共资源
├── package.json
└── README.md
```

---

## ⚠️ 免责声明 (Disclaimer)

本项目仅供计算机技术研究、个人学习以及开源学术交流使用。请支持并购买正版游戏！

---

## 📄 开源许可证

本项目基于 [MIT License](LICENSE) 开源。
