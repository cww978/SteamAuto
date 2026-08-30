<div align="center">

# 🎮 SteamAuto

**一款现代化、轻量高效的 Steam 假入库与游戏管理桌面工具**

[![Version](https://img.shields.io/badge/version-1.0.0-00f2ff?style=flat-square)](https://github.com/cww978/SteamAuto)
[![Platform](https://img.shields.io/badge/platform-Windows-0077ff?style=flat-square)](https://github.com/cww978/SteamAuto)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg?style=flat-square)](LICENSE)

</div>

---

## ✨ 功能特点

- 🚀 **智能极速入库**
  - **清单压缩包一键导入**：直接拖拽 `.zip` 下载清单压缩包，自动解压 Lua 脚本与 `depotcache` 清单文件并精准部署。
  - **单 Lua 脚本导入**：支持导入独立的 `.lua` 文件，智能识别游戏真名、AppID、DLC 清单与 DepotKey 密匙。
  - **Steam 商店链接抓取**：粘贴任意 Steam 商店链接（如 `https://store.steampowered.com/app/2868840/`），自动解析游戏元数据、封面海报并一键生成标准入库脚本。
  - **自定义脚本编辑**：内置代码编辑器，支持快速插入 `addappid`、`setManifestid`、`addtoken` 等指令。

- 🛡️ **集成 OpenSteamTool 核心套件**
  - 内置完整核心套件（`OpenSteamTool.dll`、`dwmapi.dll`、`xinput1_4.dll` 及 `opensteamtool.toml`）。
  - 支持**解锁模式**与**原生纯净模式**一键无缝切换，无需手动配置系统文件。
  - 支持一键环境检测、注册表 Steam 安装路径自动嗅探与 DLL 钩子修复。

- 👥 **多账号快速切换**
  - 自动检测本地所有已登录 Steam 账号的昵称、AccountID 及 SteamID64。
  - 一键免密快速切换登录账号并自动拉起重启 Steam 客户端。

- ⚡ **轻量纯粹与现代化交互**
  - 极低系统资源占用，秒级快速启动。
  - 赛博深色暗黑美学，完美适配分屏与紧凑型小窗口操作。
  - 实时监控 Steam 客户端运行状态与内存占用。

---

## 🖥️ 模块介绍

| 模块 | 说明 |
| :--- | :--- |
| **已入库游戏** | 提供网格与列表双视图，直观展示游戏封面、AppID、已解锁 DLC 数量、Manifest 清单状态，支持快速启动与编辑管理 |
| **添加游戏** | 支持压缩包拖拽、单 Lua 文件导入、商店链接抓取与自定义脚本 4 种添加模式 |
| **账号管理** | 识别本地多登录账号，支持一键免密切换 |
| **设置与工具** | Steam 安装路径配置、核心套件状态监控、清单缓存一键清理与 Steam 进程快速启停 |

---

## 📦 运行与构建

### 安装依赖
```bash
npm install
```

### 启动应用
```bash
npm run tauri dev
```

### 编译打包
```bash
npm run tauri build
```

---

## ⚠️ 免责声明

本项目仅供个人学习、计算机技术研究与开源交流使用。请支持并购买正版游戏！

---

## 📄 开源许可证

本项目基于 [MIT License](LICENSE) 开源。
