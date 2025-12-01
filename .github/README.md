# GitHub Actions 构建说明

本项目包含多个 GitHub Actions 工作流，用于自动构建应用程序。

## 可用的工作流

### 1. Build Windows（推荐用于快速构建 Windows 版本）

**触发方式：**
- 手动触发（workflow_dispatch）
- 推送标签（v*）
- 推送到 main/master 分支

**用途：** 快速构建 Windows 版本的应用程序并创建 GitHub Release。

**使用方法：**
1. 在 GitHub 仓库页面，点击 "Actions" 标签
2. 选择 "Build Windows" 工作流
3. 点击 "Run workflow" 按钮
4. 选择分支（通常是 main）
5. 点击 "Run workflow" 确认

### 2. Build All Platforms（构建所有平台）

**触发方式：**
- 手动触发（workflow_dispatch）
- 推送标签（v*）

**用途：** 同时构建 macOS、Linux 和 Windows 版本。

**使用方法：**
1. 在 GitHub 仓库页面，点击 "Actions" 标签
2. 选择 "Build All Platforms" 工作流
3. 点击 "Run workflow" 按钮
4. 选择分支和标签
5. 点击 "Run workflow" 确认

### 3. Manual Build（手动构建，不上传 Release）

**触发方式：**
- 仅手动触发（workflow_dispatch）

**用途：** 仅构建应用程序，不上传到 GitHub Release，构建产物作为 Artifacts 下载。

**使用方法：**
1. 在 GitHub 仓库页面，点击 "Actions" 标签
2. 选择 "Manual Build" 工作流
3. 点击 "Run workflow" 按钮
4. 构建完成后，在 Actions 页面下载 Artifacts

## 构建产物位置

### Windows
- MSI 安装包：`src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/*.msi`
- NSIS 安装包：`src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe`
- 可执行文件：`src-tauri/target/x86_64-pc-windows-msvc/release/*.exe`

### macOS
- DMG：`src-tauri/target/release/bundle/dmg/*.dmg`
- App Bundle：`src-tauri/target/release/bundle/macos/*.app`

### Linux
- AppImage：`src-tauri/target/release/bundle/appimage/*.AppImage`
- DEB：`src-tauri/target/release/bundle/deb/*.deb`

## 注意事项

1. **首次构建可能需要较长时间**（10-20 分钟），因为需要下载和编译所有依赖
2. **后续构建会更快**，因为会缓存依赖
3. **构建产物会在 GitHub Release 页面**（如果使用 Build Windows 或 Build All Platforms）
4. **Artifacts 会保留 7 天**（如果使用 Manual Build）

## 故障排除

如果构建失败：
1. 检查 Actions 页面的错误日志
2. 确保所有依赖都已正确配置
3. 检查 Rust 和 Node.js 版本是否兼容
4. 查看构建日志中的详细错误信息

