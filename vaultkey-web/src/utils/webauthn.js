class WebAuthnHelper {
  static base64urlToBuffer(base64url) {
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

  static bufferToBase64url(buffer) {
    const bytes = new Uint8Array(buffer);
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    const base64 = btoa(binary);
    return base64.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
  }

  static async register(username, challenge, userId) {
    const publicKey = {
      challenge: this.base64urlToBuffer(challenge),
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

    // Convert to JSON-safe format
    return {
      id: credential.id,
      rawId: this.bufferToBase64url(credential.rawId),
      type: credential.type,
      response: {
        clientDataJSON: this.bufferToBase64url(credential.response.clientDataJSON),
        attestationObject: this.bufferToBase64url(credential.response.attestationObject)
      }
    };
  }

  static async authenticate(challenge) {
    const publicKey = {
      challenge: this.base64urlToBuffer(challenge),
      rpId: "localhost",
      allowCredentials: [],
      userVerification: "required",
      timeout: 60000
    };

    const assertion = await navigator.credentials.get({ publicKey });

    return {
      id: assertion.id,
      rawId: this.bufferToBase64url(assertion.rawId),
      type: assertion.type,
      response: {
        clientDataJSON: this.bufferToBase64url(assertion.response.clientDataJSON),
        authenticatorData: this.bufferToBase64url(assertion.response.authenticatorData),
        signature: this.bufferToBase64url(assertion.response.signature),
        userHandle: assertion.response.userHandle ?
          this.bufferToBase64url(assertion.response.userHandle) : null
      }
    };
  }
}

export default WebAuthnHelper;