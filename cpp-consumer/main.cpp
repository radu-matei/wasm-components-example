#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <iostream>

#include "bindings/cache.h"

int main(int argc, char **argv)
{
    char *key = "almost-consul";
    char *value = "Caligula's horse, Incitatus";
    ;

    cache_string_t *skey;
    skey->len = strlen(key);
    skey->ptr = key;

    cache_payload_t *svalue;
    svalue->len = strlen(value);
    svalue->ptr = (uint8_t *)value;

    cache_set(skey, svalue, NULL);

    cache_payload_t *res;
    cache_get(skey, res);
    printf("Retrieved from `%s`: `%s`", key, (char *)res->ptr);

    assert(svalue->len == res->len);
}
