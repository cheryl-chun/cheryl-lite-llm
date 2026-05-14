# Cheryl LLM Proxy

一个轻量级、生产就绪的大模型网关，用 Rust 实现。支持统一的 OpenAI 兼容 API 访问不同的 LLM 提供商，具备完整的密钥管理和访问控制系统。

## ✨ 核心特性

### 🔐 两层密钥系统
- **Master Key** (`mk_`)
  - 管理员专用，只能通过 CLI 生成
  - 用于调用 Admin API 管理 Virtual Keys
  - SHA256 哈希存储，原始密钥只显示一次
  
- **Virtual Key** (`sk_live_` / `sk_test_`)
  - 最终用户使用，通过 Admin API 创建
  - 用于调用 LLM API
  - 支持模型白名单、过期时间、禁用等控制

### 🚀 技术特性
- 异步 I/O（tokio + axum）
- 工厂模式支持 MySQL/PostgreSQL 切换
- Repository 模式抽象数据访问
- 类型安全的路由和错误处理

### 🌐 多提供商支持
- OpenAI
- 火山方舟 (Volcengine Ark)
- 易扩展新提供商

## 🚀 快速开始

### 前置要求

- Rust 1.75+
- MySQL 8.0+ 或 PostgreSQL 13+

### 安装

```bash
# 克隆项目
git clone <your-repo-url>
cd cheryl_llm_proxy

# 构建
cargo build --release
```

### 配置

创建 `config.toml`：

```toml
[server]
host = "127.0.0.1"
port = 3000

[database]
url = "mysql://user:password@localhost/cheryl_lite_llm"
# 或使用 PostgreSQL
# url = "postgres://user:password@localhost/cheryl_lite_llm"

[providers.ark]
api_key = "your-ark-api-key"
base_url = "https://ark.cn-beijing.volces.com/api/v3"

[providers.openai]
api_key = "your-openai-api-key"
```

### 启动服务

```bash
./target/release/cheryl-lite-llm server --config config.toml
```

## 🔧 CLI 使用

### 生成 Master Key

生成用于管理系统的 Master Key：

```bash
# 使用配置文件（推荐）
./cheryl-lite-llm generate-master-key \
  --config config.toml \
  --description "Admin key for production" \
  --expires-in-days 365

# 或直接指定数据库 URL
./cheryl-lite-llm generate-master-key \
  --database-url "mysql://root:pass@localhost/db" \
  --description "Temporary admin key"

# 生成永不过期的密钥
./cheryl-lite-llm generate-master-key \
  --config config.toml \
  --expires-in-days 0
```

**输出示例：**
```
✅ Master Key generated successfully!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⚠️  IMPORTANT: Save this key now! It will NOT be shown again.

  Key:         mk_a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0
  ID:          123e4567-e89b-12d3-a456-426614174000
  Expires:     Never
  Description: Admin key for production
```

### 列出所有 Master Keys

查看系统中所有的 Master Keys（不显示原始密钥）：

```bash
./cheryl-lite-llm list-masters --config config.toml
```

**输出示例：**
```
📋 Master Keys (3)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ ACTIVE | 123e4567-e89b-12d3-a456-426614174000 | 2024-01-15 10:30 | Production admin key
❌ DISABLED | 234f5678-f90c-23e4-b567-537725285111 | 2024-01-10 09:20 | Revoked key
⏰ EXPIRED | 345g6789-g01d-34f5-c678-648836396222 | 2024-01-01 08:15 | Old admin key
```

### 验证 Master Key

检查某个 Master Key 是否有效：

```bash
./cheryl-lite-llm verify-master \
  --key "mk_your_master_key_here" \
  --config config.toml
```

**输出示例（有效）：**
```
✅ Valid Master Key
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
ID:          123e4567-e89b-12d3-a456-426614174000
Enabled:     true
Created:     2024-01-15T10:30:00Z
Expires:     Never
Description: Production admin key
```

**输出示例（无效）：**
```
❌ Invalid Master Key
```

### 启用 Master Key

启用被禁用的 Master Key：

```bash
./cheryl-lite-llm enable-master \
  --id "123e4567-e89b-12d3-a456-426614174000" \
  --config config.toml
```

**输出：**
```
✅ Master Key 123e4567-e89b-12d3-a456-426614174000 enabled successfully
```

### 禁用 Master Key

禁用某个 Master Key（不删除记录）：

```bash
./cheryl-lite-llm disable-master \
  --id "123e4567-e89b-12d3-a456-426614174000" \
  --config config.toml
```

**输出：**
```
✅ Master Key 123e4567-e89b-12d3-a456-426614174000 disabled successfully
```

### Kubernetes 环境中使用

在 K8s 中通过 kubectl exec 执行 CLI 命令：

```bash
# 生成 Master Key
kubectl exec -it deployment/cheryl-llm -- \
  /app/cheryl-lite-llm generate-master-key \
  --config /etc/cheryl/config.toml \
  --description "K8s admin key"

# 列出 Master Keys
kubectl exec -it deployment/cheryl-llm -- \
  /app/cheryl-lite-llm list-masters \
  --config /etc/cheryl/config.toml
```

## 🌐 API 使用

### Admin API（需要 Master Key）

使用 Master Key 管理 Virtual Keys：

#### 创建 Virtual Key

