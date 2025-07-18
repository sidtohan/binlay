#include <stddef.h>

int sum_array(const int *arr, size_t len) {
    int sum = 0;
    for (size_t i = 0; i < len; ++i) {
        sum += arr[i];
    }
    return sum;
}

int main() {
    int numbers[] = {1, 2, 3, 4, 5};
    return sum_array(numbers, 5);
}

