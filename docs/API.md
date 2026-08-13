# API Reference

The Vaultkey server provides WebAuthn registration/login and vault sync endpoints.

## Base URL

```
http://localhost:8080
```

## Authentication

Most endpoints require a JWT token obtained from the login endpoint. Include it in request headers:

```
Authorization: Bearer <jwt-token>
```

## WebAuthn Flow

### 1. Start Registration

**Endpoint**: `POST /register/start`

Initialize WebAuthn registration.

**Request**:

```json
{
  "username": "alice"
}
```

**Response** (200 OK):

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
        }
      ],
      "timeout": 300000,
      "attestation": "none"
    }
  },
  "user_id": "user-uuid"
}
```

### 2. Complete Registration

**Endpoint**: `POST /register/finish`

Finish WebAuthn registration with attestation response.

**Request**:

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

**Response** (200 OK):

```json
{
  "success": true
}
```

### 3. Start Login

**Endpoint**: `POST /login/start`

Initiate WebAuthn authentication.

**Request**:

```json
{
  "username": "alice"
}
```

**Response** (200 OK):

```json
{
  "publicKey": {
    "challenge": "base64-challenge",
    "rpId": "localhost",
    "allowCredentials": [
      {
        "type": "public-key",
        "id": "base64-credential-id"
      }
    ],
    "userVerification": "required"
  }
}
```

### 4. Complete Login

**Endpoint**: `POST /login/finish`

Complete authentication with signed assertion.

**Request**:

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

**Response** (200 OK):

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Errors**:

- `400 Bad Request` - Invalid request format
- `401 Unauthorized` - Authentication failed
- `404 Not Found` - User not found

## Vault Operations

All vault endpoints require JWT authentication.

### Upload Vault

**Endpoint**: `PUT /vault`

Upload encrypted vault data to the server.

**Headers**:

```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request**:

```json
{
  "data": "base64-encrypted-vault-data",
  "version": 1
}
```

**Response** (200 OK):

```json
{
  "success": true,
  "size": 2048
}
```

### Download Vault

**Endpoint**: `GET /vault`

Retrieve encrypted vault data from the server.

**Headers**:

```
Authorization: Bearer <token>
```

**Response** (200 OK):

```
<binary encrypted vault data>
```

**Headers**:

```
Content-Type: application/octet-stream
Content-Length: 2048
```

### Delete Vault

**Endpoint**: `DELETE /vault`

Remove vault from the server.

**Headers**:

```
Authorization: Bearer <token>
```

**Response** (204 No Content):

```
(empty body)
```

### Vault Status

**Endpoint**: `GET /vault/status`

Check vault existence and metadata.

**Headers**:

```
Authorization: Bearer <token>
```

**Response** (200 OK):

```json
{
  "exists": true,
  "size": 2048,
  "last_updated": "2024-01-15T10:30:00Z",
  "version": 1
}
```

## Error Responses

### 400 Bad Request

```json
"Invalid request format"
```

Common causes:

- Missing required fields
- Invalid JSON
- Invalid data types

### 401 Unauthorized

```json
"Unauthorized"
```

Common causes:

- Missing JWT token
- Expired token
- Invalid signature

### 404 Not Found

```json
"Not found"
```

Common causes:

- User doesn't exist
- Vault not found

### 500 Internal Server Error

```json
"Internal server error"
```

Indicates a server-side issue. Check server logs.

## Rate Limiting

**Status**: Not currently implemented

Production deployments should implement rate limiting:

- Authentication attempts: 5 per minute per IP
- Vault operations: 100 per hour per user
- Account registration: 10 per hour per IP

## HTTPS and Security

The API supports both HTTP and HTTPS. For production:

1. **Use HTTPS only**
   - Protects against man-in-the-middle attacks
   - Required for security

2. **Configure CORS**
   - Restrict to trusted origins
   - Prevent unauthorized cross-origin requests

3. **Set Security Headers**
   ```
   Strict-Transport-Security: max-age=31536000
   X-Content-Type-Options: nosniff
   X-Frame-Options: DENY
   ```

## Example: Client Flow

```bash
# 1. Start registration
curl -X POST http://localhost:8080/register/start \
  -H "Content-Type: application/json" \
  -d '{"username":"alice"}'

# 2. [Client processes challenge, gets credential]

# 3. Complete registration
curl -X POST http://localhost:8080/register/finish \
  -H "Content-Type: application/json" \
  -d '{
    "username":"alice",
    "user_id":"...",
    "registration":{...}
  }'

# 4. Start login
curl -X POST http://localhost:8080/login/start \
  -H "Content-Type: application/json" \
  -d '{"username":"alice"}'

# 5. [Client processes challenge, signs assertion]

# 6. Complete login
curl -X POST http://localhost:8080/login/finish \
  -H "Content-Type: application/json" \
  -d '{
    "username":"alice",
    "authentication":{...}
  }'

# 7. Upload vault (with token)
curl -X PUT http://localhost:8080/vault \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"data":"...","version":1}'

# 8. Download vault
curl -X GET http://localhost:8080/vault \
  -H "Authorization: Bearer $TOKEN" \
  -o vault.bin
```

## Versioning

The API is versioned via the `version` field in vault uploads. Current version: `1`

Breaking changes will increment the version. Clients should validate the version when downloading.

## Changelog

### v1.0

- WebAuthn registration and login
- Vault upload/download
- JWT authentication
- Vault status endpoint

### Planned for v1.1

- Rate limiting
- Audit logging
- User metadata
- Backup/restore endpoints

---

For more details on WebAuthn, see [webauthn-rs documentation](https://github.com/kanidm/webauthn-rs).
