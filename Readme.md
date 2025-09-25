# Tinify 图片压缩客户端

## 使用方式
仅支持macOS

### 下载并配置环境变量 tinifycli
```zsh
mkdir -p ~/.tinifycli && curl -L -o ~/.tinifycli/tinifycli "<URL>" && chmod 755 ~/.tinifycli/tinifycli
echo 'export TINIFY_KEY="$(cat ~/.tinifycli/key)"' >> ~/.zshrc
source ~/.zshrc
```

### v0.1
tinify_cli <TINIFY API_KEY>

TINIFY API_KEY获取方式 https://tinify.cn/developers

## 功能 in future
- API KEY 设置与读取
- 异步压缩
