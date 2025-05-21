# Rust-Solana 项目

拿AI随便撸的Solana智能合约项目。

## 环境要求

- Rust 1.70.0+
- Solana CLI 1.17.0+
- NodeJS 18+ (用于客户端集成)

## 项目结构

- `src/lib.rs` - Solana程序入口文件
- `src/instruction.rs` - 程序指令定义（待创建）
- `src/processor.rs` - 指令处理逻辑（待创建）
- `src/state.rs` - 程序状态管理（待创建）
- `src/error.rs` - 自定义错误处理（待创建）

## 构建与部署

### 构建项目

```bash
cargo build-bpf
```

### 部署到本地测试网

```bash
solana program deploy target/deploy/rust_solana.so
```

### 运行测试

```bash
cargo test-bpf
```