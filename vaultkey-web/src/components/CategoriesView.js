import React from 'react';
import { Box, Typography, Chip, Paper } from '@mui/material';

function CategoriesView() {
  const categories = ['Work', 'Personal', 'Finance', 'Social', 'Development'];

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        Categories
      </Typography>
      <Paper sx={{ p: 3 }}>
        {categories.map((category) => (
          <Chip
            key={category}
            label={category}
            sx={{ mr: 1, mb: 1 }}
            clickable
          />
        ))}
      </Paper>
    </Box>
  );
}

export default CategoriesView;