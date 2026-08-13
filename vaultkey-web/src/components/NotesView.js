import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Button,
  Card,
  CardContent,
  CardActions,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Grid,
  Chip,
  Snackbar,
} from '@mui/material';
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  Visibility as VisibilityIcon,
} from '@mui/icons-material';
import { vaultAPI } from '../utils/api';

function NotesView() {
  const [notes, setNotes] = useState([]);
  const [openAdd, setOpenAdd] = useState(false);
  const [snackbar, setSnackbar] = useState({ open: false, message: '' });
  const [newNote, setNewNote] = useState({
    title: '',
    content: '',
    category: '',
  });

  useEffect(() => {
    loadNotes();
  }, []);

  const loadNotes = async () => {
    try {
      const response = await vaultAPI.getNotes();
      setNotes(response.data || []);
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  };

  const handleAddNote = async () => {
    try {
      await vaultAPI.addNote(newNote);
      setOpenAdd(false);
      setNewNote({ title: '', content: '', category: '' });
      loadNotes();
      setSnackbar({ open: true, message: 'Note added successfully!' });
    } catch (error) {
      setSnackbar({ open: true, message: 'Failed to add note' });
    }
  };

  const handleDelete = async (title) => {
    if (window.confirm(`Delete note "${title}"?`)) {
      try {
        await vaultAPI.deleteNote(title);
        loadNotes();
        setSnackbar({ open: true, message: 'Note deleted' });
      } catch (error) {
        setSnackbar({ open: true, message: 'Failed to delete' });
      }
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 3 }}>
        <Typography variant="h5">Secure Notes</Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={() => setOpenAdd(true)}
        >
          Add Note
        </Button>
      </Box>

      <Grid container spacing={2}>
        {notes.map((note) => (
          <Grid item xs={12} sm={6} md={4} key={note.title}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  {note.title}
                </Typography>
                <Typography variant="body2" color="textSecondary" noWrap>
                  {note.content?.substring(0, 100)}
                </Typography>
                {note.category && (
                  <Chip
                    label={note.category}
                    size="small"
                    sx={{ mt: 1 }}
                  />
                )}
              </CardContent>
              <CardActions>
                <IconButton>
                  <VisibilityIcon />
                </IconButton>
                <IconButton onClick={() => handleDelete(note.title)}>
                  <DeleteIcon />
                </IconButton>
              </CardActions>
            </Card>
          </Grid>
        ))}
      </Grid>

      <Dialog open={openAdd} onClose={() => setOpenAdd(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Add Note</DialogTitle>
        <DialogContent>
          <TextField
            fullWidth
            label="Title"
            value={newNote.title}
            onChange={(e) => setNewNote({ ...newNote, title: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Content"
            multiline
            rows={6}
            value={newNote.content}
            onChange={(e) => setNewNote({ ...newNote, content: e.target.value })}
            margin="normal"
          />
          <TextField
            fullWidth
            label="Category"
            value={newNote.category}
            onChange={(e) => setNewNote({ ...newNote, category: e.target.value })}
            margin="normal"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpenAdd(false)}>Cancel</Button>
          <Button onClick={handleAddNote} variant="contained">Save</Button>
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

export default NotesView;