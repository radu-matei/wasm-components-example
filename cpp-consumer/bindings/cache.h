#ifndef __BINDINGS_CACHE_H
#define __BINDINGS_CACHE_H
#ifdef __cplusplus
extern "C"
{
  #endif
  
  #include <stdint.h>
  #include <stdbool.h>
  
  typedef struct {
    char *ptr;
    size_t len;
  } cache_string_t;
  
  void cache_string_set(cache_string_t *ret, const char *s);
  void cache_string_dup(cache_string_t *ret, const char *s);
  void cache_string_free(cache_string_t *ret);
  // A simple cache interface.
  // Type for cache errors.
  typedef uint8_t cache_error_t;
  #define CACHE_ERROR_RUNTIME_ERROR 0
  #define CACHE_ERROR_NOT_FOUND_ERROR 1
  // Payload for cache values.
  typedef struct {
    uint8_t *ptr;
    size_t len;
  } cache_payload_t;
  void cache_payload_free(cache_payload_t *ptr);
  typedef struct {
    // `true` if `val` is present, `false` otherwise
    bool tag;
    uint32_t val;
  } cache_option_u32_t;
  cache_error_t cache_set(cache_string_t *key, cache_payload_t *value, cache_option_u32_t *ttl);
  cache_error_t cache_get(cache_string_t *key, cache_payload_t *ret0);
  cache_error_t cache_delete(cache_string_t *key);
  #ifdef __cplusplus
}
#endif
#endif
