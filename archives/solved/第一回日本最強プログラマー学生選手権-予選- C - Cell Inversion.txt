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
using namespace std;

int N;
string S;
vector<int> B;
int MOD = 1e9 + 7;

void solve() {
	cin >> N >> S;
	B.resize(2 * N);
	if (S[0] == 'W') {
		cout << 0 << endl;
		return;
	}
	B[0] = 1;
	long long lc = 1;
	for (int n = 1; n < 2 * N; n++) {
		if (S[n] == S[n - 1])B[n] = 1 - B[n - 1];
		else B[n] = B[n - 1];
		lc += B[n];
	}
	if (lc != N) {
		cout << 0 << endl;
		return;
	}
	lc = 0;
	long long ans = 1;
	for (int n = 0; n < 2 * N; n++) {
		if (B[n] == 1)lc++;
		else {
			ans *= lc;
			ans %= MOD;
			lc--;
		}
	}
	for (int n = 2; n <= N; n++) {
		ans *= n;
		ans %= MOD;
	}
	cout << ans << endl;
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
