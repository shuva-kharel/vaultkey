const API_BASE = 'http://localhost:8000';

document.addEventListener('DOMContentLoaded', () => {
  checkLoginStatus();
  setupEventListeners();
});

function checkLoginStatus() {
  chrome.storage.local.get(['vaultkey_token', 'vaultkey_user'], (result) => {
    if (result.vaultkey_token && result.vaultkey_user) {
      showLoggedIn(result.vaultkey_user);
    } else {
      showLoggedOut();
    }
  });
}

function setupEventListeners() {
  document.getElementById('login-btn').addEventListener('click', loginWithPasskey);
  document.getElementById('register-btn').addEventListener('click', registerWithPasskey);
  document.getElementById('logout-btn').addEventListener('click', logout);
  document.getElementById('generate-btn').addEventListener('click', generatePassword);
  document.getElementById('settings-btn').addEventListener('click', openSettings);
  document.getElementById('search').addEventListener('input', filterPasswords);
}

function showLoggedOut() {
  document.getElementById('not-logged-in').classList.remove('hidden');
  document.getElementById('logged-in').classList.add('hidden');
}

function showLoggedIn(username) {
  document.getElementById('not-logged-in').classList.add('hidden');
  document.getElementById('logged-in').classList.remove('hidden');
  loadPasswords();
  showStatus(`Logged in as ${username}`, 'success');
}

function showStatus(message, type) {
  const statusDiv = document.getElementById('status-message');
  statusDiv.textContent = message;
  statusDiv.className = type;
  setTimeout(() => {
    statusDiv.textContent = '';
    statusDiv.className = '';
  }, 3000);
}

async function loginWithPasskey() {
  const username = document.getElementById('username').value;

  if (!username) {
    showStatus('Please enter a username', 'error');
    return;
  }

  showLoading();

  try {
    // Step 1: Start login
    const startResponse = await fetch(`${API_BASE}/login/start`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username })
    });

    if (!startResponse.ok) {
      throw new Error('User not found. Please register first.');
    }

    const startData = await startResponse.json();
    const challenge = startData.publicKey.challenge;

    // Step 2: Perform WebAuthn authentication
    const assertion = await performWebAuthn(challenge);

    // Step 3: Complete login
    const finishResponse = await fetch(`${API_BASE}/login/finish`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username,
        authentication: assertion
      })
    });

    if (!finishResponse.ok) {
      throw new Error('Authentication failed');
    }

    const finishData = await finishResponse.json();

    // Save token
    chrome.storage.local.set({
      'vaultkey_token': finishData.token,
      'vaultkey_user': username
    }, () => {
      showLoggedIn(username);
    });

  } catch (error) {
    showStatus(`Login failed: ${error.message}`, 'error');
  }

  hideLoading();
}

async function registerWithPasskey() {
  const username = document.getElementById('username').value;

  if (!username) {
    showStatus('Please enter a username', 'error');
    return;
  }

  showLoading();

  try {
    // Step 1: Start registration
    const startResponse = await fetch(`${API_BASE}/register/start`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username })
    });

    if (!startResponse.ok) {
      throw new Error('Registration failed');
    }

    const startData = await startResponse.json();
    const challenge = startData.challenge.publicKey.challenge;
    const userId = startData.user_id;

    // Step 2: Perform WebAuthn registration
    const credential = await performWebAuthnRegistration(username, challenge, userId);

    // Step 3: Complete registration
    const finishResponse = await fetch(`${API_BASE}/register/finish`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username,
        user_id: userId,
        registration: credential
      })
    });

    if (!finishResponse.ok) {
      throw new Error('Registration failed');
    }

    showStatus('Registration successful! Please login now.', 'success');

  } catch (error) {
    showStatus(`Registration failed: ${error.message}`, 'error');
  }

  hideLoading();
}

// WebAuthn helpers
function base64urlToBuffer(base64url) {
  const base64 = base64url.replace(/-/g, '+').replace(/_/g, '/');
  const padding = '='.repeat((4 - base64.length % 4) % 4);
  const paddedBase64 = base64 + padding;

  const binary = atob(paddedBase64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    bytes[i] = binary.charCodeAt(i);
  }
  return bytes.buffer;
}

