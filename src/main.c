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

// char* buildBarSection(const char* sectionText, const char* color)

int main(int argc, char *argv[]) {
  if (argc > 1) {
    printf("\n");
  }
}
