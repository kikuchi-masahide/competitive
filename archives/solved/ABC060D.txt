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

int N;
vector<long long> V, W;
long long Wmax;
vector<vector<vector<long long>>> DP;

int main() {
	cin >> N >> Wmax;
	V.resize(N + 1);
	W.resize(N + 1);
	for (int n = 1; n <= N; n++) {
		cin >> W[n] >> V[n];
	}
	DP.resize(N + 1, vector<vector<long long>>(N + 1, vector<long long>(301, -1)));
	DP[0][0][0] = 0;
	for (int n = 1; n <= N; n++) {
		DP[n][0][0] = 0;
		for(int k = 1;k <= n;k++)
		for (int w = 0; w <= 300; w++) {
			DP[n][k][w] = DP[n - 1][k][w];
			if (w - W[n] + W[1] >= 0) {
				DP[n][k][w] = max(DP[n][k][w], DP[n - 1][k - 1][w - W[n] + W[1]] + V[n]);
			}
		}
	}
	long long ans = 0;
	for (int k = 0; k <= N; k++) {
		for (int w = 0; w <= min((long long)300,Wmax - k * W[1]); w++) {
			ans = max(ans, DP[N][k][w]);
		}
	}
	cout << ans << endl;
	return 0;
}