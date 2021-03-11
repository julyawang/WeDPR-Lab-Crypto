#ifndef _WEDPR_CRYPTO_H_
#define _WEDPR_CRYPTO_H_
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>
#include <ostream>

struct HashResult
{
    char* data;
    uintptr_t len;
};

struct KeyPairData
{
    char* public_key;
    uintptr_t public_key_len;
    char* private_key;
    uintptr_t private_key_len;
};

struct PublicKey
{
    char* public_key;
    uintptr_t public_key_len;
};

struct SignatureResult
{
    char* signature_data;
    uintptr_t signature_len;
};

extern "C" {

/// C interface for 'wedpr_keccak256_hash_binary'.
HashResult wedpr_keccak256_hash_binary(const char* encoded_message, uintptr_t message_len);

/// C interface for 'wedpr_sm3_hash_binary'.
HashResult wedpr_sm3_hash_binary(const char* encoded_message, uintptr_t message_len);

void dealloc_hash_result(HashResult hash_result);

/// C interface for 'wedpr_secp256k1_gen_binary_key_pair'.
KeyPairData wedpr_secp256k1_gen_binary_key_pair();

/// C interface for 'wedpr_secp256k1_derive_binary_public_key'.
PublicKey wedpr_secp256k1_derive_binary_public_key(
    const char* encoded_private_key, uintptr_t encoded_private_key_len);

/// C interface for 'wedpr_secp256k1_sign_binary'.
SignatureResult wedpr_secp256k1_sign_binary(const char* encoded_private_key,
    uintptr_t encoded_private_key_len, const char* encoded_message_hash,
    uintptr_t encoded_message_hash_len);

/// C interface for 'wedpr_secp256k1_verify_binary'.
int8_t wedpr_secp256k1_verify_binary(const char* encoded_public_key,
    uintptr_t encoded_public_key_len, const char* encoded_message_hash,
    uintptr_t encoded_message_hash_len, const char* encoded_signature,
    uintptr_t encoded_signature_len);

/// C interface for 'wedpr_secp256k1_recover_binary_public_key'.
PublicKey wedpr_secp256k1_recover_binary_public_key(const char* encoded_message_hash,
    uintptr_t encoded_message_hash_len, const char* encoded_signature,
    uintptr_t encoded_signature_len);

/// C interface for 'wedpr_sm2_gen_binary_key_pair'.
KeyPairData wedpr_sm2_gen_binary_key_pair();

/// C interface for 'wedpr_sm2_derive_binary_public_key'.
PublicKey wedpr_sm2_derive_binary_public_key(
    const char* encoded_private_key, uintptr_t encoded_private_key_len);

/// C interface for 'wedpr_sm2_sign_binary'.
SignatureResult wedpr_sm2_sign_binary(const char* encoded_private_key,
    uintptr_t encoded_private_key_len, const char* encoded_message_hash,
    uintptr_t encoded_message_hash_len);

/// C interface for 'wedpr_sm2_sign_binary_fast'.
SignatureResult wedpr_sm2_sign_binary_fast(const char* encoded_private_key,
    uintptr_t encoded_private_key_len, const char* encoded_public_key,
    uintptr_t encoded_public_key_len, const char* encoded_message_hash,
    uintptr_t encoded_message_hash_len);

/// C interface for 'wedpr_sm2_verify_binary'.
int8_t wedpr_sm2_verify_binary(const char* encoded_public_key, uintptr_t encoded_public_key_len,
    const char* encoded_message_hash, uintptr_t encoded_message_hash_len,
    const char* encoded_signature, uintptr_t signature_len);

void dealloc_key_pair(KeyPairData key_pair);

void dealloc_signature_data(SignatureResult signature_result);

void dealloc_public_key_data(PublicKey public_key_result);

/// C interface for 'wedpr_secp256k1_ecies_encrypt'.
char* wedpr_secp256k1_ecies_encrypt(char* encoded_public_key, char* encoded_plaintext);

/// C interface for 'wedpr_secp256k1_ecies_decrypt'.
char* wedpr_secp256k1_ecies_decrypt(char* encoded_private_key, char* encoded_ciphertext);

/// C interface for 'wedpr_keccak256_hash'.
char* wedpr_keccak256_hash(const char* encoded_message);

/// C interface for 'wedpr_sm3_hash'.
char* wedpr_sm3_hash(const char* encoded_message);

/// C interface for 'wedpr_secp256k1_gen_key_pair'.
char* wedpr_secp256k1_gen_key_pair();

/// C interface for 'wedpr_secp256k1_derive_public_key'.
char* wedpr_secp256k1_derive_public_key(const char* encoded_private_key);

/// C interface for 'wedpr_secp256k1_sign'.
char* wedpr_secp256k1_sign(const char* encoded_private_key, const char* encoded_message_hash);

/// C interface for 'wedpr_secp256k1_verify'.
int8_t wedpr_secp256k1_verify(const char* encoded_public_key, const char* encoded_message_hash,
    const char* encoded_signature);

/// C interface for 'wedpr_secp256k1_recover_public_key'.
char* wedpr_secp256k1_recover_public_key(
    const char* encoded_message_hash, const char* encoded_signature);

/// C interface for 'wedpr_sm2_gen_key_pair'.
char* wedpr_sm2_gen_key_pair();

/// C interface for 'wedpr_sm2_derive_public_key'.
char* wedpr_sm2_derive_public_key(const char* encoded_private_key);

/// C interface for 'wedpr_sm2_sign'.
char* wedpr_sm2_sign(const char* encoded_private_key, const char* encoded_message_hash);

/// C interface for 'wedpr_sm2_sign_fast'.
char* wedpr_sm2_sign_fast(const char* encoded_private_key, const char* encoded_public_key,
    const char* encoded_message_hash);

/// C interface for 'wedpr_sm2_verify'.
int8_t wedpr_sm2_verify(const char* encoded_public_key, const char* encoded_message_hash,
    const char* encoded_signature);

/// C interface for 'wedpr_curve25519_vrf_derive_public_key'.
char* wedpr_curve25519_vrf_derive_public_key(const char* encoded_private_key);

/// C interface for 'wedpr_curve25519_vrf_prove_utf8'.
char* wedpr_curve25519_vrf_prove_utf8(const char* encoded_private_key, const char* utf8_message);

/// C interface for 'wedpr_curve25519_vrf_prove_fast_utf8'.
char* wedpr_curve25519_vrf_prove_fast_utf8(
    const char* encoded_private_key, const char* encoded_public_key, const char* utf8_message);

/// C interface for 'wedpr_curve25519_vrf_verify_utf8'.
int8_t wedpr_curve25519_vrf_verify_utf8(
    const char* encoded_public_key, const char* utf8_message, const char* encoded_proof);

/// C interface for 'wedpr_curve25519_vrf_proof_to_hash'.
char* wedpr_curve25519_vrf_proof_to_hash(const char* encoded_proof);

/// C interface for 'wedpr_curve25519_vrf_is_valid_public_key'.
int8_t wedpr_curve25519_vrf_is_valid_public_key(const char* encoded_public_key);

}  // extern "C"

#endif
