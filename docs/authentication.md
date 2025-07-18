# Authentication System Documentation

## Overview

The agentgateway now includes a comprehensive, enterprise-grade authentication system that provides secure user login, session management, role-based access control (RBAC), and OAuth2 integration for the web UI.

## Features

### Core Authentication Features
- **Session Management**: JWT-based sessions with secure cookie handling
- **Password Authentication**: Traditional username/password login with bcrypt hashing
- **OAuth2 Integration**: Support for Google, GitHub, and other OAuth2 providers
- **Role-Based Access Control**: Fine-grained permissions based on user roles
- **Multi-Provider Support**: File-based and database user storage

### Security Features
- **CSRF Protection**: Cross-site request forgery protection
- **Rate Limiting**: Configurable request rate limiting
- **Secure Sessions**: HTTP-only, secure cookies with configurable SameSite policy
- **Password Policy**: Configurable password complexity requirements
- **Session Refresh**: Automatic session token refresh

### User Interface
- **Login Page**: Modern, responsive login interface
- **User Menu**: Display current user info and logout functionality
- **Protected Routes**: Automatic redirection for unauthenticated users
- **Role Display**: Show user roles and permissions in the UI

## Configuration

### Basic Configuration

Add the `auth` section to your `config.yaml`:

```yaml
auth:
  enabled: true
  session:
    secret: "your-session-secret-key-here-change-this-in-production"
    duration: "24h"
    refresh_threshold: "1h"
    secure: true  # Set to true in production with HTTPS
    same_site: "strict"
  
  admin:
    username: "admin"
    password: "secure-password"
    email: "admin@example.com"
```

### Session Configuration

```yaml
auth:
  session:
    secret: "your-32-character-secret-key"  # Required: Session signing key
    duration: "24h"                         # Session lifetime
    refresh_threshold: "1h"                 # Auto-refresh threshold
    secure: true                            # HTTPS-only cookies
    same_site: "strict"                     # SameSite policy: strict, lax, none
    domain: "example.com"                   # Optional: Cookie domain
    path: "/"                               # Optional: Cookie path
```

### User Providers

#### File-based Provider

```yaml
auth:
  providers:
    file:
      enabled: true
      path: "./users.yaml"
      watch: true  # Auto-reload on file changes
```

Create a `users.yaml` file:

```yaml
users:
  - username: "admin"
    password: "$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/1YsQxSWW6"
    email: "admin@example.com"
    roles: ["admin", "user"]
    auth_method: "Traditional"
```

#### Database Provider

```yaml
auth:
  providers:
    database:
      enabled: true
      url: "postgresql://user:password@localhost/agentgateway"
      table: "users"
      connection_pool_size: 10
```

### OAuth2 Configuration

```yaml
auth:
  oauth2:
    providers:
      google:
        enabled: true
        client_id: "your-google-client-id"
        client_secret: "your-google-client-secret"
        scopes: ["openid", "profile", "email"]
        redirect_uri: "https://your-domain.com/auth/oauth2/google/callback"
      
      github:
        enabled: true
        client_id: "your-github-client-id"
        client_secret: "your-github-client-secret"
        scopes: ["user:email"]
        redirect_uri: "https://your-domain.com/auth/oauth2/github/callback"
```

### Security Settings

```yaml
auth:
  security:
    csrf_protection: true
    rate_limiting:
      enabled: true
      max_requests: 100
      window_seconds: 60
    password_policy:
      min_length: 8
      require_uppercase: true
      require_lowercase: true
      require_numbers: true
      require_special: false
```

## API Endpoints

### Authentication Endpoints

- `POST /auth/login` - User login
- `POST /auth/logout` - User logout
- `GET /auth/user` - Get current user info
- `GET /auth/oauth2/providers` - List available OAuth2 providers
- `GET /auth/oauth2/{provider}` - Initiate OAuth2 login
- `GET /auth/oauth2/{provider}/callback` - OAuth2 callback

### Request/Response Examples

#### Login Request
```json
POST /auth/login
{
  "username": "admin",
  "password": "password123"
}
```

#### Login Response
```json
{
  "success": true,
  "message": "Login successful",
  "csrf_token": "abc123..."
}
```

#### User Info Response
```json
GET /auth/user
{
  "username": "admin",
  "email": "admin@example.com",
  "roles": ["admin", "user"],
  "auth_method": "Traditional",
  "csrf_token": "abc123..."
}
```

## User Management

### Password Hashing

Passwords are hashed using bcrypt with a cost factor of 12. You can generate hashed passwords using:

