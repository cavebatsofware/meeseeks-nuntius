# Meeseeks Nuntius Docker 配置

本文档说明如何使用 Docker 和 Docker Swarm 设置和运行 Meeseeks Nuntius。

## 先决条件

- Docker Engine 20.10+
- Docker Compose v2+
- 生产环境：已初始化的 Docker Swarm

## 环境配置

1. 复制示例环境文件：
   ```bash
   cp .env.example .env
   ```

2. 编辑 `.env` 文件进行配置：
   ```bash
   # 数据库配置
   POSTGRES_DB=meeseeks_nuntius
   POSTGRES_USER=postgres
   POSTGRES_PASSWORD=your-secure-password

   # API 配置
   API_PORT=8080
   RUST_LOG=info

   # 应用程序数据库 URL
   DATABASE_URL=postgres://postgres:your-secure-password@postgres:5432/meeseeks_nuntius
   ```

## 开发环境设置

用于支持热重载的本地开发：

```bash
# 启动开发环境
docker-compose -f docker-compose.dev.yml up

# 或者在后台模式运行
docker-compose -f docker-compose.dev.yml up -d

# 查看日志
docker-compose -f docker-compose.dev.yml logs -f api
```

## 生产部署

### 单节点（Docker Compose）

```bash
# 启动生产服务
docker-compose up -d

# 查看日志
docker-compose logs -f
```

### Docker Swarm（多节点）

1. 初始化 Docker Swarm（如果尚未完成）：
   ```bash
   docker swarm init
   ```

2. 部署堆栈：
   ```bash
   docker stack deploy -c docker-compose.yml meeseeks
   ```

3. 检查服务状态：
   ```bash
   docker service ls
   docker stack ps meeseeks
   ```

4. 扩展 API 服务：
   ```bash
   docker service scale meeseeks_api=3
   ```

## 数据库管理

### 迁移

迁移在 API 服务器启动时自动运行。手动运行迁移：

```bash
# 连接到 API 容器
docker exec -it meeseeks-api-1 /bin/bash

# 运行迁移（如果实现为 CLI）
cargo run --features server --bin migrate
```

### 数据库访问

```bash
# 连接到 PostgreSQL
docker exec -it meeseeks-postgres-1 psql -U postgres -d meeseeks_nuntius

# 备份数据库
docker exec -t meeseeks-postgres-1 pg_dump -U postgres meeseeks_nuntius > backup.sql

# 恢复数据库
docker exec -i meeseeks-postgres-1 psql -U postgres -d meeseeks_nuntius < backup.sql
```

## 健康检查

两个服务都包含健康检查：

- PostgreSQL：`pg_isready`
- API：HTTP 端点 `/health`

检查健康状态：
```bash
docker ps  # 显示健康状态
docker-compose ps  # 用于 compose 部署
```

## 故障排除

### 常见问题

1. **数据库连接失败**
   - 检查 PostgreSQL 容器是否正在运行且健康
   - 验证 `DATABASE_URL` 环境变量
   - 确保数据库凭据正确

2. **API 服务器无法启动**
   - 检查构建日志：`docker-compose logs api`
   - 验证所有环境变量都已设置
   - 确保 API 端口没有被占用

3. **迁移失败**
   - 确保数据库可访问
   - 检查迁移文件是否有语法错误
   - 验证数据库用户具有必要权限

### 日志

```bash
# 查看所有日志
docker-compose logs

# 查看特定服务日志
docker-compose logs postgres
docker-compose logs api

# 实时跟踪日志
docker-compose logs -f api
```

### 清理

```bash
# 停止并删除容器（保留卷）
docker-compose down

# 删除容器和卷（数据丢失！）
docker-compose down -v

# 删除所有内容包括镜像
docker-compose down -v --rmi all
```

## 安全注意事项

- 在生产环境中更改默认密码
- 对敏感环境变量使用密钥管理
- 考虑在生产部署中使用 SSL/TLS 证书
- 定期更新容器镜像以获得安全补丁