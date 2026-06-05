# Image Filter - 图片筛选器

一个基于 Tauri 2.0 + Element Plus + Vite + TypeScript 构建的桌面图片筛选应用。

## 功能特性

- 📁 **文件夹选择**: 选择本地文件夹浏览所有图片
- 🖼️ **RAW 格式支持**: 支持预览 ARW、NEF、CR2、CR3、DNG 等单反相机 RAW 格式
- 👁️ **图片预览**: 缩略图网格浏览，点击可查看大图预览
- ✅ **多选功能**: 支持单选、全选、取消全选
- 📋 **批量复制**: 将选中的图片复制到目标文件夹
- 💾 **外接驱动器检测**: 自动检测并显示外接硬盘/USB 驱动器
- 🔔 **文件系统监听**: 监听文件夹变化，自动刷新图片列表
- 🎨 **现代化 UI**: 基于 Element Plus 的美观界面

## 支持的图片格式

- **常规格式**: JPG, JPEG, PNG, GIF, BMP, WebP, TIFF, TIF
- **RAW 格式**: ARW (Sony), NEF (Nikon), CR2/CR3 (Canon), DNG (Adobe), RAF (Fujifilm), ORF (Olympus), PEF (Pentax), SRW (Samsung), RW2 (Panasonic), IIQ (Phase One), 3FR (Hasselblad), FFF (Hasselblad), MOS (Mamiya), MEF (Mamiya), KDC (Kodak)

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **UI 框架**: Element Plus
- **后端框架**: Tauri 2.0
- **Rust 依赖**:
  - `tauri-plugin-fs`: 文件系统操作
  - `tauri-plugin-dialog`: 系统对话框
  - `tauri-plugin-shell`: Shell 功能
  - `notify`: 文件系统监听
  - `image`: 图片处理和缩略图生成
  - `base64`: Base64 编码
  - `kamadak-exif`: EXIF 数据读取

## 安装要求

1. **Node.js** >= 18.0.0
2. **Rust** >= 1.77.2
3. **Tauri CLI**: `cargo install tauri-cli`

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装依赖

```bash
cd image-filter
npm install
```

## 开发运行

```bash
# 开发模式（同时启动 Vite 和 Tauri）
npm run tauri dev

# 或者分别启动
npm run dev          # 启动 Vite 开发服务器
npm run tauri dev    # 启动 Tauri 应用
```

## 构建发布

```bash
# 构建生产版本
npm run tauri build

# 构建产物位于 src-tauri/target/release/bundle/
```

## 项目结构

```
image-filter/
├── src/                      # 前端源码
│   ├── App.vue              # 主应用组件
│   ├── main.ts              # 入口文件
│   ├── types.ts             # TypeScript 类型定义
│   └── components/          # Vue 组件
├── src-tauri/               # Tauri/Rust 后端
│   ├── src/
│   │   ├── lib.rs          # 主要 Rust 代码
│   │   └── main.rs         # 应用入口
│   ├── Cargo.toml          # Rust 依赖配置
│   ├── tauri.conf.json     # Tauri 配置
│   └── capabilities/        # 权限配置
├── package.json            # Node.js 依赖
└── vite.config.ts          # Vite 配置
```

## 主要功能说明

### 1. 选择文件夹
点击"Select Folder"按钮，通过系统对话框选择包含图片的文件夹。应用会自动扫描并加载所有支持的图片格式。

### 2. 外接驱动器
点击"Detect Drives"按钮或应用启动时自动检测外接驱动器。检测到后可直接点击驱动器标签加载其中的图片。

### 3. 图片浏览
- 网格视图显示所有图片缩略图
- 每张卡片显示文件名、格式和大小
- 悬停显示预览按钮
- 点击卡片选中/取消选中图片
- 选中状态以蓝色边框高亮显示

### 4. 图片预览
点击"Preview"按钮或图片卡片打开大图预览对话框，显示：
- 高清预览图
- 文件详细信息（路径、格式、大小）
- 快速选中/取消选中

### 5. 批量复制
1. 选中需要复制的图片（可多选）
2. 点击"Copy Selected"按钮
3. 选择目标文件夹
4. 确认复制

### 6. 文件监听
应用会监听当前文件夹的文件变化，当有新图片添加或删除时自动刷新列表。

## 注意事项

### RAW 文件预览
- 部分 RAW 格式可能包含嵌入的 JPEG 预览图，可直接显示
- 某些 RAW 格式可能需要额外的解码器支持
- 如无法生成预览，会显示"No Preview"提示

### 性能优化
- 缩略图大小限制为 200x200 像素
- 使用懒加载优化大量图片的加载性能
- 大文件夹可能需要数秒扫描时间

### 权限说明
应用需要以下系统权限：
- 文件系统读写权限（用于浏览和复制图片）
- 对话框权限（用于选择文件夹）

## 故障排除

### 构建失败
确保已正确安装 Rust 和 Tauri CLI：
```bash
rustc --version
cargo install tauri-cli
```

### 无法预览 RAW 文件
某些 RAW 格式可能需要特定的解码库。可以尝试：
1. 更新 `image` crate 到最新版本
2. 在 Cargo.toml 中启用更多图片格式支持

### 外部驱动器未检测到
- Windows: 确保驱动器已正确挂载
- macOS: 检查 /Volumes 目录
- Linux: 检查 /media 或 /mnt 目录

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
