// Created by Noirkelo at 2025/08/05 14:04
// leetgo: dev
// https://leetcode.cn/problems/container-with-most-water/

#include <bits/stdc++.h>
#include "LC_IO.h"
using namespace std;

// @lc code=begin

class Solution {
	public:
		int maxArea(vector<int>& height) {
			int l = 0;
			int r = height.size()-1;
			int max_area = 0;
			while (l<r){
				if ( height[l]<height[r]){
					int current_area = height[l] * (r - l);
					max_area = max(current_area,max_area);
					l++;
				}
				else
				{
					int current_area = height[r] * (r - l);
					max_area = max(current_area,max_area);
					r--;
				}
			}
			return max_area;
		}
};

// @lc code=end

int main() {
	ios_base::sync_with_stdio(false);
	stringstream out_stream;

	vector<int> height;
	LeetCodeIO::scan(cin, height);

	Solution *obj = new Solution();
	auto res = obj->maxArea(height);
	LeetCodeIO::print(out_stream, res);
	cout << "\noutput: " << out_stream.rdbuf() << endl;

	delete obj;
	return 0;
}
