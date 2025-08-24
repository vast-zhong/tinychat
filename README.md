# tinychat

一个用 Rust 编写的超轻量本地聊天室示例，基于 Tokio 异步网络和 Crossterm 终端输入处理。

- 多客户端广播：所有已连接客户端都会收到任何一位用户发送的消息

## 目录
- [运行要求](#运行要求)
- [快速开始](#快速开始)
- [使用说明](#使用说明)
- [项目结构](#项目结构)
- [配置](#配置)
- [工作原理概览](#工作原理概览)
- [常见问题](#常见问题)
- [许可证](#许可证)

## 运行要求
- 已安装 Rust（建议使用最新稳定版，通过 rustup 安装）

## 快速开始
在两个终端窗口中分别运行服务端与客户端：

1. 终端 A（服务端）：
   - 运行：`cargo run --bin server`
   - 看到日志：`server run on 127.0.0.1:8080`

2. 终端 B（客户端）：
   - 运行：`cargo run --bin client`
   - 看到日志：`Connected to server at 127.0.0.1:8080`

3. 再打开更多终端运行客户端，即可进行多人聊天（所有客户端都会收到消息广播）。

## 使用说明
- 输入任意文本，按 Enter 发送
- Ctrl+C 可退出客户端

## 项目结构
- Cargo.toml：二进制目标配置（server、client）与依赖（tokio, crossterm）

## 配置
- 默认监听地址与端口：`127.0.0.1:8080`
  - 需要修改时，请在服务端与客户端源码中同步修改：
    - 服务端：<mcfile name="server.rs" path="f:\AllCode\RUST\tinychat\src\server.rs"></mcfile>（TcpListener::bind）
    - 客户端：<mcfile name="client.rs" path="f:\AllCode\RUST\tinychat\src\client.rs"></mcfile>（TcpStream::connect）

## 工作原理概览
- 服务端
  - 监听本地端口，接受连接，分配客户端 ID
  - 维护客户端写端集合，通过广播通道向所有客户端转发消息
  - 打印收到的消息，并广播“加入/离开”系统提示
- 客户端
  - 启用终端原始模式，监听键盘事件
  - 仅处理 KeyEventKind::Press，避免按键释放/重复导致的多次处理
  - 输入字符时本地回显；按下 Enter 发送前会清理本地当前输入行，之后仅显示服务端广播回来的消息

## 许可证
- 本项目使用 MIT 许可证，详见根目录 <mcfile name="LICENSE" path="f:\AllCode\RUST\tinychat\LICENSE"></mcfile>
