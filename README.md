# 🎮 ValoCompAnalyser (无畏契约阵容分析器)

![Vue](https://img.shields.io/badge/Vue%203-4FC08D?style=for-the-badge&logo=vuedotjs&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

一款为《无畏契约》(Valorant) 玩家打造的现代化桌面应用。通过完全自定义的战术标签系统，实时诊断车队阵容的合理性，告别“排位选人选得好，进图发现没烟没闪”的尴尬局面！

## ✨ 核心特性

- **自定义战术标签**：不再局限于官方的四大职业，你可以自由定义“短烟”、“破点”、“守包”、“全图信息”等战术维度的标签。
- **灵活的角色管理**：根据你对游戏的理解，为不同特工分配不同的战术标签。
- **实时阵容面板**：在选人界面挑选5名特工，系统会立刻亮起绿灯或红灯，直观展示当前阵容是否具备所有“关键战术属性”。
- **智能配置合并**：配置文件支持本地导入/导出，且在游戏更新新特工后，导入旧版配置会自动补全缺失的新角色，配置永不丢失。

## 🚀 使用方法

1. **配置标签 (Tags)**：前往“标签管理”页面，添加你认为一个好阵容必须具备的战术要素，并将核心要素设为 `Key`（关键标签）。
2. **分配特工 (Agents)**：前往“角色管理”页面，点击特工头像，为他们勾选对应的战术标签。
3. **阵容诊断 (Analyzer)**：在主界面点击 `+` 号选择特工，下方的雷达面板会实时反馈该阵容是否满足了所有的关键标签需求。
4. **导出备份**：点击右上角的“📤 导出”按钮，将你的心血配置保存为 `.json` 文件。

## 🤝 欢迎分享你的个性化配置！

一千个玩家心里有一千种对《无畏契约》的理解。你对特工定位的划分是什么样的？
**强烈欢迎大家在 [Discussions (讨论区)](https://github.com/Badnuker/ValoCompAnalyser/discussions/categories/share) 发帖，附上你导出的 `.json` 配置文件并分享你的战术思路！** 
你可以通过导入其他玩家的配置，体验完全不同的角色理解视角！

## 🛠️ 本地开发与构建 (How to build)

如果你想自己修改代码并编译此项目，请按照以下步骤操作：

### 1. 环境准备
- 安装 [Node.js](https://nodejs.org/) (推荐 v18+)
- 安装 [Rust](https://www.rust-lang.org/tools/install)
- 确保安装了 Tauri 所需的系统级依赖（参考 [Tauri 官方环境配置文档](https://v2.tauri.app/start/prerequisites/)）

### 2. 克隆项目
```bash
git clone https://github.com/Badnuker/ValoCompAnalyser.git
cd ValoCompAnalyser
```

### 3. 安装前端依赖
```bash
npm install
```

### 4. 运行开发环境
```bash
npm run tauri dev
```

### 5. 打包生成可执行文件 (.exe)
```bash
npm run tauri build
```
编译完成后，安装包将位于 `src-tauri/target/release/bundle/` 目录下的对应操作系统文件夹中（Windows 通常在 `nsis` 文件夹内）。
