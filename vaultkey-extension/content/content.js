// Listen for messages from popup
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
  if (request.action === 'fillPassword') {
    fillPasswordFields(request.username, request.password);
    sendResponse({ success: true });
  }
  return true;
});

function fillPasswordFields(username, password) {
  // Find username field
  const usernameField = findUsernameField();
  const passwordField = findPasswordField();

  if (usernameField) {
    setNativeValue(usernameField, username);
    usernameField.dispatchEvent(new Event('input', { bubbles: true }));
    usernameField.dispatchEvent(new Event('change', { bubbles: true }));
  }

  if (passwordField) {
    setNativeValue(passwordField, password);
    passwordField.dispatchEvent(new Event('input', { bubbles: true }));
    passwordField.dispatchEvent(new Event('change', { bubbles: true }));
  }
}

function findUsernameField() {
  // Try common selectors
  const selectors = [
    'input[type="email"]',
    'input[type="text"][name*="user" i]',
    'input[type="text"][name*="email" i]',
    'input[type="text"][id*="user" i]',
    'input[type="text"][id*="email" i]',
    'input[autocomplete="username"]',
    'input[autocomplete="email"]'
  ];

  for (const selector of selectors) {
    const field = document.querySelector(selector);
    if (field) return field;
  }

  return null;
}

function findPasswordField() {
  const selectors = [
    'input[type="password"]',
    'input[autocomplete="current-password"]',
    'input[autocomplete="new-password"]'
  ];

  for (const selector of selectors) {
    const field = document.querySelector(selector);
    if (field) return field;
  }

  return null;
}

function setNativeValue(element, value) {
  const valueSetter = Object.getOwnPropertyDescriptor(element, 'value')?.set;
  const prototype = Object.getPrototypeOf(element);
  const prototypeValueSetter = Object.getOwnPropertyDescriptor(prototype, 'value')?.set;

  if (valueSetter && valueSetter !== prototypeValueSetter) {
    prototypeValueSetter?.call(element, value);
  } else {
    valueSetter?.call(element, value);
  }
}