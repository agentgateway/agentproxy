import React from 'react';
import { useAuth } from '../../lib/auth-context';
import { Button } from '../ui/button';
import { LogOut, User, Shield, Settings } from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '../ui/dropdown-menu';
import { Badge } from '../ui/badge';

export function UserMenu() {
  const { user, logout, isAdmin } = useAuth();

  if (!user) {
    return null;
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="flex items-center gap-2">
          <User className="h-4 w-4" />
          <span className="hidden sm:inline">{user.username}</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end" className="w-56">
        <DropdownMenuLabel>
          <div className="flex flex-col space-y-1">
            <p className="text-sm font-medium leading-none">{user.username}</p>
            {user.email && (
              <p className="text-xs leading-none text-muted-foreground">
                {user.email}
              </p>
            )}
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        
        {/* User roles */}
        <div className="px-2 py-1.5">
          <p className="text-xs text-muted-foreground mb-1">Roles</p>
          <div className="flex flex-wrap gap-1">
            {user.roles.map((role) => (
              <Badge key={role} variant="secondary" className="text-xs">
                {role}
              </Badge>
            ))}
          </div>
        </div>
        
        <DropdownMenuSeparator />
        
        {/* Admin settings */}
        {isAdmin() && (
          <DropdownMenuItem>
            <Settings className="mr-2 h-4 w-4" />
            <span>Admin Settings</span>
          </DropdownMenuItem>
        )}
        
        <DropdownMenuItem onClick={logout}>
          <LogOut className="mr-2 h-4 w-4" />
          <span>Log out</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

interface AuthStatusProps {
  showDetails?: boolean;
}

export function AuthStatus({ showDetails = false }: AuthStatusProps) {
  const { user, isAuthenticated, isAdmin } = useAuth();

  if (!isAuthenticated || !user) {
    return (
      <div className="flex items-center gap-2 text-muted-foreground">
        <Shield className="h-4 w-4" />
        <span>Not authenticated</span>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-2">
      <div className="flex items-center gap-2">
        <Shield className="h-4 w-4 text-green-500" />
        <span className="text-sm font-medium">{user.username}</span>
        {isAdmin() && (
          <Badge variant="default" className="text-xs">
            Admin
          </Badge>
        )}
      </div>
      
      {showDetails && (
        <div className="text-xs text-muted-foreground">
          {typeof user.authMethod === 'string' ? 
            user.authMethod : 
            `OAuth2 (${user.authMethod.OAuth2.provider})`
          }
        </div>
      )}
    </div>
  );
}
