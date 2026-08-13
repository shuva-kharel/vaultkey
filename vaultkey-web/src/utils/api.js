import axios from 'axios';

const API_BASE = process.env.REACT_APP_API_URL || 'http://localhost:8000';

const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('vaultkey_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const authAPI = {
  registerStart: (username) => api.post('/register/start', { username }),
  registerFinish: (username, userId, registration) =>
    api.post('/register/finish', { username, user_id: userId, registration }),
  loginStart: (username) => api.post('/login/start', { username }),
  loginFinish: (username, authentication) =>
    api.post('/login/finish', { username, authentication }),
};

export const vaultAPI = {
  getSecrets: () => api.get('/vault/secrets'),
  addSecret: (secret) => api.post('/vault/secrets', secret),
  updateSecret: (name, secret) => api.put(`/vault/secrets/${name}`, secret),
  deleteSecret: (name) => api.delete(`/vault/secrets/${name}`),
  getNotes: () => api.get('/vault/notes'),
  addNote: (note) => api.post('/vault/notes', note),
  deleteNote: (title) => api.delete(`/vault/notes/${title}`),
  uploadVault: (data) => api.put('/vault', data),
  downloadVault: () => api.get('/vault'),
};

export default api;