```bash
curl -X POST http://localhost:3000/admin/keys \
  -H "Authorization: Bearer mk_your_master_key" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user_123",
    "models": ["gpt-4", "doubao-pro-32k"],
    "expires_in_days": 90,
    "description": "User 123 API key"
  }'
```

#### 列出所有 Virtual Keys

```bash
curl http://localhost:3000/admin/keys \
  -H "Authorization: Bearer mk_your_master_key"
```

#### 查看 Virtual Key 详情

```bash
curl http://localhost:3000/admin/keys/{key_id} \
  -H "Authorization: Bearer mk_your_master_key"
```

#### 禁用 Virtual Key

```bash
curl -X POST http://localhost:3000/admin/keys/{key_id}/revoke \
  -H "Authorization: Bearer mk_your_master_key"
```

#### 删除 Virtual Key

```bash
curl -X DELETE http://localhost:3000/admin/keys/{key_id} \
  -H "Authorization: Bearer mk_your_master_key"
```

### LLM API（需要 Virtual Key）

使用 Virtual Key 调用 LLM 模型：

#### 火山方舟

```bash
curl -X POST http://localhost:3000/ark/chat/completions \
  -H "Authorization: Bearer sk_live_your_virtual_key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "doubao-pro-32k",
    "messages": [
      {"role": "user", "content": "你好"}
    ]
  }'
```

#### OpenAI

```bash
curl -X POST http://localhost:3000/openai/chat/completions \
  -H "Authorization: Bearer sk_live_your_virtual_key" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4",
    "messages": [
      {"role": "user", "content": "Hello"}
    ]
  }'
```

### 公开 API

#### 健康检查

```bash
curl http://localhost:3000/health
```

## 📁 项目结构

```
src/
├── main.rs                      # 入口 + CLI 命令
├── lib.rs                       # 库根模块
├── models/                      # 数据模型
│   ├── message.rs
│   ├── request.rs
│   └── response.rs
├── adapters/                    # LLM 提供商适配器
│   ├── traits.rs                # Provider trait
│   ├── openai.rs
│   ├── ark.rs
│   ├── factory.rs               # 工厂模式
│   └── builder.rs
├── database/                    # 数据库层
│   ├── traits.rs                # Repository traits
│   ├── models.rs                # 实体模型
│   ├── mysql_repo.rs            # MySQL 实现
│   ├── pg_repo.rs               # PostgreSQL 实现
│   ├── factory.rs               # 数据库工厂
│   ├── context.rs               # 数据库上下文
│   └── builder.rs               # Builder 模式
├── middleware/                  # 认证中间件
│   ├── master_auth.rs           # Master Key 认证
│   └── virtual_auth.rs          # Virtual Key 认证
├── clis/                        # CLI 命令实现
│   ├── generate_master_key.rs
│   ├── enable_master_key.rs
│   └── disable_master_key.rs
├── server/                      # HTTP 服务
│   ├── mod.rs                   # 路由定义
│   ├── handler.rs               # 请求处理
│   └── state.rs                 # 应用状态
├── router/                      # 业务路由
│   └── mod.rs
├── config/                      # 配置管理
│   └── mod.rs
├── error/                       # 错误处理
│   └── types.rs
└── utils/                       # 工具函数
    └── utils.rs                 # 密钥生成、哈希等

tests/
└── mysql_repo_test.rs           # 数据库集成测试
```

## 🛠️ 技术栈

| 分类 | 技术 |
|------|------|
| **Web 框架** | axum 0.7 |
| **异步运行时** | tokio |
| **数据库** | sqlx (MySQL / PostgreSQL) |
| **HTTP 客户端** | reqwest |
| **序列化** | serde + serde_json |
| **日志** | tracing + tracing-subscriber |
| **错误处理** | thiserror + anyhow |
| **配置** | toml |
| **CLI** | clap |
| **密码学** | sha2 |
| **UUID** | uuid |

## 🔒 安全设计

1. **Master Key 只能通过 CLI 生成** - 需要服务器/K8s 访问权限，防止未授权创建
2. **密钥哈希存储** - 数据库只存储 SHA256 哈希，不存储原始密钥
3. **密钥只显示一次** - 生成后必须立即保存，无法再次查看
4. **双层认证** - 管理和使用权限分离，Master Key 不能调用 LLM API
5. **模型白名单** - Virtual Key 可限制访问的模型列表
6. **过期机制** - 支持密钥自动过期

## ✅ 测试

```bash
# 运行所有测试
cargo test

# 运行数据库测试（需要本地数据库）
cargo test --test mysql_repo_test -- --nocapture

# 设置测试数据库 URL
export TEST_DATABASE_URL="mysql://root:password@localhost/test_cheryl"
cargo test
```

## 📝 开发路线图

- [x] 核心路由功能
- [x] 多提供商适配器（OpenAI、火山方舟）
- [x] 两层密钥认证系统
- [x] 数据库抽象层（MySQL/PostgreSQL）
- [x] CLI 工具（Master Key 管理）
- [x] 数据库集成测试
- [x] Admin API 实现（Virtual Key CRUD）
- [x] 流式响应支持
- [ ] 请求限流和配额
- [ ] 完整的监控指标（Prometheus）
- [ ] Admin Web UI
- [ ] 更多 LLM 提供商（Anthropic、Gemini 等）
- [ ] Docker / Kubernetes 部署文档

## 📄 许可证

MIT

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！
