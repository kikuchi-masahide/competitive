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

int main() {
	std::ios::sync_with_stdio(false);
	std::cin.tie(0);
	LL N;
	vector<string> S;
	VLL T;
	cin >> N;
	S.resize(N);
	T.resize(N);
	for (LL n = 0; n < N; n++) {
		cin >> S[n] >> T[n];
	}
	string end;
	cin >> end;
	LL n = 0;
	while (n < N) {
		if (S[n] == end)break;
		n++;
	}
	n++;
	LL ans = 0;
	for (; n < N; n++) {
		ans += T[n];
	}
	cout << ans  << "\n";
	return 0;
}
