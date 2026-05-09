# Cheryl LLM Proxy

一个轻量级的大模型网关，用 Rust 实现，类似 LiteLLM 的核心功能。支持统一的 API 接口访问不同的 LLM 提供商。

## 特性

- 统一的 OpenAI 兼容 API 格式
- 支持多个 LLM 提供商
  - OpenAI
  - 火山方舟 (Volcengine Ark)
- 高性能异步架构（基于 tokio + axum）
- 类型安全的路由和错误处理
- 简单的配置文件管理

## 快速开始

### 安装依赖

需要 Rust 1.75+ 工具链：

```bash
# 安装 Rust（如果还没有）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 克隆项目
git clone <your-repo-url>
cd cheryl_llm_proxy
```

### 配置

创建 `config.toml` 文件（参考 `config.example.toml`）：

```toml
[server]
host = "127.0.0.1"
port = 3000

[providers.ark]
api_key = "your-ark-api-key"
base_url = "https://ark.cn-beijing.volces.com/api/v3"

[providers.openai]
api_key = "your-openai-api-key"
# base_url = "https://api.openai.com/v1"  # 可选
```

### 运行

```bash
# 开发模式
cargo run

# 生产构建
cargo build --release
./target/release/cheryl_llm_proxy
```

## API 使用

### 请求格式

```bash
POST /{provider}/chat/completions
```

路径参数：
- `provider`: 提供商名称（如 `ark`, `openai`）

### 示例

使用火山方舟：

```bash
curl -X POST http://127.0.0.1:3000/ark/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "doubao-pro-32k",
    "messages": [
      {"role": "user", "content": "你好"}
    ]
  }'
```

使用 OpenAI：

```bash
curl -X POST http://127.0.0.1:3000/openai/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4",
    "messages": [
      {"role": "user", "content": "Hello"}
    ]
  }'
```

### 健康检查

```bash
curl http://127.0.0.1:3000/health
```

## 项目结构

```
src/
├── main.rs              # 程序入口
├── lib.rs               # 库根模块
├── models/              # 数据模型（请求/响应）
│   ├── mod.rs
│   ├── message.rs
│   ├── request.rs
│   └── response.rs
├── adapters/            # LLM 提供商适配器
│   ├── mod.rs
│   ├── traits.rs        # Provider trait 定义
│   ├── openai.rs        # OpenAI 适配器
│   └── ark.rs           # 火山方舟适配器
├── router/              # 业务路由
│   └── mod.rs
├── server/              # HTTP 服务
│   ├── mod.rs
│   ├── handler.rs       # 请求处理
│   └── state.rs         # 应用状态
├── config/              # 配置管理
│   └── mod.rs
├── error/               # 错误处理
│   ├── mod.rs
│   └── types.rs
└── middleware/          # 中间件（预留）
    └── mod.rs
```

## 技术栈

- **Web 框架**: axum 0.7
- **异步运行时**: tokio
- **HTTP 客户端**: reqwest
- **序列化**: serde + serde_json
- **日志**: tracing + tracing-subscriber
- **错误处理**: thiserror + anyhow
- **配置**: toml

## 开发路线图

- [x] 核心路由功能
- [x] OpenAI 和火山方舟适配器
- [x] 基础错误处理
- [x] 配置文件管理
- [ ] 鉴权中间件
- [ ] 流式响应支持
- [ ] 请求限流和配额
- [ ] 日志和监控
- [ ] 更多 LLM 提供商（Anthropic、Gemini 等）

## 许可证

MIT

## 贡献

欢迎提交 Issue 和 Pull Request。
