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
using namespace std;

int N;
vector<int> A, T;
typedef pair<int, int> pii;
vector<pii> range;

pii cap(pii a, pii b) {
	if (a == pii(0, -1) || b == pii(0, -1))return pii(0, -1);
	pii c = pii(max(a.first, b.first), min(a.second, b.second));
	if (c.first > c.second)return pii(0, -1);
	else return c;
}

int main() {
	cin >> N;
	for (int n = 0; n < N; n++) {
		int a;
		cin >> a;
		T.push_back(a);
	}
	for (int n = 0; n < N; n++) {
		int a;
		cin >> a;
		A.push_back(a);
	}
	range.resize(N, pii(1, 1e9));
	range[0] = pii(T[0], T[0]);
	for (int n = 1; n < N; n++) {
		if (T[n - 1] != T[n]) {
			range[n] = cap(range[n], pii(T[n], T[n]));
		}
		else {
			range[n] = cap(range[n], pii(1, T[n]));
		}
	}
	range[N - 1] = cap(range[N - 1], pii(A[N - 1], A[N - 1]));
	for (int n = N - 2; n >= 0; n--) {
		if (A[n + 1] != A[n]) {
			range[n] = cap(range[n], pii(A[n], A[n]));
		}
		else {
			range[n] = cap(range[n], pii(1, A[n]));
		}
	}
	long long ans = 1;
	int MOD = 1e9 + 7;
	for (int n = 0; n < N; n++) {
		ans *= (range[n].second - range[n].first + 1);
		ans %= MOD;
	}
	cout << ans << endl;
	return 0;
}