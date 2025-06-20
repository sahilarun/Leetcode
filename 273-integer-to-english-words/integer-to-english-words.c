char* below_20[] = {
    "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
    "Sixteen", "Seventeen", "Eighteen", "Nineteen"
};

char* tens[] = {
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"
};

char* thousands[] = { "", "Thousand", "Million", "Billion" };

void helper(int num, char* buf) {
    if (num >= 100) {
        strcat(buf, below_20[num / 100]); strcat(buf, " Hundred ");
        num %= 100;
    }
    if (num >= 20) {
        strcat(buf, tens[num / 10]); strcat(buf, " ");
        num %= 10;
    }
    if (num > 0) {
        strcat(buf, below_20[num]); strcat(buf, " ");
    }
}

char* numberToWords(int num) {
    if (num == 0) return strdup("Zero");

    char* res = calloc(1024, 1);
    int i = 0;
    while (num > 0) {
        int chunk = num % 1000;
        if (chunk) {
            char temp[256] = "";
            helper(chunk, temp);
            if (*thousands[i]) { strcat(temp, thousands[i]); strcat(temp, " "); }
            strcat(temp, res); strcpy(res, temp);
        }
        num /= 1000; i++;
    }

    int len = strlen(res);
    if (res[len - 1] == ' ') res[len - 1] = '\0';
    return res;
}