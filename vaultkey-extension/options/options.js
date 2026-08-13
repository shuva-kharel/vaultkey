document.addEventListener('DOMContentLoaded', loadSettings);
document.getElementById('save-btn').addEventListener('click', saveSettings);

function loadSettings() {
  chrome.storage.local.get(['vaultkey_server', 'vaultkey_clipboard_timeout', 'vaultkey_auto_fill'], (result) => {
    document.getElementById('server-url').value = result.vaultkey_server || 'http://localhost:8000';
    document.getElementById('clipboard-timeout').value = result.vaultkey_clipboard_timeout || 30;
    document.getElementById('auto-fill').checked = result.vaultkey_auto_fill !== false;
  });
}

function saveSettings() {
  const settings = {
    'vaultkey_server': document.getElementById('server-url').value,
    'vaultkey_clipboard_timeout': parseInt(document.getElementById('clipboard-timeout').value) || 30,
    'vaultkey_auto_fill': document.getElementById('auto-fill').checked
  };

  chrome.storage.local.set(settings, () => {
    alert('Settings saved!');
  });
}