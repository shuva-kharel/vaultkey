# Vaultkey API Reference

The Vaultkey server provides WebAuthn registration/login, vault sync, and secret/note management endpoints.

## Base URL

Development:

```text
http://localhost:8000
```

Docker Deployment:

```text
http://localhost:8443
```

---

## Authentication

Most endpoints require a JWT token obtained from the login endpoint.

Include it in request headers:

```http
Authorization: Bearer <jwt-token>
```

---

# WebAuthn Flow

## 1. Start Registration

**Endpoint**

```http
POST /register/start
```

Initialize WebAuthn registration.

### Request

```json
{
  "username": "alice"
}
```

### Response (200 OK)

```json
{
  "challenge": {
    "publicKey": {
      "rp": {
        "name": "Vaultkey",
        "id": "localhost"
      },
      "user": {
        "id": "user-uuid",
        "name": "alice",
        "displayName": "alice"
      },
      "challenge": "base64-encoded-challenge",
      "pubKeyCredParams": [
        {
          "alg": -7,
          "type": "public-key"
        },
        {
          "alg": -257,
          "type": "public-key"
        }
      ],
      "timeout": 60000,
      "attestation": "none"
    }
  },
  "user_id": "user-uuid"
}
```

---

## 2. Complete Registration

**Endpoint**

```http
POST /register/finish
```

Finish WebAuthn registration with attestation response.

### Request

```json
{
  "username": "alice",
  "user_id": "user-uuid",
  "registration": {
    "id": "credential-id",
    "rawId": "base64-credential-id",
    "type": "public-key",
    "response": {
      "clientDataJSON": "base64-data",
      "attestationObject": "base64-data"
    }
  }
}
```

### Response

```json
"Registration successful"
```

---

## 3. Start Login

**Endpoint**

```http
POST /login/start
```

### Request

```json
{
  "username": "alice"
}
```

### Response

```json
{
  "publicKey": {
    "challenge": "base64-challenge",
    "rpId": "localhost",
    "allowCredentials": [],
    "userVerification": "required",
    "timeout": 60000
  }
}
```

---

## 4. Complete Login

**Endpoint**

```http
POST /login/finish
```

### Request

```json
{
  "username": "alice",
  "authentication": {
    "id": "credential-id",
    "rawId": "base64-credential-id",
    "type": "public-key",
    "response": {
      "clientDataJSON": "base64-data",
      "authenticatorData": "base64-data",
      "signature": "base64-signature",
      "userHandle": null
    }
  }
}
```

### Response

```json
{
  "token": "jwt-token"
}
```

### Errors

- `400 Bad Request` - Invalid request format
- `401 Unauthorized` - Authentication failed
- `404 Not Found` - User not found

---

# Secret Management

All secret endpoints require JWT authentication.

## List Secrets

```http
GET /vault/secrets
```

### Response

```json
[
  {
    "name": "github",
    "username": "user@email.com",
    "password": "secret-password",
    "url": "https://github.com",
    "notes": "My GitHub account",
    "category": "Development"
  }
]
```

---

## Add Secret

```http
POST /vault/secrets
```

### Request

```json
{
  "name": "github",
  "username": "user@email.com",
  "password": "secret-password",
  "url": "https://github.com",
  "notes": "My GitHub account",
  "category": "Development"
}
```

### Response

Returns the created secret.

---

## Get Secret

```http
GET /vault/secrets/{name}
```

### Response

```json
{
  "name": "github",
  "username": "user@email.com",
  "password": "secret-password",
  "url": "https://github.com",
  "notes": "My GitHub account",
  "category": "Development"
}
```

---

## Delete Secret

```http
DELETE /vault/secrets/{name}
```

### Response

```json
"Secret deleted"
```

---

# Note Management

## List Notes

```http
GET /vault/notes
```

### Response

```json
[
  {
    "title": "Recovery Codes",
    "content": "My recovery codes...",
    "category": "Security"
  }
]
```

---

## Add Note

```http
POST /vault/notes
```

### Request

```json
{
  "title": "Recovery Codes",
  "content": "My recovery codes...",
  "category": "Security"
}
```

---

## Get Note

```http
GET /vault/notes/{title}
```

---

## Delete Note

```http
DELETE /vault/notes/{title}
```

---

# Vault Blob Operations

## Upload Vault

```http
PUT /vault
```

### Request

```json
{
  "data": "base64-encrypted-vault-data",
  "version": 1
}
```

---

## Download Vault

```http
GET /vault
```

Returns encrypted vault data.

---

## Delete Vault

```http
DELETE /vault
```

---

# Health Check

```http
GET /health
```

### Response

```json
"ok"
```

---

# CORS

CORS is enabled for all origins in development.

For production deployments, restrict access to trusted origins.

---

# Error Responses

## 400 Bad Request

```json
"Invalid request"
```

## 401 Unauthorized

```json
"Unauthorized"
```

## 404 Not Found

```json
"Not found"
```

## 500 Internal Server Error

```json
"Internal server error"
```

---

# Rate Limiting

**Current Status:** Not implemented

Recommended production limits:

- Authentication: 5 attempts/minute/IP
- Vault operations: 100 requests/hour/user
- Registration: 10 requests/hour/IP

---

# Example Client Flow

```bash
curl -X POST http://localhost:8000/register/start \
-H "Content-Type: application/json" \
-d '{"username":"alice"}'
```

```bash
curl -X POST http://localhost:8000/register/finish \
-H "Content-Type: application/json" \
-d '{"username":"alice","user_id":"...","registration":{...}}'
```

```bash
curl -X POST http://localhost:8000/login/start \
-H "Content-Type: application/json" \
-d '{"username":"alice"}'
```

```bash
curl -X POST http://localhost:8000/login/finish \
-H "Content-Type: application/json" \
-d '{"username":"alice","authentication":{...}}'
```

```bash
curl -X POST http://localhost:8000/vault/secrets \
-H "Authorization: Bearer $TOKEN" \
-H "Content-Type: application/json" \
-d '{"name":"github","username":"user","password":"pass"}'
```

```bash
curl -X GET http://localhost:8000/vault/secrets \
-H "Authorization: Bearer $TOKEN"
```

---

# Versioning

Current API Version:

```text
v1
```

Breaking changes will be announced in future releases.
Clients should always validate server responses.
