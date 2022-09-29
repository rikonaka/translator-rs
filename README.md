# rust-translator

一个用 rust 写的 PDF 论文简单实时翻译，翻译 API 为 Google 提供（主要支持 Linux 用户，Windows 用户也可以用但是貌似有比这还好的软件？）。

支持单个单词查看详细相似翻译

![example](./vids/example.gif)

# Linux 使用

## 使用之前请先安装 `xsel`

```bash
sudo apt install xsel
```

将 release 页面下的压缩包解压到本地，之后将二进制文件放到 `/usr/bin` 下（任何PATH目录都行），之后运行命令：

```
rust-translator
```

或者进入解压后的目录直接执行：

```
./rust-translator
```

## 使用

直接点选单词或者一段话既可（在 Ubuntu 22.04 Gnome&Wayland 桌面环境上测试过，还有 Kubuntu 的 Plasma 桌面，其他桌面没有测试过）。

## 如要自行编译

编译之前请先安装依赖（`Debian` or `Ubuntu`）

```bash
sudo apt install xsel xcb libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

## 测试

执行命令 `cargo run` 之前先得将 `.cargo/config.toml` 这个配置文件改成一个不能被 cargo 识别的配置名字（如 `config.toml.bak`）。

# Windows 使用

下载对应的 Windows 版本之后直接双击运行，和 Linux 版本不同的是，Windows 版本鼠标选出要翻译的文字之后，还要再按一个 `ctl-c`（复制）。

# 注

**根据最新的防火墙规则（GFW）已经将 Google 翻译 API 列入黑名单，所以新增 proxy 选项。**

所以如果出现访问超时的情况，请考虑为翻译软件设置代理，目前支持代理：

* https 代理
* socks5 代理

```bash
rust-translator --proxy socks5://192.168.122.67:1080
```

目前支持的翻译语种包含了

* english
* chinese
* japanese
* french
* german

如有需要，请使用如下命令来指定`源语言`和`目标语言`：

```bash
rust-translator --sourcelanguage enligh --targetlanguage french
```

或者缩写

```bash
rust-translator -s enligh -t french
```

如果觉得翻译速度慢可以使用 `fast` 模式（功耗可能会比 `slow` 模式高，默认是 `slow` 模式）：

```bash
rust-translator -m fast
```

release 页面有多个版本下载：
* Linux 一个版本版本是使用 `x86_64-unknown-linux-gnu` 静态编译的（Linux 默认的 glibc）
* Linux 另外个版本是使用 `x86_64-unknown-linux-musl` 静态编译的（占用空间小）
* Windows 版本是使用 `x86_64-pc-windows-gnu` 静态编译的（和 Windows 的默认编译器 MSVC 对比来说可能会有一些 bug 但是我没发现）
* ARM 本是使用 `aarch64-unknown-linux-gnu` 静态编译的
