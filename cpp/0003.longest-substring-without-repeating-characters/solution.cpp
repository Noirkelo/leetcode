// Created by Noirkelo at 2025/08/05 19:17
// leetgo: dev
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/

#include <bits/stdc++.h>
#include "LC_IO.h"
using namespace std;

// @lc code=begin

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        if(s==""){
            return 0;
        }
		// dp[i-1]+1
		// i-pre[s[i]]

		
		//   	   st|...|j-1
		//bacdab
        vector<int > pre(128,-1);
        int ans=1;
        pre[s[0]]=0;
        int cur_length=1;
		int ss = 1;
		for (int i = 1; i < s.size(); i++)
		{
			if(i-pre[s[i]]>ss){
                ss=ss+1;
            }
            else {
                ss=i-pre[s[i]];
            }
            ans=max(ans,ss);
			pre[s[i]]=i;
		}
		return ans;
    }
};


// @lc code=end

int main() {
	ios_base::sync_with_stdio(false);
	stringstream out_stream;

	string s;
	LeetCodeIO::scan(cin, s);

	Solution *obj = new Solution();
	auto res = obj->lengthOfLongestSubstring(s);
	LeetCodeIO::print(out_stream, res);
	cout << "\noutput: " << out_stream.rdbuf() << endl;

	delete obj;
	return 0;
}
