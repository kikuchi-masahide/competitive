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
#include <bitset>

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
typedef unsigned long long ULL;
//typedef boost::multiprecision::cpp_int bigint;

int main() {
	std::ios::sync_with_stdio(false);
	std::cin.tie(0);
	string S;
	cin >> S;
	ll N = S.size();
	VLL Q(N + 1, 0);
	if (S[0] == '?')Q[0] = 1;
	for (ll n = 1; n < N; n++) {
		if (S[n] == '?')Q[n] = Q[n - 1] + 1;
		else Q[n] = Q[n - 1];
	}
	Q[N] = Q[N - 1];
	VLL pow3(N + 1,1);
	for (ll n = 1; n <= N; n++) {
		pow3[n] = (pow3[n - 1] * 3) % MOD;
	}
	VLL C(N,0);
	if (S.back() == '?' || S.back() == 'C')C.back() = 1;
	else C.back() = 0;
	for (ll n = N - 2; n >= 2; n--) {
		if (S[n] == '?') {
			C[n] = (3 * C[n + 1] + pow3[Q[N - 1] - Q[n]]) % MOD;
		}
		else if (S[n] == 'C') {
			C[n] = (C[n + 1] + pow3[Q[N - 1] - Q[n]]) % MOD;
		}
		else {
			C[n] = C[n + 1];
		}
	}
	VLL A(N, 0);
	if (S[0] == '?' || S[0] == 'A')A[0] = 1;
	else A[0] = 0;
	for (ll n = 1; n < N - 2; n++) {
		if (S[n] == '?') {
			A[n] = (3 * A[n - 1] + pow3[Q[n - 1]]) % MOD;
		}
		else if (S[n] == 'A') {
			A[n] = (A[n - 1] + pow3[Q[n - 1]]) % MOD;
		}
		else {
			A[n] = A[n - 1];
		}
	}
	ll ans = 0;
	for (ll k = 1; k < N - 1; k++) {
		if (S[k] == 'B' || S[k] == '?') {
			ans = (ans + A[k - 1] * C[k + 1]) % MOD;
		}
	}
	cout << ans << "\n";
	return 0;
}
