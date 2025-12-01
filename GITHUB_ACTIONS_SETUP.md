# GitHub Actions 快速设置指南

## 📋 前置要求

1. 拥有一个 GitHub 账户
2. 代码已推送到 GitHub 仓库

## 🚀 快速开始

### 步骤 1: 推送代码到 GitHub

如果还没有 GitHub 仓库，请先创建：

```bash
# 初始化 git（如果还没有）
git init

# 添加所有文件
git add .

# 提交更改
git commit -m "Initial commit: TXT to Excel converter"

# 添加远程仓库（替换为你的仓库地址）
git remote add origin https://github.com/你的用户名/txt_to_excel.git

# 推送到 GitHub
git push -u origin main
```

### 步骤 2: 触发构建

#### 方法 A: 手动触发（最简单）

1. 打开你的 GitHub 仓库页面
2. 点击顶部的 **"Actions"** 标签
3. 在左侧工作流列表中选择 **"Build Windows"**
4. 点击右侧的 **"Run workflow"** 按钮
5. 选择分支（通常是 `main`）
6. 点击绿色的 **"Run workflow"** 按钮确认

#### 方法 B: 通过标签触发（用于发布版本）

```bash
# 创建并推送标签
git tag v0.1.0
git push origin v0.1.0
```

推送标签后，GitHub Actions 会自动开始构建。

### 步骤 3: 下载构建产物

构建完成后：

1. 如果使用 **Build Windows** 或 **Build All Platforms**：
   - 前往仓库的 **"Releases"** 页面
   - 下载最新版本的安装包（.msi 或 .exe）

2. 如果使用 **Manual Build**：
   - 在 **Actions** 页面找到完成的构建
   - 点击构建任务
   - 在页面底部找到 **"Artifacts"** 部分
   - 下载构建产物

## 📦 可用的工作流

### 1. Build Windows ⭐ 推荐
- **用途**: 快速构建 Windows 版本
- **触发**: 手动或推送标签
- **输出**: GitHub Release

### 2. Build All Platforms
- **用途**: 同时构建所有平台（Windows、macOS、Linux）
- **触发**: 手动或推送标签
- **输出**: GitHub Release

### 3. Manual Build
- **用途**: 仅构建，不上传 Release
- **触发**: 仅手动
- **输出**: Artifacts（保留 7 天）

## ⏱️ 构建时间

- **首次构建**: 15-25 分钟（需要下载和编译所有依赖）
- **后续构建**: 5-10 分钟（使用缓存）

## 🔍 查看构建状态

1. 在 GitHub 仓库页面点击 **"Actions"** 标签
2. 点击正在运行或已完成的构建任务
3. 查看详细的构建日志

## ❓ 常见问题

### Q: 构建失败怎么办？
A: 
1. 检查 Actions 页面的错误日志
2. 确保所有文件都已正确提交到 GitHub
3. 检查 `tauri.conf.json` 配置是否正确

### Q: 如何只构建 Windows 版本？
A: 使用 **"Build Windows"** 工作流

### Q: 构建产物在哪里？
A: 
- 使用 Build Windows/All Platforms: 在 **Releases** 页面
- 使用 Manual Build: 在构建任务的 **Artifacts** 部分

### Q: 可以自动构建吗？
A: 可以！推送标签（如 `v0.1.0`）会自动触发构建

## 📝 下一步

构建完成后，你可以：
1. 在 Windows 机器上测试安装包
2. 分发给用户使用
3. 创建新的标签发布新版本

## 🎉 完成！

现在你已经设置好了 GitHub Actions，可以随时通过 GitHub 界面构建 Windows 版本的应用程序了！

