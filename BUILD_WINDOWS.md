# Windows 构建说明

## 方法一：使用 GitHub Actions（推荐）

1. 将代码推送到 GitHub 仓库
2. 在 GitHub 上打开 Actions 标签页
3. 选择 "Build Windows" 工作流
4. 点击 "Run workflow" 按钮
5. 等待构建完成
6. 在 Releases 页面下载构建好的 Windows 安装包

## 方法二：在 Windows 机器上本地构建

如果你有 Windows 机器，可以按照以下步骤构建：

### 前置要求

1. 安装 [Rust](https://www.rust-lang.org/tools/install)
2. 安装 [Node.js](https://nodejs.org/) 和 pnpm
3. 安装 [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)

### 构建步骤

```bash
# 1. 克隆仓库
git clone <your-repo-url>
cd txt_to_excel

# 2. 安装依赖
pnpm install

# 3. 添加 Windows 目标
rustup target add x86_64-pc-windows-msvc

# 4. 构建 Windows 版本
pnpm tauri build --target x86_64-pc-windows-msvc
```

构建完成后，可执行文件会在 `src-tauri/target/x86_64-pc-windows-msvc/release/` 目录下。

## 方法三：使用 Docker（高级）

如果你熟悉 Docker，可以使用 Windows 容器来构建：

```bash
docker run --rm -v "$(pwd):/workspace" -w /workspace mcr.microsoft.com/windows/servercore:ltsc2022
# 然后在容器内安装 Rust 和 Node.js，然后构建
```

## 输出文件位置

构建成功后，Windows 安装包会在以下位置：
- MSI 安装包：`src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/txt_to_excel_0.1.0_x64_en-US.msi`
- NSIS 安装包：`src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/txt_to_excel_0.1.0_x64-setup.exe`

## 注意事项

- 在 macOS 上交叉编译到 Windows 需要额外的工具链配置，比较复杂
- 推荐使用 GitHub Actions 或直接在 Windows 机器上构建
- 构建的 exe 文件需要 Windows 10 或更高版本才能运行

