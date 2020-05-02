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

using namespace std;
const long long MOD = 1e9 + 7;
typedef long long ll;
typedef long double ld;
typedef pair<ll, ll> pll;
typedef pair<ld, ll> pdl;
typedef pair<ld, ld> pdd;
//typedef boost::multiprecision::cpp_int bigint;

vector<ll> seg, lazy;

void eval(ll k) {
	if (k >= seg.size())return;
	if (lazy[k] == 0)return;
	seg[k] += lazy[k];
	if (k < seg.size() / 2) {
		lazy[2 * k] += lazy[k];
		lazy[2 * k + 1] += lazy[k];
	}
	lazy[k] = 0;
}

void add(ll a, ll b, ll k = 1, ll l = 0, ll r = seg.size() / 2 - 1) {
	eval(k);
	if (r < a || b < l)return;
	if (a <= l && r <= b) {
		lazy[k] += 1;
		eval(k);
	}
	else {
		add(a, b, 2 * k, l, (l + r) / 2);
		add(a, b, 2 * k + 1, (l + r) / 2 + 1, r);
		seg[k] = seg[2 * k] + seg[2 * k + 1];
	}
}

ll get(ll a, ll b, ll k = 1, ll l = 0, ll r = seg.size() / 2 - 1) {
	if (r < a || b < l)return 0;
	eval(k);
	if (a <= l && r <= b)return seg[k];
	else return get(a, b, 2 * k, l, (l + r) / 2) + get(a, b, 2 * k + 1, (l + r) / 2 + 1, r);
}

ll N, K;
vector<ll> A;
vector<ll> S;
map<ll, ll> order;

int main() {
	cin >> N >> K;
	A.resize(N + 1,K);
	for (ll n = 1; n <= N; n++)cin >> A[n];
	S.resize(N + 1,0);
	for (ll n = 1; n <= N; n++) {
		S[n] = S[n - 1] + (A[n] - K);
	}
	{
		set<ll> s;
		for (ll n = 0; n <= N; n++)s.insert(S[n]);
		vector<ll> V;
		for (auto n : s)V.push_back(n);
		for (ll n = 0; n < V.size(); n++) {
			order.insert(pll(V[n], n));
		}
	}
	ll ans = 0;
	ll RN = 1;
	while (RN < N)RN *= 2;
	RN *= 2;
	seg.resize(RN);
	lazy.resize(RN);
	for (ll n = 1; n <= N; n++) {
		ll ord = order.find(S[n])->second;
		ans += ::get(ord, ord);
		if (S[n] >= 0)ans++;
		::add(ord, RN - 1);
	}
	cout << ans << endl;
	return 0;
}