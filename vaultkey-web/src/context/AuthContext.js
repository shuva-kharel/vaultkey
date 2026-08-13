import React, { createContext, useState, useContext } from 'react';

const AuthContext = createContext();

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(localStorage.getItem('vaultkey_token'));

  const login = (username, token) => {
    setUser(username);
    setToken(token);
    localStorage.setItem('vaultkey_token', token);
    localStorage.setItem('vaultkey_user', username);
  };

  const logout = () => {
    setUser(null);
    setToken(null);
    localStorage.removeItem('vaultkey_token');
    localStorage.removeItem('vaultkey_user');
  };

  return (
    <AuthContext.Provider value={{ user, token, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};