# 文档构建工具说明

本文档介绍了用于构建和管理 mdbook 文档的各种工具和脚本。

## 🛠️ 可用工具

### 1. 内置 Python 脚本

#### `generate_structured_summary.py` (推荐)
生成结构化的 SUMMARY.md，按文档类型自动分类：
```bash
python3 generate_structured_summary.py
```

**特点：**
- 自动识别文档类型（概览、配置、指南、参考等）
- 按逻辑顺序组织文档结构
- 支持中文标题
- 自动提取文档标题

#### `generate_summary.py`
生成简单的 SUMMARY.md，按文件系统结构组织：
```bash
python3 generate_summary.py
```

**特点：**
- 按目录结构生成
- 适合简单的文档项目
- 自动提取标题

### 2. Makefile 命令

#### 基本命令
```bash
make help          # 显示所有可用命令
make build         # 构建 mdbook 文档
make serve         # 启动本地服务器 (http://localhost:3000)
make summary       # 生成结构化 SUMMARY.md
make summary-simple# 生成简单 SUMMARY.md
make validate      # 验证链接和格式
make clean         # 清理构建文件
make rebuild       # 完整重建 (清理+生成+构建)
```

#### 开发命令
```bash
make install       # 安装所需工具
make watch         # 监控文件变化自动重建 (需要 inotify-tools)
```

### 3. 第三方工具

#### mdbook-auto-summary
```bash
# 安装
cargo install mdbook-auto-summary

# 使用
mdbook-auto-summary
```

#### markdown-link-check
```bash
# 安装
npm install -g markdown-link-check

# 使用
markdown-link-check **/*.md
```

## 📁 项目结构

```
docs/
├── generate_summary.py           # 简单 SUMMARY 生成器
├── generate_structured_summary.py # 结构化 SUMMARY 生成器
├── Makefile                     # 构建命令集合
├── README_TOOLS.md              # 本文档
├── SUMMARY.md                   # 自动生成的目录
├── book/                        # mdbook 构建输出
└── ...                         # 文档内容
```

## 🚀 快速开始

### 1. 首次设置
```bash
# 安装所需工具
make install

# 生成初始 SUMMARY.md
make summary

# 构建文档
make build
```

### 2. 本地开发
```bash
# 启动本地服务器
make serve

# 在浏览器中访问 http://localhost:3000
```

### 3. 文档更新
```bash
# 添加新文档后，重新生成 SUMMARY.md
make summary

# 重新构建
make build
```

## 🔧 自定义配置

### 修改分类规则

编辑 `generate_structured_summary.py` 中的 `get_document_type()` 函数：

```python
def get_document_type(file_path):
    # 自定义分类逻辑
    path_parts = file_path.parts
    if path_parts[0] == 'my-category':
        return 'my_category'
    # ... 其他分类规则
```

### 修改章节顺序

编辑 `generate_structured_summary.py` 中的 `section_order` 列表：

```python
section_order = [
    'overview', 'getting-started', 'concepts',
    # 添加你的自定义顺序
]
```

### 修改忽略规则

编辑脚本中的 `should_ignore()` 函数：

```python
def should_ignore(file_path):
    ignore_patterns = [
        'SUMMARY.md',
        'generate_*.py',
        'YOUR_PATTERN',
    ]
```

## 📋 最佳实践

### 1. 文档命名规范
- 使用小写字母和连字符：`file-name.md`
- 目录名使用小写字母：`directory-name/`
- 标题使用中文或英文，保持一致性

### 2. 标题提取
- 确保每个文档都有明确的 H1 标题
- 标题应该简洁且具有描述性
- 避免在标题中使用特殊字符

### 3. 工作流程
1. 编写文档内容
2. 运行 `make summary` 更新目录
3. 运行 `make validate` 检查链接
4. 运行 `make build` 构建
5. 运行 `make serve` 本地预览

### 4. 自动化
```bash
# 设置 git hooks 自动更新 SUMMARY.md
echo '#!/bin/bash
cd docs
python3 generate_structured_summary.py
git add SUMMARY.md' > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## 🐛 故障排查

### 常见问题

#### 1. Python 脚本无法运行
```bash
# 检查 Python 版本
python3 --version

# 检查文件权限
chmod +x generate_*.py
```

#### 2. mdbook 未找到
```bash
# 安装 mdbook
curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.21/mdbook-v0.4.21-x86_64-apple-darwin.tar.gz | tar xz -C /usr/local/bin

# 或使用 homebrew
brew install mdbook
```

#### 3. 生成的 SUMMARY.md 格式错误
- 检查是否有重复的文件名
- 确认文档标题格式正确
- 检查文件编码是否为 UTF-8

### 调试模式

在脚本中添加调试信息：
```python
# 在 generate_structured_summary.py 中添加
print(f"Processing: {file_path}")
print(f"Type: {doc_type}")
print(f"Title: {title}")
```

## 📚 相关资源

- [mdBook 官方文档](https://rust-lang.github.io/mdBook/)
- [Markdown 语法指南](https://www.markdownguide.org/)
- [Python pathlib 文档](https://docs.python.org/3/library/pathlib.html)