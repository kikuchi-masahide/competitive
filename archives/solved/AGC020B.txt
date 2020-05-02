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
	A.resize(N);
	for (ll n = 0; n < N; n++)cin >> A[n];
	if (A[N - 1] != 2) {
		cout << -1 << endl;
		return 0;
	}
	ll dp1 = 2, dp2 = 2;
	for (ll n = N - 2; n >= 0; n--) {
		ll bottom = ceil((ld)dp1 / A[n]);
		ll top = floor((ld)(dp2 + A[n + 1] - 1) / A[n]);
		if (bottom > top) {
			cout << -1 << endl;
			return 0;
		}
		dp1 = bottom * A[n];
		dp2 = top * A[n];
	}
	cout << dp1 << " " << dp2+A[0]-1 << endl;
	return 0;
}