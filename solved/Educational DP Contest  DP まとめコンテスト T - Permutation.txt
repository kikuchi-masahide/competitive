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
#include <iomanip>
#include <cassert>
using namespace std;
long long MOD = 1e9 + 7;

long long N;
vector<long long> comp;
vector<vector<long long>>DP;

int main() {
	cin >> N;
	comp.resize(N);
	string S;
	cin >> S;
	for (long long n = 1; n <= N - 1; n++) {
		if (S[n-1] == '>')comp[n] = 1;
		else comp[n] = -1;
	}
	DP.resize(N + 1,vector<long long>(N+1,0));
	for (long long n = 1; n <= N; n++)DP[1][n] = 1;
	for (long long n = 2; n <= N; n++) {
		if (comp[n - 1] == 1) {
			DP[n][n] = 0;
			for (long long j = n - 1; j >= 1; j--) {
				DP[n][j] = (DP[n - 1][j] + DP[n][j + 1]) % MOD;
			}
		}
		else {
			DP[n][1] = 0;
			for (long long j = 1; j <= N; j++) {
				DP[n][j] = (DP[n][j - 1] + DP[n - 1][j - 1]) % MOD;
			}
		}
	}
	long long ans = 0;
	for (long long n = 1; n <= N; n++) {
		ans = (ans + DP[N][n]) % MOD;
	}
	cout << ans << endl;
	return 0;
}