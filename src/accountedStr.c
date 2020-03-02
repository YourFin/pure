#include "accountedStr.h"
#include <stdlib.h>
#include <string.h>

// Creates an accountedStr from null-terminated char*
// Copies $in, so the original can be reused
AccountedStr *AccountedStr_fromCStr(const char *in) {
  AccountedStr *ret = (AccountedStr *)malloc(sizeof(AccountedStr));
  if (ret == NULL) {
    return NULL;
  }
  ret->length = strlen(in);
  ret->__str = malloc(sizeof(char) * (ret->length + 1));
  if (ret->__str == NULL) {
    free(ret);
    return NULL;
  }
  memcpy(ret->__str, in, ret->length + 1); // Copy over null byte too
  return ret;
}

// Free $str
void AccountedStr_destroy(AccountedStr *str) {
  free(str->__str);
  free(str);
}

// Append second to first
// Returns new AccountedStr, does not affect inputs
AccountedStr *AccountedStr_append(AccountedStr *first, AccountedStr *second) {
  AccountedStr *ret = malloc(sizeof(AccountedStr));
  if (ret == NULL) {
    return NULL;
  }
  ret->length = first->length + second->length;
  ret->__str = malloc(sizeof(char) * ret->length);
  if (ret->__str == NULL) {
    free(ret);
    return NULL;
  }
  // Copy first string without null terminator
  memcpy(ret->__str, first->__str, first->length);

  // Copy over second including null terminated string
  memcpy(ret->__str + first->length, second->__str, second->length + 1);

  return ret;
}

// Get CString from $in
// Copies memory, so you don't have to worry bout screwing with what you get
char *AccountedStr_toCStr(AccountedStr *in) {
  char *ret = malloc(sizeof(char) * (in->length + 1));
  if (ret == NULL) {
    return NULL;
  }
  memcpy(ret, in->__str, in->length + 1);
}
