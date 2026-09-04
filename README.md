<div align="center">

# 🎮 SteamAuto

**一款现代化、轻量高效的 Steam 假入库与游戏管理桌面工具**

[![Release](https://img.shields.io/github/v/release/cww978/SteamAuto?style=flat-square&color=00f2ff)](https://github.com/cww978/SteamAuto/releases/tag/v1.0.0)
[![Platform](https://img.shields.io/badge/platform-Windows-0077ff?style=flat-square)](https://github.com/cww978/SteamAuto)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg?style=flat-square)](LICENSE)

<br/>

[![Download](https://img.shields.io/badge/📥_前往下载-Release_v1.0.0-00f2ff?style=for-the-badge)](https://github.com/cww978/SteamAuto/releases/tag/v1.0.0)

</div>

---

## 📥 下载安装

最新版本及打包资产已发布至 GitHub Release，点击下方链接即可进入 Release 详情页或直接下载：

👉 **[前往 GitHub Release v1.0.0 页面下载](https://github.com/cww978/SteamAuto/releases/tag/v1.0.0)**

| 安装包类型 | 文件名 | 说明 | 下载直链 |
| :--- | :--- | :--- | :--- |
| **标准安装包 (推荐)** | `SteamAuto_1.0.0_x64-setup.exe` | 推荐，支持桌面图标与开始菜单安装引导 | [点击下载](https://github.com/cww978/SteamAuto/releases/download/v1.0.0/SteamAuto_1.0.0_x64-setup.exe) |
| **MSI 安装包** | `SteamAuto_1.0.0_x64_en-US.msi` | Windows 原生 MSI 格式安装程序 | [点击下载](https://github.com/cww978/SteamAuto/releases/download/v1.0.0/SteamAuto_1.0.0_x64_en-US.msi) |
| **绿色便携版** | `SteamAuto_1.0.0_portable.zip` | 免安装绿色包，解压即用 | [点击下载](https://github.com/cww978/SteamAuto/releases/download/v1.0.0/SteamAuto_1.0.0_portable.zip) |

---

## ✨ 功能特点

- 🚀 **智能极速入库**
  - **AppID 在线下载清单与 Key**：仅需输入 Steam 游戏 AppID，软件将自动从高速云端检索下载包含 `.manifest` 实体清单与带解密密钥的 `.lua` 脚本包，并自动解压部署至 Steam 核心目录。
  - **清单上游源灵活切换**：支持自由选择 **WUDRM**（高速源）、**OST**（官方源）、**SR**（备用源），配置自动实时同步至 `opensteamtool.toml`，客户端内点击下载即可动态拦截清单。
  - **清单压缩包一键导入**：直接拖拽 `.zip` 下载清单压缩包，自动解压 Lua 脚本与 `depotcache` 清单文件并精准部署。
  - **单 Lua 脚本导入**：支持导入独立的 `.lua` 文件，智能识别游戏真名、AppID、DLC 清单与 DepotKey 密匙。
  - **自定义脚本编辑**：内置代码编辑器，支持快速插入 `addappid`、`setManifestid` 等指令。

- 🛡️ **集成 OpenSteamTool 核心套件**
  - 内置完整核心套件（`OpenSteamTool.dll`、`dwmapi.dll`、`xinput1_4.dll` 及 `opensteamtool.toml`）。
  - 支持**解锁模式**与**原生纯净模式**一键无缝切换，无需手动配置系统文件。
  - 支持一键环境检测、注册表 Steam 安装路径自动嗅探与 DLL 钩子修复。

- 👥 **多账号快速切换**
  - 自动检测本地所有已登录 Steam 账号的昵称、AccountID 及 SteamID64。
  - 一键免密快速切换登录账号并自动拉起重启 Steam 客户端。

- ⚡ **轻量纯粹与现代化交互**
  - 极低系统资源占用，秒级快速启动。
  - 赛博暗黑美学设计，支持分屏与紧凑型小窗口自适应操作。
  - 实时监控 Steam 客户端运行状态。

---

## 🖥️ 模块介绍

| 模块 | 说明 |
| :--- | :--- |
| **已入库游戏** | 提供网格与列表双视图，直观展示游戏封面、AppID、已解锁 DLC 数量、Manifest 清单状态，支持快速启动与编辑管理 |
| **添加游戏** | 支持「本地导入」（.zip / .lua 拖拽）、「在线下载」（输入 AppID 下载清单实体与 Key）与「自定义脚本」 3 种添加模式 |
| **账号管理** | 识别本地多登录账号，支持一键免密切换 |
| **设置与工具** | Steam 安装路径配置、清单上游源切换、核心套件状态监控、清单缓存一键清理与 Steam 进程快速启停 |

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
