import React, { useState } from 'react';
import {
  Box,
  AppBar,
  Toolbar,
  Typography,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  IconButton,
  Avatar,
  Menu,
  MenuItem,
} from '@mui/material';
import {
  Menu as MenuIcon,
  Lock as LockIcon,
  Note as NoteIcon,
  Key as KeyIcon,
  Logout as LogoutIcon,
  Category as CategoryIcon,
  Settings as SettingsIcon,
} from '@mui/icons-material';
import { useAuth } from '../context/AuthContext';
import { useNavigate } from 'react-router-dom';
import SecretsView from './SecretsView';
import NotesView from './NotesView';
import GeneratorView from './GeneratorView';
import CategoriesView from './CategoriesView';
import SettingsView from './SettingsView';

const drawerWidth = 240;

function Dashboard() {
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [activeView, setActiveView] = useState('secrets');
  const [anchorEl, setAnchorEl] = useState(null);
  const { user, logout } = useAuth();
  const navigate = useNavigate();

  const menuItems = [
    { id: 'secrets', label: 'Secrets', icon: <LockIcon /> },
    { id: 'notes', label: 'Notes', icon: <NoteIcon /> },
    { id: 'generator', label: 'Generator', icon: <KeyIcon /> },
    { id: 'categories', label: 'Categories', icon: <CategoryIcon /> },
    { id: 'settings', label: 'Settings', icon: <SettingsIcon /> },
  ];

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  const renderView = () => {
    switch (activeView) {
      case 'secrets':
        return <SecretsView />;
      case 'notes':
        return <NotesView />;
      case 'generator':
        return <GeneratorView />;
      case 'categories':
        return <CategoriesView />;
      case 'settings':
        return <SettingsView />;
      default:
        return <SecretsView />;
    }
  };

  return (
    <Box sx={{ display: 'flex' }}>
      <AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
        <Toolbar>
          <IconButton
            color="inherit"
            edge="start"
            onClick={() => setDrawerOpen(!drawerOpen)}
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton>
          <LockIcon sx={{ mr: 1 }} />
          <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1 }}>
            Vaultkey
          </Typography>

          <IconButton
            color="inherit"
            onClick={(e) => setAnchorEl(e.currentTarget)}
          >
            <Avatar sx={{ width: 32, height: 32, bgcolor: 'primary.main' }}>
              {user?.[0]?.toUpperCase() || 'U'}
            </Avatar>
          </IconButton>

          <Menu
            anchorEl={anchorEl}
            open={Boolean(anchorEl)}
            onClose={() => setAnchorEl(null)}
          >
            <MenuItem disabled>
              <Typography variant="body2">Signed in as {user}</Typography>
            </MenuItem>
            <MenuItem onClick={handleLogout}>
              <ListItemIcon>
                <LogoutIcon fontSize="small" />
              </ListItemIcon>
              Logout
            </MenuItem>
          </Menu>
        </Toolbar>
      </AppBar>

      <Drawer
        variant="permanent"
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          [`& .MuiDrawer-paper`]: {
            width: drawerWidth,
            boxSizing: 'border-box',
            bgcolor: 'background.paper',
          },
        }}
      >
        <Toolbar />
        <Box sx={{ overflow: 'auto', mt: 2 }}>
          <List>
            {menuItems.map((item) => (
              <ListItem key={item.id} disablePadding sx={{ mb: 1, px: 1 }}>
                <ListItemButton
                  onClick={() => setActiveView(item.id)}
                  sx={{
                    borderRadius: 2,
                    bgcolor: activeView === item.id ? 'primary.main' : 'transparent',
                    '&:hover': {
                      bgcolor: activeView === item.id ? 'primary.dark' : 'action.hover',
                    },
                  }}
                >
                  <ListItemIcon sx={{ color: activeView === item.id ? 'white' : 'inherit' }}>
                    {item.icon}
                  </ListItemIcon>
                  <ListItemText
                    primary={item.label}
                    sx={{ color: activeView === item.id ? 'white' : 'inherit' }}
                  />
                </ListItemButton>
              </ListItem>
            ))}
          </List>
        </Box>
      </Drawer>

      <Box
        component="main"
        sx={{
          flexGrow: 1,
          bgcolor: 'background.default',
          p: 3,
          minHeight: '100vh',
        }}
      >
        <Toolbar />
        {renderView()}
      </Box>
    </Box>
  );
}

export default Dashboard;