function bufferToBase64url(buffer) {
  const bytes = new Uint8Array(buffer);
  let binary = '';
  for (let i = 0; i < bytes.length; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  const base64 = btoa(binary);
  return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}

async function performWebAuthn(challenge) {
  const publicKey = {
    challenge: base64urlToBuffer(challenge),
    rpId: "localhost",
    allowCredentials: [],
    userVerification: "required",
    timeout: 60000
  };

  const assertion = await navigator.credentials.get({ publicKey });

  return {
    id: assertion.id,
    rawId: bufferToBase64url(assertion.rawId),
    type: assertion.type,
    response: {
      clientDataJSON: bufferToBase64url(assertion.response.clientDataJSON),
      authenticatorData: bufferToBase64url(assertion.response.authenticatorData),
      signature: bufferToBase64url(assertion.response.signature),
      userHandle: assertion.response.userHandle ?
        bufferToBase64url(assertion.response.userHandle) : null
    }
  };
}

async function performWebAuthnRegistration(username, challenge, userId) {
  const publicKey = {
    challenge: base64urlToBuffer(challenge),
    rp: {
      name: "Vaultkey",
      id: "localhost"
    },
    user: {
      id: new TextEncoder().encode(userId).buffer,
      name: username,
      displayName: username
    },
    pubKeyCredParams: [
      { type: "public-key", alg: -7 },
      { type: "public-key", alg: -257 }
    ],
    timeout: 60000,
    attestation: "none"
  };

  const credential = await navigator.credentials.create({ publicKey });

  return {
    id: credential.id,
    rawId: bufferToBase64url(credential.rawId),
    type: credential.type,
    response: {
      clientDataJSON: bufferToBase64url(credential.response.clientDataJSON),
      attestationObject: bufferToBase64url(credential.response.attestationObject)
    }
  };
}

function logout() {
  chrome.storage.local.remove(['vaultkey_token', 'vaultkey_user'], () => {
    showLoggedOut();
    showStatus('Logged out', 'success');
  });
}

async function loadPasswords() {
  showLoading();

  try {
    const token = await getToken();

    if (!token) {
      hideLoading();
      return;
    }

    const response = await fetch(`${API_BASE}/vault/secrets`, {
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    });

    if (response.ok) {
      const secrets = await response.json();
      displayPasswords(secrets);
    } else {
      showStatus('Failed to load passwords', 'error');
    }
  } catch (error) {
    showStatus('Failed to connect to server', 'error');
  }

  hideLoading();
}

function getToken() {
  return new Promise((resolve) => {
    chrome.storage.local.get(['vaultkey_token'], (result) => {
      resolve(result.vaultkey_token);
    });
  });
}

function displayPasswords(secrets) {
  const list = document.getElementById('password-list');
  list.innerHTML = '';

  if (!secrets || secrets.length === 0) {
    list.innerHTML = '<div class="empty-state">No passwords saved</div>';
    return;
  }

  secrets.forEach(secret => {
    const item = document.createElement('div');
    item.className = 'password-item';
    item.innerHTML = `
            <div class="password-info">
                <div class="password-name">${secret.name}</div>
                <div class="password-username">${secret.username}</div>
            </div>
            <div class="password-actions">
                <button class="action-btn" data-action="fill" data-name="${secret.name}">Fill</button>
                <button class="action-btn" data-action="copy" data-name="${secret.name}">Copy</button>
            </div>
        `;

    item.querySelector('[data-action="fill"]').addEventListener('click', () => fillPassword(secret));
    item.querySelector('[data-action="copy"]').addEventListener('click', () => copyPassword(secret));

    list.appendChild(item);
  });
}

async function fillPassword(secret) {
  chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
    chrome.tabs.sendMessage(tabs[0].id, {
      action: 'fillPassword',
      username: secret.username,
      password: secret.password
    });
  });
  window.close();
}

async function copyPassword(secret) {
  await navigator.clipboard.writeText(secret.password);

  // Clear after 30 seconds
  setTimeout(async () => {
    await navigator.clipboard.writeText('');
  }, 30000);

  showStatus('Password copied! Auto-clear in 30s.', 'success');
}

function filterPasswords() {
  const query = document.getElementById('search').value.toLowerCase();
  const items = document.querySelectorAll('.password-item');

  items.forEach(item => {
    const text = item.textContent.toLowerCase();
    item.style.display = text.includes(query) ? 'flex' : 'none';
  });
}

function generatePassword() {
  const length = 20;
  const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?';

  let password = '';
  const array = new Uint32Array(length);
  crypto.getRandomValues(array);

  for (let i = 0; i < length; i++) {
    password += charset[array[i] % charset.length];
  }

  navigator.clipboard.writeText(password);
  showStatus('Generated password copied!', 'success');
}

function openSettings() {
  chrome.runtime.openOptionsPage();
}

function showLoading() {
  document.getElementById('loading').classList.remove('hidden');
}

function hideLoading() {
  document.getElementById('loading').classList.add('hidden');
}

// Add to setupEventListeners
document.getElementById('pin-login-btn').addEventListener('click', showPinSection);
document.getElementById('pin-submit-btn').addEventListener('click', loginWithPin);

function showPinSection() {
  document.getElementById('pin-section').classList.remove('hidden');
  document.getElementById('pin-input').focus();
}

async function loginWithPin() {
  const username = document.getElementById('username').value;
  const pin = document.getElementById('pin-input').value;

  if (!username || !pin) {
    showStatus('Please enter username and PIN', 'error');
    return;
  }

  showLoading();

  try {
    // Check if PIN matches stored PIN for this user
    const result = await chrome.storage.local.get('vaultkey_pins');
    const pins = result.vaultkey_pins || {};

    if (pins[username] === pin) {
      // Generate a simple token (in production, this would be more secure)
      const token = 'pin-' + btoa(username + ':' + Date.now());

      chrome.storage.local.set({
        'vaultkey_token': token,
        'vaultkey_user': username,
        'vaultkey_auth_method': 'pin'
      }, () => {
        showLoggedIn(username);
      });
    } else {
      showStatus('Invalid PIN', 'error');
    }
  } catch (error) {
    showStatus('Login failed: ' + error.message, 'error');
  }

  hideLoading();
}

// Add PIN setup after registration
async function setupPin(username) {
  const pin = prompt('Set up a PIN for quick access (4-6 digits):');

  if (pin && pin.length >= 4 && pin.length <= 6) {
    const result = await chrome.storage.local.get('vaultkey_pins');
    const pins = result.vaultkey_pins || {};
    pins[username] = pin;

    await chrome.storage.local.set({ 'vaultkey_pins': pins });
    showStatus('PIN set up successfully!', 'success');
  } else if (pin) {
    showStatus('PIN must be 4-6 digits', 'error');
  }
}