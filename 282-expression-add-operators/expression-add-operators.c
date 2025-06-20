#define MAX_RES 10000
#define MAX_EXPR 128

char** res;
int resSize;

void dfs(char* num, int len, int pos, long eval, long last, char* expr, int eLen, int target) {
    if (pos == len) {
        if (eval == target) {
            expr[eLen] = '\0';
            res[resSize++] = strdup(expr);
        }
        return;
    }

    long val = 0;
    int start = pos, lenAdded;

    for (int i = pos; i < len; i++) {
        if (i > pos && num[pos] == '0') break;
        val = val * 10 + (num[i] - '0');
        lenAdded = i - pos + 1;
        memcpy(expr + eLen + 1, num + pos, lenAdded);

        if (pos == 0) {
            memcpy(expr, num, lenAdded);
            dfs(num, len, i + 1, val, val, expr, lenAdded, target);
        } else {
            expr[eLen] = '+';
            dfs(num, len, i + 1, eval + val, val, expr, eLen + lenAdded + 1, target);

            expr[eLen] = '-';
            dfs(num, len, i + 1, eval - val, -val, expr, eLen + lenAdded + 1, target);

            expr[eLen] = '*';
            dfs(num, len, i + 1, eval - last + last * val, last * val, expr, eLen + lenAdded + 1, target);
        }
    }
}

char** addOperators(char* num, int target, int* returnSize) {
    res = malloc(MAX_RES * sizeof(char*));
    resSize = 0;
    char* expr = malloc(MAX_EXPR);
    dfs(num, strlen(num), 0, 0, 0, expr, 0, target);
    free(expr);
    *returnSize = resSize;
    return res;
}