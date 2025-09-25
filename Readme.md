# Tinify CLI


仅支持 macOS

## 下载
```zsh
# 注意修改 URL 为最新release版本的地址
mkdir -p ~/.tinifycli && curl -L -o ~/.tinifycli/tinifycli "<URL>" && chmod 755 ~/.tinifycli/tinifycli
```

## 设置环境变量
```zsh
echo 'echo 'export PATH="$HOME/.tinifycli:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## 使用方式
```zsh
tinifycli set <TINIFY KEY> # 保存 KEY 到本地（~/.tinifycli/key）
tinifycli <TINIFY KEY> # 本次使用提供的 KEY
tinifycli # 使用已保存的 KEY
```

TINIFY API_KEY 获取方式 https://tinify.cn/developers


## 功能 in future
- 压缩当前目录下的所有图片文件 ✅
- API KEY 设置与读取 ✅
- 异步压缩
- 更多的压缩选项
  - 压缩后删除原文件
  - 已经压缩的文件不再压缩（名称区分）
  - 压缩的质量
