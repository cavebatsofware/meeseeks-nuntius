<img width="512" height="512" alt="image" src="https://github.com/user-attachments/assets/9d509fb4-677a-4a0d-bc1e-ef104c98ba4c" />

[中国人](https://github.com/cavebatsofware/meeseeks-nuntius/blob/main/README_ZH.md)
# Secure Messaging Platform ***code meeseeks-nuntius***

A cross-platform secure messaging application built with Dioxus, implementing end-to-end encryption using the OpenPGP standard and anonymous message relay.

## Overview

This project aims to create a messaging platform that prioritizes both message security and metadata privacy. By combining OpenPGP encryption with an anonymous relay system, users can communicate securely without the relay service knowing message contents or sender/recipient identities. This project should allow anyone to quickly and easily setup an ad-hoc secure message service. It could also be used in a SaaS capacity for a service fee.

## Key Features (Planned)

- **End-to-End Encryption**: Messages encrypted using the OpenPGP standard
- **Metadata Protection**: Anonymous relay system prevents linking of identities to messages
- **Cross-Platform**: Built with Dioxus for deployment across desktop, web, and mobile platforms
- **Authenticated Anonymity**: Cryptographic verification without revealing user identities
- **Forward Secrecy**: Single-use message keys for enhanced security

## Architecture

The system uses a two-stage setup process:

1. **Identity Generation**: Users generate private signing and encryption keys
2. **Message Key Creation**: Single-use message tokens are acquired and used to get to message keys ( message key is essentially like a phone number or email).
3. **Anonymous Relay**: Messages are routed through a relay that can verify authenticity without learning identities. Message keys are registered with a public key (provided by the client) that they must be signed by (clients private key does the actual signing) to verify message sender is authorized.

## Technology Stack

- **Framework**: [Dioxus](https://dioxuslabs.com/) - Rust-based cross-platform UI
- **Cryptography**: OpenPGP standard implementation
- **Language**: Rust

## Project Status

🚧 **Early Development** - This project is in the initial planning and architecture phase.

## Development Setup

*Coming soon - Build instructions will be added as the project structure is established*

## Contributing

This project is in early development. If you're interested in contributing or have suggestions, please open an issue for discussion.

## License

LGPL-3

---

**Note**: This project is under active development. Features, architecture, and implementation details are subject to change.
