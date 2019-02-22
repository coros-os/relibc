#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    FILE *fp = fopen("stdio/fread.in", "rb");

    char buf[33] = { 0 };
    for (int i = 1; i <= 32; ++i) {
        if (fread(buf, 1, i, fp) < 0) {
            perror("fread");
            return EXIT_FAILURE;
        }
        buf[i] = 0;

        printf("%s\n", buf);
    }

    fclose(fp);
}
