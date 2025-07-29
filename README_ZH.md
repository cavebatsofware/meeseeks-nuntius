<img width="512" height="512" alt="image" src="https://github.com/user-attachments/assets/9d509fb4-677a-4a0d-bc1e-ef104c98ba4c" />

[English]](https://github.com/cavebatsofware/meeseeks-nuntius/blob/main/README.md)
# 安全消息项目 ***代号 meeseeks-nuntius 短暂的-信使*** 

一个使用 Dioxus 构建的跨平台安全消息应用程序，采用端到端加密，利用 [RustCrypto](https://github.com/RustCrypto/nacl-compat/tree/master/crypto_box) 并实现匿名消息中继。

## 概述

该项目的目标是创建一个优先考虑消息安全和元数据隐私的消息平台。通过将加密与匿名中继系统相结合，用户可以在不泄露消息内容或发送者/接收者身份的情况下进行安全通信。 该项目应允许任何人快速轻松地设置一个临时安全消息服务。 它也可以用于 SaaS 模式。

## 主要功能 (计划中)

- **端到端加密**: 使用 [RustCrypto](https://github.com/RustCrypto/nacl-compat/tree/master/crypto_box) 加密消息
- **元数据保护**: 匿名中继系统防止将身份与消息关联
- **跨平台**: 使用 Dioxus 构建，可在桌面、Web 和移动平台上部署
- **认证匿名性**: 在不泄露用户身份的情况下进行加密验证
- **前向保密**:  使用单次使用的消息密钥以增强安全性

## 架构

该系统采用两阶段设置过程：

1. **身份生成**: 用户生成私有签名和加密密钥。
2. **消息密钥创建**: 获取一次性消息令牌，并用于获取消息密钥（消息密钥本质上类似于电话号码或电子邮件地址）。
3. **匿名中继**: 消息通过一个中继路由，该中继可以在不了解身份的情况下验证其真实性。

## 技术栈

- **框架**: [Dioxus](https://dioxuslabs.com/) - 基于 Rust 的跨平台 UI
- **密码学**: RustCrypto 实现
- **语言**: Rust

## 项目状态

🚧 **早期开发阶段** - 该项目处于初始规划和架构阶段。

## 开发设置

*即将推出 – 随着项目结构的建立，将添加构建说明*

## 贡献

该项目处于早期开发阶段。 如果您有兴趣贡献或提出建议，请打开一个 issue 进行讨论。

## 许可证

[GPL-3](https://www.gnu.org/licenses/gpl-3.0.txt)

---

**注意**: 该项目正在积极开发中。 功能、架构和实现细节可能会发生变化。
