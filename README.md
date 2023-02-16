# translator-rs

A simple real time translation of PDF papers written in rust for Linux users, with a translation API for Google.
To avoid misunderstandings, rename the program from `rust-translator` to `translator-rs`.

# Features

* A simple selection can be translated to speed up reading papers.
* Long sentence translation with automatic sentence break.
* Support single word to view detailed similar translation.

![example](./vids/example.gif)

# Installation

## Linux

Please install the `xsel` package before using it.


**Installation on Debian and Ubuntu**
```bash
sudo apt install xsel
```

**Installation on Fedora**
```bash
sudo dnf install xsel
```

Unpack the Linux package from the [release](https://github.com/rikonaka/translator-rs/releases) page, then place the binary file in `/usr/bin` (any `PATH` directory such `/usr/local/bin` you want).

### Self-compiling and installation

Please install the dependencies before compiling（`Debian` and `Ubuntu`）

```bash
sudo apt install xsel xcb libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

# Usage

## Linux

If the binary file is **already in the PATH** directory, you can run the following command directly.

```
translator-rs
```

If not, use the terminal run with following command.

```
./translator-rs
```

Click on a word or a paragraph (tested on Ubuntu 22.04 Gnome with Wayland and X11 desktop environment and Kubuntu22.04 Plasma desktop).

## Option Description

### Proxy Options

**The Google Translate API has been blacklisted according to the latest firewall rules (GFW), so a proxy option has been added.**

So if there is an access timeout, please consider setting a proxy for the translation software, which currently supports the following proxies.

* https proxy
* socks5 proxy

```bash
translator-rs --proxy socks5://192.168.122.67:1080
```

Or.

```bash
translator-rs -p socks5://192.168.122.67:1080
```

### Switching translation languages

The languages currently supported for translation include.

* English
* Chinese
* Japanese
* French
* German

If you need to translate into another language rather then default, use the following command to specify `source language` and `target language`.

```bash
translator-rs --sourcelanguage Engligh --targetlanguage French
```

Or.

```bash
translator-rs -s Engligh -t French
```

### Faster sampling speed

If you think the translation speed is slow, you can use `fast` mode (power consumption may be higher than `slow` mode, the default is `slow` mode).

```bash
translator-rs --mode fast
```

Or.

```bash
translator-rs -m fast
```

**Added support for some applications on Linux that do not automatically get selected text**

Some Linux applications that do not automatically get the selected text can now automatically translate it after copying the text via `ctrl-c`, like `Zotero`.

### Clear Screen Mode

**New clear screen mode**

The default parameter in this mode clears the previous translations for each `n` translation.

```bash
translator-rs --clear
```

Or.

```bash
translator-rs -c
```

If you want to clear the screen after three translations, you can use the following command.

```bash
translator-rs --clear 3
```

Or.

```bash
translator-rs -c 3
```

### Do not show original text

**New option of not showing original text**

If you want to not show the original text when translating, you can use the following options.

```bash
translator-rs --no-original
```

Or.

```bash
translator-rs -n
```

### No automatic sentence break

**Added no automatic sentence break**

If you do not want to break the sentence automatically, you can use the option.

```bash
translator-rs --disable-auto-break
```

Or.

```bash
translator-rs -d
```

# Why don't you support gui or tui?

Do not want to waste time in this area, can meet the use of the line 😘, in fact, there is no need for this.

You are welcome to submit any code to improve the program.
