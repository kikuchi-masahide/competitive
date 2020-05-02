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
using namespace std;
long long MOD = 1e9 + 7;
typedef long long ll;
typedef pair<ll, ll> pll;
//typedef boost::multiprecision::cpp_int bigint;

ll N, A, B, C, D;

bool judge(ll n) {
	ll x = B - A + (N - 2 * n - 1) * C;
	ll up = n * (D - C);
	ll down = -(N - 1 - n) * (D - C);
	if (up >= x && x >= down)return true;
	else return false;
}

int main() {
	cin >> N >> A >> B >> C >> D;
	for (ll n = 0; n < N; n++) {
		if (judge(n)) {
			cout << "YES" << endl;
			return 0;
		}
	}
	cout << "NO" << endl;
	return 0;
}