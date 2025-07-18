"use client";

import React, { useEffect, useState } from "react";
import { LoginForm } from "@/components/auth/login-form";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { useAuth } from "@/lib/auth-context";
import { Loader2, Shield } from "lucide-react";

interface OAuth2Provider {
  name: string;
  display_name: string;
  auth_url: string;
}

export default function LoginPage() {
  const [providers, setProviders] = useState<OAuth2Provider[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const { isAuthenticated, isLoading } = useAuth();

  useEffect(() => {
    const fetchProviders = async () => {
      try {
        const response = await fetch('/auth/oauth2/providers');
        if (response.ok) {
          const data = await response.json();
          setProviders(data.providers || []);
        }
      } catch (err) {
        console.error('Failed to fetch OAuth2 providers:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchProviders();
  }, []);

  const handleLoginSuccess = () => {
    window.location.href = '/';
  };

  // Show loading spinner while checking auth status
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin" />
      </div>
    );
  }

  // Redirect if already authenticated
  if (isAuthenticated) {
    window.location.href = '/';
    return null;
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div className="text-center">
          <Shield className="mx-auto h-12 w-12 text-blue-600" />
          <h2 className="mt-6 text-3xl font-extrabold text-gray-900">
            Welcome to Agentgateway
          </h2>
          <p className="mt-2 text-sm text-gray-600">
            Sign in to access the dashboard
          </p>
        </div>

        {error && (
          <Alert variant="destructive">
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        )}

        <div className="space-y-6">
          <LoginForm onSuccess={handleLoginSuccess} />
          
          {loading ? (
            <div className="flex justify-center">
              <Loader2 className="h-4 w-4 animate-spin" />
            </div>
          ) : (
            <OAuth2Providers providers={providers} />
          )}
        </div>

        <div className="text-center">
          <p className="text-xs text-gray-500">
            By signing in, you agree to our terms of service and privacy policy.
          </p>
        </div>
      </div>
    </div>
  );
}
