#include <stdio.h> // printf
#include <stdint.h> // uint8_t, uint32_t
#include <stdlib.h> // exit

void encode_utf8(uint32_t *src, char *dst) {
    int i = 0;
    int j = 0;

    while (1) {
        uint32_t scalar = src[i];

        if (scalar == 0) {
            dst[j] = 0; // null terminator
            break;
        }

        if (scalar > 0b11111111111) {
            fprintf(stderr, "Can only encode codepoints <= 0x%x", 0b11111111111);
            exit(1);
        }

        if (scalar > 0b1111111) { // 7 bits
            // 2-byte sequence

            uint8_t b1 = 0b11000000 | ((uint8_t) ((scalar & 0b11111000000) >> 6));
            //           2-byte marker              first 5 of 11 bits

            uint8_t b2 = 0b10000000 | ((uint8_t) (scalar & 0b111111));
            //           continuation               last 6 of 11 bits  

            dst[j + 0] = b1;
            dst[j + 1] = b2;
            j += 2;
        } else {
            // 1-byte sequence
            dst[j] = (char) scalar;
            j++;
        }

        i++;
    }
}

void decode_utf8(char *src, uint32_t *dst) {
    int i = 0;
    int j = 0;

    while (1) {
        uint8_t c = src[i];
        if (c == 0) {
            dst[j] = 0;
            break; // null terminator
        }

        uint32_t scalar;
        int len;

        if (c >> 3 == 0b11110) {
            fprintf(stderr, "decode_utf8: 4-byte sequences are not supported!\n");
            exit(1);
        } if (c >> 4 == 0b1110) {
            fprintf(stderr, "decode_utf8: 3-byte sequences are not supported!\n");
            exit(1);
        } else if (c >> 5 == 0b110) {
            // 2-byte sequence
            uint32_t b1 = (uint32_t) src[i];
            uint32_t b2 = (uint32_t) src[i + 1];
            uint32_t mask1 = 0b0000011111000000;
            uint32_t mask2 = 0b0000000000111111;

            scalar = ((b1 << 6) & mask1) | ((b2 << 0) & mask2);
            len = 2;
        } else {
            // 1-byte sequence
            scalar = (uint32_t) c;
            len = 1;
        }
        dst[j++] = scalar;
        i += len;
    }
}

int main(int argc, char **argv) {
    uint32_t scalars[1024]; // hopefully that's enough
    decode_utf8(argv[1], scalars);

    for (int i = 0;; i++) {
        if (scalars[i] == 0) {
            break;
        }
        printf("U+%04X ", scalars[i]);
    }
    printf("\n");

    uint8_t result[1024]; // yolo
    encode_utf8(scalars, result);

    printf("%s\n", result);

    return 0;
}
