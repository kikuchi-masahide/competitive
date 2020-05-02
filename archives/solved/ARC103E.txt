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
	if (S.back() == '1') {
		cout << "-1\n";
		return 0;
	}
	if(S[0] == '0') {
		cout << "-1\n";
		return 0;
	}
	for (ll n = 1; n < N; n++) {
		if (S[n - 1] != S[N - n - 1]) {
			cout << "-1\n";
			return 0;
		}
	}
	VLL V;
	for (ll n = 2; n < N; n++) {
		if (S[n - 1] == '1')V.push_back(n);
	}
	V.push_back(N);
	ll prev = 1;
	for (ll n : V) {
		while (prev < n) {
			cout << prev << " " << n << "\n";
			prev++;
		}
	}
	return 0;
}
