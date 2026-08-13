let authToken = null;
let currentUser = null;

// API functions
const API_BASE = window.location.origin;

async function apiCall(endpoint, method = 'GET', body = null, token = null) {
  const headers = {
    'Content-Type': 'application/json'
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const options = {
    method,
    headers
  };

  if (body) {
    options.body = JSON.stringify(body);
  }

  const response = await fetch(`${API_BASE}${endpoint}`, options);

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `HTTP ${response.status}`);
  }

  return response.json();
}

// Authentication functions
async function register() {
  const username = document.getElementById('username').value;
  if (!username) {
    showStatus('Please enter a username', 'error');
    return;
  }

  showStatus('Starting registration...', 'success');

  try {
    // Start registration
    const startResponse = await apiCall('/register/start', 'POST', { username });

    // Perform WebAuthn registration
    const credential = await WebAuthnHelper.register(
      username,
      startResponse.challenge.publicKey.challenge,
      startResponse.user_id
    );

    // Complete registration
    await apiCall('/register/finish', 'POST', {
      username,
      user_id: startResponse.user_id,
      registration: credential
    });

    showStatus('Registration successful! Please login.', 'success');
    document.getElementById('register-btn').style.display = 'none';
  } catch (error) {
    showStatus(`Registration failed: ${error.message}`, 'error');
  }
}

async function login() {
  const username = document.getElementById('username').value;
  if (!username) {
    showStatus('Please enter a username', 'error');
    return;
  }

  showStatus('Starting login...', 'success');

  try {
    const startResponse = await apiCall('/login/start', 'POST', { username });
    const challenge = startResponse.publicKey.challenge;

    showStatus('Please authenticate with your passkey...', 'success');

    const assertion = await WebAuthnHelper.authenticate(challenge);

    const loginResponse = await apiCall('/login/finish', 'POST', {
      username,
      authentication: assertion
    });

    authToken = loginResponse.token;
    currentUser = username;

    showMainScreen();
    showStatus('Login successful!', 'success');
  } catch (error) {
    console.error('Login error:', error);

    if (error.name === 'NotAllowedError') {
      showStatus('Authentication cancelled or no passkey available. Try using a passkey manager like ProtonPass, 1Password, or Windows Hello.', 'error');
    } else if (error.message.includes('domain')) {
      showStatus('Please use http://localhost:8080 (not 127.0.0.1)', 'error');
    } else if (error.message.includes('timeout')) {
      showStatus('Operation timed out. Try again.', 'error');
    } else {
      showStatus(`Login failed: ${error.message}`, 'error');
    }
  }
}

function logout() {
  authToken = null;
  currentUser = null;
  document.getElementById('login-screen').classList.remove('hidden');
  document.getElementById('main-screen').classList.add('hidden');
}

// UI functions
function showMainScreen() {
  document.getElementById('login-screen').classList.add('hidden');
  document.getElementById('main-screen').classList.remove('hidden');
  document.getElementById('current-user').textContent = currentUser;
  loadSecrets();
}

function showStatus(message, type) {
  const statusDiv = document.getElementById('status-message');
  statusDiv.textContent = message;
  statusDiv.className = type;
  setTimeout(() => {
    statusDiv.textContent = '';
    statusDiv.className = '';
  }, 5000);
}

function showSection(section) {
  document.querySelectorAll('.section').forEach(s => s.classList.add('hidden'));
  document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));

  document.getElementById(`${section}-section`).classList.remove('hidden');
  event.target.classList.add('active');

  if (section === 'secrets') loadSecrets();
  if (section === 'notes') loadNotes();
}

// Secrets functions
async function loadSecrets() {
  try {
    const secrets = await apiCall('/vault/secrets', 'GET', null, authToken);
    displaySecrets(secrets);
  } catch (error) {
    console.error('Failed to load secrets:', error);
  }
}

function displaySecrets(secrets) {
  const list = document.getElementById('secrets-list');
  list.innerHTML = '';

  secrets.forEach(secret => {
    const item = document.createElement('div');
    item.className = 'secret-item';
    item.innerHTML = `
            <div class="secret-info">
                <h3>${secret.name}</h3>
                <p>${secret.username}${secret.url ? ` • ${secret.url}` : ''}</p>
                ${secret.category ? `<p>Category: ${secret.category}</p>` : ''}
            </div>
            <div class="secret-actions">
                <button onclick="copySecret('${secret.name}')">Copy</button>
                <button onclick="viewSecret('${secret.name}')">View</button>
                <button onclick="deleteSecret('${secret.name}')">Delete</button>
            </div>
        `;
    list.appendChild(item);
  });
}

function showAddSecret() {
  document.getElementById('add-secret-modal').classList.remove('hidden');
}

function closeModal() {
  document.querySelectorAll('.modal').forEach(m => m.classList.add('hidden'));
}

