#ifndef __ACCOUNTEDSTR_H_
#define __ACCOUNTEDSTR_H_

typedef struct AccountedStr {
  char *__str; // Null-terminated internally, so one bigger than length
  int length;
} AccountedStr;

#endif // __ACCOUNTEDSTR_H_
