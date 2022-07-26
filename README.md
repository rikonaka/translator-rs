# rust-translator

一个用 rust 写的 PDF 论文简单实时翻译，翻译 API 为 Google 提供（只支持 Linux 用户）。

![example](./vids/example.gif)

# 安装之前请先安装xsel

```bash
sudo apt install xsel
```

# 使用

将 release 页面下的压缩包解压到本地，之后放到 `/usr/bin` 下或者直接执行即可。

# 注

release 页面有两个版本下载：
* `musl` 版本是使用 `x86_64-unknown-linux-musl` 静态编译的
* `gnu` 版本是是使用 `x86_64-unknown-linux-gnu` 静态编译的