async function saveSecret() {
  const secret = {
    name: document.getElementById('secret-name').value,
    username: document.getElementById('secret-username').value,
    password: document.getElementById('secret-password').value,
    url: document.getElementById('secret-url').value || null,
    category: document.getElementById('secret-category').value || null,
    notes: document.getElementById('secret-notes').value || null
  };

  try {
    await apiCall('/vault/secrets', 'POST', secret, authToken);
    closeModal();
    loadSecrets();
    showStatus('Secret saved!', 'success');
  } catch (error) {
    showStatus(`Failed to save: ${error.message}`, 'error');
  }
}

async function copySecret(name) {
  try {
    const secret = await apiCall(`/vault/secrets/${name}`, 'GET', null, authToken);
    await navigator.clipboard.writeText(secret.password);
    showStatus('Password copied to clipboard!', 'success');

    setTimeout(async () => {
      await navigator.clipboard.writeText('');
      showStatus('Clipboard cleared', 'success');
    }, 80800);
  } catch (error) {
    showStatus(`Failed to copy: ${error.message}`, 'error');
  }
}

async function viewSecret(name) {
  try {
    const secret = await apiCall(`/vault/secrets/${name}`, 'GET', null, authToken);
    alert(`Username: ${secret.username}\nPassword: ${secret.password}`);
  } catch (error) {
    showStatus(`Failed to view: ${error.message}`, 'error');
  }
}

async function deleteSecret(name) {
  if (!confirm(`Delete secret "${name}"?`)) return;

  try {
    await apiCall(`/vault/secrets/${name}`, 'DELETE', null, authToken);
    loadSecrets();
    showStatus('Secret deleted!', 'success');
  } catch (error) {
    showStatus(`Failed to delete: ${error.message}`, 'error');
  }
}

function searchSecrets() {
  const query = document.getElementById('search-input').value.toLowerCase();
  const items = document.querySelectorAll('.secret-item');

  items.forEach(item => {
    const text = item.textContent.toLowerCase();
    item.style.display = text.includes(query) ? 'flex' : 'none';
  });
}

// Notes functions
async function loadNotes() {
  try {
    const notes = await apiCall('/vault/notes', 'GET', null, authToken);
    displayNotes(notes);
  } catch (error) {
    console.error('Failed to load notes:', error);
  }
}

function displayNotes(notes) {
  const list = document.getElementById('notes-list');
  list.innerHTML = '';

  notes.forEach(note => {
    const item = document.createElement('div');
    item.className = 'note-item';
    item.innerHTML = `
            <div class="note-info">
                <h3>${note.title}</h3>
                ${note.category ? `<p>Category: ${note.category}</p>` : ''}
            </div>
            <div class="note-actions">
                <button onclick="viewNote('${note.title}')">View</button>
                <button onclick="deleteNote('${note.title}')">Delete</button>
            </div>
        `;
    list.appendChild(item);
  });
}

function showAddNote() {
  document.getElementById('add-note-modal').classList.remove('hidden');
}

async function saveNote() {
  const note = {
    title: document.getElementById('note-title').value,
    content: document.getElementById('note-content').value,
    category: document.getElementById('note-category').value || null
  };

  try {
    await apiCall('/vault/notes', 'POST', note, authToken);
    closeModal();
    loadNotes();
    showStatus('Note saved!', 'success');
  } catch (error) {
    showStatus(`Failed to save: ${error.message}`, 'error');
  }
}

async function viewNote(title) {
  try {
    const note = await apiCall(`/vault/notes/${title}`, 'GET', null, authToken);
    alert(`Title: ${note.title}\n\n${note.content}`);
  } catch (error) {
    showStatus(`Failed to view: ${error.message}`, 'error');
  }
}

async function deleteNote(title) {
  if (!confirm(`Delete note "${title}"?`)) return;

  try {
    await apiCall(`/vault/notes/${title}`, 'DELETE', null, authToken);
    loadNotes();
    showStatus('Note deleted!', 'success');
  } catch (error) {
    showStatus(`Failed to delete: ${error.message}`, 'error');
  }
}

// Password generation
function updateLength() {
  document.getElementById('length-display').textContent = document.getElementById('password-length').value;
}

function generatePassword() {
  const length = document.getElementById('password-length').value;
  const useUpper = document.getElementById('gen-uppercase').checked;
  const useLower = document.getElementById('gen-lowercase').checked;
  const useNumbers = document.getElementById('gen-numbers').checked;
  const useSymbols = document.getElementById('gen-symbols').checked;

  let charset = '';
  if (useUpper) charset += 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  if (useLower) charset += 'abcdefghijklmnopqrstuvwxyz';
  if (useNumbers) charset += '0123456789';
  if (useSymbols) charset += '!@#$%^&*()_+-=[]{}|;:,.<>?';

  if (!charset) {
    showStatus('Select at least one character set', 'error');
    return;
  }

  let password = '';
  const array = new Uint32Array(length);
  crypto.getRandomValues(array);

  for (let i = 0; i < length; i++) {
    password += charset[array[i] % charset.length];
  }

  document.getElementById('generated-password').textContent = password;
}

function generateAndFill() {
  generatePassword();
  const password = document.getElementById('generated-password').textContent;
  if (password) {
    document.getElementById('secret-password').value = password;
  }
}