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

ll N;
vector<ll> A;

int main() {
	cin >> N;
	A.resize(N + 2,0);
	for (ll n = 1; n <= N; n++)cin >> A[n];
	for (ll n = 1; n <= N + 1; n++) {
		A[n] += A[n - 1];
	}
	ll ans = LLONG_MAX;
	for (ll n = 1; n < N; n++) {
		ll res = A[n] - (A[N] - A[n]);
		ans = min(ans, abs(res));
	}
	cout << ans << endl;
	return 0;
}
