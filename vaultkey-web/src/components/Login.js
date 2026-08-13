import React, { useState } from 'react';
import {
  Container,
  Paper,
  TextField,
  Button,
  Typography,
  Box,
  Alert,
  Link,
} from '@mui/material';
import LockIcon from '@mui/icons-material/Lock';
import { useAuth } from '../context/AuthContext';
import { authAPI } from '../utils/api';
import WebAuthnHelper from '../utils/webauthn';
import { useNavigate } from 'react-router-dom';

function Login() {
  const [username, setUsername] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [isRegister, setIsRegister] = useState(false);
  const { login } = useAuth();
  const navigate = useNavigate();

  const handleAuth = async () => {
    if (!username) {
      setError('Please enter a username');
      return;
    }

    setLoading(true);
    setError('');

    try {
      if (isRegister) {
        // Register flow
        const startResponse = await authAPI.registerStart(username);
        const { challenge, user_id } = startResponse.data;

        const credential = await WebAuthnHelper.register(
          username,
          challenge.publicKey.challenge,
          user_id
        );

        await authAPI.registerFinish(username, user_id, credential);
        setError('');
        setIsRegister(false);
        alert('Registration successful! Please login now.');
      } else {
        // Login flow
        const startResponse = await authAPI.loginStart(username);
        const challenge = startResponse.data.publicKey.challenge;

        const assertion = await WebAuthnHelper.authenticate(challenge);

        const loginResponse = await authAPI.loginFinish(username, assertion);
        login(username, loginResponse.data.token);
        navigate('/dashboard');
      }
    } catch (err) {
      console.error('Auth error:', err);
      if (err.response?.status === 401) {
        setError('User not found. Please register first.');
      } else if (err.response?.status === 400) {
        setError(err.response.data || 'Authentication failed');
      } else {
        setError(err.message || 'Authentication failed');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container maxWidth="sm">
      <Box
        sx={{
          marginTop: 8,
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
        }}
      >
        <Paper
          elevation={3}
          sx={{
            padding: 4,
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
            width: '100%',
          }}
        >
          <LockIcon sx={{ fontSize: 48, color: 'primary.main', mb: 2 }} />
          <Typography variant="h4" gutterBottom>
            Vaultkey
          </Typography>
          <Typography variant="body1" color="textSecondary" gutterBottom>
            Hardware-backed password manager
          </Typography>

          <TextField
            fullWidth
            label="Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            margin="normal"
            variant="outlined"
          />

          {error && (
            <Alert severity="error" sx={{ mt: 2, width: '100%' }}>
              {error}
            </Alert>
          )}

          <Button
            fullWidth
            variant="contained"
            onClick={handleAuth}
            disabled={loading}
            sx={{ mt: 3, mb: 2 }}
          >
            {loading ? 'Please wait...' : isRegister ? 'Register with Passkey' : 'Login with Passkey'}
          </Button>

          <Link
            component="button"
            variant="body2"
            onClick={() => setIsRegister(!isRegister)}
          >
            {isRegister ? 'Already have an account? Login' : "Don't have an account? Register"}
          </Link>

          <Box sx={{ mt: 3, textAlign: 'left' }}>
            <Typography variant="caption" color="textSecondary">
              💡 Need a passkey? Use Windows Hello, ProtonPass, 1Password, or a security key.
            </Typography>
          </Box>
        </Paper>
      </Box>
    </Container>
  );
}

export default Login;