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

using namespace std;
//std::ios::sync_with_stdio(false);
//std::cin.tie(0);
const long long MOD = 1e9 + 7;
typedef long long ll;
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
	ll N;
	vector<ll> A;
	cin >> N;
	A.reserve(N);
	ll a;
	ll suma = 0;
	for (ll n = 0; n < N; n++) {
		cin >> a;
		A.push_back(a);
		suma += a;
	}
	ll time = N * (N + 1) / 2;
	if (suma % time != 0)goto no;
	time = suma / time;
	for (ll n = 0; n < N; n++) {
		A[n] -= time * n;
	}
	suma = 0;
	for (ll i = 0; i < N - 1; i++) {
		if (A[i] - A[i+1] < 0)goto no;
		if ((A[i] - A[i + 1]) % N != 0)goto no;
		suma += (A[i] - A[i + 1]) / N;
	}
	if (suma > time)goto no;
	goto yes;
yes:
	cout << "YES\n";
	return 0;
no:
	cout << "NO\n";
	return 0;
}