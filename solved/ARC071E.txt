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

string S, T;
vector<ll>AS,AT;

int main() {
	cin >> S >> T;
	AS.resize(S.size() + 1,0);
	AT.resize(T.size() + 1,0);
	for (ll n = 0; n < S.size(); n++) {
		if (S[n] == 'A')AS[n + 1] = AS[n] + 1;
		else AS[n + 1] = AS[n];
	}
	for (ll n = 0; n < T.size(); n++) {
		if (T[n] == 'A')AT[n + 1] = AT[n] + 1;
		else AT[n + 1] = AT[n];
	}
	ll Q;
	cin >> Q;
	for (ll q = 0; q < Q; q++) {
		ll a, b, c, d;
		cin >> a >> b >> c >> d;
		ll s = 2 * (AS[b] - AS[a - 1]) - (b - a + 1);
		ll t = 2 * (AT[d] - AT[c - 1]) - (d - c + 1);
		s = (s % 3) + 3;
		s %= 3;
		t = (t % 3) + 3;
		t %= 3;
		if(s == t)cout << "YES" << endl;
		else cout << "NO" << endl;
	}
	return 0;
}