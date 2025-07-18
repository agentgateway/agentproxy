import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export interface User {
  username: string;
  email?: string;
  roles: string[];
  authMethod: 'Traditional' | { OAuth2: { provider: string } };
}

export interface AuthContextType {
  user: User | null;
  login: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
  isAuthenticated: boolean;
  isLoading: boolean;
  csrfToken: string | null;
  hasRole: (role: string) => boolean;
  hasAnyRole: (roles: string[]) => boolean;
  isAdmin: () => boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}

interface AuthProviderProps {
  children: ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [csrfToken, setCsrfToken] = useState<string | null>(null);

  // Check if user is authenticated on mount
  useEffect(() => {
    checkAuthStatus();
  }, []);

  const checkAuthStatus = async () => {
    try {
      const response = await fetch('/auth/user', {
        credentials: 'include',
      });
      
      if (response.ok) {
        const userData = await response.json();
        setUser({
          username: userData.username,
          email: userData.email,
          roles: userData.roles,
          authMethod: userData.auth_method,
        });
        setCsrfToken(userData.csrf_token);
      } else {
        setUser(null);
        setCsrfToken(null);
      }
    } catch (error) {
      console.error('Auth check failed:', error);
      setUser(null);
      setCsrfToken(null);
    } finally {
      setIsLoading(false);
    }
  };

  const login = async (username: string, password: string) => {
    setIsLoading(true);
    try {
      const response = await fetch('/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify({ username, password }),
      });

      const result = await response.json();
      
      if (result.success) {
        await checkAuthStatus(); // Refresh user data
        if (result.redirect_url) {
          window.location.href = result.redirect_url;
        }
      } else {
        throw new Error(result.message || 'Login failed');
      }
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async () => {
    setIsLoading(true);
    try {
      await fetch('/auth/logout', {
        method: 'POST',
        credentials: 'include',
      });
      setUser(null);
      setCsrfToken(null);
      window.location.href = '/auth/login';
    } catch (error) {
      console.error('Logout failed:', error);
      // Force logout on client side even if server request fails
      setUser(null);
      setCsrfToken(null);
      window.location.href = '/auth/login';
    } finally {
      setIsLoading(false);
    }
  };

  const hasRole = (role: string): boolean => {
    return user?.roles.includes(role) || false;
  };

  const hasAnyRole = (roles: string[]): boolean => {
    return roles.some(role => hasRole(role));
  };

  const isAdmin = (): boolean => {
    return hasRole('admin');
  };

  const value: AuthContextType = {
    user,
    login,
    logout,
    isAuthenticated: !!user,
    isLoading,
    csrfToken,
    hasRole,
    hasAnyRole,
    isAdmin,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}
