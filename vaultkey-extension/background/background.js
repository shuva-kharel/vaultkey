chrome.runtime.onInstalled.addListener(() => {
  console.log('Vaultkey extension installed');

  // Initialize storage
  chrome.storage.local.get(['vaultkey_initialized'], (result) => {
    if (!result.vaultkey_initialized) {
      chrome.storage.local.set({
        'vaultkey_initialized': true,
        'vaultkey_secrets': [],
        'vaultkey_notes': []
      });
    }
  });
});

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'getToken') {
    chrome.storage.local.get(['vaultkey_token'], (result) => {
      sendResponse({ token: result.vaultkey_token });
    });
    return true;
  }
});