class Solution {
    public int calculate(String s) {
        return helper(s.toCharArray(), new int[]{0});
    }

    private int helper(char[] s, int[] i) {
        int num = 0, res = 0, sign = 1;

        while (i[0] < s.length) {
            char c = s[i[0]++];
            if (Character.isDigit(c)) {
                num = num * 10 + (c - '0');
            } else if (c == '+') {
                res += sign * num;
                num = 0;
                sign = 1;
            } else if (c == '-') {
                res += sign * num;
                num = 0;
                sign = -1;
            } else if (c == '(') {
                num = helper(s, i);
            } else if (c == ')') {
                break;
            }
        }

        return res + sign * num;
    }
}
