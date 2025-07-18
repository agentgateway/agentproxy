import React, { ReactNode } from 'react';
import { useAuth } from '../../lib/auth-context';

interface ProtectedRouteProps {
  children: ReactNode;
  requiredRoles?: string[];
  fallback?: ReactNode;
}

export function ProtectedRoute({ children, requiredRoles = [], fallback }: ProtectedRouteProps) {
  const { isAuthenticated, isLoading, hasAnyRole } = useAuth();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  if (!isAuthenticated) {
    if (fallback) {
      return <>{fallback}</>;
    }
    
    // Redirect to login
    window.location.href = '/auth/login';
    return null;
  }

  if (requiredRoles.length > 0 && !hasAnyRole(requiredRoles)) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <h2 className="text-2xl font-bold text-gray-900 mb-4">Access Denied</h2>
          <p className="text-gray-600 mb-4">You don't have permission to access this resource.</p>
          <p className="text-sm text-gray-500">
            Required roles: {requiredRoles.join(', ')}
          </p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
}

interface AdminRouteProps {
  children: ReactNode;
  fallback?: ReactNode;
}

export function AdminRoute({ children, fallback }: AdminRouteProps) {
  return (
    <ProtectedRoute requiredRoles={['admin']} fallback={fallback}>
      {children}
    </ProtectedRoute>
  );
}
