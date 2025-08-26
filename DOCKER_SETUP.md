# Docker Setup for Meeseeks Nuntius

This document explains how to set up and run Meeseeks Nuntius using Docker and Docker Swarm.

## Prerequisites

- Docker Engine 20.10+
- Docker Compose v2+
- For development: Rust toolchain installed locally
- For production: Docker Swarm initialized

## Environment Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your configuration:
   ```bash
   # Database Configuration
   POSTGRES_DB=meeseeks_nuntius
   POSTGRES_USER=postgres
   POSTGRES_PASSWORD=your-secure-password

   # API Configuration
   API_PORT=8080
   RUST_LOG=info

   # Database URL for application (choose based on environment)
   # Production (container-to-container):
   DATABASE_URL=postgres://postgres:your-secure-password@postgres:5432/meeseeks_nuntius
   
   # Development (host to container):
   # DATABASE_URL=postgres://postgres:devpassword@localhost:5432/meeseeks_nuntius_dev
   ```

## Development Setup

For local development, we run only PostgreSQL in Docker and the Rust server directly on the host for better development experience:

### 1. Start PostgreSQL

```bash
# Start PostgreSQL for development
docker-compose -f docker-compose.dev.yml up -d

# Check that it's running
docker-compose -f docker-compose.dev.yml ps
```

### 2. Set up your local environment

```bash
# Set the development database URL
export DATABASE_URL="postgres://postgres:devpassword@localhost:5432/meeseeks_nuntius_dev"

# Or add to your shell profile (.bashrc, .zshrc, etc.)
echo 'export DATABASE_URL="postgres://postgres:devpassword@localhost:5432/meeseeks_nuntius_dev"' >> ~/.bashrc
```

### 3. Run the server locally

```bash
# Run with server features for database support
cargo run --features server

# Or run a specific binary if you have multiple
cargo run --bin server --features server

# For development with debug logging
RUST_LOG=debug cargo run --features server
```

### 4. Development workflow

```bash
# View PostgreSQL logs
docker-compose -f docker-compose.dev.yml logs -f postgres

# Reset the development database
docker-compose -f docker-compose.dev.yml down -v
docker-compose -f docker-compose.dev.yml up -d
```

## Production Deployment

### Single Node (Docker Compose)

```bash
# Start production services
docker-compose up -d

# View logs
docker-compose logs -f
```

### Docker Swarm (Multi-node)

1. Initialize Docker Swarm (if not already done):
   ```bash
   docker swarm init
   ```

2. Deploy the stack:
   ```bash
   docker stack deploy -c docker-compose.yml meeseeks
   ```

3. Check service status:
   ```bash
   docker service ls
   docker stack ps meeseeks
   ```

4. Scale the API service:
   ```bash
   docker service scale meeseeks_api=3
   ```

## Database Management

### Migrations

Migrations run automatically when the API server starts. To run them manually:

```bash
# Connect to the API container
docker exec -it meeseeks-api-1 /bin/bash

# Run migrations (if implemented as CLI)
cargo run --features server --bin migrate
```

### Database Access

```bash
# Connect to PostgreSQL
docker exec -it meeseeks-postgres-1 psql -U postgres -d meeseeks_nuntius

# Backup database
docker exec -t meeseeks-postgres-1 pg_dump -U postgres meeseeks_nuntius > backup.sql

# Restore database
docker exec -i meeseeks-postgres-1 psql -U postgres -d meeseeks_nuntius < backup.sql
```

## Health Checks

Both services include health checks:

- PostgreSQL: `pg_isready`
- API: HTTP endpoint at `/health`

Check health status:
```bash
docker ps  # Shows health status
docker-compose ps  # For compose deployments
```

## Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Check if PostgreSQL container is running and healthy
   - Verify `DATABASE_URL` environment variable
   - Ensure database credentials are correct

2. **API Server Won't Start**
   - Check build logs: `docker-compose logs api`
   - Verify all environment variables are set
   - Ensure the API port is not already in use

3. **Migrations Failing**
   - Ensure database is accessible
   - Check migration files for syntax errors
   - Verify database user has necessary permissions

### Logs

```bash
# View all logs
docker-compose logs

# View specific service logs
docker-compose logs postgres
docker-compose logs api

# Follow logs in real-time
docker-compose logs -f api
```

### Cleanup

```bash
# Stop and remove containers (preserves volumes)
docker-compose down

# Remove containers and volumes (DATA LOSS!)
docker-compose down -v

# Remove everything including images
docker-compose down -v --rmi all
```

## Security Considerations

- Change default passwords in production
- Use secrets management for sensitive environment variables
- Consider using SSL/TLS certificates for production deployments
- Regularly update container images for security patches