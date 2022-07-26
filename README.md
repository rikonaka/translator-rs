# rust-translator

一个用 rust 写的 PDF 论文简单实时翻译，翻译 API 为 Google 提供（主要支持 Linux 用户，Windows 用户也可以用但是貌似有比这还好的软件？）。

![example](./vids/example.gif)

# Linux 使用

## 使用之前请先安装 `xsel`

```bash
sudo apt install xsel
```

## 编译之前请先安装依赖
```bash
sudo apt install xsel xcb libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

将 release 页面下的压缩包解压到本地，之后放到 `/usr/bin` 下或者直接执行即可。

# Windows 使用

下载对应的 Windows 版本之后直接双击运行。

# 注

release 页面有多个版本下载：
* Linux 一个版本版本是是使用 `x86_64-unknown-linux-gnu` 静态编译的
* Linux 另外个版本是使用 `x86_64-unknown-linux-musl` 静态编译的（占用空间更小）
* Windows 一个版本版本是是使用 `x86_64-pc-windows-gnu` 静态编译的
