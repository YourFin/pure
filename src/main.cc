#include "main.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

/*
 * Things to keep track of:
 * current directory
 *  - collapse directories with single directory children (cough java)
 *  - $HOME -> ~
 * git
 *  - Current branch
 *  - Dirty
 *  - Vs. remote
 *  - In process of rebase?
 *  - merge?
 * SSH status
 * background process running
 *  - pid?
 *  - command?
 *  - Stopped jobs?
 * How long previous command took
 * Previous command exit status
 * nix?
 * python venv?
 */

bool isSsh() {
  if (getenv("SSH_CLIENT") != NULL) {
    return true;
  } else if (getenv("SSH_TTY") != NULL) {
    return true;
  } else {
    return false;
  }
}

// See:
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
#define COLOR_RESET 22
#define FULL_RESET 0
// Allow macro as argument
#define ANSI_GRAPHIC_ESCAPE(CODE) XANSI_GRAPHIC_ESCAPE(CODE)
#define XANSI_GRAPHIC_ESCAPE(CODE) "\033[" #CODE "m"
#define DELIMITER_CHAR " "
char *barSection(size_t sectionTextC, const char *sectionText, rgb *bg) {
  size_t charNum = 3              // Number of characters in start of escape
                   + 4            // Number of characters in start 8-bit escape
                   + (3 * 3 + 2)  // Maximum number of characters in color code
                   + sectionTextC // Number of charcters in text to print
                   + 1;           // delimiter character
  char *ret = (char *)malloc(sizeof(char) * charNum);

  // clang-format off
  int err = snprintf(ret,
					 charNum,
					 ANSI_GRAPHIC_ESCAPE(48;2;%d;%d;%d) "%s" DELIMITER_CHAR,
					 bg->red,
					 bg->green,
					 bg->blue,
					 sectionText);
  // clang-format on
  if (err != 0) {
    fprintf(stderr,
            "snprintf returned %d while building section with text '%s'\n", err,
            sectionText);
  }
  return ret;
}

int main(int argc, char *argv[]) {
  char *str = "hello";
  printf("%s" ANSI_GRAPHIC_ESCAPE(COLOR_RESET) "\n",
         barSection(5, str, &(struct rgb){0, 0, 255}));
  return 0;
}