```bash
# Using bcrypt CLI tool
bcrypt-cli hash "your-password"

# Using Python
python3 -c "import bcrypt; print(bcrypt.hashpw(b'your-password', bcrypt.gensalt()).decode())"

# Using Node.js
node -e "const bcrypt = require('bcrypt'); console.log(bcrypt.hashSync('your-password', 12));"
```

### Role-Based Access Control

Define roles in your user configuration:

```yaml
users:
  - username: "admin"
    roles: ["admin", "user"]  # Multiple roles
  - username: "developer"
    roles: ["developer", "user"]
  - username: "readonly"
    roles: ["readonly"]
```

### Database Schema

If using the database provider, create the users table:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    roles TEXT[] DEFAULT '{}',
    auth_method VARCHAR(50) DEFAULT 'Traditional',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Frontend Integration

### Protected Routes

The UI automatically protects routes that require authentication:

```tsx
import { ProtectedRoute } from '@/components/auth/protected-route';

function App() {
  return (
    <AuthProvider>
      <ProtectedRoute requiredRoles={['admin']}>
        <AdminPanel />
      </ProtectedRoute>
    </AuthProvider>
  );
}
```

### User Context

Access user information in components:

```tsx
import { useAuth } from '@/lib/auth-context';

function UserProfile() {
  const { user, isAuthenticated, logout } = useAuth();
  
  if (!isAuthenticated) {
    return <LoginForm />;
  }
  
  return (
    <div>
      <h1>Welcome, {user.username}!</h1>
      <p>Roles: {user.roles.join(', ')}</p>
      <button onClick={logout}>Logout</button>
    </div>
  );
}
```

## OAuth2 Setup

### Google OAuth2

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select existing
3. Enable Google+ API
4. Create OAuth2 credentials
5. Add authorized redirect URIs:
   - `https://your-domain.com/auth/oauth2/google/callback`

### GitHub OAuth2

1. Go to GitHub Settings > Developer settings > OAuth Apps
2. Create a new OAuth App
3. Set Authorization callback URL:
   - `https://your-domain.com/auth/oauth2/github/callback`

## Security Best Practices

### Production Deployment

1. **Use HTTPS**: Always enable HTTPS in production
2. **Strong Session Secret**: Use a cryptographically secure 32+ character secret
3. **Secure Cookies**: Set `secure: true` and `same_site: "strict"`
4. **Rate Limiting**: Enable and configure appropriate limits
5. **Password Policy**: Enforce strong password requirements

### Environment Variables

Store sensitive configuration in environment variables:

```yaml
auth:
  session:
    secret: "${AUTH_SESSION_SECRET}"
  oauth2:
    providers:
      google:
        client_secret: "${GOOGLE_CLIENT_SECRET}"
      github:
        client_secret: "${GITHUB_CLIENT_SECRET}"
```

### Regular Security Updates

- Regularly update dependencies
- Monitor for security vulnerabilities
- Rotate session secrets periodically
- Review user access and roles regularly

## Troubleshooting

### Common Issues

1. **Authentication fails**: Check password hashing and user configuration
2. **Session not persisting**: Verify cookie settings and HTTPS configuration
3. **OAuth2 errors**: Check redirect URIs and client credentials
4. **CSRF errors**: Ensure CSRF tokens are included in requests

### Debug Logging

Enable debug logging in your configuration:

```yaml
logging:
  level: "debug"
  auth: true
```

### Health Checks

Monitor authentication health:

```bash
curl -i http://localhost:3000/auth/health
```

## Migration Guide

### From No Authentication

1. Add `auth` section to your configuration
2. Create initial admin user
3. Configure user providers
4. Test authentication flow
5. Deploy with authentication enabled

### Database Migration

If migrating from file-based to database users:

1. Create users table
2. Export users from YAML
3. Import users to database
4. Update configuration
5. Test and verify

## API Reference

### Authentication Middleware

The authentication middleware automatically:
- Validates session tokens
- Extracts user information
- Enforces route protection
- Handles CSRF validation
- Applies rate limiting

### Custom Authentication

For custom authentication logic, extend the user providers:

```rust
use agentgateway::auth::{UserProvider, User};

struct CustomUserProvider {
    // Custom implementation
}

impl UserProvider for CustomUserProvider {
    async fn authenticate(&self, username: &str, password: &str) -> Result<User, Error> {
        // Custom authentication logic
    }
}
```

## Support

For issues and questions:
- Check the troubleshooting section
- Review configuration examples
- Enable debug logging
- Check server logs for errors
