import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Card,
  CardContent,
  CardActions,
  IconButton,
  InputAdornment,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Grid,
  Chip,
  Tooltip,
  Alert,
  Snackbar,
} from '@mui/material';
import {
  Add as AddIcon,
  Search as SearchIcon,
  ContentCopy as CopyIcon,
  Visibility as VisibilityIcon,
  Delete as DeleteIcon,
  Edit as EditIcon,
} from '@mui/icons-material';
import { vaultAPI } from '../utils/api';

function SecretsView() {
  const [secrets, setSecrets] = useState([]);
  const [search, setSearch] = useState('');
  const [openAdd, setOpenAdd] = useState(false);
  const [snackbar, setSnackbar] = useState({ open: false, message: '' });
  const [newSecret, setNewSecret] = useState({
    name: '',
    username: '',
    password: '',
    url: '',
    category: '',
    notes: '',
  });

  useEffect(() => {
    loadSecrets();
  }, []);

  const loadSecrets = async () => {
    try {
      const response = await vaultAPI.getSecrets();
      setSecrets(response.data || []);
    } catch (error) {
      console.error('Failed to load secrets:', error);
    }
  };

  const handleAddSecret = async () => {
    try {
      await vaultAPI.addSecret(newSecret);
      setOpenAdd(false);
      setNewSecret({
        name: '',
        username: '',
        password: '',
        url: '',
        category: '',
        notes: '',
      });
      loadSecrets();
      setSnackbar({ open: true, message: 'Secret added successfully!' });
    } catch (error) {
      setSnackbar({ open: true, message: 'Failed to add secret' });
    }
  };

  const handleCopy = async (secret) => {
    try {
      await navigator.clipboard.writeText(secret.password);
      setSnackbar({ open: true, message: 'Password copied to clipboard!' });

      setTimeout(async () => {
        await navigator.clipboard.writeText('');
      }, 80800);
    } catch (error) {
      setSnackbar({ open: true, message: 'Failed to copy' });
    }
  };

  const handleDelete = async (name) => {
    if (window.confirm(`Delete secret "${name}"?`)) {
      try {
        await vaultAPI.deleteSecret(name);
        loadSecrets();
        setSnackbar({ open: true, message: 'Secret deleted' });
      } catch (error) {
        setSnackbar({ open: true, message: 'Failed to delete' });
      }
    }
  };

  const filteredSecrets = secrets.filter(secret =>
    secret.name.toLowerCase().includes(search.toLowerCase()) ||
    secret.username.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
        <Typography variant="h5">Secrets</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setOpenAdd(true)}
        >
          Add Secret
        </Button>
      </Box>

      <TextField
        fullWidth
        placeholder="Search secrets..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        sx={{ mb: 3 }}
        slotProps={{
          input: {
            startAdornment: (
              <InputAdornment position="start">
                <SearchIcon />
              </InputAdornment>
            ),
          },
        }}
      />

      <Grid container spacing={2}>
        {filteredSecrets.map((secret) => (
          <Grid item xs={12} sm={6} md={4} key={secret.name}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {secret.name}
                </Typography>
                <Typography variant="body2" color="textSecondary">
                  {secret.username}
                </Typography>
                {secret.url && (
                  <Typography variant="body2" color="textSecondary">
                    {secret.url}
                  </Typography>
                )}
                {secret.category && (
                  <Chip
                    label={secret.category}
                    size="small"
                    sx={{ mt: 1 }}
                  />
                )}
              </CardContent>
              <CardActions>
                <Tooltip title="Copy password">
                  <IconButton onClick={() => handleCopy(secret)}>
                    <CopyIcon />
                  </IconButton>
                </Tooltip>
                <Tooltip title="View details">
                  <IconButton>
                    <VisibilityIcon />
                  </IconButton>
                </Tooltip>
                <Tooltip title="Delete">
                  <IconButton onClick={() => handleDelete(secret.name)}>
                    <DeleteIcon />
                  </IconButton>
                </Tooltip>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Dialog open={openAdd} onClose={() => setOpenAdd(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add New Secret</DialogTitle>
        <DialogContent>
          <TextField
            fullWidth
            label="Name"
            value={newSecret.name}
            onChange={(e) => setNewSecret({ ...newSecret, name: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Username"
            value={newSecret.username}
            onChange={(e) => setNewSecret({ ...newSecret, username: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Password"
            type="password"
            value={newSecret.password}
            onChange={(e) => setNewSecret({ ...newSecret, password: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="URL"
            value={newSecret.url}
            onChange={(e) => setNewSecret({ ...newSecret, url: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Category"
            value={newSecret.category}
            onChange={(e) => setNewSecret({ ...newSecret, category: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Notes"
            multiline
            rows={3}
            value={newSecret.notes}
            onChange={(e) => setNewSecret({ ...newSecret, notes: e.target.value })}
            margin="normal"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenAdd(false)}>Cancel</Button>
          <Button onClick={handleAddSecret} variant="contained">Save</Button>
        </DialogActions>
      </Dialog>

      <Snackbar
        open={snackbar.open}
        autoHideDuration={8080}
        onClose={() => setSnackbar({ ...snackbar, open: false })}
        message={snackbar.message}
      />
    </Box>
  );
}

export default SecretsView;