// Created by Noirkelo at 2025/08/05 14:18
// leetgo: dev
// https://leetcode.cn/problems/3sum/

#include <bits/stdc++.h>
#include "LC_IO.h"
using namespace std;

// @lc code=begin

class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
		sort(nums.begin(),nums.end());
		vector<vector<int>> ans = vector<vector<int>>{};
		for (int i = 0; i < nums.size(); i++){
			if (i>0 && nums[i]==nums[i-1]){
				continue;
			}
			int k = nums.size() - 1;
			for (int j = i + 1; j < nums.size(); j++)
			{
				if (j>i+1 && nums[j]==nums[j-1]){
					continue;
				}
				while (j<k&& nums[i]+nums[j]+nums[k]>0){
					k--;
				}
				if (j==k){
					break;
				}
				if (nums[i]+nums[j]+nums[k]==0){
					ans.push_back(vector<int>{nums[i],nums[j],nums[k]});
				}
			}
		}
		return ans;
	}
};

// @lc code=end

int main() {
	ios_base::sync_with_stdio(false);
	stringstream out_stream;

	vector<int> nums;
	LeetCodeIO::scan(cin, nums);

	Solution *obj = new Solution();
	auto res = obj->threeSum(nums);
	LeetCodeIO::print(out_stream, res);
	cout << "\noutput: " << out_stream.rdbuf() << endl;

	delete obj;
	return 0;
}
