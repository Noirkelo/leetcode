// Created by Noirkelo at 2025/08/06 11:37
// leetgo: dev
// https://leetcode.cn/problems/trapping-rain-water/

#include <bits/stdc++.h>
#include "LC_IO.h"
using namespace std;

// @lc code=begin

class Solution {
public:
    int trap(vector<int>& height) {
        
    }
};

// @lc code=end

int main() {
	ios_base::sync_with_stdio(false);
	stringstream out_stream;

	vector<int> height;
	LeetCodeIO::scan(cin, height);

	Solution *obj = new Solution();
	auto res = obj->trap(height);
	LeetCodeIO::print(out_stream, res);
	cout << "\noutput: " << out_stream.rdbuf() << endl;

	delete obj;
	return 0;
}
