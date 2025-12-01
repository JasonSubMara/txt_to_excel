# TXT转Excel工具

一个基于 Tauri + Vue 3 开发的跨平台桌面应用，用于将特定格式的 TXT 文件转换为 Excel 文件。

## 功能特性

- 📁 选择本地 TXT 文件
- 📊 选择 Excel 模板文件
- 🔄 自动解析 TXT 数据并填充到 Excel
- 💾 自定义保存位置
- 🖥️ 跨平台支持（Windows、macOS、Linux）

## 快速开始

### 开发环境

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri dev
```

### 构建应用

#### 使用 GitHub Actions（推荐）

1. 将代码推送到 GitHub 仓库
2. 在 GitHub 仓库页面，点击 "Actions" 标签
3. 选择 "Build Windows" 工作流
4. 点击 "Run workflow" 按钮
5. 等待构建完成
6. 在 Releases 页面下载构建好的安装包

详细说明请查看 [GitHub Actions 文档](.github/README.md)

#### 本地构建

**Windows:**
```bash
rustup target add x86_64-pc-windows-msvc
pnpm tauri build --target x86_64-pc-windows-msvc
```

**macOS:**
```bash
pnpm tauri build
```

**Linux:**
```bash
pnpm tauri build
```

## TXT 文件格式

```
日期：2025-10-11
00:00 254
00:15 890
00:30 782
00:45 243
01:00 365
日期：2025-10-12
00:00 123
00:30 456
```

## Excel 模板格式

第一行应为列头，例如：
| 日期 | 00:00 | 00:30 | 01:00 | ...

## 技术栈

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue 3](https://vuejs.org/) - 前端框架
- [Vite](https://vitejs.dev/) - 构建工具
- [Rust](https://www.rust-lang.org/) - 后端语言

## 推荐 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
