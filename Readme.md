# Tinify 图片压缩客户端

## 使用方式

仅支持 macOS

### 下载并配置环境变量 tinifycli

```zsh
mkdir -p ~/.tinifycli && curl -L -o ~/.tinifycli/tinifycli "<URL>" && chmod 755 ~/.tinifycli/tinifycli
echo 'echo 'export PATH="$HOME/.tinifycli:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

tinifycli set <TINIFY KEY> # 保存 KEY 到本地（~/.tinifycli/key）
tinifycli <TINIFY KEY> # 本次使用提供的 KEY
tinifycli # 使用已保存的 KEY

TINIFY API_KEY 获取方式 https://tinify.cn/developers

## 功能 in future

- API KEY 设置与读取
- 异步压缩
