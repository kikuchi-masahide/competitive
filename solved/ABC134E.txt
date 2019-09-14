#include <iostream>
#include <vector>
#include <limits.h>
#include <algorithm>
#include <string>
#include <math.h>
#include <limits.h>
#include <queue>
#include <map>
#include <set>
#include <iomanip>
#include <bitset>
#include <cassert>
#include <random>
#include <functional>
#include <stack>
using namespace std;

int N;
vector<int> A;
void solve() {
	cin >> N;
	A.clear();
	A.resize(N);
	for (int n = 0; n < N; n++)cin >> A[n];
	vector<int> dp;
	dp.resize(N, -1);
	dp[0] = A[0];
	for (int n = 1; n < N; n++) {
		int s = -1, e = N-1;
		while (e - s > 1) {
			int m = (e + s) / 2;
			if (dp[m] >= A[n])s = m;
			else e = m;
		}
		dp[e] = A[n];
	}
	int n = 0;
	for (; n < N; n++) {
		if (dp[n] == -1)break;
	}
	cout << n << endl;
	return;
}

//////////////////////////////
//////////////////////////////

int main() {
#ifdef DEBUGTIME
	for (int time = 0; time < DEBUGTIME; time++) {
		solve();
		cout << "####################" << endl;
	}
#else
	solve();
#endif
	return 0;
}
