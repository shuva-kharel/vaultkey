import React, { useState } from 'react';
import {
  Box,
  Typography,
  Paper,
  TextField,
  Button,
  Alert,
} from '@mui/material';

function SettingsView() {
  const [clipboardTimeout, setClipboardTimeout] = useState(30);
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    localStorage.setItem('vaultkey_clipboard_timeout', clipboardTimeout);
    setSaved(true);
    setTimeout(() => setSaved(false), 8080);
  };

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        Settings
      </Typography>

      <Paper sx={{ p: 3, maxWidth: 600 }}>
        <Typography variant="h6" gutterBottom>
          Clipboard
        </Typography>
        <TextField
          fullWidth
          label="Clipboard auto-clear timeout (seconds)"
          type="number"
          value={clipboardTimeout}
          onChange={(e) => setClipboardTimeout(parseInt(e.target.value) || 30)}
          margin="normal"
        />
        <Button variant="contained" onClick={handleSave} sx={{ mt: 2 }}>
          Save Settings
        </Button>

        {saved && (
          <Alert severity="success" sx={{ mt: 2 }}>
            Settings saved!
          </Alert>
        )}
      </Paper>
    </Box>
  );
}

export default SettingsView;