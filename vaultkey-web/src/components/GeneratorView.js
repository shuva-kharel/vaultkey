import React, { useState } from 'react';
import {
  Box,
  Typography,
  Slider,
  FormControlLabel,
  Checkbox,
  Button,
  Paper,
  IconButton,
  TextField,
} from '@mui/material';
import {
  ContentCopy as CopyIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material';

function GeneratorView() {
  const [length, setLength] = useState(20);
  const [useUpper, setUseUpper] = useState(true);
  const [useLower, setUseLower] = useState(true);
  const [useNumbers, setUseNumbers] = useState(true);
  const [useSymbols, setUseSymbols] = useState(true);
  const [password, setPassword] = useState('');

  const generatePassword = () => {
    let charset = '';
    if (useUpper) charset += 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
    if (useLower) charset += 'abcdefghijklmnopqrstuvwxyz';
    if (useNumbers) charset += '0123456789';
    if (useSymbols) charset += '!@#$%^&*()_+-=[]{}|;:,.<>?';

    if (!charset) return;

    let result = '';
    const array = new Uint32Array(length);
    crypto.getRandomValues(array);

    for (let i = 0; i < length; i++) {
      result += charset[array[i] % charset.length];
    }

    setPassword(result);
  };

  const copyToClipboard = async () => {
    if (password) {
      await navigator.clipboard.writeText(password);
    }
  };

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        Password Generator
      </Typography>

      <Paper sx={{ p: 3, maxWidth: 600 }}>
        <Box sx={{ mb: 3 }}>
          <Typography gutterBottom>
            Password Length: {length}
          </Typography>
          <Slider
            value={length}
            onChange={(e, val) => setLength(val)}
            min={8}
            max={64}
            valueLabelDisplay="auto"
          />
        </Box>

        <Box sx={{ mb: 3 }}>
          <FormControlLabel
            control={<Checkbox checked={useUpper} onChange={(e) => setUseUpper(e.target.checked)} />}
            label="Uppercase (A-Z)"
          />
          <FormControlLabel
            control={<Checkbox checked={useLower} onChange={(e) => setUseLower(e.target.checked)} />}
            label="Lowercase (a-z)"
          />
          <FormControlLabel
            control={<Checkbox checked={useNumbers} onChange={(e) => setUseNumbers(e.target.checked)} />}
            label="Numbers (0-9)"
          />
          <FormControlLabel
            control={<Checkbox checked={useSymbols} onChange={(e) => setUseSymbols(e.target.checked)} />}
            label="Symbols (!@#$...)"
          />
        </Box>

        <Button
          variant="contained"
          startIcon={<RefreshIcon />}
          onClick={generatePassword}
          fullWidth
          sx={{ mb: 2 }}
        >
          Generate Password
        </Button>

        {password && (
          <Box sx={{ display: 'flex', gap: 1, alignItems: 'center' }}>
            <TextField
              fullWidth
              value={password}
              InputProps={{
                readOnly: true,
              }}
            />
            <IconButton onClick={copyToClipboard}>
              <CopyIcon />
            </IconButton>
          </Box>
        )}
      </Paper>
    </Box>
  );
}

export default GeneratorView;