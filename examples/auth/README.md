# Authentication Example

This example demonstrates how to configure and use the authentication system in agentgateway.

## Quick Start

1. **Start the gateway with authentication enabled**:
   ```bash
   cargo run --release -- --config examples/auth/config.yaml
   ```

2. **Access the web UI**:
   Open your browser to `http://localhost:3000`

3. **Login with sample users**:
   - **Admin**: username: `admin`, password: `admin123`
   - **User**: username: `user`, password: `user123`
   - **Developer**: username: `developer`, password: `dev123`
   - **Readonly**: username: `readonly`, password: `readonly123`

## Configuration Files

### config.yaml
The main configuration file that includes:
- Authentication settings
- Session configuration
- User providers
- OAuth2 setup (optional)
- Security settings

### users.yaml
Sample user database with:
- Pre-hashed passwords
- Role assignments
- User metadata

## Features Demonstrated

### User Authentication
- Traditional username/password login
- Secure session management
- Automatic session refresh

### Role-Based Access Control
- Different user roles: admin, user, developer, readonly
- Role-based UI elements
- Permission checking

### Security Features
- CSRF protection
- Rate limiting
- Secure cookie handling
- Password policy enforcement

### User Interface
- Modern login page
- User menu with role display
- Protected routes
- Automatic authentication state management

## Testing Authentication

### Manual Testing
1. Try logging in with different users
2. Check role-based access
3. Test session persistence
4. Verify logout functionality

### API Testing
Use curl to test the authentication API:

```bash
# Login
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}' \
  -c cookies.txt

# Get user info
curl -X GET http://localhost:3000/auth/user \
  -b cookies.txt

# Logout
curl -X POST http://localhost:3000/auth/logout \
  -b cookies.txt
```

## Customization

### Adding New Users
Edit `users.yaml` and add new users:

```yaml
users:
  - username: "newuser"
    password: "$2b$12$your-hashed-password"
    email: "newuser@example.com"
    roles: ["user"]
    auth_method: "Traditional"
```

### Changing Passwords
Generate new password hashes:

```bash
# Using Python
python3 -c "import bcrypt; print(bcrypt.hashpw(b'newpassword', bcrypt.gensalt()).decode())"
```

### Configuring OAuth2
Uncomment and configure OAuth2 providers in `config.yaml`:

```yaml
auth:
  oauth2:
    providers:
      google:
        enabled: true
        client_id: "your-client-id"
        client_secret: "your-client-secret"
```

## Security Notes

This example uses:
- **Demo passwords**: Change all passwords in production
- **HTTP cookies**: Use HTTPS in production
- **Simple session secret**: Use a secure random key in production

## Production Checklist

Before deploying to production:

- [ ] Change all default passwords
- [ ] Use HTTPS for all connections
- [ ] Generate a secure session secret
- [ ] Configure proper CORS origins
- [ ] Enable rate limiting
- [ ] Set up proper logging
- [ ] Configure OAuth2 providers
- [ ] Test all authentication flows
- [ ] Implement proper backup procedures
- [ ] Monitor authentication metrics

## Architecture

The authentication system consists of:

### Backend Components
- **Auth Module**: Core authentication logic
- **User Providers**: File-based and database user storage
- **Session Management**: JWT-based sessions with refresh
- **OAuth2 Integration**: External provider support
- **Middleware**: Route protection and CSRF validation

### Frontend Components
- **AuthProvider**: React context for authentication state
- **ProtectedRoute**: Route protection wrapper
- **LoginForm**: User login interface
- **UserMenu**: User info and logout functionality

## Troubleshooting

### Common Issues

1. **Login fails**: Check username/password and user configuration
2. **Session not persisting**: Verify cookie settings
3. **UI not loading**: Ensure UI is built and path is correct
4. **CORS errors**: Check allowed origins in configuration

### Debug Mode

Enable debug logging in `config.yaml`:

```yaml
logging:
  level: "debug"
```

This will provide detailed information about authentication flows and errors.
