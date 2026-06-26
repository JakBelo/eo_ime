# EO IME

世界语输入法 (Esperanto Input Method Editor) — 一款轻量级的 Windows 输入法，让你轻松输入带帽子的世界语字母。

## ✨ 功能特性

- **多种输入方式**：支持 x-system (cx→ĉ, ux→ŭ)
- **轻量高效**：基于 Rust 编写，资源占用少，响应迅速
- **开箱即用**：无需安装额外运行库，下载即可使用
- **系统级支持**：在任意 Windows 应用程序中均可使用

## 📦 下载安装

### 系统要求
- Windows 11（64位）

### 安装步骤
1. 从 [Releases 页面](https://github.com/JakBelo/eo_ime/releases) 下载最新版本的 `eo_ime.exe`
2. 双击运行程序
3. 在系统语言栏中切换至 "世界语(美式键盘)" 即可开始输入

## ⌨️ 输入方法
| 输入 | 结果 | 输入 | 结果 |
|------|------|------|------|
| cx   | ĉ    | Cx   | Ĉ    |
| gx   | ĝ    | Gx   | Ĝ    |
| hx   | ĥ    | Hx   | Ĥ    |
| jx   | ĵ    | Jx   | Ĵ    |
| sx   | ŝ    | Sx   | Ŝ    |
| ux   | ŭ    | Ux   | Ŭ    |

## 🛠️ 从源码编译
```bash
# 克隆仓库
git clone https://github.com/JakBelo/eo_ime.git
cd eo_ime

# 编译
cargo build --release

# 编译产物位于 target/release/eo_ime.exe
