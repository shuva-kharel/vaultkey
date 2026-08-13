# Security Model

## Overview

Vaultkey is built on the principle that your encryption key should never exist as a password. Instead, it's derived from a hardware device you physically control.

## Cryptographic Primitives

### Symmetric Encryption

- **Algorithm**: ChaCha20-Poly1305 (AEAD)
- **Key Size**: 256-bit
- **Nonce**: 96-bit random, unique per encryption
- **Authentication**: Detects any tampering with ciphertext

ChaCha20-Poly1305 is a modern, audited AEAD cipher chosen for its:
- Speed on all platforms
- Resistance to timing attacks
- Authenticated encryption (prevents tampering)
- No hardware acceleration required

### Key Derivation

Currently: SHA-256 from hardware key challenge

Future: HMAC-based KDF or Argon2 for stronger key derivation from weaker secrets.

### Authentication

- **JWT**: HS256 for server communication
- **WebAuthn**: FIDO2 compatible registration and login
- **Timeout**: Tokens expire after 24 hours (configurable)

## Data Flow

### Storing a Secret

```
User Password
     ↓
Hardware Key Challenge
     ↓
Encryption Key Derivation (SHA-256)
     ↓
ChaCha20-Poly1305 Encryption
     ↓
Nonce + Ciphertext
     ↓
SQLite Database (encrypted at rest)
```

### Retrieving a Secret

```
Hardware Key Present
     ↓
Encryption Key Derivation
     ↓
ChaCha20-Poly1305 Decryption
     ↓
Plaintext in Memory
     ↓
Display/Clipboard
     ↓
Auto-clear (30 seconds)
```

### Server Sync

```
Encrypted Vault
     ↓
HTTP POST to Server
     ↓
Server Storage (encrypted blob only)
     ↓
Client: Authorized via JWT
```

The server never sees plaintext secrets. It only stores and syncs encrypted blobs.

## Threat Model

### Protected Against

| Threat | Mitigation |
|--------|-----------|
| **Database Theft** | Entire database is encrypted at rest |
| **Server Compromise** | Zero-knowledge architecture; server only stores encrypted blobs |
| **Password Guessing** | No master password to guess |
| **Replay Attacks** | Nonce-based encryption + AEAD authentication |
| **Man-in-the-Middle** | JWT tokens + HTTPS (in production) |
| **Brute Force** | 256-bit encryption keys |
| **Data Tampering** | Poly1305 AEAD authentication |

### Not Protected Against

| Threat | Reason | Mitigation |
|--------|--------|-----------|
| **Hardware Key Theft** | Possessor can decrypt all secrets | Use backup keys, full-disk encryption |
| **Malware on Device** | Can read process memory | Keep system clean, monitor processes |
| **Keylogging** | Can capture passwords before encryption | Use secure input methods |
| **Physical Access** | Can copy vault files | Full-disk encryption essential |
| **Weak Hardware Key PIN** | Limited tries on key | Use strong PIN if key requires one |

## Security Best Practices

### For Users

1. **Secure Your Hardware Key**
   - Store it in a safe place
   - Keep it away from others
   - Use a backup hardware key
   - Never share your key

2. **System Security**
   - Enable full-disk encryption (BitLocker, FileVault, LUKS)
   - Keep your OS and software updated
   - Use antivirus software
   - Lock your computer when away

3. **Password Management**
   - Use unique passwords for each service
   - Use generated passwords when possible
   - Enable 2FA where available
   - Don't share vault database files

4. **Server Sync**
   - Use HTTPS in production
   - Keep your JWT tokens private
   - Log out from untrusted devices
   - Change secret key regularly

### For Deployment

1. **Production Server**
   ```bash
   # Generate strong JWT secret
   openssl rand -base64 32
   ```

2. **HTTPS/TLS**
   - Always use HTTPS in production
   - Use valid certificates
   - Enable HSTS headers

3. **Database Security**
   - Use database backups
   - Monitor database access logs
   - Implement rate limiting
   - Use firewall rules

4. **Monitoring**
   - Log authentication attempts
   - Monitor server resource usage
   - Set up alerts for unusual activity
   - Regular security audits

## Security Limitations

### Current Version

- **Simulated Hardware Key**: Uses a file-based key instead of real FIDO2/TPM
- **No HTTPS**: Development only; use HTTP for testing only
- **Basic Key Derivation**: SHA-256 instead of Argon2
- **No Rate Limiting**: Server vulnerable to brute force
- **Limited Audit Logging**: Minimal activity logging

These limitations are acceptable for local/development use but should be addressed before production deployment.

### Future Improvements

- [ ] Real FIDO2 YubiKey support
- [ ] TPM integration
- [ ] Phone passkey support
- [ ] Argon2 key derivation function
- [ ] Rate limiting and DDoS protection
- [ ] Comprehensive audit logging
- [ ] Secure memory handling
- [ ] Post-quantum cryptography research

## Cryptographic Assumptions

We rely on:

1. **ChaCha20 is a secure cipher** - Audited, no known weaknesses
2. **Poly1305 is a secure authenticator** - Standard AEAD scheme
3. **SHA-256 has collision resistance** - NIST standardized
4. **Hardware keys are tamper-resistant** - Depends on device quality
5. **Random nonces are truly random** - Ensured by OS RNG

## Security Audits

Vaultkey is open source. Community security reviews are welcome. If you discover a vulnerability, please report it privately to `security@vaultkey.example.com`.

## Additional Resources

- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Argon2: The Memory-Hard Password Hash](https://password-hashing.net/)
- [FIDO2 Specifications](https://fidoalliance.org/fido2/)
- [ChaCha20 Paper](https://cr.yp.to/chacha/chacha-20080128.pdf)

## Incident Response

If you suspect a security breach:

1. **Immediate Actions**
   - Disconnect affected device from network
   - Check for unauthorized access
   - Review recent account activity

2. **Investigation**
   - Check server logs
   - Verify vault integrity
   - Review JWT token usage

3. **Remediation**
   - Change all passwords
   - Revoke active JWT tokens
   - Rotate hardware keys if necessary
   - Update security practices

4. **Recovery**
   - Restore from clean backup if available
   - Rebuild vault on clean system
   - Re-register hardware keys

## Contact

For security concerns or vulnerability reports, please reach out privately. Do not create public GitHub issues for security vulnerabilities.

---

Last updated: 2024
Security is a continuous process. Keep updated with the latest changes.