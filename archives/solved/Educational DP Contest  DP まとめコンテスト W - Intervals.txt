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
//#include <boost/multiprecision/cpp_int.hpp>
#include <complex>
#include <cstdio>
#include <list>

//< in.txt > out.txt
using namespace std;
//std::ios::sync_with_stdio(false);
//std::cin.tie(0);
const long long MOD = 1e9 + 7;
typedef long long ll;
typedef long long LL;
typedef long double ld;
typedef pair<ll, ll> pll;
typedef pair<ld, ll> pdl;
typedef pair<ld, ld> pdd;
typedef vector<ll> VLL;
typedef vector<VLL> VVLL;
//typedef boost::multiprecision::cpp_int bigint;

typedef struct {
	LL l, r, s;
} query;

VLL lazy, seg;

void eval(LL k, LL l, LL r) {
	if (k >= seg.size())return;
	if (lazy[k] == 1e15)return;
	seg[k] += lazy[k];
	if (k < seg.size() / 2) {
		if (lazy[2 * k] == 1e15)lazy[2 * k] = lazy[k];
		else lazy[2 * k] += lazy[k];
		if (lazy[2 * k + 1] == 1e15)lazy[2 * k + 1] = lazy[k];
		else lazy[2 * k + 1] += lazy[k];
	}
	lazy[k] = 1e15;
}

void add(LL a, LL b, LL x, LL k = 1, LL l = 0, LL r = seg.size() / 2 - 1) {
	eval(k, l, r);
	if (r < a || b < l)return;
	if (a <= l && r <= b) {
		if (lazy[k] == 1e15)lazy[k] = x;
		else lazy[k] += x;
		eval(k, l, r);
	}
	else {
		add(a, b, x, 2 * k, l, (l + r) / 2);
		add(a, b, x, 2 * k + 1, (l + r) / 2+1, r);
		seg[k] = max(seg[2 * k], seg[2 * k + 1]);
	}
}

LL getmax(LL a, LL b, LL k = 1, LL l = 0, LL r = seg.size() / 2-1) {
	if (r < a || b < l)return 0;
	eval(k, l, r);
	if (a <= l && r <= b)return seg[k];
	else return max(
		getmax(a, b, 2 * k, l, (l + r) / 2),
		getmax(a, b, 2 * k + 1, (l + r) / 2 + 1, r)
	);
}

int main() {
	std::ios::sync_with_stdio(false);
	std::cin.tie(0);
	LL N;
	cin >> N;
	LL M;
	cin >> M;
	vector<query> querys(M + 1);
	LL SUM = 0;
	for (LL m = 0; m < M; m++) {
		query& q = querys[m];
		cin >> q.l >> q.r >> q.s;
		q.s *= -1;
		SUM -= q.s;
	}
	querys.back() = { N + 1,N + 1,0 };
	VLL DP(N + 1, 0);
	LL RN = 1;
	while (RN < N+1)RN *= 2;
	seg.resize(2 * RN, 0);
	lazy.resize(2 * RN, 1e15);
	sort(querys.begin(), querys.end(), [](query a, query b) {
		return a.r < b.r;
	});
	LL look = 0;
	for (LL n = 1; n <= N; n++) {
		LL t = getmax(0, n - 1);
		DP[n] = t;
		add(n, n, t);
		while (querys[look].r == n) {
			add(0, querys[look].l - 1, querys[look].s);
			look++;
		}
	}
	for (LL n = 0; n < lazy.size(); n++)lazy[n] = 1e15;
	for (LL n = 0; n <= N; n++) seg[RN + n] = DP[n];
	for (LL n = RN - 1; n >= 1; n--) {
		seg[n] = max(seg[2 * n], seg[2 * n + 1]);
	}
	for (auto q : querys) {
		add(0, q.l - 1, q.s);
	}
	cout << SUM+getmax(0,N) << "\n";
	return 0;
}
